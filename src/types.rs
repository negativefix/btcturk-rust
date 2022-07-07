use core::f64;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    data: Data,
    pub success: bool,
    pub message: String,
    pub code: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub time_zone: String,
    pub server_time: u64,
    pub symbols: Vec<Symbol>,
    pub currencies: Vec<Currency>,
    pub currency_operation_blocks: Vec<CurrencyOperationStatus>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub id: u64,
    pub name: String,
    pub name_normalized: String,
    pub status: String,
    pub numerator: String,
    pub denominator: String,
    pub numerator_scale: u64,
    pub denominator_scale: u64,
    pub has_fraction: bool,
    pub filters: Vec<SymbolFilter>,
    pub order_methods: Vec<String>,
    pub display_format: String,
    pub commission_from_numerator: bool,
    pub order: u64,
    pub price_rounding: bool,
    pub is_new: bool,
    pub market_price_warning_threshold_percentage: f64,
    pub maximum_order_amount: Option<f64>,
    pub maximum_limit_order_price: f64,
    pub minimum_limit_order_price: f64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SymbolFilter {
    pub filter_type: String,
    pub min_price: String,
    pub max_price: String,
    pub tick_size: String,
    pub min_exchange_value: String,
    pub min_amount: Option<String>,
    pub max_amount: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub min_len: Option<u64>,
    pub max_len: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub enable: bool,
    pub name: Option<String>,
    pub min_len: Option<f64>,
    pub max_len: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub id: u64,
    pub symbol: String,
    pub min_withdrawal: f64,
    pub min_deposit: f64,
    pub precision: u64,
    pub address: Address,
    pub currency_type: String,
    pub tag: Tag,
    pub color: String,
    pub name: String,
    pub is_address_renewable: bool,
    pub get_auto_address_disabled: bool,
    pub is_partial_withdrawal_enabled: bool,
    pub is_new: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyOperationStatus {
    pub currency_symbol: String,
    pub withdrawal_disabled: bool,
    pub deposit_disabled: bool,
}

// types for ticker endpoint
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    data: Option<Vec<PairData>>,
    success: bool,
    message: Option<String>,
    code: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PairData {
    pair: String,
    pair_normalized: String,
    last: f64,
    high: f64,
    low: f64,
    bid: f64,
    ask: f64,
    open: f64,
    volume: f64,
    average: f64,
    daily: f64,
    daily_percent: f64,
    denominator_symbol: String,
    numerator_symbol: String,
    order: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub data: Option<OrderBookData>,
    pub success: bool,
    pub message: Option<String>,
    pub code: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookData {
   pub timestamp: String,
   pub bids: Vec<Vec<String>>,
   pub asks: Vec<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub data: Option<TradeData>,
    pub success: bool,
    pub message: Option<String>,
    pub code: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeData {
    pub data: Option<Vec<TradePair>>,
    pub success: bool,
    pub message: Option<String>,
    pub code: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradePair {
    pub pair: String,
    pub pair_normalized: String,
    pub numerator: String,
    pub denominator: String,
    pub date: u64,
    pub tid: String,
    pub price: String,
    pub amount: String,
    pub side: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ohlc {
    data: Vec<OhlcPair>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OhlcPair {
    pair: String,
    time: u64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    total: f64,
    average: f64,
    daily_change_amount: f64,
    daily_change_percentage: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    s: Option<String>,
    nb: Option<u64>,
    t: Vec<u64>,
    h: Vec<f64>,
    o: Vec<f64>,
    l: Vec<f64>,
    c: Vec<f64>,
    v: Vec<f64>,
}







