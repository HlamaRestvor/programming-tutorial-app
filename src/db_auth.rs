//! Database helpers specific to authentication: users, confirmation codes,
//! session↔account linking. Brevo-only era (Google OAuth removed).
//!
//! Session architecture (v2 — strict separation):
//! * Guest sessions carry `user_id = NULL` and belong to the cookie only.
//! * Registering / logging in ALWAYS plants a brand-new session linked to
//!   the user (`user_id` set). The guest session that was active before is
//!   left untouched — guest and account progress never mix.
//! * Logout does NOT mutate the account session row: it only issues a fresh
//!   guest cookie. Re-login aggregates stats across all of the user's
//!   sessions, so the account garden is always where you left it.
//! * Password recovery and e-mail change reuse `email_confirmations` with a
//!   `purpose` column: 'register' | 'recovery' | 'email_change:{user_id}'.

use chrono::Utc;
use serde::Serialize;
use sqlx::{Row, SqlitePool};

/// A registered user row.
#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    /// Non-unique display name (chosen at registration, editable in /account).
    pub nickname: Option<String>,
    pub password_hash: Option<String>,
    pub email_verified: bool,
}

// ---------------------------------------------------------------------------
// Schema extension – called from db::init_db after the base tables.
// ---------------------------------------------------------------------------

/// Apply auth-specific migrations (users, email_confirmations and the
/// nullable `user_id` column on `user_sessions`).
pub async fn apply_auth_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Users table (nickname is NOT unique — it's a display name)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id               INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            email            TEXT    UNIQUE NOT NULL,
            nickname         TEXT,
            password_hash    TEXT,
            email_verified   INTEGER NOT NULL DEFAULT 0,
            created_at       TEXT    NOT NULL,
            last_login_at    TEXT    NOT NULL
        )",
    ).execute(pool).await?;

    // Column backfills for pre-existing databases
    let user_cols: Vec<String> = sqlx::query("PRAGMA table_info(users)")
        .fetch_all(pool)
        .await?
        .into_iter()
        .filter_map(|r| r.try_get::<String, _>("name").ok())
        .collect();
    if !user_cols.contains(&"nickname".to_string()) {
        sqlx::query("ALTER TABLE users ADD COLUMN nickname TEXT")
            .execute(pool)
            .await?;
    }

    // Email confirmation codes.
    // `purpose`: 'register' | 'recovery' | 'email_change:{user_id}' —
    // codes of different purposes never collide even for the same address.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS email_confirmations (
            id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            email         TEXT    NOT NULL,
            code          TEXT    NOT NULL,
            purpose       TEXT    NOT NULL DEFAULT 'register',
            nickname      TEXT,
            password_hash TEXT,
            expires_at    TEXT    NOT NULL,
            used          INTEGER NOT NULL DEFAULT 0
        )",
    ).execute(pool).await?;

    let conf_cols: Vec<String> = sqlx::query("PRAGMA table_info(email_confirmations)")
        .fetch_all(pool)
        .await?
        .into_iter()
        .filter_map(|r| r.try_get::<String, _>("name").ok())
        .collect();
    for (col, ddl) in [
        ("password_hash", "ALTER TABLE email_confirmations ADD COLUMN password_hash TEXT"),
        ("nickname", "ALTER TABLE email_confirmations ADD COLUMN nickname TEXT"),
        ("purpose", "ALTER TABLE email_confirmations ADD COLUMN purpose TEXT NOT NULL DEFAULT 'register'"),
    ] {
        if !conf_cols.contains(&col.to_string()) {
            sqlx::query(ddl).execute(pool).await?;
        }
    }

    // Add user_id column to user_sessions if not yet present.
    let cols: Vec<String> = sqlx::query("PRAGMA table_info(user_sessions)")
        .fetch_all(pool)
        .await?
        .into_iter()
        .filter_map(|r| r.try_get::<String, _>("name").ok())
        .collect();

    if !cols.contains(&"user_id".to_string()) {
        sqlx::query("ALTER TABLE user_sessions ADD COLUMN user_id INTEGER")
            .execute(pool)
            .await?;
    }

    tracing::info!("Auth migrations applied.");
    Ok(())
}

