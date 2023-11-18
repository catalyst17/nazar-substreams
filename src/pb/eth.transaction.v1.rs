// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub chain: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub account_abstraction_type: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
// @@protoc_insertion_point(module)
