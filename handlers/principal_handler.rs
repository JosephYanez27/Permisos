use actix_web::{get, HttpResponse, Responder};

/// 🔹 Principal 1
#[get("/principal1")]
pub async fn principal1() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "modulo": "Principal 1",
        "acciones": ["Agregar", "Editar", "Eliminar", "Consultar", "Detalle"],
        "mensaje": "Pantalla estática sin conexión a base de datos"
    }))
}

/// 🔹 Principal 1.1
#[get("/principal1_1")]
pub async fn principal1_1() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "modulo": "Principal 1.1",
        "acciones": ["Agregar", "Editar", "Eliminar", "Consultar", "Detalle"],
        "mensaje": "Pantalla estática"
    }))
}

/// 🔹 Principal 1.2
#[get("/principal1_2")]
pub async fn principal1_2() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "modulo": "Principal 1.2",
        "acciones": ["Agregar", "Editar", "Eliminar", "Consultar", "Detalle"],
        "mensaje": "Pantalla estática"
    }))
}

/// 🔹 Principal 2
#[get("/principal2")]
pub async fn principal2() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "modulo": "Principal 2",
        "acciones": ["Agregar", "Editar", "Eliminar", "Consultar", "Detalle"],
        "mensaje": "Pantalla estática"
    }))
}

/// 🔹 Principal 2.1
#[get("/principal2_1")]
pub async fn principal2_1() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "modulo": "Principal 2.1",
        "acciones": ["Agregar", "Editar", "Eliminar", "Consultar", "Detalle"],
        "mensaje": "Pantalla estática"
    }))
}

/// 🔹 Principal 2.2
#[get("/principal2_2")]
pub async fn principal2_2() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "modulo": "Principal 2.2",
        "acciones": ["Agregar", "Editar", "Eliminar", "Consultar", "Detalle"],
        "mensaje": "Pantalla estática"
    }))
}