//! Standalone error pages (404 / 503 / 500), English-only by design.
//!
//! Templates live in `templates/{404,503,500}.tera` and share
//! `templates/partials/error_shell.tera` + `static/css/error.css`.
//! Every page carries the round binary theme button (moon ⇄ sun) and shows
//! the requested URL path; `500` also embeds the human-readable database
//! error message from the backend context.
//!
//! `not_found_handler` is wired as the Router `.fallback(...)` in main.rs,
//! and `render_error_response` replaces the old inline "debug HTML" bodies
//! whenever a GET page handler fails (template error, DB error, …).

use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::{Html, IntoResponse, Response},
};

use crate::state::AppState;

/// Build an error page response.
///
/// * `code`      — HTTP status number, also rendered on the card ("404"/"500"/…)
/// * `template`  — which thin wrapper to render ("404.tera" / "503.tera" / "500.tera")
/// * `title`     — headline on the card
/// * `message`   — human explanation (escaped by Tera before insertion)
/// * `path`      — the URL path shown in the address chip
pub fn render_error_response(
    state: &AppState,
    code: u16,
    template: &str,
    title: &str,
    message: &str,
    path: &str,
) -> Response {
    let status = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let mut ctx = tera::Context::new();
    ctx.insert("code", &code.to_string());
    ctx.insert("error_title", &title);
    ctx.insert("error_message", &message);
    ctx.insert("request_path", &path);

    match state.tera.render(template, &ctx) {
        Ok(body) => (status, Html(body)).into_response(),
        Err(e) => {
            // The error page itself failed to render — return a minimal,
            // dependency-free fallback so something readable still shows up.
            tracing::error!("Error page render failed ({template}): {e}");
            let body = format!(
                "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\">\
                 <title>{code} · Ferris Garden</title></head>\
                 <body style=\"font-family:sans-serif;text-align:center;padding:4rem\">\
                 <h1>{code}</h1><p>Ferris hit an unexpected snag rendering this page.</p>\
                 <p><a href=\"/\">← Back home</a></p></body></html>"
            );
            (status, Html(body)).into_response()
        }
    }
}

/// Human-friendly wording for database failures surfacing on GET pages.
pub fn db_error_message(context: &str, e: &sqlx::Error) -> String {
    format!("The Garden's database hiccuped while {context}: {e}")
}

/// Router fallback: every unmatched URL lands on the 404 scene.
/// No session work is done here on purpose — the 404 page must render
/// even when the database is unavailable.
pub async fn not_found_handler(State(state): State<AppState>, uri: Uri) -> Response {
    render_error_response(
        &state,
        404,
        "404.tera",
        "Looks like you're lost",
        "This path doesn't grow anywhere in the garden. Maybe the rune was mistyped, or the sprout was never planted at all.",
        uri.path(),
    )
}
