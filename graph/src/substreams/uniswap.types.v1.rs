#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniswapTokens {
    #[prost(message, repeated, tag="1")]
    pub uniswap_tokens: ::prost::alloc::vec::Vec<UniswapToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniswapToken {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Liquidity {
    #[prost(string, tag="1")]
    pub pool_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub token0: ::core::option::Option<UniswapToken>,
    #[prost(message, optional, tag="3")]
    pub token1: ::core::option::Option<UniswapToken>,
    #[prost(string, tag="4")]
    pub creation_transaction_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub fee: u32,
    #[prost(string, tag="6")]
    pub block_num: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub log_ordinal: u64,
    #[prost(int32, tag="8")]
    pub tick_spacing: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInitializations {
    #[prost(message, repeated, tag="1")]
    pub pool_initializations: ::prost::alloc::vec::Vec<PoolInitialization>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInitialization {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub initialization_transaction_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub log_ordinal: u64,
    #[prost(string, tag="4")]
    pub tick: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub sqrt_price: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqrtPriceUpdates {
    #[prost(message, repeated, tag="1")]
    pub sqrt_prices: ::prost::alloc::vec::Vec<SqrtPriceUpdate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqrtPriceUpdate {
    #[prost(string, tag="1")]
    pub pool_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub ordinal: u64,
    #[prost(string, tag="3")]
    pub sqrt_price: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub tick: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(uint64, tag="100")]
    pub log_ordinal: u64,
    #[prost(string, tag="101")]
    pub pool_address: ::prost::alloc::string::String,
    #[prost(string, tag="102")]
    pub token0: ::prost::alloc::string::String,
    #[prost(string, tag="103")]
    pub token1: ::prost::alloc::string::String,
    #[prost(string, tag="104")]
    pub fee: ::prost::alloc::string::String,
    #[prost(string, tag="105")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="106")]
    pub timestamp: u64,
    #[prost(oneof="event::Type", tags="1, 2, 3")]
    pub r#type: ::core::option::Option<event::Type>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        Swap(super::Swap),
        #[prost(message, tag="2")]
        Burn(super::Burn),
        #[prost(message, tag="3")]
        Mint(super::Mint),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swaps {
    #[prost(message, repeated, tag="1")]
    pub swaps: ::prost::alloc::vec::Vec<Swap>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swap {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub amount_0: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount_1: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub sqrt_price: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub liquidity: ::prost::alloc::string::String,
    #[prost(int32, tag="8")]
    pub tick: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Burn {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount_0: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub amount_1: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub tick_lower: i32,
    #[prost(int32, tag="5")]
    pub tick_upper: i32,
    #[prost(string, tag="6")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mint {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount_0: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub amount_1: ::prost::alloc::string::String,
    #[prost(int32, tag="6")]
    pub tick_lower: i32,
    #[prost(int32, tag="7")]
    pub tick_upper: i32,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ticks {
    #[prost(message, repeated, tag="1")]
    pub ticks: ::prost::alloc::vec::Vec<Tick>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tick {
    #[prost(string, tag="1")]
    pub pool_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub idx: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub price0: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub price1: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fees {
    #[prost(message, repeated, tag="1")]
    pub fees: ::prost::alloc::vec::Vec<Fee>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fee {
    #[prost(uint32, tag="1")]
    pub fee: u32,
    #[prost(int32, tag="2")]
    pub tick_spacing: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flashes {
    #[prost(message, repeated, tag="1")]
    pub flashes: ::prost::alloc::vec::Vec<Flash>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flash {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub amount_0: u64,
    #[prost(uint64, tag="4")]
    pub amount_1: u64,
    #[prost(uint64, tag="5")]
    pub paid_0: u64,
    #[prost(uint64, tag="6")]
    pub paid_1: u64,
    #[prost(string, tag="7")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub log_ordinal: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntitiesChanges {
    #[prost(message, repeated, tag="1")]
    pub entity_changes: ::prost::alloc::vec::Vec<EntityChange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityChange {
    #[prost(string, tag="1")]
    pub entity: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub ordinal: u64,
    #[prost(enumeration="entity_change::Operation", tag="4")]
    pub operation: i32,
    #[prost(message, repeated, tag="5")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
}
/// Nested message and enum types in `EntityChange`.
pub mod entity_change {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        Unset = 0,
        Create = 1,
        Update = 2,
        Delete = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="field::Type", tag="2")]
    pub value_type: i32,
    #[prost(bytes="vec", tag="3")]
    pub new_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="4")]
    pub new_value_null: bool,
    #[prost(bytes="vec", tag="5")]
    pub old_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="6")]
    pub old_value_null: bool,
}
/// Nested message and enum types in `Field`.
pub mod field {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unset = 0,
        Bigdecimal = 1,
        Bigint = 2,
        /// int32
        Int = 3,
        Bytes = 4,
        String = 5,
    }
}
