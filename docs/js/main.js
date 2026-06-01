(function () {
  const captions = {
    dashboard: "Rating, streak, progress, and what to solve next.",
    problems: "Browse the full catalog — search, filter, open. No Codeforces tab needed.",
    contests: "Upcoming and live Codeforces contests with countdowns.",
    analytics: "Rating graph, topic breakdown, and activity heatmap.",
    recommend: "30 unsolved problems picked for your weak tags.",
  };

  const captionEl = document.getElementById("caption");
  const tabs = document.querySelectorAll(".tab");
  const panels = document.querySelectorAll(".frame .panel");

  tabs.forEach((tab) => {
    tab.addEventListener("click", () => {
      const id = tab.dataset.screen;
      if (tab.classList.contains("active")) return;

      tabs.forEach((t) => t.classList.remove("active"));
      tab.classList.add("active");

      panels.forEach((p) => p.classList.remove("active"));
      const next = document.getElementById(`screen-${id}`);
      if (next) next.classList.add("active");

      if (captionEl) {
        captionEl.classList.add("fade");
        setTimeout(() => {
          captionEl.textContent = captions[id] || "";
          captionEl.classList.remove("fade");
        }, 120);
      }
    });
  });

  document.querySelectorAll("[data-copy]").forEach((btn) => {
    btn.addEventListener("click", async () => {
      const cmd = btn.dataset.copy;
      try {
        await navigator.clipboard.writeText(cmd);
        const code = btn.querySelector("code");
        if (code) {
          const orig = code.textContent;
          code.textContent = "copied!";
          setTimeout(() => { code.textContent = orig; }, 1500);
        }
      } catch {
        window.prompt("Run:", cmd);
      }
    });
  });
})();
