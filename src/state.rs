use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

#[cw_serde]
pub struct State {
    pub creator: Addr,
    pub owner: Addr,
    pub collateral: Vec<Coin>, // array/set of coins
    pub counter_offer: Vec<Coin>, // array/set of coins
    // contract takes native coins, not cw20s so its good with IBC 
    pub expires: u64, // block height
} 
// for storage, we typically use the canonical address (over HumanAddr, check docs incase this changes)
    // they are shorter, which saves data space
    // they will last over a fork

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<State> = Item::new(CONFIG_KEY);


// Old boilerplate from the tutorial
// It would create singletons that return state
// You query, get state object back, and parse it properly
// pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
//   singleton(storage, CONFIG_KEY)
// }

// pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
//   singleton_read(storage, CONFIG_KEY)
// }

