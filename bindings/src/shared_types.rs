///`Erc20HoldData(address,address,address,uint256,uint256,bytes32,uint8)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Erc20HoldData {
    pub sender: ::ethers::core::types::Address,
    pub recipient: ::ethers::core::types::Address,
    pub notary: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub expiration_date_time: ::ethers::core::types::U256,
    pub secret_hash: [u8; 32],
    pub status: u8,
}
///`ValidateData(address,bytes,bytes32,address,address,address,uint256,bytes,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ValidateData {
    pub token: ::ethers::core::types::Address,
    pub payload: ::ethers::core::types::Bytes,
    pub partition: [u8; 32],
    pub operator: ::ethers::core::types::Address,
    pub from: ::ethers::core::types::Address,
    pub to: ::ethers::core::types::Address,
    pub value: ::ethers::core::types::U256,
    pub data: ::ethers::core::types::Bytes,
    pub operator_data: ::ethers::core::types::Bytes,
}
