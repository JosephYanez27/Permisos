use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub usuario: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct Usuario {
    pub id: i32,
    pub strnombreusuario: String,
    pub idperfil: i32,
    pub idestadousuario: i32,
    pub strcorreo: String,
    pub strnumerocelular: String,
}

#[derive(Deserialize)]
pub struct CrearUsuario {
    pub strnombreusuario: String,
    pub idperfil: i32,
    pub strpwd: String,
    pub idestadousuario: i32,
    pub strcorreo: String,
    pub strnumerocelular: String,
}