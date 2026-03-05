use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Modulo {
    pub id: i32,
    pub strnombremodulo: String,
    pub idmodulopadre: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct CrearModulo {
    pub strnombremodulo: String,
    pub idmodulopadre: Option<i32>
}