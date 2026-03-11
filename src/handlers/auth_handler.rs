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

    // 🔐 1️⃣ Validar reCAPTCHA
    match verify_recaptcha(&data.recaptcha_token).await {
        Ok(true) => {}
        Ok(false) => return HttpResponse::Unauthorized().body("Captcha inválido"),
        Err(_) => return HttpResponse::InternalServerError().body("Error verificando captcha"),
    }

    // 🔎 2️⃣ Buscar usuario
    let usuario = sqlx::query(
        r#"
        SELECT u.id, u.strpwd, u.idperfil, e.strdescripcion
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

    let usuario = usuario.unwrap();

    if usuario.is_none() {
        return HttpResponse::Unauthorized().body("Credenciales inválidas");
    }

    let row = usuario.unwrap();

    let id: i32 = row.get("id");
    let hash_guardado: String = row.get("strpwd");
    let idperfil: i32 = row.get("idperfil");
    let estado: String = row.get("strdescripcion");

    // 🔑 3️⃣ Verificar password
    if !verify_password(&data.password, &hash_guardado) {
        return HttpResponse::Unauthorized().body("Credenciales inválidas");
    }

    // 🚫 4️⃣ Verificar estado
    if estado.to_lowercase() != "activo" {
        return HttpResponse::Unauthorized().body("Usuario inactivo");
    }
    println!("Usuario login: {}", data.usuario);
println!("Password login: {}", data.password);
    // 🎟 5️⃣ Generar JWT
    let token = generate_jwt(id, idperfil);

    HttpResponse::Ok().json(serde_json::json!({
        "token": token
    }))
}