// @context {HTML} ../../templates/base.tera
// Fairy Ferris - Interactive Companion & App Logic (Vanilla JS)

document.addEventListener('DOMContentLoaded', () => {
  initTheme();
  initLanguagePicker();
  initFairyCursor();
  initSpellPlayground();
  initQuestRunner();
  initAuthUI();
  initGlobalAchievements();
  initSkyParallax();
  initScrollProgress();
  initLigaturePlayground();
  initAccountUI();
  initRecoveryUI();
});

/* -------------------------------------------------------------
 * 1. THEME MANAGEMENT (Dark by default, Light, System)
 * ------------------------------------------------------------- */
function initTheme() {
  const themePicker = document.getElementById('theme-picker');
  const _ = document.getElementsByClassName('theme-picker');
  const savedTheme = localStorage.getItem('fairy_theme') || 'dark';

  applyTheme(savedTheme);

  if (themePicker) {
    themePicker.value = savedTheme;
    themePicker.addEventListener('change', (e) => {
      const selected = e.target.value;
      localStorage.setItem('fairy_theme', selected);
      applyTheme(selected);
    });
  }

  // Listen to OS theme changes if 'system' is selected
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
    const current = localStorage.getItem('fairy_theme') || 'dark';
    if (current === 'system') {
      applyTheme('system');
    }
  });
}

function applyTheme(theme) {
  const root = document.documentElement;
  if (theme === 'system') {
    const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    root.setAttribute('data-theme', isDark ? 'dark' : 'light');
  } else {
    root.setAttribute('data-theme', theme);
  }
}

/* -------------------------------------------------------------
 * 2. MULTILINGUAL PICKER (KO, EN, RU)
 * ------------------------------------------------------------- */
function initLanguagePicker() {
  const langPicker = document.getElementById('lang-picker');
  if (!langPicker) return;

  langPicker.addEventListener('change', (e) => {
    const selectedLang = e.target.value;
    const url = new URL(window.location.href);
    url.searchParams.set('lang', selectedLang);
    window.location.href = url.toString();
  });
}

/* -------------------------------------------------------------
 * 3. GLOWING FAIRY CURSOR COMPANION
 * ------------------------------------------------------------- */
function initFairyCursor() {
  // Only enable on pointer-fine devices (desktop mouse)
  if (window.matchMedia('(pointer: coarse)').matches) return;

  const cursor = document.createElement('div');
  cursor.className = 'fairy-cursor';
  cursor.innerHTML = '<div class="fairy-orb"></div>';
  document.body.appendChild(cursor);

  let mouseX = -64;
  let mouseY = -64;
  let cursorX = mouseX;
  let cursorY = mouseY;
  let isHovered = false;

  window.addEventListener('mousemove', (e) => {
    mouseX = e.clientX;
    mouseY = e.clientY;
  });

  // Smooth lerp trailing loop
  function renderCursor() {
    cursorX += Math.trunc((mouseX - cursorX)) * .375;
    cursorY += Math.trunc((mouseY - cursorY)) * .375;
    cursor.style.transform = `translate3d(
      calc(${cursorX}px - var(--radius--fairy-cursor)),
      calc(${cursorY}px - var(--radius--fairy-cursor)), 0)
      scale(${isHovered ? 1.5 : 1}
    )`;
    requestAnimationFrame(renderCursor);
  }
  renderCursor();

  // Interactive hover reaction
  const interactives = 'a, button, input, select, .hero-mascot-card, .spell-card';
  document.addEventListener('mouseover', (e) => {
    if (e.target.closest(interactives)) {
      isHovered = true;
    }
  });
  document.addEventListener('mouseout', (e) => {
    if (e.target.closest(interactives)) {
      isHovered = false;
    }
  });
}

/* -------------------------------------------------------------
 * 4. INTERACTIVE FIRST SPELL PLAYGROUND (RUST PUZZLE)
 * ------------------------------------------------------------- */
