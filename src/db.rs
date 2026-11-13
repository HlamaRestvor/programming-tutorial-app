use chrono::Utc;
use serde::Serialize;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Row, SqlitePool,
};
use std::str::FromStr;
use uuid::Uuid;

use crate::i18n::Translation;

#[derive(Debug, Clone, Serialize, Default)]
pub struct UserStats {
    pub session_id: String,
    pub xp: i64,
    pub completed_count: i64,
    pub has_completed_first: bool,
    pub user_email: Option<String>,
    pub completed_quest_ids: Vec<String>,
    pub unlocked_achievements_count: usize,
}

/// Initialize SQLite connection pool and apply lightweight schema migrations.
pub async fn init_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let opts = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
    let pool = SqlitePoolOptions::new()
        .max_connections(16)
        .connect_with(opts)
        .await?;

    // Create base tables
    sqlx::query("
        CREATE TABLE IF NOT EXISTS user_sessions (
            session_id     TEXT NOT NULL PRIMARY KEY,
            created_at     TEXT NOT NULL,
            last_active_at TEXT NOT NULL,
            user_id        INTEGER
        );
        CREATE TABLE IF NOT EXISTS user_stats (
            session_id TEXT PRIMARY KEY,
            xp INTEGER NOT NULL DEFAULT 0,
            completed_count INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (session_id) REFERENCES user_sessions(session_id) ON DELETE CASCADE
        );
        CREATE TABLE IF NOT EXISTS quest_completions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id TEXT NOT NULL,
            quest_id TEXT NOT NULL,
            xp_awarded INTEGER NOT NULL,
            completed_at TEXT NOT NULL,
            UNIQUE(session_id, quest_id),
            FOREIGN KEY (session_id) REFERENCES user_sessions(session_id) ON DELETE CASCADE
        );
        CREATE TABLE IF NOT EXISTS user_achievements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_identifier TEXT NOT NULL,
            achievement_id TEXT NOT NULL,
            unlocked_at TEXT NOT NULL,
            notified INTEGER NOT NULL DEFAULT 0,
            UNIQUE(user_identifier, achievement_id)
        );
    ")
    .execute(&pool)
    .await?;

    // Migration check for user_achievements.notified
    let ach_cols: Vec<String> = sqlx::query("PRAGMA table_info(user_achievements)")
        .fetch_all(&pool)
        .await?
        .into_iter()
        .filter_map(|r| r.try_get::<String, _>("name").ok())
        .collect();
    if !ach_cols.contains(&"notified".to_string()) {
        let _ = sqlx::query("ALTER TABLE user_achievements ADD COLUMN notified INTEGER NOT NULL DEFAULT 0")
            .execute(&pool)
            .await;
    }

    tracing::info!("SQLite database initialized and migrations applied.");
    Ok(pool)
}

/// Fetch or create an anonymous visitor session and its statistics.
pub async fn get_or_create_session(
    pool: &SqlitePool,
    session_id_opt: Option<&str>,
) -> Result<(String, UserStats), sqlx::Error> {
    let now = Utc::now().to_rfc3339();

    if let Some(sid) = session_id_opt {
        // Check if session exists in DB
        let row_opt = sqlx::query("SELECT session_id FROM user_sessions WHERE session_id = ?")
            .bind(sid)
            .fetch_optional(pool)
            .await?;

        if row_opt.is_some() {
            // Update last_active_at
            let _ = sqlx::query("UPDATE user_sessions SET last_active_at = ? WHERE session_id = ?")
                .bind(&now)
                .bind(sid)
                .execute(pool)
                .await;
            let stats = get_user_stats(pool, sid).await?;
            return Ok((sid.to_string(), stats));
        }
    }

    // Generate fresh session UUID
    let new_sid = Uuid::new_v4().to_string();

    sqlx::query("INSERT INTO user_sessions (session_id, created_at, last_active_at) VALUES (?, ?, ?)")
        .bind(&new_sid)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

    sqlx::query("INSERT INTO user_stats (session_id, xp, completed_count) VALUES (?, 0, 0)")
        .bind(&new_sid)
        .execute(pool)
        .await?;

    Ok((
        new_sid.clone(),
        UserStats {
            session_id: new_sid,
            xp: 0,
            completed_count: 0,
            has_completed_first: false,
            user_email: None,
            completed_quest_ids: Vec::new(),
            unlocked_achievements_count: 0,
        },
    ))
}

