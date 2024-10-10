use cosmwasm_std::{Addr, Uint128};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime; // Used for async execution in non-async contexts

// Token price API request
#[derive(Serialize, Deserialize, Debug)]
pub struct PriceResponse {
    pub token: String,
    pub price: Uint128,
}

// User position API request
#[derive(Serialize, Deserialize, Debug)]
pub struct UserPositionResponse {
    pub address: Addr,
    pub debt: Uint128,
    pub collateral: Uint128,
    pub health_factor: f64,
}

// Liquidation Request
#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidationRequest {
    pub user_address: Addr,
    pub debt: Uint128,
    pub collateral: Uint128,
}

// Fetch token prices
pub async fn fetch_token_price(token: &str) -> Result<PriceResponse, Error> {
    let url = format!("https://api.lending.com/prices/{}", token);
    let response = reqwest::get(&url).await?.json::<PriceResponse>().await?;
    Ok(response)
}

// Fetch user positions
pub async fn fetch_user_position(user_address: &str) -> Result<UserPositionResponse, Error> {
    let url = format!("https://api.lending.com/user/{}/positions", user_address);
    let response = reqwest::get(&url).await?.json::<UserPositionResponse>().await?;
    Ok(response)
}

// Trigger liquidation
pub async fn trigger_liquidation(liquidation_req: LiquidationRequest) -> Result<(), Error> {
    let url = "https://api.lending.com/liquidation";
    let client = Client::new();
    let response = client.post(url)
        .json(&liquidation_req)
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Liquidation triggered for {}", liquidation_req.user_address);
    } else {
        println!("Failed to trigger liquidation");
    }
    
    Ok(())
}

// Syncing external data to internal logic
pub fn sync_data(token: &str, user_address: &str) -> Result<(), Error> {
    let rt = Runtime::new().unwrap();

    // Fetch token price
    let price = rt.block_on(fetch_token_price(token)).unwrap();
    println!("Fetched token price: {:?}", price);

    // Fetch user position
    let user_position = rt.block_on(fetch_user_position(user_address)).unwrap();
    println!("Fetched user position: {:?}", user_position);

    // Check if liquidation is necessary based on health factor
    if user_position.health_factor < 1.0 {
        let liquidation_req = LiquidationRequest {
            user_address: user_position.address.clone(),
            debt: user_position.debt,
            collateral: user_position.collateral,
        };

        // Trigger liquidation
        rt.block_on(trigger_liquidation(liquidation_req)).unwrap();
    } else {
        println!("No liquidation required for user {}", user_address);
    }

    Ok(())
}

fn main() {
    // Example usage for token price and user address
    sync_data("tokenA", "cosmos1useraddress123").unwrap();
}
