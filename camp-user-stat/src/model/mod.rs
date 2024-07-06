use chrono::{DateTime, Utc};
use sqlx::{prelude::FromRow, PgExecutor};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserStatError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct UserStat {
    pub email: String,
    pub name: String,
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
