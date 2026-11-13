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

(() => {
  const header = document.querySelector(".header");
  if (header) {
    const menus = [...header.querySelectorAll("details")];
    let scrollQueued = false;
    const updateHeader = () => {
      header.classList.toggle("is-scrolled", window.scrollY > 16);
      scrollQueued = false;
    };
    updateHeader();
    window.addEventListener("scroll", () => {
      if (scrollQueued) return;
      scrollQueued = true;
      requestAnimationFrame(updateHeader);
    }, { passive: true });

    menus.forEach((menu) => {
      menu.addEventListener("toggle", () => {
        if (!menu.open) return;
        menus.forEach((other) => {
          if (other !== menu) other.open = false;
        });
      });
    });
    const solutionMenu = header.querySelector(".solutions-menu");
    const choices = [...(solutionMenu?.querySelectorAll("[data-mega-target]") || [])];
    const panels = [...(solutionMenu?.querySelectorAll("[data-mega-content]") || [])];
    const selectSolution = (choice) => {
      choices.forEach((item) => item.setAttribute("aria-pressed", String(item === choice)));
      panels.forEach((panel) => { panel.hidden = panel.dataset.megaContent !== choice.dataset.megaTarget; });
    };
    choices.forEach((choice) => {
      choice.addEventListener("click", () => selectSolution(choice));
      choice.addEventListener("pointerenter", () => selectSolution(choice));
    });
    document.addEventListener("pointerdown", (event) => {
      if (menus.some((menu) => menu.contains(event.target))) return;
      menus.forEach((menu) => { menu.open = false; });
    });
    document.addEventListener("keydown", (event) => {
      if (event.key !== "Escape") return;
      const openMenu = menus.find((menu) => menu.open);
      if (!openMenu) return;
      openMenu.open = false;
      if (openMenu.contains(document.activeElement)) {
        openMenu.querySelector("summary")?.focus();
      }
    });
  }

  const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)");
  if (!("IntersectionObserver" in window) || reduceMotion.matches) return;

  const targets = [...document.querySelectorAll(
    ".quick-heading, .quick-grid > a, .editorial-intro, .services-section .section-heading-row, .service-card, .sector-strip-inner, .focus-photo, .focus-copy, .estate-copy, .estate-visual, .home-method-heading, .home-method-grid article, .locations-section .section-heading-row, .location-list > a, .partners-heading, .partners-copy, .band-inner, .page-hero .container, .page-hero + .section, .cards > .card, .editorial-copy, .editorial-media, .service-offers .section-heading-row, .offer-card, .situations-section .section-heading-row, .situation-card, .company-network .section-heading-row, .contact-locations .section-heading-row, .source-section .editorial-split, .process-section .container, .faq-section, .directory-lead, .directory-list article, .location-directory article, .empty-editorial, .company-divisions a, .contact-paths a"
  )];
  if (!targets.length) return;

  const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (!entry.isIntersecting) return;
      entry.target.classList.add("is-visible");
      observer.unobserve(entry.target);
    });
  }, { rootMargin: "0px 0px -6% 0px", threshold: 0.08 });

  targets.forEach((target) => {
    target.classList.add("motion-target");
    if (target.matches(".quick-grid > a, .cards > .card, .location-list > a, .offer-card, .situation-card, .directory-list article, .location-directory article, .company-divisions a, .contact-paths a, .home-method-grid article")) {
      const position = [...target.parentElement.children].indexOf(target);
      target.style.setProperty("--motion-delay", `${(position % 4) * 70}ms`);
    }
    if (target.getBoundingClientRect().top < window.innerHeight * 0.92) {
      target.classList.add("is-visible");
    }
    observer.observe(target);
  });
  document.documentElement.classList.add("motion-ready");

  reduceMotion.addEventListener("change", (event) => {
    if (!event.matches) return;
    observer.disconnect();
    document.documentElement.classList.remove("motion-ready");
    targets.forEach((target) => target.classList.add("is-visible"));
  });
})();

(() => {
  const nav = document.querySelector(".page-anchor-nav");
  if (!nav) return;
  const links = [...nav.querySelectorAll('a[href^="#"]')];
  const sections = links.map((link) => document.getElementById(link.hash.slice(1)));
  if (sections.some((section) => !section)) return;

  let queued = false;
  const update = () => {
    const sectionOffset = parseFloat(getComputedStyle(sections[0]).scrollMarginTop) || 0;
    const threshold = nav.getBoundingClientRect().bottom + sectionOffset + 24;
    let active = 0;
    sections.forEach((section, index) => {
      if (section.getBoundingClientRect().top <= threshold) active = index;
    });
    links.forEach((link, index) => {
      const selected = index === active;
      link.classList.toggle("is-active", selected);
      if (selected) link.setAttribute("aria-current", "location");
      else link.removeAttribute("aria-current");
    });
    queued = false;
  };
  update();
  links.forEach((link, index) => {
    link.addEventListener("click", (event) => {
      event.preventDefault();
      const section = sections[index];
      const top = window.scrollY + section.getBoundingClientRect().top - nav.getBoundingClientRect().height - 24;
      history.pushState(null, "", link.hash);
      window.scrollTo({ top, behavior: window.matchMedia("(prefers-reduced-motion: reduce)").matches ? "instant" : "smooth" });
      update();
    });
  });
  window.addEventListener("scroll", () => {
    if (queued) return;
    queued = true;
    requestAnimationFrame(update);
  }, { passive: true });
  window.addEventListener("resize", update);
})();
