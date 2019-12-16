use crate::core::msg::{Request, Response};
use crate::core::ServiceType;
use bytes::Bytes;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub struct GetRequest {
    id: String,
    sender: Option<Sender<GetResponse>>,
}

impl GetRequest {
    pub fn new<S: Into<String>>(sender: Sender<GetResponse>, id: S) -> Self {
        Self {
            sender: Some(sender),
            id: id.into(),
        }
    }
}

impl Request for GetRequest {
    type Item = GetResponse;

    fn encode(&self) -> Bytes {
        unimplemented!()
    }

    fn decode(&self, input: Bytes) -> Self::Item {
        GetResponse::new(1234, input)
    }

    fn succeed(&mut self, response: Self::Item) {
        let sender = self.sender.take().unwrap();
        sender.send(response).expect("Could not send! - fix me.");
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Kv
    }
}

#[derive(Debug)]
pub struct GetResponse {
    cas: u64,
    content: Bytes,
}

impl GetResponse {
    pub fn new(cas: u64, content: Bytes) -> Self {
        Self { cas, content }
    }

    pub fn cas(&self) -> u64 {
        self.cas
    }

    pub fn content(&self) -> &Bytes {
        &self.content
    }
}

impl Response for GetResponse {}