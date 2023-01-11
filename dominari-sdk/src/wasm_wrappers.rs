use dominari::component::TroopClass;
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
    pub troop: Option<WasmTroop>,
}

#[derive(Serialize, Deserialize)]
pub struct WasmFeature {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmTroop {
    // Metadata
    pub name: String,
    pub id: String,
    // Owner
    pub troop_owner_player_id: String, //u64 as string,
    pub troop_owner_player_key: String,
    // Damage
    pub min_damage: String, //u64
    pub max_damge: String, //u64
    pub bonus_infantry: String, //u32
    pub bonus_armor: String, //u32
    pub bonus_aircraft: String, //u32
    pub bonus_feature: String, //u32
    // Health 
    pub health: String, //u64
    // Troop Class
    pub class: TroopClass, // Enum
    // Range
    pub movement: u8,
    pub attack_range: u8,
    // Last Used
    pub last_used: String, //u64
    pub recovery: String, //u64
    // Value
    pub value: String, //u64
}

#[derive(Serialize, Deserialize)]
pub struct WasmPlayer {
    pub id: String,
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
