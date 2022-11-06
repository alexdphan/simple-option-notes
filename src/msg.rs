use crate::state::State;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    // owner and creator come from env (whoever signed it)
    // collateral comes from env (how much was sent)
    pub counter_offer: Vec<Coin>,
    pub expires: u64,
    // We are saying the counter_offer is a Vec<Coin> and expires is a u64
}

#[cw_serde]
pub enum ExecuteMsg {
    // What are the possible things you can handle?
    // Owner can post counter_offer on unexpired option to execute and get the collateral
    /// Owner can transfer to a new owner
    Transfer { recipient: String }, // we say the recipient is a strong, which is an address
    /// Owner can post counter_offer on unexpired option to execute and get the collateral
    Execute {}, // We can execute the option which takes nothing. You must pay an amount to it to
    /// Burn will release collateral if expired
    Burn {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {}, // get state, tells us the configurations
}

// We define a custom struct for each query response
// ConfigResponse is just the state
pub type ConfigResponse = State;
