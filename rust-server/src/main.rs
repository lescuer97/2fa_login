use actix_cors::Cors;
use actix_web::{get, guard, http, middleware, web, App, HttpResponse, HttpServer};
use rust_server::routes::auth::{check_login, login_function, logout, register_user};
use serde_json::json;
use sqlx::{PgPool, Pool, Postgres};
use std::env;

async fn options_call() -> HttpResponse {
    HttpResponse::Ok().finish() // <
}

#[get("/api/content")]
pub async fn content() -> HttpResponse {
    let values = json!([{"user":"leo", "gender":"male"},{"user":"valeria", "gender":"mujer"}]);

    HttpResponse::Ok().json(values)
}
#[get("/api/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let database_env_var: String = match env::var("DATABASE_URL") {
        Ok(variable) => variable,
        Err(_) => {
            println!("there is no DATABASE_URL variable set");
            String::from("")
        }
    };

    let pool: Pool<Postgres> = PgPool::connect(&database_env_var).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://leito.dev")
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("https://login.localhost")
            .allowed_methods(vec!["GET", "POST"])
            .supports_credentials()
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            // enable logger
            .service(health)
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(
                web::resource("/auth/login")
                    .route(web::route().guard(guard::Options()).to(options_call))
                    .route(web::post().to(login_function)),
            )
            .service(register_user)
            .service(content)
            .service(logout)
            .service(check_login)
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::{login_function, options_call, register_user};
    use actix_web::{guard, http::StatusCode, test, web, App};
    use dotenv;
    use rand::{thread_rng, Rng};
    use serde_json::json;
    use sqlx;
    use sqlx::postgres::PgPool;
    use std::env;

    #[actix_web::test]
    async fn register_new_user_success() {
        dotenv::dotenv().ok();
        let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        let mut rng = thread_rng();

        let random_number: u32 = rng.gen_range(0..999999);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register_user),
        )
        .await;
        let email_to_register = format!("test{}@test.com", random_number);
        let data = json!({
            "email": email_to_register,
            "password": "&#8V*n%!WL5^544#Z7xr",
            "password_repeat": "&#8V*n%!WL5^544#Z7xr",
        });

        // Create request object
        let req = test::TestRequest::post()
            .set_json(data)
            .uri("/auth/register")
            // .header(header::CONTENT_TYPE, "application/json")
            .to_request();

        // Execute application
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn register_new_user_already_exists() {
        dotenv::dotenv().ok();
        let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register_user),
        )
        .await;

        let data = json!({
            "email": "test22s@test.com",
            "password": "&#8V*n%!WL5^544#Z7xr",
            "password_repeat": "&#8V*n%!WL5^544#Z7xr",
        });

        // Create request object
        let req = test::TestRequest::post()
            .set_json(data)
            .uri("/auth/register")
            // .header(header::CONTENT_TYPE, "application/json")
            .to_request();

        // Execute application
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[actix_web::test]
    async fn login_user() {
        dotenv::dotenv().ok();

        let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let app = test::init_service(
            App::new().app_data(web::Data::new(pool.clone())).service(
                web::resource("/auth/login")
                    .route(web::route().guard(guard::Options()).to(options_call))
                    .route(web::post().to(login_function)),
            ),
        )
        .await;

        let data = json!({
            "email": "test22s@test.com",
            "password": "&#8V*n%!WL5^544#Z7xr",
        });

        // Create request object
        let req = test::TestRequest::post()
            .set_json(data)
            .uri("/auth/login")
            .to_request();

        // Execute application
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
