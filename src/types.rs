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

struct Symbol {
    id: u64,
    name: String,
    name_normalized: String,
    status: String,
    numerator: String,
    denominator: String,
    numeral_scale: u64,
    denominator_scale: u64,
    has_fraction: bool,
    filters: Vec<SymbolFilter>,
    order_methods: Vec<String>,
    display_format: String,
    commisson_from_numerator: bool,
    order: u64,
    price_rounding: bool,
    is_new: bool,
    market_price_warning_threshold_percentage: f64,
    maximum_order_amount: Option<f64>,
    maximum_limit_order_price: f64,
    minimum_limit_order_price: f64,
}

struct SymbolFilter {
    filter_type: String,
    min_price: f64,
    max_price: f64,
    tick_size: u64,
    min_exchange_value: f64,
    min_amount: Option<f64>,
    max_amount: Option<f64>,
}

struct Address {
    min_len: Option<u64>,
    max_len: Option<u64>,
}

struct Tag {
    enable: bool,
    name: Option<String>,
    min_len: Option<f64>,
    max_len: Option<f64>,
}

struct Currency {
    id: u64,
    symbol: String,
    min_withdrawal: f64,
    min_deposit: f64,
    precision: u64,
    address: Address,
    currency_type: String,
    tag: Tag,
    color: String,
    name: String,
    is_address_renewable: bool,
    get_auto_address_disabled: bool,
    is_partial_withdrawal_enabled: bool,
    is_new: bool,
}

struct CurrencyOperationStatus {
    currency_symbol: String,
    withdraw_disabled: bool,
    deposit_disabled: bool,
}