function initSpellPlayground() {
  const slotInput = document.getElementById('slot-token');
  // Visible mirror of the hidden demo input (Chromium/Blink breaks JetBrains
  // Mono ligatures inside <input>, the <output> renders them correctly).
  const slotMirror = document.querySelector('output.slot-mirror');
  const castBtn = document.getElementById('btn-cast-spell');
  const resultBox = document.getElementById('spell-result');
  const speechBubble = document.getElementById('mascot-speech-text');

  if (!castBtn || !slotInput || !resultBox) return;

  // Paint both layers of the demo slot.
  const markDemo = (cls) => {
    slotInput.className = `slot-real ${cls}`;
    if (slotMirror) slotMirror.className = `slot-input slot-mirror ${cls}`;
  };

  // Keep the mirror in sync while the visitor types.
  if (slotMirror) {
    const syncMirror = () => { slotMirror.value = slotInput.value; };
    slotInput.addEventListener('input', syncMirror);
    syncMirror();
    slotMirror.addEventListener('click', () => slotInput.focus());
  }

  async function checkAnswer() {
    const token = slotInput.value.trim();
    const lang = document.body.dataset.lang || 'en';

    // Call the Axum backend AJAX endpoint
    try {
      const resp = await fetch(`/api/quest/quick-check?lang=${encodeURIComponent(lang)}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ token: token })
      });
      const data = await resp.json();
      console.debug(data)

      if (data.correct) {
        markDemo('correct');
        resultBox.className = 'spell-result-box success';
        resultBox.innerHTML = `✨ <span>${data.message}</span>`;
        if (speechBubble) {
          speechBubble.textContent = data.mascot_response || "🦀: Ура! Компилятор счастлив, руна mut оживила код!";
        }
        // Success ⇒ welcome the cozy Ferris back.
        setMascotMood('cozy', 'mascot-cozy', 'mascot-grump');
        // Update header XP counter in real-time from SQLite response
        const xpCounter = document.getElementById('player-xp-counter');
        if (xpCounter && typeof data.total_xp === 'number') {
          xpCounter.textContent = data.total_xp;
          xpCounter.parentElement.style.transform = 'scale(1.25)';
          setTimeout(() => {
            if (xpCounter.parentElement) xpCounter.parentElement.style.transform = '';
          }, 350);
        }
        // The success may have unlocked achievements — drain the global
        // toast pipeline a moment later so the popup is not jarring.
        setTimeout(() => { drainAchievements(); }, 1200);

        // THE GREEN CTA: the hero button stops being "try the puzzle" and
        // becomes the green "continue the adventure" link to /adventures.
        // No full reload is needed — this mirrors what SSR renders on the
        // next any-page load, so both paths agree within the session.
        const cta = document.getElementById('intro-cta-btn');
        const cast = document.getElementById("btn-cast-spell");
        if (cta && cast) {
          cta.classList.remove('btn-primary');
          cta.classList.add('btn-success');
          cta.setAttribute('href', `/adventures?lang=${lang}`);
          cta.dataset.successText = data.intro_continue;
          const cta_label = cta.querySelector('span');
          if (cta_label) cta_label.textContent = `🗺️ ${cta.dataset.successText}`;

          cast.classList.add('btn-success');
          cast.setAttribute('href', `/adventures?lang=${lang}`);
          cast.dataset.successText = data.intro_continue;
          const cast_label = cast.querySelector('span');
          if (cast_label) cast_label.textContent = `🗺️ ${cast.dataset.successText}`;
        }
      } else {
        markDemo('wrong');
        resultBox.className = 'spell-result-box error';
        resultBox.innerHTML = `⚠️ <span>${data.message}</span>`;
        if (speechBubble) {
          speechBubble.textContent = data.mascot_response || "🦀: Упс! Rust защищает память — вспомни, какое слово делает переменную изменяемой?";
        }
        // Wrong answer ⇒ the kawaii grumpy Ferris takes over the hero card.
        setMascotMood('grumpy', 'mascot-cozy', 'mascot-grump');
      }
    } catch (err) {
      // Fallback local check if offline
      if (token === 'mut') {
        markDemo('correct');
        resultBox.className = 'spell-result-box success';
        resultBox.innerHTML = '✨ <span>Скомпилировано! Переменная теперь изменяема (mut).</span>';
      } else {
        markDemo('wrong');
        resultBox.className = 'spell-result-box error';
        resultBox.innerHTML = '⚠️ <span>Ошибка компилятора: переменная неизменяема. Попробуй "mut"!</span>';
        setMascotMood('grumpy', 'mascot-cozy', 'mascot-grump');
      }
    }
  }

  castBtn.addEventListener('click', checkAnswer);
  slotInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      e.preventDefault(); // no implicit form submission on Enter
      checkAnswer();
    }
  });
}


/* -------------------------------------------------------------
 * 5. AUTH MODALS & FORMS (Stage 3)
 *    NOTE: initAuthUI() is called ONCE from the single DOMContentLoaded
 *    listener above. A previous duplicate DOMContentLoaded block ran this
 *    function a second time, attaching every submit handler TWICE — so
 *    clicking "Send code" fired TWO POST /api/auth/register requests and
 *    the user received two confirmation emails. One listener only!
 * ------------------------------------------------------------- */

function initAuthUI() {
  // --- Element references ---
  const modalLogin    = document.getElementById('modal-login');
  const modalRegister = document.getElementById('modal-register');
  const modalLogout   = document.getElementById('modal-logout-confirm');

  // NOTE: on successful auth ops the page does a FULL RELOAD instead of a
  // JS UI swap — the server renders the session state (XP counter, auth nav).
  const allModals = [modalLogin, modalRegister, modalLogout].filter(Boolean);

  if (!modalLogin || !modalRegister) return;

  // ---- Modal open/close helpers ----

  function openModal(modal) {
    modal.hidden = false;
    modal.style.display = '';
    // Trap focus: focus first input
    const first = modal.querySelector('input, button');
    if (first) setTimeout(() => first.focus(), 50);
  }

  function closeModal(modal) {
    modal.hidden = true;
    modal.style.display = 'none';
    clearErrors(modal);
  }

  function closeAll() {
    allModals.forEach(m => closeModal(m));
  }

  // Close on overlay click (but not card click)
  allModals.forEach(m => {
    m.addEventListener('click', (e) => {
      if (e.target === m) closeAll();
    });
  });

  // Close buttons inside modals
  document.querySelectorAll('[data-modal-close]').forEach(btn => {
    btn.addEventListener('click', () => {
      const id = btn.dataset.modalClose;
      const modal = document.getElementById(id);
      if (modal) closeModal(modal);
    });
  });

  // ESC closes active modal
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') closeAll();
  });

  // Switch-to links (e.g. "Already have account? Log in")
  document.querySelectorAll('[data-switch-to]').forEach(btn => {
    btn.addEventListener('click', () => {
      closeAll();
      const target = document.getElementById(btn.dataset.switchTo);
      if (target) openModal(target);
    });
  });

  // Header nav buttons
  document.getElementById('btn-open-login')?.addEventListener('click', () => {
    closeAll();
    openModal(modalLogin);
  });
  document.getElementById('btn-open-register')?.addEventListener('click', () => {
    closeAll();
    openModal(modalRegister);
  });

  // ---- Error display helpers ----

  function showError(el, msg) {
    if (!el) return;
    el.textContent = msg;
    el.hidden = false;
  }

  function clearErrors(container) {
    container.querySelectorAll('.auth-error').forEach(el => {
      el.textContent = '';
      el.hidden = true;
    });
  }

  // ---- REGISTER FLOW ----

  const formRegister = document.getElementById('form-register');
  const registerStep1 = document.getElementById('register-step-1');
  const registerStep2 = document.getElementById('register-step-2');
  const registerErrorEl = document.getElementById('register-error');
  let pendingRegisterEmail = '';

  formRegister?.addEventListener('submit', async (e) => {
    e.preventDefault();
    clearErrors(modalRegister);

    const nickname = document.getElementById('register-nickname')?.value.trim() || '';
    const email = document.getElementById('register-email')?.value.trim();
    const password = document.getElementById('register-password')?.value.trim();

    if (!nickname) {
      showError(registerErrorEl, 'Please choose a nickname.');
      return;
    }
    if (!email || !password) return;

    const submitBtn = formRegister.querySelector('[type="submit"]');
    submitBtn.disabled = true;

    try {
      const resp = await fetch('/api/auth/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ nickname, email, password }),
      });
      const data = await resp.json();

      if (data.ok) {
        pendingRegisterEmail = email;
        // Advance to step 2 (enter code)
        registerStep1.hidden = true;
        registerStep2.hidden = false;
        const codeInput = document.getElementById('verify-code');
        if (codeInput) setTimeout(() => codeInput.focus(), 50);
      } else {
        showError(registerErrorEl, data.error || 'Registration failed.');
      }
    } catch {
      showError(registerErrorEl, 'Network error. Please try again.');
    } finally {
      submitBtn.disabled = false;
    }
  });

  // ---- VERIFY EMAIL (step 2) ----

  const formVerify = document.getElementById('form-verify');
  const verifyErrorEl = document.getElementById('verify-error');

  formVerify?.addEventListener('submit', async (e) => {
    e.preventDefault();
    clearErrors(modalRegister);

    const code = document.getElementById('verify-code')?.value.trim();
    if (!code || code.length !== 6) {
      showError(verifyErrorEl, 'Please enter the 6-digit code.');
      return;
    }

    const submitBtn = formVerify.querySelector('[type="submit"]');
    submitBtn.disabled = true;

    try {
      const resp = await fetch('/api/auth/verify-email', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          email: pendingRegisterEmail,
          code,
        }),
      });
      const data = await resp.json();

      if (data.ok) {
        // Defer to the SSR page: cookie now belongs to the account session.
        closeModal(modalRegister);
        window.location.reload();
      } else {
        showError(verifyErrorEl, data.error || 'Invalid code.');
      }
    } catch {
      showError(verifyErrorEl, 'Network error. Please try again.');
    } finally {
      submitBtn.disabled = false;
    }
  });

  // ---- LOGIN FLOW ----

  const formLogin = document.getElementById('form-login');
  const loginErrorEl = document.getElementById('login-error');

  formLogin?.addEventListener('submit', async (e) => {
    e.preventDefault();
    clearErrors(modalLogin);

    const email    = document.getElementById('login-email')?.value.trim();
    const password = document.getElementById('login-password')?.value.trim();

    if (!email || !password) return;

    const submitBtn = formLogin.querySelector('[type="submit"]');
    submitBtn.disabled = true;

    try {
      const resp = await fetch('/api/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password }),
      });
      const data = await resp.json();

      if (data.ok) {
        // Defer to the SSR page: cookie now belongs to the account session.
        closeModal(modalLogin);
        window.location.reload();
      } else {
        showError(loginErrorEl, data.error || 'Login failed.');
      }
    } catch {
      showError(loginErrorEl, 'Network error. Please try again.');
    } finally {
      submitBtn.disabled = false;
    }
  });

  // ---- LOGOUT (with confirmation dialog) ----

  document.getElementById('btn-logout')?.addEventListener('click', () => {
    closeAll();
    if (modalLogout) openModal(modalLogout);
  });

  const formLogout = document.getElementById('form-logout');
  formLogout?.addEventListener('submit', async (e) => {
    e.preventDefault();
    const submitBtn = formLogout.querySelector('[type="submit"]');
    if (submitBtn) submitBtn.disabled = true;

    try {
      await fetch('/api/auth/logout', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({}),
      });
    } catch { /* ignore network hiccups — reload anyway */ }

    // Defer to the SSR page: the server swapped the cookie for a fresh
    // guest session and left the account session intact in the DB.
    window.location.reload();
  });

  // ---- Handle stale ?auth_error=... params ----
  const urlParams = new URLSearchParams(window.location.search);
  if (urlParams.get('auth_error')) {
    // Could show a toast; for now log it
    console.warn('Auth error:', urlParams.get('auth_error'));
  }
}


/* -------------------------------------------------------------
 * 6. QUEST RUNNER ENGINE (Stage 5)
 * ------------------------------------------------------------- */
function initQuestRunner() {
  const compileBtn = document.getElementById('btn-compile-quest');
  const tokenInput = document.getElementById('quest-token-input');
  const feedbackBox = document.getElementById('runner-feedback-box');
  const mascotSpeech = document.getElementById('runner-mascot-speech');
  const questIdInput = document.getElementById('runner-quest-id');
  const nextQuestBtn = document.getElementById('btn-next-quest');
  const sidebarStatus = document.getElementById('sidebar-quest-status');
  const xpCounter = document.getElementById('player-xp-counter');

  // Slots are hidden real <input>s (`.slot-real`, value holders), each paired
  // with a visible <output> mirror (`.slot-mirror`): Chromium/Blink breaks
  // JetBrains Mono ligatures inside <input>, so the mirror renders the typed
  // runes. Slot 1 additionally carries the `quest-token-input` id.
  const slotInputs = Array.from(document.querySelectorAll('input.slot-real'))
    .sort((a, b) => Number(a.dataset.slotIndex) - Number(b.dataset.slotIndex));
  const slotMirrors = Array.from(document.querySelectorAll('output.slot-mirror'))
    .sort((a, b) => Number(a.dataset.slotIndex) - Number(b.dataset.slotIndex));
  const mirrorBySlot = new Map(slotMirrors.map((m) => [Number(m.dataset.slotIndex), m]));

  if (!compileBtn || !tokenInput || !questIdInput || slotInputs.length === 0) return;

  // Mark every slot on both layers (hidden input + visible mirror).
  const markSlots = (cls) => {
    slotInputs.forEach((inp) => { inp.className = `slot-real ${cls}`; });
    slotMirrors.forEach((m) => {
      m.className = `slot-input runner-slot-input slot-mirror ${cls}`;
    });
  };

  // Keep mirrors in sync with their inputs; clicks on the mirror focus the
  // (invisible) real input; Enter submits the whole token tuple.
  slotInputs.forEach((inp) => {
    const mirror = mirrorBySlot.get(Number(inp.dataset.slotIndex));
    if (mirror) {
      const syncMirror = () => { mirror.value = inp.value; };
      inp.addEventListener('input', syncMirror);
      syncMirror(); // heal any SSR/JS value drift on load
      mirror.addEventListener('click', () => inp.focus());
    }
    inp.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
        e.preventDefault();
        compileRune();
      }
    });
  });

  const questId = questIdInput.value;
  const currentLang = document.body.dataset.lang || 'en';
  const questCompletedText = document.body.dataset.tQuestCompleted || 'Completed';
  const networkErrorText = document.body.dataset.tNetworkError || 'Network error while checking the rune.';

  async function compileRune() {
    // One token per slot, in slot order — the API validates the tuple.
    const tokens = slotInputs.map((inp) => inp.value.trim());
    compileBtn.disabled = true;

    try {
      const resp = await fetch(`/api/quest/validate?lang=${encodeURIComponent(currentLang)}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ quest_id: questId, tokens }),
      });
      const data = await resp.json();

      if (data.correct) {
        markSlots('correct');
        if (feedbackBox) {
          feedbackBox.hidden = false;
          feedbackBox.className = 'runner-feedback-box success';
          feedbackBox.innerHTML = `✨ <span>${data.message}</span>`;
        }
        if (mascotSpeech) {
          mascotSpeech.textContent = data.mascot_response;
        }
        // Correct rune ⇒ calm cozy Ferris returns.
        setMascotMood('cozy', 'runner-mascot-cozy', 'runner-mascot-grump');
        if (nextQuestBtn && data.next_quest_id) {
          nextQuestBtn.hidden = false;
          nextQuestBtn.href = `/quest/${data.next_quest_id}?lang=${encodeURIComponent(currentLang)}`;
        }
        if (sidebarStatus) {
          sidebarStatus.textContent = `✅ ${questCompletedText}`;
        }
        // Update XP counter with scaling bounce
        if (xpCounter && typeof data.total_xp === 'number') {
          xpCounter.textContent = data.total_xp;
          const badge = xpCounter.parentElement;
          if (badge) {
            badge.style.transform = 'scale(1.25)';
            setTimeout(() => { badge.style.transform = ''; }, 350);
          }
        }

        // Show floating toasts for newly unlocked achievements
        if (Array.isArray(data.newly_unlocked_achievements) && data.newly_unlocked_achievements.length > 0) {
          data.newly_unlocked_achievements.forEach((ach, i) => {
            setTimeout(() => {
              showAchievementToast(ach.icon, ach.title, ach.xp_bonus);
            }, i * 1200);
          });
        }
      } else {
        markSlots('wrong');
        if (feedbackBox) {
          feedbackBox.hidden = false;
          feedbackBox.className = 'runner-feedback-box error';
          feedbackBox.innerHTML = `⚠️ <span>${data.message}</span>`;
        }
        if (mascotSpeech) {
          mascotSpeech.textContent = data.mascot_response;
        }
        // Wrong rune ⇒ the grumpy twin shows its indignation.
        setMascotMood('grumpy', 'runner-mascot-cozy', 'runner-mascot-grump');
      }
    } catch (err) {
      if (feedbackBox) {
        feedbackBox.hidden = false;
        feedbackBox.className = 'runner-feedback-box error';
        feedbackBox.innerHTML = `⚠️ <span>${networkErrorText}</span>`;
      }
    } finally {
      compileBtn.disabled = false;
    }
  }

  compileBtn.addEventListener('click', compileRune);
}


