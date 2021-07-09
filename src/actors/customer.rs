use iota_streams::{
    app::{
        message::HasLink,
        transport::tangle::PAYLOAD_BYTES,
    },
    app_channels::api::tangle::{
        Author,
        Subscriber,
        Transport,
    },
    core::{
        panic_if_not,
        prelude::Rc,
        print,
        println,
        try_or,
        Errors::*,
        Result,
        LOCATION_LOG,
    },
    ddml::types::*,
};

use iota_streams::app::transport::tangle::TangleAddress;
use iota_streams::app_channels::api::tangle;


pub type Address = TangleAddress;


// Customer represents the prostitute's customer who wants to verify
//
// the prostitute's certificate
// 
// Customer has a subscriber attribute to fetch messages
pub struct Customer<T>{
    subscriber: Subscriber<T>,
}
impl<Trans: Transport> Customer<Trans> {

    pub fn new(seed: &str, encoding: &str, payload_length: usize, transport: Trans) -> Self {
        let subscriber = Subscriber::new(seed, transport);
        
        Self{ subscriber:  subscriber}
    }

    pub fn get_subscriber(&mut self) -> &mut Subscriber<Trans> {
        &mut self.subscriber
    }
  }