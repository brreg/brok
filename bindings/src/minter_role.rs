pub use minter_role::*;
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
pub mod minter_role {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MinterAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MinterRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addMinter\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isMinter\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeMinter\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceMinter\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static MINTERROLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct MinterRole<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MinterRole<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MinterRole<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MinterRole<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MinterRole<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(MinterRole)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MinterRole<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MINTERROLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addMinter` (0x983b2d56) function
        pub fn add_minter(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 59, 45, 86], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isMinter` (0xaa271e1a) function
        pub fn is_minter(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([170, 39, 30, 26], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeMinter` (0x3092afd5) function
        pub fn remove_minter(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 146, 175, 213], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceMinter` (0x98650275) function
        pub fn renounce_minter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 101, 2, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `MinterAdded` event
        pub fn minter_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinterAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinterRemoved` event
        pub fn minter_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinterRemovedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinterRoleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MinterRole<M> {
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
    #[ethevent(name = "MinterAdded", abi = "MinterAdded(address)")]
    pub struct MinterAddedFilter {
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
    #[ethevent(name = "MinterRemoved", abi = "MinterRemoved(address)")]
    pub struct MinterRemovedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MinterRoleEvents {
        MinterAddedFilter(MinterAddedFilter),
        MinterRemovedFilter(MinterRemovedFilter),
    }
    impl ::ethers::contract::EthLogDecode for MinterRoleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MinterAddedFilter::decode_log(log) {
                return Ok(MinterRoleEvents::MinterAddedFilter(decoded));
            }
            if let Ok(decoded) = MinterRemovedFilter::decode_log(log) {
                return Ok(MinterRoleEvents::MinterRemovedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MinterRoleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MinterAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinterRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MinterAddedFilter> for MinterRoleEvents {
        fn from(value: MinterAddedFilter) -> Self {
            Self::MinterAddedFilter(value)
        }
    }
    impl ::core::convert::From<MinterRemovedFilter> for MinterRoleEvents {
        fn from(value: MinterRemovedFilter) -> Self {
            Self::MinterRemovedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addMinter` function with signature `addMinter(address)` and selector `0x983b2d56`
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
    #[ethcall(name = "addMinter", abi = "addMinter(address)")]
    pub struct AddMinterCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isMinter` function with signature `isMinter(address)` and selector `0xaa271e1a`
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
    #[ethcall(name = "isMinter", abi = "isMinter(address)")]
    pub struct IsMinterCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeMinter` function with signature `removeMinter(address)` and selector `0x3092afd5`
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
    #[ethcall(name = "removeMinter", abi = "removeMinter(address)")]
    pub struct RemoveMinterCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceMinter` function with signature `renounceMinter()` and selector `0x98650275`
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
    #[ethcall(name = "renounceMinter", abi = "renounceMinter()")]
    pub struct RenounceMinterCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MinterRoleCalls {
        AddMinter(AddMinterCall),
        IsMinter(IsMinterCall),
        RemoveMinter(RemoveMinterCall),
        RenounceMinter(RenounceMinterCall),
    }
    impl ::ethers::core::abi::AbiDecode for MinterRoleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddMinterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddMinter(decoded));
            }
            if let Ok(decoded)
                = <IsMinterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsMinter(decoded));
            }
            if let Ok(decoded)
                = <RemoveMinterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveMinter(decoded));
            }
            if let Ok(decoded)
                = <RenounceMinterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceMinter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MinterRoleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddMinter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsMinter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveMinter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceMinter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MinterRoleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddMinter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsMinter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveMinter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceMinter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddMinterCall> for MinterRoleCalls {
        fn from(value: AddMinterCall) -> Self {
            Self::AddMinter(value)
        }
    }
    impl ::core::convert::From<IsMinterCall> for MinterRoleCalls {
        fn from(value: IsMinterCall) -> Self {
            Self::IsMinter(value)
        }
    }
    impl ::core::convert::From<RemoveMinterCall> for MinterRoleCalls {
        fn from(value: RemoveMinterCall) -> Self {
            Self::RemoveMinter(value)
        }
    }
    impl ::core::convert::From<RenounceMinterCall> for MinterRoleCalls {
        fn from(value: RenounceMinterCall) -> Self {
            Self::RenounceMinter(value)
        }
    }
    ///Container type for all return fields from the `isMinter` function with signature `isMinter(address)` and selector `0xaa271e1a`
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
    pub struct IsMinterReturn(pub bool);
}
