use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

use crate::models::modulo::Modulo;
use crate::models::menu::{MenuItem, MenuHijo};

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

    let modulos = match result {
        Ok(m) => m,
        Err(_) => return HttpResponse::InternalServerError().body("Error cargando menú")
    };

    let mut menu: Vec<MenuItem> = Vec::new();

    for padre in modulos.iter().filter(|m| m.idmodulopadre.is_none()) {

        let hijos: Vec<MenuHijo> = modulos
            .iter()
            .filter(|m| m.idmodulopadre == Some(padre.id))
            .map(|h| MenuHijo {
                id: h.id,
                nombre: h.strnombremodulo.clone()
            })
            .collect();

        menu.push(MenuItem {
            id: padre.id,
            nombre: padre.strnombremodulo.clone(),
            hijos
        });
    }

    HttpResponse::Ok().json(menu)
}