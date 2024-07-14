use crate::model::message::{Message, MessageType};
use camp_core::core_fake::{before, vec_range_faker, PrefixUUID, UniqueEmail};
use chrono::{DateTime, Utc};
use fake::{
    faker::{barcode::zh_cn::Isbn10, chrono::zh_cn::DateTimeBetween, name::en::Name},
    Dummy, Fake, Faker, Rng,
};

#[derive(Debug, Dummy)]
pub struct EmailMessageFaker {
    #[dummy(faker = r#"PrefixUUID("email")"#)]
    pub id: String,
    #[dummy(faker = "Name()")]
    pub subject: String,
    #[dummy(faker = "UniqueEmail")]
    pub sender: String,
    #[dummy(faker = "vec_range_faker(1, 10, UniqueEmail)")]
    pub recipients: Vec<String>,
    #[dummy(faker = "Isbn10()")]
    pub body: String,
}

#[derive(Debug, Dummy)]
pub struct SmsMessageFaker {
    #[dummy(faker = r#"PrefixUUID("sms")"#)]
    pub id: String,
    #[dummy(faker = "Name()")]
    pub sender: String,
    #[dummy(faker = "vec_range_faker(1, 10, UniqueEmail)")]
    pub recipients: Vec<String>,
    #[dummy(faker = "Isbn10()")]
    pub body: String,
}

#[derive(Debug, Dummy)]
pub struct InappMessageFaker {
    #[dummy(faker = r#"PrefixUUID("inapp")"#)]
    pub id: String,
    #[dummy(faker = "Name()")]
    pub sender: String,
    #[dummy(faker = "vec_range_faker(1, 10, UniqueEmail)")]
    pub recipients: Vec<String>,
    #[dummy(faker = "Isbn10()")]
    pub body: String,
}

impl From<InappMessageFaker> for MessageFaker {
    fn from(value: InappMessageFaker) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: value.id,
            r#type: MessageType::Inapp,
            sender: value.sender,
            body: value.body,
            subject: None,
            recipients: Some(value.recipients),
            times: 1,
            device_id: Some(PrefixUUID("device").fake_with_rng(&mut rng)),
            title: Some(PrefixUUID("title").fake_with_rng(&mut rng)),
            created_at: DateTimeBetween(before(90), before(30)).fake_with_rng(&mut rng),
            updated_at: DateTimeBetween(before(29), before(1)).fake_with_rng(&mut rng),
        }
    }
}

impl From<SmsMessageFaker> for MessageFaker {
    fn from(value: SmsMessageFaker) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: value.id,
            r#type: MessageType::Sms,
            sender: value.sender,
            body: value.body,
            subject: None,
            recipients: Some(value.recipients),
            times: 1,
            device_id: None,
            title: None,
            created_at: DateTimeBetween(before(90), before(30)).fake_with_rng(&mut rng),
            updated_at: DateTimeBetween(before(29), before(1)).fake_with_rng(&mut rng),
        }
    }
}

impl From<EmailMessageFaker> for MessageFaker {
    fn from(value: EmailMessageFaker) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: value.id,
            r#type: MessageType::Email,
            sender: value.sender,
            body: value.body,
            subject: Some(value.subject),
            recipients: Some(value.recipients),
            times: 1,
            device_id: None,
            title: None,
            created_at: DateTimeBetween(before(90), before(30)).fake_with_rng(&mut rng),
            updated_at: DateTimeBetween(before(29), before(1)).fake_with_rng(&mut rng),
        }
    }
}

pub struct MessageFaker {
    pub id: String,
    pub r#type: MessageType,
    pub sender: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub subject: Option<String>,
    pub recipients: Option<Vec<String>>,
    pub device_id: Option<String>,
    pub title: Option<String>,
    pub times: i32,
}

impl Dummy<Faker> for MessageType {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        match rng.gen_range(0..4) {
            0 => Self::Email,
            1 => Self::Sms,
            2 => Self::Inapp,
            _ => Self::Unknown,
        }
    }
}

impl MessageFaker {
    pub fn fake() -> Self {
        let rng = &mut rand::thread_rng();
        let r#type: MessageType = Faker.fake_with_rng(rng);
        match r#type {
            MessageType::Email => {
                let email: EmailMessageFaker = Faker.fake_with_rng(rng);
                email.into()
            }
            MessageType::Sms => {
                let sms: SmsMessageFaker = Faker.fake_with_rng(rng);
                sms.into()
            }
            _ => {
                let inapp: InappMessageFaker = Faker.fake_with_rng(rng);
                inapp.into()
            }
        }
    }
}

impl From<MessageFaker> for Message {
    fn from(value: MessageFaker) -> Self {
        Self {
            id: value.id,
            r#type: value.r#type,
            sender: value.sender,
            body: value.body,
            subject: value.subject,
            recipients: value.recipients,
            times: value.times,
            device_id: value.device_id,
            title: value.title,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fake() {
        for _ in 0..10 {
            let message: Message = MessageFaker::fake().into();
            println!("{:?}", message);
        }
    }
}