/// Retrieve user stats and check if quests are completed.
///
/// If the session is linked to a registered user account (`user_id`),
/// this automatically aggregates stats across all sessions for that user,
/// ensuring XP, achievements, and completed quests survive cookie clears and device switches.
pub async fn get_user_stats(pool: &SqlitePool, session_id: &str) -> Result<UserStats, sqlx::Error> {
    // Check if session is linked to an account
    let session_user_row = sqlx::query("SELECT user_id FROM user_sessions WHERE session_id = ?")
        .bind(session_id)
        .fetch_optional(pool)
        .await?;

    let user_id_opt: Option<i64> = match session_user_row {
        Some(r) => r.try_get("user_id")?,
        None => None,
    };

    if let Some(uid) = user_id_opt {
        // Authenticated user: aggregate progress across all of user's sessions
        let email: Option<String> = match sqlx::query("SELECT email FROM users WHERE id = ?")
            .bind(uid)
            .fetch_optional(pool)
            .await?
        {
            Some(r) => r.try_get("email")?,
            None => None,
        };

        let quest_rows = sqlx::query(
            "SELECT DISTINCT quest_id, xp_awarded FROM quest_completions
             WHERE session_id IN (SELECT session_id FROM user_sessions WHERE user_id = ?)",
        )
        .bind(uid)
        .fetch_all(pool)
        .await?;

        // Dedupe per quest: a quest must never be paid for twice, even if
        // parallel submissions (or legacy rows with different rewards)
        // inserted several completion rows for it.
        let mut quest_xp_by_id: std::collections::HashMap<String, i64> =
            std::collections::HashMap::new();
        for r in quest_rows {
            if let Ok(qid) = r.try_get::<String, _>("quest_id") {
                let xp = r.try_get::<i64, _>("xp_awarded").unwrap_or(0);
                let best = quest_xp_by_id.entry(qid).or_insert(0);
                if xp > *best {
                    *best = xp;
                }
            }
        }
        let completed_quest_ids: Vec<String> = quest_xp_by_id.keys().cloned().collect();
        let quest_xp_total: i64 = quest_xp_by_id.values().sum();

        let completed_count = completed_quest_ids.len() as i64;
        let has_completed_first = completed_quest_ids.iter().any(|q| q == "rust-01-mutability");

        // Include achievement bonus XP
        let user_ident = format!("user:{}", uid);
        let ach_rows = sqlx::query("SELECT achievement_id FROM user_achievements WHERE user_identifier = ?")
            .bind(&user_ident)
            .fetch_all(pool)
            .await?;

        let mut ach_bonus: i64 = 0;
        let unlocked_achievements_count = ach_rows.len();
        for r in ach_rows {
            if let Ok(aid) = r.try_get::<String, _>("achievement_id") {
                if let Some(ach) = crate::achievements::find_achievement(&aid) {
                    ach_bonus += ach.xp_bonus;
                }
            }
        }
        // Keep the current session's user_stats row in sync with the ledger.
        // IMPORTANT: user_stats.xp stores ONLY quest XP. Achievement bonuses
        // are added at display time (header + leaderboard), exactly once —
        // persisting the summed total here made the leaderboard count the
        // bonus a second time (the XP mismatch bug).
        let _ = sqlx::query(
            "UPDATE user_stats SET xp = ?, completed_count = ? WHERE session_id = ?",
        )
        .bind(quest_xp_total)
        .bind(completed_count)
        .bind(session_id)
        .execute(pool)
        .await;

        let total_xp = quest_xp_total + ach_bonus;

        return Ok(UserStats {
            session_id: session_id.to_string(),
            xp: total_xp,
            completed_count,
            has_completed_first,
            user_email: email,
            completed_quest_ids,
            unlocked_achievements_count,
        });
    }

    // Guest / Anonymous session
    let quest_rows = sqlx::query("SELECT quest_id, xp_awarded FROM quest_completions WHERE session_id = ?")
        .bind(session_id)
        .fetch_all(pool)
        .await?;

    // Dedupe per quest (same rule as the registered path): one reward per
    // quest, the best recorded one.
    let mut quest_xp_by_id: std::collections::HashMap<String, i64> =
        std::collections::HashMap::new();
    for r in quest_rows {
        if let Ok(qid) = r.try_get::<String, _>("quest_id") {
            let xp = r.try_get::<i64, _>("xp_awarded").unwrap_or(0);
            let best = quest_xp_by_id.entry(qid).or_insert(0);
            if xp > *best {
                *best = xp;
            }
        }
    }
    let completed_quest_ids: Vec<String> = quest_xp_by_id.keys().cloned().collect();
    let calculated_quest_xp: i64 = quest_xp_by_id.values().sum();

    let completed_count = completed_quest_ids.len() as i64;
    let has_completed_first = completed_quest_ids
        .iter()
        .any(|qid| qid == "rust-01-mutability");

    // Include achievement bonus XP for guest
    let sess_ident = format!("session:{}", session_id);
    let ach_rows = sqlx::query("SELECT achievement_id FROM user_achievements WHERE user_identifier = ?")
        .bind(&sess_ident)
        .fetch_all(pool)
        .await?;

    let mut ach_bonus: i64 = 0;
    let unlocked_achievements_count = ach_rows.len();
    for r in ach_rows {
        if let Ok(aid) = r.try_get::<String, _>("achievement_id") {
            if let Some(ach) = crate::achievements::find_achievement(&aid) {
                ach_bonus += ach.xp_bonus;
            }
        }
    }

    // user_stats.xp is a quest-XP-only cache (see the registered branch):
    // achievement bonuses are added once at display time.
    let _ = sqlx::query("UPDATE user_stats SET xp = ?, completed_count = ? WHERE session_id = ?")
        .bind(calculated_quest_xp)
        .bind(completed_count)
        .bind(session_id)
        .execute(pool)
        .await;

    let total_xp = calculated_quest_xp + ach_bonus;

    Ok(UserStats {
        session_id: session_id.to_string(),
        xp: total_xp,
        completed_count,
        has_completed_first,
        user_email: None,
        completed_quest_ids,
        unlocked_achievements_count,
    })
}

