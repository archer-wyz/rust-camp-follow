use crate::model::{Gender, UserStat};
use anyhow::Result;
use camp_core::core_fake::{before, Int, IntList, UniqueEmail};
use chrono::Utc;
use fake::{
    faker::{chrono::zh_cn::DateTimeBetween, name::zh_cn::Name},
    Dummy, Fake, Faker,
};
use std::{
    cmp::min,
    hash::{Hash, Hasher},
};
use tracing::info;
pub struct UserStatFaker;

impl Dummy<UserStatFaker> for UserStat {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UserStatFaker, rng: &mut R) -> Self {
        UserStat {
            email: UniqueEmail.fake_with_rng(rng),
            name: Name().fake_with_rng(rng),
            gender: Faker.fake_with_rng(rng),
            viewed_but_not_started: IntList(1000, 100000, 200000).fake_with_rng(rng),
            recent_watched: IntList(1000, 100000, 200000).fake_with_rng(rng),
            started_but_not_finished: IntList(1000, 100000, 200000).fake_with_rng(rng),
            finished: IntList(1000, 100000, 200000).fake_with_rng(rng),
            created_at: DateTimeBetween(before(90), before(60)).fake_with_rng(rng),
            last_visited_at: DateTimeBetween(before(60), before(30)).fake_with_rng(rng),
            last_watched_at: DateTimeBetween(before(45), before(20)).fake_with_rng(rng),
            last_email_notification: DateTimeBetween(before(90), before(60)).fake_with_rng(rng),
            last_in_app_notification: DateTimeBetween(before(90), before(60)).fake_with_rng(rng),
            last_sms_notification: DateTimeBetween(before(90), before(60)).fake_with_rng(rng),
        }
    }
}

pub struct UserStatFakerList(pub usize);

impl Dummy<Faker> for Gender {
    fn dummy_with_rng<R: rand::prelude::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        match Int(0, 2).fake_with_rng(rng) {
            0 => Gender::Female,
            1 => Gender::Male,
            _ => Gender::Unknown,
        }
    }
}

impl Dummy<UserStatFakerList> for Vec<UserStat> {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UserStatFakerList, rng: &mut R) -> Self {
        let mut hash = std::collections::HashSet::new();
        for _ in 0..config.0 {
            hash.insert(UserStatFaker.fake_with_rng(rng));
        }
        hash.into_iter().collect()
    }
}

impl UserStatFakerList {
    pub async fn fake_and_insert(&self, pool: sqlx::PgPool) -> Result<()> {
        let user_stats: Vec<UserStat> = self.fake();
        const EACH_COMMIT: usize = 40;
        let mut i = 0;
        while i < user_stats.len() {
            let end = min(i + EACH_COMMIT, user_stats.len());
            let now = Utc::now();
            let mut transaction = pool.begin().await?;
            for user in &user_stats[i..end] {
                user.pg_insert(&mut *transaction).await?;
            }
            let elapsed = Utc::now() - now;
            info!(
                "committing {} records use {}",
                min(EACH_COMMIT, user_stats.len() - i),
                elapsed
            );
            transaction.commit().await?;
            i = end;
        }
        Ok(())
    }
}

impl Hash for UserStat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_user_stat_faker() {
        let user: UserStat = UserStatFaker.fake();
        println!("{:?}", user);
    }
}
