use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::errors::Error;
use tokio::task;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub locked_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new<A, B, C>(name: A, email: B, password_hash: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        let now = Utc::now().naive_utc();
        Self {
            id: 0, // Will be set by the database
            name: name.into(),
            email: email.into(),
            password: password_hash.into(),
            updated_at: now,
            created_at: now,
            locked_at: None,
        }
    }

    pub fn is_password_match(&self, password: &str) -> bool {
        bcrypt::verify(password, self.password.as_ref()).unwrap_or(false)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            updated_at: DateTime::from_naive_utc_and_offset(user.updated_at, Utc),
            created_at: DateTime::from_naive_utc_and_offset(user.created_at, Utc),
        }
    }
}

pub async fn hash_password<P>(password: P) -> Result<String, Error>
where
    P: AsRef<str> + Send + 'static,
{
    // TODO: Hash password with salt.
    // https://docs.rs/bcrypt/latest/bcrypt/fn.hash_with_salt.html
    #[cfg(not(test))]
    let cost = bcrypt::DEFAULT_COST;
    #[cfg(test)]
    let cost = 4;
    task::spawn_blocking(move || bcrypt::hash(password.as_ref(), cost))
        .await
        .map_err(Error::RunSyncTask)?
        .map_err(Error::HashPassword)
}
