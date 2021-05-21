
use iota_streams::app::transport::tangle::TangleAddress as Address;

// This class serves as a temporary storage and will later be replaced by json on the prostitutes phone
pub struct Prostitute {
    pub hash: String,
    pub channel_address: Address,
    pub keyload_link: Address,
    pub signed_message_link: Address,
    pub tagged_message_link: Address
}
 
impl Prostitute {
    pub fn new(channel_address: Address, keyload_link: Address, signed_message_link: Address, tagged_message_link:Address) -> Self{
            Self {  hash: String::from("Hash of the Json object[prostitute's data]"),
                    channel_address: channel_address,
                    keyload_link: keyload_link,
                    signed_message_link: signed_message_link,
                    tagged_message_link: tagged_message_link
            }
    }
}