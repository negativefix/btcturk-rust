#[derive(Debug)]
pub struct ExchangeInfo {
    pub timezone: String,
    pub server_time: u64,
    pub symbols: Vec<Symbol>,
    pub currencies: Vec<Currency>,
    pub currecy_operation_blocks: Vec<CurrencyOperationStatus>,
    pub success: bool,
    pub message: String,
    pub code: u64,
}

#[derive(Debug)]
struct Symbol {
    pub id: u64,
    pub name: String,
    pub name_normalized: String,
    pub status: String,
    pub numerator: String,
    pub denominator: String,
    pub numeral_scale: u64,
    pub denominator_scale: u64,
    pub has_fraction: bool,
    pub filters: Vec<SymbolFilter>,
    pub order_methods: Vec<String>,
    pub display_format: String,
    pub commisson_from_numerator: bool,
    pub order: u64,
    pub price_rounding: bool,
    pub is_new: bool,
    pub market_price_warning_threshold_percentage: f64,
    pub maximum_order_amount: Option<f64>,
    pub maximum_limit_order_price: f64,
    pub minimum_limit_order_price: f64,
}

#[derive(Debug)]
struct SymbolFilter {
    pub filter_type: String,
    pub min_price: f64,
    pub max_price: f64,
    pub tick_size: u64,
    pub min_exchange_value: f64,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
}

#[derive(Debug)]
struct Address {
    pub min_len: Option<u64>,
    pub max_len: Option<u64>,
}

#[derive(Debug)]
pub struct Tag {
    pub enable: bool,
    pub name: Option<String>,
    pub min_len: Option<f64>,
    pub max_len: Option<f64>,
}

#[derive(Debug)]
struct Currency {
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

#[derive(Debug)]
pub struct CurrencyOperationStatus {
    pub currency_symbol: String,
    pub withdraw_disabled: bool,
    pub deposit_disabled: bool,
}

