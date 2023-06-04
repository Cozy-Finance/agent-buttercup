///`TriggerMetadata(string,string,string,string)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TriggerMetadata {
    pub name: ::std::string::String,
    pub category: ::std::string::String,
    pub description: ::std::string::String,
    pub logo_uri: ::std::string::String,
}
