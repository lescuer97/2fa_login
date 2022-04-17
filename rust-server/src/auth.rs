use actix_web::HttpRequest;
use anyhow::{bail, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// this secret is temporary and not for proudction
const SECRET: &str = "AYdW4qq7$x#hjn4CUY%WjvcaUVP6MnBqkr6X6T6Ym2Lu6S5Duv98jciW&s*^N*UNynGnp6^";

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct JWTToken {
    pub sub: String,
    pub exp: usize,
}
impl JWTToken {
    pub fn create_jwt_token(email: &str) -> Result<String> {
        let expiration_time = Utc::now() + Duration::days(30);

        let token_setup = JWTToken {
            sub: email.to_owned(),
            exp: expiration_time.timestamp() as usize,
        };

        let token: String = match encode(
            &Header::default(),
            &token_setup,
            &EncodingKey::from_secret(SECRET.as_ref()),
        ) {
            Ok(token) => token,
            Err(error) => bail!(error),
        };

        Ok(token)
    }
    pub fn validate_jwt_token_from_cookie(request: HttpRequest) -> Result<()> {
        let auth_token = match request.cookie("auth") {
            Some(token) => token.value().to_string(),
            None => bail!("There was a problem extracting the cookie"),
        };

        match decode::<JWTToken>(
            auth_token.as_str(),
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        ) {
            Ok(token) => token,
            Err(error) => {
                bail!("unable to decode token {}", error)
            }
        };

        Ok(())
    }
    pub fn logout_jwt_token() -> Result<String> {
        let expiration_time = Utc::now() - Duration::seconds(1);

        let token_setup = JWTToken {
            sub: "".to_owned(),
            exp: expiration_time.timestamp() as usize,
        };

        let token: String = match encode(
            &Header::default(),
            &token_setup,
            &EncodingKey::from_secret(SECRET.as_ref()),
        ) {
            Ok(token) => token,
            Err(error) => bail!(error),
        };

        Ok(token)
    }
}
#[cfg(test)]
mod tests {
    use crate::{auth::JWTToken, utils::generate_cookie};
    use actix_web::{cookie::Cookie, test::TestRequest};
    use dotenv;
    #[test]
    fn create_jwt_token_success() {
        dotenv::dotenv().ok();
        let email: String = String::from("test@test.com");

        let token = JWTToken::create_jwt_token(&email).unwrap();

        assert_eq!(token.is_empty(), false);
    }

    #[actix_web::test]
    async fn validate_jwt_token_success() {
        dotenv::dotenv().ok();

        let email: String = String::from("test@test.com");
        let jwt_token = JWTToken::create_jwt_token(&email).unwrap();

        let jwt_cookie: Cookie = generate_cookie("auth", jwt_token);

        let req = TestRequest::default().cookie(jwt_cookie).to_http_request();

        let token = JWTToken::validate_jwt_token_from_cookie(req).unwrap();
        assert_eq!((), token);
        // assert_eq!("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0QHRlc3QuY29tIiwiZXhwIjoxMDAwMDAwMDAwMDAwfQ.ZnJtOg79-8NNAinIzFOLM240P_O16BiH5IaQZZvP35g", token);
    }
}
