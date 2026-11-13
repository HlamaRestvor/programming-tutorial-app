use std::{env, error::Error, net::SocketAddr, process, sync::Arc };
use axum::{routing, Router};
use tera::Tera;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod achievements;
mod articles;
mod auth;
mod auth_routes;
mod db;
mod db_auth;
mod error_pages;
mod i18n;
mod quests;
mod routes;
mod state;
mod config;

use auth_routes::{
    account_email_confirm_handler, account_email_request_handler, account_handler,
    account_nickname_handler, account_password_handler, account_signout_others_handler,
    login_handler, logout_handler, recovery_confirm_handler, recovery_page_handler,
    recovery_request_handler, register_handler, verify_email_handler,
};
use routes::{
    adventure_chapter_handler, adventures_handler, codex_article_handler, codex_handler,
    health_handler, home_handler,
    leaderboard_handler, pending_achievements_handler, quest_runner_handler, quest_shortcut_handler,
    quick_check_handler, validate_quest_handler,
};
use state::AppState;
use crate::state::AppStateInner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    // 1. Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
            "programming_tutorial_app=debug,tower_http=debug".into()
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("Initializing Ferris Garden (Rust Learning Game v0.0.1)...");

    // 2. Initialize Database (SQLite with automatic migrations)
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite://app.db?mode=rwc".to_string());
    let db = db::init_db(&db_url).await.unwrap_or_else(|e| {
        tracing::error!("Failed to initialize database: {e}");
        process::exit(1);
    });

    // Apply auth-specific schema migrations
    db_auth::apply_auth_migrations(&db).await.unwrap_or_else(|e| {
        tracing::error!("Failed to apply auth migrations: {e}");
        process::exit(1);
    });

    // 3. Initialize Tera Templates
    let tera = Tera::new("templates/**/*.tera").unwrap_or_else(|e| {
        tracing::error!("Template parsing error: {e}");
        process::exit(1);
    });
    tracing::info!("Loaded templates successfully.");

    // 4. Read auth configuration from environment
    let brevo_api_key = env::var("BREVO_API_KEY").unwrap_or_default();
    if brevo_api_key.is_empty() {
        tracing::warn!("BREVO_API_KEY not set — email sending is in dev-skip mode.");
    }

    // 5. Initialize AppState (Arc wrapper with Deref)
    let state: AppState = Arc::new(
        AppStateInner { tera, db, brevo_api_key }
    );

    // 6. Setup Router & Static File Serving
    let app = Router::new()
        .route("/", routing::get(home_handler))
        .route("/adventures", routing::get(adventures_handler))
        .route("/adventures/{chapter}", routing::get(adventure_chapter_handler))
        .route("/codex", routing::get(codex_handler))
        .route("/codex/{chapter}/article", routing::get(codex_article_handler))
        .route("/leaderboard", routing::get(leaderboard_handler))
        .route("/quest/{quest_id}", routing::get(quest_runner_handler))
        .route("/rust-{slug}", routing::get(quest_shortcut_handler))
        // Account cabinet & password recovery (Brevo-only auth era)
        .route("/account", routing::get(account_handler))
        .route("/recovery", routing::get(recovery_page_handler))
        .route("/health", routing::get(health_handler))
        .route("/api/quest/quick-check", routing::post(quick_check_handler))
        .route("/api/quest/validate", routing::post(validate_quest_handler))
        .route("/api/achievements/pending", routing::get(pending_achievements_handler))
        // Sign-in and sign-out
        .route("/api/auth/login", routing::post(login_handler))
        .route("/api/auth/logout", routing::post(logout_handler))
        // Auth API (via Brevo)
        .route("/api/auth/register", routing::post(register_handler))
        .route("/api/auth/verify-email", routing::post(verify_email_handler))
        // Password recovery
        .route("/api/auth/recovery-request", routing::post(recovery_request_handler))
        .route("/api/auth/recovery-confirm", routing::post(recovery_confirm_handler))
        // Account cabinet API
        .route("/api/auth/account/nickname", routing::post(account_nickname_handler))
        .route("/api/auth/account/email-request", routing::post(account_email_request_handler))
        .route("/api/auth/account/email-confirm", routing::post(account_email_confirm_handler))
        .route("/api/auth/account/password", routing::post(account_password_handler))
        .route("/api/auth/account/signout-others", routing::post(account_signout_others_handler))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(error_pages::not_found_handler)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // 7. Bind to Port (compatible with local test & Leapcell remote hosting)
    let port: u16 = (env::var("PORT").ok())
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0; 4], port));
    tracing::info!("✨ Ferris Garden running on http://127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
