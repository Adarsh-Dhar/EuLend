use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Binary, Coin};
use std::time::Duration;



#[cw_serde]
pub enum InstantiateMsg {}


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





