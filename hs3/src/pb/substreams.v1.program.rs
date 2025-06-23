// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub wager_initialized_event_list: ::prost::alloc::vec::Vec<WagerInitializedEvent>,
    #[prost(message, repeated, tag="2")]
    pub wager_initialized_v2_event_list: ::prost::alloc::vec::Vec<WagerInitializedV2Event>,
    #[prost(message, repeated, tag="3")]
    pub wager_resolved_event_list: ::prost::alloc::vec::Vec<WagerResolvedEvent>,
    #[prost(message, repeated, tag="4")]
    pub wager_resolved_v2_event_list: ::prost::alloc::vec::Vec<WagerResolvedV2Event>,
    #[prost(message, repeated, tag="5")]
    pub finalize_wager_v2_instruction_list: ::prost::alloc::vec::Vec<FinalizeWagerV2Instruction>,
    #[prost(message, repeated, tag="6")]
    pub initialize_wager_instruction_list: ::prost::alloc::vec::Vec<InitializeWagerInstruction>,
    #[prost(message, repeated, tag="7")]
    pub initialize_wager_v2_instruction_list: ::prost::alloc::vec::Vec<InitializeWagerV2Instruction>,
    #[prost(message, repeated, tag="8")]
    pub resolve_wager_instruction_list: ::prost::alloc::vec::Vec<ResolveWagerInstruction>,
    #[prost(message, repeated, tag="9")]
    pub resolve_wager_v2_instruction_list: ::prost::alloc::vec::Vec<ResolveWagerV2Instruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WagerInitializedEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub wager: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub mint: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub amount: u64,
    #[prost(int64, tag="7")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WagerInitializedV2Event {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub wager: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub wager_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub payout_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub ipfs_cid: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub wager_amount: u64,
    #[prost(int64, tag="9")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WagerResolvedEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub wager: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub mint: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub payout_amount: u64,
    #[prost(int64, tag="7")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WagerResolvedV2Event {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub wager: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub wager_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub payout_mint: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub wager_amount: u64,
    #[prost(uint64, tag="8")]
    pub payout_amount: u64,
    #[prost(int64, tag="9")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeWagerV2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_wager: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_operator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeWagerInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(string, tag="3")]
    pub acct_wager: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_operator: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_operator_token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeWagerV2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub wager_amount: u64,
    #[prost(string, tag="3")]
    pub ipfs_cid: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_wager: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_operator: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_wager_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_payout_mint: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_operator_token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveWagerInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub payout_amount: u64,
    #[prost(string, tag="3")]
    pub acct_wager: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_operator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_user_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_operator_token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveWagerV2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub payout_amount: u64,
    #[prost(string, tag="3")]
    pub acct_wager: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_operator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_user_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_operator_token_account: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
