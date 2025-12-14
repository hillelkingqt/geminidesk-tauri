const GEMINI_URL = 'https://gemini.google.com/app';

function openExternal() {
  if (window.__TAURI__?.shell?.open) {
    return window.__TAURI__.shell.open(GEMINI_URL);
  }
  window.open(GEMINI_URL, '_blank', 'noopener,noreferrer');
}

function navigateInApp() {
  try {
    window.location.replace(GEMINI_URL);
  } catch (err) {
    console.warn('Failed to redirect to Gemini', err);
  }
}

document.addEventListener('DOMContentLoaded', () => {
  // Immediately try to move into the hosted Gemini experience.
  navigateInApp();

  const retry = document.querySelector('[data-retry]');
  if (retry) {
    retry.addEventListener('click', () => navigateInApp());
  }

  const external = document.querySelector('[data-open-external]');
  if (external) {
    external.addEventListener('click', () => openExternal());
  }
});

export { GEMINI_URL, navigateInApp, openExternal };