/// Record a quest completion. If already completed, awards 0 extra XP.
///
/// Returns `(already_completed, current_total_xp)`.
pub async fn record_quest_completion(
    pool: &SqlitePool,
    session_id: &str,
    quest_id: &str,
    xp_to_award: i64,
) -> Result<(bool, i64), sqlx::Error> {
    // Check if already completed by this session or user account
    let current_stats = get_user_stats(pool, session_id).await?;
    if current_stats.completed_quest_ids.iter().any(|q| q == quest_id) {
        return Ok((true, current_stats.xp));
    }

    let now = Utc::now().to_rfc3339();
    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO quest_completions (session_id, quest_id, xp_awarded, completed_at) VALUES (?, ?, ?, ?)",
    )
    .bind(session_id)
    .bind(quest_id)
    .bind(xp_to_award)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE user_stats SET xp = xp + ?, completed_count = completed_count + 1 WHERE session_id = ?",
    )
    .bind(xp_to_award)
    .bind(session_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let updated_stats = get_user_stats(pool, session_id).await?;
    Ok((false, updated_stats.xp))
}

/// Check and award achievements for the given session/user.
/// Returns any achievements that were newly unlocked during this check.
pub async fn sync_and_award_achievements(
    pool: &SqlitePool,
    session_id: &str,
    has_visited_codex: bool,
) -> Result<Vec<&'static crate::achievements::Achievement>, sqlx::Error> {
    // 1. Determine user_identifier
    let user_id_row = sqlx::query("SELECT user_id FROM user_sessions WHERE session_id = ?")
        .bind(session_id)
        .fetch_optional(pool)
        .await?;
    let user_id: Option<i64> = user_id_row
        .and_then(|r| r.try_get::<Option<i64>, _>("user_id").ok().flatten());

    let user_identifier = match user_id {
        Some(uid) => format!("user:{}", uid),
        None => format!("session:{}", session_id),
    };

    // 2. Fetch current stats
    let stats = get_user_stats(pool, session_id).await?;

    // 3. Fetch already unlocked achievements
    let existing_rows = sqlx::query("SELECT achievement_id FROM user_achievements WHERE user_identifier = ?")
        .bind(&user_identifier)
        .fetch_all(pool)
        .await?;
    let existing_ids: std::collections::HashSet<String> = existing_rows
        .into_iter()
        .filter_map(|r| r.try_get::<String, _>("achievement_id").ok())
        .collect();

    // 4. Check eligibility
    let has_q = |qid: &str| stats.completed_quest_ids.iter().any(|q| q == qid);

    let mut newly_awarded = Vec::new();
    let now = Utc::now().to_rfc3339();

    for ach in crate::achievements::ALL_ACHIEVEMENTS {
        if existing_ids.contains(ach.id) {
            continue;
        }

        let is_eligible = match ach.id {
            "first_spark" => has_q("rust-01-mutability"),
            "shadow_weaver" => has_q("rust-02-shadowing"),
            "tablet_bearer" => has_q("rust-03-ownership"),
            "borrow_sage" => has_q("rust-04-references"),
            "slice_master" => has_q("rust-05-slices"),
            // chapter1_conqueror: every quest of the first five chapters
            // (Basics, Flow, Forms, Collections, Strings — 51 quests).
            "chapter1_conqueror" => ["basics", "flow", "forms", "collections", "strings"]
                .iter()
                .all(|&ch_id| {
                    crate::quests::chapter_quests(ch_id).iter().all(|q| has_q(q.id))
                }),
            // grand_architect: 50 completed quests in total.
            "grand_architect" => stats.completed_count >= 50,
            "century_spark" => stats.xp >= 1000,
            "high_paragon" => stats.xp >= 2500,
            "codex_scholar" => has_visited_codex,
            "consecrated_traveler" => stats.user_email.is_some(),
            _ => false,
        };

        if is_eligible {
            let res = sqlx::query(
                "INSERT OR IGNORE INTO user_achievements (user_identifier, achievement_id, unlocked_at, notified) VALUES (?, ?, ?, 0)"
            )
            .bind(&user_identifier)
            .bind(ach.id)
            .bind(&now)
            .execute(pool)
            .await?;

            if res.rows_affected() > 0 {
                newly_awarded.push(ach);
            }
        }
    }

    // Refresh stats if new achievements were unlocked
    if !newly_awarded.is_empty() {
        let _ = get_user_stats(pool, session_id).await;
    }

    Ok(newly_awarded)
}

