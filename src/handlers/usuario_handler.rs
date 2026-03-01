use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;
use std::collections::HashMap;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::usuario::{Usuario, CrearUsuario};

//
// 📌 GET USUARIOS (Paginado)
//
#[get("/usuario")]
pub async fn get_usuarios(
    pool: web::Data<PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {

    let page: i64 = query.get("page")
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);

    let offset = (page - 1) * 5;

    let usuarios = sqlx::query_as::<_, Usuario>(
        r#"
        SELECT id,
               strnombreusuario,
               strcorreo,
               idperfil,
               bitactivo
        FROM usuario
        ORDER BY id
        LIMIT 5 OFFSET $1
        "#
    )
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    match usuarios {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener usuarios"),
    }
}

//
// 📌 DETALLE USUARIO
//
#[get("/usuario/{id}")]
pub async fn get_usuario_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let usuario = sqlx::query_as::<_, Usuario>(
        r#"
        SELECT id,
               strnombreusuario,
               strcorreo,
               idperfil,
               bitactivo
        FROM usuario
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await;

    match usuario {
        Ok(Some(data)) => HttpResponse::Ok().json(data),
        Ok(None) => HttpResponse::NotFound().body("Usuario no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener usuario"),
    }
}




#[post("/usuarios")]
pub async fn create_usuario(
    pool: web::Data<PgPool>,
    data: web::Json<CrearUsuario>,
) -> HttpResponse {

    if data.strnombreusuario.trim().is_empty()
        || data.strpwd.trim().is_empty()
    {
        return HttpResponse::BadRequest().body("Campos obligatorios vacíos");
    }

    let hashed_password = match hash(&data.strpwd, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Error hash password"),
    };

    let result = sqlx::query(
        r#"
        INSERT INTO usuario (
            strnombreusuario,
            idperfil,
            strpwd,
            idestadousuario,
            strcorreo,
            strnumerocelular
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#
    )
    .bind(&data.strnombreusuario)
    .bind(data.idperfil)
    .bind(hashed_password)
    .bind(data.idestadousuario)
    .bind(&data.strcorreo)
    .bind(&data.strnumerocelular)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Usuario creado"),
        Err(_) => HttpResponse::InternalServerError().body("Error creando usuario"),
    }
}

#[put("/usuarios/{id}")]
pub async fn update_usuario(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    data: web::Json<CrearUsuario>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = if !data.strpwd.trim().is_empty() {

        let hashed_password = match hash(&data.strpwd, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => return HttpResponse::InternalServerError().body("Error hash password"),
        };

        sqlx::query(
            r#"
            UPDATE usuario
            SET strnombreusuario = $1,
                idperfil = $2,
                strpwd = $3,
                idestadousuario = $4,
                strcorreo = $5,
                strnumerocelular = $6
            WHERE id = $7
            "#
        )
        .bind(&data.strnombreusuario)
        .bind(data.idperfil)
        .bind(hashed_password)
        .bind(data.idestadousuario)
        .bind(&data.strcorreo)
        .bind(&data.strnumerocelular)
        .bind(id)
        .execute(pool.get_ref())
        .await

    } else {

        sqlx::query(
            r#"
            UPDATE usuario
            SET strnombreusuario = $1,
                idperfil = $2,
                idestadousuario = $3,
                strcorreo = $4,
                strnumerocelular = $5
            WHERE id = $6
            "#
        )
        .bind(&data.strnombreusuario)
        .bind(data.idperfil)
        .bind(data.idestadousuario)
        .bind(&data.strcorreo)
        .bind(&data.strnumerocelular)
        .bind(id)
        .execute(pool.get_ref())
        .await
    };

    match result {
        Ok(r) if r.rows_affected() > 0u64 =>
            HttpResponse::Ok().body("Usuario actualizado"),
        _ =>
            HttpResponse::InternalServerError().body("Error actualizando usuario"),
    }
}

//
// 📌 DELETE USUARIO
//
#[delete("/usuario/{id}")]
pub async fn delete_usuario(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM usuario WHERE id = $1"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => {
            HttpResponse::Ok().body("Usuario eliminado")
        }
        Ok(_) => HttpResponse::NotFound().body("Usuario no encontrado"),
        Err(_) => HttpResponse::InternalServerError().body("Error al eliminar usuario"),
    }
}

