pub use extension_mock::*;
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
pub mod extension_mock {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAllowlistAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addBlocklistAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addCertificateSigner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPauser\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static EXTENSIONMOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct ExtensionMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ExtensionMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ExtensionMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ExtensionMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ExtensionMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ExtensionMock)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ExtensionMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    EXTENSIONMOCK_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addAllowlistAdmin` (0xfa061ca0) function
        pub fn add_allowlist_admin(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 6, 28, 160], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addBlocklistAdmin` (0xc428b202) function
        pub fn add_blocklist_admin(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 40, 178, 2], (token, account))
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `addPauser` (0xeb5314bd) function
        pub fn add_pauser(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 83, 20, 189], (token, account))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ExtensionMock<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `addAllowlistAdmin` function with signature `addAllowlistAdmin(address,address)` and selector `0xfa061ca0`
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
    #[ethcall(name = "addAllowlistAdmin", abi = "addAllowlistAdmin(address,address)")]
    pub struct AddAllowlistAdminCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addBlocklistAdmin` function with signature `addBlocklistAdmin(address,address)` and selector `0xc428b202`
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
    #[ethcall(name = "addBlocklistAdmin", abi = "addBlocklistAdmin(address,address)")]
    pub struct AddBlocklistAdminCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `addPauser` function with signature `addPauser(address,address)` and selector `0xeb5314bd`
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
    #[ethcall(name = "addPauser", abi = "addPauser(address,address)")]
    pub struct AddPauserCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ExtensionMockCalls {
        AddAllowlistAdmin(AddAllowlistAdminCall),
        AddBlocklistAdmin(AddBlocklistAdminCall),
        AddCertificateSigner(AddCertificateSignerCall),
        AddPauser(AddPauserCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExtensionMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddAllowlistAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddAllowlistAdmin(decoded));
            }
            if let Ok(decoded)
                = <AddBlocklistAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddBlocklistAdmin(decoded));
            }
            if let Ok(decoded)
                = <AddCertificateSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddCertificateSigner(decoded));
            }
            if let Ok(decoded)
                = <AddPauserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddPauser(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExtensionMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAllowlistAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddBlocklistAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddCertificateSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddPauser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ExtensionMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAllowlistAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddBlocklistAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddCertificateSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddPauser(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddAllowlistAdminCall> for ExtensionMockCalls {
        fn from(value: AddAllowlistAdminCall) -> Self {
            Self::AddAllowlistAdmin(value)
        }
    }
    impl ::core::convert::From<AddBlocklistAdminCall> for ExtensionMockCalls {
        fn from(value: AddBlocklistAdminCall) -> Self {
            Self::AddBlocklistAdmin(value)
        }
    }
    impl ::core::convert::From<AddCertificateSignerCall> for ExtensionMockCalls {
        fn from(value: AddCertificateSignerCall) -> Self {
            Self::AddCertificateSigner(value)
        }
    }
    impl ::core::convert::From<AddPauserCall> for ExtensionMockCalls {
        fn from(value: AddPauserCall) -> Self {
            Self::AddPauser(value)
        }
    }
}
