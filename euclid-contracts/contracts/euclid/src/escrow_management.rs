use cosmwasm_std::{Deps, StdResult, to_binary, Binary, Addr};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Escrow {
    pub escrow_address: Addr,
    pub denoms: Vec<String>, // allowed denoms
}

// Query escrow details and allowed denoms
pub fn query_escrow(deps: Deps, chain_uid: String, token_id: String) -> StdResult<Binary> {
    let escrow = Escrow {
        escrow_address: Addr::unchecked("wasm1hdgaz7707rwfsnm8clj440d4qzj88czvu9fqyv3m8v0z4vkw08fqz98ena"),
        denoms: vec!["uusdta".to_string()],
    };

    to_binary(&escrow)
}
