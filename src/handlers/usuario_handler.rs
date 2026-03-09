use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;

use bcrypt::{hash, DEFAULT_COST};
use crate::models::usuario::{Usuario, CrearUsuario,UsuarioQuery,UsuarioResponse};
use crate::utils::email::enviar_credenciales;



#[get("/usuario")]
pub async fn get_usuarios(
    pool: web::Data<PgPool>,
    query: web::Query<UsuarioQuery>,
) -> HttpResponse {

    let page = query.page.unwrap_or(1);
    let limit = 10;
    let offset = (page - 1) * limit;

    let usuario = query.usuario.clone().unwrap_or_default();
    let perfil = query.perfil.unwrap_or(0);
    let estado = query.estado.unwrap_or(0);

    // TOTAL
    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM usuario
        WHERE
        ($1 = '' OR strnombreusuario ILIKE '%' || $1 || '%')
        AND ($2 = 0 OR idperfil = $2)
        AND ($3 = 0 OR idestadousuario = $3)
        "#
    )
    .bind(&usuario)
    .bind(perfil)
    .bind(estado)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    // DATA
    let usuarios = sqlx::query_as::<_, Usuario>(
        r#"
        SELECT
            u.id,
            u.strnombreusuario,
            p.strnombreperfil AS perfil,
            e.strnombreestado AS estado,
            u.strcorreo,
            u.strnumerocelular
        FROM usuario u
        JOIN perfil p ON u.idperfil = p.idperfil
        JOIN estadousuario e ON u.idestadousuario = e.idestadousuario
        WHERE
        ($1 = '' OR u.strnombreusuario ILIKE '%' || $1 || '%')
        AND ($2 = 0 OR u.idperfil = $2)
        AND ($3 = 0 OR u.idestadousuario = $3)
        ORDER BY u.strnombreusuario
        LIMIT $4 OFFSET $5
        "#
    )
    .bind(&usuario)
    .bind(perfil)
    .bind(estado)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    match usuarios {
        Ok(data) => HttpResponse::Ok().json(UsuarioResponse {
            total: total.0,
            data,
        }),
        Err(e) => {
            println!("ERROR USUARIOS: {:?}", e);
            HttpResponse::InternalServerError().body("Error cargando usuarios")
        }
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




#[post("/usuario")]
pub async fn create_usuario(
    pool: web::Data<PgPool>,
    data: web::Json<CrearUsuario>,
) -> HttpResponse {

    if data.strnombreusuario.trim().is_empty()
        || data.strpwd.trim().is_empty()
    {
        return HttpResponse::BadRequest().body("Campos obligatorios vacíos");
    }

    let plain_password = data.strpwd.clone();

    let hashed_password = match hash(&plain_password, DEFAULT_COST) {
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

        Ok(_) => {

            // 🔹 Enviar correo
            let _ = enviar_credenciales(
                &data.strcorreo,
                &data.strnombreusuario,
                &plain_password
            );

            HttpResponse::Ok().body("Usuario creado y correo enviado")
        }

        Err(_) => HttpResponse::InternalServerError().body("Error creando usuario"),
    }
}

#[put("/usuario/{id}")]
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

