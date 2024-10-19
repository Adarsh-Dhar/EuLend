use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Binary, Coin};
use std::time::Duration;
use pyth_sdk_cw::{
    Price,
    PriceIdentifier,
};


#[cw_serde]
pub struct InstantiateMsg {
    pub price_feed_id:      PriceIdentifier,
    pub pyth_contract_addr: String,
}


#[cw_serde]
pub enum ExecuteMsg {
    Deposit {
        token_address: String,
        amount: Uint128,
        account_id: Uint128,
    },
    Withdraw {
        token_address: String,
        amount: Uint128,
        account_id: Uint128,

    },
    Borrow {
        borrow_token_address: String,
        amount: Uint128,
        collateral_token_address: String,
        account_id: Uint128,

    },
    Repay {
        token_address: String,
        amount: Uint128,
        account_id: Uint128,

    },
    // Liquidate {
    //     borrower: String,
    //     debt_token: String,
    //     collateral_token: String,
    // },
    // Transfer {
    //     to_address: String,
    //     amount: Uint128,
    // },
    // AddLiquidity {
    //     token1: String,
    //     token2: String,
    //     amount1: Uint128,
    //     amount2: Uint128,
    // },
    // CreateLendingPool { 
    //     params: LendingPoolParams 
    // },
    // UpdatePrice { 
    //     token_address: String, 
    //     new_price: Uint128
    // },
}



#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(FetchPriceResponse)]
    FetchPrice {},
    #[returns(Coin)]
    FetchUpdateFee { vaas: Vec<Binary> },
    #[returns(Duration)]
    FetchValidTimePeriod,
}

#[cw_serde]
pub struct FetchPriceResponse {
    pub current_price: Price,
    pub ema_price:     Price,
}