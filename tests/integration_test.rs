use btcturk_rust::{BTCTRResult, Api, BASE_API_URL, GRAPH_API_URL};

#[tokio::test]
async fn exchange_info() -> BTCTRResult<()> {
    let api = Api::new(BASE_API_URL, None);
    let _json = api.exchange_info().await?;
    Ok(())
}

#[tokio::test]
async fn ticker_pair() -> BTCTRResult<()> {
    let api = Api::new(BASE_API_URL, None);
    let _json = api.pair(None).await?;
    Ok(())
}

#[tokio::test]
async fn ticker_currency() -> BTCTRResult<()> {
    let api = Api::new(BASE_API_URL, None);
    let _json = api.currency("BTCTRY").await?;
    Ok(())
}

#[tokio::test]
async fn order_book() -> BTCTRResult<()> {
    let api = Api::new(BASE_API_URL, None);
    let _json = api.order_book("BTCTRY", None).await?;
    Ok(())
}

#[tokio::test]
async fn trade() -> BTCTRResult<()> {
    let api = Api::new(BASE_API_URL, None);
    let _json = api.trades("BTCTRY", None).await?;
    Ok(())
}

#[tokio::test]
async fn kline_default() -> BTCTRResult<()> {
    let api = Api::new(BASE_API_URL, None);
    let _json = api.kline_data("BTCTRY", 80, 1602925320, 1603152000).await?;
    Ok(())
}

#[tokio::test]
async fn kline_invalid_resolution() -> BTCTRResult<()> {
    let api = Api::new(BASE_API_URL, None);
    let _json = api.kline_data("BTCTRY", 0, 1602925320, 1603152000).await?;
    Ok(())
}  

#[tokio::test]
async fn kline_invalid_pair() -> BTCTRResult<()> {
    let api = Api::new(BASE_API_URL, None);
    let _json = api.kline_data("brx22", 90, 1602925320, 1603152000).await?;
    Ok(())
}


#[tokio::test]
async fn kline_invalid_range() -> BTCTRResult<()> {
    let api = Api::new(BASE_API_URL, None);
    let _json = api.kline_data("BTCTRY", 90, 0, 0).await?;
    Ok(())
}  
