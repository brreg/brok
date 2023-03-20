pub use i_holdable_erc1400_token_extension::*;
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
pub mod i_holdable_erc1400_token_extension {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"lockPreimage\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeHold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"retrieveHoldData\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"notary\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expiration\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"secretHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"secret\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"enum HoldStatusCode\",\"name\":\"status\",\"type\":\"uint8\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IHOLDABLEERC1400TOKENEXTENSION_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IHoldableERC1400TokenExtension<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IHoldableERC1400TokenExtension<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IHoldableERC1400TokenExtension<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IHoldableERC1400TokenExtension<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IHoldableERC1400TokenExtension<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IHoldableERC1400TokenExtension))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IHoldableERC1400TokenExtension<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IHOLDABLEERC1400TOKENEXTENSION_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `executeHold` (0x54c6f08c) function
        pub fn execute_hold(
            &self,
            token: ::ethers::core::types::Address,
            hold_id: [u8; 32],
            value: ::ethers::core::types::U256,
            lock_preimage: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 198, 240, 140], (token, hold_id, value, lock_preimage))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `retrieveHoldData` (0x12da4de6) function
        pub fn retrieve_hold_data(
            &self,
            token: ::ethers::core::types::Address,
            hold_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 32],
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                [u8; 32],
                [u8; 32],
                u8,
            ),
        > {
            self.0
                .method_hash([18, 218, 77, 230], (token, hold_id))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IHoldableERC1400TokenExtension<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `executeHold` function with signature `executeHold(address,bytes32,uint256,bytes32)` and selector `0x54c6f08c`
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
        name = "executeHold",
        abi = "executeHold(address,bytes32,uint256,bytes32)"
    )]
    pub struct ExecuteHoldCall {
        pub token: ::ethers::core::types::Address,
        pub hold_id: [u8; 32],
        pub value: ::ethers::core::types::U256,
        pub lock_preimage: [u8; 32],
    }
    ///Container type for all input parameters for the `retrieveHoldData` function with signature `retrieveHoldData(address,bytes32)` and selector `0x12da4de6`
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
    #[ethcall(name = "retrieveHoldData", abi = "retrieveHoldData(address,bytes32)")]
    pub struct RetrieveHoldDataCall {
        pub token: ::ethers::core::types::Address,
        pub hold_id: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IHoldableERC1400TokenExtensionCalls {
        ExecuteHold(ExecuteHoldCall),
        RetrieveHoldData(RetrieveHoldDataCall),
    }
    impl ::ethers::core::abi::AbiDecode for IHoldableERC1400TokenExtensionCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ExecuteHoldCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteHold(decoded));
            }
            if let Ok(decoded)
                = <RetrieveHoldDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RetrieveHoldData(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IHoldableERC1400TokenExtensionCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ExecuteHold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RetrieveHoldData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IHoldableERC1400TokenExtensionCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExecuteHold(element) => ::core::fmt::Display::fmt(element, f),
                Self::RetrieveHoldData(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteHoldCall> for IHoldableERC1400TokenExtensionCalls {
        fn from(value: ExecuteHoldCall) -> Self {
            Self::ExecuteHold(value)
        }
    }
    impl ::core::convert::From<RetrieveHoldDataCall>
    for IHoldableERC1400TokenExtensionCalls {
        fn from(value: RetrieveHoldDataCall) -> Self {
            Self::RetrieveHoldData(value)
        }
    }
    ///Container type for all return fields from the `retrieveHoldData` function with signature `retrieveHoldData(address,bytes32)` and selector `0x12da4de6`
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
    pub struct RetrieveHoldDataReturn {
        pub partition: [u8; 32],
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub notary: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub expiration: ::ethers::core::types::U256,
        pub secret_hash: [u8; 32],
        pub secret: [u8; 32],
        pub status: u8,
    }
}
