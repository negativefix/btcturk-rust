use chrono::{self, Utc, DateTime};

// TODO - create types for responses
// TODO - add integrations tests for the public/private endpoints/
// TODO - configure .env to read apikey and secret from environment within integration tests
// TODO - check if utc nonces matches the server

// move api key and screet to env
pub const BASE_URL: &str = "https://api.btcturk.com";

mod types;
mod errors;

pub use errors::BTCTRResult;
use reqwest::{Response, Url};
use types::{ExchangeInfo, Pair, OrderBook};

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

pub struct Api {
    base_url: &'static str,
    config: Option<Config>
}

impl Api {

    pub fn new(base_url: &'static str, config: Option<Config>) -> Self {
        Self {
            base_url,
            config,
        }
    }
    
    // Public api endpoints
    pub async fn exchange_info(&self) -> BTCTRResult<ExchangeInfo> {
        let response = self.fetch("/api/v2/server/exchangeinfo").await?;
        let json: ExchangeInfo = response.json().await?;
        Ok(json)
    }

    // no need to pass self since it wont be used within the function
    // TODO - move this into a module
    pub async fn pair(&self, pair_symbol: Option<&str>) -> BTCTRResult<Pair> {
        let endpoint = format!("{}{}", self.base_url, "/api/v2/ticker");
        let url = match pair_symbol {
            Some(pair) => Url::parse_with_params(&endpoint, &[("pairSymbol", pair)])?,
            None => Url::parse(&endpoint)?,
        };
        let json = reqwest::get(url.as_str()).await?.json().await?;
        Ok(json)
    }

    pub async fn currency(&self, symbol: &str) -> BTCTRResult<Pair>{
        let endpoint = format!("{}{}", self.base_url, "/api/v2/ticker/currency");
        let url = Url::parse_with_params(&endpoint, &[("symbol", symbol)])?;
        let json = reqwest::get(url.as_str()).await?.json().await?;
        Ok(json)
    }
    
    pub async fn order_book(&self, pair_symbol: &str, limit: Option<u32>) -> BTCTRResult<OrderBook> {
        let endpoint = format!("{}{}", self.base_url, "/api/v2/orderbook");
        let pair = &[("pairSymbol", pair_symbol)];
        let mut url = Url::parse_with_params(&endpoint, pair)?;
        if let Some(l) = limit {
            url.query_pairs_mut().append_pair("limit", &l.to_string());
        }
        let json = reqwest::get(url.as_str()).await?.json().await?;
        Ok(json)
    }

    pub fn trades(pair_symbol: &str, last: Option<u32>) {}
    pub fn ohlc_data(pair_symbol: &str, from: u64, to: u64) {}
    pub fn kline_data(pair_symbol: &str, resolution: u64, from: u64, to: u64) {}

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
        ) {}

    pub async fn fetch(&self, endpoint: &str) -> BTCTRResult<Response> {
        let url = format!("{}{}", self.base_url, endpoint);
        let res = reqwest::get(&url).await?;
        println!("{}", url);
        Ok(res)
    }
}