/**
 * Floating celebration toast for newly unlocked achievements.
 * The "Achievement Unlocked!" headline comes from the SSR translation
 * (body data-attribute) — never hardcoded per-language.
 */
function showAchievementToast(icon, title, xpBonus) {
  const toast = document.createElement('div');
  toast.className = 'achievement-toast';
  // Stack toasts vertically so simultaneous unlocks remain readable.
  const stackIndex = document.querySelectorAll('.achievement-toast').length;
  toast.style.bottom = `calc(2rem + ${stackIndex * 5.4}rem)`;
  toast.innerHTML = `
    <span class="ach-toast-icon">${icon}</span>
    <div class="ach-toast-info">
      <strong>🏆 ${document.body.dataset.tAchToast || 'Achievement Unlocked!'}</strong>
      <span>${title} (+${xpBonus} XP)</span>
    </div>
  `;
  document.body.appendChild(toast);

  setTimeout(() => {
    toast.style.opacity = '0';
    toast.style.transform = 'translateY(20px)';
    toast.style.transition = 'all 0.5s ease';
    setTimeout(() => {
      toast.remove();
    }, 500);
  }, 4500);
}


/* -------------------------------------------------------------
 * 7. GLOBAL ACHIEVEMENT TOAST PIPELINE (any page)
 * ------------------------------------------------------------- */

