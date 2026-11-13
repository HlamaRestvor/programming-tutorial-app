use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::config;
use crate::db;
use crate::error_pages;
use crate::i18n;
use crate::quests;
use crate::state::AppState;

const SESSION_COOKIE_NAME: &str = "fg_session";

#[derive(Debug, Deserialize)]
pub struct HomeQuery {
    pub lang: Option<String>,
}

// todo: rewrite it into axum-extra
/// Helper to extract session ID from incoming Cookie header
pub fn extract_session_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .find_map(|pair| {
            let mut parts = pair.trim().splitn(2, '=');
            let name = parts.next()?.trim();
            let value = parts.next()?.trim();
            if name == SESSION_COOKIE_NAME && !value.is_empty() {
                Some(value.to_string())
            } else {
                None
            }
        })
}

/// Build Set-Cookie header value for persistent guest session
fn make_session_cookie(session_id: &str) -> HeaderValue {
    let cookie_str = format!(
        "{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=31536000",
        SESSION_COOKIE_NAME, session_id
    );
    HeaderValue::from_str(&cookie_str).unwrap_or(
        HeaderValue::from_static("fg_session=anonymous; Path=/")
    )
}

// ---------------------------------------------------------------------------
// Chapter & quest view models (chapter-split pages, Phase E)
// ---------------------------------------------------------------------------

/// One chapter rendered as a card on the adventures map / codex overview.
#[derive(Debug, Serialize)]
pub struct ChapterCardView {
    pub id: &'static str,
    pub number: u8,
    pub icon: &'static str,
    pub title: String,
    pub desc: String,
    pub quest_count: usize,
    pub completed_count: usize,
    pub completed_all: bool,
    pub progress_percent: i64,
    pub has_article: bool,
}

/// One quest rendered as a card on a chapter page (adventure or codex).
#[derive(Debug, Serialize)]
pub struct ChapterQuestView {
    pub id: &'static str,
    pub number: u8,
    pub title: String,
    pub concept: String,
    pub xp_reward: i64,
    pub is_completed: bool,
    pub is_unlocked: bool,
}

/// One rendered piece of a code line: a plain text span (`slot_number = 0`)
/// or a slot input (`slot_number` = 1-based slot index).
#[derive(Debug, Serialize)]
pub struct QuestPieceView {
    pub text: String,
    pub slot_number: usize,
    /// Per-slot input hint (text pieces keep it empty).
    pub placeholder_text: String,
    /// Canonical answer of this slot, revealed only after completion.
    pub answer_value: String,
}

/// One rendered line of the quest code block. Slotted lines carry one or
/// more slot inputs inside `pieces` (text spans alternate with inputs in
/// reading order — several slots may share one line, e.g. a tuple);
/// `slot` is true when at least one input is present.
#[derive(Debug, Serialize)]
pub struct QuestLineView {
    pub indent_spaces: usize,
    pub indent_str: String,
    pub slot: bool,
    pub pieces: Vec<QuestPieceView>,
    pub comment: Option<String>,
}

fn build_chapter_cards(
    translation: &i18n::Translation,
    completed: &[String],
) -> Vec<ChapterCardView> {
    quests::CHAPTERS
        .iter()
        .map(|ch| {
            let ch_quests = quests::chapter_quests(ch.id);
            let done = ch_quests
                .iter()
                .filter(|q| completed.iter().any(|c| c == q.id))
                .count();
            let total = ch_quests.len();
            ChapterCardView {
                id: ch.id,
                number: ch.number,
                icon: ch.icon,
                title: ch.title(translation).to_string(),
                desc: ch.desc(translation).to_string(),
                quest_count: total,
                completed_count: done,
                completed_all: total > 0 && done == total,
                progress_percent: if total == 0 { 0 } else { (done * 100 / total) as i64 },
                has_article: crate::articles::has_article(ch.id),
            }
        })
        .collect()
}

fn build_chapter_quests(
    chapter_id: &str,
    translation: &i18n::Translation,
    completed: &[String],
) -> Vec<ChapterQuestView> {
    quests::chapter_quests(chapter_id)
        .iter()
        .map(|q| {
            let is_completed = completed.iter().any(|c| c == q.id);
            let is_unlocked = is_completed
                || quests::prev_quest(q.number)
                    .map(|prev| completed.iter().any(|c| c == prev.id))
                    .unwrap_or(true);
            ChapterQuestView {
                id: q.id,
                number: q.number,
                title: q.get_title(translation).to_string(),
                concept: q.get_concept(translation).to_string(),
                xp_reward: q.xp_reward,
                is_completed,
                is_unlocked,
            }
        })
        .collect()
}