/// Explicitly award one achievement to a user identifier
/// (`user:{uid}` / `session:{sid}`) — used for `garden_founder`, which is
/// granted at the exact moment an account is created (verify-email success),
/// not by the periodic eligibility engine.
/// Returns `true` when the row was newly inserted.
pub async fn award_achievement_for_identifier(
    pool: &SqlitePool,
    user_identifier: &str,
    achievement_id: &str,
) -> Result<bool, sqlx::Error> {
    let now = Utc::now().to_rfc3339();
    let res = sqlx::query(
        "INSERT OR IGNORE INTO user_achievements (user_identifier, achievement_id, unlocked_at, notified)
         VALUES (?, ?, ?, 0)",
    )
    .bind(user_identifier)
    .bind(achievement_id)
    .bind(&now)
    .execute(pool)
    .await?;
    Ok(res.rows_affected() > 0)
}

/// Retrieve any achievements that have been unlocked but not yet notified to the client.
/// Marks all returned achievements as notified (`notified = 1`).
pub async fn get_and_clear_pending_achievements(
    pool: &SqlitePool,
    session_id: &str,
) -> Result<Vec<&'static crate::achievements::Achievement>, sqlx::Error> {
    let user_id_row = sqlx::query("SELECT user_id FROM user_sessions WHERE session_id = ?")
        .bind(session_id)
        .fetch_optional(pool)
        .await?;
    let user_id: Option<i64> = user_id_row
        .and_then(|r| r.try_get::<Option<i64>, _>("user_id").ok().flatten());

    let user_identifier = match user_id {
        Some(uid) => format!("user:{}", uid),
        None => format!("session:{}", session_id),
    };

    let rows = sqlx::query(
        "SELECT achievement_id FROM user_achievements WHERE user_identifier = ? AND notified = 0"
    )
    .bind(&user_identifier)
    .fetch_all(pool)
    .await?;

    let mut pending = Vec::new();
    for r in rows {
        if let Ok(aid) = r.try_get::<String, _>("achievement_id") {
            if let Some(ach) = crate::achievements::find_achievement(&aid) {
                pending.push(ach);
            }
        }
    }

    if !pending.is_empty() {
        let _ = sqlx::query(
            "UPDATE user_achievements SET notified = 1 WHERE user_identifier = ? AND notified = 0"
        )
        .bind(&user_identifier)
        .execute(pool)
        .await;
    }

    Ok(pending)
}

