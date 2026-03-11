use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Permiso {
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