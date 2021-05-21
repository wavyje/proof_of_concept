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

pub type Address = TangleAddress;

// The registration office publishes a channel for the prostitute and
//
// posts hash of the prostitutes data as a verification of the prostitutes registration
//
// Is the author of the channel, therefore has the author as an attribute
pub struct RegistrationOffice<T> {
    author: Author<T>,
    pub name: String,
}

impl<Trans: Transport> RegistrationOffice<Trans> {

    pub fn new(seed: &str, encoding: &str, payload_length: usize, multi_branching: bool, transport: Trans, name: &str) -> Self{
        let author = Author::new(seed, encoding, payload_length, multi_branching, transport);
        
        Self {author: author,
              name: String::from(name),
        }
    }

    pub fn get_author(&mut self) -> &mut Author<Trans>{
        &mut self.author
    }
    
}