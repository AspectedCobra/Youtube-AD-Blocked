(() => {
  if (window.__ytonly) return; window.__ytonly = true;

  // ---- Force YouTube's dark theme (cookie), reload once if needed ----
  try {
    if (/(^|\.)youtube\.com$/.test(location.hostname)) {
      const m = document.cookie.match(/(?:^|; )PREF=([^;]*)/);
      const p = new URLSearchParams(m ? m[1] : '');
      if (p.get('f6') !== '400') {
        p.set('f6', '400');
        document.cookie = 'PREF=' + p.toString() + '; domain=.youtube.com; path=/; max-age=31536000; secure';
        if (!sessionStorage.__ytonlyDark) { sessionStorage.__ytonlyDark = 1; location.reload(); }
      }
    }
  } catch (e) {}

  // ---- OLED true-black overrides ----
  const css = `
    :root, html[dark], html {
      color-scheme: dark;
      --yt-spec-base-background:#000 !important;
      --yt-spec-raised-background:#000 !important;
      --yt-spec-menu-background:#0a0a0a !important;
      --yt-spec-general-background-a:#000 !important;
      --yt-spec-general-background-b:#000 !important;
      --yt-spec-general-background-c:#000 !important;
      --yt-spec-brand-background-solid:#000 !important;
      --yt-spec-brand-background-primary:#000 !important;
      --yt-spec-static-overlay-background-solid:#000 !important;
      --yt-spec-badge-chip-background:#1a1a1a !important;
    }
    html, body, ytd-app, #background, ytd-masthead, #masthead-container,
    #guide-content, ytd-mini-guide-renderer, tp-yt-app-drawer,
    ytd-feed-filter-chip-bar-renderer, #chips-wrapper, ytd-browse, ytd-watch-flexy {
      background: #000 !important;
    }`;
  const st = document.createElement('style');
  st.textContent = css;
  (document.head || document.documentElement).appendChild(st);

  // ---- Keep everything in this one window ----
  window.open = (u) => { if (u) location.href = u; return null; };
  document.addEventListener('click', (e) => {
    const a = e.target.closest && e.target.closest('a[target="_blank"]');
    if (a) a.target = '_self';
  }, true);

  // ---- Shorts auto-advance + backup ad skipper (event-driven, no polling) ----
  let advancing = false;
  const next = () => {
    const b = document.querySelector('#navigation-button-down button, #navigation-button-down');
    if (b) { b.click(); return; }
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowDown', keyCode: 40, bubbles: true }));
  };
  // timeupdate doesn't bubble, so listen in the capture phase; it only fires while a video plays
  document.addEventListener('timeupdate', (e) => {
    const v = e.target;
    if (!(v instanceof HTMLVideoElement) || !isFinite(v.duration) || v.duration < 1) return;
    if (v.closest('.html5-video-player.ad-showing')) {
      v.muted = true; v.currentTime = v.duration;
      const s = document.querySelector('.ytp-skip-ad-button, .ytp-ad-skip-button, .ytp-ad-skip-button-modern');
      if (s) s.click();
      return;
    }
    if (!location.pathname.startsWith('/shorts/')) return;
    v.loop = false;
    if (!advancing && (v.ended || v.duration - v.currentTime < 0.4)) {
      advancing = true; next();
      setTimeout(() => { advancing = false; }, 1500);
    }
  }, true);
})();
