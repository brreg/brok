pub use certificate_signer_role::*;
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
pub mod certificate_signer_role {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CertificateSignerAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CertificateSignerRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addCertificateSigner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCertificateSigner\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeCertificateSigner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceCertificateSigner\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static CERTIFICATESIGNERROLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct CertificateSignerRole<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CertificateSignerRole<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CertificateSignerRole<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CertificateSignerRole<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CertificateSignerRole<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(CertificateSignerRole))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CertificateSignerRole<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CERTIFICATESIGNERROLE_ABI.clone(),
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
        ///Calls the contract's `isCertificateSigner` (0x9e59ceba) function
        pub fn is_certificate_signer(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([158, 89, 206, 186], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeCertificateSigner` (0xe81ea541) function
        pub fn remove_certificate_signer(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 30, 165, 65], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceCertificateSigner` (0x3bddaf8f) function
        pub fn renounce_certificate_signer(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 221, 175, 143], token)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CertificateSignerAdded` event
        pub fn certificate_signer_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CertificateSignerAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CertificateSignerRemoved` event
        pub fn certificate_signer_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CertificateSignerRemovedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CertificateSignerRoleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CertificateSignerRole<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CertificateSignerAdded",
        abi = "CertificateSignerAdded(address,address)"
    )]
    pub struct CertificateSignerAddedFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CertificateSignerRemoved",
        abi = "CertificateSignerRemoved(address,address)"
    )]
    pub struct CertificateSignerRemovedFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CertificateSignerRoleEvents {
        CertificateSignerAddedFilter(CertificateSignerAddedFilter),
        CertificateSignerRemovedFilter(CertificateSignerRemovedFilter),
    }
    impl ::ethers::contract::EthLogDecode for CertificateSignerRoleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CertificateSignerAddedFilter::decode_log(log) {
                return Ok(
                    CertificateSignerRoleEvents::CertificateSignerAddedFilter(decoded),
                );
            }
            if let Ok(decoded) = CertificateSignerRemovedFilter::decode_log(log) {
                return Ok(
                    CertificateSignerRoleEvents::CertificateSignerRemovedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CertificateSignerRoleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CertificateSignerAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CertificateSignerRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CertificateSignerAddedFilter>
    for CertificateSignerRoleEvents {
        fn from(value: CertificateSignerAddedFilter) -> Self {
            Self::CertificateSignerAddedFilter(value)
        }
    }
    impl ::core::convert::From<CertificateSignerRemovedFilter>
    for CertificateSignerRoleEvents {
        fn from(value: CertificateSignerRemovedFilter) -> Self {
            Self::CertificateSignerRemovedFilter(value)
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
    ///Container type for all input parameters for the `isCertificateSigner` function with signature `isCertificateSigner(address,address)` and selector `0x9e59ceba`
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
        name = "isCertificateSigner",
        abi = "isCertificateSigner(address,address)"
    )]
    pub struct IsCertificateSignerCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeCertificateSigner` function with signature `removeCertificateSigner(address,address)` and selector `0xe81ea541`
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
        name = "removeCertificateSigner",
        abi = "removeCertificateSigner(address,address)"
    )]
    pub struct RemoveCertificateSignerCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceCertificateSigner` function with signature `renounceCertificateSigner(address)` and selector `0x3bddaf8f`
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
        name = "renounceCertificateSigner",
        abi = "renounceCertificateSigner(address)"
    )]
    pub struct RenounceCertificateSignerCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CertificateSignerRoleCalls {
        AddCertificateSigner(AddCertificateSignerCall),
        IsCertificateSigner(IsCertificateSignerCall),
        RemoveCertificateSigner(RemoveCertificateSignerCall),
        RenounceCertificateSigner(RenounceCertificateSignerCall),
    }
    impl ::ethers::core::abi::AbiDecode for CertificateSignerRoleCalls {
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
                = <IsCertificateSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsCertificateSigner(decoded));
            }
            if let Ok(decoded)
                = <RemoveCertificateSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveCertificateSigner(decoded));
            }
            if let Ok(decoded)
                = <RenounceCertificateSignerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceCertificateSigner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CertificateSignerRoleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddCertificateSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsCertificateSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveCertificateSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceCertificateSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CertificateSignerRoleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddCertificateSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsCertificateSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveCertificateSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceCertificateSigner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddCertificateSignerCall> for CertificateSignerRoleCalls {
        fn from(value: AddCertificateSignerCall) -> Self {
            Self::AddCertificateSigner(value)
        }
    }
    impl ::core::convert::From<IsCertificateSignerCall> for CertificateSignerRoleCalls {
        fn from(value: IsCertificateSignerCall) -> Self {
            Self::IsCertificateSigner(value)
        }
    }
    impl ::core::convert::From<RemoveCertificateSignerCall>
    for CertificateSignerRoleCalls {
        fn from(value: RemoveCertificateSignerCall) -> Self {
            Self::RemoveCertificateSigner(value)
        }
    }
    impl ::core::convert::From<RenounceCertificateSignerCall>
    for CertificateSignerRoleCalls {
        fn from(value: RenounceCertificateSignerCall) -> Self {
            Self::RenounceCertificateSigner(value)
        }
    }
    ///Container type for all return fields from the `isCertificateSigner` function with signature `isCertificateSigner(address,address)` and selector `0x9e59ceba`
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
    pub struct IsCertificateSignerReturn(pub bool);
}