// ---------------------------------------------------------------------------
// User CRUD
// ---------------------------------------------------------------------------

/// Insert a new VERIFIED user (the account is created only after the email
/// confirmation code succeeds). Returns its `id`.
pub async fn create_verified_user(
    pool: &SqlitePool,
    email: &str,
    password_hash: Option<&str>,
    nickname: Option<&str>,
) -> Result<i64, sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    let row = sqlx::query(
        "INSERT INTO users (email, nickname, password_hash, email_verified, created_at, last_login_at)
         VALUES (?, ?, ?, 1, ?, ?)
         RETURNING id",
    )
    .bind(email)
    .bind(nickname)
    .bind(password_hash)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    row.try_get::<i64, _>("id")
}

/// Remove a leftover UNVERIFIED user row (an abandoned registration
/// attempt being replaced). No session can be linked to such a row,
/// so nothing else is lost.
pub async fn delete_unverified_user(pool: &SqlitePool, email: &str) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM users WHERE email = ? AND email_verified = 0")
        .bind(email)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn find_user_by_email(pool: &SqlitePool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, email, nickname, password_hash, email_verified FROM users WHERE email = ?",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    if let Some(r) = row {
        Ok(Some(User {
            id: r.try_get::<i64, _>("id")?,
            email: r.try_get::<String, _>("email")?,
            nickname: r.try_get::<Option<String>, _>("nickname")?,
            password_hash: r.try_get::<Option<String>, _>("password_hash")?,
            email_verified: r.try_get::<i64, _>("email_verified")? != 0,
        }))
    } else { Ok(None) }
}

/// The account currently linked to the given session (None for guests).
pub async fn get_session_user(pool: &SqlitePool, session_id: &str) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT u.id, u.email, u.nickname, u.password_hash, u.email_verified
         FROM users u
         JOIN user_sessions s ON s.user_id = u.id
         WHERE s.session_id = ?",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;

    if let Some(r) = row {
        Ok(Some(User {
            id: r.try_get::<i64, _>("id")?,
            email: r.try_get::<String, _>("email")?,
            nickname: r.try_get::<Option<String>, _>("nickname")?,
            password_hash: r.try_get::<Option<String>, _>("password_hash")?,
            email_verified: r.try_get::<i64, _>("email_verified")? != 0,
        }))
    } else { Ok(None) }
}

pub async fn update_last_login(pool: &SqlitePool, user_id: i64) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE users SET last_login_at = ? WHERE id = ?")
        .bind(&now)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Change the display nickname — always allowed, never unique.
