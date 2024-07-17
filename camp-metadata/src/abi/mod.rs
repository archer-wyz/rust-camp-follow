use std::pin::Pin;

use camp_core::core_fake::{before, Int, TimeStampBetween, VecFaker};
use fake::{
    faker::{lorem::zh_cn::Sentence, name::zh_cn::Name},
    Dummy, Fake, Faker, Rng,
};
use futures::Stream;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use tracing::info;

use crate::pb::metadata::{
    metadata_server::Metadata, Content, ContentType, MaterializeRequest, MaterializeResponse,
    Publisher,
};

impl Dummy<Faker> for Publisher {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self {
            id: 0,
            name: Name().fake_with_rng(rng),
            avatar: "https://fakeimg.pl/400x300/?text=i'm a publisher".to_string(),
        }
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
            publishers: VecFaker(10).fake_with_rng(rng),
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
        info!("materialize");
        tokio::spawn(async move {
            while let Some(req) = stream.next().await {
                let req = req.unwrap();
                info!("req: {:?}", req);
                let resp = MaterializeResponse {
                    id: req.id,
                    content: Some(Faker.fake()),
                };
                info!("metadata sending resp {:?}", resp.id);
                tx.send(Ok(resp)).await.unwrap();
            }
        });
        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream)))
    }
}

pub struct Tpl<'a>(pub &'a [Content]);

impl<'a> Tpl<'a> {
    pub fn to_body(&self) -> String {
        format!("Tpl: {:?}", self.0)
    }
}
