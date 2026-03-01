use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize,FromRow)]
pub struct LoginRequest {
    pub usuario: String,
    pub password: String,
    pub recaptcha_token: String,
}
#[derive(Serialize,FromRow)]
pub struct Usuario {
    pub id: i32,
    pub strnombreusuario: String,
    pub idperfil: i32,
    pub idestadousuario: i32,
    pub strcorreo: String,
    pub strnumerocelular: String,
}

#[derive(Deserialize,FromRow)]
pub struct CrearUsuario {
    pub strnombreusuario: String,
    pub idperfil: i32,
    pub strpwd: String,
    pub idestadousuario: i32,
    pub strcorreo: String,
    pub strnumerocelular: String,
}