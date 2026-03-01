use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct PermisoPerfil {
    pub id: i32,
    pub idperfil: i32,
    pub idmodulo: i32,
    pub bitagregar: bool,
    pub biteditar: bool,
    pub bitconsulta: bool,
    pub biteliminar: bool,
    pub bitdetalle: bool,
}

#[derive(Deserialize)]
pub struct CrearPermisoPerfil {
    pub idperfil: i32,
    pub idmodulo: i32,
    pub bitagregar: bool,
    pub biteditar: bool,
    pub bitconsulta: bool,
    pub biteliminar: bool,
    pub bitdetalle: bool,
}