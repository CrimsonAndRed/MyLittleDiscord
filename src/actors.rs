use crate::connector::*;
use crate::data::POOL;
use actix::{Actor, Addr};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ACTORS: ActorsPool = ActorsPool::new();
}

/// Pool of actors addresses.
pub struct ActorsPool {
    pub request_connector: Addr<RequestConnector>,
}

impl ActorsPool {
    fn new() -> Self {
        // Pool does NOT depend on actors.
        // Keep this or inverse dependencies.
        let p = &POOL;
        let addr = RequestConnector {
            key_header: format!("Bot {}", &p.key),
        }
        .start();

        ActorsPool {
            request_connector: addr,
        }
    }
}
