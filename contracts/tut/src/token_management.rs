use cosmwasm_std::{Deps, StdResult, to_binary, Binary};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub token: String,
    pub chain_uid: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ChainInfo {
    pub chain_uid: String,
    pub balance: String,
    pub chain_id: String,
}

// Fetch available tokens
pub fn query_all_tokens(deps: Deps) -> StdResult<Binary> {
    let tokens: Vec<TokenInfo> = vec![
        TokenInfo { token: "eth".to_string(), chain_uid: "ethereum".to_string() },
        TokenInfo { token: "sol".to_string(), chain_uid: "osmosis".to_string() },
        TokenInfo { token: "atom".to_string(), chain_uid: "nibiru".to_string() },
    ];

    to_binary(&tokens)
}

// Fetch escrows for token_in
pub fn query_escrows(deps: Deps, token: String) -> StdResult<Binary> {
    let escrows: Vec<ChainInfo> = vec![
        ChainInfo {
            chain_uid: "ethereum".to_string(),
            balance: "10056388656303".to_string(),
            chain_id: "localwasma-1".to_string(),
        },
    ];

    to_binary(&escrows)
}
