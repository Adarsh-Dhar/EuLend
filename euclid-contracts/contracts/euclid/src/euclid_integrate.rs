use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
struct VlpInfo {
    vlp: String,
    token_1: String,
    token_2: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PoolInfo {
    chain_uid: String,
    pool: Pool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pool {
    reserve_1: String,
    reserve_2: String,
    lp_shares: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EscrowInfo {
    escrow_address: String,
    denoms: Vec<TokenType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum TokenType {
    Smart { smart: SmartToken },
    Native { native: NativeToken },
}

#[derive(Serialize, Deserialize, Debug)]
struct SmartToken {
    contract_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NativeToken {
    denom: String,
}

pub struct EuclidSdk {
    client: Client,
    base_url: String,
}

impl EuclidSdk {
    pub fn new(base_url: &str) -> Self {
        EuclidSdk {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_all_vlps(&self) -> Result<Vec<VlpInfo>> {
        let response: serde_json::Value = self.client
            .get(&format!("{}/graphql", self.base_url))
            .json(&serde_json::json!({
                "query": "query All_vlps { router { all_vlps { vlps { vlp token_1 token_2 } } } }"
            }))
            .send()
            .await?
            .json()
            .await?;

        let vlps = response["data"]["router"]["all_vlps"]["vlps"]
            .as_array()
            .unwrap()
            .iter()
            .map(|vlp| serde_json::from_value(vlp.clone()).unwrap())
            .collect();

        Ok(vlps)
    }

    pub async fn get_all_pools(&self, contract: &str) -> Result<Vec<PoolInfo>> {
        let response: serde_json::Value = self.client
            .get(&format!("{}/graphql", self.base_url))
            .json(&serde_json::json!({
                "query": "query Vlp($contract: String!) { vlp(contract: $contract) { all_pools { pools { chain_uid pool { reserve_1 reserve_2 lp_shares } } } } }",
                "variables": { "contract": contract }
            }))
            .send()
            .await?
            .json()
            .await?;

        let pools = response["data"]["vlp"]["all_pools"]["pools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|pool| serde_json::from_value(pool.clone()).unwrap())
            .collect();

        Ok(pools)
    }

    pub async fn get_escrow_info(&self, chain_uid: &str, token_id: &str) -> Result<EscrowInfo> {
        let response: serde_json::Value = self.client
            .get(&format!("{}/graphql", self.base_url))
            .json(&serde_json::json!({
                "query": "query Escrow($chainUid: String!, $tokenId: String) { factory(chain_uid: $chainUid) { escrow(token_id: $tokenId) { escrow_address denoms { ... on SmartTokenType { smart { contract_address } } ... on NativeTokenType { native { denom } } } } } }",
                "variables": { "chainUid": chain_uid, "tokenId": token_id }
            }))
            .send()
            .await?
            .json()
            .await?;

        let escrow_info = serde_json::from_value(response["data"]["factory"]["escrow"].clone())?;

        Ok(escrow_info)
    }

    pub async fn add_liquidity(&self, params: AddLiquidityParams) -> Result<String> {
        let response: serde_json::Value = self.client
            .post(&format!("{}/execute/liquidity/add", self.base_url))
            .json(&params)
            .send()
            .await?
            .json()
            .await?;

        let tx_hash = response["txhash"].as_str().unwrap().to_string();
        Ok(tx_hash)
    }
}

#[derive(Serialize)]
pub struct AddLiquidityParams {
    pub token_1_liquidity: String,
    pub token_2_liquidity: String,
    pub slippage_tolerance: u64,
    pub pair_info: PairInfo,
    pub sender: SenderInfo,
}

#[derive(Serialize)]
pub struct PairInfo {
    pub token_1: String,
    pub token_2: String,
    pub vlp_address: String,
}

#[derive(Serialize)]
pub struct SenderInfo {
    pub address: String,
    pub chain_uid: String,
}