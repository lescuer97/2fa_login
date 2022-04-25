use actix_web::web::Data;
use anyhow::{bail, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo, Postgres},
    FromRow, Pool, Type,
};
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Password Doesn't match")]
    PasswordDontMatch,
    #[error("Error from db")]
    DBError(#[source] sqlx::Error),
    #[error("Error wasn't expected")]
    UnexpectedError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginData {
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginDataToSend {
    pub email: String,
    pub password: Option<String>,
    pub u2f: bool,
    pub totp: bool,
}

impl LoginData {
    pub async fn login(&self, pool: Data<Pool<Postgres>>) -> Result<LoginDataToSend> {
        let user_query: LoginDataToSend = match sqlx::query_as!(
            LoginDataToSend,
            r#"
        SELECT email, password, u2f, totp
        FROM users
        where email = $1
            "#,
            self.email
        )
        .fetch_one(pool.get_ref())
        .await
        {
            Ok(user) => user,
            Err(error) => bail!(error),
        };

        self.check_password_matches_database(user_query.to_owned())?;

        Ok(LoginDataToSend {
            email: user_query.email,
            password: None,
            u2f: user_query.u2f,
            totp: user_query.totp,
        })
    }
    fn check_password_matches_database(&self, user_query: LoginDataToSend) -> Result<()> {
        let password_hash_string = user_query.password.unwrap();

        let parsed_hash = match PasswordHash::new(password_hash_string.as_str()) {
            Ok(hashed_value) => hashed_value,
            Err(error) => bail!(error),
        };

        let password_match = Argon2::default()
            .verify_password(self.password.clone().unwrap().as_bytes(), &parsed_hash)
            .is_ok();

        if password_match {
            Ok(())
        } else {
            bail!("Password Doesn't match")
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct RegisterUserData {
    pub email: String,
    pub password: String,
    pub password_repeat: String,
    pub rol: Option<Vec<UserRole>>,
    pub id: Option<Uuid>,
    pub u2f: Option<bool>,
    pub totp: Option<bool>,
}

impl RegisterUserData {
    // fn new(&self) -> Self {}

    fn set_default_user(&mut self) -> Result<()> {
        let same_password = self.check_same_password();

        if !same_password {
            bail!("passwords are not the same");
        }
        let uuid = Uuid::new_v4();

        self.rol = Some(vec![UserRole::EditSelf]);
        self.id = Some(uuid);
        self.password = self.hash_password()?;
        self.u2f = Some(false);
        self.totp = Some(false);

        Ok(())
    }

    pub async fn register(&mut self, pool: Data<Pool<Postgres>>) -> Result<()> {
        self.set_default_user()?;

        let mut conn = pool.acquire().await.unwrap();

        match sqlx::query_as!(
            RegisterUserData,
            r#"
    INSERT INTO users ( email, password,id, u2f, totp, rol)
    VALUES ( $1, $2, $3, $4, $5, $6::user_role[])
            "#,
            self.email,
            self.password,
            self.id,
            self.u2f,
            self.totp,
            self.rol.to_owned() as _,
        )
        .execute(&mut conn)
        .await
        {
            Ok(user) => user,
            Err(error) => {
                println!("error: {}", error);
                bail!(error)
            }
        };

        Ok(())
    }

    fn check_same_password(&self) -> bool {
        self.password == self.password_repeat
    }
    fn hash_password(&self) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        let password_bytes = self.password.as_bytes();

        let password_hash = match argon2.hash_password(password_bytes, &salt) {
            Ok(hashed_password) => hashed_password.to_string(),
            Err(_) => bail!("There was a problem hashing the password"),
        };

        let parsed_hash = match PasswordHash::new(&password_hash) {
            Ok(hashed_value) => hashed_value,
            Err(_) => bail!("There was a getting the hashed password"),
        };

        Ok(parsed_hash.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[sqlx(transparent)]
pub struct TwoFactorActivated(u8);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    EditSelf,
    EditOther,
    RemoveOther,
    WatchOther,
    SuperAdmin,
}
impl PgHasArrayType for UserRole {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_user_role")
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::users::{RegisterUserData, UserRole};
    // use actix_web::test::TestRequest;

    #[test]
    fn register_user_same_password_success() {
        let uuid = Uuid::new_v4();

        let register_user: RegisterUserData = RegisterUserData {
            email: "test@test.com".to_string(),
            password: "String".to_string(),
            password_repeat: "String".to_string(),
            rol: Some(vec![UserRole::EditSelf]),
            id: Some(uuid),
            u2f: Some(false),
            totp: Some(false),
        };
        let check = register_user.check_same_password();

        assert_eq!(true, check);
    }

    #[test]
    fn register_user_same_password_fail() {
        let uuid = Uuid::new_v4();

        let register_user: RegisterUserData = RegisterUserData {
            email: "test@test.com".to_string(),
            password: "String".to_string(),
            password_repeat: "String_different".to_string(),
            rol: Some(vec![UserRole::EditSelf]),
            id: Some(uuid),
            u2f: Some(false),
            totp: Some(false),
        };
        let check = register_user.check_same_password();

        assert_eq!(false, check);
    }
}
