use crate::auth::{JWTToken, AUTHENTIFIED_COOKIE_NAME, TOTP_COOKIE_NAME};
use crate::server_messages::ResponseBodyMessage;
use crate::users::{LoginData, RegisterUserData};
use crate::utils::generate_cookie;

use actix_web::cookie::Cookie;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use chrono::{DateTime, Duration, Utc};
use serde_json::json;
use sqlx::{Pool, Postgres};

#[post("/auth/register")]
pub async fn register_user(
    mut item: web::Json<RegisterUserData>,
    pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    match item.register(pool).await {
        Ok(()) => (),
        Err(error) => {
            println!("error {:?}", error);
            return HttpResponse::UnprocessableEntity().body("");
        }
    };
    let success_registering = ResponseBodyMessage::success_message(json!("Registed successfuly"));

    HttpResponse::Ok().json(success_registering)
}
#[post("/auth/logout")]
pub async fn logout() -> HttpResponse {
    let expiration_time: DateTime<Utc> = Utc::now();

    let mut jwt_cookie =
        match generate_cookie(AUTHENTIFIED_COOKIE_NAME, "".to_string(), expiration_time) {
            Ok(err) => err,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

    jwt_cookie.make_removal();

    let logout_response = ResponseBodyMessage::success_message("Logged out user");

    println!("logout_response {:?}", logout_response);
    HttpResponse::Ok().cookie(jwt_cookie).json(logout_response)
}

/// This handler uses json extractor
pub async fn login_function(
    req: HttpRequest,
    item: web::Json<LoginData>,
    pool: web::Data<Pool<Postgres>>,
) -> HttpResponse {
    // used in case that the client calls when already signed in
    match JWTToken::validate_jwt_token_from_cookie(req, AUTHENTIFIED_COOKIE_NAME) {
        Ok(()) => {
            let already_logedin_value = ResponseBodyMessage::success_message("Already Logged in");

            return HttpResponse::Accepted().json(already_logedin_value);
        }
        Err(_) => println!("No jwt token"),
    }

    let login_data = match item.login(pool).await {
        Ok(data) => data,
        Err(error) => {
            println!("login Error {:?}", error);
            let incorrect_username =
                ResponseBodyMessage::success_message("Incorrect username or password");
            return HttpResponse::UnprocessableEntity().json(incorrect_username);
        }
    };

    if login_data.totp || login_data.u2f {
        let expiration_time: DateTime<Utc> = Utc::now() + Duration::minutes(5);

        let jwt: String = match JWTToken::create_jwt_token(&item.email, expiration_time) {
            Ok(token) => token,
            Err(_) => {
                let incorrect_username =
                    ResponseBodyMessage::success_message("Incorrect username or password");

                return HttpResponse::UnprocessableEntity().json(incorrect_username);
            }
        };
        let jwt_cookie: Cookie = match generate_cookie(TOTP_COOKIE_NAME, jwt, expiration_time) {
            Ok(cookie) => cookie,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

        let success_registering = ResponseBodyMessage::success_message(login_data);

        HttpResponse::Ok()
            .cookie(jwt_cookie)
            .json(success_registering)
    } else {
        let expiration_time: DateTime<Utc> = Utc::now() + Duration::days(30);

        let jwt: String = match JWTToken::create_jwt_token(&item.email, expiration_time) {
            Ok(token) => token,
            Err(_) => {
                let incorrect_username =
                    ResponseBodyMessage::success_message("Incorrect username or password");

                return HttpResponse::UnprocessableEntity().json(incorrect_username);
            }
        };
        let expiration_time: DateTime<Utc> = Utc::now() + Duration::days(30);
        let jwt_cookie: Cookie =
            match generate_cookie(AUTHENTIFIED_COOKIE_NAME, jwt, expiration_time) {
                Ok(err) => err,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };

        let success_registering = ResponseBodyMessage::success_message(login_data);

        HttpResponse::Ok()
            .cookie(jwt_cookie)
            .json(success_registering)
    }
}

#[post("/auth/totp")]
pub async fn totp_check(_req: HttpRequest) -> HttpResponse {
    let values = json!([{"user":"leo", "gender":"male"},{"user":"valeria", "gender":"mujer"}]);

    HttpResponse::Ok().json(values)
}

#[get("/auth/checklogin")]
pub async fn check_login(req: HttpRequest) -> HttpResponse {
    match JWTToken::validate_jwt_token_from_cookie(req, AUTHENTIFIED_COOKIE_NAME) {
        Ok(()) => return HttpResponse::Ok().finish(),
        Err(_) => {
            let expiration_time: DateTime<Utc> = Utc::now();

            let mut jwt_cookie =
                match generate_cookie(AUTHENTIFIED_COOKIE_NAME, "".to_string(), expiration_time) {
                    Ok(err) => err,
                    Err(_) => return HttpResponse::InternalServerError().finish(),
                };

            jwt_cookie.make_removal();
            return HttpResponse::UnprocessableEntity()
                .cookie(jwt_cookie)
                .finish();
        }
    };
}
