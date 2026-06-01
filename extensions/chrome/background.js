// CPOS background worker — polls VS Code (:27122) and TUI (:27121) for pending
// submissions, then injects into the logged-in browser tab.

const ENDPOINTS = [
  { name: "CPOS VS Code", baseUrl: "http://127.0.0.1:27122" },
  { name: "CPOS TUI", baseUrl: "http://127.0.0.1:27121" }
];

const CF_LANGUAGE_IDS = {
  cpp: 54,
  c: 43,
  python: 31,
  pypy: 40,
  java: 60,
  kotlin: 73,
  rust: 75,
  go: 32,
  csharp: 79,
  javascript: 55,
  ruby: 67,
  haskell: 12,
  pascal: 51
};

let handling = false;

async function fetchPending() {
  for (const endpoint of ENDPOINTS) {
    try {
      const res = await fetch(`${endpoint.baseUrl}/pending-submit`);
      if (!res.ok) continue;
      const data = await res.json();
      if (!data.ok || !data.code) continue;
      return { endpoint, data };
    } catch {
      /* try next */
    }
  }
  return null;
}

async function ack(endpoint) {
  try {
    await fetch(`${endpoint.baseUrl}/pending-submit/consumed`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: "{}"
    });
  } catch {
    /* ignore */
  }
}

function sleep(ms) {
  return new Promise((r) => setTimeout(r, ms));
}

function waitForTab(tabId, timeoutMs = 15000) {
  return new Promise((resolve) => {
    const deadline = Date.now() + timeoutMs;
    const listener = (id, info) => {
      if (id === tabId && info.status === "complete") {
        chrome.tabs.onUpdated.removeListener(listener);
        resolve(true);
      }
      if (Date.now() > deadline) {
        chrome.tabs.onUpdated.removeListener(listener);
        resolve(false);
      }
    };
    chrome.tabs.onUpdated.addListener(listener);
    setTimeout(() => {
      chrome.tabs.onUpdated.removeListener(listener);
      resolve(false);
    }, timeoutMs);
  });
}

function urlsMatch(a, b) {
  try {
    const ua = new URL(a);
    const ub = new URL(b);
    return ua.hostname === ub.hostname && ua.pathname === ub.pathname;
  } catch {
    return a === b;
  }
}

async function findOrOpenTab(url) {
  if (!url) return null;

  for (let i = 0; i < 8; i++) {
    const tabs = await chrome.tabs.query({});
    const match = tabs.find((t) => t.url && urlsMatch(t.url, url));
    if (match?.id != null) return match;
    await sleep(400);
  }

  const tab = await chrome.tabs.create({ url, active: true });
  if (tab.id != null) await waitForTab(tab.id);
  return tab;
}

