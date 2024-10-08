use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
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
}

// Storage items for Pool State
const POOL_STATE: Item<PoolState> = Item::new("pool_state");

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = PoolState {
        owner: info.sender.clone(),
        total_deposits: Uint128::zero(),
    };
    POOL_STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
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

// Execute message to handle deposits
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: DepositMsg,
) -> Result<Response, ContractError> {
    // Handle token deposits here, simple storage update.
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
        msg: to_binary(&euclid_msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("action", "add_liquidity")
        .add_attribute("from", info.sender)
        .add_attribute("amount1", amount1.to_string())
        .add_attribute("amount2", amount2.to_string()))
}

// Query function to get pool state
pub fn query_pool_state(deps: Deps) -> StdResult<Binary> {
    let state = POOL_STATE.load(deps.storage)?;
    to_binary(&state)
}

// Error handling
#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("Invalid Pair Data")]
    InvalidPairData {},
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
        let msg = InstantiateMsg {};
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
        let msg = InstantiateMsg {};
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
