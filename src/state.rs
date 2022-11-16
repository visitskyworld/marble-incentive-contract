use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Owner If None set, contract is frozen.
    pub owner: Option<Addr>,
    pub reward_token_address: Addr,
    pub stake_token_address: Addr,
    pub reward_amount: Uint128,
    pub stake_amount: Uint128,
    pub daily_reward_amount: Uint128,
    pub apy_prefix: Uint128,
    pub reward_interval: u64
    
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const STAKERS_KEY: &str = "stakers";
pub const STAKERS: Map<Addr, (Uint128, Uint128, u64)> = Map::new(STAKERS_KEY);
