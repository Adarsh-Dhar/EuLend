use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, Addr, CosmosMsg, WasmMsg, BankMsg, coin,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw20_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Wrap { token_address: Addr },
    Unwrap { wrapped_token_address: Addr },
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetWrappedToken { token_address: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WrappedTokenResponse {
    pub wrapped_token: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub wrapped_tokens: Vec<(Addr, Addr)>,
    pub cw20_code_id: u64,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        wrapped_tokens: vec![],
        cw20_code_id: msg.cw20_code_id,
    };
    deps.storage.set(b"state", &to_binary(&state)?);
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
        ExecuteMsg::Wrap { token_address } => wrap(deps, env, info, token_address),
        ExecuteMsg::Unwrap { wrapped_token_address } => unwrap(deps, env, info, wrapped_token_address),
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
    }
}

pub fn wrap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_address: Addr,
) -> StdResult<Response> {
    let mut state: State = deps.storage.get(b"state").unwrap().unwrap();
    
    if let Some((_, wrapped_token)) = state.wrapped_tokens.iter().find(|(addr, _)| addr == &token_address) {
        // Token is already wrapped, mint additional wrapped tokens
        let amount = info.funds.iter().find(|c| c.denom == token_address.to_string()).map(|c| c.amount).unwrap_or(Uint128::zero());
        let msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: wrapped_token.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Mint {
                recipient: info.sender.to_string(),
                amount,
            })?,
            funds: vec![],
        });
        Ok(Response::new().add_message(msg))
    } else {
        // Create new wrapped token
        let msg = CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: Some(env.contract.address.to_string()),
            code_id: state.cw20_code_id,
            msg: to_binary(&cw20::InstantiateMsg {
                name: format!("Wrapped {}", token_address),
                symbol: format!("w{}", token_address),
                decimals: 6,
                initial_balances: vec![],
                mint: Some(cw20::MinterResponse {
                    minter: env.contract.address.to_string(),
                    cap: None,
                }),
            })?,
            funds: vec![],
            label: format!("Wrapped {}", token_address),
        });
        state.wrapped_tokens.push((token_address, Addr::unchecked("")));
        deps.storage.set(b"state", &to_binary(&state)?);
        Ok(Response::new().add_message(msg))
    }
}

pub fn unwrap(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    wrapped_token_address: Addr,
) -> StdResult<Response> {
    let state: State = deps.storage.get(b"state").unwrap().unwrap();
    if let Some((token_address, _)) = state.wrapped_tokens.iter().find(|(_, addr)| addr == &wrapped_token_address) {
        let amount = info.funds.iter().find(|c| c.denom == wrapped_token_address.to_string()).map(|c| c.amount).unwrap_or(Uint128::zero());
        let msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![coin(amount.u128(), token_address.to_string())],
        });
        Ok(Response::new().add_message(msg))
    } else {
        Err(StdError::generic_err("Invalid wrapped token address"))
    }
}

pub fn receive_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let state: State = deps.storage.get(b"state").unwrap().unwrap();
    if let Some((token_address, _)) = state.wrapped_tokens.iter().find(|(_, addr)| addr == &info.sender) {
        let msg = CosmosMsg::Bank(BankMsg::Send {
            to_address: cw20_msg.sender,
            amount: vec![coin(cw20_msg.amount.u128(), token_address.to_string())],
        });
        Ok(Response::new().add_message(msg))
    } else {
        Err(StdError::generic_err("Invalid wrapped token"))
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetWrappedToken { token_address } => to_binary(&query_wrapped_token(deps, token_address)?),
    }
}

fn query_wrapped_token(deps: Deps, token_address: Addr) -> StdResult<WrappedTokenResponse> {
    let state: State = deps.storage.get(b"state").unwrap().unwrap();
    let wrapped_token = state
        .wrapped_tokens
        .iter()
        .find(|(addr, _)| addr == &token_address)
        .map(|(_, wrapped)| wrapped.clone())
        .ok_or_else(|| StdError::generic_err("Token not wrapped"))?;
    Ok(WrappedTokenResponse { wrapped_token })
}