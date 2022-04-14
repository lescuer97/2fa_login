use actix_web::HttpRequest;
use anyhow::{bail, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// this secret is temporary and not for proudction
const SECRET: &str = "AYdW4qq7$x#hjn4CUY%WjvcaUVP6MnBqkr6X6T6Ym2Lu6S5Duv98jciW&s*^N*UNynGnp6^";
pub const EXPIRATION: usize = 1000000000000;

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTToken {
    pub sub: String,
    pub exp: usize,
}
impl JWTToken {
    pub fn create_jwt_token(email: &str) -> Result<String> {
        let token_setup = JWTToken {
            sub: email.to_owned(),
            exp: EXPIRATION,
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
    pub fn validate_jwt_token(request: HttpRequest) -> Result<()> {
        let auth_token = match request.cookie("auth") {
            Some(token) => token.value().to_string(),
            None => bail!("No jwt Token"),
        };

        let token_message = match decode::<JWTToken>(
            auth_token.as_str(),
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        ) {
            Ok(token) => token,
            Err(error) => {
                bail!("unable to decode token {}", error)
            }
        };

        println!("token_message {:?}", token_message);
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::auth::JWTToken;
    use actix_web::test::TestRequest;
    #[test]
    fn create_jwt_token_success() {
        let email: String = String::from("test@test.com");

        let token = JWTToken::create_jwt_token(&email).unwrap();

        assert_eq!("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0QHRlc3QuY29tIiwiZXhwIjoxMDAwMDAwMDAwMDAwfQ.ZnJtOg79-8NNAinIzFOLM240P_O16BiH5IaQZZvP35g", token);
    }

    #[actix_web::test]
    async fn validate_jwt_token_success() {
        let email: String = String::from("test@test.com");
        let jwt_token = JWTToken::create_jwt_token(&email).unwrap();

        let req = TestRequest::default()
            .insert_header(("Cookie", format!("auth={}", jwt_token)))
            .to_http_request();

        let token = JWTToken::validate_jwt_token(req).unwrap();

        assert_eq!((), token);
        // assert_eq!("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0QHRlc3QuY29tIiwiZXhwIjoxMDAwMDAwMDAwMDAwfQ.ZnJtOg79-8NNAinIzFOLM240P_O16BiH5IaQZZvP35g", token);
    }
}
