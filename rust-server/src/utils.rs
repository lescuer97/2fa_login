use std::env;

use actix_web::cookie;
use actix_web::cookie::Cookie;

pub const ENVIROMENT: &'static str = "ENVIROMENT";

pub fn get_env_variable(name: &str) -> Option<String> {
    match env::var(name) {
        Ok(variable) => Some(variable),
        Err(_) => {
            println!(
                "there is no enviroment variable set with the name {} variable set",
                name
            );
            None
        }
    }
}

pub fn generate_cookie(name: &str, value: String) -> Cookie {
    let enviroment = get_env_variable(ENVIROMENT);

    let cookie: Cookie = if let Some(env_variable) = enviroment {
        if env_variable == "development" {
            cookie::Cookie::build(name, value)
                .path("/")
                .secure(true)
                .http_only(true)
                // .expires(EXPIRATION)
                .same_site(cookie::SameSite::Strict)
                .finish()
        } else {
            cookie::Cookie::build(name, value)
                .path("/")
                .domain(".leito.dev")
                .secure(true)
                .http_only(true)
                // .expires(EXPIRATION)
                .same_site(cookie::SameSite::Strict)
                .finish()
        }
    } else {
        cookie::Cookie::build(name, value)
            .path("/")
            .domain(".leito.dev")
            .secure(true)
            .http_only(true)
            // .expires(EXPIRATION)
            .same_site(cookie::SameSite::Strict)
            .finish()
    };
    cookie
}
