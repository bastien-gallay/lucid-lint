/* ============================================================
 * lucid-lint — navigation enhancements
 *
 * Runs after the mdBook body renders (additional-js loads at
 * end-of-body). Adds:
 *
 *   1. Skip-to-content link — AAA requirement; first focusable
 *      element; visible only on focus.
 *   2. Breadcrumbs — derived from the sidebar's part-title /
 *      active-chapter structure; inserted above the page H1.
 *   3. Page TOC — right-rail sticky nav of the page's H2/H3
 *      headings, visible on viewports wide enough to host it
 *      (~1100px+); hidden on narrower screens (responsive pass
 *      in /adapt will polish the mobile treatment).
 *   4. Brand mark — lens SVG next to the sidebar title.
 *   5. Language switch — EN | FR in the header; FR links to
 *      /fr/ stub page (real French content is F25 in v0.2).
 *   6. Reading demonstrator — wires the "Use this" buttons on
 *      the Introduction page to the font preset and persists
 *      the choice.
 *
 * Smooth-scroll on anchor click is gated on prefers-reduced-
 * motion: users who asked for less motion get the browser's
 * default instant jump.
 *
 * Copy strings are bilingual EN/FR so the v0.2 French mirror
 * picks them up without a rewrite.
 * ============================================================ */

const LUCID_COPY = {
  en: {
    skip: 'Skip to main content',
    brand: 'lucid-lint',
    langEn: 'EN',
    langFr: 'FR',
    langLabel: 'Choose language',
    tocLabel: 'On this page',
    demoUsed: 'Reading set to ',
    demoUndo: 'Undo',
    demoFontLabels: {
      atkinson: 'Atkinson Hyperlegible Next',
      standard: 'Standard (serif headings)',
      dyslexic: 'OpenDyslexic',
    },
    // Copy for the future reading-preferences popover (stored
    // here so the popover UI picks it up when it lands).
    prefsTitle: 'Reading preferences',
    prefsFont: 'Font',
    prefsFontRec: 'recommended',
    prefsSpacing: 'Line spacing',
    prefsSize: 'Text size',
    prefsReset: 'Reset to defaults',
    prefsResetToast: 'Defaults restored.',
  },
  fr: {
    skip: 'Aller au contenu principal',
    brand: 'lucid-lint',
    langEn: 'EN',
    langFr: 'FR',
    langLabel: 'Choisir la langue',
    tocLabel: 'Sur cette page',
    demoUsed: 'Lecture réglée sur ',
    demoUndo: 'Annuler',
    demoFontLabels: {
      atkinson: 'Atkinson Hyperlegible Next',
      standard: 'Standard (titres serif)',
      dyslexic: 'OpenDyslexic',
    },
    prefsTitle: 'Préférences de lecture',
    prefsFont: 'Police',
    prefsFontRec: 'recommandée',
    prefsSpacing: 'Interligne',
    prefsSize: 'Taille du texte',
    prefsReset: 'Réinitialiser',
    prefsResetToast: 'Paramètres par défaut rétablis.',
  },
};

