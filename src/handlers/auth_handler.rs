use actix_web::{post, web, HttpResponse};
use sqlx::{PgPool, Row};

use crate::models::usuario::LoginRequest;
use crate::utils::{hash::verify_password, jwt::generate_jwt};
use crate::utils::recaptcha::verify_recaptcha;

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    data: web::Json<LoginRequest>,
) -> HttpResponse {

    // 🔐 CAPTCHA
    match verify_recaptcha(&data.recaptcha_token).await {
        Ok(true) => {}
        Ok(false) => return HttpResponse::Unauthorized().body("Captcha inválido"),
        Err(_) => return HttpResponse::InternalServerError().body("Error verificando captcha"),
    }

    // 🔎 QUERY
    let usuario = sqlx::query(
        r#"
        SELECT u.id, u.strpwd, u.idperfil, u.strnombreusuario, e.strdescripcion
        FROM usuario u
        JOIN estadousuario e ON e.id = u.idestadousuario
        WHERE u.strnombreusuario = $1
        "#
    )
    .bind(&data.usuario)
    .fetch_optional(pool.get_ref())
    .await;

    if usuario.is_err() {
        return HttpResponse::InternalServerError().body("Error interno");
    }

    let usuario = match usuario.unwrap() {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().body("Credenciales inválidas"),
    };

    let id: i32 = usuario.get("id");
    let hash_guardado: String = usuario.get("strpwd");
    let idperfil: i32 = usuario.get("idperfil");
    let nombre: String = usuario.get("strnombreusuario");
    let estado: String = usuario.get("strdescripcion");

    // 🔑 PASSWORD
    if !verify_password(&data.password, &hash_guardado) {
        return HttpResponse::Unauthorized().body("Credenciales inválidas");
    }

    // 🚫 ESTADO
    if estado.to_lowercase() != "activo" {
        return HttpResponse::Unauthorized().body("Usuario inactivo");
    }

    // 🎟 TOKEN
    let token = generate_jwt(id, idperfil);

    HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "id": id,
        "usuario": nombre
    }))
}