/// Splits the quest `code` into renderable lines. Every slot marker on a
/// line becomes a slot input in `pieces`, with text spans between them; a
/// trailing ` // ...` becomes a separate comment span. Markers never collide
/// because `{slot}` is not a substring of `{slot2}`/`{slot3}`...
fn build_render_lines(quest: &quests::Quest, is_completed: bool) -> Vec<QuestLineView> {
    quest
        .code
        .iter()
        .map(|raw| {
            let indent_spaces = raw.len() - raw.trim_start_matches(' ').len();
            let (body, comment) = match raw.find(" // ") {
                Some(pos) => (
                    raw[..pos].to_string(),
                    Some(raw[pos + 1..].to_string()),
                ),
                None => (raw.to_string(), None),
            };

            // All slot markers on this line, in reading order (sorted by
            // position, so an unusual `fn pair<{slot2}>(x: {slot})` would
            // still render correctly).
            let mut hits: Vec<(usize, usize)> = (1..=quest.answers.len())
                .filter_map(|i| {
                    body.find(&quests::Quest::slot_marker(i)).map(|pos| (pos, i))
                })
                .collect();
            hits.sort_by_key(|&(pos, _)| pos);

            let mut pieces: Vec<QuestPieceView> = Vec::new();
            let mut cursor = 0usize;
            for (n, &(pos, i)) in hits.iter().enumerate() {
                let marker = quests::Quest::slot_marker(i);
                // The first text span carries the line's indentation — it is
                // rendered as its own span, so strip it from the text.
                let mut text = body[cursor..pos].to_string();
                if n == 0 {
                    let cut = indent_spaces.min(text.len());
                    text = text[cut..].to_string();
                }
                if !text.is_empty() {
                    pieces.push(QuestPieceView {
                        text,
                        slot_number: 0,
                        placeholder_text: String::new(),
                        answer_value: String::new(),
                    });
                }
                let placeholder = quest.placeholders.get(i - 1)
                    .copied().flatten().unwrap_or("");
                let answer = if is_completed {
                    quest.answers[i - 1].first().copied().unwrap_or("")
                } else {
                    ""
                };
                pieces.push(QuestPieceView {
                    text: String::new(),
                    slot_number: i,
                    placeholder_text: placeholder.to_string(),
                    answer_value: answer.to_string(),
                });
                cursor = pos + marker.len();
            }
            // Trailing text after the last marker — or the whole body on
            // plain lines.
            let mut text = body[cursor..].to_string();
            if hits.is_empty() {
                let cut = indent_spaces.min(text.len());
                text = text[cut..].to_string();
            }
            if !text.is_empty() {
                pieces.push(QuestPieceView {
                    text,
                    slot_number: 0,
                    placeholder_text: String::new(),
                    answer_value: String::new(),
                });
            }

            QuestLineView {
                indent_spaces,
                indent_str: " ".repeat(indent_spaces),
                slot: !hits.is_empty(),
                pieces,
                comment,
            }
        })
        .collect()
}

