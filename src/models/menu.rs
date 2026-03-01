use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Permiso {
    pub bitagregar: bool,
    pub biteditar: bool,
    pub bitconsulta: bool,
    pub biteliminar: bool,
    pub bitdetalle: bool,
}