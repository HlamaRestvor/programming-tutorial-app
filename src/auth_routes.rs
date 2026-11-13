//! Auth API + page handlers (Brevo-only era; Google OAuth removed).
//!
//! Session architecture (v2 — strict separation):
//! * **Register / login ALWAYS plant a brand-new session** linked to the
//!   user. The guest session that was active before is never reused, and
//!   guest progress is NEVER migrated — guest and account gardens stay
//!   strictly apart (there is no "keep progress" checkbox anymore).
//! * **Logout does not touch the account session row** — it merely issues a
//!   fresh guest cookie. Consistency comes from aggregation in
//!   `get_user_stats` / `get_leaderboard` over ALL sessions of the user.
//! * Confirmation codes are scoped by `purpose`:
//!   'register' | 'recovery' | 'email_change:{user_id}' — and the newest
//!   code always invalidates the previous ones of the same purpose.

use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::auth;
use crate::config;
use crate::db;
use crate::db_auth::{self, User};
use crate::i18n;
use crate::routes::HomeQuery;
use crate::state::AppState;

const SESSION_COOKIE_NAME: &str = "fg_session";

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

/// Build Set-Cookie header value for a persistent session (same contract as
/// routes.rs; duplicated here so both modules stay dependency-free).
fn make_session_cookie(session_id: &str) -> HeaderValue {
    let cookie_str = format!(
        "{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=31536000",
        SESSION_COOKIE_NAME, session_id
    );
    HeaderValue::from_str(&cookie_str).unwrap_or_else(|_| {
        HeaderValue::from_static("fg_session=anonymous; Path=/")
    })
}

/// `{ok: true}` — the only success shape the frontend understands.
fn api_ok() -> Response {
    Json(json!({ "ok": true })).into_response()
}

/// `{ok: false, error: "…"}` — shown verbatim in the modal error boxes.
fn api_err(status: StatusCode, msg: &str) -> Response {
    (status, Json(json!({ "ok": false, "error": msg }))).into_response()
}

/// Minimal e-mail sanity check (the code is the real proof of ownership).
fn valid_email(email: &str) -> bool {
    let e = email.trim();
    let (local, domain) = match e.rsplit_once('@') {
        Some(pair) => pair,
        None => return false,
    };
    !local.is_empty()
        && !domain.is_empty()
        && domain.contains('.')
        && !e.chars().any(char::is_whitespace)
        && e.len() <= 254
}

/// Nickname rules: 1..=32 visible characters after trimming.
fn valid_nickname(nickname: &str) -> bool {
    let n = nickname.trim();
    !n.is_empty() && n.chars().count() <= 32
}

