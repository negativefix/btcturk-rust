use chrono::{self, Utc, DateTime};

// TODO - create types for responses
// TODO - add integrations tests for the public/private endpoints/
// TODO - configure .env to read apikey and secret from environment within integration tests
// TODO - check if utc nonces matches the server

// move api key and screet to env
const BASE_URL: &str = "https://api.btcturk.com";

mod types;
mod errors;

pub use errors::BTCTRResult;
use reqwest::Response;
use types::ExchangeInfo;

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

    pub fn pair(pair_symbol: &str) {}
    pub fn currency(symbol: &str) {}
    pub fn order_book(pair_symbol: &str, limit: Option<u32>) {}
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


#[cfg(test)]
mod tests {
    use crate::{Api, BASE_URL, BTCTRResult};

    #[tokio::test]
    async fn exchange_info() -> BTCTRResult<()> {
        let api = Api::new(BASE_URL, None);
        let _json = api.exchange_info().await?;
        Ok(())
    }
}
