use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::{PgPool,Row};
use std::collections::HashMap;
use crate::models::perfil::{Perfil, CrearPerfil};

//
// 📌 GET PERFIL (Paginado 5 registros)
//
#[get("/perfil")]
pub async fn get_perfiles(
    pool: web::Data<PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {

    let page: i64 = query.get("page")
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);

    let filtro = query.get("filtro")
        .unwrap_or(&"".to_string())
        .to_string();

    let offset = (page - 1) * 5;

    let perfiles = sqlx::query_as::<_, Perfil>(
        r#"
        SELECT id, strnombreperfil, bitadministrador
        FROM perfil
        WHERE LOWER(strnombreperfil) LIKE LOWER($1)
        ORDER BY id
        LIMIT 5 OFFSET $2
        "#
    )
    .bind(format!("%{}%", filtro))
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    match perfiles {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener perfiles"),
    }
}

//
// 📌 CREATE PERFIL
//
#[post("/perfil")]
pub async fn create_perfil(
    pool: web::Data<PgPool>,
    data: web::Json<CrearPerfil>,
) -> HttpResponse {

    if data.strnombreperfil.trim().is_empty() {
        return HttpResponse::BadRequest().body("El nombre es obligatorio");
    }

    // 🔥 INSERT y obtener ID
    let result = sqlx::query(
        r#"
        INSERT INTO perfil (strnombreperfil, bitadministrador)
        VALUES ($1, $2)
        RETURNING id
        "#
    )
    .bind(&data.strnombreperfil)
    .bind(data.bitadministrador)
    .fetch_one(pool.get_ref())
    .await;

    let row = match result {
        Ok(r) => r,
        Err(_) => return HttpResponse::InternalServerError().body("Error al crear perfil"),
    };

    let idperfil: i32 = row.get("id");

    // 🔥 SI ES ADMIN → DAR TODOS LOS PERMISOS
    if data.bitadministrador {

        let _ = sqlx::query(
            r#"
            INSERT INTO permisosperfil (
                idperfil, idmodulo,
                bitagregar, biteditar,
                bitconsulta, biteliminar, bitdetalle
            )
            SELECT $1, m.id, true, true, true, true, true
            FROM modulo m
            ON CONFLICT (idperfil,idmodulo)
            DO UPDATE SET
                bitagregar = true,
                biteditar = true,
                bitconsulta = true,
                biteliminar = true,
                bitdetalle = true
            "#
        )
        .bind(idperfil)
        .execute(pool.get_ref())
        .await;
    }

    HttpResponse::Ok().body("Perfil creado correctamente")
}

//
// 📌 UPDATE PERFIL
//
#[put("/perfil/{id}")]
pub async fn update_perfil(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    data: web::Json<CrearPerfil>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query(
        r#"
        UPDATE perfil
        SET strnombreperfil = $1,
            bitadministrador = $2
        WHERE id = $3
        "#
    )
    .bind(&data.strnombreperfil)
    .bind(data.bitadministrador)
    .bind(id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => {

            // 🔥 SI ES ADMIN → DAR TODO
            if data.bitadministrador {

                let _ = sqlx::query(
                    r#"
                    INSERT INTO permisosperfil (
                        idperfil, idmodulo,
                        bitagregar, biteditar,
                        bitconsulta, biteliminar, bitdetalle
                    )
                    SELECT $1, m.id, true, true, true, true, true
                    FROM modulo m
                    ON CONFLICT (idperfil,idmodulo)
                    DO UPDATE SET
                        bitagregar = true,
                        biteditar = true,
                        bitconsulta = true,
                        biteliminar = true,
                        bitdetalle = true
                    "#
                )
                .bind(id)
                .execute(pool.get_ref())
                .await;

            } else {
                // 🔥 SI YA NO ES ADMIN → QUITAR TODO

                let _ = sqlx::query(
                    r#"
                    UPDATE permisosperfil
                    SET bitagregar = false,
                        biteditar = false,
                        bitconsulta = false,
                        biteliminar = false,
                        bitdetalle = false
                    WHERE idperfil = $1
                    "#
                )
                .bind(id)
                .execute(pool.get_ref())
                .await;
            }

            HttpResponse::Ok().body("Perfil actualizado")
        }

        Ok(_) => HttpResponse::NotFound().body("Perfil no encontrado"),

        Err(_) => HttpResponse::InternalServerError().body("Error al actualizar"),
    }
}

//
// 📌 DELETE PERFIL
//
#[delete("/perfil/{id}")]
pub async fn delete_perfil(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM perfil WHERE id = $1"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => HttpResponse::Ok().body("Perfil eliminado"),
        Ok(_) => HttpResponse::NotFound().body("Perfil no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al eliminar"),
    }
}

//
// 📌 DETALLE PERFIL
//
#[get("/perfil/{id}")]
pub async fn get_perfil_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let perfil = sqlx::query_as::<_, Perfil>(
        r#"
        SELECT id, strnombreperfil, bitadministrador
        FROM perfil
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await;

    match perfil {
        Ok(Some(data)) => HttpResponse::Ok().json(data),
        Ok(None) => HttpResponse::NotFound().body("Perfil no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener perfil"),
    }
}

#[get("/perfil")]
pub async fn get_perfil(pool: web::Data<PgPool>) -> HttpResponse {

    let perfiles = sqlx::query_as::<_, Perfil>(
        "SELECT id, strnombreperfil FROM perfil ORDER BY strnombreperfil"
    )
    .fetch_all(pool.get_ref())
    .await;

    match perfiles {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            eprintln!("Error perfiles: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}