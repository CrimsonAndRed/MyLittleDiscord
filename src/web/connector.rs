use super::super::data::POOL;
use log::{debug, error};

use actix::*;
use actix_web::client;
use actix_web::client::{ClientResponse, SendRequestError};
use futures::Future;

struct RequestConnector {
    key_header: String,
}

impl Actor for RequestConnector {
    type Context = Context<Self>;
}

/// Sync
impl Handler<RequestMessage> for RequestConnector {
    type Result = Result<ClientResponse, SendRequestError>;

    fn handle(&mut self, msg: RequestMessage, ctx: &mut Context<Self>) -> Self::Result {
        client::get(msg.url)
            .header("authorization", self.key_header.to_string())
            .finish()
            .unwrap()
            .send()
            .wait()
    }
}

struct RequestMessage {
    method: HttpMethod,
    url: String,
    data: String, //??? TODO
}

impl Message for RequestMessage {
    type Result = Result<ClientResponse, SendRequestError>;
}

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE
}
