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

#[derive(Serialize, Deserialize)]
pub struct WasmPlayer {
    pub name: String,
    pub image: String,
    pub score: String, //u64 as String
    pub kills: String, //u64 as String
    //Blueprint Names rather than Pubkey
    pub cards: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct ComponentDropTableWASM {
    pub drop_table: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct ComponentMetadataWASM {
    pub name: String,
    pub entity_type: String,
}