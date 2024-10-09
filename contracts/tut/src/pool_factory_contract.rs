use cosmwasm_std::{
    entry_point, to_json_binary,from_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Addr, CosmosMsg, WasmMsg,StdError
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub lending_pool_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateLendingPool { params: LendingPoolParams },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetLendingPools {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LendingPoolParams {
    pub asset_token: Addr,
    pub interest_rate_model: InterestRateModel,
    pub reserve_factor: u64,
    pub liquidation_threshold: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InterestRateModel {
    pub base_rate: u64,
    pub slope1: u64,
    pub slope2: u64,
    pub optimal_utilization: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LendingPoolsResponse {
    pub lending_pools: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub lending_pools: Vec<Addr>,
    pub lending_pool_code_id: u64,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        lending_pools: vec![],
        lending_pool_code_id: msg.lending_pool_code_id,
    };
    deps.storage.set(b"state", &to_json_binary(&state)?);
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateLendingPool { params } => create_lending_pool(deps, env, info, params),
    }
}

pub fn create_lending_pool(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    params: LendingPoolParams,
) -> StdResult<Response> {
    // Load the state from storage and handle if it does not exist
    let state_bytes = deps.storage.get(b"state").ok_or(StdError::not_found("State"))?;

    // Convert Vec<u8> to Binary
    let state_binary = Binary(state_bytes);

    // Deserialize the state from Binary
    let mut state: State = from_binary(&state_binary)?;

    // Create the message to instantiate a new lending pool
    let msg = CosmosMsg::Wasm(WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id: state.lending_pool_code_id,
        msg: to_json_binary(&params)?,
        funds: vec![],
        label: "whatever".to_string(),
    });

    // Return the response with the message included
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "create_lending_pool")
        .add_attribute("creator", _info.sender.to_string())) // Include any attributes you want to add
}