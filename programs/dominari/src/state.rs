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

#[cfg_attr(feature = "sdk", derive(serde::Serialize, serde::Deserialize))]
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct GameConfig {
    pub max_players: u16,
    pub starting_cards: Vec<Pubkey>,
}

impl DependentMaxSize for GameConfig {
    fn get_max_size(&self) -> u64 {
        return 2 + 4 + (self.starting_cards.len() as u64 * 32_u64);
    }
}

pub trait DependentMaxSize {
    fn get_max_size(&self) -> u64;
}