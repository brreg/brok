pub use ierc1400_tokens_sender::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod ierc1400_tokens_sender {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"canTransfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tokensToTransfer\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC1400TOKENSSENDER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IERC1400TokensSender<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC1400TokensSender<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC1400TokensSender<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC1400TokensSender<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC1400TokensSender<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC1400TokensSender))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC1400TokensSender<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IERC1400TOKENSSENDER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `canTransfer` (0xe8a24c6a) function
        pub fn can_transfer(
            &self,
            payload: ::ethers::core::types::Bytes,
            partition: [u8; 32],
            operator: ::ethers::core::types::Address,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operator_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [232, 162, 76, 106],
                    (payload, partition, operator, from, to, value, data, operator_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokensToTransfer` (0x4e755a54) function
        pub fn tokens_to_transfer(
            &self,
            payload: ::ethers::core::types::Bytes,
            partition: [u8; 32],
            operator: ::ethers::core::types::Address,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operator_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [78, 117, 90, 84],
                    (payload, partition, operator, from, to, value, data, operator_data),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IERC1400TokensSender<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `canTransfer` function with signature `canTransfer(bytes,bytes32,address,address,address,uint256,bytes,bytes)` and selector `0xe8a24c6a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "canTransfer",
        abi = "canTransfer(bytes,bytes32,address,address,address,uint256,bytes,bytes)"
    )]
    pub struct CanTransferCall {
        pub payload: ::ethers::core::types::Bytes,
        pub partition: [u8; 32],
        pub operator: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `tokensToTransfer` function with signature `tokensToTransfer(bytes,bytes32,address,address,address,uint256,bytes,bytes)` and selector `0x4e755a54`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "tokensToTransfer",
        abi = "tokensToTransfer(bytes,bytes32,address,address,address,uint256,bytes,bytes)"
    )]
    pub struct TokensToTransferCall {
        pub payload: ::ethers::core::types::Bytes,
        pub partition: [u8; 32],
        pub operator: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC1400TokensSenderCalls {
        CanTransfer(CanTransferCall),
        TokensToTransfer(TokensToTransferCall),
    }
    impl ::ethers::core::abi::AbiDecode for IERC1400TokensSenderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CanTransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CanTransfer(decoded));
            }
            if let Ok(decoded)
                = <TokensToTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TokensToTransfer(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC1400TokensSenderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CanTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokensToTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IERC1400TokensSenderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CanTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokensToTransfer(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CanTransferCall> for IERC1400TokensSenderCalls {
        fn from(value: CanTransferCall) -> Self {
            Self::CanTransfer(value)
        }
    }
    impl ::core::convert::From<TokensToTransferCall> for IERC1400TokensSenderCalls {
        fn from(value: TokensToTransferCall) -> Self {
            Self::TokensToTransfer(value)
        }
    }
    ///Container type for all return fields from the `canTransfer` function with signature `canTransfer(bytes,bytes32,address,address,address,uint256,bytes,bytes)` and selector `0xe8a24c6a`
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
    pub struct CanTransferReturn(pub bool);
}
