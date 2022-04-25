use std::env;

use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::Cookie;
use actix_web::cookie::{self, Expiration};
use chrono::{DateTime, Utc};

use crate::auth::AuthError;

pub const ENVIROMENT: &str = "ENVIROMENT";

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

pub fn generate_cookie(
    name: &str,
    value: String,
    expiration_time: DateTime<Utc>,
) -> anyhow::Result<Cookie> {
    let enviroment = get_env_variable(ENVIROMENT);
    let formated_offset = match OffsetDateTime::from_unix_timestamp(expiration_time.timestamp()) {
        Ok(offset) => offset,

        Err(_) => return Err(anyhow::anyhow!(AuthError::UnexpectedError)),
    };
    let formated_expiration = Expiration::from(formated_offset);
    let cookie: Cookie = if let Some(env_variable) = enviroment {
        if env_variable == "development" {
            cookie::Cookie::build(name, value)
                .path("/")
                .secure(true)
                .http_only(true)
                .expires(formated_expiration)
                .same_site(cookie::SameSite::Strict)
                .finish()
        } else {
            cookie::Cookie::build(name, value)
                .path("/")
                .domain(".leito.dev")
                .secure(true)
                .http_only(true)
                .expires(formated_expiration)
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
    Ok(cookie)
}
