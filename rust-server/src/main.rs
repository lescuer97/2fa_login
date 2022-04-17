use actix_cors::Cors;
use actix_web::cookie::Cookie;
use actix_web::{
    cookie, guard, http, http::header::ContentType, middleware, post, web, App, HttpRequest,
    HttpResponse, HttpServer,
};
use rust_server::auth::JWTToken;
use rust_server::server_messages::ResponseBodyMessage;
use rust_server::users::{LoginData, RegisterUserData};
use sqlx::{PgPool, Pool, Postgres};
use std::env;

async fn options_call() -> HttpResponse {
    HttpResponse::Ok()
        // .append_header(("Access-Control-Allow-Origin", "*"))
        .content_type(ContentType::json())
        .finish() // <
}

/// This handler uses json extractor
async fn login_function(
    req: HttpRequest,
    item: web::Json<LoginData>,
    pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    // used in case that the client calls when already signed in
    match JWTToken::validate_jwt_token_from_cookie(req) {
        Ok(()) => {
            let already_logedin_value = ResponseBodyMessage::success_message("Already Logged in");

            return HttpResponse::Accepted()
                .content_type(ContentType::json())
                .json(already_logedin_value);
        }
        Err(_) => println!("No jwt token"),
    }

    match &item.login(pool).await {
        Ok(()) => (),
        Err(error) => {
            println!("login Error {:?}", error);
            return HttpResponse::NotFound()
                .content_type(ContentType::json())
                .body("Incorrect username or password");
        }
    };

    let jwt: String = match JWTToken::create_jwt_token(&item.email) {
        Ok(token) => token,
        Err(_) => {
            return HttpResponse::NotFound()
                .content_type(ContentType::json())
                .body("Incorrect username or password");
        }
    };
    let jwt_cookie: Cookie = generate_cookie("auth", jwt);

    let success_registering = ResponseBodyMessage::success_message(json!("Logged in"));

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .cookie(jwt_cookie)
        .json(success_registering)
}

#[get("/auth/checklogin")]
pub async fn check_login(req: HttpRequest) -> HttpResponse {
    let json_token_valid = match JWTToken::validate_jwt_token_from_cookie(req) {
        Ok(()) => true,
        Err(_) => {
            println!("No jwt token");
            false
        }
    };
    if json_token_valid {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .finish()
    } else {
        HttpResponse::TemporaryRedirect().finish()
    }
}

    };

    HttpResponse::Ok().cookie(jwt_cookie).finish()
}

#[post("/auth/register")]
pub async fn register_user(
    mut item: web::Json<RegisterUserData>,
    pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    match item.register(pool).await {
        Ok(()) => (),
        Err(error) => {
            println!("error {:?}", error);
            return HttpResponse::UnprocessableEntity()
                .content_type(ContentType::json())
                .body("");
        }
    };

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
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(
                web::resource("/auth/login")
                    .route(web::route().guard(guard::Options()).to(options_call))
                    .route(web::post().to(login_function)),
            )
            .service(register_user)
    })
    .bind(("localhost", 3010))?
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
        println!("res: {:?}", res);
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
