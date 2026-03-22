use actix_web::{get, post, put, delete, web, HttpResponse,HttpRequest,HttpMessage};
use sqlx::PgPool;
use std::collections::HashMap;
use crate::models::permisos::{PermisoPerfil, CrearPermisoPerfil,PermisoModulo};
use crate::utils::jwt::Claims;

//
// 📌 GET PERMISOS (Paginado 5 registros)
//
#[get("/permisosperfil/{idperfil}")]
pub async fn get_permisos_por_perfil(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let idperfil = path.into_inner();

    let permisos = sqlx::query_as::<_, PermisoPerfil>(
        r#"
        SELECT 
            m.id as idmodulo,
            m.strnombremodulo as modulo,
            COALESCE(p.idperfil, $1) as idperfil,
            p.id,
            COALESCE(p.bitagregar,false) as bitagregar,
            COALESCE(p.biteditar,false) as biteditar,
            COALESCE(p.bitconsulta,false) as bitconsulta,
            COALESCE(p.biteliminar,false) as biteliminar,
            COALESCE(p.bitdetalle,false) as bitdetalle
        FROM modulo m
        LEFT JOIN permisosperfil p 
            ON p.idmodulo = m.id 
            AND p.idperfil = $1

        -- 🔥 SOLO HIJOS (NO PADRES)
        WHERE NOT EXISTS (
            SELECT 1 FROM modulo hijo
            WHERE hijo.idmodulopadre = m.id
        )

        ORDER BY m.id
        "#
    )
    .bind(idperfil)
    .fetch_all(pool.get_ref())
    .await;

    match permisos {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error obteniendo permisos"),
    }
}

//
// 📌 DETALLE PERMISO
//
#[get("/permisosperfil/{id}")]
pub async fn get_permiso_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let permiso = sqlx::query_as::<_, PermisoPerfil>(
        r#"
        SELECT id, idperfil, idmodulo,
               bitagregar, biteditar, bitconsulta,
               biteliminar, bitdetalle
        FROM permisosperfil
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await;

    match permiso {
        Ok(Some(data)) => HttpResponse::Ok().json(data),
        Ok(None) => HttpResponse::NotFound().body("Permiso no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener permiso"),
        Err(e) => {
        println!("ERROR SQL: {:?}", e);
        HttpResponse::InternalServerError().body("Error obteniendo permisos")
    }
    }
}

//
// 📌 CREATE PERMISO
//
#[post("/permisosperfil")]
pub async fn create_permiso(
    pool: web::Data<PgPool>,
    data: web::Json<Vec<CrearPermisoPerfil>>,
) -> HttpResponse {

    for mut p in data.into_inner() {

        // 🔥 reglas
        if p.bitagregar || p.biteditar || p.biteliminar {
            p.bitdetalle = true;
        }

        if p.bitdetalle {
            p.bitconsulta = true;
        }

        let _ = sqlx::query(
            r#"
            INSERT INTO permisosperfil(
                idperfil,idmodulo,
                bitagregar,biteditar,
                bitconsulta,biteliminar,bitdetalle
            )
            VALUES($1,$2,$3,$4,$5,$6,$7)
            ON CONFLICT (idperfil,idmodulo)
            DO UPDATE SET
                bitagregar = EXCLUDED.bitagregar,
                biteditar = EXCLUDED.biteditar,
                bitconsulta = EXCLUDED.bitconsulta,
                biteliminar = EXCLUDED.biteliminar,
                bitdetalle = EXCLUDED.bitdetalle
            "#
        )
        .bind(p.idperfil)
        .bind(p.idmodulo)
        .bind(p.bitagregar)
        .bind(p.biteditar)
        .bind(p.bitconsulta)
        .bind(p.biteliminar)
        .bind(p.bitdetalle)
        .execute(pool.get_ref())
        .await;
    }

    HttpResponse::Ok().body("Permisos creados correctamente")
}

//
// 📌 UPDATE PERMISO
//
#[put("/permisosperfil")]
pub async fn guardar_permisos(
    pool: web::Data<PgPool>,
    data: web::Json<Vec<PermisoPerfil>>,
) -> HttpResponse {

    for mut p in data.into_inner() {

        // 🔥 REGLAS AUTOMÁTICAS

        // Si tiene CRUD → activar detalle
        if p.bitagregar || p.biteditar || p.biteliminar {
            p.bitdetalle = true;
        }

        // Si tiene detalle → activar consulta
        if p.bitdetalle {
            p.bitconsulta = true;
        }

        let _ = sqlx::query(
            r#"
            INSERT INTO permisosperfil(
                idperfil,idmodulo,
                bitagregar,biteditar,
                bitconsulta,biteliminar,bitdetalle
            )
            VALUES($1,$2,$3,$4,$5,$6,$7)
            ON CONFLICT (idperfil,idmodulo)
            DO UPDATE SET
                bitagregar = EXCLUDED.bitagregar,
                biteditar = EXCLUDED.biteditar,
                bitconsulta = EXCLUDED.bitconsulta,
                biteliminar = EXCLUDED.biteliminar,
                bitdetalle = EXCLUDED.bitdetalle
            "#
        )
        .bind(p.idperfil)
        .bind(p.idmodulo)
        .bind(p.bitagregar)
        .bind(p.biteditar)
        .bind(p.bitconsulta)
        .bind(p.biteliminar)
        .bind(p.bitdetalle)
        .execute(pool.get_ref())
        .await;
    }

    HttpResponse::Ok().body("Permisos guardados")
}

//
// 📌 DELETE PERMISO
//
#[delete("/permisosperfil/{id}")]
pub async fn delete_permiso(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM permisosperfil WHERE id = $1"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => {
            HttpResponse::Ok().body("Permiso eliminado")
        }
        Ok(_) => HttpResponse::NotFound().body("Permiso no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al eliminar permiso"),
    }
}


#[get("/mis-permisos")]
pub async fn mis_permisos(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {

    let claims = match req.extensions().get::<Claims>().cloned() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().body("No autorizado"),
    };

let permisos = sqlx::query_as::<_, PermisoModulo>(
    r#"
    SELECT 
        m.strnombremodulo AS modulo,
        COALESCE(p.bitconsulta, false) AS bitconsulta,
        COALESCE(p.bitagregar, false) AS bitagregar,
        COALESCE(p.biteditar, false) AS biteditar,
        COALESCE(p.bitdetalle, false) AS bitdetalle,
        COALESCE(p.biteliminar, false) AS biteliminar
    FROM permisosperfil p
    JOIN modulo m ON m.id = p.idmodulo
    WHERE p.idperfil = $1
    "#
)
.bind(claims.id_perfil)
.fetch_all(pool.get_ref())
.await;

    match permisos {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => {
            println!("Error permisos: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener permisos")
        }
    }
}