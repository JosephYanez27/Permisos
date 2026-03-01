use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Modulo {
    pub id: i32,
    pub strnombremodulo: String,
}

#[derive(Deserialize)]
pub struct CrearModulo {
    pub strnombremodulo: String,
}