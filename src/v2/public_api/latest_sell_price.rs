use crate::v2::{
    CoinSpotPublic,
    PublicUtils,
    types::{
        CoinSpotResult,
        LatestActionPrice,
        Market
    }
};

impl CoinSpotPublic {

    pub async fn latest_sell_price(market: Market<'_>) -> CoinSpotResult<LatestActionPrice> {
        match market {
            Market::Coin(coin_symbol) => {
                PublicUtils::latest_sell_price(coin_symbol)
                .await
            },
            Market::TradePair(coin_symbol, market) => {
                PublicUtils::latest_sell_price_market(coin_symbol, market)
                .await
            }
        }
    }

}