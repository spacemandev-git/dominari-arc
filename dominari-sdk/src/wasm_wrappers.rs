use serde::{Serialize, Deserialize};

/**
 * Wrapper around dominari::state::GameConfig
 * Because GameConfig consumes Pubkeys, we need to first 
 * fetch the string values from the JS, then convert them
 * to Pubkey objects.
 */
#[derive(Deserialize)]
pub struct GameConfigFile {
    pub max_players:u16,
    pub starting_cards: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct WasmTile {
    pub x: u8,
    pub y: u8,
    pub feature: Option<WasmFeature>,
    pub troop: Option<WasmTroop>
}

#[derive(Serialize, Deserialize)]
pub struct WasmFeature {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmTroop {
    pub name: String,
    pub id: String
}