let achDrainInFlight = false;

/**
 * Boot the global pipeline: one drain shortly after every page load (catches
 * achievements granted between visits / by SSR actions) plus a periodic poll
 * so unlocks become visible without a manual reload.
 */
function initGlobalAchievements() {
  setTimeout(() => { drainAchievements(); }, 400);
  setInterval(() => { drainAchievements(); }, 25000);
}

/**
 * Drain all achievements that were unlocked (notified = 0) for the current
 * session and show them as sequential glowing toasts. Single-flight guarded.
 * After showing a batch it re-checks once more — a bonus XP award may itself
 * have crossed another achievement threshold ("there may be more").
 */
async function drainAchievements(depth = 0) {
  if (achDrainInFlight) return;
  achDrainInFlight = true;

  let shown = 0;
  try {
    const lang = document.body.dataset.lang || 'en';
    const resp = await fetch(`/api/achievements/pending?lang=${encodeURIComponent(lang)}`);
    const data = await resp.json();
    const list = Array.isArray(data.achievements) ? data.achievements : [];

    for (const ach of list) {
      showAchievementToast(ach.icon, ach.title, ach.xp_bonus);
      shown += 1;
      // Pace the toasts so each one is readable before the next appears.
      await new Promise(resolve => setTimeout(resolve, 1400));
    }
  } catch {
    // Toasts are decorative — silently tolerate network/envelope errors.
  } finally {
    achDrainInFlight = false;
  }

  if (shown > 0 && depth < 3) {
    await drainAchievements(depth + 1);
  }
}