/// Mark specific achievements as already shown to the client.
///
/// Used when achievements are displayed inline (e.g. right after a quest
/// validation), so the global polling endpoint will not show them twice.
pub async fn mark_achievements_notified(
    pool: &SqlitePool,
    session_id: &str,
    achievement_ids: &[&str],
) -> Result<(), sqlx::Error> {
    if achievement_ids.is_empty() {
        return Ok(());
    }

    let user_id_row = sqlx::query("SELECT user_id FROM user_sessions WHERE session_id = ?")
        .bind(session_id)
        .fetch_optional(pool)
        .await?;
    let user_id: Option<i64> = user_id_row
        .and_then(|r| r.try_get::<Option<i64>, _>("user_id").ok().flatten());

    let user_identifier = match user_id {
        Some(uid) => format!("user:{}", uid),
        None => format!("session:{}", session_id),
    };

    for &aid in achievement_ids {
        let _ = sqlx::query(
            "UPDATE user_achievements SET notified = 1
             WHERE user_identifier = ? AND achievement_id = ?",
        )
        .bind(&user_identifier)
        .bind(aid)
        .execute(pool)
        .await;
    }

    Ok(())
}

/// Returns mapping of achievement_id -> unlocked_at for this session or linked user.
pub async fn get_unlocked_achievements(
    pool: &SqlitePool,
    session_id: &str,
) -> Result<std::collections::HashMap<String, String>, sqlx::Error> {
    let user_id_row = sqlx::query("SELECT user_id FROM user_sessions WHERE session_id = ?")
        .bind(session_id)
        .fetch_optional(pool)
        .await?;
    let user_id: Option<i64> = user_id_row
        .and_then(|r| r.try_get::<Option<i64>, _>("user_id").ok().flatten());

    let user_identifier = match user_id {
        Some(uid) => format!("user:{}", uid),
        None => format!("session:{}", session_id),
    };

    let rows = sqlx::query("SELECT achievement_id, unlocked_at FROM user_achievements WHERE user_identifier = ?")
        .bind(&user_identifier)
        .fetch_all(pool)
        .await?;

    let mut map = std::collections::HashMap::new();
    for r in rows {
        if let (Ok(aid), Ok(uat)) = (
            r.try_get::<String, _>("achievement_id"),
            r.try_get::<String, _>("unlocked_at"),
        ) {
            map.insert(aid, uat);
        }
    }

    Ok(map)
}

#[derive(Debug, Clone, Serialize)]
pub struct LeaderboardItem {
    pub rank: usize,
    pub display_name: String,
    pub is_registered: bool,
    pub is_current_user: bool,
    pub total_xp: i64,
    pub completed_quests: usize,
    pub unlocked_achievements: usize,
    pub rank_title: String,
    pub avatar_emoji: String,
}

