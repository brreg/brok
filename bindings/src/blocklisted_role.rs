pub use blocklisted_role::*;
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
pub mod blocklisted_role {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BlocklistAdminAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BlocklistAdminRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BlocklistedAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BlocklistedRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addBlocklistAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addBlocklisted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isBlocklistAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isBlocklisted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeBlocklistAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeBlocklisted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceBlocklistAdmin\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static BLOCKLISTEDROLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct BlocklistedRole<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BlocklistedRole<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BlocklistedRole<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BlocklistedRole<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BlocklistedRole<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(BlocklistedRole)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BlocklistedRole<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BLOCKLISTEDROLE_ABI.clone(),
                    client,
                ),
            )
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
        ///Calls the contract's `addBlocklisted` (0x2c55b05f) function
        pub fn add_blocklisted(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 85, 176, 95], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isBlocklistAdmin` (0xa28e7c02) function
        pub fn is_blocklist_admin(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([162, 142, 124, 2], (token, account))
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
        ///Calls the contract's `removeBlocklistAdmin` (0xebf42f77) function
        pub fn remove_blocklist_admin(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 244, 47, 119], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeBlocklisted` (0x9bd483ec) function
        pub fn remove_blocklisted(
            &self,
            token: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 212, 131, 236], (token, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceBlocklistAdmin` (0xe323e671) function
        pub fn renounce_blocklist_admin(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 35, 230, 113], token)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `BlocklistAdminAdded` event
        pub fn blocklist_admin_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlocklistAdminAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BlocklistAdminRemoved` event
        pub fn blocklist_admin_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlocklistAdminRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BlocklistedAdded` event
        pub fn blocklisted_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlocklistedAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BlocklistedRemoved` event
        pub fn blocklisted_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlocklistedRemovedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlocklistedRoleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BlocklistedRole<M> {
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
        name = "BlocklistAdminAdded",
        abi = "BlocklistAdminAdded(address,address)"
    )]
    pub struct BlocklistAdminAddedFilter {
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
        name = "BlocklistAdminRemoved",
        abi = "BlocklistAdminRemoved(address,address)"
    )]
    pub struct BlocklistAdminRemovedFilter {
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
    #[ethevent(name = "BlocklistedAdded", abi = "BlocklistedAdded(address,address)")]
    pub struct BlocklistedAddedFilter {
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
    #[ethevent(name = "BlocklistedRemoved", abi = "BlocklistedRemoved(address,address)")]
    pub struct BlocklistedRemovedFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BlocklistedRoleEvents {
        BlocklistAdminAddedFilter(BlocklistAdminAddedFilter),
        BlocklistAdminRemovedFilter(BlocklistAdminRemovedFilter),
        BlocklistedAddedFilter(BlocklistedAddedFilter),
        BlocklistedRemovedFilter(BlocklistedRemovedFilter),
    }
    impl ::ethers::contract::EthLogDecode for BlocklistedRoleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BlocklistAdminAddedFilter::decode_log(log) {
                return Ok(BlocklistedRoleEvents::BlocklistAdminAddedFilter(decoded));
            }
            if let Ok(decoded) = BlocklistAdminRemovedFilter::decode_log(log) {
                return Ok(BlocklistedRoleEvents::BlocklistAdminRemovedFilter(decoded));
            }
            if let Ok(decoded) = BlocklistedAddedFilter::decode_log(log) {
                return Ok(BlocklistedRoleEvents::BlocklistedAddedFilter(decoded));
            }
            if let Ok(decoded) = BlocklistedRemovedFilter::decode_log(log) {
                return Ok(BlocklistedRoleEvents::BlocklistedRemovedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BlocklistedRoleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BlocklistAdminAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BlocklistAdminRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BlocklistedAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BlocklistedRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BlocklistAdminAddedFilter> for BlocklistedRoleEvents {
        fn from(value: BlocklistAdminAddedFilter) -> Self {
            Self::BlocklistAdminAddedFilter(value)
        }
    }
    impl ::core::convert::From<BlocklistAdminRemovedFilter> for BlocklistedRoleEvents {
        fn from(value: BlocklistAdminRemovedFilter) -> Self {
            Self::BlocklistAdminRemovedFilter(value)
        }
    }
    impl ::core::convert::From<BlocklistedAddedFilter> for BlocklistedRoleEvents {
        fn from(value: BlocklistedAddedFilter) -> Self {
            Self::BlocklistedAddedFilter(value)
        }
    }
    impl ::core::convert::From<BlocklistedRemovedFilter> for BlocklistedRoleEvents {
        fn from(value: BlocklistedRemovedFilter) -> Self {
            Self::BlocklistedRemovedFilter(value)
        }
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
    ///Container type for all input parameters for the `addBlocklisted` function with signature `addBlocklisted(address,address)` and selector `0x2c55b05f`
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
    #[ethcall(name = "addBlocklisted", abi = "addBlocklisted(address,address)")]
    pub struct AddBlocklistedCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isBlocklistAdmin` function with signature `isBlocklistAdmin(address,address)` and selector `0xa28e7c02`
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
    #[ethcall(name = "isBlocklistAdmin", abi = "isBlocklistAdmin(address,address)")]
    pub struct IsBlocklistAdminCall {
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
    ///Container type for all input parameters for the `removeBlocklistAdmin` function with signature `removeBlocklistAdmin(address,address)` and selector `0xebf42f77`
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
        name = "removeBlocklistAdmin",
        abi = "removeBlocklistAdmin(address,address)"
    )]
    pub struct RemoveBlocklistAdminCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeBlocklisted` function with signature `removeBlocklisted(address,address)` and selector `0x9bd483ec`
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
    #[ethcall(name = "removeBlocklisted", abi = "removeBlocklisted(address,address)")]
    pub struct RemoveBlocklistedCall {
        pub token: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceBlocklistAdmin` function with signature `renounceBlocklistAdmin(address)` and selector `0xe323e671`
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
    #[ethcall(name = "renounceBlocklistAdmin", abi = "renounceBlocklistAdmin(address)")]
    pub struct RenounceBlocklistAdminCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BlocklistedRoleCalls {
        AddBlocklistAdmin(AddBlocklistAdminCall),
        AddBlocklisted(AddBlocklistedCall),
        IsBlocklistAdmin(IsBlocklistAdminCall),
        IsBlocklisted(IsBlocklistedCall),
        RemoveBlocklistAdmin(RemoveBlocklistAdminCall),
        RemoveBlocklisted(RemoveBlocklistedCall),
        RenounceBlocklistAdmin(RenounceBlocklistAdminCall),
    }
    impl ::ethers::core::abi::AbiDecode for BlocklistedRoleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddBlocklistAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddBlocklistAdmin(decoded));
            }
            if let Ok(decoded)
                = <AddBlocklistedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddBlocklisted(decoded));
            }
            if let Ok(decoded)
                = <IsBlocklistAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsBlocklistAdmin(decoded));
            }
            if let Ok(decoded)
                = <IsBlocklistedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsBlocklisted(decoded));
            }
            if let Ok(decoded)
                = <RemoveBlocklistAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveBlocklistAdmin(decoded));
            }
            if let Ok(decoded)
                = <RemoveBlocklistedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveBlocklisted(decoded));
            }
            if let Ok(decoded)
                = <RenounceBlocklistAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceBlocklistAdmin(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BlocklistedRoleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddBlocklistAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddBlocklisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsBlocklistAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsBlocklisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBlocklistAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBlocklisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceBlocklistAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BlocklistedRoleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddBlocklistAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddBlocklisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsBlocklistAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsBlocklisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveBlocklistAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveBlocklisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceBlocklistAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddBlocklistAdminCall> for BlocklistedRoleCalls {
        fn from(value: AddBlocklistAdminCall) -> Self {
            Self::AddBlocklistAdmin(value)
        }
    }
    impl ::core::convert::From<AddBlocklistedCall> for BlocklistedRoleCalls {
        fn from(value: AddBlocklistedCall) -> Self {
            Self::AddBlocklisted(value)
        }
    }
    impl ::core::convert::From<IsBlocklistAdminCall> for BlocklistedRoleCalls {
        fn from(value: IsBlocklistAdminCall) -> Self {
            Self::IsBlocklistAdmin(value)
        }
    }
    impl ::core::convert::From<IsBlocklistedCall> for BlocklistedRoleCalls {
        fn from(value: IsBlocklistedCall) -> Self {
            Self::IsBlocklisted(value)
        }
    }
    impl ::core::convert::From<RemoveBlocklistAdminCall> for BlocklistedRoleCalls {
        fn from(value: RemoveBlocklistAdminCall) -> Self {
            Self::RemoveBlocklistAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveBlocklistedCall> for BlocklistedRoleCalls {
        fn from(value: RemoveBlocklistedCall) -> Self {
            Self::RemoveBlocklisted(value)
        }
    }
    impl ::core::convert::From<RenounceBlocklistAdminCall> for BlocklistedRoleCalls {
        fn from(value: RenounceBlocklistAdminCall) -> Self {
            Self::RenounceBlocklistAdmin(value)
        }
    }
    ///Container type for all return fields from the `isBlocklistAdmin` function with signature `isBlocklistAdmin(address,address)` and selector `0xa28e7c02`
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
    pub struct IsBlocklistAdminReturn(pub bool);
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
}
