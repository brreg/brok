pub use ierc1400_tokens_validator_extended::*;
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
pub mod ierc1400_tokens_validator_extended {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAllowlisted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isBlocklisted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"retrieveTokenSetup\",\"outputs\":[{\"internalType\":\"enum IExtensionTypes.CertificateValidation\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"spendableBalanceOfByPartition\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC1400TOKENSVALIDATOREXTENDED_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IERC1400TokensValidatorExtended<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC1400TokensValidatorExtended<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC1400TokensValidatorExtended<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC1400TokensValidatorExtended<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC1400TokensValidatorExtended<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC1400TokensValidatorExtended))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC1400TokensValidatorExtended<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IERC1400TOKENSVALIDATOREXTENDED_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `isAllowlisted` (0x61c2e281) function
        pub fn is_allowlisted(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([97, 194, 226, 129], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isBlocklisted` (0xa4aa023f) function
        pub fn is_blocklisted(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 170, 2, 63], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `retrieveTokenSetup` (0xaccf6175) function
        pub fn retrieve_token_setup(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, bool, bool, bool, bool, ::std::vec::Vec<::ethers::core::types::Address>),
        > {
            self.0
                .method_hash([172, 207, 97, 117], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spendableBalanceOfByPartition` (0x732c916b) function
        pub fn spendable_balance_of_by_partition(
            &self,
            token: ::ethers::core::types::Address,
            partition: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([115, 44, 145, 107], (token, partition, account))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IERC1400TokensValidatorExtended<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `isAllowlisted` function with signature `isAllowlisted(address,address)` and selector `0x61c2e281`
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
    #[ethcall(name = "isAllowlisted", abi = "isAllowlisted(address,address)")]
    pub struct IsAllowlistedCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isBlocklisted` function with signature `isBlocklisted(address,address)` and selector `0xa4aa023f`
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
    #[ethcall(name = "isBlocklisted", abi = "isBlocklisted(address,address)")]
    pub struct IsBlocklistedCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `retrieveTokenSetup` function with signature `retrieveTokenSetup(address)` and selector `0xaccf6175`
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
    #[ethcall(name = "retrieveTokenSetup", abi = "retrieveTokenSetup(address)")]
    pub struct RetrieveTokenSetupCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `spendableBalanceOfByPartition` function with signature `spendableBalanceOfByPartition(address,bytes32,address)` and selector `0x732c916b`
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
        name = "spendableBalanceOfByPartition",
        abi = "spendableBalanceOfByPartition(address,bytes32,address)"
    )]
    pub struct SpendableBalanceOfByPartitionCall {
        pub token: ::ethers::core::types::Address,
        pub partition: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC1400TokensValidatorExtendedCalls {
        IsAllowlisted(IsAllowlistedCall),
        IsBlocklisted(IsBlocklistedCall),
        RetrieveTokenSetup(RetrieveTokenSetupCall),
        SpendableBalanceOfByPartition(SpendableBalanceOfByPartitionCall),
    }
    impl ::ethers::core::abi::AbiDecode for IERC1400TokensValidatorExtendedCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <IsAllowlistedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsAllowlisted(decoded));
            }
            if let Ok(decoded)
                = <IsBlocklistedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsBlocklisted(decoded));
            }
            if let Ok(decoded)
                = <RetrieveTokenSetupCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RetrieveTokenSetup(decoded));
            }
            if let Ok(decoded)
                = <SpendableBalanceOfByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SpendableBalanceOfByPartition(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC1400TokensValidatorExtendedCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsAllowlisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsBlocklisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RetrieveTokenSetup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpendableBalanceOfByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IERC1400TokensValidatorExtendedCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsAllowlisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsBlocklisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::RetrieveTokenSetup(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SpendableBalanceOfByPartition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IsAllowlistedCall>
    for IERC1400TokensValidatorExtendedCalls {
        fn from(value: IsAllowlistedCall) -> Self {
            Self::IsAllowlisted(value)
        }
    }
    impl ::core::convert::From<IsBlocklistedCall>
    for IERC1400TokensValidatorExtendedCalls {
        fn from(value: IsBlocklistedCall) -> Self {
            Self::IsBlocklisted(value)
        }
    }
    impl ::core::convert::From<RetrieveTokenSetupCall>
    for IERC1400TokensValidatorExtendedCalls {
        fn from(value: RetrieveTokenSetupCall) -> Self {
            Self::RetrieveTokenSetup(value)
        }
    }
    impl ::core::convert::From<SpendableBalanceOfByPartitionCall>
    for IERC1400TokensValidatorExtendedCalls {
        fn from(value: SpendableBalanceOfByPartitionCall) -> Self {
            Self::SpendableBalanceOfByPartition(value)
        }
    }
    ///Container type for all return fields from the `isAllowlisted` function with signature `isAllowlisted(address,address)` and selector `0x61c2e281`
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
    pub struct IsAllowlistedReturn(pub bool);
    ///Container type for all return fields from the `isBlocklisted` function with signature `isBlocklisted(address,address)` and selector `0xa4aa023f`
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
    pub struct IsBlocklistedReturn(pub bool);
    ///Container type for all return fields from the `retrieveTokenSetup` function with signature `retrieveTokenSetup(address)` and selector `0xaccf6175`
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
    pub struct RetrieveTokenSetupReturn(
        pub u8,
        pub bool,
        pub bool,
        pub bool,
        pub bool,
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `spendableBalanceOfByPartition` function with signature `spendableBalanceOfByPartition(address,bytes32,address)` and selector `0x732c916b`
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
    pub struct SpendableBalanceOfByPartitionReturn(pub ::ethers::core::types::U256);
}
