#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChessGame {
    #[prost(string, repeated, tag = "1")]
    pub moves: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChessDataSet {
    #[prost(message, repeated, tag = "1")]
    pub games: ::prost::alloc::vec::Vec<ChessGame>,
}
