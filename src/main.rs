
use iota_streams::{
    app::{
        message::HasLink,
        transport::tangle::PAYLOAD_BYTES,
    },
    app_channels::{
        api::tangle
    },
    core::{
        prelude::Rc}
};

use iota_streams::{
    app::transport::{
        TransportOptions,
        tangle::client::{SendOptions, Client, },
    },
};




use core::cell::RefCell;

mod actors;
use actors::branch;


#[tokio::main]
async fn main() {
    let node_url = "https://api.lb-0.testnet.chrysalis2.com";

    // Creates Client and calls the publish_certificate method
    //
    // Fails at unwrap when the url isnt working
    
    let client = Client::new_from_url(&node_url);

    let mut transport = Rc::new(RefCell::new(client));
    let send_opt = SendOptions::default();
    transport.set_send_options(send_opt);

    branch::publish_certificate(transport).unwrap();
}