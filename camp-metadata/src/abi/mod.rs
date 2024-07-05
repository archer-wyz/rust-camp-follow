use std::pin::Pin;

use camp_core::core_fake::{before, Int, TimeStampBetween};
use fake::{
    faker::{lorem::zh_cn::Sentence, name::zh_cn::Name},
    Dummy, Fake, Faker, Rng,
};
use futures::Stream;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

use crate::pb::metadata::{
    metadata_server::Metadata, Content, ContentType, MaterializeRequest, MaterializeResponse,
    Publisher,
};

impl Publisher {
    pub fn fake() -> Self {
        todo!()
    }
}

impl Dummy<Faker> for ContentType {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let rand: u32 = Int(0, 40).fake_with_rng(rng);
        let rand2 = rand % 5;
        match rand2 {
            0 => Self::Vlog,
            1 => Self::Srhot,
            2 => Self::Moive,
            3 => Self::AiGenerated,
            _ => Self::Unspecified,
        }
    }
}

impl Dummy<Faker> for Content {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self {
            id: 0,
            name: Name().fake_with_rng(rng),
            description: Sentence(10..100).fake(),
            publishers: vec![Publisher::fake()],
            r#type: Faker.fake(),
            created_at: TimeStampBetween(before(90), before(10)).fake_with_rng(rng),
            updated_at: TimeStampBetween(before(10), before(5)).fake_with_rng(rng),
            views: Int(0, 1000).fake_with_rng(rng),
            likes: Int(0, 1000).fake_with_rng(rng),
            dislikes: Int(0, 1000).fake_with_rng(rng),
            url: "https://fakeimg.pl/400x300/?text=Hello".to_string(),
            image: "https://fakeimg.pl/400x300/?text=Hello".to_string(),
        }
    }
}

pub struct MetadataGRPC;

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<MaterializeResponse, Status>> + Send>>;

#[tonic::async_trait]
impl Metadata for MetadataGRPC {
    type MaterializeStream = ResponseStream;

    async fn materialize(
        &self,
        request: Request<Streaming<MaterializeRequest>>,
    ) -> ServiceResult<Self::MaterializeStream> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            loop {
                let sended = match stream.next().await {
                    Some(Ok(request)) => {
                        let resp = MaterializeResponse {
                            id: request.id,
                            content: Some(Faker.fake()),
                        };
                        tx.send(Ok(resp)).await
                    }
                    Some(Err(e)) => tx.send(Err(Status::invalid_argument(e.to_string()))).await,
                    None => {
                        tx.send(Err(Status::invalid_argument("No request provided")))
                            .await
                    }
                };
                match sended {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::MaterializeStream
        ))
    }
}
