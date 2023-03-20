pub use ierc1400_extended::*;
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
pub mod ierc1400_extended {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"controllers\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDefaultPartitions\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalPartitions\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupplyByPartition\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC1400EXTENDED_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IERC1400Extended<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC1400Extended<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC1400Extended<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC1400Extended<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC1400Extended<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC1400Extended)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC1400Extended<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IERC1400EXTENDED_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `controllers` (0x7cc0c3a7) function
        pub fn controllers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([124, 192, 195, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDefaultPartitions` (0xbcfdc0cf) function
        pub fn get_default_partitions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([188, 253, 192, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalPartitions` (0x69598efe) function
        pub fn total_partitions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([105, 89, 142, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupplyByPartition` (0xa26734dc) function
        pub fn total_supply_by_partition(
            &self,
            partition: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([162, 103, 52, 220], partition)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IERC1400Extended<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `controllers` function with signature `controllers()` and selector `0x7cc0c3a7`
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
    #[ethcall(name = "controllers", abi = "controllers()")]
    pub struct ControllersCall;
    ///Container type for all input parameters for the `getDefaultPartitions` function with signature `getDefaultPartitions()` and selector `0xbcfdc0cf`
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
    #[ethcall(name = "getDefaultPartitions", abi = "getDefaultPartitions()")]
    pub struct GetDefaultPartitionsCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `totalPartitions` function with signature `totalPartitions()` and selector `0x69598efe`
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
    #[ethcall(name = "totalPartitions", abi = "totalPartitions()")]
    pub struct TotalPartitionsCall;
    ///Container type for all input parameters for the `totalSupplyByPartition` function with signature `totalSupplyByPartition(bytes32)` and selector `0xa26734dc`
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
    #[ethcall(name = "totalSupplyByPartition", abi = "totalSupplyByPartition(bytes32)")]
    pub struct TotalSupplyByPartitionCall {
        pub partition: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC1400ExtendedCalls {
        Controllers(ControllersCall),
        GetDefaultPartitions(GetDefaultPartitionsCall),
        Owner(OwnerCall),
        TotalPartitions(TotalPartitionsCall),
        TotalSupplyByPartition(TotalSupplyByPartitionCall),
    }
    impl ::ethers::core::abi::AbiDecode for IERC1400ExtendedCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ControllersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Controllers(decoded));
            }
            if let Ok(decoded)
                = <GetDefaultPartitionsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetDefaultPartitions(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <TotalPartitionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalPartitions(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TotalSupplyByPartition(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC1400ExtendedCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Controllers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDefaultPartitions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalPartitions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupplyByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IERC1400ExtendedCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Controllers(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDefaultPartitions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalPartitions(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupplyByPartition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ControllersCall> for IERC1400ExtendedCalls {
        fn from(value: ControllersCall) -> Self {
            Self::Controllers(value)
        }
    }
    impl ::core::convert::From<GetDefaultPartitionsCall> for IERC1400ExtendedCalls {
        fn from(value: GetDefaultPartitionsCall) -> Self {
            Self::GetDefaultPartitions(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for IERC1400ExtendedCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<TotalPartitionsCall> for IERC1400ExtendedCalls {
        fn from(value: TotalPartitionsCall) -> Self {
            Self::TotalPartitions(value)
        }
    }
    impl ::core::convert::From<TotalSupplyByPartitionCall> for IERC1400ExtendedCalls {
        fn from(value: TotalSupplyByPartitionCall) -> Self {
            Self::TotalSupplyByPartition(value)
        }
    }
    ///Container type for all return fields from the `controllers` function with signature `controllers()` and selector `0x7cc0c3a7`
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
    pub struct ControllersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getDefaultPartitions` function with signature `getDefaultPartitions()` and selector `0xbcfdc0cf`
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
    pub struct GetDefaultPartitionsReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalPartitions` function with signature `totalPartitions()` and selector `0x69598efe`
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
    pub struct TotalPartitionsReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `totalSupplyByPartition` function with signature `totalSupplyByPartition(bytes32)` and selector `0xa26734dc`
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
    pub struct TotalSupplyByPartitionReturn(pub ::ethers::core::types::U256);
}