(function () {
  const d = document;
  const body = d.body;
  const content = d.getElementById('mdbook-content');
  if (!content) return;

  const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  const isFr = location.pathname.indexOf('/fr/') !== -1;
  const t = isFr ? LUCID_COPY.fr : LUCID_COPY.en;

  // ---- 1. Skip to content ----------------------------------
  (function skipLink() {
    const a = d.createElement('a');
    a.href = '#mdbook-content';
    a.className = 'lucid-skip';
    a.textContent = t.skip;
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
    label.textContent = t.tocLabel;
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

  // ---- 4. Sidebar brand mark -------------------------------
  (function brandMark() {
    const title = d.querySelector('.menu-title');
    if (!title || title.querySelector('.lucid-brand__mark')) return;
    // path_to_root is embedded by mdBook into links; derive a
    // usable prefix from the sidebar's first chapter link so the
    // brand image resolves correctly from deep pages.
    const probe = d.querySelector('.sidebar .chapter a');
    const href = probe ? probe.getAttribute('href') : '';
    // mdBook serves sidebar links relative to path_to_root. We
    // back-walk from the current page to the site root by keeping
    // only the `../` prefix of the first chapter's href.
    const toRoot = href && href.startsWith('..') ? href.replace(/[^/]+$/, '') : '';
    const img = d.createElement('img');
    img.className = 'lucid-brand__mark';
    img.alt = '';
    img.setAttribute('aria-hidden', 'true');
    img.src = (toRoot || './') + '_brand/lens-icon.svg';
    title.insertBefore(img, title.firstChild);
  })();

  // ---- 5. Language switch ----------------------------------
  (function langSwitch() {
    const bar = d.querySelector('.right-buttons, .menu-bar .right-buttons, .menu-bar');
    if (!bar) return;
    const wrap = d.createElement('div');
    wrap.className = 'lucid-lang';
    wrap.setAttribute('role', 'group');
    wrap.setAttribute('aria-label', t.langLabel);

    const probe = d.querySelector('.sidebar .chapter a');
    const href = probe ? probe.getAttribute('href') : '';
    const toRoot = href && href.startsWith('..') ? href.replace(/[^/]+$/, '') : '';

    const makeLink = (code, label, target, active) => {
      const a = d.createElement('a');
      a.className = 'lucid-lang__link' + (active ? ' is-active' : '');
      a.href = target;
      a.hreflang = code;
      a.textContent = label;
      if (active) a.setAttribute('aria-current', 'true');
      return a;
    };

    const enHref = isFr ? '../introduction.html' : '#';
    const frHref = isFr ? '#' : (toRoot || './') + 'fr/index.html';
    wrap.appendChild(makeLink('en', t.langEn, enHref, !isFr));
    const sep = d.createElement('span');
    sep.className = 'lucid-lang__sep';
    sep.setAttribute('aria-hidden', 'true');
    sep.textContent = '|';
    wrap.appendChild(sep);
    wrap.appendChild(makeLink('fr', t.langFr, frHref, isFr));

    bar.appendChild(wrap);
  })();

  // ---- 6. Reading demonstrator -----------------------------
  (function demonstrator() {
    const buttons = d.querySelectorAll('.reading-demo__apply[data-apply]');
    if (!buttons.length) return;

    const applyPreset = (preset) => {
      d.documentElement.setAttribute('data-font', preset);
      try {
        const prev = JSON.parse(localStorage.getItem('lucidLintReading') || '{}');
        const next = Object.assign({}, prev, { font: preset });
        localStorage.setItem('lucidLintReading', JSON.stringify(next));
      } catch (e) { /* silent */ }
      // Update button pressed state for accessibility
      buttons.forEach((b) => b.setAttribute('aria-pressed', b.dataset.apply === preset ? 'true' : 'false'));
      // Toast
      showToast(t.demoUsed + t.demoFontLabels[preset], preset);
    };

    // Restore current preset on load
    const current = d.documentElement.getAttribute('data-font') || 'atkinson';
    buttons.forEach((b) => b.setAttribute('aria-pressed', b.dataset.apply === current ? 'true' : 'false'));

    buttons.forEach((b) => {
      b.addEventListener('click', () => applyPreset(b.dataset.apply));
    });

    let toastTimer;
    function showToast(msg, _preset) {
      let toast = d.getElementById('lucid-toast');
      if (!toast) {
        toast = d.createElement('div');
        toast.id = 'lucid-toast';
        toast.className = 'lucid-toast';
        toast.setAttribute('role', 'status');
        toast.setAttribute('aria-live', 'polite');
        body.appendChild(toast);
      }
      toast.textContent = msg;
      toast.classList.add('is-visible');
      clearTimeout(toastTimer);
      toastTimer = setTimeout(() => toast.classList.remove('is-visible'), 2400);
    }
  })();
})();
