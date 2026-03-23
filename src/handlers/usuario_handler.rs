use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;
use actix_multipart::Multipart;
use futures_util::StreamExt;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::usuario::{Usuario, CrearUsuario,UsuarioQuery,UsuarioResponse,UsuarioDetalle,UsuarioFoto};
use crate::utils::email::enviar_credenciales;
use std::io::Write;


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
            e.strdescripcion AS estado,
            u.strcorreo,
            u.strnumerocelular
        FROM usuario u
        JOIN perfil p ON u.idperfil = p.id
        JOIN estadousuario e ON u.idestadousuario = e.id
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
    eprintln!("ERROR SQLX USUARIOS: {:?}", e);
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

    let usuario = sqlx::query_as::<_, UsuarioDetalle>(
        r#"
        SELECT 
            id,
            strnombreusuario,
            strcorreo,
            strnumerocelular,
            idperfil,
            idestadousuario
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
        Err(e) => {
            eprintln!("ERROR SQLX DETALLE USUARIO: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener usuario")
        }
    }
}




#[post("/usuario")]
pub async fn create_usuario(
    pool: web::Data<PgPool>,
    data: web::Json<CrearUsuario>,
) -> HttpResponse {

    if data.strnombreusuario.trim().is_empty()
        || data.strpwd.as_ref().map_or(true, |p| p.trim().is_empty())
    {
        return HttpResponse::BadRequest().body("Campos obligatorios vacíos");
    }

    let plain_password = match &data.strpwd {
    Some(p) => p.clone(),
    None => return HttpResponse::BadRequest().body("Password requerido"),
};

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

    let result = if let Some(password) = &data.strpwd {

        let hashed_password = match hash(password, DEFAULT_COST) {
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
        Ok(r) if r.rows_affected() > 0 =>
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
#[get("/usuario/foto/{id}")]
pub async fn get_usuario_foto(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {

    let id = path.into_inner();

    let result = sqlx::query_as::<_, UsuarioFoto>(
        r#"
        SELECT strnombreusuario, strfoto
        FROM usuario
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().body("Usuario no encontrado"),
    }
}
#[post("/usuario/upload-foto/{id}")]
pub async fn upload_foto(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    mut payload: Multipart,
) -> HttpResponse {

    let id = path.into_inner();
    let mut filepath = String::new();

    while let Some(item) = payload.next().await {

        let mut field = match item {
            Ok(f) => f,
            Err(_) => return HttpResponse::BadRequest().body("Error leyendo archivo"),
        };

        // 🔥 VALIDAR CONTENT TYPE (MIME)
        let content_type = field.content_type().to_string();

        if !content_type.starts_with("image/") {
            return HttpResponse::BadRequest().body("Solo se permiten imágenes");
        }

        // 🔥 VALIDAR EXTENSIÓN
        let cd = field.content_disposition();
        let filename_original = cd
            .get_filename()
            .unwrap_or("file");

        let extension = filename_original
            .split('.')
            .last()
            .unwrap_or("")
            .to_lowercase();

        let extensiones_validas = ["jpg", "jpeg", "png", "webp"];

        if !extensiones_validas.contains(&extension.as_str()) {
            return HttpResponse::BadRequest()
                .body("Formato no permitido (jpg, png, webp)");
        }

        // 🔥 GENERAR NOMBRE SEGURO
        let filename = format!("foto_{}.{}", id, extension);
        let path = format!("./uploads/{}", filename);

        let mut file = match std::fs::File::create(&path) {
            Ok(f) => f,
            Err(_) => return HttpResponse::InternalServerError().body("Error creando archivo"),
        };

        // 🔥 VALIDAR TAMAÑO (máx 2MB)
        let mut size: usize = 0;
        let max_size = 2 * 1024 * 1024;

        while let Some(chunk) = field.next().await {

            let data = match chunk {
                Ok(d) => d,
                Err(_) => return HttpResponse::BadRequest().body("Error leyendo chunk"),
            };

            size += data.len();

            if size > max_size {
                return HttpResponse::BadRequest()
                    .body("Imagen demasiado grande (máx 2MB)");
            }

            if file.write_all(&data).is_err() {
                return HttpResponse::InternalServerError().body("Error guardando archivo");
            }
        }

        filepath = format!("/uploads/{}", filename);
    }

    // 🔥 GUARDAR EN BD
    let _ = sqlx::query(
        "UPDATE usuario SET strfoto = $1 WHERE id = $2"
    )
    .bind(&filepath)
    .bind(id)
    .execute(pool.get_ref())
    .await;

    HttpResponse::Ok().body("Imagen subida correctamente")
}