/* -------------------------------------------------------------
 * 8. MASCOT MOOD SWAP (wrong answer ⇒ grumpy Ferris)
 *    Every mascot home has a cozy <svg> and a grumpy twin; flipping
 *    their [hidden] attribute swaps the mood. Both include the same
 *    SVG partials, so the swap is instant (no fetching).
 * ------------------------------------------------------------- */

function setMascotMood(mood, cozyId, grumpId) {
  const cozy = document.getElementById(cozyId);
  const grump = document.getElementById(grumpId);
  if (!cozy || !grump) return;
  const grumpy = mood === 'grumpy';
  cozy.hidden = grumpy;
  grump.hidden = !grumpy;
}

/* -------------------------------------------------------------
 * 9. SKY PARALLAX — the moon/sun orb drifts up slowly on scroll
 *    (about 4–8x slower than the scroll itself; factor = 1/6,
 *    mirroring the reference site's dreamy night sky).
 *    rAF-throttled translate3d; honours prefers-reduced-motion.
 * ------------------------------------------------------------- */
function initSkyParallax() {
  const sky = document.getElementById('sky-parallax');
  if (!sky) return;
  if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;

  const SLOWDOWN = 16; // orb moves at 1/16 of the scroll speed
  let ticking = false;

  const apply = () => {
    ticking = false;
    sky.style.transform = `translate3d(0, ${-(window.scrollY / SLOWDOWN)}px, 0)`;
  };

  window.addEventListener('scroll', () => {
    if (!ticking) {
      ticking = true;
      requestAnimationFrame(apply);
    }
  }, { passive: true });

  apply();
}

