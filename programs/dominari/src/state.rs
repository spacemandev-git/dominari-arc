use anchor_lang::prelude::*;

use core_ds::account::MaxSize;

#[cfg_attr(feature = "sdk", derive(serde::Serialize, serde::Deserialize))]
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct RelevantComponentKeys {
    pub metadata: Pubkey,
    pub mapmeta: Pubkey,
    pub location: Pubkey,
    pub feature: Pubkey,
    pub owner: Pubkey,
    pub value: Pubkey,
    pub occupant: Pubkey,
    pub player_stats: Pubkey,
    pub last_used: Pubkey,
    pub feature_rank: Pubkey,
    pub range: Pubkey,
    pub drop_table: Pubkey,
    pub uses: Pubkey,
    pub healing_power: Pubkey,
    pub health: Pubkey,
    pub damage: Pubkey,
    pub troop_class: Pubkey,
    pub active: Pubkey,
    pub cost: Pubkey,
    pub offchain_metadata: Pubkey
}

impl MaxSize for RelevantComponentKeys {
    fn get_max_size() -> u64 {
        return 32*20;
    }
}

/**
 * Specifically NOT giving this a Deserialize Trait for SDK
 * Because it contains Vec<Pubkey> we need to have a wrapper on the SDK  
 * that's Vec<string> that we then map into Vec<Pubkey> for this GameConfig
 */
#[cfg_attr(feature = "sdk", derive(serde::Serialize, serde::Deserialize))]
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct GameConfig {
    pub max_players: u16,
    pub starting_cards: Vec<Pubkey>,
    pub game_mode: GameMode,
}

impl DependentMaxSize for GameConfig {
    fn get_max_size(&self) -> u64 {
        return 2 + 4 + (self.starting_cards.len() as u64 * 32_u64) + self.game_mode.get_max_size();
    }
}

pub trait DependentMaxSize {
    fn get_max_size(&self) -> u64;
}

#[cfg_attr(feature = "sdk", derive(serde::Serialize, serde::Deserialize))]
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub enum UseFeatureType {
    Healer, 
    Portal,
    Attack,
    Loot,
}

#[cfg_attr(feature = "sdk", derive(serde::Serialize, serde::Deserialize))]
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub enum GameMode {
    Sandbox,    // No End
    KOTH {
        max_score: u64,
        score_interval_in_sec: u64,
        score_per_interval: u64,
        hill_tiles: Vec<u64> //tile ids
    },       // King of the Hill, get Points for holding the Center
    MaxScore,   // First one to reach specified score
    Artifact    // Find an artifact and get to exit
}

impl DependentMaxSize for GameMode {
    fn get_max_size(&self) -> u64 {
        // For enums it's 1+Max Size of the largest Enum
        match self {
            GameMode::KOTH {
                max_score: _,
                score_interval_in_sec: _,
                score_per_interval: _,
                hill_tiles
            } => {
                return 1+8+8+8+(8 * hill_tiles.len() as u64);
            },
            _=> {return 1+1}
        }
    }
}