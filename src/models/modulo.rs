use serde::{Serialize, Deserialize};
use sqlx::FromRow;
#[derive(Serialize,FromRow)]
pub struct Modulo {
    pub id: i32,
    pub strnombremodulo: String,
}

#[derive(Deserialize,FromRow)]
pub struct CrearModulo {
    pub strnombremodulo: String,
}