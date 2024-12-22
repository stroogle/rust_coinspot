use reqwest::StatusCode;

use crate::v2::{
    PublicUtils,
    types::{
        CoinSpotBadResponse,
        CoinSpotResponse,
        CoinSpotResult,
        CompletedOrdersSummary
    }
};

impl PublicUtils {

    /// Used to get the latest buy price of a specific coin.
    /// CoinSpot's API also throws a 400 error for invalid markets.
    /// This 400 error will return a CoinSpotResponse::Bad response
    pub async fn completed_orders_coin_summary(coin_symbol: &str) -> CoinSpotResult<CompletedOrdersSummary>{
        let url = format!("https://www.coinspot.com.au/pubapi/v2/orders/summary/completed/{}", coin_symbol);
        
        let res = reqwest::get(
            &url
        ).await?;

        match res.status() {
            StatusCode::OK => {
                let text = res.text().await?;
                
                let json: CompletedOrdersSummary = serde_json::from_str(&text)?;
                return Ok(
                    CoinSpotResponse::Ok(json)
                )
            },
            StatusCode::BAD_REQUEST => {
                let text = res.text().await?;
                
                let json: CoinSpotBadResponse = serde_json::from_str(&text)?;
                return Ok(
                    CoinSpotResponse::Bad(json)
                )
            }
            _ => {
                return Err(format!("CoinSpot API never expects status: {:?}", res.status()).into())
            }
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_completed_orders_coin_summary() {
    
        let result: CoinSpotResponse<CompletedOrdersSummary>;
        result = PublicUtils::completed_orders_coin_summary("btc")
        .await
        .unwrap();

        match result {
            CoinSpotResponse::Ok(res) => {
                assert_eq!(res.status, "ok");
                assert_eq!(res.message, "ok");
                assert_eq!(res.orders[0].coin, "BTC");
            },
            _ => {}
        }    
    }

    #[tokio::test]
    async fn test_completed_orders_fake_coin_summary() {
        let result2 = PublicUtils::completed_orders_coin_summary("sdfsdf")
        .await
        .unwrap();

        match result2 {
            CoinSpotResponse::Bad(res) => {
                assert_eq!(res.status, "error");
                assert_eq!(res.message, "Coin not found");
            },
            _ => assert!(false)
        }
    }
}