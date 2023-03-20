pub use ierc20_holdable_token::*;
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
pub mod ierc20_holdable_token {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"lockPreimage\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExecutedHold\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"notary\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expirationDateTime\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"lockHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewHold\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReleaseHold\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOnHold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"lockPreimage\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeHold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"lockPreimage\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeHold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeHold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"notary\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expirationDateTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"lockHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"holdStatus\",\"outputs\":[{\"internalType\":\"enum HoldStatusCode\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"releaseHold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"holdId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"retrieveHoldData\",\"outputs\":[{\"internalType\":\"struct ERC20HoldData\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"notary\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expirationDateTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"secretHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"enum HoldStatusCode\",\"name\":\"status\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"spendableBalanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupplyOnHold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC20HOLDABLETOKEN_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IERC20HoldableToken<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC20HoldableToken<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC20HoldableToken<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC20HoldableToken<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC20HoldableToken<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC20HoldableToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC20HoldableToken<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IERC20HOLDABLETOKEN_ABI.clone(),
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
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOnHold` (0xc5a468c6) function
        pub fn balance_on_hold(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([197, 164, 104, 198], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeHold` (0x445850af) function
        pub fn execute_hold_1(
            &self,
            hold_id: [u8; 32],
            lock_preimage: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 88, 80, 175], (hold_id, lock_preimage))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeHold` (0x603dfcb5) function
        pub fn execute_hold_2(
            &self,
            hold_id: [u8; 32],
            lock_preimage: [u8; 32],
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 61, 252, 181], (hold_id, lock_preimage, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeHold` (0xf8abd4bc) function
        pub fn execute_hold_0(
            &self,
            hold_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 171, 212, 188], hold_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hold` (0xc0e8330e) function
        pub fn hold(
            &self,
            hold_id: [u8; 32],
            recipient: ::ethers::core::types::Address,
            notary: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            expiration_date_time: ::ethers::core::types::U256,
            lock_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [192, 232, 51, 14],
                    (hold_id, recipient, notary, amount, expiration_date_time, lock_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `holdStatus` (0xb67f7bd4) function
        pub fn hold_status(
            &self,
            hold_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([182, 127, 123, 212], hold_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `releaseHold` (0x9335c28f) function
        pub fn release_hold(
            &self,
            hold_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 53, 194, 143], hold_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `retrieveHoldData` (0xda1ec246) function
        pub fn retrieve_hold_data(
            &self,
            hold_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Erc20HoldData> {
            self.0
                .method_hash([218, 30, 194, 70], hold_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spendableBalanceOf` (0x0f8f8b83) function
        pub fn spendable_balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([15, 143, 139, 131], account)
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
        ///Calls the contract's `totalSupplyOnHold` (0x51bfb961) function
        pub fn total_supply_on_hold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([81, 191, 185, 97], ())
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
        ///Gets the contract's `ExecutedHold` event
        pub fn executed_hold_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutedHoldFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewHold` event
        pub fn new_hold_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewHoldFilter> {
            self.0.event()
        }
        ///Gets the contract's `ReleaseHold` event
        pub fn release_hold_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReleaseHoldFilter,
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IERC20HoldableTokenEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IERC20HoldableToken<M> {
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
    #[ethevent(name = "ExecutedHold", abi = "ExecutedHold(bytes32,bytes32,address)")]
    pub struct ExecutedHoldFilter {
        #[ethevent(indexed)]
        pub hold_id: [u8; 32],
        pub lock_preimage: [u8; 32],
        pub recipient: ::ethers::core::types::Address,
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
        name = "NewHold",
        abi = "NewHold(bytes32,address,address,uint256,uint256,bytes32)"
    )]
    pub struct NewHoldFilter {
        #[ethevent(indexed)]
        pub hold_id: [u8; 32],
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub notary: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub expiration_date_time: ::ethers::core::types::U256,
        pub lock_hash: [u8; 32],
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
    #[ethevent(name = "ReleaseHold", abi = "ReleaseHold(bytes32,address)")]
    pub struct ReleaseHoldFilter {
        #[ethevent(indexed)]
        pub hold_id: [u8; 32],
        pub sender: ::ethers::core::types::Address,
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC20HoldableTokenEvents {
        ApprovalFilter(ApprovalFilter),
        ExecutedHoldFilter(ExecutedHoldFilter),
        NewHoldFilter(NewHoldFilter),
        ReleaseHoldFilter(ReleaseHoldFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for IERC20HoldableTokenEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(IERC20HoldableTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ExecutedHoldFilter::decode_log(log) {
                return Ok(IERC20HoldableTokenEvents::ExecutedHoldFilter(decoded));
            }
            if let Ok(decoded) = NewHoldFilter::decode_log(log) {
                return Ok(IERC20HoldableTokenEvents::NewHoldFilter(decoded));
            }
            if let Ok(decoded) = ReleaseHoldFilter::decode_log(log) {
                return Ok(IERC20HoldableTokenEvents::ReleaseHoldFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(IERC20HoldableTokenEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IERC20HoldableTokenEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutedHoldFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewHoldFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseHoldFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for IERC20HoldableTokenEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ExecutedHoldFilter> for IERC20HoldableTokenEvents {
        fn from(value: ExecutedHoldFilter) -> Self {
            Self::ExecutedHoldFilter(value)
        }
    }
    impl ::core::convert::From<NewHoldFilter> for IERC20HoldableTokenEvents {
        fn from(value: NewHoldFilter) -> Self {
            Self::NewHoldFilter(value)
        }
    }
    impl ::core::convert::From<ReleaseHoldFilter> for IERC20HoldableTokenEvents {
        fn from(value: ReleaseHoldFilter) -> Self {
            Self::ReleaseHoldFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for IERC20HoldableTokenEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
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
    ///Container type for all input parameters for the `balanceOnHold` function with signature `balanceOnHold(address)` and selector `0xc5a468c6`
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
    #[ethcall(name = "balanceOnHold", abi = "balanceOnHold(address)")]
    pub struct BalanceOnHoldCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `executeHold` function with signature `executeHold(bytes32,bytes32)` and selector `0x445850af`
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
    #[ethcall(name = "executeHold", abi = "executeHold(bytes32,bytes32)")]
    pub struct ExecuteHold1Call {
        pub hold_id: [u8; 32],
        pub lock_preimage: [u8; 32],
    }
    ///Container type for all input parameters for the `executeHold` function with signature `executeHold(bytes32,bytes32,address)` and selector `0x603dfcb5`
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
    #[ethcall(name = "executeHold", abi = "executeHold(bytes32,bytes32,address)")]
    pub struct ExecuteHold2Call {
        pub hold_id: [u8; 32],
        pub lock_preimage: [u8; 32],
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `executeHold` function with signature `executeHold(bytes32)` and selector `0xf8abd4bc`
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
    #[ethcall(name = "executeHold", abi = "executeHold(bytes32)")]
    pub struct ExecuteHold0Call {
        pub hold_id: [u8; 32],
    }
    ///Container type for all input parameters for the `hold` function with signature `hold(bytes32,address,address,uint256,uint256,bytes32)` and selector `0xc0e8330e`
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
        name = "hold",
        abi = "hold(bytes32,address,address,uint256,uint256,bytes32)"
    )]
    pub struct HoldCall {
        pub hold_id: [u8; 32],
        pub recipient: ::ethers::core::types::Address,
        pub notary: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub expiration_date_time: ::ethers::core::types::U256,
        pub lock_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `holdStatus` function with signature `holdStatus(bytes32)` and selector `0xb67f7bd4`
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
    #[ethcall(name = "holdStatus", abi = "holdStatus(bytes32)")]
    pub struct HoldStatusCall {
        pub hold_id: [u8; 32],
    }
    ///Container type for all input parameters for the `releaseHold` function with signature `releaseHold(bytes32)` and selector `0x9335c28f`
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
    #[ethcall(name = "releaseHold", abi = "releaseHold(bytes32)")]
    pub struct ReleaseHoldCall {
        pub hold_id: [u8; 32],
    }
    ///Container type for all input parameters for the `retrieveHoldData` function with signature `retrieveHoldData(bytes32)` and selector `0xda1ec246`
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
    #[ethcall(name = "retrieveHoldData", abi = "retrieveHoldData(bytes32)")]
    pub struct RetrieveHoldDataCall {
        pub hold_id: [u8; 32],
    }
    ///Container type for all input parameters for the `spendableBalanceOf` function with signature `spendableBalanceOf(address)` and selector `0x0f8f8b83`
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
    #[ethcall(name = "spendableBalanceOf", abi = "spendableBalanceOf(address)")]
    pub struct SpendableBalanceOfCall {
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `totalSupplyOnHold` function with signature `totalSupplyOnHold()` and selector `0x51bfb961`
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
    #[ethcall(name = "totalSupplyOnHold", abi = "totalSupplyOnHold()")]
    pub struct TotalSupplyOnHoldCall;
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC20HoldableTokenCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BalanceOnHold(BalanceOnHoldCall),
        ExecuteHold1(ExecuteHold1Call),
        ExecuteHold2(ExecuteHold2Call),
        ExecuteHold0(ExecuteHold0Call),
        Hold(HoldCall),
        HoldStatus(HoldStatusCall),
        ReleaseHold(ReleaseHoldCall),
        RetrieveHoldData(RetrieveHoldDataCall),
        SpendableBalanceOf(SpendableBalanceOfCall),
        TotalSupply(TotalSupplyCall),
        TotalSupplyOnHold(TotalSupplyOnHoldCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for IERC20HoldableTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BalanceOnHoldCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOnHold(decoded));
            }
            if let Ok(decoded)
                = <ExecuteHold1Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteHold1(decoded));
            }
            if let Ok(decoded)
                = <ExecuteHold2Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteHold2(decoded));
            }
            if let Ok(decoded)
                = <ExecuteHold0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteHold0(decoded));
            }
            if let Ok(decoded)
                = <HoldCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Hold(decoded));
            }
            if let Ok(decoded)
                = <HoldStatusCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HoldStatus(decoded));
            }
            if let Ok(decoded)
                = <ReleaseHoldCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReleaseHold(decoded));
            }
            if let Ok(decoded)
                = <RetrieveHoldDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RetrieveHoldData(decoded));
            }
            if let Ok(decoded)
                = <SpendableBalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SpendableBalanceOf(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyOnHoldCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TotalSupplyOnHold(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC20HoldableTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOnHold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteHold1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteHold2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteHold0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Hold(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HoldStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReleaseHold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RetrieveHoldData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpendableBalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupplyOnHold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IERC20HoldableTokenCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOnHold(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteHold1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteHold2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteHold0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Hold(element) => ::core::fmt::Display::fmt(element, f),
                Self::HoldStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReleaseHold(element) => ::core::fmt::Display::fmt(element, f),
                Self::RetrieveHoldData(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpendableBalanceOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupplyOnHold(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for IERC20HoldableTokenCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for IERC20HoldableTokenCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for IERC20HoldableTokenCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalanceOnHoldCall> for IERC20HoldableTokenCalls {
        fn from(value: BalanceOnHoldCall) -> Self {
            Self::BalanceOnHold(value)
        }
    }
    impl ::core::convert::From<ExecuteHold1Call> for IERC20HoldableTokenCalls {
        fn from(value: ExecuteHold1Call) -> Self {
            Self::ExecuteHold1(value)
        }
    }
    impl ::core::convert::From<ExecuteHold2Call> for IERC20HoldableTokenCalls {
        fn from(value: ExecuteHold2Call) -> Self {
            Self::ExecuteHold2(value)
        }
    }
    impl ::core::convert::From<ExecuteHold0Call> for IERC20HoldableTokenCalls {
        fn from(value: ExecuteHold0Call) -> Self {
            Self::ExecuteHold0(value)
        }
    }
    impl ::core::convert::From<HoldCall> for IERC20HoldableTokenCalls {
        fn from(value: HoldCall) -> Self {
            Self::Hold(value)
        }
    }
    impl ::core::convert::From<HoldStatusCall> for IERC20HoldableTokenCalls {
        fn from(value: HoldStatusCall) -> Self {
            Self::HoldStatus(value)
        }
    }
    impl ::core::convert::From<ReleaseHoldCall> for IERC20HoldableTokenCalls {
        fn from(value: ReleaseHoldCall) -> Self {
            Self::ReleaseHold(value)
        }
    }
    impl ::core::convert::From<RetrieveHoldDataCall> for IERC20HoldableTokenCalls {
        fn from(value: RetrieveHoldDataCall) -> Self {
            Self::RetrieveHoldData(value)
        }
    }
    impl ::core::convert::From<SpendableBalanceOfCall> for IERC20HoldableTokenCalls {
        fn from(value: SpendableBalanceOfCall) -> Self {
            Self::SpendableBalanceOf(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for IERC20HoldableTokenCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TotalSupplyOnHoldCall> for IERC20HoldableTokenCalls {
        fn from(value: TotalSupplyOnHoldCall) -> Self {
            Self::TotalSupplyOnHold(value)
        }
    }
    impl ::core::convert::From<TransferCall> for IERC20HoldableTokenCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for IERC20HoldableTokenCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
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
    ///Container type for all return fields from the `balanceOnHold` function with signature `balanceOnHold(address)` and selector `0xc5a468c6`
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
    pub struct BalanceOnHoldReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `holdStatus` function with signature `holdStatus(bytes32)` and selector `0xb67f7bd4`
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
    pub struct HoldStatusReturn(pub u8);
    ///Container type for all return fields from the `retrieveHoldData` function with signature `retrieveHoldData(bytes32)` and selector `0xda1ec246`
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
    pub struct RetrieveHoldDataReturn(pub Erc20HoldData);
    ///Container type for all return fields from the `spendableBalanceOf` function with signature `spendableBalanceOf(address)` and selector `0x0f8f8b83`
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
    pub struct SpendableBalanceOfReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `totalSupplyOnHold` function with signature `totalSupplyOnHold()` and selector `0x51bfb961`
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
    pub struct TotalSupplyOnHoldReturn(pub ::ethers::core::types::U256);
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