pub async fn update_user_nickname(pool: &SqlitePool, user_id: i64, nickname: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET nickname = ? WHERE id = ?")
        .bind(nickname)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Point the account at a new (pre-verified via code) e-mail address.
pub async fn update_user_email(pool: &SqlitePool, user_id: i64, new_email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET email = ? WHERE id = ?")
        .bind(new_email)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Replace the account password hash (recovery flow or /account change).
pub async fn update_user_password(pool: &SqlitePool, user_id: i64, password_hash: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
        .bind(password_hash)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Email confirmation codes
// ---------------------------------------------------------------------------

/// Store a confirmation code. Any previous unused codes for the SAME
/// (email, purpose) pair are invalidated first — so when a user requests
/// several codes in a row, the LAST one is the only valid one.
pub async fn create_confirmation(
    pool: &SqlitePool,
    email: &str,
    code: &str,
    password_hash: Option<&str>,
    nickname: Option<&str>,
    purpose: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE email_confirmations SET used = 1 WHERE email = ? AND purpose = ? AND used = 0")
        .bind(email)
        .bind(purpose)
        .execute(pool)
        .await?;

    let expires_at = (Utc::now() + chrono::Duration::minutes(15)).to_rfc3339();
    sqlx::query(
        "INSERT INTO email_confirmations (email, code, purpose, nickname, password_hash, expires_at, used)
         VALUES (?, ?, ?, ?, ?, ?, 0)",
    )
    .bind(email)
    .bind(code)
    .bind(purpose)
    .bind(nickname)
    .bind(password_hash)
    .bind(&expires_at)
    .execute(pool)
    .await?;

    Ok(())
}

/// What a successfully verified code carries along.
#[derive(Debug, Clone)]
pub struct ConfirmedCode {
    /// Registration flow: the pending Argon2 hash. Recovery / e-mail change:
    /// `None` (the new value arrives with the request payload instead).
    pub password_hash: Option<String>,
    /// Registration flow: the chosen nickname.
    pub nickname: Option<String>,
}

/// Verify a confirmation code for the given (email, purpose) pair.
/// If valid, marks it used and returns what it carries.
pub async fn verify_confirmation_code(
    pool: &SqlitePool,
    email: &str,
    code: &str,
    purpose: &str,
) -> Result<Option<ConfirmedCode>, sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    let row = sqlx::query(
        "SELECT id, password_hash, nickname FROM email_confirmations
         WHERE email = ? AND code = ? AND purpose = ? AND used = 0 AND expires_at > ?
         ORDER BY id DESC
         LIMIT 1",
    )
    .bind(email)
    .bind(code)
    .bind(purpose)
    .bind(&now)
    .fetch_optional(pool)
    .await?;

    if let Some(r) = row {
        let id: i64 = r.try_get("id")?;
        let hash: Option<String> = r.try_get("password_hash")?;
        let nickname: Option<String> = r.try_get("nickname")?;
        sqlx::query("UPDATE email_confirmations SET used = 1 WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        return Ok(Some(ConfirmedCode { password_hash: hash, nickname }));
    }
    Ok(None)
}

// ---------------------------------------------------------------------------
// Session ↔ account linking
// ---------------------------------------------------------------------------

/// Link a session row to the user account. Account sessions are always
/// FRESH rows created for this login/registration — never a reused guest
/// session, so guest and account progress never mix.
pub async fn link_session_to_user(
    pool: &SqlitePool,
    session_id: &str,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE user_sessions SET user_id = ? WHERE session_id = ?")
        .bind(user_id)
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// "Sign out other devices": log every OTHER session of this user out.
///
/// 1. Quest completions from the doomed sessions are merged into the
///    surviving one (deduped per quest) — no drop of earned XP is lost.
/// 2. The other `user_sessions` rows are deleted; FK cascades clear their
///    `user_stats` and duplicate `quest_completions` rows.
///
/// Achievements are keyed `user:{uid}` (never per session), so they survive.
/// The other devices' cookies now point at nonexistent sessions → their very
/// next request plants fresh guest sessions. Returns deleted session count.
pub async fn sign_out_other_sessions(
    pool: &SqlitePool,
    current_session_id: &str,
    user_id: i64,
) -> Result<u64, sqlx::Error> {
    sqlx::query(
        "INSERT OR IGNORE INTO quest_completions (session_id, quest_id, xp_awarded, completed_at)
         SELECT ?, quest_id, xp_awarded, completed_at
         FROM quest_completions
         WHERE session_id IN (SELECT session_id FROM user_sessions WHERE user_id = ? AND session_id != ?)",
    )
    .bind(current_session_id)
    .bind(user_id)
    .bind(current_session_id)
    .execute(pool)
    .await?;

    let res = sqlx::query("DELETE FROM user_sessions WHERE user_id = ? AND session_id != ?")
        .bind(user_id)
        .bind(current_session_id)
        .execute(pool)
        .await?;

    Ok(res.rows_affected())
}
