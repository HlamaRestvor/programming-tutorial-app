//! Authentication utilities: password hashing, 6-digit email codes, Brevo email sending.

use std::env;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::Rng;

/// Hash a plaintext password with Argon2id.
/// Returns the full PHC-string (salt embedded).
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

/// Verify a plaintext password against a stored Argon2 PHC-string hash.
pub fn verify_password(password: &str, hash_str: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(hash_str)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}

/// Generate a cryptographically random 6-digit numeric code (000000–999999).
pub fn generate_confirmation_code() -> String {
    let n: u32 = rand::thread_rng().gen_range(0..1_000_000);
    format!("{:06}", n)
}

/// Send a confirmation email via Brevo Transactional API.
/// Silently returns Ok if `brevo_api_key` is empty (dev skip mode).
pub async fn send_confirmation_email(
    to_email: &str,
    code: &str,
    brevo_api_key: &str,
) -> Result<(), reqwest::Error> {
    dotenvy::dotenv().unwrap();

    // Dev mode: skip sending if no key configured
    if brevo_api_key.is_empty() {
        tracing::warn!("BREVO_API_KEY not set — skipping email send. Code for {to_email}: {code}");
        return Ok(());
    }

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "sender": { "name": "Ferris Garden", "email": env::var("BREVO_SENDER_EMAIL").unwrap() },
        "to": [{ "email": to_email }],
        "subject": "Your Ferris Garden confirmation code",
        "htmlContent": format!(
            "<h2>🦀 Ferris Garden</h2>\
             <p>Your confirmation code is:</p>\
             <h1 style=\"letter-spacing:0.25em;font-family:monospace\">{}</h1>\
             <p>It expires in 15 minutes.</p>",
            code
        )
    });

    let resp = client
        .post("https://api.brevo.com/v3/smtp/email")
        .header("api-key", brevo_api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        tracing::error!("Brevo API error {status}: {text}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_confirmation_code() {
        dotenvy::dotenv().unwrap();
        println!("{}", env::var("BREVO_SENDER_EMAIL").unwrap());
    }
}