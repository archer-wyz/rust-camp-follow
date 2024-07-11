use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgExecutor};
use std::collections::HashSet;
use thiserror::Error;
use tokio::sync::OnceCell;

static ONCE: OnceCell<HashSet<String>> = OnceCell::const_new();

#[derive(Debug, Error)]
pub enum UserStatError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "gender", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Female,
    Male,
    Unknown,
}

#[derive(Debug, Clone, FromRow, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserStat {
    pub email: String,
    pub name: String,
    pub gender: Gender,
    pub viewed_but_not_started: Vec<i32>,
    pub recent_watched: Vec<i32>,
    pub started_but_not_finished: Vec<i32>,
    pub finished: Vec<i32>,
    pub created_at: DateTime<Utc>,
    pub last_visited_at: Option<DateTime<Utc>>,
    pub last_watched_at: Option<DateTime<Utc>>,
    pub last_email_notification: Option<DateTime<Utc>>,
    pub last_in_app_notification: Option<DateTime<Utc>>,
    pub last_sms_notification: Option<DateTime<Utc>>,
}

impl Default for UserStat {
    fn default() -> Self {
        UserStat {
            email: "".to_string(),
            name: "".to_string(),
            gender: Gender::Unknown,
            viewed_but_not_started: vec![],
            recent_watched: vec![],
            started_but_not_finished: vec![],
            finished: vec![],
            created_at: DateTime::from_timestamp(0, 0).expect("invalid timestamp"),
            last_visited_at: None,
            last_watched_at: None,
            last_email_notification: None,
            last_in_app_notification: None,
            last_sms_notification: None,
        }
    }
}

impl UserStat {
    async fn _fields() -> Result<HashSet<String>> {
        let mut fields = HashSet::<String>::new();
        let user_stat = UserStat::default();
        let value = serde_json::to_value(user_stat)?;
        if let serde_json::Value::Object(map) = value {
            for (k, _) in map {
                fields.insert(k);
            }
        }
        Ok(fields)
    }
    pub async fn fields() -> Result<&'static HashSet<String>> {
        ONCE.get_or_try_init(UserStat::_fields).await
    }
}

impl<'a> UserStat {
    pub async fn pg_insert<T>(&self, executor: T) -> Result<(), UserStatError>
    where
        T: PgExecutor<'a>,
    {
        sqlx::query(
            r#"
            INSERT INTO user_stats (
                email,
                name,
                viewed_but_not_started,
                recent_watched,
                started_but_not_finished,
                finished,
                created_at,
                last_visited_at,
                last_watched_at,
                last_email_notification,
                last_in_app_notification,
                last_sms_notification
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
        )
        .bind(&self.email)
        .bind(&self.name)
        .bind(&self.viewed_but_not_started)
        .bind(&self.recent_watched)
        .bind(&self.started_but_not_finished)
        .bind(&self.finished)
        .bind(self.created_at)
        .bind(self.last_visited_at)
        .bind(self.last_watched_at)
        .bind(self.last_email_notification)
        .bind(self.last_in_app_notification)
        .bind(self.last_sms_notification)
        .execute(executor)
        .await?;

        Ok(())
    }
}