/* -------------------------------------------------------------
 * 10. SCROLL PROGRESS — the 8px gradient strip on top of the
 *     sticky header (see .scroll-progress in layout.css).
 *     rAF-throttled scaleX 0 → 1 across the whole page.
 * ------------------------------------------------------------- */
function initScrollProgress() {
  const bar = document.getElementById('scroll-progress');
  if (!bar) return;

  let ticking = false;
  const apply = () => {
    ticking = false;
    const doc = document.documentElement;
    const max = doc.scrollHeight - window.innerHeight;
    const p = max > 0 ? Math.min(1, Math.max(0, window.scrollY / max)) : 0;
    bar.style.transform = `scaleX(${p})`;
  };

  window.addEventListener('scroll', () => {
    if (!ticking) {
      ticking = true;
      requestAnimationFrame(apply);
    }
  }, { passive: true });
  window.addEventListener('resize', apply);
  window.addEventListener('load', apply);

  apply();
}

/* -------------------------------------------------------------
 * 11. LIGATURE PLAYGROUND (home page, fonts section)
 *     - .lig-typer rows type their sequence character by character
 *       so the JetBrains Mono ligature snaps in when the last
 *       char lands; then it clears and types again.
 *     - #lig-play-* try-it-yourself field is the quest-runner mirror
 *       trick again: an invisible real <input> (value holder, 32 chars
 *       max) plus a visible <output> mirror that renders the
 *       ligatures. Chromium breaks ligatures inside <input> and
 *       [contenteditable] — but not inside <output>.
 *     - #home-open-register opens the register modal.
 * ------------------------------------------------------------- */
function initLigaturePlayground() {
  const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

  // --- live typing rows ---
  const typers = Array.from(document.querySelectorAll('.lig-typer'));
  if (typers.length) {
    if (reduced) {
      typers.forEach(t => {
        const out = t.querySelector('.lig-typed');
        if (out) out.textContent = t.dataset.seq || '';
      });
    } else {
      typers.forEach((t, i) => runLigTyper(t, i * 380));
    }
  }

  // --- try-it-yourself field (invisible input + visible output mirror) ---
  const playForm = document.getElementById('lig-play-form');
  const playInput = document.getElementById('lig-play-input');
  const playMirror = document.getElementById('lig-play-mirror');
  const playCount = document.getElementById('lig-play-count');

  if (playInput && playMirror) {
    const MAX = 32;

    const syncPlay = () => {
      // maxlength=32 already truncates typed & pasted text at the browser
      // level; this is a belt-and-suspenders trim for programmatic input.
      if (playInput.value.length > MAX) {
        playInput.value = playInput.value.slice(0, MAX);
      }
      // <output>.value reflects/sets its text content — the mirror renders
      // the ligatures Chromium would break.
      playMirror.value = playInput.value;
      if (playCount) playCount.textContent = `${playInput.value.length} / ${MAX}`;
    };

    playInput.addEventListener('input', syncPlay);
    // A click into the (invisible) field must land the caret in the input.
    playMirror.addEventListener('click', () => playInput.focus());
    // Enter must not submit the decorative form (no server endpoint here).
    playInput.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') e.preventDefault();
    });
    playForm?.addEventListener('submit', (e) => e.preventDefault());
    syncPlay(); // heal any SSR/JS value drift on load
  }

  // --- register button in the play section opens the real modal ---
  document.getElementById('home-open-register')?.addEventListener('click', () => {
    const m = document.getElementById('modal-register');
    if (m) {
      m.hidden = false;
      m.style.display = '';
      const first = m.querySelector('input, button');
      if (first) setTimeout(() => first.focus(), 50);
    }
  });
}

