use actix_web::{App, HttpServer, web, middleware::Logger};
use actix_cors::Cors;
use actix_files::Files;
use dotenv::dotenv;
use std::env;
use bcrypt::{hash, DEFAULT_COST};
use sqlx::Row;
mod db;
mod middleware;
mod handlers;
mod utils;
mod models;

use db::connect_db;

use middleware::jwt_middleware::JwtMiddleware;
use middleware::permission_middleware::PermissionMiddleware;

use handlers::auth_handler::login;
use handlers::perfil_handler::*;
use handlers::usuario_handler::*;
use handlers::modulo_handler::*;
use handlers::permisos_handler::*;
use handlers::principal_handler::*;
use handlers::menu_handler::*;


/// ===============================
/// 🔥 SEED AUTOMÁTICO
/// ===============================
async fn seed_database(pool: &sqlx::PgPool) {

    println!("🌱 Ejecutando seed...");

    // ===============================
    // 🔎 Buscar perfil Administrador
    // ===============================
    let perfil = sqlx::query(
        "SELECT id FROM perfil WHERE strnombreperfil = 'Administrador'"
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    let id_perfil: i32 = if let Some(row) = perfil {
        row.get("id")
    } else {
        let result = sqlx::query(
            "INSERT INTO perfil (strnombreperfil, bitadministrador)
             VALUES ('Administrador', true)
             RETURNING id"
        )
        .fetch_one(pool)
        .await
        .unwrap();

        result.get("id")
    };

    // ===============================
    // 📦 Insertar módulos base
    // ===============================
    let modulos_base = vec![
        "perfil",
        "usuario",
        "modulo",
        "permisosperfil",
        "principal1_1",
        "principal1_2",
        "principal2_1",
        "principal2_2",
    ];

    for nombre in modulos_base {
        sqlx::query(
            "INSERT INTO modulo (strnombremodulo)
             VALUES ($1)
             ON CONFLICT (strnombremodulo) DO NOTHING"
        )
        .bind(nombre)
        .execute(pool)
        .await
        .unwrap();
    }

    // ===============================
    // 🔎 Obtener módulos
    // ===============================
    let modulos = sqlx::query("SELECT id FROM modulo")
        .fetch_all(pool)
        .await
        .unwrap();

    for m in modulos {

        let id_modulo: i32 = m.get("id");

        sqlx::query(
            r#"
            INSERT INTO permisosperfil (
                idperfil, idmodulo,
                bitagregar, biteditar, bitconsulta,
                biteliminar, bitdetalle
            )
            VALUES ($1, $2, true, true, true, true, true)
            ON CONFLICT DO NOTHING
            "#
        )
        .bind(id_perfil)
        .bind(id_modulo)
        .execute(pool)
        .await
        .unwrap();
    }

    // ===============================
    // 🔎 Buscar superadmin
    // ===============================
    let usuario = sqlx::query(
        "SELECT id FROM usuario WHERE strnombreusuario = 'superadmin'"
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    if usuario.is_none() {

        let hashed_pwd = hash("123456", DEFAULT_COST).unwrap();

        sqlx::query(
            r#"
            INSERT INTO usuario (
                strnombreusuario,
                idperfil,
                strpwd,
                idestadousuario,
                strcorreo,
                strnumerocelular
            )
            VALUES ($1, $2, $3, 1, $4, $5)
            "#
        )
        .bind("superadmin")
        .bind(id_perfil)
        .bind(hashed_pwd)
        .bind("admin@admin.com")
        .bind("0000000000")
        .execute(pool)
        .await
        .unwrap();

        println!("🔥 Superadmin creado: superadmin / 123456");
    }

    println!("✅ Seed completado");
}

/// ===============================
/// 🚀 MAIN
/// ===============================
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();

    let pool = connect_db().await;

    seed_database(&pool).await;

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("Puerto inválido");

    println!("🚀 Servidor iniciado en puerto {}", port);

    HttpServer::new(move || {

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))

          

            // 🔓 LOGIN
            .service(login)

            // 🔐 API PROTEGIDA
            .service(
                web::scope("/api")
                    .wrap(PermissionMiddleware { pool: pool.clone() })
                    .wrap(JwtMiddleware)
                  

                    .service(get_perfiles)
                    .service(get_perfil_by_id)
                    .service(create_perfil)
                    .service(update_perfil)
                    .service(delete_perfil)

                    .service(get_usuarios)
                    .service(get_usuario_by_id)
                    .service(create_usuario)
                    .service(update_usuario)
                    .service(delete_usuario)

                    .service(get_menu)
                   

                    .service(get_permisos_por_perfil)
                    .service(get_permiso_by_id)
                    .service(create_permiso)
                    .service(guardar_permisos)
                    .service(delete_permiso)

                    .service(mis_permisos)

                    .service(principal1)
                    .service(principal1_1)
                    .service(principal1_2)
                    .service(principal2)
                    .service(principal2_1)
                    .service(principal2_2)
            )
              // 📁 STATIC
            .service(
                Files::new("/", "./static")
                    .index_file("login.html")
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}