/// Resolve the account linked to the caller's session cookie.
/// Returns `(session_id, user)` or a 401 JSON response.
async fn require_session_user(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<(String, User), Response> {
    let sid = crate::routes::extract_session_id(headers)
        .ok_or_else(|| api_err(StatusCode::UNAUTHORIZED, "Please sign in first."))?;

    let user = db_auth::get_session_user(&state.db, &sid)
        .await
        .map_err(|e| {
            tracing::error!("get_session_user error: {e}");
            api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.")
        })?
        .ok_or_else(|| api_err(StatusCode::UNAUTHORIZED, "Please sign in first."))?;

    Ok((sid, user))
}

// ---------------------------------------------------------------------------
// POST /api/auth/register  — step 1: nickname + email + password
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub nickname: String,
    pub email: String,
    pub password: String,
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Response {
    let nickname = payload.nickname.trim().to_string();
    let email = payload.email.trim().to_lowercase();

    if !valid_nickname(&nickname) {
        return api_err(StatusCode::BAD_REQUEST, "Please choose a nickname (1–32 characters).");
    }
    if !valid_email(&email) {
        return api_err(StatusCode::BAD_REQUEST, "Please enter a valid email address.");
    }
    if payload.password.len() < 8 {
        return api_err(StatusCode::BAD_REQUEST, "Password must be at least 8 characters.");
    }

    // Already a live account? -> refuse. A leftover UNVERIFIED row from an
    // abandoned earlier attempt is silently replaced (no session can be
    // linked to it, so nothing else is lost).
    match db_auth::find_user_by_email(&state.db, &email).await {
        Ok(Some(u)) if u.email_verified => {
            return api_err(StatusCode::CONFLICT, "This email is already registered. Try signing in instead.");
        }
        Ok(Some(_)) => {
            if let Err(e) = db_auth::delete_unverified_user(&state.db, &email).await {
                tracing::error!("delete_unverified_user error: {e}");
                return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
            }
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!("find_user_by_email error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    }

    // The password hash travels inside the confirmation row — the account
    // row is created only when the code is verified (deferred creation).
    let password_hash = match auth::hash_password(&payload.password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("hash_password error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Could not hash the password.");
        }
    };
    let code = auth::generate_confirmation_code();

    if let Err(e) = db_auth::create_confirmation(
        &state.db, &email, &code, Some(&password_hash), Some(&nickname), "register",
    ).await {
        tracing::error!("create_confirmation error: {e}");
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
    }

    if let Err(e) = auth::send_confirmation_email(&email, &code, &state.brevo_api_key).await {
        tracing::error!("Brevo send error: {e}");
        return api_err(StatusCode::BAD_GATEWAY, "Could not send the email. Please try again in a minute.");
    }

    api_ok()
}

// ---------------------------------------------------------------------------
// POST /api/auth/verify-email  — step 2: the account materializes here
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
}

pub async fn verify_email_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Response {
    let email = payload.email.trim().to_lowercase();
    let code = payload.code.trim().to_string();

    let confirmed = match db_auth::verify_confirmation_code(&state.db, &email, &code, "register").await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("verify_confirmation_code error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };
    let Some(confirmed) = confirmed else {
        // Expired, already used, or not the newest code — the modal hint
        // tells the user to take the code from the FRESHEST email.
        return api_err(StatusCode::BAD_REQUEST, "Invalid or expired code. Use the code from the newest email.");
    };

    // A verified user row must not exist yet (register_handler guards this,
    // but double-check against races).
    if let Err(e) = db_auth::delete_unverified_user(&state.db, &email).await {
        tracing::error!("delete_unverified_user error: {e}");
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
    }

    let user_id = match db_auth::create_verified_user(
        &state.db, &email, confirmed.password_hash.as_deref(), confirmed.nickname.as_deref(),
    ).await {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("create_verified_user error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };

    // 🏡 Garden Founder — granted at the exact moment the account is planted.
    let ident = format!("user:{user_id}");
    if let Err(e) = db::award_achievement_for_identifier(&state.db, &ident, "garden_founder").await {
        tracing::error!("award garden_founder error: {e}");
    }

    // STRICT SEPARATION: a brand-new session is planted for the account.
    // The previous guest session (and its progress) stays untouched.
    let (new_sid, _stats) = match db::get_or_create_session(&state.db, None).await {
        Ok(pair) => pair,
        Err(e) => {
            tracing::error!("get_or_create_session error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };
    if let Err(e) = db_auth::link_session_to_user(&state.db, &new_sid, user_id).await {
        tracing::error!("link_session_to_user error: {e}");
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
    }

    // Run the eligibility engine once against the (now linked) session so
    // consecrated_traveler etc. unlock immediately. We deliberately do NOT
    // mark them notified here — the global toast pipeline presents the
    // 🏡 Garden Founder (and any other fresh ones) on the next page load.
    match db::sync_and_award_achievements(&state.db, &new_sid, false).await {
        Ok(newly) if !newly.is_empty() => {
            let ids: Vec<&str> = newly.iter().map(|a| a.id).collect();
            tracing::info!("Achievements unlocked at registration: {ids:?}");
        }
        Ok(_) => {}
        Err(e) => tracing::error!("sync_and_award_achievements error: {e}"),
    }

    let mut resp = api_ok();
    resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&new_sid));
    resp
}

// ---------------------------------------------------------------------------
// POST /api/auth/login
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    let email = payload.email.trim().to_lowercase();

    let user = match db_auth::find_user_by_email(&state.db, &email).await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("find_user_by_email error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };
    let Some(user) = user else {
        return api_err(StatusCode::BAD_REQUEST, "No account with this email.");
    };
    let Some(hash) = user.password_hash.as_deref() else {
        return api_err(StatusCode::BAD_REQUEST, "This account has no password set. Use password recovery.");
    };
    match auth::verify_password(&payload.password, hash) {
        Ok(true) => {}
        Ok(false) => return api_err(StatusCode::BAD_REQUEST, "Incorrect password."),
        Err(e) => {
            tracing::error!("verify_password error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Could not verify the password.");
        }
    }

    // Login is ALWAYS a fresh account session — never a reused guest one.
    let (new_sid, _stats) = match db::get_or_create_session(&state.db, None).await {
        Ok(pair) => pair,
        Err(e) => {
            tracing::error!("get_or_create_session error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };
    if let Err(e) = db_auth::link_session_to_user(&state.db, &new_sid, user.id).await {
        tracing::error!("link_session_to_user error: {e}");
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
    }
    if let Err(e) = db_auth::update_last_login(&state.db, user.id).await {
        tracing::error!("update_last_login error: {e}");
    }

    let mut resp = api_ok();
    resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&new_sid));
    resp
}

// ---------------------------------------------------------------------------
// POST /api/auth/logout — fresh guest cookie; the account session is left
// intact in the DB (NO unlink!), so its progress keeps aggregating forever.
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, Default)]
pub struct LogoutRequest {}

pub async fn logout_handler(
    State(state): State<AppState>,
    Json(_payload): Json<LogoutRequest>,
) -> Response {
    // Passing None ignores the incoming cookie and plants a fresh guest
    // session + issues its cookie. The old account session row keeps its
    // user_id — it is simply not referenced by this browser anymore.
    let (fresh_sid, _stats) = match db::get_or_create_session(&state.db, None).await {
        Ok(pair) => pair,
        Err(e) => {
            tracing::error!("get_or_create_session(logout) error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };

    let mut resp = api_ok();
    resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&fresh_sid));
    resp
}

// ---------------------------------------------------------------------------
// GET /recovery — standalone recovery page (guests & users alike)
// ---------------------------------------------------------------------------

pub async fn recovery_page_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HomeQuery>,
) -> Response {
    let lang = params.lang.unwrap_or_else(|| i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let existing_sid = crate::routes::extract_session_id(&headers);
    let (session_id, user_stats) = match db::get_or_create_session(&state.db, existing_sid.as_deref()).await {
        Ok(pair) => pair,
        Err(e) => {
            let msg = crate::error_pages::db_error_message("loading the recovery page", &e);
            tracing::error!("DB error in recovery page: {e}");
            return crate::error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked", &msg, "/recovery",
            );
        }
    };

    let mut ctx = tera::Context::new();
    ctx.insert("lang", &lang);
    ctx.insert("translation", translation);
    ctx.insert("user_stats", &user_stats);
    ctx.insert("active_page", "recovery");
    ctx.insert("version", config::VERSION);

    let body = match state.tera.render("recovery.tera", &ctx) {
        Ok(html) => html,
        Err(e) => {
            tracing::error!("Template render error (recovery): {e:?}");
            return crate::error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked",
                "The recovery ritual could not be rendered. Please try again in a moment.",
                "/recovery",
            );
        }
    };

    let mut resp = Html(body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}

// ---------------------------------------------------------------------------
// POST /api/auth/recovery-request — step 1 of the ritual
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct RecoveryRequest {
    pub email: String,
}

pub async fn recovery_request_handler(
    State(state): State<AppState>,
    Json(payload): Json<RecoveryRequest>,
) -> Response {
    let email = payload.email.trim().to_lowercase();
    if !valid_email(&email) {
        return api_err(StatusCode::BAD_REQUEST, "Please enter a valid email address.");
    }

    // The account must exist — otherwise there is no key to reforge.
    let user = match db_auth::find_user_by_email(&state.db, &email).await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("find_user_by_email error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };
    let Some(_user) = user else {
        return api_err(StatusCode::BAD_REQUEST, "No account with this email.");
    };

    let code = auth::generate_confirmation_code();
    if let Err(e) = db_auth::create_confirmation(&state.db, &email, &code, None, None, "recovery").await {
        tracing::error!("create_confirmation(recovery) error: {e}");
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
    }

    if let Err(e) = auth::send_confirmation_email(&email, &code, &state.brevo_api_key).await {
        tracing::error!("Brevo send error: {e}");
        return api_err(StatusCode::BAD_GATEWAY, "Could not send the email. Please try again in a minute.");
    }

    api_ok()
}

// ---------------------------------------------------------------------------
// POST /api/auth/recovery-confirm — step 2: forge the new password
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct RecoveryConfirmRequest {
    pub email: String,
    pub code: String,
    pub password: String,
}

pub async fn recovery_confirm_handler(
    State(state): State<AppState>,
    Json(payload): Json<RecoveryConfirmRequest>,
) -> Response {
    let email = payload.email.trim().to_lowercase();
    let code = payload.code.trim().to_string();

    if payload.password.len() < 8 {
        return api_err(StatusCode::BAD_REQUEST, "Password must be at least 8 characters.");
    }

    let confirmed = match db_auth::verify_confirmation_code(&state.db, &email, &code, "recovery").await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("verify_confirmation_code(recovery) error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };
    if confirmed.is_none() {
        return api_err(StatusCode::BAD_REQUEST, "Invalid or expired code. Use the code from the newest email.");
    }

    let user = match db_auth::find_user_by_email(&state.db, &email).await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("find_user_by_email error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };
    let Some(user) = user else {
        return api_err(StatusCode::BAD_REQUEST, "No account with this email.");
    };

    let hash = match auth::hash_password(&payload.password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("hash_password error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Could not hash the password.");
        }
    };
    if let Err(e) = db_auth::update_user_password(&state.db, user.id, &hash).await {
        tracing::error!("update_user_password error: {e}");
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
    }

    api_ok()
}

// ---------------------------------------------------------------------------
// GET /account — the personal cabinet (registered users only)
// ---------------------------------------------------------------------------

pub async fn account_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HomeQuery>,
) -> Response {
    // Guests have nothing to tune here — send them to the home page.
    let Some(existing_sid) = crate::routes::extract_session_id(&headers) else {
        return Redirect::to("/").into_response();
    };

    let lang = params.lang.unwrap_or_else(|| i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let user_stats = match db::get_user_stats(&state.db, &existing_sid).await {
        Ok(s) => s,
        Err(e) => {
            let msg = crate::error_pages::db_error_message("loading your account", &e);
            tracing::error!("DB error in account page: {e}");
            return crate::error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked", &msg, "/account",
            );
        }
    };

    let user = match db_auth::get_session_user(&state.db, &existing_sid).await {
        Ok(u) => u,
        Err(e) => {
            let msg = crate::error_pages::db_error_message("loading your account", &e);
            tracing::error!("DB error in account page: {e}");
            return crate::error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked", &msg, "/account",
            );
        }
    };
    let Some(user) = user else {
        return Redirect::to("/").into_response();
    };

    let mut ctx = tera::Context::new();
    ctx.insert("lang", &lang);
    ctx.insert("translation", translation);
    ctx.insert("user_stats", &user_stats);
    ctx.insert("active_page", "account");
    ctx.insert("version", config::VERSION);
    ctx.insert("account_email", &user.email);
    ctx.insert("account_nickname", &user.nickname.unwrap_or_default());

    let body = match state.tera.render("account.tera", &ctx) {
        Ok(html) => html,
        Err(e) => {
            tracing::error!("Template render error (account): {e:?}");
            return crate::error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked",
                "Your cabinet could not be rendered. Please try again in a moment.",
                "/account",
            );
        }
    };

    Html(body).into_response()
}

// ---------------------------------------------------------------------------
// POST /api/auth/account/nickname — always allowed, never unique
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct NicknameRequest {
    pub nickname: String,
}

pub async fn account_nickname_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<NicknameRequest>,
) -> Response {
    let (_sid, user) = match require_session_user(&state, &headers).await {
        Ok(pair) => pair,
        Err(resp) => return resp,
    };

    let nickname = payload.nickname.trim().to_string();
    if !valid_nickname(&nickname) {
        return api_err(StatusCode::BAD_REQUEST, "The nickname must be 1–32 characters long.");
    }

    if let Err(e) = db_auth::update_user_nickname(&state.db, user.id, &nickname).await {
        tracing::error!("update_user_nickname error: {e}");
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
    }

    api_ok()
}

// ---------------------------------------------------------------------------
// POST /api/auth/account/email-request — code to the NEW address
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct EmailChangeRequest {
    pub new_email: String,
}

pub async fn account_email_request_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<EmailChangeRequest>,
) -> Response {
    let (_sid, user) = match require_session_user(&state, &headers).await {
        Ok(pair) => pair,
        Err(resp) => return resp,
    };

    let new_email = payload.new_email.trim().to_lowercase();
    if !valid_email(&new_email) {
        return api_err(StatusCode::BAD_REQUEST, "Please enter a valid email address.");
    }
    if new_email.eq_ignore_ascii_case(&user.email) {
        return api_err(StatusCode::BAD_REQUEST, "This is already your email.");
    }

    // Uniqueness is checked twice: here (friendly error) and at the DB
    // (UNIQUE constraint as the last line of defense).
    match db_auth::find_user_by_email(&state.db, &new_email).await {
        Ok(Some(_)) => return api_err(StatusCode::CONFLICT, "That email is already in use by another account."),
        Ok(None) => {}
        Err(e) => {
            tracing::error!("find_user_by_email error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    }

    let code = auth::generate_confirmation_code();
    let purpose = format!("email_change:{}", user.id);
    if let Err(e) = db_auth::create_confirmation(&state.db, &new_email, &code, None, None, &purpose).await {
        tracing::error!("create_confirmation(email_change) error: {e}");
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
    }

    if let Err(e) = auth::send_confirmation_email(&new_email, &code, &state.brevo_api_key).await {
        tracing::error!("Brevo send error: {e}");
        return api_err(StatusCode::BAD_GATEWAY, "Could not send the email. Please try again in a minute.");
    }

    api_ok()
}

// ---------------------------------------------------------------------------
// POST /api/auth/account/email-confirm — swap the address after the code
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct EmailConfirmRequest {
    pub new_email: String,
    pub code: String,
}

pub async fn account_email_confirm_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<EmailConfirmRequest>,
) -> Response {
    let (_sid, user) = match require_session_user(&state, &headers).await {
        Ok(pair) => pair,
        Err(resp) => return resp,
    };

    let new_email = payload.new_email.trim().to_lowercase();
    let code = payload.code.trim().to_string();
    if !valid_email(&new_email) {
        return api_err(StatusCode::BAD_REQUEST, "Please enter a valid email address.");
    }

    let purpose = format!("email_change:{}", user.id);
    let confirmed = match db_auth::verify_confirmation_code(&state.db, &new_email, &code, &purpose).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("verify_confirmation_code(email_change) error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };
    if confirmed.is_none() {
        return api_err(StatusCode::BAD_REQUEST, "Invalid or expired code. Use the code from the newest email.");
    }

    if let Err(e) = db_auth::update_user_email(&state.db, user.id, &new_email).await {
        tracing::error!("update_user_email error: {e}");
        let msg = if e.as_database_error().map(|d| d.is_unique_violation()).unwrap_or(false) {
            "That email is already in use by another account."
        } else {
            "Database error. Please try again."
        };
        return api_err(StatusCode::CONFLICT, msg);
    }

    api_ok()
}

// ---------------------------------------------------------------------------
// POST /api/auth/account/password — only with the CURRENT password known;
// otherwise the frontend points the user to /recovery.
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct PasswordChangeRequest {
    pub old_password: String,
    pub new_password: String,
}

pub async fn account_password_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<PasswordChangeRequest>,
) -> Response {
    let (_sid, user) = match require_session_user(&state, &headers).await {
        Ok(pair) => pair,
        Err(resp) => return resp,
    };

    if payload.new_password.len() < 8 {
        return api_err(StatusCode::BAD_REQUEST, "The new password must be at least 8 characters.");
    }

    let Some(hash) = user.password_hash.as_deref() else {
        return api_err(StatusCode::BAD_REQUEST,
            "This account has no password to verify. Recover it via email instead.");
    };
    match auth::verify_password(&payload.old_password, hash) {
        Ok(true) => {}
        Ok(false) => return api_err(StatusCode::BAD_REQUEST,
            "The current password is incorrect. Recover it via email instead."),
        Err(e) => {
            tracing::error!("verify_password error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Could not verify the password.");
        }
    }

    let new_hash = match auth::hash_password(&payload.new_password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("hash_password error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Could not hash the password.");
        }
    };
    if let Err(e) = db_auth::update_user_password(&state.db, user.id, &new_hash).await {
        tracing::error!("update_user_password error: {e}");
        return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
    }

    api_ok()
}

// ---------------------------------------------------------------------------
// POST /api/auth/account/signout-others — merge other sessions' progress
// into this one, then delete those sessions (FK cascade cleans up).
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, Default)]
pub struct SignOutOthersRequest {}

#[derive(Debug, Serialize)]
pub struct SignOutOthersResponse {
    pub ok: bool,
    pub signed_out: u64,
}

pub async fn account_signout_others_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(_payload): Json<SignOutOthersRequest>,
) -> Response {
    let (sid, user) = match require_session_user(&state, &headers).await {
        Ok(pair) => pair,
        Err(resp) => return resp,
    };

    let n = match db_auth::sign_out_other_sessions(&state.db, &sid, user.id).await {
        Ok(n) => n,
        Err(e) => {
            tracing::error!("sign_out_other_sessions error: {e}");
            return api_err(StatusCode::INTERNAL_SERVER_ERROR, "Database error. Please try again.");
        }
    };

    Json(SignOutOthersResponse { ok: true, signed_out: n }).into_response()
}