function runLigTyper(el, startDelay) {
  const seq = el.dataset.seq || '';
  const pause = parseInt(el.dataset.pause || '1600', 10);
  const out = el.querySelector('.lig-typed');
  if (!out) return;
  let i = 0;

  setTimeout(function type() {
    if (i < seq.length) {
      i += 1;
      out.textContent = seq.slice(0, i);
      setTimeout(type, 230 + Math.random() * 170);
    } else {
      setTimeout(() => {
        i = 0;
        out.textContent = '';
        setTimeout(type, 420);
      }, pause);
    }
  }, startDelay);
}

/* -------------------------------------------------------------
 * 12. ACCOUNT CABINET (/account) — 2-step forms over the API
 *     * nickname: always editable (never unique)
 *     * email: request code → confirm code (unique, verified)
 *     * password: current password required, else → /recovery
 *     * other devices: merge their progress, then sign them out
 * ------------------------------------------------------------- */
function initAccountUI() {
  const lang = document.body.dataset.lang || 'en';

  const setStatus = (el, msg, kind) => {
    if (!el) return;
    el.textContent = msg;
    el.className = `account-status ${kind}`;
    el.hidden = false;
  };
  const clearStatus = (el) => {
    if (el) { el.textContent = ''; el.hidden = true; }
  };

  // --- Nickname -------------------------------------------------
  const formNickname = document.getElementById('form-nickname');
  const nicknameStatus = document.getElementById('nickname-status');

  formNickname?.addEventListener('submit', async (e) => {
    e.preventDefault();
    clearStatus(nicknameStatus);

    const nickname = document.getElementById('input-nickname')?.value.trim() || '';
    if (!nickname) {
      setStatus(nicknameStatus, 'Please choose a nickname (1–32 characters).', 'err');
      return;
    }

    const btn = formNickname.querySelector('[type="submit"]');
    btn.disabled = true;
    try {
      const resp = await fetch('/api/auth/account/nickname', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ nickname }),
      });
      const data = await resp.json();
      if (data.ok) {
        setStatus(nicknameStatus, document.body.dataset.tAccountSaved || 'Saved!', 'ok');
      } else {
        setStatus(nicknameStatus, data.error || 'Failed to save.', 'err');
      }
    } catch {
      setStatus(nicknameStatus, 'Network error. Please try again.', 'err');
    } finally {
      btn.disabled = false;
    }
  });

  // --- Email change (2 steps) -----------------------------------
  const formEmail1 = document.getElementById('form-email-1');
  const formEmail2 = document.getElementById('form-email-2');
  const emailStep1 = document.getElementById('email-step-1');
  const emailStep2 = document.getElementById('email-step-2');
  const emailStatus1 = document.getElementById('email-status-1');
  const emailStatus2 = document.getElementById('email-status-2');
  let pendingNewEmail = '';

  formEmail1?.addEventListener('submit', async (e) => {
    e.preventDefault();
    clearStatus(emailStatus1);

    const newEmail = document.getElementById('input-email-new')?.value.trim() || '';
    if (!newEmail) return;
    pendingNewEmail = newEmail;

    const btn = formEmail1.querySelector('[type="submit"]');
    btn.disabled = true;
    try {
      const resp = await fetch('/api/auth/account/email-request', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ new_email: newEmail }),
      });
      const data = await resp.json();
      if (data.ok) {
        emailStep1.hidden = true;
        emailStep2.hidden = false;
        const codeInput = document.getElementById('input-email-code');
        if (codeInput) setTimeout(() => codeInput.focus(), 50);
      } else {
        setStatus(emailStatus1, data.error || 'Failed to send the code.', 'err');
      }
    } catch {
      setStatus(emailStatus1, 'Network error. Please try again.', 'err');
    } finally {
      btn.disabled = false;
    }
  });

  formEmail2?.addEventListener('submit', async (e) => {
    e.preventDefault();
    clearStatus(emailStatus2);

    const code = document.getElementById('input-email-code')?.value.trim() || '';
    if (code.length !== 6) {
      setStatus(emailStatus2, 'Please enter the 6-digit code.', 'err');
      return;
    }

    const btn = formEmail2.querySelector('[type="submit"]');
    btn.disabled = true;
    try {
      const resp = await fetch('/api/auth/account/email-confirm', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ new_email: pendingNewEmail, code }),
      });
      const data = await resp.json();
      if (data.ok) {
        // Full reload: the dock/title now shows the new email.
        window.location.reload();
      } else {
        setStatus(emailStatus2, data.error || 'Invalid code.', 'err');
      }
    } catch {
      setStatus(emailStatus2, 'Network error. Please try again.', 'err');
    } finally {
      btn.disabled = false;
    }
  });

  // --- Password change -------------------------------------------
  const formPassword = document.getElementById('form-password');
  const passwordStatus = document.getElementById('password-status');

  formPassword?.addEventListener('submit', async (e) => {
    e.preventDefault();
    clearStatus(passwordStatus);

    const oldPassword = document.getElementById('input-password-old')?.value || '';
    const newPassword = document.getElementById('input-password-new')?.value || '';
    if (newPassword.length < 8) {
      setStatus(passwordStatus, 'The new password must be at least 8 characters.', 'err');
      return;
    }

    const btn = formPassword.querySelector('[type="submit"]');
    btn.disabled = true;
    try {
      const resp = await fetch('/api/auth/account/password', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ old_password: oldPassword, new_password: newPassword }),
      });
      const data = await resp.json();
      if (data.ok) {
        setStatus(passwordStatus, document.body.dataset.tAccountSaved || 'Saved!', 'ok');
        formPassword.reset();
      } else {
        setStatus(passwordStatus, data.error || 'Failed to change the password.', 'err');
      }
    } catch {
      setStatus(passwordStatus, 'Network error. Please try again.', 'err');
    } finally {
      btn.disabled = false;
    }
  });

  // --- Sign out other devices -------------------------------------
  const btnOthers = document.getElementById('btn-signout-others');
  const sessionsStatus = document.getElementById('sessions-status');

  btnOthers?.addEventListener('click', async () => {
    clearStatus(sessionsStatus);
    btnOthers.disabled = true;
    try {
      const resp = await fetch('/api/auth/account/signout-others', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({}),
      });
      const data = await resp.json();
      if (data.ok) {
        // Progress from the kicked sessions is merged into THIS session.
        window.location.reload();
      } else {
        setStatus(sessionsStatus, data.error || 'Failed to sign out other devices.', 'err');
      }
    } catch {
      setStatus(sessionsStatus, 'Network error. Please try again.', 'err');
    } finally {
      btnOthers.disabled = false;
    }
  });
}

