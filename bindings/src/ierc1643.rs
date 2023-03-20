pub use ierc1643::*;
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
pub mod ierc1643 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"name\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"uri\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"documentHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DocumentRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"name\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"uri\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"documentHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DocumentUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllDocuments\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_name\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDocument\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_name\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeDocument\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_name\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_uri\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_documentHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDocument\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC1643_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IERC1643<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC1643<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC1643<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC1643<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC1643<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC1643)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC1643<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IERC1643_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getAllDocuments` (0x9fa5f50b) function
        pub fn get_all_documents(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([159, 165, 245, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDocument` (0xb10d6b41) function
        pub fn get_document(
            &self,
            name: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::string::String, [u8; 32], ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([177, 13, 107, 65], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeDocument` (0xc3501848) function
        pub fn remove_document(
            &self,
            name: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 80, 24, 72], name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDocument` (0x010648ca) function
        pub fn set_document(
            &self,
            name: [u8; 32],
            uri: ::std::string::String,
            document_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 6, 72, 202], (name, uri, document_hash))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DocumentRemoved` event
        pub fn document_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DocumentRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DocumentUpdated` event
        pub fn document_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DocumentUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IERC1643Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IERC1643<M> {
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
        name = "DocumentRemoved",
        abi = "DocumentRemoved(bytes32,string,bytes32)"
    )]
    pub struct DocumentRemovedFilter {
        #[ethevent(indexed)]
        pub name: [u8; 32],
        pub uri: ::std::string::String,
        pub document_hash: [u8; 32],
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
        name = "DocumentUpdated",
        abi = "DocumentUpdated(bytes32,string,bytes32)"
    )]
    pub struct DocumentUpdatedFilter {
        #[ethevent(indexed)]
        pub name: [u8; 32],
        pub uri: ::std::string::String,
        pub document_hash: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC1643Events {
        DocumentRemovedFilter(DocumentRemovedFilter),
        DocumentUpdatedFilter(DocumentUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IERC1643Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DocumentRemovedFilter::decode_log(log) {
                return Ok(IERC1643Events::DocumentRemovedFilter(decoded));
            }
            if let Ok(decoded) = DocumentUpdatedFilter::decode_log(log) {
                return Ok(IERC1643Events::DocumentUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IERC1643Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DocumentRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DocumentUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DocumentRemovedFilter> for IERC1643Events {
        fn from(value: DocumentRemovedFilter) -> Self {
            Self::DocumentRemovedFilter(value)
        }
    }
    impl ::core::convert::From<DocumentUpdatedFilter> for IERC1643Events {
        fn from(value: DocumentUpdatedFilter) -> Self {
            Self::DocumentUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `getAllDocuments` function with signature `getAllDocuments()` and selector `0x9fa5f50b`
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
    #[ethcall(name = "getAllDocuments", abi = "getAllDocuments()")]
    pub struct GetAllDocumentsCall;
    ///Container type for all input parameters for the `getDocument` function with signature `getDocument(bytes32)` and selector `0xb10d6b41`
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
    #[ethcall(name = "getDocument", abi = "getDocument(bytes32)")]
    pub struct GetDocumentCall {
        pub name: [u8; 32],
    }
    ///Container type for all input parameters for the `removeDocument` function with signature `removeDocument(bytes32)` and selector `0xc3501848`
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
    #[ethcall(name = "removeDocument", abi = "removeDocument(bytes32)")]
    pub struct RemoveDocumentCall {
        pub name: [u8; 32],
    }
    ///Container type for all input parameters for the `setDocument` function with signature `setDocument(bytes32,string,bytes32)` and selector `0x010648ca`
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
    #[ethcall(name = "setDocument", abi = "setDocument(bytes32,string,bytes32)")]
    pub struct SetDocumentCall {
        pub name: [u8; 32],
        pub uri: ::std::string::String,
        pub document_hash: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC1643Calls {
        GetAllDocuments(GetAllDocumentsCall),
        GetDocument(GetDocumentCall),
        RemoveDocument(RemoveDocumentCall),
        SetDocument(SetDocumentCall),
    }
    impl ::ethers::core::abi::AbiDecode for IERC1643Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetAllDocumentsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAllDocuments(decoded));
            }
            if let Ok(decoded)
                = <GetDocumentCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDocument(decoded));
            }
            if let Ok(decoded)
                = <RemoveDocumentCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveDocument(decoded));
            }
            if let Ok(decoded)
                = <SetDocumentCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDocument(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC1643Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetAllDocuments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDocument(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveDocument(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDocument(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IERC1643Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetAllDocuments(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDocument(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveDocument(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDocument(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetAllDocumentsCall> for IERC1643Calls {
        fn from(value: GetAllDocumentsCall) -> Self {
            Self::GetAllDocuments(value)
        }
    }
    impl ::core::convert::From<GetDocumentCall> for IERC1643Calls {
        fn from(value: GetDocumentCall) -> Self {
            Self::GetDocument(value)
        }
    }
    impl ::core::convert::From<RemoveDocumentCall> for IERC1643Calls {
        fn from(value: RemoveDocumentCall) -> Self {
            Self::RemoveDocument(value)
        }
    }
    impl ::core::convert::From<SetDocumentCall> for IERC1643Calls {
        fn from(value: SetDocumentCall) -> Self {
            Self::SetDocument(value)
        }
    }
    ///Container type for all return fields from the `getAllDocuments` function with signature `getAllDocuments()` and selector `0x9fa5f50b`
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
    pub struct GetAllDocumentsReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getDocument` function with signature `getDocument(bytes32)` and selector `0xb10d6b41`
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
    pub struct GetDocumentReturn(
        pub ::std::string::String,
        pub [u8; 32],
        pub ::ethers::core::types::U256,
    );
}
