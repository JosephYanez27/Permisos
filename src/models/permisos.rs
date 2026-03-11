use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize,FromRow)]
pub struct PermisoPerfil {
    pub id: Option<i32>,
    pub idperfil: i32,
    pub idmodulo: i32,
    pub modulo: String,
    pub bitagregar: bool,
    pub biteditar: bool,
    pub bitconsulta: bool,
    pub biteliminar: bool,
    pub bitdetalle: bool
}

#[derive(Deserialize,FromRow)]
pub struct CrearPermisoPerfil {
    pub idperfil: i32,
    pub idmodulo: i32,
    pub bitagregar: bool,
    pub biteditar: bool,
    pub bitconsulta: bool,
    pub biteliminar: bool,
    pub bitdetalle: bool,
}

#[derive(Serialize, FromRow)]
pub struct PermisoModulo {
    pub modulo: String,
    pub bitagregar: bool,
    pub biteditar: bool,
    pub bitconsulta: bool,
    pub biteliminar: bool,
    pub bitdetalle: bool,
}