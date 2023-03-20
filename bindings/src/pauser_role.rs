pub use pauser_role::*;
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
pub mod pauser_role {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PauserAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PauserRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPauser\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPauser\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removePauser\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renouncePauser\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static PAUSERROLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct PauserRole<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PauserRole<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PauserRole<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PauserRole<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PauserRole<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(PauserRole)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PauserRole<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PAUSERROLE_ABI.clone(),
                    client,
                ),
            )
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
        ///Calls the contract's `isPauser` (0x2fd33398) function
        pub fn is_pauser(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 211, 51, 152], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePauser` (0xed2f9095) function
        pub fn remove_pauser(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 47, 144, 149], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renouncePauser` (0x41eb24bb) function
        pub fn renounce_pauser(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 235, 36, 187], token)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PauserAdded` event
        pub fn pauser_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauserAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PauserRemoved` event
        pub fn pauser_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauserRemovedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauserRoleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PauserRole<M> {
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
    #[ethevent(name = "PauserAdded", abi = "PauserAdded(address,address)")]
    pub struct PauserAddedFilter {
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
    #[ethevent(name = "PauserRemoved", abi = "PauserRemoved(address,address)")]
    pub struct PauserRemovedFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PauserRoleEvents {
        PauserAddedFilter(PauserAddedFilter),
        PauserRemovedFilter(PauserRemovedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PauserRoleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PauserAddedFilter::decode_log(log) {
                return Ok(PauserRoleEvents::PauserAddedFilter(decoded));
            }
            if let Ok(decoded) = PauserRemovedFilter::decode_log(log) {
                return Ok(PauserRoleEvents::PauserRemovedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PauserRoleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PauserAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<PauserAddedFilter> for PauserRoleEvents {
        fn from(value: PauserAddedFilter) -> Self {
            Self::PauserAddedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRemovedFilter> for PauserRoleEvents {
        fn from(value: PauserRemovedFilter) -> Self {
            Self::PauserRemovedFilter(value)
        }
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
    ///Container type for all input parameters for the `isPauser` function with signature `isPauser(address,address)` and selector `0x2fd33398`
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
    #[ethcall(name = "isPauser", abi = "isPauser(address,address)")]
    pub struct IsPauserCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removePauser` function with signature `removePauser(address,address)` and selector `0xed2f9095`
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
    #[ethcall(name = "removePauser", abi = "removePauser(address,address)")]
    pub struct RemovePauserCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renouncePauser` function with signature `renouncePauser(address)` and selector `0x41eb24bb`
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
    #[ethcall(name = "renouncePauser", abi = "renouncePauser(address)")]
    pub struct RenouncePauserCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PauserRoleCalls {
        AddPauser(AddPauserCall),
        IsPauser(IsPauserCall),
        RemovePauser(RemovePauserCall),
        RenouncePauser(RenouncePauserCall),
    }
    impl ::ethers::core::abi::AbiDecode for PauserRoleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddPauserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddPauser(decoded));
            }
            if let Ok(decoded)
                = <IsPauserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsPauser(decoded));
            }
            if let Ok(decoded)
                = <RemovePauserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemovePauser(decoded));
            }
            if let Ok(decoded)
                = <RenouncePauserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenouncePauser(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PauserRoleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddPauser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPauser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovePauser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenouncePauser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PauserRoleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddPauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovePauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenouncePauser(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddPauserCall> for PauserRoleCalls {
        fn from(value: AddPauserCall) -> Self {
            Self::AddPauser(value)
        }
    }
    impl ::core::convert::From<IsPauserCall> for PauserRoleCalls {
        fn from(value: IsPauserCall) -> Self {
            Self::IsPauser(value)
        }
    }
    impl ::core::convert::From<RemovePauserCall> for PauserRoleCalls {
        fn from(value: RemovePauserCall) -> Self {
            Self::RemovePauser(value)
        }
    }
    impl ::core::convert::From<RenouncePauserCall> for PauserRoleCalls {
        fn from(value: RenouncePauserCall) -> Self {
            Self::RenouncePauser(value)
        }
    }
    ///Container type for all return fields from the `isPauser` function with signature `isPauser(address,address)` and selector `0x2fd33398`
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
    pub struct IsPauserReturn(pub bool);
}
