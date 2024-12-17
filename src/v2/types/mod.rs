use serde::{Deserialize, Serialize};
use std::error::Error;

/// CoinSpot's api documentation says that any bad response will fall into this format.
#[derive(Serialize, Deserialize, Debug)]
pub struct CoinSpotBadResponse {
    pub status: String,
    pub message: String,
}

/// This enum encompasses the expected responses from the CoinSpot api. It uses a generic type enable easier pattern matching.
pub enum CoinSpotResponse<T> {
    Ok(T),
    Bad(CoinSpotBadResponse)
}

/// This is an alias of Rust's Result type, the generic passed in will map to the Ok type in CoinSpotResponse.
pub type CoinSpotResult<T> = Result<CoinSpotResponse<T>, Box<dyn Error>>;

#[derive(Deserialize, Serialize, Debug)]
pub struct LatestActionPrice {
    pub status: String,
    pub message: String,
    pub rate: String,
    pub market: String
}