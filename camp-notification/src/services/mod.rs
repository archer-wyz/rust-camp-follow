use chrono::{DateTime, Utc};
use std::sync::Arc;
use thiserror::Error;
pub mod email;
pub mod inapp;
pub mod sms;

#[derive(Debug, Clone)]
pub struct SendResponse {
    pub id: String,
    pub timestamp: DateTime<Utc>,
}

pub trait ServicesFactory {
    fn email(&self) -> Arc<Box<dyn email::Email>>;
    fn inapp(&self) -> Arc<Box<dyn inapp::InApp>>;
    fn sms(&self) -> Arc<Box<dyn sms::Sms>>;
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Email error: {0}")]
    Email(#[from] email::EmailError),
    #[error("InApp error: {0}")]
    InApp(#[from] inapp::InAppError),
    #[error("Sms error: {0}")]
    Sms(#[from] sms::SmsError),
}

pub enum ServicesTypes {
    AllUnimock,
}

pub struct ServicesFactoryImpl {
    pub email: Arc<Box<dyn email::Email>>,
    pub inapp: Arc<Box<dyn inapp::InApp>>,
    pub sms: Arc<Box<dyn sms::Sms>>,
}

impl ServicesFactory for ServicesFactoryImpl {
    fn email(&self) -> Arc<Box<dyn email::Email>> {
        self.email.clone()
    }
    fn inapp(&self) -> Arc<Box<dyn inapp::InApp>> {
        self.inapp.clone()
    }
    fn sms(&self) -> Arc<Box<dyn sms::Sms>> {
        self.sms.clone()
    }
}

impl ServicesFactoryImpl {
    pub fn new(r#type: ServicesTypes, pool: sqlx::PgPool) -> Self {
        match r#type {
            ServicesTypes::AllUnimock => {
                let email = email::random_return_email(pool.clone());
                let inapp = inapp::random_return_inapp();
                let sms = sms::return_sms_mock();
                Self {
                    email: Arc::new(email),
                    inapp: Arc::new(Box::new(inapp)),
                    sms: Arc::new(Box::new(sms)),
                }
            }
        }
    }
}
