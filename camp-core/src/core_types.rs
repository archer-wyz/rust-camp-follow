use futures::Stream;
use std::pin::Pin;
use tonic::Status;

pub type PinBoxStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;

pub type PinBoxTonicStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;
