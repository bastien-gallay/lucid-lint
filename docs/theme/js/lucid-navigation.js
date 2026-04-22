/* ============================================================
 * lucid-lint — navigation enhancements
 *
 * Runs after the mdBook body renders (additional-js loads at
 * end-of-body). Adds:
 *
 *   1. Smooth-scroll handler for the server-rendered skip link
 *      (F35a — the anchor itself lives in `theme/index.hbs` so
 *      WCAG 2.4.1 Bypass Blocks is satisfied without JS).
 *   2. Breadcrumbs — derived from the sidebar's part-title /
 *      active-chapter structure; inserted above the page H1.
 *   3. Page TOC — right-rail sticky nav of the page's H2/H3
 *      headings, visible on viewports wide enough to host it
 *      (~1100px+); hidden on narrower screens (responsive pass
 *      in /adapt will polish the mobile treatment).
 *   4. Brand mark — lens SVG next to the sidebar title.
 *   5. Reading demonstrator — wires the "Use this" buttons on
 *      the Introduction page to the font preset and persists
 *      the choice.
 *
 * The EN / FR language switch is server-rendered in
 * `theme/index.hbs` (F35a) and no longer injected here.
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
    crumbLabel: 'Breadcrumb',
    tocAriaLabel: 'On this page',
    demoUsePrefix: 'Use ',
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
    crumbLabel: 'Fil d’Ariane',
    tocAriaLabel: 'Sur cette page',
    demoUsePrefix: 'Utiliser ',
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

  // Note: <html lang="fr"> is set earlier (in head.hbs, before body
  // paint) so assistive tech reads it on first parse rather than
  // after a post-body mutation. No override needed here.

  // ---- 1. Skip to content (enhancement only) ---------------
  // The anchor itself is server-rendered in `theme/index.hbs`
  // (F35a). This handler adds the smooth-scroll + focus-move
  // polish; if the script fails to load, the browser still
  // follows the anchor, so WCAG 2.4.1 Bypass Blocks is
  // unaffected. One listener per variant (EN / FR) — CSS
  // removes the wrong-lang copy from layout, but the node
  // still exists in the DOM.
  d.querySelectorAll('.lucid-skip').forEach((a) => {
    a.addEventListener('click', (e) => {
      e.preventDefault();
      content.setAttribute('tabindex', '-1');
      content.focus({ preventScroll: reduceMotion });
      if (!reduceMotion) content.scrollIntoView({ behavior: 'smooth', block: 'start' });
      else content.scrollIntoView();
    });
  });

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
    nav.setAttribute('aria-label', t.crumbLabel);

    const ol = d.createElement('ol');
    // VoiceOver strips list semantics from <ol> with list-style: none;
    // re-assert the role so assistive tech still announces "list, N items".
    ol.setAttribute('role', 'list');
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
    aside.setAttribute('aria-label', t.tocAriaLabel);

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

  // ---- 4b. FR sidebar cue ----------------------------------
  // The book is single-locale (mdBook limitation) so the sidebar
  // lists EN chapter titles. On /fr/* pages we can't swap them
  // for a real FR sidebar without a parallel build, but we can
  // frame them honestly: a brief FR header at the top, then the
  // EN chapters under a "Documentation anglaise" divider. That
  // stops the reader from thinking "I clicked FR and got English."
  if (isFr) {
    (function frSidebarCue() {
      // mdBook's sidebar nests .sidebar > .sidebar-scrollbox > ol.chapter.
      // Insert the cue and divider inside the scrollbox so both live as
      // siblings of the chapter list — keeps insertBefore valid and lets
      // the cue scroll with the list on short viewports.
      const scrollbox = d.querySelector('.sidebar .sidebar-scrollbox');
      if (!scrollbox) return;
      if (scrollbox.querySelector('.lucid-fr-cue')) return;

      const chapters = scrollbox.querySelector('.chapter');

      const cue = d.createElement('div');
      cue.className = 'lucid-fr-cue';
      cue.innerHTML =
        '<p class="lucid-fr-cue__title">Version française</p>' +
        '<p class="lucid-fr-cue__note">' +
        'La documentation complète est en chantier (F25). ' +
        'En attendant, les chapitres ci-dessous sont en anglais.' +
        '</p>';
      scrollbox.insertBefore(cue, scrollbox.firstChild);

      if (chapters) {
        const divider = d.createElement('p');
        divider.className = 'lucid-fr-cue__divider';
        divider.textContent = 'Documentation anglaise';
        scrollbox.insertBefore(divider, chapters);
      }
    })();
  }

  // ---- 4c. Theme picker relabel -----------------------------
  // mdBook ships a 5-option theme menu (Light · Rust · Coal · Navy ·
  // Ayu). lucid-colors.css collapses those five into two palettes,
  // so the extra labels mislead. Trim the menu to Auto + Lucid light
  // + Lucid dark in place; hide the rest. (A full index.hbs override
  // was considered and rejected — the DOM-level trim keeps us on the
  // stock template and survives mdBook upgrades.)
  (function themePickerRelabel() {
    const list = d.getElementById('mdbook-theme-list');
    if (!list) return;
    const labels = isFr
      ? { auto: 'Auto', light: 'Lucid clair', coal: 'Lucid sombre' }
      : { auto: 'Auto', light: 'Lucid light', coal: 'Lucid dark' };
    const keep = {
      'mdbook-theme-default_theme': labels.auto,
      'mdbook-theme-light':         labels.light,
      'mdbook-theme-coal':          labels.coal,
    };
    list.querySelectorAll('li').forEach((li) => {
      const btn = li.querySelector('button');
      if (!btn) return;
      if (Object.prototype.hasOwnProperty.call(keep, btn.id)) {
        btn.textContent = keep[btn.id];
      } else {
        li.hidden = true;
      }
    });
  })();

  // ---- 5. Language switch ----------------------------------
  // Server-rendered in `theme/index.hbs` (F35a). No JS
  // injection.

  // ---- 6. Reading demonstrator -----------------------------
  // Two markups supported:
  //  (a) Legacy 3-card:  .reading-demo__apply[data-apply]   (button per card)
  //  (b) Chip-selector:  .reading-demo__chip[data-apply]    (radio chips)
  // In (b) the single preview swaps font + label to match the
  // selected chip; persistence + toast behavior is shared.
  (function demonstrator() {
    const legacyButtons = d.querySelectorAll('.reading-demo__apply[data-apply]');
    const chips = d.querySelectorAll('.reading-demo__chip[data-apply]');
    const controls = [...legacyButtons, ...chips];
    if (!controls.length) return;

    const preview       = d.querySelector('.reading-demo--chips .reading-demo__sample');
    const previewLabel  = d.querySelector('.reading-demo--chips [data-chip-label]');

    const applyPreset = (preset) => {
      d.documentElement.setAttribute('data-font', preset);
      try {
        const prev = JSON.parse(localStorage.getItem('lucidLintReading') || '{}');
        const next = Object.assign({}, prev, { font: preset });
        localStorage.setItem('lucidLintReading', JSON.stringify(next));
      } catch (e) { /* silent */ }
      legacyButtons.forEach((b) => b.setAttribute('aria-pressed', b.dataset.apply === preset ? 'true' : 'false'));
      chips.forEach((c) => c.setAttribute('aria-checked', c.dataset.apply === preset ? 'true' : 'false'));
      if (preview) preview.setAttribute('data-demo', preset);
      if (previewLabel) previewLabel.textContent = t.demoFontLabels[preset] || previewLabel.textContent;
      showToast(t.demoUsed + t.demoFontLabels[preset], preset);
    };

    const current = d.documentElement.getAttribute('data-font') || 'atkinson';
    legacyButtons.forEach((b) => {
      b.setAttribute('aria-pressed', b.dataset.apply === current ? 'true' : 'false');
      const presetName = t.demoFontLabels[b.dataset.apply];
      if (presetName) b.setAttribute('aria-label', t.demoUsePrefix + presetName);
    });
    chips.forEach((c) => {
      c.setAttribute('aria-checked', c.dataset.apply === current ? 'true' : 'false');
      const presetName = t.demoFontLabels[c.dataset.apply];
      if (presetName) c.setAttribute('aria-label', t.demoUsePrefix + presetName);
    });
    // Ensure preview matches persisted preset on load
    if (preview) preview.setAttribute('data-demo', current);
    if (previewLabel && t.demoFontLabels[current]) previewLabel.textContent = t.demoFontLabels[current];

    controls.forEach((el) => {
      el.addEventListener('click', () => applyPreset(el.dataset.apply));
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
