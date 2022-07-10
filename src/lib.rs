use chrono::{self, Utc, DateTime};
use reqwest::Url;

// TODO - create types for responses
// TODO - add integrations tests for the public/private endpoints/
// TODO - configure .env to read apikey and secret from environment within integration tests
// TODO - check if utc nonces matches the server

mod types;
mod errors;

pub use errors::BTCTRResult;
pub use types::{
    ServerTime,
    ExchangeInfo, 
    Pair, 
    OrderBook, 
    Trade, 
    Ohlc, 
    Kline, 
    OhlcParams, 
    KlineParams,
    TradesParams, 
    OrderBookParams
};


// move api key and screet to env
pub const BASE_API_URL: &str = "https://api.btcturk.com";
pub const GRAPH_API_URL: &str = "https://graph-api.btcturk.com";


fn create_endpoint_url(path: &str, graph: bool) -> Result<reqwest::Url, url::ParseError> {
    let base_url = if graph { GRAPH_API_URL } else { BASE_API_URL };
    let mut url = Url::parse(base_url)?;
    url.set_path(path);
    Ok(url)
}

// Config for authenticated requests
// better name would be AuthConfig
pub struct Config {
    api_key: String,
    api_secret: String,
    nonce: DateTime<Utc>,
}

impl Config {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        Self { 
            api_key: api_key.to_owned(),
            api_secret: api_secret.to_owned(), 
            nonce: chrono::offset::Utc::now()
        }
    }
}


pub struct PublicApi;

impl PublicApi {

    pub async fn server_time(&self) -> BTCTRResult<ServerTime> {
        let url = create_endpoint_url("/api/v2/server/time", false)?;
        let data = reqwest::get(url.as_str()).await?
            .error_for_status()?
            .json()
            .await?;
        Ok(data)
    }

    pub async fn exchange_info(&self) -> BTCTRResult<ExchangeInfo> {
        let url = create_endpoint_url("/api/v2/server/exchangeinfo", false)?;
        let data = reqwest::get(url.as_str()).await?
            .error_for_status()?
            .json()
            .await?;
        Ok(data)
    }

    pub async fn pair(&self, pair_symbol: Option<&str>) -> BTCTRResult<Pair> {
        let mut url = create_endpoint_url("/api/v2/ticker", false)?;
        if let Some(pair) = pair_symbol {
            url
                .query_pairs_mut()
                .append_pair("pairSymbol", pair);
        }
        let data = reqwest::get(url.as_str()).await?
            .error_for_status()?
            .json()
            .await?;
        Ok(data)
    }

    pub async fn currency(&self, symbol: &str) -> BTCTRResult<Pair>{
        let mut url = create_endpoint_url("/api/v2/ticker/currency", false)?;
        url
            .query_pairs_mut()
            .append_pair("symbol", symbol);
        let data = reqwest::get(url.as_str()).await?
            .error_for_status()?
            .json()
            .await?;
        Ok(data)
    }
    
    pub async fn order_book(&self, pair_symbol: &str, limit: Option<u32>) -> BTCTRResult<OrderBook> {
        let mut url = create_endpoint_url("/api/v2/orderbook", false)?;
        let params = OrderBookParams { pair_symbol, limit };
        let url_params = serde_url_params::to_string(&params)?;
        url.set_query(Some(&url_params));
        let data = reqwest::get(url.as_str()).await?
            .error_for_status()?
            .json()
            .await?;
        Ok(data)
    }

    pub async fn trades(&self, pair_symbol: &str, last: Option<u32>) -> BTCTRResult<Trade> {
        let mut url = create_endpoint_url("/api/v2/trades", false)?;
        let params = TradesParams { pair_symbol, last };
        let url_params = serde_url_params::to_string(&params)?;
        url.set_query(Some(&url_params));
        let data = reqwest::get(url.as_str()).await?
            .error_for_status()?
            .json()
            .await?;
        Ok(data)
    }

    pub async fn ohlc_data(&self, pair_symbol: &str, from: Option<u64>, to: Option<u64>) -> BTCTRResult<Ohlc> {
        let mut url = create_endpoint_url("/v1/ohlcs", true)?;
        let params = OhlcParams { pair_symbol, from, to };
        let url_params = serde_url_params::to_string(&params)?;
        url.set_query(Some(&url_params));
        let data = reqwest::get(url.as_str()).await?
            .error_for_status()?
            .json()
            .await?;
        Ok(data)
    }

    pub async fn kline_data(&self, symbol: &str, resolution: u64, from: u64, to: u64) -> BTCTRResult<Kline>{
        let mut url = create_endpoint_url("/v1/klines/history", true)?;
        let params = KlineParams { symbol, from, to, resolution };
        let query_params = serde_url_params::to_string(&params)?;
        url.set_query(Some(&query_params));
        let data = reqwest::get(url.as_str()).await?
            .error_for_status()?
            .json()
            .await?;
        Ok(data)
    }
}

pub struct PrivateApi {
    config: Config,
    nonce: u64,
}

impl PrivateApi {

    pub fn new(config: Config) -> Self {
        Self {
            config,
            nonce: 12
        }
    }
   
    // Private api endpoints
    pub fn balances() {}
    pub fn all_transactions() {}
    pub fn fiat_transactions() {}
    pub fn crypto_transactions() {}
    pub fn open_orders() {}
    pub fn all_orders() {}
    pub fn single_order(order_id: u64) {}
    pub fn submit_order(
        quantity: f64, 
        price: f64, 
        stop_price: f64, 
        new_order_client_id: &str, 
        order_method: &str, 
        order_type: &str, 
        pair_symbol: &str
        ) {

    }
}