/* -------------------------------------------------------------
 * 13. PASSWORD RECOVERY (/recovery) — 2-step ritual
 *     step 1: email → code (purpose = 'recovery')
 *     step 2: code + new password → done panel
 * ------------------------------------------------------------- */
function initRecoveryUI() {
  const form1 = document.getElementById('form-recovery-1');
  const form2 = document.getElementById('form-recovery-2');
  if (!form1 && !form2) return;

  const step1 = document.getElementById('recovery-step-1');
  const step2 = document.getElementById('recovery-step-2');
  const successCard = document.getElementById('recovery-success-card');
  const status1 = document.getElementById('recovery-status-1');
  const status2 = document.getElementById('recovery-status-2');
  const speechBubble = document.querySelector('.recovery-container .mascot-text');

  const setStatus = (el, msg, kind) => {
    if (!el) return;
    el.textContent = msg;
    el.className = `account-status ${kind}`;
    el.hidden = false;
  };
  const clearStatus = (el) => {
    if (el) { el.textContent = ''; el.hidden = true; }
  };

  let pendingEmail = '';

  // Step 1 — request the code
  form1?.addEventListener('submit', async (e) => {
    e.preventDefault();
    clearStatus(status1);

    const email = document.getElementById('input-recovery-email')?.value.trim() || '';
    if (!email) return;
    pendingEmail = email;

    const btn = form1.querySelector('[type="submit"]');
    btn.disabled = true;
    try {
      const resp = await fetch('/api/auth/recovery-request', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email }),
      });
      const data = await resp.json();
      if (data.ok) {
        step1.hidden = true;
        step2.hidden = false;
        if (speechBubble) speechBubble.textContent = document.body.dataset.tRecoveryStep2 || '';
        const codeInput = document.getElementById('input-recovery-code');
        if (codeInput) setTimeout(() => codeInput.focus(), 50);
      } else {
        setStatus(status1, data.error || 'Failed to send the code.', 'err');
      }
    } catch {
      setStatus(status1, 'Network error. Please try again.', 'err');
    } finally {
      btn.disabled = false;
    }
  });

  // Step 2 — code + new password
  form2?.addEventListener('submit', async (e) => {
    e.preventDefault();
    clearStatus(status2);

    const code = document.getElementById('input-recovery-code')?.value.trim() || '';
    const password = document.getElementById('input-recovery-password')?.value || '';
    if (code.length !== 6) {
      setStatus(status2, 'Please enter the 6-digit code.', 'err');
      return;
    }
    if (password.length < 8) {
      setStatus(status2, 'The new password must be at least 8 characters.', 'err');
      return;
    }

    const btn = form2.querySelector('[type="submit"]');
    btn.disabled = true;
    try {
      const resp = await fetch('/api/auth/recovery-confirm', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: pendingEmail, code, password }),
      });
      const data = await resp.json();
      if (data.ok) {
        step2.hidden = true;
        successCard.hidden = false;
        if (speechBubble) speechBubble.textContent = '🦀: 🌸';
      } else {
        setStatus(status2, data.error || 'Invalid code.', 'err');
      }
    } catch {
      setStatus(status2, 'Network error. Please try again.', 'err');
    } finally {
      btn.disabled = false;
    }
  });
}
