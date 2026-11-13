use sqlx::SqlitePool;
use std::sync::Arc;
use tera::Tera;

/// Inner application state holding shared services.
pub struct AppStateInner {
    pub tera: Tera,
    pub db: SqlitePool,
    /// Brevo transactional email API key (empty string = dev skip mode).
    pub brevo_api_key: String,
}

/// AppState is an Arc wrapper around AppStateInner.
///
/// This provides single-heap-allocation sharing across all Axum handlers,
/// and transparent field access via `std::ops::Deref`.
pub type AppState = Arc<AppStateInner>;
