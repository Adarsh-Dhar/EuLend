use cosmwasm_std::{
    entry_point, to_json_binary,from_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, Addr, StdError
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdatePrice { token_address: Addr, new_price: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPrice { token_address: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    pub price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub prices: Vec<(Addr, Uint128)>,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State { prices: vec![] };
    deps.storage.set(b"state", &to_json_binary(&state)?);
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdatePrice { token_address, new_price } => {
            update_price(deps, info, token_address, new_price)
        }
    }
}

pub fn update_price(
    deps: DepsMut,
    _info: MessageInfo,
    token_address: Addr,
    new_price: Uint128,
) -> StdResult<Response> {
    let state_bytes = deps.storage.get(b"state").ok_or(StdError::not_found("State"))?;

    // Convert Vec<u8> to Binary
    let state_binary = Binary(state_bytes);

    // Deserialize the state from Binary
    let mut state: State = from_binary(&state_binary)?;
    if let Some(price) = state.prices.iter_mut().find(|(addr, _)| addr == &token_address) {
        price.1 = new_price;
    } else {
        state.prices.push((token_address, new_price));
    }
    deps.storage.set(b"state", &to_json_binary(&state)?);
    Ok(Response::new().add_attribute("action", "update_price"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice { token_address } => to_json_binary(&query_price(deps, token_address)?),
    }
}

fn query_price(deps: Deps, token_address: Addr) -> StdResult<PriceResponse> {
    let state_bytes = deps.storage.get(b"state").ok_or(StdError::not_found("State"))?;

    // Convert Vec<u8> to Binary
    let state_binary = Binary(state_bytes);

    // Deserialize the state from Binary
    let mut state: State = from_binary(&state_binary)?;
    let price = state
        .prices
        .iter()
        .find(|(addr, _)| addr == &token_address)
        .map(|(_, price)| *price)
        .unwrap_or(Uint128::zero());
    Ok(PriceResponse { price })
}