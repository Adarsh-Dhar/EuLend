use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {
        token_address: String,
        amount: i32,
    },
    Withdraw {
        token_address: String,
        amount: i32,
    },
    Borrow {
        token_address: String,
        amount: i32,
        collateral_token_address: String,
        collateral_amount: i32,
    },
    Repay {
        token_address: String,
        amount: i32,
    },
    Liquidate {
        borrower: String,
        debt_token: String,
        collateral_token: String,
    },
    Transfer {
        to_address: String,
        amount: i32,
    },
    
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetBalanceResponse)]
    GetBalance {},
}

// We define a custom struct for each query response

#[cw_serde]
pub struct GetBalanceResponse {
    pub balance: i32,
}
