pub use ierc1400::*;
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
pub mod ierc1400 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AuthorizedOperator\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AuthorizedOperatorByPartition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"fromPartition\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"toPartition\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ChangedPartition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"name\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"uri\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"documentHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DocumentRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"name\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"uri\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"documentHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DocumentUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Issued\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IssuedByPartition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Redeemed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RedeemedByPartition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RevokedOperator\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RevokedOperatorByPartition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"fromPartition\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferByPartition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowanceByPartition\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"authorizeOperator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"authorizeOperatorByPartition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfByPartition\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllDocuments\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_name\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDocument\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isControllable\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isIssuable\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOperator\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOperatorForPartition\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"issue\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"issueByPartition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"operatorRedeemByPartition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"operatorTransferByPartition\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"partitionsOf\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeem\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemByPartition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeemFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_name\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeDocument\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revokeOperator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revokeOperatorByPartition\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_name\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_uri\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_documentHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDocument\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"partition\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferByPartition\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFromWithData\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferWithData\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC1400_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IERC1400<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC1400<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC1400<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC1400<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC1400<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC1400)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC1400<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IERC1400_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowanceByPartition` (0x17ec83ca) function
        pub fn allowance_by_partition(
            &self,
            partition: [u8; 32],
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([23, 236, 131, 202], (partition, owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `authorizeOperator` (0x959b8c3f) function
        pub fn authorize_operator(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 155, 140, 63], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `authorizeOperatorByPartition` (0x103ef9e1) function
        pub fn authorize_operator_by_partition(
            &self,
            partition: [u8; 32],
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 62, 249, 225], (partition, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOfByPartition` (0x30e82803) function
        pub fn balance_of_by_partition(
            &self,
            partition: [u8; 32],
            token_holder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([48, 232, 40, 3], (partition, token_holder))
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `isControllable` (0x4c783bf5) function
        pub fn is_controllable(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([76, 120, 59, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isIssuable` (0x2f1cae85) function
        pub fn is_issuable(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 28, 174, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperator` (0xb6363cf2) function
        pub fn is_operator(
            &self,
            operator: ::ethers::core::types::Address,
            token_holder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([182, 54, 60, 242], (operator, token_holder))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperatorForPartition` (0x6d77cad6) function
        pub fn is_operator_for_partition(
            &self,
            partition: [u8; 32],
            operator: ::ethers::core::types::Address,
            token_holder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 119, 202, 214], (partition, operator, token_holder))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `issue` (0xbb3acde9) function
        pub fn issue(
            &self,
            token_holder: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 58, 205, 233], (token_holder, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `issueByPartition` (0x67c84919) function
        pub fn issue_by_partition(
            &self,
            partition: [u8; 32],
            token_holder: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 200, 73, 25], (partition, token_holder, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorRedeemByPartition` (0x98ddcec7) function
        pub fn operator_redeem_by_partition(
            &self,
            partition: [u8; 32],
            token_holder: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            operator_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [152, 221, 206, 199],
                    (partition, token_holder, value, operator_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorTransferByPartition` (0x8c0dee9c) function
        pub fn operator_transfer_by_partition(
            &self,
            partition: [u8; 32],
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operator_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [140, 13, 238, 156],
                    (partition, from, to, value, data, operator_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `partitionsOf` (0x740ab8f4) function
        pub fn partitions_of(
            &self,
            token_holder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([116, 10, 184, 244], token_holder)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeem` (0xe77c646d) function
        pub fn redeem(
            &self,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 124, 100, 109], (value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemByPartition` (0x62eb0068) function
        pub fn redeem_by_partition(
            &self,
            partition: [u8; 32],
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 235, 0, 104], (partition, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemFrom` (0x9675193c) function
        pub fn redeem_from(
            &self,
            token_holder: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 117, 25, 60], (token_holder, value, data))
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
        ///Calls the contract's `revokeOperator` (0xfad8b32a) function
        pub fn revoke_operator(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 216, 179, 42], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeOperatorByPartition` (0x168ecec5) function
        pub fn revoke_operator_by_partition(
            &self,
            partition: [u8; 32],
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 142, 206, 197], (partition, operator))
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
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferByPartition` (0xf3d490db) function
        pub fn transfer_by_partition(
            &self,
            partition: [u8; 32],
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([243, 212, 144, 219], (partition, to, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFromWithData` (0xee532f31) function
        pub fn transfer_from_with_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 83, 47, 49], (from, to, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferWithData` (0x2535f762) function
        pub fn transfer_with_data(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 53, 247, 98], (to, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AuthorizedOperator` event
        pub fn authorized_operator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AuthorizedOperatorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AuthorizedOperatorByPartition` event
        pub fn authorized_operator_by_partition_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AuthorizedOperatorByPartitionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ChangedPartition` event
        pub fn changed_partition_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChangedPartitionFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `Issued` event
        pub fn issued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IssuedFilter> {
            self.0.event()
        }
        ///Gets the contract's `IssuedByPartition` event
        pub fn issued_by_partition_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IssuedByPartitionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Redeemed` event
        pub fn redeemed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RedeemedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RedeemedByPartition` event
        pub fn redeemed_by_partition_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RedeemedByPartitionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RevokedOperator` event
        pub fn revoked_operator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RevokedOperatorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RevokedOperatorByPartition` event
        pub fn revoked_operator_by_partition_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RevokedOperatorByPartitionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TransferByPartition` event
        pub fn transfer_by_partition_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferByPartitionFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IERC1400Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IERC1400<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "AuthorizedOperator", abi = "AuthorizedOperator(address,address)")]
    pub struct AuthorizedOperatorFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_holder: ::ethers::core::types::Address,
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
        name = "AuthorizedOperatorByPartition",
        abi = "AuthorizedOperatorByPartition(bytes32,address,address)"
    )]
    pub struct AuthorizedOperatorByPartitionFilter {
        #[ethevent(indexed)]
        pub partition: [u8; 32],
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_holder: ::ethers::core::types::Address,
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
        name = "ChangedPartition",
        abi = "ChangedPartition(bytes32,bytes32,uint256)"
    )]
    pub struct ChangedPartitionFilter {
        #[ethevent(indexed)]
        pub from_partition: [u8; 32],
        #[ethevent(indexed)]
        pub to_partition: [u8; 32],
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Issued", abi = "Issued(address,address,uint256,bytes)")]
    pub struct IssuedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
        name = "IssuedByPartition",
        abi = "IssuedByPartition(bytes32,address,address,uint256,bytes,bytes)"
    )]
    pub struct IssuedByPartitionFilter {
        #[ethevent(indexed)]
        pub partition: [u8; 32],
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operator_data: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "Redeemed", abi = "Redeemed(address,address,uint256,bytes)")]
    pub struct RedeemedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
        name = "RedeemedByPartition",
        abi = "RedeemedByPartition(bytes32,address,address,uint256,bytes)"
    )]
    pub struct RedeemedByPartitionFilter {
        #[ethevent(indexed)]
        pub partition: [u8; 32],
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub operator_data: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "RevokedOperator", abi = "RevokedOperator(address,address)")]
    pub struct RevokedOperatorFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_holder: ::ethers::core::types::Address,
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
        name = "RevokedOperatorByPartition",
        abi = "RevokedOperatorByPartition(bytes32,address,address)"
    )]
    pub struct RevokedOperatorByPartitionFilter {
        #[ethevent(indexed)]
        pub partition: [u8; 32],
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_holder: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
        name = "TransferByPartition",
        abi = "TransferByPartition(bytes32,address,address,address,uint256,bytes,bytes)"
    )]
    pub struct TransferByPartitionFilter {
        #[ethevent(indexed)]
        pub from_partition: [u8; 32],
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC1400Events {
        ApprovalFilter(ApprovalFilter),
        AuthorizedOperatorFilter(AuthorizedOperatorFilter),
        AuthorizedOperatorByPartitionFilter(AuthorizedOperatorByPartitionFilter),
        ChangedPartitionFilter(ChangedPartitionFilter),
        DocumentRemovedFilter(DocumentRemovedFilter),
        DocumentUpdatedFilter(DocumentUpdatedFilter),
        IssuedFilter(IssuedFilter),
        IssuedByPartitionFilter(IssuedByPartitionFilter),
        RedeemedFilter(RedeemedFilter),
        RedeemedByPartitionFilter(RedeemedByPartitionFilter),
        RevokedOperatorFilter(RevokedOperatorFilter),
        RevokedOperatorByPartitionFilter(RevokedOperatorByPartitionFilter),
        TransferFilter(TransferFilter),
        TransferByPartitionFilter(TransferByPartitionFilter),
    }
    impl ::ethers::contract::EthLogDecode for IERC1400Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(IERC1400Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = AuthorizedOperatorFilter::decode_log(log) {
                return Ok(IERC1400Events::AuthorizedOperatorFilter(decoded));
            }
            if let Ok(decoded) = AuthorizedOperatorByPartitionFilter::decode_log(log) {
                return Ok(IERC1400Events::AuthorizedOperatorByPartitionFilter(decoded));
            }
            if let Ok(decoded) = ChangedPartitionFilter::decode_log(log) {
                return Ok(IERC1400Events::ChangedPartitionFilter(decoded));
            }
            if let Ok(decoded) = DocumentRemovedFilter::decode_log(log) {
                return Ok(IERC1400Events::DocumentRemovedFilter(decoded));
            }
            if let Ok(decoded) = DocumentUpdatedFilter::decode_log(log) {
                return Ok(IERC1400Events::DocumentUpdatedFilter(decoded));
            }
            if let Ok(decoded) = IssuedFilter::decode_log(log) {
                return Ok(IERC1400Events::IssuedFilter(decoded));
            }
            if let Ok(decoded) = IssuedByPartitionFilter::decode_log(log) {
                return Ok(IERC1400Events::IssuedByPartitionFilter(decoded));
            }
            if let Ok(decoded) = RedeemedFilter::decode_log(log) {
                return Ok(IERC1400Events::RedeemedFilter(decoded));
            }
            if let Ok(decoded) = RedeemedByPartitionFilter::decode_log(log) {
                return Ok(IERC1400Events::RedeemedByPartitionFilter(decoded));
            }
            if let Ok(decoded) = RevokedOperatorFilter::decode_log(log) {
                return Ok(IERC1400Events::RevokedOperatorFilter(decoded));
            }
            if let Ok(decoded) = RevokedOperatorByPartitionFilter::decode_log(log) {
                return Ok(IERC1400Events::RevokedOperatorByPartitionFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(IERC1400Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = TransferByPartitionFilter::decode_log(log) {
                return Ok(IERC1400Events::TransferByPartitionFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IERC1400Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorizedOperatorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AuthorizedOperatorByPartitionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangedPartitionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DocumentRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DocumentUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IssuedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IssuedByPartitionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemedByPartitionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokedOperatorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokedOperatorByPartitionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferByPartitionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for IERC1400Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<AuthorizedOperatorFilter> for IERC1400Events {
        fn from(value: AuthorizedOperatorFilter) -> Self {
            Self::AuthorizedOperatorFilter(value)
        }
    }
    impl ::core::convert::From<AuthorizedOperatorByPartitionFilter> for IERC1400Events {
        fn from(value: AuthorizedOperatorByPartitionFilter) -> Self {
            Self::AuthorizedOperatorByPartitionFilter(value)
        }
    }
    impl ::core::convert::From<ChangedPartitionFilter> for IERC1400Events {
        fn from(value: ChangedPartitionFilter) -> Self {
            Self::ChangedPartitionFilter(value)
        }
    }
    impl ::core::convert::From<DocumentRemovedFilter> for IERC1400Events {
        fn from(value: DocumentRemovedFilter) -> Self {
            Self::DocumentRemovedFilter(value)
        }
    }
    impl ::core::convert::From<DocumentUpdatedFilter> for IERC1400Events {
        fn from(value: DocumentUpdatedFilter) -> Self {
            Self::DocumentUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<IssuedFilter> for IERC1400Events {
        fn from(value: IssuedFilter) -> Self {
            Self::IssuedFilter(value)
        }
    }
    impl ::core::convert::From<IssuedByPartitionFilter> for IERC1400Events {
        fn from(value: IssuedByPartitionFilter) -> Self {
            Self::IssuedByPartitionFilter(value)
        }
    }
    impl ::core::convert::From<RedeemedFilter> for IERC1400Events {
        fn from(value: RedeemedFilter) -> Self {
            Self::RedeemedFilter(value)
        }
    }
    impl ::core::convert::From<RedeemedByPartitionFilter> for IERC1400Events {
        fn from(value: RedeemedByPartitionFilter) -> Self {
            Self::RedeemedByPartitionFilter(value)
        }
    }
    impl ::core::convert::From<RevokedOperatorFilter> for IERC1400Events {
        fn from(value: RevokedOperatorFilter) -> Self {
            Self::RevokedOperatorFilter(value)
        }
    }
    impl ::core::convert::From<RevokedOperatorByPartitionFilter> for IERC1400Events {
        fn from(value: RevokedOperatorByPartitionFilter) -> Self {
            Self::RevokedOperatorByPartitionFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for IERC1400Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<TransferByPartitionFilter> for IERC1400Events {
        fn from(value: TransferByPartitionFilter) -> Self {
            Self::TransferByPartitionFilter(value)
        }
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allowanceByPartition` function with signature `allowanceByPartition(bytes32,address,address)` and selector `0x17ec83ca`
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
        name = "allowanceByPartition",
        abi = "allowanceByPartition(bytes32,address,address)"
    )]
    pub struct AllowanceByPartitionCall {
        pub partition: [u8; 32],
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `authorizeOperator` function with signature `authorizeOperator(address)` and selector `0x959b8c3f`
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
    #[ethcall(name = "authorizeOperator", abi = "authorizeOperator(address)")]
    pub struct AuthorizeOperatorCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `authorizeOperatorByPartition` function with signature `authorizeOperatorByPartition(bytes32,address)` and selector `0x103ef9e1`
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
        name = "authorizeOperatorByPartition",
        abi = "authorizeOperatorByPartition(bytes32,address)"
    )]
    pub struct AuthorizeOperatorByPartitionCall {
        pub partition: [u8; 32],
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balanceOfByPartition` function with signature `balanceOfByPartition(bytes32,address)` and selector `0x30e82803`
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
        name = "balanceOfByPartition",
        abi = "balanceOfByPartition(bytes32,address)"
    )]
    pub struct BalanceOfByPartitionCall {
        pub partition: [u8; 32],
        pub token_holder: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `isControllable` function with signature `isControllable()` and selector `0x4c783bf5`
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
    #[ethcall(name = "isControllable", abi = "isControllable()")]
    pub struct IsControllableCall;
    ///Container type for all input parameters for the `isIssuable` function with signature `isIssuable()` and selector `0x2f1cae85`
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
    #[ethcall(name = "isIssuable", abi = "isIssuable()")]
    pub struct IsIssuableCall;
    ///Container type for all input parameters for the `isOperator` function with signature `isOperator(address,address)` and selector `0xb6363cf2`
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
    #[ethcall(name = "isOperator", abi = "isOperator(address,address)")]
    pub struct IsOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub token_holder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isOperatorForPartition` function with signature `isOperatorForPartition(bytes32,address,address)` and selector `0x6d77cad6`
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
        name = "isOperatorForPartition",
        abi = "isOperatorForPartition(bytes32,address,address)"
    )]
    pub struct IsOperatorForPartitionCall {
        pub partition: [u8; 32],
        pub operator: ::ethers::core::types::Address,
        pub token_holder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `issue` function with signature `issue(address,uint256,bytes)` and selector `0xbb3acde9`
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
    #[ethcall(name = "issue", abi = "issue(address,uint256,bytes)")]
    pub struct IssueCall {
        pub token_holder: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `issueByPartition` function with signature `issueByPartition(bytes32,address,uint256,bytes)` and selector `0x67c84919`
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
        name = "issueByPartition",
        abi = "issueByPartition(bytes32,address,uint256,bytes)"
    )]
    pub struct IssueByPartitionCall {
        pub partition: [u8; 32],
        pub token_holder: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `operatorRedeemByPartition` function with signature `operatorRedeemByPartition(bytes32,address,uint256,bytes)` and selector `0x98ddcec7`
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
        name = "operatorRedeemByPartition",
        abi = "operatorRedeemByPartition(bytes32,address,uint256,bytes)"
    )]
    pub struct OperatorRedeemByPartitionCall {
        pub partition: [u8; 32],
        pub token_holder: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub operator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `operatorTransferByPartition` function with signature `operatorTransferByPartition(bytes32,address,address,uint256,bytes,bytes)` and selector `0x8c0dee9c`
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
        name = "operatorTransferByPartition",
        abi = "operatorTransferByPartition(bytes32,address,address,uint256,bytes,bytes)"
    )]
    pub struct OperatorTransferByPartitionCall {
        pub partition: [u8; 32],
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `partitionsOf` function with signature `partitionsOf(address)` and selector `0x740ab8f4`
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
    #[ethcall(name = "partitionsOf", abi = "partitionsOf(address)")]
    pub struct PartitionsOfCall {
        pub token_holder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256,bytes)` and selector `0xe77c646d`
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
    #[ethcall(name = "redeem", abi = "redeem(uint256,bytes)")]
    pub struct RedeemCall {
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `redeemByPartition` function with signature `redeemByPartition(bytes32,uint256,bytes)` and selector `0x62eb0068`
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
        name = "redeemByPartition",
        abi = "redeemByPartition(bytes32,uint256,bytes)"
    )]
    pub struct RedeemByPartitionCall {
        pub partition: [u8; 32],
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `redeemFrom` function with signature `redeemFrom(address,uint256,bytes)` and selector `0x9675193c`
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
    #[ethcall(name = "redeemFrom", abi = "redeemFrom(address,uint256,bytes)")]
    pub struct RedeemFromCall {
        pub token_holder: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `revokeOperator` function with signature `revokeOperator(address)` and selector `0xfad8b32a`
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
    #[ethcall(name = "revokeOperator", abi = "revokeOperator(address)")]
    pub struct RevokeOperatorCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeOperatorByPartition` function with signature `revokeOperatorByPartition(bytes32,address)` and selector `0x168ecec5`
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
        name = "revokeOperatorByPartition",
        abi = "revokeOperatorByPartition(bytes32,address)"
    )]
    pub struct RevokeOperatorByPartitionCall {
        pub partition: [u8; 32],
        pub operator: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferByPartition` function with signature `transferByPartition(bytes32,address,uint256,bytes)` and selector `0xf3d490db`
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
        name = "transferByPartition",
        abi = "transferByPartition(bytes32,address,uint256,bytes)"
    )]
    pub struct TransferByPartitionCall {
        pub partition: [u8; 32],
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFromWithData` function with signature `transferFromWithData(address,address,uint256,bytes)` and selector `0xee532f31`
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
        name = "transferFromWithData",
        abi = "transferFromWithData(address,address,uint256,bytes)"
    )]
    pub struct TransferFromWithDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `transferWithData` function with signature `transferWithData(address,uint256,bytes)` and selector `0x2535f762`
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
        name = "transferWithData",
        abi = "transferWithData(address,uint256,bytes)"
    )]
    pub struct TransferWithDataCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC1400Calls {
        Allowance(AllowanceCall),
        AllowanceByPartition(AllowanceByPartitionCall),
        Approve(ApproveCall),
        AuthorizeOperator(AuthorizeOperatorCall),
        AuthorizeOperatorByPartition(AuthorizeOperatorByPartitionCall),
        BalanceOf(BalanceOfCall),
        BalanceOfByPartition(BalanceOfByPartitionCall),
        GetAllDocuments(GetAllDocumentsCall),
        GetDocument(GetDocumentCall),
        IsControllable(IsControllableCall),
        IsIssuable(IsIssuableCall),
        IsOperator(IsOperatorCall),
        IsOperatorForPartition(IsOperatorForPartitionCall),
        Issue(IssueCall),
        IssueByPartition(IssueByPartitionCall),
        OperatorRedeemByPartition(OperatorRedeemByPartitionCall),
        OperatorTransferByPartition(OperatorTransferByPartitionCall),
        PartitionsOf(PartitionsOfCall),
        Redeem(RedeemCall),
        RedeemByPartition(RedeemByPartitionCall),
        RedeemFrom(RedeemFromCall),
        RemoveDocument(RemoveDocumentCall),
        RevokeOperator(RevokeOperatorCall),
        RevokeOperatorByPartition(RevokeOperatorByPartitionCall),
        SetDocument(SetDocumentCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferByPartition(TransferByPartitionCall),
        TransferFrom(TransferFromCall),
        TransferFromWithData(TransferFromWithDataCall),
        TransferWithData(TransferWithDataCall),
    }
    impl ::ethers::core::abi::AbiDecode for IERC1400Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded)
                = <AllowanceByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AllowanceByPartition(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <AuthorizeOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AuthorizeOperator(decoded));
            }
            if let Ok(decoded)
                = <AuthorizeOperatorByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AuthorizeOperatorByPartition(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BalanceOfByPartition(decoded));
            }
            if let Ok(decoded)
                = <GetAllDocumentsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAllDocuments(decoded));
            }
            if let Ok(decoded)
                = <GetDocumentCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDocument(decoded));
            }
            if let Ok(decoded)
                = <IsControllableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsControllable(decoded));
            }
            if let Ok(decoded)
                = <IsIssuableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsIssuable(decoded));
            }
            if let Ok(decoded)
                = <IsOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsOperator(decoded));
            }
            if let Ok(decoded)
                = <IsOperatorForPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsOperatorForPartition(decoded));
            }
            if let Ok(decoded)
                = <IssueCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Issue(decoded));
            }
            if let Ok(decoded)
                = <IssueByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IssueByPartition(decoded));
            }
            if let Ok(decoded)
                = <OperatorRedeemByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OperatorRedeemByPartition(decoded));
            }
            if let Ok(decoded)
                = <OperatorTransferByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OperatorTransferByPartition(decoded));
            }
            if let Ok(decoded)
                = <PartitionsOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PartitionsOf(decoded));
            }
            if let Ok(decoded)
                = <RedeemCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Redeem(decoded));
            }
            if let Ok(decoded)
                = <RedeemByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RedeemByPartition(decoded));
            }
            if let Ok(decoded)
                = <RedeemFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RedeemFrom(decoded));
            }
            if let Ok(decoded)
                = <RemoveDocumentCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveDocument(decoded));
            }
            if let Ok(decoded)
                = <RevokeOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeOperator(decoded));
            }
            if let Ok(decoded)
                = <RevokeOperatorByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevokeOperatorByPartition(decoded));
            }
            if let Ok(decoded)
                = <SetDocumentCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDocument(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferByPartitionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferByPartition(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <TransferFromWithDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferFromWithData(decoded));
            }
            if let Ok(decoded)
                = <TransferWithDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferWithData(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC1400Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllowanceByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AuthorizeOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AuthorizeOperatorByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOfByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllDocuments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDocument(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsControllable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsIssuable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOperatorForPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Issue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IssueByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorRedeemByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorTransferByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PartitionsOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RedeemByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveDocument(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeOperatorByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDocument(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferByPartition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFromWithData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferWithData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IERC1400Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowanceByPartition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorizeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorizeOperatorByPartition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOfByPartition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAllDocuments(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDocument(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsControllable(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsIssuable(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperatorForPartition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Issue(element) => ::core::fmt::Display::fmt(element, f),
                Self::IssueByPartition(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorRedeemByPartition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorTransferByPartition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PartitionsOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemByPartition(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveDocument(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeOperatorByPartition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDocument(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferByPartition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFromWithData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferWithData(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for IERC1400Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<AllowanceByPartitionCall> for IERC1400Calls {
        fn from(value: AllowanceByPartitionCall) -> Self {
            Self::AllowanceByPartition(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for IERC1400Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AuthorizeOperatorCall> for IERC1400Calls {
        fn from(value: AuthorizeOperatorCall) -> Self {
            Self::AuthorizeOperator(value)
        }
    }
    impl ::core::convert::From<AuthorizeOperatorByPartitionCall> for IERC1400Calls {
        fn from(value: AuthorizeOperatorByPartitionCall) -> Self {
            Self::AuthorizeOperatorByPartition(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for IERC1400Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalanceOfByPartitionCall> for IERC1400Calls {
        fn from(value: BalanceOfByPartitionCall) -> Self {
            Self::BalanceOfByPartition(value)
        }
    }
    impl ::core::convert::From<GetAllDocumentsCall> for IERC1400Calls {
        fn from(value: GetAllDocumentsCall) -> Self {
            Self::GetAllDocuments(value)
        }
    }
    impl ::core::convert::From<GetDocumentCall> for IERC1400Calls {
        fn from(value: GetDocumentCall) -> Self {
            Self::GetDocument(value)
        }
    }
    impl ::core::convert::From<IsControllableCall> for IERC1400Calls {
        fn from(value: IsControllableCall) -> Self {
            Self::IsControllable(value)
        }
    }
    impl ::core::convert::From<IsIssuableCall> for IERC1400Calls {
        fn from(value: IsIssuableCall) -> Self {
            Self::IsIssuable(value)
        }
    }
    impl ::core::convert::From<IsOperatorCall> for IERC1400Calls {
        fn from(value: IsOperatorCall) -> Self {
            Self::IsOperator(value)
        }
    }
    impl ::core::convert::From<IsOperatorForPartitionCall> for IERC1400Calls {
        fn from(value: IsOperatorForPartitionCall) -> Self {
            Self::IsOperatorForPartition(value)
        }
    }
    impl ::core::convert::From<IssueCall> for IERC1400Calls {
        fn from(value: IssueCall) -> Self {
            Self::Issue(value)
        }
    }
    impl ::core::convert::From<IssueByPartitionCall> for IERC1400Calls {
        fn from(value: IssueByPartitionCall) -> Self {
            Self::IssueByPartition(value)
        }
    }
    impl ::core::convert::From<OperatorRedeemByPartitionCall> for IERC1400Calls {
        fn from(value: OperatorRedeemByPartitionCall) -> Self {
            Self::OperatorRedeemByPartition(value)
        }
    }
    impl ::core::convert::From<OperatorTransferByPartitionCall> for IERC1400Calls {
        fn from(value: OperatorTransferByPartitionCall) -> Self {
            Self::OperatorTransferByPartition(value)
        }
    }
    impl ::core::convert::From<PartitionsOfCall> for IERC1400Calls {
        fn from(value: PartitionsOfCall) -> Self {
            Self::PartitionsOf(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for IERC1400Calls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RedeemByPartitionCall> for IERC1400Calls {
        fn from(value: RedeemByPartitionCall) -> Self {
            Self::RedeemByPartition(value)
        }
    }
    impl ::core::convert::From<RedeemFromCall> for IERC1400Calls {
        fn from(value: RedeemFromCall) -> Self {
            Self::RedeemFrom(value)
        }
    }
    impl ::core::convert::From<RemoveDocumentCall> for IERC1400Calls {
        fn from(value: RemoveDocumentCall) -> Self {
            Self::RemoveDocument(value)
        }
    }
    impl ::core::convert::From<RevokeOperatorCall> for IERC1400Calls {
        fn from(value: RevokeOperatorCall) -> Self {
            Self::RevokeOperator(value)
        }
    }
    impl ::core::convert::From<RevokeOperatorByPartitionCall> for IERC1400Calls {
        fn from(value: RevokeOperatorByPartitionCall) -> Self {
            Self::RevokeOperatorByPartition(value)
        }
    }
    impl ::core::convert::From<SetDocumentCall> for IERC1400Calls {
        fn from(value: SetDocumentCall) -> Self {
            Self::SetDocument(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for IERC1400Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for IERC1400Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferByPartitionCall> for IERC1400Calls {
        fn from(value: TransferByPartitionCall) -> Self {
            Self::TransferByPartition(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for IERC1400Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferFromWithDataCall> for IERC1400Calls {
        fn from(value: TransferFromWithDataCall) -> Self {
            Self::TransferFromWithData(value)
        }
    }
    impl ::core::convert::From<TransferWithDataCall> for IERC1400Calls {
        fn from(value: TransferWithDataCall) -> Self {
            Self::TransferWithData(value)
        }
    }
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `allowanceByPartition` function with signature `allowanceByPartition(bytes32,address,address)` and selector `0x17ec83ca`
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
    pub struct AllowanceByPartitionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOfByPartition` function with signature `balanceOfByPartition(bytes32,address)` and selector `0x30e82803`
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
    pub struct BalanceOfByPartitionReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `isControllable` function with signature `isControllable()` and selector `0x4c783bf5`
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
    pub struct IsControllableReturn(pub bool);
    ///Container type for all return fields from the `isIssuable` function with signature `isIssuable()` and selector `0x2f1cae85`
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
    pub struct IsIssuableReturn(pub bool);
    ///Container type for all return fields from the `isOperator` function with signature `isOperator(address,address)` and selector `0xb6363cf2`
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
    pub struct IsOperatorReturn(pub bool);
    ///Container type for all return fields from the `isOperatorForPartition` function with signature `isOperatorForPartition(bytes32,address,address)` and selector `0x6d77cad6`
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
    pub struct IsOperatorForPartitionReturn(pub bool);
    ///Container type for all return fields from the `operatorTransferByPartition` function with signature `operatorTransferByPartition(bytes32,address,address,uint256,bytes,bytes)` and selector `0x8c0dee9c`
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
    pub struct OperatorTransferByPartitionReturn(pub [u8; 32]);
    ///Container type for all return fields from the `partitionsOf` function with signature `partitionsOf(address)` and selector `0x740ab8f4`
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
    pub struct PartitionsOfReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferByPartition` function with signature `transferByPartition(bytes32,address,uint256,bytes)` and selector `0xf3d490db`
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
    pub struct TransferByPartitionReturn(pub [u8; 32]);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}
