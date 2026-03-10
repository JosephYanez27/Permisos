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
    pub perfil: String,
    pub estado: String,
    pub strcorreo: String,
    pub strnumerocelular: Option<String>,
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
#[derive(Deserialize)]
pub struct UsuarioQuery {
   pub page: Option<i64>,
   pub usuario: Option<String>,
   pub perfil: Option<i32>,
   pub estado: Option<i32>,
}

#[derive(Serialize)]
pub struct UsuarioResponse {
   pub total: i64,
   pub data: Vec<Usuario>,
}