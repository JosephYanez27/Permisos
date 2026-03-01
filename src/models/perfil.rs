use serde::{Serialize, Deserialize};
use sqlx::FromRow;
#[derive(Serialize, Deserialize,FromRow)]
pub struct Perfil {
    pub id: i32,
    pub strnombreperfil: String,
    pub bitadministrador: bool,
}

#[derive(Serialize, Deserialize,FromRow)]
pub struct CrearPerfil {
    pub strnombreperfil: String,
    pub bitadministrador: bool,
}