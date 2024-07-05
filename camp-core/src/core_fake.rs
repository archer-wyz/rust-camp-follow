use chrono::{DateTime, Utc};
use fake::{
    faker::{chrono::zh_cn::DateTimeBetween, internet::en::SafeEmail},
    Dummy, Fake, Faker, Rng,
};
use nanoid::nanoid;
use rand::thread_rng;

pub fn before(days: usize) -> DateTime<Utc> {
    let now = Utc::now();
    now - chrono::Duration::days(days as i64)
}

pub struct UniqueEmail;
const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

impl Dummy<UniqueEmail> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_config: &UniqueEmail, rng: &mut R) -> Self {
        let email: String = SafeEmail().fake_with_rng(rng);
        let id = nanoid!(8, &ALPHABET);
        let at = email.find('@').unwrap();
        format!("{}{}{}", &email[..at], id, &email[at..])
    }
}

pub struct IntList(pub i32, pub i32, pub i32);

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

pub struct Int(pub usize, pub usize);

impl Dummy<Int> for u32 {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Int, rng: &mut R) -> Self {
        rng.gen_range(config.0..config.1) as u32
    }
}

pub struct TimeStampBetween(pub DateTime<Utc>, pub DateTime<Utc>);

impl Dummy<TimeStampBetween> for prost_types::Timestamp {
    fn dummy_with_rng<R: Rng + ?Sized>(t: &TimeStampBetween, rng: &mut R) -> Self {
        let (start, end) = (t.0, t.1);
        let time: DateTime<Utc> = DateTimeBetween(start, end).fake_with_rng(rng);
        Self {
            seconds: time.timestamp(),
            nanos: time.timestamp_subsec_nanos() as i32,
        }
    }
}

pub struct VecFaker(pub usize);

impl<T> Dummy<VecFaker> for Vec<T>
where
    T: Dummy<Faker>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &VecFaker, rng: &mut R) -> Self {
        let len = config.0;
        (0..len).map(|_| Faker.fake_with_rng(rng)).collect()
    }
}