// Runs in Codeforces page MAIN world.
async function cposSubmitOnPage(code, languageId, problemIndex, problemId) {
  const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

  function findTextarea() {
    return (
      document.getElementById("sourceCodeTextarea") ||
      document.querySelector('textarea[name="source"]') ||
      document.querySelector('textarea[name="sourceCode"]')
    );
  }

  function findLang() {
    return (
      document.querySelector('select[name="programTypeId"]') ||
      document.getElementById("programTypeId")
    );
  }

  function findProblemCodeInput() {
    const el = document.querySelector('input[name="submittedProblemCode"]');
    return el instanceof HTMLInputElement ? el : null;
  }

  function findProblemIndexSelect() {
    return document.querySelector('select[name="submittedProblemIndex"]');
  }

  function problemCode() {
    if (problemId) return String(problemId);
    if (problemIndex) return String(problemIndex);
    return indexFromUrl() || "";
  }

  function getAceEditor() {
    if (typeof window.ace === "undefined" || typeof window.ace.edit !== "function") {
      return null;
    }
    try {
      return window.ace.edit("editor");
    } catch {
      return null;
    }
  }

  function aceReady() {
    const ed = getAceEditor();
    return !!ed && typeof ed.setValue === "function";
  }

  function getCsrf() {
    return (
      document.querySelector('meta[name="X-Csrf-Token"]')?.getAttribute("content") ||
      document.querySelector(".csrf-token[data-csrf]")?.getAttribute("data-csrf") ||
      document.querySelector('[name="csrf_token"]')?.value ||
      null
    );
  }

  function readSource() {
    const ed = getAceEditor();
    if (ed) {
      const v = ed.getValue();
      if (v?.trim()) return v;
    }
    const textarea = findTextarea();
    if (textarea?.value?.trim()) return textarea.value;
    return "";
  }

  function writeSource(text) {
    const ed = getAceEditor();
    if (ed) {
      ed.setValue(text, -1);
      ed.clearSelection();
      ed.resize();
    }
    const textarea = findTextarea();
    if (textarea) {
      textarea.value = text;
      textarea.dispatchEvent(new Event("input", { bubbles: true }));
      textarea.dispatchEvent(new Event("change", { bubbles: true }));
      if (window.jQuery) {
        window.jQuery(textarea).val(text).trigger("change");
      }
    }
  }

  function setProblemField() {
    const fullId = problemCode();
    const input = findProblemCodeInput();
    if (input && fullId) {
      input.value = fullId;
      input.dispatchEvent(new Event("input", { bubbles: true }));
      input.dispatchEvent(new Event("change", { bubbles: true }));
      return true;
    }
    const select = findProblemIndexSelect();
    if (select && problemIndex) {
      setProblem(select, problemIndex);
      return true;
    }
    return false;
  }

  function setProblem(prob, idx) {
    if (!prob || !idx) return;
    const want = String(idx).toUpperCase();
    for (const opt of prob.options) {
      const val = (opt.value || "").toUpperCase();
      const text = (opt.textContent || "").trim().toUpperCase();
      if (val === want || text.startsWith(want) || val.includes(want)) {
        prob.value = opt.value;
        prob.dispatchEvent(new Event("change", { bubbles: true }));
        return;
      }
    }
  }

  function setLanguage(lang, id) {
    if (!lang || id == null) return;
    for (const opt of lang.options) {
      if (opt.value === String(id)) {
        lang.value = opt.value;
        lang.dispatchEvent(new Event("change", { bubbles: true }));
        return;
      }
    }
  }

  function submitBasePath() {
    const p = location.pathname;
    const patterns = [
      /^(\/contest\/\d+\/submit)/,
      /^(\/gym\/\d+\/submit)/,
      /^(\/group\/[^/]+\/contest\/\d+\/submit)/,
      /^(\/edu\/[^/]+\/lesson\/[^/]+\/[^/]+\/practice\/contest\/\d+\/submit)/,
      /^(\/problemset\/submit)/
    ];
    for (const re of patterns) {
      const m = p.match(re);
      if (m) return m[1];
    }
    return null;
  }

  function indexFromUrl() {
    const params = new URLSearchParams(location.search);
    return params.get("submittedProblemIndex") || params.get("submittedProblemCode") || null;
  }

  async function tryPost() {
    const basePath = submitBasePath();
    const csrf = getCsrf();
    if (!basePath || !csrf) return false;

    const lang = findLang();
    const codeInput = findProblemCodeInput();
    const problemSelect = findProblemIndexSelect();
    const programTypeId =
      (lang && lang.value) || (languageId != null ? String(languageId) : null);
    if (!programTypeId || !String(code).trim()) return false;

    let problemField;
    let problemValue;
    if (codeInput) {
      problemField = "submittedProblemCode";
      problemValue = codeInput.value || problemCode();
    } else if (problemSelect) {
      problemField = problemSelect.name || "submittedProblemIndex";
      problemValue = problemSelect.value || problemIndex || indexFromUrl();
    } else {
      problemField = "submittedProblemIndex";
      problemValue = problemIndex || indexFromUrl();
    }
    if (!problemValue) return false;

    const body = new URLSearchParams();
    body.set("csrf_token", csrf);
    body.set("action", "submitSolutionFormSubmitted");
    body.set(problemField, problemValue);
    body.set("programTypeId", programTypeId);
    body.set("source", code);

    const res = await fetch(`${basePath}?csrf_token=${encodeURIComponent(csrf)}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
        "X-Csrf-Token": csrf,
        "X-Requested-With": "XMLHttpRequest"
      },
      body: body.toString(),
      credentials: "include",
      redirect: "follow"
    });
    return res.ok || res.redirected;
  }

  function clickSubmit() {
    const btn =
      document.getElementById("singlePageSubmitButton") ||
      document.querySelector('input.submit[type="submit"]') ||
      document.querySelector('form.submit-form input[type="submit"]');
    if (btn && !btn.disabled) {
      btn.disabled = false;
      btn.click();
      return true;
    }
    const form =
      document.querySelector("form.submit-form") ||
      document.querySelector('form[action*="submit"]');
    if (form && typeof form.requestSubmit === "function") {
      form.requestSubmit();
      return true;
    }
    return false;
  }

  for (let i = 0; i < 55; i++) {
    const lang = findLang();
    if (!lang || lang.options.length <= 1) {
      await sleep(200);
      continue;
    }

    setProblemField();
    if (languageId != null) setLanguage(lang, languageId);
    await sleep(i < 3 ? 600 : 250);

    if (!aceReady() && !findTextarea() && i < 50) {
      await sleep(200);
      continue;
    }

    try {
      if (await tryPost()) return { ok: true };
    } catch {
      /* fall through to DOM fill */
    }

    writeSource(code);
    await sleep(250);
    if (!readSource().trim()) {
      writeSource(code);
      await sleep(250);
    }
    if (!readSource().trim()) {
      await sleep(200);
      continue;
    }

    if (clickSubmit()) return { ok: true };
    return { ok: false, reason: "no-submit-btn" };
  }
  return { ok: false, reason: "form-timeout" };
}

// Runs in CSES submit page MAIN world.
async function cposCsesSubmitOnPage(code, fileName, language) {
  const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
  const hints = {
    cpp: ["C++"],
    c: ["C"],
    python: ["Python3", "Python 3", "CPython"],
    java: ["Java"],
    rust: ["Rust"]
  };
  const optionHints = { cpp: ["C++17"], rust: ["2021"], python: ["CPython3"] };

  for (let i = 0; i < 40; i++) {
    let form = null;
    for (const f of document.querySelectorAll("form")) {
      if (f.querySelector('input[type="file"]')) {
        form = f;
        break;
      }
    }
    if (form) {
      const fileInput = form.querySelector('input[type="file"]');
      if (fileInput) {
        const file = new File([code], fileName || "solution.cpp", { type: "text/plain" });
        const dt = new DataTransfer();
        dt.items.add(file);
        fileInput.files = dt.files;
        fileInput.dispatchEvent(new Event("change", { bubbles: true }));
      }

      const typeSelect = form.querySelector('select[name="type"]') || form.querySelector("select");
      if (typeSelect && language) {
        const needles = hints[language] || [];
        for (const opt of typeSelect.options) {
          if (needles.some((n) => (opt.textContent || "").includes(n))) {
            typeSelect.value = opt.value;
            typeSelect.dispatchEvent(new Event("change", { bubbles: true }));
            break;
          }
        }
        await sleep(250);
      }

      const optionSelect = form.querySelector('select[name="option"]');
      const opts = optionHints[language] || [];
      if (optionSelect && opts.length) {
        for (const opt of optionSelect.options) {
          if (opts.some((n) => (opt.textContent || "").includes(n))) {
            optionSelect.value = opt.value;
            break;
          }
        }
      }

      await sleep(200);

      for (const el of form.querySelectorAll("input[type='submit'], button")) {
        const text = (el.value || el.textContent || "").trim().toLowerCase();
        if (text === "send" || text === "submit") {
          el.click();
          return { ok: true };
        }
      }
      if (typeof form.requestSubmit === "function") {
        form.requestSubmit();
        return { ok: true };
      }
      return { ok: false, reason: "no-send-btn" };
    }
    await sleep(200);
  }
  return { ok: false, reason: "cses-form-timeout" };
}

async function handleCodeforces(pending, _endpoint) {
  const tab = await findOrOpenTab(pending.submitUrl);
  if (!tab?.id) return false;

  const languageId = CF_LANGUAGE_IDS[pending.language] ?? null;
  const problemIndex = pending.index || null;

  const results = await chrome.scripting.executeScript({
    target: { tabId: tab.id, allFrames: false },
    world: "MAIN",
    func: cposSubmitOnPage,
    args: [pending.code, languageId, problemIndex, pending.id || null]
  });

  return results?.[0]?.result?.ok === true;
}

async function handleCses(pending, _endpoint) {
  const tab = await findOrOpenTab(pending.submitUrl);
  if (!tab?.id) return false;

  const results = await chrome.scripting.executeScript({
    target: { tabId: tab.id, allFrames: false },
    world: "MAIN",
    func: cposCsesSubmitOnPage,
    args: [pending.code, pending.fileName || "solution.cpp", pending.language || "cpp"]
  });

  return results?.[0]?.result?.ok === true;
}

async function pollOnce() {
  if (handling) return;
  const found = await fetchPending();
  if (!found) return;

  handling = true;
  try {
    const { data: pending } = found;
    const platform = String(pending.platform || "").toLowerCase();
    let ok = false;
    if (platform === "codeforces" || platform === "cf") {
      ok = await handleCodeforces(pending, found.endpoint);
    } else if (platform === "cses") {
      ok = await handleCses(pending, found.endpoint);
    }
    if (ok) await ack(found.endpoint);
  } finally {
    handling = false;
  }
}

chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  if (msg?.type === "cpos-cf-submit") {
    const tabId = sender.tab?.id;
    if (!tabId) {
      sendResponse({ ok: false, reason: "no-tab" });
      return;
    }
    chrome.scripting
      .executeScript({
        target: { tabId, allFrames: false },
        world: "MAIN",
        func: cposSubmitOnPage,
        args: [msg.code, msg.languageId ?? null, msg.problemIndex ?? null, msg.problemId ?? null]
      })
      .then((results) => {
        sendResponse(results?.[0]?.result || { ok: false, reason: "empty-result" });
      })
      .catch((error) => {
        sendResponse({ ok: false, reason: String(error) });
      });
    return true;
  }
  if (msg?.type === "cpos-poll-submit") {
    pollOnce().then(() => sendResponse({ ok: true }));
    return true;
  }
});

setInterval(pollOnce, 800);
pollOnce();