pub async fn get_leaderboard(
    pool: &SqlitePool,
    current_session_id: &str,
    t: &Translation,
) -> Result<Vec<LeaderboardItem>, sqlx::Error> {
    // Determine current user_id
    let cur_user_id: Option<i64> = sqlx::query("SELECT user_id FROM user_sessions WHERE session_id = ?")
        .bind(current_session_id)
        .fetch_optional(pool)
        .await?
        .and_then(|r| r.try_get::<Option<i64>, _>("user_id").ok().flatten());

    struct RawEntry {
        display_name: String,
        is_registered: bool,
        is_current_user: bool,
        quest_xp: i64,
        completed_quests: usize,
        unlocked_achievements: usize,
        ach_bonus: i64,
        avatar_emoji: String,
    }

    let mut entries: Vec<RawEntry> = Vec::new();

    // 1. Registered users
    let reg_rows = sqlx::query("SELECT id, email, nickname FROM users")
        .fetch_all(pool)
        .await?;

    for u in reg_rows {
        let uid: i64 = u.try_get("id")?;
        let email: String = u.try_get("email")?;
        // The nickname is the display name when present (1–32 chars,
        // non-unique); otherwise fall back to the masked email.
        let nickname: Option<String> = u.try_get("nickname").unwrap_or(None);
        let nickname = nickname
            .map(|n| n.trim().to_string())
            .filter(|n| !n.is_empty());

        let quests_row = sqlx::query(
            "SELECT COALESCE(SUM(xp), 0) as xp, COUNT(*) as cnt FROM (
                 SELECT quest_id, MAX(xp_awarded) as xp
                 FROM quest_completions qc
                 WHERE qc.session_id IN (SELECT session_id FROM user_sessions WHERE user_id = ?)
                 GROUP BY quest_id
             )"
        )
        .bind(uid)
        .fetch_one(pool)
        .await?;

        let quest_xp: i64 = quests_row.try_get("xp").unwrap_or(0);
        let completed_quests: i64 = quests_row.try_get("cnt").unwrap_or(0);

        let user_ident = format!("user:{}", uid);
        let ach_rows = sqlx::query("SELECT achievement_id FROM user_achievements WHERE user_identifier = ?")
            .bind(&user_ident)
            .fetch_all(pool)
            .await?;

        let mut ach_bonus: i64 = 0;
        let unlocked_achievements = ach_rows.len();
        for r in ach_rows {
            if let Ok(aid) = r.try_get::<String, _>("achievement_id") {
                if let Some(ach) = crate::achievements::find_achievement(&aid) {
                    ach_bonus += ach.xp_bonus;
                }
            }
        }

        let is_me = cur_user_id == Some(uid);
        let display_name = nickname.unwrap_or_else(|| mask_email(&email, t));

        entries.push(RawEntry {
            display_name,
            is_registered: true,
            is_current_user: is_me,
            quest_xp,
            completed_quests: completed_quests as usize,
            unlocked_achievements,
            ach_bonus,
            avatar_emoji: if is_me { "🦀".to_string() } else { "🧙‍♂️".to_string() },
        });
    }

    // 2. Active Guest Sessions (unlinked sessions with completions or
    // achievements). Quest XP is taken straight from the completions ledger,
    // NOT from the user_stats cache: the cache may be stale, and it used to
    // already include the achievement bonus — adding the bonus on top of it
    // double-counted every achievement on the leaderboard (the reported
    // header-vs-leaderboard XP mismatch).
    let guest_rows = sqlx::query(
        "SELECT s.session_id FROM user_sessions s
         WHERE s.user_id IS NULL
           AND ( EXISTS(SELECT 1 FROM quest_completions qc WHERE qc.session_id = s.session_id)
              OR EXISTS(SELECT 1 FROM user_achievements ua
                        WHERE ua.user_identifier = 'session:' || s.session_id) )"
    )
    .fetch_all(pool)
    .await?;

    for g in guest_rows {
        let sid: String = g.try_get("session_id")?;

        let quests_row = sqlx::query(
            "SELECT COALESCE(SUM(xp), 0) as xp, COUNT(*) as cnt FROM (
                 SELECT quest_id, MAX(xp_awarded) as xp
                 FROM quest_completions WHERE session_id = ?
                 GROUP BY quest_id
             )"
        )
        .bind(&sid)
        .fetch_one(pool)
        .await?;

        let q_xp: i64 = quests_row.try_get("xp").unwrap_or(0);
        let c_cnt: i64 = quests_row.try_get("cnt").unwrap_or(0);

        let sess_ident = format!("session:{}", sid);
        let ach_rows = sqlx::query("SELECT achievement_id FROM user_achievements WHERE user_identifier = ?")
            .bind(&sess_ident)
            .fetch_all(pool)
            .await?;

        let mut ach_bonus: i64 = 0;
        let unlocked_achievements = ach_rows.len();
        for r in ach_rows {
            if let Ok(aid) = r.try_get::<String, _>("achievement_id") {
                if let Some(ach) = crate::achievements::find_achievement(&aid) {
                    ach_bonus += ach.xp_bonus;
                }
            }
        }

        let is_me = cur_user_id.is_none() && current_session_id == sid;
        let short_id = if sid.len() >= 4 { &sid[..4] } else { &sid };
        let display_name = format!("{}{}", t.leaderboard_guest_prefix, short_id.to_uppercase());

        entries.push(RawEntry {
            display_name,
            is_registered: false,
            is_current_user: is_me,
            quest_xp: q_xp,
            completed_quests: c_cnt as usize,
            unlocked_achievements,
            ach_bonus,
            avatar_emoji: if is_me { "🦀".to_string() } else { "🌱".to_string() },
        });
    }

    // 3. Fallback NPC Guardians if DB has fewer than 3 players (for a lively podium in fresh environments)
    if entries.len() < 3 {
        let guardians = [
            (&t.leaderboard_npc_guardian_1, 550, 5, 11usize),
            (&t.leaderboard_npc_guardian_2, 480, 5, 10),
            (&t.leaderboard_npc_guardian_3, 410, 5, 9),
        ];
        for (name, xp, quests, achs) in guardians {
            let name = name.to_string();
            if !entries.iter().any(|e| e.display_name == name) {
                entries.push(RawEntry {
                    display_name: name,
                    is_registered: true,
                    is_current_user: false,
                    quest_xp: xp,
                    completed_quests: quests,
                    unlocked_achievements: achs,
                    ach_bonus: 0,
                    avatar_emoji: "✨".to_string(),
                });
            }
        }
    }

    // 4. Sort: highest total XP, then most achievements, then most quests
    entries.sort_by(|a, b| {
        let total_a = a.quest_xp + a.ach_bonus;
        let total_b = b.quest_xp + b.ach_bonus;
        total_b.cmp(&total_a)
            .then_with(|| b.unlocked_achievements.cmp(&a.unlocked_achievements))
            .then_with(|| b.completed_quests.cmp(&a.completed_quests))
    });

    // 5. Build final LeaderboardItem vector with rank and titles
    let mut result = Vec::new();
    for (idx, e) in entries.into_iter().enumerate() {
        let rank = idx + 1;
        let total_xp = e.quest_xp + e.ach_bonus;
        let rank_title = if rank == 1 {
            t.leaderboard_rank_1
        } else if total_xp >= 300 {
            t.leaderboard_rank_2
        } else if total_xp >= 150 {
            t.leaderboard_rank_3
        } else if total_xp >= 50 {
            t.leaderboard_rank_4
        } else {
            t.leaderboard_rank_5
        };

        result.push(LeaderboardItem {
            rank,
            display_name: e.display_name,
            is_registered: e.is_registered,
            is_current_user: e.is_current_user,
            total_xp,
            completed_quests: e.completed_quests,
            unlocked_achievements: e.unlocked_achievements,
            rank_title: rank_title.to_string(),
            avatar_emoji: e.avatar_emoji,
        });
    }

    Ok(result)
}

fn mask_email(email: &str, t: &Translation) -> String {
    if let Some((name, domain)) = email.split_once('@') {
        if name.len() <= 2 {
            format!("{}***@{}", name, domain)
        } else {
            let first = &name[..1];
            let last = &name[name.len() - 1..];
            format!("{}***{}@{}", first, last, domain)
        }
    } else {
        t.leaderboard_anon_name.to_string()
    }
}
