use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, WasmMsg,
};
use cw_storage_plus::{Item};
use serde::{Deserialize, Serialize};

// Euclid Protocol SDK Messages for addLiquidity
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EuclidAddLiquidityMsg {
    pub token1: String,
    pub token2: String,
    pub amount1: Uint128,
    pub amount2: Uint128,
}

// Storage structure for the Lending Pool
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PoolState {
    pub owner: Addr,
    pub total_deposits: Uint128,
    pub total_borrowed: Uint128,
    pub interest_rate: Uint128,
}

// Storage structure for individual user positions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserPosition {
    pub owner: Addr,
    pub deposit: Uint128,
    pub debt: Uint128,
}

// Instantiate message for contract initialization
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub interest_rate: Uint128, // e.g., 5% interest
}

// Deposit message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DepositMsg {
    pub token_address: String,
    pub amount: Uint128,
    pub liquidity_provision: bool,
    pub pair_token_address: Option<String>,
    pub pair_amount: Option<Uint128>,
}

// Borrow message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BorrowMsg {
    pub amount: Uint128,
}

// Repay message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RepayMsg {
    pub amount: Uint128,
}

// Storage items for Pool State and User Positions
const POOL_STATE: Item<PoolState> = Item::new("pool_state");
const USER_POSITIONS: Item<UserPosition> = Item::new("user_positions");

// Instantiate function to set up the pool and interest rate
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = PoolState {
        owner: info.sender.clone(),
        total_deposits: Uint128::zero(),
        total_borrowed: Uint128::zero(),
        interest_rate: msg.interest_rate,
    };
    POOL_STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

// Execute function to handle deposits, borrowing, and repayments
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: DepositMsg,
) -> Result<Response, ContractError> {
    let mut state = POOL_STATE.load(deps.storage)?;
    state.total_deposits += msg.amount;
    POOL_STATE.save(deps.storage, &state)?;

    // If liquidity_provision is enabled, we execute add_liquidity function
    if msg.liquidity_provision {
        if let (Some(pair_token), Some(pair_amount)) = (msg.pair_token_address, msg.pair_amount) {
            return add_liquidity(deps, info, msg.token_address, pair_token, msg.amount, pair_amount);
        } else {
            return Err(ContractError::InvalidPairData {});
        }
    }

    // Return a successful deposit response if no liquidity provision
    Ok(Response::new()
        .add_attribute("action", "deposit")
        .add_attribute("amount", msg.amount.to_string()))
}

// Function to handle liquidity provision via Euclid Protocol
pub fn add_liquidity(
    deps: DepsMut,
    info: MessageInfo,
    token1: String,
    token2: String,
    amount1: Uint128,
    amount2: Uint128,
) -> Result<Response, ContractError> {
    // Preparing the Euclid add_liquidity message
    let euclid_msg = EuclidAddLiquidityMsg {
        token1,
        token2,
        amount1,
        amount2,
    };

    let wasm_msg = WasmMsg::Execute {
        contract_addr: "euclid_protocol_contract_address".to_string(), // Replace with actual Euclid contract address
        msg: to_json_binary(&euclid_msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("action", "add_liquidity")
        .add_attribute("from", info.sender)
        .add_attribute("amount1", amount1.to_string())
        .add_attribute("amount2", amount2.to_string()))
}

// Borrow function
pub fn borrow(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: BorrowMsg,
) -> Result<Response, ContractError> {
    let mut state = POOL_STATE.load(deps.storage)?;
    let mut user_position = USER_POSITIONS
        .may_load(deps.storage)?
        .unwrap_or(UserPosition {
            owner: info.sender.clone(),
            deposit: Uint128::zero(),
            debt: Uint128::zero(),
        });

    // Ensure there's enough liquidity for the requested borrow amount
    if msg.amount > state.total_deposits {
        return Err(ContractError::InsufficientLiquidity {});
    }

    // Update user's debt and pool state
    user_position.debt += msg.amount;
    state.total_borrowed += msg.amount;
    state.total_deposits -= msg.amount;

    POOL_STATE.save(deps.storage, &state)?;
    USER_POSITIONS.save(deps.storage, &user_position)?;

    Ok(Response::new()
        .add_attribute("action", "borrow")
        .add_attribute("borrowed_amount", msg.amount.to_string()))
}

// Repay function
pub fn repay(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: RepayMsg,
) -> Result<Response, ContractError> {
    let mut state = POOL_STATE.load(deps.storage)?;
    let mut user_position = USER_POSITIONS
        .may_load(deps.storage)?
        .unwrap_or(UserPosition {
            owner: info.sender.clone(),
            deposit: Uint128::zero(),
            debt: Uint128::zero(),
        });

    // Ensure the user has enough debt to repay
    if msg.amount > user_position.debt {
        return Err(ContractError::Overpayment {});
    }

    // Update the user's debt and pool state
    user_position.debt -= msg.amount;
    state.total_borrowed -= msg.amount;
    state.total_deposits += msg.amount;

    POOL_STATE.save(deps.storage, &state)?;
    USER_POSITIONS.save(deps.storage, &user_position)?;

    Ok(Response::new()
        .add_attribute("action", "repay")
        .add_attribute("repaid_amount", msg.amount.to_string()))
}

// Query function to get pool state
pub fn query_pool_state(deps: Deps) -> StdResult<Binary> {
    let state = POOL_STATE.load(deps.storage)?;
    to_json_binary(&state)
}

// Error handling
#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("Invalid Pair Data")]
    InvalidPairData {},

    #[error("Insufficient Liquidity")]
    InsufficientLiquidity {},

    #[error("Overpayment Error")]
    Overpayment {},
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Uint128};

    #[test]
    fn test_deposit_without_liquidity() {
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &[]);
        let msg = InstantiateMsg {
            interest_rate: Uint128::new(500),
        };
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(res.attributes[0].value, "instantiate");

        let deposit_msg = DepositMsg {
            token_address: "token_address".to_string(),
            amount: Uint128::new(1000),
            liquidity_provision: false,
            pair_token_address: None,
            pair_amount: None,
        };

        let res = execute(deps.as_mut(), mock_env(), info, deposit_msg).unwrap();
        assert_eq!(res.attributes[0].value, "deposit");
    }

    #[test]
    fn test_deposit_with_liquidity() {
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &[]);
        let msg = InstantiateMsg {
            interest_rate: Uint128::new(500),
        };
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(res.attributes[0].value, "instantiate");

        let deposit_msg = DepositMsg {
            token_address: "token_address".to_string(),
            amount: Uint128::new(1000),
            liquidity_provision: true,
            pair_token_address: Some("pair_token_address".to_string()),
            pair_amount: Some(Uint128::new(500)),
        };

        let res = execute(deps.as_mut(), mock_env(), info, deposit_msg).unwrap();
        assert_eq!(res.attributes[0].value, "add_liquidity");
    }
}