pub async fn home_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HomeQuery>,
) -> Response {
    let lang = params.lang.unwrap_or(i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let existing_sid = extract_session_id(&headers);
    let (session_id, user_stats) = match db::get_or_create_session(
        &state.db, existing_sid.as_deref()
    ).await {
        Ok(pair) => pair,
        Err(e) => {
            let msg = error_pages::db_error_message("loading your garden", &e);
            tracing::error!("Database session error: {e}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked", &msg, "/",
            );
        }
    };

    let ctx = {
        let mut ctx = tera::Context::new();
        ctx.insert("lang", &lang);
        ctx.insert("translation", &translation);
        ctx.insert("user_stats", &user_stats);
        ctx.insert("active_page", "home");
        ctx.insert("version", config::VERSION);
        ctx
    };

    let body = match state.tera.render("index.tera", &ctx) {
        Ok(html) => html,
        Err(e) => {
            tracing::error!("Template render error: {e:?}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked",
                "The home page could not be rendered. Please try again in a moment.",
                "/",
            );
        }
    };

    let mut resp = Html(body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}

pub async fn adventures_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HomeQuery>,
) -> Response {
    let lang = params.lang.unwrap_or(i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let existing_sid = extract_session_id(&headers);
    let (session_id, user_stats) = match db::get_or_create_session(
        &state.db, existing_sid.as_deref()
    ).await {
        Ok(pair) => pair,
        Err(e) => {
            let msg = error_pages::db_error_message("loading the adventure map", &e);
            tracing::error!("Database session error in adventures: {e}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked", &msg, "/adventures",
            );
        }
    };

    let ctx = {
        let mut ctx = tera::Context::new();
        ctx.insert("lang", &lang);
        ctx.insert("translation", &translation);
        ctx.insert("user_stats", &user_stats);
        ctx.insert("chapters", &build_chapter_cards(&translation, &user_stats.completed_quest_ids));
        ctx.insert("total_quests_num", &quests::total_quest_count());
        ctx.insert("active_page", "adventures");
        ctx.insert("version", config::VERSION);
        ctx
    };

    let body = match state.tera.render("adventures.tera", &ctx) {
        Ok(html) => html,
        Err(e) => {
            tracing::error!("Template render error (adventures.tera): {e:?}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked",
                "The adventure map could not be rendered. Please try again in a moment.",
                "/adventures",
            );
        }
    };

    let mut resp = Html(body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}

pub async fn quest_runner_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(quest_id): Path<String>,
    Query(params): Query<HomeQuery>,
) -> Response {
    let lang = params.lang.unwrap_or(i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let quest = match quests::find_quest(&quest_id) {
        Some(q) => q,
        None => return Redirect::to("/adventures").into_response(),
    };

    let existing_sid = extract_session_id(&headers);
    let (session_id, user_stats) = match db::get_or_create_session(
        &state.db, existing_sid.as_deref()
    ).await {
        Ok(pair) => pair,
        Err(e) => {
            let msg = error_pages::db_error_message("loading the quest", &e);
            tracing::error!("Database session error in quest runner: {e}");
            let shown_path = format!("/quest/{quest_id}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked", &msg, &shown_path,
            );
        }
    };

    let is_completed = user_stats.completed_quest_ids.iter().any(|q| q == &quest_id);

    // Access control: prevent skipping ahead into locked quests
    let is_unlocked = is_completed
        || quests::prev_quest(quest.number)
            .map(|prev| user_stats.completed_quest_ids.iter().any(|q| q == prev.id))
            .unwrap_or(true); // no previous quest => unlocked

    if !is_unlocked {
        return Redirect::to("/adventures?locked=1").into_response();
    }

    // Prepare localized fields from single-source Translation
    let quest_title = quest.get_title(&translation);
    let quest_concept = quest.get_concept(&translation);
    let quest_lore = quest.get_lore(&translation);
    let quest_hint = quest.get_hint(&translation);
    let mascot_speech = if is_completed {
        quest.get_mascot_success(&translation)
    } else {
        quest.get_mascot_initial(&translation)
    };

    // Chapter-split navigation + parsed code lines (Phase E).
    let chapter = quest.chapter();
    let prev_id = quests::prev_quest(quest.number).map(|q| q.id);
    let next_id = quests::next_quest(quest.number).map(|q| q.id);
    let render_lines = build_render_lines(quest, is_completed);

    let ctx = {
        let mut ctx = tera::Context::new();
        ctx.insert("lang", &lang);
        ctx.insert("translation", &translation);
        ctx.insert("user_stats", &user_stats);
        ctx.insert("quest", &quest);
        ctx.insert("quest_title", quest_title);
        ctx.insert("quest_concept", quest_concept);
        ctx.insert("quest_lore", quest_lore);
        ctx.insert("quest_hint", quest_hint);
        ctx.insert("mascot_speech", mascot_speech);
        ctx.insert("is_completed", &is_completed);
        ctx.insert("render_lines", &render_lines);
        ctx.insert("chapter_title", chapter.title(&translation));
        ctx.insert("chapter_link", chapter.id);
        ctx.insert("prev_id", &prev_id.map(|s| s.to_string()));
        ctx.insert("next_id", &next_id.map(|s| s.to_string()));
        ctx.insert("active_page", "adventures");
        ctx.insert("version", config::VERSION);
        ctx
    };

    let body = match state.tera.render("quest_runner.tera", &ctx) {
        Ok(html) => html,
        Err(e) => {
            tracing::error!("Template render error (quest_runner.tera): {e:?}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked",
                "The quest page could not be rendered. Please try again in a moment.",
                &format!("/quest/{quest_id}"),
            );
        }
    };

    let mut resp = Html(body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}

#[derive(Debug, Deserialize)]
pub struct ValidateQuestRequest {
    pub quest_id: String,
    /// Legacy single-slot payload. Kept for API compatibility.
    pub token: Option<String>,
    /// Multi-slot payload: one token per slot, in slot order. Takes
    /// priority over `token` when present.
    pub tokens: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct UnlockedAchievementDto {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub xp_bonus: i64,
}

#[derive(Debug, Serialize)]
pub struct ValidateQuestResponse {
    pub correct: bool,
    pub message: String,
    pub mascot_response: String,
    pub xp_awarded: i64,
    pub total_xp: i64,
    pub already_completed: bool,
    pub next_quest_id: Option<String>,
    pub newly_unlocked_achievements: Vec<UnlockedAchievementDto>,
}

pub async fn validate_quest_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HomeQuery>,
    Json(payload): Json<ValidateQuestRequest>,
) -> Response {
    let lang = params.lang.unwrap_or(i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let existing_sid = extract_session_id(&headers);
    let (session_id, current_stats) = db::get_or_create_session(
        &state.db, existing_sid.as_deref()
    ).await.unwrap_or_else(|e| {
        tracing::error!("DB error in validate_quest: {e}");
        ("guest".to_string(), db::UserStats::default())
    });

    let quest = match quests::find_quest(&payload.quest_id) {
        Some(q) => q,
        None => return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "correct": false, "message": "Quest not found" })),
        ).into_response(),
    };

    // Strict access control: prevent cheating by validating locked quests
    let is_unlocked = current_stats.completed_quest_ids.iter().any(|q| q == &quest.id)
        || quests::prev_quest(quest.number)
            .map(|prev| current_stats.completed_quest_ids.iter().any(|q| q == prev.id))
            .unwrap_or(true); // no previous quest => unlocked

    if !is_unlocked {
        return (
            StatusCode::FORBIDDEN,
            Json(ValidateQuestResponse {
                correct: false,
                message: match lang.as_str() {
                    "en" => "This quest is locked. Complete the preceding quest first!".to_string(),
                    "ko" => "이 퀘스트는 잠겨 있습니다. 이전 퀘스트를 먼저 완료하세요!".to_string(),
                    _ => "Этот квест заблокирован. Сначала завершите предыдущее задание!".to_string(),
                },
                mascot_response: "🦀: «Нельзя перепрыгивать через ступени познания! Пройди предыдущий квест.»".to_string(),
                xp_awarded: 0,
                total_xp: current_stats.xp,
                already_completed: false,
                next_quest_id: None,
                newly_unlocked_achievements: Vec::new(),
            }),
        ).into_response();
    }

    // The multi-slot client posts `tokens` (one per slot); the legacy
    // single-slot client still posts `token`. Both funnel into one
    // trimmed list that validate_tokens understands.
    let tokens_owned: Vec<String> = payload.tokens
        .clone()
        .or_else(|| payload.token.clone().map(|t| vec![t]))
        .unwrap_or_default();
    let tokens: Vec<&str> = tokens_owned.iter().map(|t| t.trim()).collect();
    let shown_tokens = tokens.join(", ");
    let is_correct = quest.validate_tokens(&tokens);

    let json_body = if is_correct {
        let (already_completed, total_xp) = db::record_quest_completion(
            &state.db, &session_id, quest.id, quest.xp_reward,
        ).await.unwrap_or_else(|e| {
            tracing::error!("Failed to record quest completion: {e}");
            (false, current_stats.xp + quest.xp_reward)
        });

        // Sync and check for newly unlocked achievements
        let newly_unlocked = db::sync_and_award_achievements(&state.db, &session_id, false)
            .await
            .unwrap_or_default();

        let newly_dto: Vec<UnlockedAchievementDto> = newly_unlocked
            .into_iter()
            .map(|a| UnlockedAchievementDto {
                id: a.id.to_string(),
                title: a.get_title(&translation).to_string(),
                icon: a.icon.to_string(),
                xp_bonus: a.xp_bonus,
            })
            .collect();

        // These achievements are shown inline by the quest runner response,
        // so mark them notified right away — the global polling endpoint will
        // not present them a second time.
        if !newly_dto.is_empty() {
            let ids: Vec<&str> = newly_dto.iter().map(|a| a.id.as_str()).collect();
            if let Err(e) = db::mark_achievements_notified(&state.db, &session_id, &ids).await {
                tracing::error!("mark_achievements_notified error: {e}");
            }
        }

        let final_total_xp = if !newly_dto.is_empty() {
            db::get_user_stats(&state.db, &session_id)
                .await
                .map(|s| s.xp)
                .unwrap_or(total_xp)
        } else {
            total_xp
        };

        let msg = if already_completed {
            match lang.as_str() {
                "en" => format!("Correct rune! (Quest already solved previously, total XP: {})", final_total_xp),
                "ko" => format!("정답입니다! (이미 완료된 퀘스트입니다, 총 경험치: {})", final_total_xp),
                _ => format!("Руна верна! (Задание уже было пройдено ранее, всего XP: {})", final_total_xp),
            }
        } else {
            match lang.as_str() {
                "en" => format!("Compiled successfully! +{} XP earned for your journey.", quest.xp_reward),
                "ko" => format!("성공적으로 컴파일되었습니다! +{} XP를 획득했습니다.", quest.xp_reward),
                _ => format!("Успешно скомпилировано! +{} XP добавлено в твою копилку.", quest.xp_reward),
            }
        };

        ValidateQuestResponse {
            correct: true,
            message: msg,
            mascot_response: quest.get_mascot_success(&translation).to_string(),
            xp_awarded: if already_completed { 0 } else { quest.xp_reward },
            total_xp: final_total_xp,
            already_completed,
            next_quest_id: quests::next_quest(quest.number).map(|q| q.id.to_string()),
            newly_unlocked_achievements: newly_dto,
        }
    } else if tokens.iter().all(|t| t.is_empty()) {
        let (msg, mascot_msg) = match lang.as_str() {
            "en" => (
                "Rune slot is empty. Please enter the missing Rust keyword or expression.",
                "🦀: «The slot is empty! Enter the keyword to satisfy the compiler.»",
            ),
            "ko" => (
                "룬 슬롯이 비어 있습니다. 알맞은 키워드를 입력해주세요.",
                "🦀: «빈칸을 채워주세요! 컴파일러가 기다리고 있어요.»",
            ),
            _ => (
                "Поле руны пусто. Введи недостающее ключевое слово или выражение Rust.",
                "🦀: «Слот пуст! Вспомни нужное ключевое слово компилятора.»",
            ),
        };

        ValidateQuestResponse {
            correct: false,
            message: msg.to_string(),
            mascot_response: mascot_msg.to_string(),
            xp_awarded: 0,
            total_xp: current_stats.xp,
            already_completed: false,
            next_quest_id: None,
            newly_unlocked_achievements: Vec::new(),
        }
    } else {
        let (msg, mascot_msg) = match lang.as_str() {
            "en" => (
                format!("Compiler error: '{}' does not satisfy the compiler checks.", shown_tokens),
                format!("🦀: «Compiler diagnostic failed on '{}'! Check the hint scroll below.»", shown_tokens),
            ),
            "ko" => (
                format!("컴파일 에러: '{}' 키워드가 컴파일 조건을 만족하지 않습니다.", shown_tokens),
                format!("🦀: «'{}'에서 컴파일 오류가 발생했어요! 아래 힌트를 확인해보세요.»", shown_tokens),
            ),
            _ => (
                format!("Ошибка компилятора: руна '{}' не удовлетворяет правилам языка.", shown_tokens),
                format!("🦀: «Компилятор ругается на '{}'! Загляни в подсказку внизу.»", shown_tokens),
            ),
        };

        ValidateQuestResponse {
            correct: false,
            message: msg,
            mascot_response: mascot_msg,
            xp_awarded: 0,
            total_xp: current_stats.xp,
            already_completed: false,
            next_quest_id: None,
            newly_unlocked_achievements: Vec::new(),
        }
    };

    let mut resp = Json(json_body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}

/// Stage 6: Knowledge Codex handler (`/codex`)
pub async fn codex_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HomeQuery>,
) -> Response {
    let lang = params.lang.unwrap_or(i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let existing_sid = extract_session_id(&headers);
    let (session_id, initial_stats) = match db::get_or_create_session(
        &state.db, existing_sid.as_deref()
    ).await {
        Ok(pair) => pair,
        Err(e) => {
            let msg = error_pages::db_error_message("loading the codex", &e);
            tracing::error!("Database session error in codex: {e}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked", &msg, "/codex",
            );
        }
    };

    // Award Codex Scholar achievement upon exploring the Codex
    let _ = db::sync_and_award_achievements(&state.db, &session_id, true).await;
    let user_stats = db::get_user_stats(&state.db, &session_id).await.unwrap_or(initial_stats);

    let codex_total = quests::total_quest_count();
    let codex_percent = if codex_total == 0 {
        0
    } else {
        (user_stats.completed_count.max(0) as usize * 100 / codex_total) as i64
    };

    let ctx = {
        let mut ctx = tera::Context::new();
        ctx.insert("lang", &lang);
        ctx.insert("translation", &translation);
        ctx.insert("user_stats", &user_stats);
        ctx.insert("chapters", &build_chapter_cards(&translation, &user_stats.completed_quest_ids));
        ctx.insert("codex_total", &codex_total);
        ctx.insert("codex_percent", &codex_percent);
        ctx.insert("active_page", "codex");
        ctx.insert("version", config::VERSION);
        ctx
    };

    let body = match state.tera.render("codex.tera", &ctx) {
        Ok(html) => html,
        Err(e) => {
            tracing::error!("Template render error (codex.tera): {e:?}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked",
                "The codex could not be rendered. Please try again in a moment.",
                "/codex",
            );
        }
    };

    let mut resp = Html(body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}

/// Stage 7: Leaderboard & Achievements handler (`/leaderboard`)
pub async fn leaderboard_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HomeQuery>,
) -> Response {
    let lang = params.lang.unwrap_or(i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let existing_sid = extract_session_id(&headers);
    let (session_id, initial_stats) = match db::get_or_create_session(
        &state.db, existing_sid.as_deref()
    ).await {
        Ok(pair) => pair,
        Err(e) => {
            let msg = error_pages::db_error_message("loading the hall of fame", &e);
            tracing::error!("Database session error in leaderboard: {e}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked", &msg, "/leaderboard",
            );
        }
    };

    // Sync achievements before displaying
    let _ = db::sync_and_award_achievements(&state.db, &session_id, false).await;
    let user_stats = db::get_user_stats(&state.db, &session_id)
        .await
        .unwrap_or(initial_stats);

    let leaderboard_entries = db::get_leaderboard(&state.db, &session_id, translation)
        .await
        .unwrap_or_default();

    let unlocked_map = db::get_unlocked_achievements(&state.db, &session_id)
        .await
        .unwrap_or_default();

    let achievements_view: Vec<crate::achievements::AchievementView> = crate::achievements::ALL_ACHIEVEMENTS
        .iter()
        .map(|a| {
            let uat = unlocked_map.get(a.id).cloned();
            let is_unlocked = uat.is_some();
            crate::achievements::AchievementView {
                id: a.id.to_string(),
                icon: a.icon.to_string(),
                title: a.get_title(&translation).to_string(),
                desc: a.get_desc(&translation).to_string(),
                xp_bonus: a.xp_bonus,
                category: a.category.to_string(),
                is_unlocked,
                unlocked_at: uat,
            }
        })
        .collect();

    let ctx = {
        let mut ctx = tera::Context::new();
        ctx.insert("lang", &lang);
        ctx.insert("translation", &translation);
        ctx.insert("user_stats", &user_stats);
        ctx.insert("leaderboard_entries", &leaderboard_entries);
        ctx.insert("achievements", &achievements_view);
        ctx.insert("active_page", "leaderboard");
        ctx.insert("quests_nbr", &quests::QUESTS.len());
        // leaderboard.tera also divides unlocked achievements by the total
        // ({{ user_stats.unlocked_achievements_count }} / {{ achievements_nbr }}).
        ctx.insert("achievements_nbr", &crate::achievements::ALL_ACHIEVEMENTS.len());
        ctx.insert("version", config::VERSION);
        ctx
    };

    let body = match state.tera.render("leaderboard.tera", &ctx) {
        Ok(html) => html,
        Err(e) => {
            tracing::error!("Template render error (leaderboard.tera): {e:?}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked",
                "The hall of fame could not be rendered. Please try again in a moment.",
                "/leaderboard",
            );
        }
    };

    let mut resp = Html(body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}

/// User convenience shortcut: redirects `/rust-:id` directly to `/quest/rust-:id`
pub async fn quest_shortcut_handler(
    Path(slug): Path<String>,
    Query(params): Query<HomeQuery>,
) -> Response {
    let lang_query = params.lang.map(|l| format!("?lang={l}")).unwrap_or_default();
    Redirect::to(&format!("/quest/rust-{slug}{lang_query}")).into_response()
}

// ---------------------------------------------------------------------------
// Chapter-split pages (Phase E):
//   /adventures/{chapter}     — chapter map with quest cards
//   /codex/{chapter}          — codex section with quest cards
//   /codex/{chapter}/article  — long-form deep-dive article
// `{chapter}` resolves by slug ("basics") OR by 1-based number ("2").
// ---------------------------------------------------------------------------

/// Prev/next chapter navigation pill.
#[derive(Debug, Serialize)]
pub struct NavChapterView {
    pub id: &'static str,
    pub icon: &'static str,
    pub title: String,
}

fn nav_chapter_view(ch: &'static quests::Chapter, t: &i18n::Translation) -> NavChapterView {
    NavChapterView { id: ch.id, icon: ch.icon, title: ch.title(t).to_string() }
}

fn neighboring_chapters(
    chapter: &'static quests::Chapter,
    t: &i18n::Translation,
) -> (Option<NavChapterView>, Option<NavChapterView>) {
    let prev = if chapter.number > 1 {
        quests::find_chapter(&(chapter.number - 1).to_string()).map(|c| nav_chapter_view(c, t))
    } else {
        None
    };
    let next = if chapter.number < quests::CHAPTERS.len() as u8 {
        quests::find_chapter(&(chapter.number + 1).to_string()).map(|c| nav_chapter_view(c, t))
    } else {
        None
    };
    (prev, next)
}

pub async fn adventure_chapter_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(chapter_param): Path<String>,
    Query(params): Query<HomeQuery>,
) -> Response {
    let lang = params.lang.unwrap_or(i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let chapter = match quests::find_chapter(&chapter_param) {
        Some(ch) => ch,
        None => return Redirect::to(&format!("/adventures?lang={lang}")).into_response(),
    };

    let existing_sid = extract_session_id(&headers);
    let (session_id, user_stats) = match db::get_or_create_session(
        &state.db, existing_sid.as_deref()
    ).await {
        Ok(pair) => pair,
        Err(e) => {
            let msg = error_pages::db_error_message("loading the chapter", &e);
            tracing::error!("Database session error in adventure chapter: {e}");
            let shown_path = format!("/adventures/{chapter_param}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked", &msg, &shown_path,
            );
        }
    };

    let quests_view = build_chapter_quests(chapter.id, &translation, &user_stats.completed_quest_ids);
    let done_count = quests_view.iter().filter(|q| q.is_completed).count();
    let total_count = quests_view.len();
    let progress_percent = if total_count == 0 { 0 } else { (done_count * 100 / total_count) as i64 };
    let (prev_chapter, next_chapter) = neighboring_chapters(chapter, &translation);

    let ctx = {
        let mut ctx = tera::Context::new();
        ctx.insert("lang", &lang);
        ctx.insert("translation", &translation);
        ctx.insert("user_stats", &user_stats);
        ctx.insert("active_page", "adventures");
        ctx.insert("chapter_id", chapter.id);
        ctx.insert("chapter_num", &chapter.number);
        ctx.insert("chapter_icon", chapter.icon);
        ctx.insert("chapter_title", chapter.title(&translation));
        ctx.insert("chapter_desc", chapter.desc(&translation));
        ctx.insert("mascot_speech", chapter.mascot_initial(&translation));
        ctx.insert("quests", &quests_view);
        ctx.insert("done_count", &done_count);
        ctx.insert("total_count", &total_count);
        ctx.insert("progress_percent", &progress_percent);
        ctx.insert("prev_chapter", &prev_chapter);
        ctx.insert("next_chapter", &next_chapter);
        ctx.insert("has_article", &crate::articles::has_article(chapter.id));
        ctx.insert("version", config::VERSION);
        ctx
    };

    let body = match state.tera.render("chapter.tera", &ctx) {
        Ok(html) => html,
        Err(e) => {
            tracing::error!("Template render error (chapter.tera): {e:?}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked",
                "The chapter could not be rendered. Please try again in a moment.",
                &format!("/adventures/{chapter_param}"),
            );
        }
    };

    let mut resp = Html(body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}

pub async fn codex_article_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(chapter_param): Path<String>,
    Query(params): Query<HomeQuery>,
) -> Response {
    let lang = params.lang.unwrap_or(i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let chapter = match quests::find_chapter(&chapter_param) {
        Some(ch) => ch,
        None => return Redirect::to(&format!("/codex?lang={lang}")).into_response(),
    };

    let existing_sid = extract_session_id(&headers);
    let (session_id, user_stats) = match db::get_or_create_session(
        &state.db, existing_sid.as_deref()
    ).await {
        Ok(pair) => pair,
        Err(e) => {
            let msg = error_pages::db_error_message("loading the article", &e);
            tracing::error!("Database session error in codex article: {e}");
            let shown_path = format!("/codex/{chapter_param}/article");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked", &msg, &shown_path,
            );
        }
    };

    // Award Codex Scholar achievement upon exploring the Codex — including
    // a direct visit to /codex/{chapter}/article that skipped the index page.
    // Repeat awards are impossible (UNIQUE(user_identifier, achievement_id)).
    let _ = db::sync_and_award_achievements(&state.db, &session_id, true).await;
    let user_stats = db::get_user_stats(&state.db, &session_id).await.unwrap_or(user_stats);

    let article = crate::articles::find_article(chapter.id, &lang);
    let (article_title, paragraphs) = match article {
        Some(a) => (
            a.title.to_string(),
            a.paragraphs.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        ),
        None => (chapter.title(&translation).to_string(), Vec::new()),
    };

    let ctx = {
        let mut ctx = tera::Context::new();
        ctx.insert("lang", &lang);
        ctx.insert("translation", &translation);
        ctx.insert("user_stats", &user_stats);
        ctx.insert("active_page", "codex");
        ctx.insert("chapter_id", chapter.id);
        ctx.insert("chapter_icon", chapter.icon);
        ctx.insert("chapter_title", chapter.title(&translation));
        ctx.insert("article_title", &article_title);
        ctx.insert("paragraphs", &paragraphs);
        ctx.insert("has_article", &article.is_some());
        ctx.insert("version", config::VERSION);
        ctx
    };

    let body = match state.tera.render("codex_article.tera", &ctx) {
        Ok(html) => html,
        Err(e) => {
            tracing::error!("Template render error (codex_article.tera): {e:?}");
            return error_pages::render_error_response(
                &state, 500, "500.tera", "The compiler panicked",
                "The article could not be rendered. Please try again in a moment.",
                &format!("/codex/{chapter_param}/article"),
            );
        }
    };

    let mut resp = Html(body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}

#[derive(Debug, Deserialize)]
pub struct QuickCheckRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct QuickCheckResponse {
    pub correct: bool,
    pub message: String,
    pub mascot_response: String,
    pub intro_continue: Option<String>,
    pub xp_awarded: i64,
    pub total_xp: i64,
    pub already_completed: bool,
}

pub async fn quick_check_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HomeQuery>,
    Json(payload): Json<QuickCheckRequest>,
) -> Response {
    let translation = i18n::get_translation(params.lang.as_deref().unwrap_or(i18n::DEF_LANG));

    let existing_sid = extract_session_id(&headers);
    let (session_id, current_stats) = db::get_or_create_session(
        &state.db, existing_sid.as_deref()
    ).await.unwrap_or_else(|e| {
        tracing::error!("DB error in quick_check: {e}");
        ("guest".to_string(), db::UserStats::default())
    });

    let token = payload.token.trim().to_lowercase();

    let json_body = match token.as_str() {
        "mut" => {
            // XP follows the catalog: quests 1..=5 cost a flat 25 now.
            let first_xp = quests::find_quest("rust-01-mutability")
                .map(|q| q.xp_reward)
                .unwrap_or(25);
            let (already_completed, total_xp) = db::record_quest_completion(
                &state.db, &session_id, "rust-01-mutability", first_xp,
            ).await.unwrap_or_else(|e| {
                tracing::error!("Failed to record quest completion: {e}");
                (false, current_stats.xp + first_xp)
            });

            // Achievements (first_spark etc.) may unlock right here on the index
            // page; the client-side popup will fetch them via the drain endpoint.
            let _ = db::sync_and_award_achievements(&state.db, &session_id, false).await;

            let (msg, mascot_msg, xp_awarded) = if already_completed {
                (translation.qc_already.to_string(), translation.qc_already_mascot.to_string(), 0)
            } else {
                (translation.qc_success.to_string(), translation.qc_success_mascot.to_string(), first_xp)
            };

            QuickCheckResponse {
                correct: true,
                message: msg,
                mascot_response: mascot_msg,
                intro_continue: Some(translation.intro_continue.to_owned()),
                xp_awarded,
                total_xp,
                already_completed,
            }
        },
        "" => QuickCheckResponse {
            correct: false,
            message: translation.qc_empty.to_string(),
            mascot_response: translation.qc_empty_mascot.to_string(),
            intro_continue: None,
            xp_awarded: 0,
            total_xp: current_stats.xp,
            already_completed: false,
        },
        _ => QuickCheckResponse {
            correct: false,
            message: translation.qc_wrong.replace("{token}", &token),
            mascot_response: translation.qc_wrong_mascot.to_string(),
            intro_continue: None,
            xp_awarded: 0,
            total_xp: current_stats.xp,
            already_completed: false,
        }
    };

    let mut resp = Json(json_body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}

pub async fn health_handler() -> &'static str {
    "OK"
}

/// Global achievement popup pipeline (Stage: "achievement toast on any page").
///
/// Returns and silently marks as notified all achievements that were unlocked
/// for the current session/user but not yet shown to the client. The frontend
/// pollers this shortly after page load, periodically, and right after the
/// index-page playground succeeds.
pub async fn pending_achievements_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<HomeQuery>,
) -> Response {
    let lang = params.lang.unwrap_or(i18n::DEF_LANG.to_owned());
    let translation = i18n::get_translation(&lang);

    let existing_sid = extract_session_id(&headers);
    let (session_id, _) = db::get_or_create_session(
        &state.db, existing_sid.as_deref()
    ).await.unwrap_or_else(|e| {
        tracing::error!("DB error in pending achievements: {e}");
        (existing_sid.clone().unwrap_or_else(|| "fallback".to_string()), db::UserStats::default())
    });

    let newly_available = db::get_and_clear_pending_achievements(&state.db, &session_id)
        .await
        .unwrap_or_default();

    let payload: Vec<UnlockedAchievementDto> = newly_available
        .into_iter()
        .map(|a| UnlockedAchievementDto {
            id: a.id.to_string(),
            title: a.get_title(&translation).to_string(),
            icon: a.icon.to_string(),
            xp_bonus: a.xp_bonus,
        })
        .collect();

    let body = serde_json::json!({ "achievements": payload });

    let mut resp = Json(body).into_response();
    if existing_sid.as_deref() != Some(&session_id) {
        resp.headers_mut().insert(header::SET_COOKIE, make_session_cookie(&session_id));
    }
    resp
}
