(() => {
  const root = document.documentElement;
  const button = document.querySelector(".theme-toggle");
  if (!button) return;

  const apply = (theme, persist) => {
    const isDark = theme === "dark";
    root.setAttribute("data-theme", isDark ? "dark" : "light");
    button.setAttribute("aria-pressed", String(isDark));
    button.setAttribute(
      "aria-label",
      isDark
        ? button.getAttribute("data-light-label")
        : button.getAttribute("data-dark-label"),
    );
    const icon = button.querySelector(".theme-icon");
    if (icon) icon.textContent = isDark ? "☼" : "◐";
    const label = button.querySelector(".theme-label");
    if (label) {
      label.textContent = isDark
        ? button.getAttribute("data-dark-mode-label")
        : button.getAttribute("data-light-mode-label");
    }
    if (persist) {
      try {
        localStorage.setItem("saptrans-theme", isDark ? "dark" : "light");
      } catch (_) {
        // Theme switching still works for this page when storage is unavailable.
      }
    }
  };

  const initial = root.getAttribute("data-theme") || "light";
  apply(initial, false);
  button.addEventListener("click", () => {
    apply(root.getAttribute("data-theme") === "dark" ? "light" : "dark", true);
  });
})();
