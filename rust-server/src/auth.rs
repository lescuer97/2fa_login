use actix_web::HttpRequest;
use anyhow::{bail, Result};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::get_env_variable;

pub const AUTHENTIFIED_COOKIE_NAME: &str = "auth";
pub const TOTP_COOKIE_NAME: &str = "auth-2fa";

#[derive(Debug, Serialize, Deserialize, thiserror::Error, Copy, Clone)]
pub enum AuthError {
    #[error("No JWT Error")]
    NoJWTToken,
    #[error("The token is not valid")]
    InvalidToken,
    #[error("Unexpected error has ocurred")]
    UnexpectedError,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct JWTToken {
    pub sub: String,
    pub exp: usize,
}
impl JWTToken {
    pub fn create_jwt_token(email: &str, duration: DateTime<Utc>) -> Result<String> {
        let secret_jwt = get_env_variable("SECRET_FOR_JWT");
        let token_setup = JWTToken {
            sub: email.to_owned(),
            exp: duration.timestamp() as usize,
        };

        let token: String = match encode(
            &Header::default(),
            &token_setup,
            &EncodingKey::from_secret(secret_jwt.as_ref()),
        ) {
            Ok(token) => token,
            Err(error) => bail!(error),
        };

        Ok(token)
    }
    pub fn validate_jwt_token_from_cookie(
        request: HttpRequest,
        name_of_token: &str,
    ) -> Result<(), AuthError> {
        let auth_token = match request.cookie(name_of_token) {
            Some(token) => token.value().to_string(),
            None => return Err(AuthError::NoJWTToken),
        };

        let secret_jwt = get_env_variable("SECRET_FOR_JWT");

        match decode::<JWTToken>(
            auth_token.as_str(),
            &DecodingKey::from_secret(secret_jwt.as_ref()),
            &Validation::default(),
        ) {
            Ok(token) => token,
            Err(_) => return Err(AuthError::InvalidToken),
        };

        Ok(())
    }
    pub fn logout_jwt_token() -> Result<String> {
        let expiration_time = Utc::now() - Duration::seconds(1);

        let token_setup = JWTToken {
            sub: "".to_owned(),
            exp: expiration_time.timestamp() as usize,
        };

        let secret_jwt = get_env_variable("SECRET_FOR_JWT");

        let token: String = match encode(
            &Header::default(),
            &token_setup,
            &EncodingKey::from_secret(secret_jwt.as_ref()),
        ) {
            Ok(token) => token,
            Err(error) => bail!(error),
        };

        Ok(token)
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        auth::{JWTToken, AUTHENTIFIED_COOKIE_NAME},
        utils::generate_cookie,
    };
    use actix_web::{cookie::Cookie, test::TestRequest};
    use chrono::{DateTime, Duration, Utc};

    use dotenv;
    #[test]
    fn create_jwt_token_success() {
        dotenv::dotenv().ok();
        let email: String = String::from("test@test.com");
        let expiration_time: DateTime<Utc> = Utc::now() + Duration::days(30);

        let token = JWTToken::create_jwt_token(&email, expiration_time).unwrap();

        assert_eq!(token.is_empty(), false);
    }

    #[actix_web::test]
    async fn validate_jwt_token_success() {
        dotenv::dotenv().ok();

        let email: String = String::from("test@test.com");
        let expiration_time: DateTime<Utc> = Utc::now() + Duration::days(30);

        let jwt_token = JWTToken::create_jwt_token(&email, expiration_time).unwrap();

        let jwt_cookie: Cookie =
            generate_cookie(AUTHENTIFIED_COOKIE_NAME, jwt_token, expiration_time).unwrap();

        let req = TestRequest::default().cookie(jwt_cookie).to_http_request();

        let token =
            JWTToken::validate_jwt_token_from_cookie(req, AUTHENTIFIED_COOKIE_NAME).unwrap();
        assert_eq!((), token);
        // assert_eq!("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0QHRlc3QuY29tIiwiZXhwIjoxMDAwMDAwMDAwMDAwfQ.ZnJtOg79-8NNAinIzFOLM240P_O16BiH5IaQZZvP35g", token);
    }
    #[actix_web::test]
    async fn invalid_jwt_token() {
        dotenv::dotenv().ok();

        let email: String = String::from("test@test.com");
        let expiration_time: DateTime<Utc> = Utc::now() - Duration::days(30);

        let jwt_token = JWTToken::create_jwt_token(&email, expiration_time).unwrap();

        let jwt_cookie: Cookie =
            generate_cookie(AUTHENTIFIED_COOKIE_NAME, jwt_token, expiration_time).unwrap();

        let req = TestRequest::default().cookie(jwt_cookie).to_http_request();

        let token = JWTToken::validate_jwt_token_from_cookie(req, AUTHENTIFIED_COOKIE_NAME);
        assert_eq!(true, token.is_err());
    }
}
