/* ============================================================
 * lucid-lint — navigation enhancements
 *
 * Runs after the mdBook body renders (additional-js loads at
 * end-of-body). Adds three accessibility-and-orientation wins:
 *
 *   1. Skip-to-content link — AAA requirement; first focusable
 *      element; visible only on focus.
 *   2. Breadcrumbs — derived from the sidebar's part-title /
 *      active-chapter structure; inserted above the page H1.
 *   3. Page TOC — right-rail sticky nav of the page's H2/H3
 *      headings, visible on viewports wide enough to host it
 *      (~1100px+); hidden on narrower screens (responsive pass
 *      in /adapt will polish the mobile treatment).
 *
 * Smooth-scroll on anchor click is gated on prefers-reduced-
 * motion: users who asked for less motion get the browser's
 * default instant jump.
 * ============================================================ */

(function () {
  const d = document;
  const body = d.body;
  const content = d.getElementById('mdbook-content');
  if (!content) return;

  const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  // ---- 1. Skip to content ----------------------------------
  (function skipLink() {
    const a = d.createElement('a');
    a.href = '#mdbook-content';
    a.className = 'lucid-skip';
    a.textContent = 'Skip to main content';
    body.insertBefore(a, body.firstChild);

    a.addEventListener('click', (e) => {
      e.preventDefault();
      content.setAttribute('tabindex', '-1');
      content.focus({ preventScroll: reduceMotion });
      if (!reduceMotion) content.scrollIntoView({ behavior: 'smooth', block: 'start' });
      else content.scrollIntoView();
    });
  })();

  // ---- 2. Breadcrumbs --------------------------------------
  (function breadcrumbs() {
    const active = d.querySelector('.sidebar a.active');
    if (!active) return;
    const li = active.closest('li');
    // Walk previous siblings (up the sidebar list) to find the
    // nearest preceding .part-title. mdBook's sidebar places part
    // titles as siblings rather than parents.
    let part = null;
    for (let sib = li.previousElementSibling; sib; sib = sib.previousElementSibling) {
      if (sib.classList && sib.classList.contains('part-title')) { part = sib; break; }
    }

    const h1 = d.querySelector('.content h1');
    if (!h1) return;

    const nav = d.createElement('nav');
    nav.className = 'lucid-breadcrumbs';
    nav.setAttribute('aria-label', 'Breadcrumb');

    const ol = d.createElement('ol');
    if (part) {
      const liPart = d.createElement('li');
      liPart.textContent = part.textContent.trim();
      ol.appendChild(liPart);
    }
    const liHere = d.createElement('li');
    liHere.setAttribute('aria-current', 'page');
    liHere.textContent = active.textContent.trim();
    ol.appendChild(liHere);
    nav.appendChild(ol);

    h1.parentNode.insertBefore(nav, h1);
  })();

  // ---- 3. Page TOC -----------------------------------------
  (function pageToc() {
    const headings = Array.from(content.querySelectorAll('h2, h3'));
    if (headings.length < 2) return;

    const aside = d.createElement('aside');
    aside.className = 'lucid-pagetoc';
    aside.setAttribute('aria-label', 'On this page');

    const label = d.createElement('p');
    label.className = 'lucid-pagetoc__label';
    label.textContent = 'On this page';
    aside.appendChild(label);

    const ul = d.createElement('ul');
    headings.forEach((h) => {
      if (!h.id) return;
      const li = d.createElement('li');
      if (h.tagName === 'H3') li.className = 'lucid-pagetoc__nested';
      const a = d.createElement('a');
      a.href = '#' + h.id;
      a.textContent = h.textContent.trim();
      li.appendChild(a);
      ul.appendChild(li);
    });
    aside.appendChild(ul);

    // Insert as the first child of .content so CSS can float it
    // to the right rail without restructuring mdBook's layout.
    content.insertBefore(aside, content.firstChild);

    // Active-section highlight via IntersectionObserver.
    const links = new Map();
    aside.querySelectorAll('a').forEach((a) => {
      links.set(a.getAttribute('href').slice(1), a);
    });
    if (!('IntersectionObserver' in window)) return;

    const io = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        const link = links.get(entry.target.id);
        if (!link) return;
        if (entry.isIntersecting) {
          aside.querySelectorAll('a[aria-current]').forEach((x) => x.removeAttribute('aria-current'));
          link.setAttribute('aria-current', 'location');
        }
      });
    }, { rootMargin: '0px 0px -70% 0px', threshold: 0 });

    headings.forEach((h) => io.observe(h));
  })();
})();
