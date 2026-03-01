use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;
use crate::models::usuario::LoginRequest;
use crate::utils::{hash::verify_password, jwt::generate_jwt};
use crate::utils::recaptcha::verify_recaptcha;

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    data: web::Json<LoginRequest>,
) -> HttpResponse {

    // 🔐 1️⃣ VALIDAR RECAPTCHA PRIMERO
    let captcha_valido = verify_recaptcha(&data.recaptcha_token).await;

    match captcha_valido {
    Ok(true) => {
        // captcha correcto, continuar login
    }
    Ok(false) => {
        return HttpResponse::Unauthorized().body("Captcha inválido");
    }
    Err(_) => {
        return HttpResponse::InternalServerError()
            .body("Error verificando reCAPTCHA");
    }
}

    // 🔎 2️⃣ Buscar usuario
    let usuario = sqlx::query!(
        r#"
        SELECT u.id, u.strpwd, u.idperfil, e.strdescripcion
        FROM usuario u
        JOIN estadousuario e ON e.id = u.idestadousuario
        WHERE u.strnombreusuario = $1
        "#,
        data.usuario
    )
    .fetch_optional(pool.get_ref())
    .await;

    if usuario.is_err() {
        return HttpResponse::InternalServerError().body("Error interno");
    }

    let usuario = usuario.unwrap();

    if usuario.is_none() {
        return HttpResponse::Unauthorized().body("Credenciales inválidas");
    }

    let u = usuario.unwrap();

    // 🔑 3️⃣ Verificar password
    if !verify_password(&data.password, &u.strpwd) {
        return HttpResponse::Unauthorized().body("Credenciales inválidas");
    }

    // 🚫 4️⃣ Verificar estado
    if u.strdescripcion.to_lowercase() != "activo" {
        return HttpResponse::Unauthorized().body("Usuario inactivo");
    }

    // 🎟 5️⃣ Generar JWT
    let token = generate_jwt(u.id, u.idperfil);

    HttpResponse::Ok().json(serde_json::json!({
        "token": token
    }))
}