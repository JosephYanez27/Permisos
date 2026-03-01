use serde::Deserialize;
use reqwest::Client;
use std::env;

#[derive(Deserialize)]
struct RecaptchaResponse {
    success: bool,
    score: Option<f32>,
    action: Option<String>,
}

pub async fn verify_recaptcha(token: &str) -> Result<bool, reqwest::Error> {

    let secret = env::var("RECAPTCHA_SECRET")
        .expect("RECAPTCHA_SECRET no definido en .env");

    let client = Client::new();

    let params = [
        ("secret", secret),
        ("response", token.to_string())
    ];

    let res = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&params)
        .send()
        .await?;

    let body: RecaptchaResponse = res.json().await?;

    // Si usas v2 solo revisa success
    Ok(body.success)

    // Si usas v3 puedes validar score:
    // Ok(body.success && body.score.unwrap_or(0.0) > 0.5)
}