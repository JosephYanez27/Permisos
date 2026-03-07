
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use crate::models::modulo::Modulo;
use serde_json::json;

#[get("/menu")]
pub async fn get_menu(
    pool: web::Data<PgPool>
) -> HttpResponse {

    let result = sqlx::query_as::<_, Modulo>(
        r#"
        SELECT id, strnombremodulo, idmodulopadre
        FROM modulo
        ORDER BY idmodulopadre NULLS FIRST, id
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    let modulos: Vec<Modulo> = match result {
        Ok(m) => m,
        Err(e) => {
            println!("Error cargando menú: {:?}", e);
            return HttpResponse::InternalServerError().body("Error cargando menú");
        }
    };

    let mut menu = Vec::new();

    for padre in modulos.iter().filter(|m| m.idmodulopadre.is_none()) {

        let hijos: Vec<_> = modulos
            .iter()
            .filter(|m| m.idmodulopadre == Some(padre.id))
            .map(|h| {
                json!({
                    "id": h.id,
                    "nombre": h.strnombremodulo
                })
            })
            .collect();

        menu.push(json!({
            "id": padre.id,
            "nombre": padre.strnombremodulo,
            "hijos": hijos
        }));
    }

    HttpResponse::Ok().json(menu)
}