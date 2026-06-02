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

  for (let i = 0; i < 12; i++) {
    const tabs = await chrome.tabs.query({});
    const match = tabs.find((t) => t.url && urlsMatch(t.url, url));
    if (match?.id != null) {
      await waitForTab(match.id, 15000);
      return match;
    }
    await sleep(400);
  }

  const tab = await chrome.tabs.create({ url, active: true });
  if (tab.id != null) await waitForTab(tab.id, 15000);
  return tab;
}

// Runs in the Codeforces page main world: set sourceCodeTextarea, programTypeId,
// problem field, then click .submit. Do not fire change events on selects (resets Ace).
async function cposSubmitOnPage(code, languageId, languageKey, problemIndex, submitByIndex, problemCode) {
  const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

  const LANG_RANK = {
    cpp: [/GNU G\+\+23/i, /GNU G\+\+20/i, /GNU G\+\+17/i, /G\+\+23/i, /G\+\+20/i, /G\+\+17/i, /G\+\+/i],
    c: [/GNU GCC C11/i, /GNU GCC C\b/i, /\bGNU C\b/i],
    python: [/^Python 3/i, /Python 3\.\d/i, /\bPython 3\b/i, /PyPy 3/i],
    pypy: [/PyPy 3/i, /PyPy/i],
    java: [/Java 21/i, /Java 17/i, /Java 11/i, /\bJava\b/i],
    kotlin: [/Kotlin/i],
    rust: [/Rust 1\.\d/i, /Rust/i],
    go: [/\bGo\b/i],
    csharp: [/\.NET[^#]*C#/i, /Mono C#/i, /C#/i],
    javascript: [/Node\.js/i, /JavaScript/i],
    ruby: [/Ruby/i],
    haskell: [/Haskell/i],
    pascal: [/PascalABC/i, /Free Pascal/i, /Delphi/i]
  };

  function pickLanguage(select) {
    if (!select || select.options.length <= 1) return;
    if (languageId != null) {
      const want = String(languageId);
      for (const opt of select.options) {
        if (opt.value === want) {
          select.value = opt.value;
          return;
        }
      }
    }
    const ranks = LANG_RANK[languageKey] || [];
    for (const re of ranks) {
      for (const opt of select.options) {
        if (re.test(opt.textContent || "")) {
          select.value = opt.value;
          return;
        }
      }
    }
  }

  for (let i = 0; i < 100; i++) {
    const sourceCodeEl = document.getElementById("sourceCodeTextarea");
    const languageEl = document.getElementsByName("programTypeId")[0];
    const submitBtn = document.querySelector(".submit");

    if (
      sourceCodeEl &&
      languageEl &&
      languageEl.options.length > 1 &&
      submitBtn &&
      String(code).trim()
    ) {
      sourceCodeEl.value = code;
      pickLanguage(languageEl);

      if (submitByIndex && problemIndex) {
        const problemIndexEl = document.getElementsByName("submittedProblemIndex")[0];
        if (problemIndexEl) {
          problemIndexEl.value = String(problemIndex);
        }
      } else if (problemCode) {
        const codeInput = document.querySelector('input[name="submittedProblemCode"]');
        if (codeInput) {
          codeInput.value = String(problemCode);
        }
      }

      if (!sourceCodeEl.value.trim()) {
        await sleep(200);
        continue;
      }

      submitBtn.disabled = false;
      submitBtn.click();
      return { ok: true };
    }

    await sleep(250);
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

function cfSubmitFlags(pending) {
  let pathname = "";
  try {
    pathname = new URL(pending.submitUrl || "").pathname;
  } catch {
    pathname = "";
  }
  const submitByIndex =
    /\/contest\/\d+\/submit/.test(pathname) ||
    /\/gym\/\d+\/submit/.test(pathname) ||
    /\/group\/[^/]+\/contest\/\d+\/submit/.test(pathname);
  const problemIndex = pending.index || null;
  const problemCode = submitByIndex ? null : pending.id || null;
  return { submitByIndex, problemIndex, problemCode };
}

async function handleCodeforces(pending, _endpoint) {
  const tab = await findOrOpenTab(pending.submitUrl);
  if (!tab?.id) return false;

  const languageId = CF_LANGUAGE_IDS[pending.language] ?? null;
  const { submitByIndex, problemIndex, problemCode } = cfSubmitFlags(pending);

  for (let attempt = 0; attempt < 8; attempt++) {
    if (attempt > 0) await sleep(800);

    let ok = false;
    try {
      const results = await chrome.scripting.executeScript({
        target: { tabId: tab.id, allFrames: false },
        world: "MAIN",
        func: cposSubmitOnPage,
        args: [
          pending.code,
          languageId,
          pending.language || "cpp",
          problemIndex,
          submitByIndex,
          problemCode
        ]
      });
      ok = results?.[0]?.result?.ok === true;
    } catch {
      try {
        const t = await chrome.tabs.get(tab.id);
        if (t?.url && !/\/submit/.test(new URL(t.url).pathname)) ok = true;
      } catch {
        ok = true;
      }
    }

    if (ok) return true;
  }

  return false;
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
        args: [
          msg.code,
          msg.languageId ?? null,
          msg.language || "cpp",
          msg.problemIndex ?? null,
          msg.submitByIndex !== false,
          msg.problemCode ?? null
        ]
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
