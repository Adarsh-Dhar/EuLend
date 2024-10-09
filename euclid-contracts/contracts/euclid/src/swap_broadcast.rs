use cosmwasm_std::{DepsMut, Env, Response, StdError};

// Broadcast the transaction
pub fn broadcast_swap(deps: DepsMut, _env: Env, tx_msg: CosmosMsg) -> Result<Response, StdError> {
    Ok(Response::new()
        .add_message(tx_msg)
        .add_attribute("method", "broadcast_swap")
    )
}
