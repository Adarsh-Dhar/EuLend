use cosmwasm_schema::{cw_serde, QueryResponses};


#[cw_serde]
pub struct InstantiateMsg {}


#[cw_serde]
pub enum ExecuteMsg {
    Deposit {
        token_address: String,
        amount: f64,
    },
    Withdraw {
        token_address: String,
        amount: f64,
    },
    Borrow {
        borrow_token_address: String,
        amount: f64,
        collateral_token_address: String,
    },
    Repay {
        token_address: String,
        amount: f64,
    },
    // Liquidate {
    //     borrower: String,
    //     debt_token: String,
    //     collateral_token: String,
    // },
    // Transfer {
    //     to_address: String,
    //     amount: f64,
    // },
    // AddLiquidity {
    //     token1: String,
    //     token2: String,
    //     amount1: f64,
    //     amount2: f64,
    // },
    // CreateLendingPool { 
    //     params: LendingPoolParams 
    // },
    // UpdatePrice { 
    //     token_address: String, 
    //     new_price: f64
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
    pub count: f64,
}
