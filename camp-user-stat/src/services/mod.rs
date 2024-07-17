use crate::model::UserStat;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use itertools::Itertools as _;
use sqlx::FromRow;
use std::collections::HashMap;
use thiserror::Error;
use tonic::async_trait;
use tracing::info;

#[derive(Debug, Error)]
pub enum UserStatError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("field not found: {0}")]
    FieldNotFound(String),

    #[error("unknown error {0}")]
    Any(#[from] anyhow::Error),
}

pub struct TimeQuery {
    pub(crate) lower: Option<DateTime<Utc>>,
    pub(crate) upper: Option<DateTime<Utc>>,
}

pub struct IdQuery {
    pub(crate) ids: Vec<u32>,
}

pub struct Query {
    pub(crate) timestamps: HashMap<String, TimeQuery>,
    pub(crate) ids: HashMap<String, IdQuery>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserStatVO {
    pub email: String,
    pub name: String,
    #[sqlx(default)]
    pub started_but_not_finished: Option<Vec<i32>>,
}

#[async_trait]
pub trait UserStatService {
    async fn query(
        &self,
        request: impl Into<Query> + Send,
    ) -> Result<Vec<UserStatVO>, UserStatError>;

    async fn raw_query(
        &self,
        request: impl Into<String> + Send,
    ) -> Result<Vec<UserStatVO>, UserStatError>;
}

#[derive(Debug, Clone, Builder)]
pub struct UserStatServiceImpl {
    pub pool: sqlx::PgPool,
}

#[async_trait]
impl UserStatService for UserStatServiceImpl {
    async fn query(
        &self,
        request: impl Into<Query> + Send,
    ) -> Result<Vec<UserStatVO>, UserStatError> {
        let query: Query = request.into();
        let query_string = query.try_to_string().await?;
        info!("query string: {}", query_string);
        let ret = sqlx::query_as(&query_string).fetch_all(&self.pool).await?;
        Ok(ret)
    }

    async fn raw_query(
        &self,
        request: impl Into<String> + Send,
    ) -> Result<Vec<UserStatVO>, UserStatError> {
        let query: String = request.into();
        Ok(sqlx::query_as(&query).fetch_all(&self.pool).await?)
    }
}

impl TimeQuery {
    fn to_raw_query(&self, name: &str) -> String {
        match Some((self.lower, self.upper)) {
            Some((Some(lower), Some(upper))) => {
                format!("{} BETWEEN '{:?}' AND '{:?}'", name, lower, upper)
            }
            Some((Some(lower), None)) => format!("{} >= '{:?}'", name, lower),
            Some((None, Some(upper))) => format!("{} <= '{:?}'", name, upper),
            _ => "true".to_string(),
        }
    }
}

impl IdQuery {
    fn to_raw_query(&self, name: &str) -> String {
        if self.ids.is_empty() {
            return "true".to_string();
        }
        format!("array{:?} <@ {}", self.ids, name)
    }
}

impl Query {
    async fn try_to_string(&self) -> Result<String, UserStatError> {
        const SELECT_FORMAT: &str = r#"SELECT email, name FROM user_stats WHERE "#;
        let fields = UserStat::fields().await?;
        let mut query = String::from(SELECT_FORMAT);
        let time_condition = self
            .timestamps
            .iter()
            .filter(|(k, _)| fields.contains(*k))
            .map(|(k, v)| v.to_raw_query(k))
            .join(" AND ");
        if !time_condition.is_empty() {
            query.push_str(&time_condition);
        }

        let id_condition = self
            .ids
            .iter()
            .filter(|(k, _)| fields.contains(*k))
            .map(|(k, v)| v.to_raw_query(k))
            .join(" AND ");

        if time_condition.is_empty() && id_condition.is_empty() {
            return Err(UserStatError::FieldNotFound(
                "no field will be used for found".to_string(),
            ));
        }
        if !id_condition.is_empty() {
            if !time_condition.is_empty() {
                query.push_str(" AND ");
            }
            query.push_str(&id_condition);
        }

        Ok(query)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_query_nofields() {
        let query = Query {
            timestamps: HashMap::new(),
            ids: HashMap::new(),
        };
        query.try_to_string().await.unwrap_err();
    }

    #[tokio::test]
    async fn test_query() {
        let query = Query {
            timestamps: vec![(
                "created_at".to_string(),
                TimeQuery {
                    lower: Some(DateTime::from_timestamp_nanos(10)),
                    upper: Some(DateTime::from_timestamp_nanos(20)),
                },
            )]
            .into_iter()
            .collect(),
            ids: vec![("recent_watched".to_string(), IdQuery { ids: vec![1, 2, 3] })]
                .into_iter()
                .collect(),
        };
        let query = query.try_to_string().await.unwrap();
        assert_eq!(
            query,
            r#"SELECT email, name FROM user_stats WHERE created_at BETWEEN '1970-01-01T00:00:00.000000010Z' AND '1970-01-01T00:00:00.000000020Z' AND array[1, 2, 3] <@ recent_watched"#
        );
        println!("{}", query);
    }

    #[tokio::test]
    async fn test_query_filter() {
        let query = Query {
            timestamps: vec![
                (
                    "created_at".to_string(),
                    TimeQuery {
                        lower: Some(DateTime::from_timestamp_nanos(10)),
                        upper: Some(DateTime::from_timestamp_nanos(20)),
                    },
                ),
                (
                    "not_exists".to_string(),
                    TimeQuery {
                        lower: None,
                        upper: None,
                    },
                ),
            ]
            .into_iter()
            .collect(),
            ids: HashMap::new(),
        };
        let query = query.try_to_string().await.unwrap();
        assert_eq!(
            query,
            r#"SELECT email, name FROM user_stats WHERE created_at BETWEEN '1970-01-01T00:00:00.000000010Z' AND '1970-01-01T00:00:00.000000020Z'"#
        );
        println!("{}", query);
    }

    #[tokio::test]
    async fn test_query_filter_id() {
        let query = Query {
            timestamps: HashMap::new(),
            ids: vec![
                ("recent_watched".to_string(), IdQuery { ids: vec![1, 2, 3] }),
                ("not_exists".to_string(), IdQuery { ids: vec![] }),
            ]
            .into_iter()
            .collect(),
        };
        let query = query.try_to_string().await.unwrap();
        assert_eq!(
            query,
            r#"SELECT email, name FROM user_stats WHERE array[1, 2, 3] <@ recent_watched"#
        );
        println!("{}", query);
    }

    #[tokio::test]
    async fn test_true() {
        let query = Query {
            timestamps: HashMap::new(),
            ids: vec![
                ("recent_watched".to_string(), IdQuery { ids: vec![1, 2, 3] }),
                (
                    "started_but_not_finished".to_string(),
                    IdQuery { ids: vec![] },
                ),
                ("not_exists".to_string(), IdQuery { ids: vec![] }),
            ]
            .into_iter()
            .collect(),
        };
        let query = query.try_to_string().await.unwrap();
        assert!(
            query
                == r#"SELECT email, name FROM user_stats WHERE true AND array[1, 2, 3] <@ recent_watched"#
                || query
                    == r#"SELECT email, name FROM user_stats WHERE array[1, 2, 3] <@ recent_watched AND true"#
        );
        println!("{}", query);
    }
}
