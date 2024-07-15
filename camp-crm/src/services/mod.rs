pub mod metadata;
pub mod notification;
pub mod user_stat;

use std::fmt::{self, Display, Formatter};

pub use metadata::{MetaData, MetaDataImpl as MetaDataV1};
pub use notification::{Notification, NotificationImpl as NotificationV1};
use thiserror::Error;
pub use user_stat::{UserStat, UserStatImpl as UserStatV1};

#[derive(Debug, Error)]
pub enum ServiceError {
    MeataData(#[from] metadata::MetaDataError),
    UserStat(#[from] user_stat::UserError),
    Notification(#[from] notification::NotificationError),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::MeataData(e) => write!(f, "{}", e),
            ServiceError::UserStat(e) => write!(f, "{}", e),
            ServiceError::Notification(e) => write!(f, "{}", e),
        }
    }
}
