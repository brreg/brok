pub use allowlisted_role::*;
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
pub mod allowlisted_role {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AllowlistAdminAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AllowlistAdminRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AllowlistedAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AllowlistedRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAllowlistAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addAllowlisted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAllowlistAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isAllowlisted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeAllowlistAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeAllowlisted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceAllowlistAdmin\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ALLOWLISTEDROLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct AllowlistedRole<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AllowlistedRole<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AllowlistedRole<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AllowlistedRole<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AllowlistedRole<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(AllowlistedRole)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AllowlistedRole<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ALLOWLISTEDROLE_ABI.clone(),
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
        ///Calls the contract's `addAllowlisted` (0x4261699f) function
        pub fn add_allowlisted(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 97, 105, 159], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAllowlistAdmin` (0x42be5787) function
        pub fn is_allowlist_admin(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([66, 190, 87, 135], (token, account))
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `removeAllowlistAdmin` (0xdfe15da0) function
        pub fn remove_allowlist_admin(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 225, 93, 160], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAllowlisted` (0x685e2519) function
        pub fn remove_allowlisted(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 94, 37, 25], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceAllowlistAdmin` (0x0d21cdd8) function
        pub fn renounce_allowlist_admin(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 33, 205, 216], token)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AllowlistAdminAdded` event
        pub fn allowlist_admin_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowlistAdminAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowlistAdminRemoved` event
        pub fn allowlist_admin_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowlistAdminRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowlistedAdded` event
        pub fn allowlisted_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowlistedAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AllowlistedRemoved` event
        pub fn allowlisted_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowlistedRemovedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllowlistedRoleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AllowlistedRole<M> {
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
        name = "AllowlistAdminAdded",
        abi = "AllowlistAdminAdded(address,address)"
    )]
    pub struct AllowlistAdminAddedFilter {
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
        name = "AllowlistAdminRemoved",
        abi = "AllowlistAdminRemoved(address,address)"
    )]
    pub struct AllowlistAdminRemovedFilter {
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
    #[ethevent(name = "AllowlistedAdded", abi = "AllowlistedAdded(address,address)")]
    pub struct AllowlistedAddedFilter {
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
    #[ethevent(name = "AllowlistedRemoved", abi = "AllowlistedRemoved(address,address)")]
    pub struct AllowlistedRemovedFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AllowlistedRoleEvents {
        AllowlistAdminAddedFilter(AllowlistAdminAddedFilter),
        AllowlistAdminRemovedFilter(AllowlistAdminRemovedFilter),
        AllowlistedAddedFilter(AllowlistedAddedFilter),
        AllowlistedRemovedFilter(AllowlistedRemovedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AllowlistedRoleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllowlistAdminAddedFilter::decode_log(log) {
                return Ok(AllowlistedRoleEvents::AllowlistAdminAddedFilter(decoded));
            }
            if let Ok(decoded) = AllowlistAdminRemovedFilter::decode_log(log) {
                return Ok(AllowlistedRoleEvents::AllowlistAdminRemovedFilter(decoded));
            }
            if let Ok(decoded) = AllowlistedAddedFilter::decode_log(log) {
                return Ok(AllowlistedRoleEvents::AllowlistedAddedFilter(decoded));
            }
            if let Ok(decoded) = AllowlistedRemovedFilter::decode_log(log) {
                return Ok(AllowlistedRoleEvents::AllowlistedRemovedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AllowlistedRoleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllowlistAdminAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowlistAdminRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowlistedAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowlistedRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AllowlistAdminAddedFilter> for AllowlistedRoleEvents {
        fn from(value: AllowlistAdminAddedFilter) -> Self {
            Self::AllowlistAdminAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowlistAdminRemovedFilter> for AllowlistedRoleEvents {
        fn from(value: AllowlistAdminRemovedFilter) -> Self {
            Self::AllowlistAdminRemovedFilter(value)
        }
    }
    impl ::core::convert::From<AllowlistedAddedFilter> for AllowlistedRoleEvents {
        fn from(value: AllowlistedAddedFilter) -> Self {
            Self::AllowlistedAddedFilter(value)
        }
    }
    impl ::core::convert::From<AllowlistedRemovedFilter> for AllowlistedRoleEvents {
        fn from(value: AllowlistedRemovedFilter) -> Self {
            Self::AllowlistedRemovedFilter(value)
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
    ///Container type for all input parameters for the `addAllowlisted` function with signature `addAllowlisted(address,address)` and selector `0x4261699f`
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
    #[ethcall(name = "addAllowlisted", abi = "addAllowlisted(address,address)")]
    pub struct AddAllowlistedCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isAllowlistAdmin` function with signature `isAllowlistAdmin(address,address)` and selector `0x42be5787`
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
    #[ethcall(name = "isAllowlistAdmin", abi = "isAllowlistAdmin(address,address)")]
    pub struct IsAllowlistAdminCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `removeAllowlistAdmin` function with signature `removeAllowlistAdmin(address,address)` and selector `0xdfe15da0`
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
        name = "removeAllowlistAdmin",
        abi = "removeAllowlistAdmin(address,address)"
    )]
    pub struct RemoveAllowlistAdminCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeAllowlisted` function with signature `removeAllowlisted(address,address)` and selector `0x685e2519`
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
    #[ethcall(name = "removeAllowlisted", abi = "removeAllowlisted(address,address)")]
    pub struct RemoveAllowlistedCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceAllowlistAdmin` function with signature `renounceAllowlistAdmin(address)` and selector `0x0d21cdd8`
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
    #[ethcall(name = "renounceAllowlistAdmin", abi = "renounceAllowlistAdmin(address)")]
    pub struct RenounceAllowlistAdminCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AllowlistedRoleCalls {
        AddAllowlistAdmin(AddAllowlistAdminCall),
        AddAllowlisted(AddAllowlistedCall),
        IsAllowlistAdmin(IsAllowlistAdminCall),
        IsAllowlisted(IsAllowlistedCall),
        RemoveAllowlistAdmin(RemoveAllowlistAdminCall),
        RemoveAllowlisted(RemoveAllowlistedCall),
        RenounceAllowlistAdmin(RenounceAllowlistAdminCall),
    }
    impl ::ethers::core::abi::AbiDecode for AllowlistedRoleCalls {
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
                = <AddAllowlistedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddAllowlisted(decoded));
            }
            if let Ok(decoded)
                = <IsAllowlistAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsAllowlistAdmin(decoded));
            }
            if let Ok(decoded)
                = <IsAllowlistedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsAllowlisted(decoded));
            }
            if let Ok(decoded)
                = <RemoveAllowlistAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveAllowlistAdmin(decoded));
            }
            if let Ok(decoded)
                = <RemoveAllowlistedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveAllowlisted(decoded));
            }
            if let Ok(decoded)
                = <RenounceAllowlistAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceAllowlistAdmin(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AllowlistedRoleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAllowlistAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAllowlisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAllowlistAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAllowlisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAllowlistAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAllowlisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceAllowlistAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AllowlistedRoleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAllowlistAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAllowlisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAllowlistAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAllowlisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAllowlistAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveAllowlisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceAllowlistAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddAllowlistAdminCall> for AllowlistedRoleCalls {
        fn from(value: AddAllowlistAdminCall) -> Self {
            Self::AddAllowlistAdmin(value)
        }
    }
    impl ::core::convert::From<AddAllowlistedCall> for AllowlistedRoleCalls {
        fn from(value: AddAllowlistedCall) -> Self {
            Self::AddAllowlisted(value)
        }
    }
    impl ::core::convert::From<IsAllowlistAdminCall> for AllowlistedRoleCalls {
        fn from(value: IsAllowlistAdminCall) -> Self {
            Self::IsAllowlistAdmin(value)
        }
    }
    impl ::core::convert::From<IsAllowlistedCall> for AllowlistedRoleCalls {
        fn from(value: IsAllowlistedCall) -> Self {
            Self::IsAllowlisted(value)
        }
    }
    impl ::core::convert::From<RemoveAllowlistAdminCall> for AllowlistedRoleCalls {
        fn from(value: RemoveAllowlistAdminCall) -> Self {
            Self::RemoveAllowlistAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveAllowlistedCall> for AllowlistedRoleCalls {
        fn from(value: RemoveAllowlistedCall) -> Self {
            Self::RemoveAllowlisted(value)
        }
    }
    impl ::core::convert::From<RenounceAllowlistAdminCall> for AllowlistedRoleCalls {
        fn from(value: RenounceAllowlistAdminCall) -> Self {
            Self::RenounceAllowlistAdmin(value)
        }
    }
    ///Container type for all return fields from the `isAllowlistAdmin` function with signature `isAllowlistAdmin(address,address)` and selector `0x42be5787`
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
    pub struct IsAllowlistAdminReturn(pub bool);
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
}
