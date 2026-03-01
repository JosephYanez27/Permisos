use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Perfil {
    pub id: i32,
    pub strnombreperfil: String,
    pub bitadministrador: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CrearPerfil {
    pub strnombreperfil: String,
    pub bitadministrador: bool,
}