use btcturk_rust::{BTCTRResult, Api, BASE_URL};

#[tokio::test]
async fn exchange_info() -> BTCTRResult<()> {
    let api = Api::new(BASE_URL, None);
    let _json = api.exchange_info().await?;
    Ok(())
}

#[tokio::test]
async fn ticker_pair() -> BTCTRResult<()> {
    let api = Api::new(BASE_URL, None);
    let _json = api.pair(None).await?;
    Ok(())
}

#[tokio::test]
async fn ticker_currency() -> BTCTRResult<()> {
    let api = Api::new(BASE_URL, None);
    let _json = api.currency("BTCTRY").await?;
    Ok(())
}
