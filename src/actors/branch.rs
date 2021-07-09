use core::cell::RefCell;

use super::registration_office::RegistrationOffice;
use super::doctor::Doctor;
use super::prostitute::Prostitute;
use super::customer::Customer;

use rand::Rng;

use iota_streams::{app::{message::HasLink, transport::tangle::{PAYLOAD_BYTES, TangleAddress, TangleMessage}}, app_channels::{api::tangle::{
            Author,
            Subscriber,
        }, message::announce}, core::{
        prelude::Rc,
        print,
        println,
        try_or,
        LOCATION_LOG,
        Errors::*,
    }, ddml::types::*};

use iota_streams::{
    app::transport::{
        TransportOptions,
        tangle::client::{SendOptions, Client, },
    },
    app_channels::api::tangle::Transport,
    core::{
        prelude::{ String},
        Result,
    },
};


// This method creates a branch for a prostitute, lets the registration office and
// 
// the doctor publish messages and the prostitute's customer read it (without subscribing).
//
// To clarify the use case in this short proof of concept, the actors are represented by 
//
// their own classes, having the relevant role as an attribute[author, subscriber]

pub fn publish_certificate(transport: Rc<RefCell<Client>>) -> Result<()>{

    //TODO: Error handling
    //TODO: Clear the unused imports
    //TODO: if author, subscriber already exists

    // The prostitute first visits the registration office, which then creates a channel and branch
    //
    // and puts the hash of the prostitute's data on it.
    //
    // Therefore we first have to create an instance of the registration office
    // 
    // and an instance of the prostitute, representing his/her phone as a data storage.

    // Instance of the Registration Office is created
    let encoding = "utf-8";
    let multi_branching = true;
    let name1 = "Oldenburg";
    let alph9 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
    let seed1: &str = &(0..10)
        .map(|_| alph9.chars().nth(rand::thread_rng().gen_range(0, 27)).unwrap())
        .collect::<String>();
    let mut registration_office = RegistrationOffice::new(seed1, encoding, PAYLOAD_BYTES, multi_branching, transport.clone(), name1);
 
    
    println!("Prostitute visiting registration office");
    println!();
    // Registration Office publishes a channel

    let announce_link = registration_office.get_author().send_announce().unwrap();

    println!("{} published a public channel!", registration_office.name);
    println!("Channel Address: {}, MsgTag: {}", announce_link, announce_link.msgid);
    println!();

    // Instance of the prostitute is created
    //
    // As dummy parameters only the channel link will be used for construction
    //
    // of the prostitute's instance.
    //
    // The last three links will be exchanged during the following process

    let mut prostitute_save = Prostitute::new(announce_link.clone(), announce_link.clone(), announce_link.clone(), announce_link.clone(), String::from("One"));

    // Registration office creates a new branch for the prostitute via keyload

    println!("Keyload:");
    let keyload_link = {
    let (msg, seq)= registration_office.get_author().send_keyload_for_everyone(&announce_link)?;
    let seq = seq.unwrap();
    println!("  msg => <{}> {:?}", msg.msgid, msg);
    println!("  seq => <{}> {:?}", seq.msgid, seq);
    println!();
    seq
    };

    // The registration office takes the public payload from the prostitute's phone
    //
    // This will later be the hash of the json object, consisting of the prostitute's data
    //
    // The masked payload is empty, because we need public access

    let public_payload = Bytes(prostitute_save.hash.as_bytes().to_vec());
    let empty_masked_payload = Bytes("".as_bytes().to_vec());

    // The registration office posts the signed message containing the hash on the channel

    println!("Sending Signed Packet!");
    let signed_packet_link = {
        let (msg, seq) = registration_office.get_author().send_signed_packet(&keyload_link, &public_payload, &empty_masked_payload)?;
        let seq = seq.unwrap();
        println!(" msg => <{}> {:?}", msg.msgid, msg);
        println!(" msg => <{}> {:?}", seq.msgid, seq);
        println!();
        println!("--------------------------------------");
        seq
    };

    // Prostitute saves keyload_link and signed packet link on her phone

    prostitute_save.keyload_link = keyload_link.clone();
    prostitute_save.signed_message_link = signed_packet_link.clone();

    // The prostitute now leaves the registration office and has the channel address,
    //
    // keyload_link and signed_packet_link saved on her phone

    // The prostitute visits the doctor to get her health certificate
    //
    // Here the doctor instance is created
    //
    // This will later be a different session
    println!("Prostitute visiting doctor");
    println!();

    let name2 = "DoctorOne";
    let alph9 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
    let seed2: &str = &(0..10)
        .map(|_| alph9.chars().nth(rand::thread_rng().gen_range(0, 27)).unwrap())
        .collect::<String>();
    let mut doctor_one = Doctor::new(seed2, encoding, PAYLOAD_BYTES, transport.clone(), name2);



    // Doctor gets channel address via the prostitutes phone

    println!("Doctor Receiving Channel Address!");
    println!();
    doctor_one.get_subscriber().receive_announcement(&prostitute_save.channel_address)?;
    try_or!(registration_office.get_author().channel_address() == doctor_one.get_subscriber().channel_address(), ApplicationInstanceMismatch(String::from("One")))?;
    

    // Doctor subscribes to the channel

    let subscribe_link = doctor_one.get_subscriber().send_subscribe(&prostitute_save.channel_address)?;
    println!("{}, subscribed to the channel", doctor_one.name);
    println!();


    // Doctor receives keyload from the prostitute's phone and therefore access to the specific branch

    println!("Doctor receive keyload!");
    let msg_tag = doctor_one.get_subscriber().receive_sequence(&prostitute_save.keyload_link)?;
    println!();

    // Doctor receives signed message to compare the hash

    println!("Doctor receiving Signed message!");
    let msg_tag = doctor_one.get_subscriber().receive_sequence(&prostitute_save.signed_message_link)?;

    let (_signer_pk, unwrapped_public, unwrapped_masked) = doctor_one.get_subscriber().receive_signed_packet(&msg_tag)?;
    let unwrapped_public = unwrapped_public.0;
    println!("Public Message: {}", String::from_utf8(unwrapped_public.clone())?);
   
    try_or!(String::from_utf8(unwrapped_public.clone())? == String::from(&prostitute_save.hash), PublicPayloadMismatch(String::from("Hashes not equal!"), String::from(&prostitute_save.hash)))?;

    println!("Hashes are equal!");
    println!();
    // The doctor gets the public payload from the prostitute's phone
    //
    // This will later be the hash of the prostitute's data
    let doc_public_payload = Bytes(prostitute_save.hash.as_bytes().to_vec());
    let doc_empty_masked_payload = Bytes("".as_bytes().to_vec()); 

    
    // Doctor sends the tagged packet containing the hash
    println!("Doctor sending Tagged Packet!");
    let tagged_packet_link = {
        let (msg, seq) = doctor_one.get_subscriber().send_tagged_packet(&prostitute_save.signed_message_link, &doc_public_payload, &doc_empty_masked_payload)?;
        let seq = seq.unwrap();
        println!("  msg => <{}> {:?}", msg.msgid, msg);
        println!("  seq => <{}> {:?}", seq.msgid, seq);
        println!("  SubscriberA: {}", doctor_one.name);
        println!();
        println!("--------------------------------------");
        seq
    };

    // Prostitute saves the tagged packet link

    prostitute_save.tagged_message_link = tagged_packet_link.clone();


    
    // The customer now wants to access the channel
    // 
    // Creating a customer instance, customer has subscriber attribute
    //
    // This will later be a different session
   
    let alph9 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
    let seed3: &str = &(0..10)
        .map(|_| alph9.chars().nth(rand::thread_rng().gen_range(0, 27)).unwrap())
        .collect::<String>();
    let mut customer = Customer::new(seed3, encoding, PAYLOAD_BYTES, transport.clone());

    // Customer takes the channel address from the prostitute and registers in the channel
    //
    // Note that he does not need to subscribe
    println!("Customer visits prostitute");
    println!();

    customer.get_subscriber().receive_announcement(&prostitute_save.channel_address)?;
    
    println!("Customer joined the Channel");

    // Customer now gets the keyload_link and therefore access to the prostitutes branch

    println!("Customer receiving keyload");
    println!();
    let msg_tag = customer.get_subscriber().receive_sequence(&prostitute_save.keyload_link)?;

    // customer now gets the signed_message_link(regsitration office) and receives the
    //
    // message sequence and content of the signed packet.
    //
    // This would be the hash of the prostitute's data published by the regsitration office
    //
    // TODO: After fetching the public payload will be compared to the hash generated by the prostitue's phone

    println!("Customer receiving Signed Message!");
    let msg_tag = customer.get_subscriber().receive_sequence(&prostitute_save.signed_message_link)?;
    let (_signer_pk, unwrapped_public, unwrapped_masked) = customer.get_subscriber().receive_signed_packet(&msg_tag)?;
    let unwrapped_public = unwrapped_public.0;
    println!("Public Message: {}", String::from_utf8(unwrapped_public.clone())?);

    try_or!(String::from_utf8(unwrapped_public.clone())? == String::from(&prostitute_save.hash), PublicPayloadMismatch(String::from("Hashes not equal!"), String::from(&prostitute_save.hash)))?;

    println!("Hashes are equal!");
    println!();
    // customer now gets the tagged_message_link(doctor) and receives sequence and content
    //
    // This would be the has of the prostitute's data published by the doctor
    //
    // Then the hashes get compared

    println!("Customer receiving Tagged Packet!");
    let msg_tag = customer.get_subscriber().receive_sequence(&prostitute_save.tagged_message_link)?;
    let (unwrapped_public, unwrapped_masked) = customer.get_subscriber().receive_tagged_packet(&msg_tag)?;
    let unwrapped_public = unwrapped_public.0;
    println!("Public Message: {}", String::from_utf8(unwrapped_public.clone())?);

    try_or!(String::from_utf8(unwrapped_public.clone())? == String::from(&prostitute_save.hash), PublicPayloadMismatch(String::from("Hashes not equal!"), String::from(&prostitute_save.hash)))?;
    println!("Hashes are equal!");
    println!();

    //////////////////////////////////////////////////////////////////////////////////////////////////
    //////// If everything works and matches the customer gets the verification //////////////////////
    println!("The Certificats are Valid!");
    Ok(())
}