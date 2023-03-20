pub use extension::*;
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
pub mod extension {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addCertificateSigner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"enum IExtensionTypes.CertificateValidation\",\"name\":\"certificateActivated\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"allowlistActivated\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"blocklistActivated\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"granularityByPartitionActivated\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"holdsActivated\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"operators\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerTokenSetup\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static EXTENSION_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct Extension<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Extension<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Extension<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Extension<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Extension<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Extension)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Extension<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EXTENSION_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addCertificateSigner` (0xc69664bb) function
        pub fn add_certificate_signer(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 150, 100, 187], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerTokenSetup` (0x118fa5db) function
        pub fn register_token_setup(
            &self,
            token: ::ethers::core::types::Address,
            certificate_activated: u8,
            allowlist_activated: bool,
            blocklist_activated: bool,
            granularity_by_partition_activated: bool,
            holds_activated: bool,
            operators: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [17, 143, 165, 219],
                    (
                        token,
                        certificate_activated,
                        allowlist_activated,
                        blocklist_activated,
                        granularity_by_partition_activated,
                        holds_activated,
                        operators,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Extension<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `addCertificateSigner` function with signature `addCertificateSigner(address,address)` and selector `0xc69664bb`
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
        name = "addCertificateSigner",
        abi = "addCertificateSigner(address,address)"
    )]
    pub struct AddCertificateSignerCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `registerTokenSetup` function with signature `registerTokenSetup(address,uint8,bool,bool,bool,bool,address[])` and selector `0x118fa5db`
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
        name = "registerTokenSetup",
        abi = "registerTokenSetup(address,uint8,bool,bool,bool,bool,address[])"
    )]
    pub struct RegisterTokenSetupCall {
        pub token: ::ethers::core::types::Address,
        pub certificate_activated: u8,
        pub allowlist_activated: bool,
        pub blocklist_activated: bool,
        pub granularity_by_partition_activated: bool,
        pub holds_activated: bool,
        pub operators: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExtensionCalls {
        AddCertificateSigner(AddCertificateSignerCall),
        RegisterTokenSetup(RegisterTokenSetupCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExtensionCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddCertificateSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddCertificateSigner(decoded));
            }
            if let Ok(decoded)
                = <RegisterTokenSetupCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RegisterTokenSetup(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExtensionCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddCertificateSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterTokenSetup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ExtensionCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddCertificateSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterTokenSetup(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddCertificateSignerCall> for ExtensionCalls {
        fn from(value: AddCertificateSignerCall) -> Self {
            Self::AddCertificateSigner(value)
        }
    }
    impl ::core::convert::From<RegisterTokenSetupCall> for ExtensionCalls {
        fn from(value: RegisterTokenSetupCall) -> Self {
            Self::RegisterTokenSetup(value)
        }
    }
}
