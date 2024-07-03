use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{Executor, FromRow};

#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct UserStat {
    pub email: String,
    pub name: String,
    pub create_at: DateTime<Utc>,
    pub last_visited_at: Option<DateTime<Utc>>,
    pub last_watch_at: Option<DateTime<Utc>>,
    pub recent_watched: Vec<i32>,
    pub started_but_not_finished: Vec<i32>,
    pub finished: Vec<i32>,
    pub last_email_notification: Option<DateTime<Utc>>,
    pub last_in_app_notification: Option<DateTime<Utc>>,
    pub last_sms_notification: Option<DateTime<Utc>>,
}

impl UserStat {
    pub async fn insert<'c, E>(&self, executor: E) -> Result<()>
    where
        E: Executor<'c, Database = sqlx::Postgres>,
    {
        sqlx::query(
            r#"
            INSERT INTO user_stats (email, name, create_at, last_visited_at, last_watch_at, recent_watched, started_but_not_finished, finished, last_email_notification, last_in_app_notification, last_sms_notification)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#)
            .bind(&self.email)
            .bind(&self.name)
            .bind(self.create_at)
            .bind(self.last_visited_at)
            .bind(self.last_watch_at)
            .bind(&self.recent_watched)
            .bind(&self.started_but_not_finished)
            .bind(&self.finished)
            .bind(self.last_email_notification)
            .bind(self.last_in_app_notification)
            .bind(self.last_sms_notification)
            .execute(executor).await?;
        Ok(())
    }
}

#[cfg(feature = "local_utils")]
mod local_utils {
    use super::*;
    use fake::{
        faker::{chrono::en::DateTimeBetween, internet::en::SafeEmail, name::zh_cn::Name},
        Dummy, Fake, Faker, Rng,
    };
    use nanoid::nanoid;
    use rand::thread_rng;
    use std::{
        collections::HashSet,
        hash::{Hash, Hasher},
    };
    impl Hash for UserStat {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.email.hash(state);
        }
    }

    impl Dummy<Faker> for UserStat {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
            UserStat {
                email: UniqueEmail.fake_with_rng(rng),
                name: Name().fake_with_rng(rng),
                create_at: DateTimeBetween(before(365 * 5), before(90)).fake_with_rng(rng),
                last_visited_at: Some(DateTimeBetween(before(20), before(1)).fake_with_rng(rng)),
                last_watch_at: Some(DateTimeBetween(before(90), before(20)).fake_with_rng(rng)),
                recent_watched: IntList(50, 100000, 1000000).fake_with_rng(rng),
                started_but_not_finished: IntList(50, 100000, 1000000).fake_with_rng(rng),
                finished: IntList(50, 100000, 1000000).fake_with_rng(rng),
                last_email_notification: Some(
                    DateTimeBetween(before(90), before(20)).fake_with_rng(rng),
                ),
                last_in_app_notification: Some(
                    DateTimeBetween(before(90), before(20)).fake_with_rng(rng),
                ),
                last_sms_notification: Some(
                    DateTimeBetween(before(90), before(20)).fake_with_rng(rng),
                ),
            }
        }
    }

    impl UserStat {
        pub fn gen(size: usize) -> HashSet<UserStat> {
            (0..size).map(|_| Faker.fake::<UserStat>()).collect()
        }
        pub async fn gen_and_insert(size: usize, pool: sqlx::PgPool) -> Result<()> {
            let stats = UserStat::gen(size);
            let mut transaction = pool.begin().await?;
            for stat in stats {
                let now = Utc::now();
                stat.insert(&mut *transaction).await?;
                let elapsed = Utc::now() - now;
                println!("elapsed: {:?}ms", elapsed.num_milliseconds());
            }
            Ok(())
        }
    }

    fn before(days: usize) -> DateTime<Utc> {
        let now = Utc::now();
        now - chrono::Duration::days(days as i64)
    }

    struct UniqueEmail;
    const ALPHABET: [char; 36] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    impl Dummy<UniqueEmail> for String {
        fn dummy_with_rng<R: Rng + ?Sized>(_config: &UniqueEmail, rng: &mut R) -> Self {
            let email: String = SafeEmail().fake_with_rng(rng);
            let id = nanoid!(8, &ALPHABET);
            let at = email.find('@').unwrap();
            format!("{}{}{}", &email[..at], id, &email[at..])
        }
    }

    struct IntList(pub i32, pub i32, pub i32);

    impl Dummy<IntList> for Vec<i32> {
        fn dummy_with_rng<R: Rng + ?Sized>(config: &IntList, _: &mut R) -> Self {
            let (max, start, len) = (config.0, config.1, config.2);
            let mut rng = thread_rng();
            let max = rng.gen_range(0..max);
            (0..max)
                .map(|_| rng.gen_range(start..start + len))
                .collect()
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use fake::Faker;
        #[test]
        fn test_dummy() {
            let stat = Faker.fake::<UserStat>();
            println!("{:?}", stat);
        }

        #[tokio::test]
        async fn test_gen() {
            let _tdb = sqlx_db_tester::TestPg::new(
                "postgres://postgres:postgres@localhost:5432".to_string(),
                std::path::Path::new("./migrations"),
            );
            let pool = _tdb.get_pool().await;
            UserStat::gen_and_insert(100, pool).await.unwrap();
        }
    }
}
