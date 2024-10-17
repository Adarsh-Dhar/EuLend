use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128};


#[cw_serde]
pub struct InstantiateMsg {}


#[cw_serde]
pub enum ExecuteMsg {
    Deposit {
        token_address: String,
        amount: Uint128,
    },
    Withdraw {
        token_address: String,
        amount: Uint128,
    },
    Borrow {
        borrow_token_address: String,
        amount: Uint128,
        collateral_token_address: String,
    },
    Repay {
        token_address: String,
        amount: Uint128,
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
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: Uint128,
}
