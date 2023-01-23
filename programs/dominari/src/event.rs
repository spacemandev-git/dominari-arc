use anchor_lang::prelude::*;

use crate::account::PlayPhase;

#[event]
pub struct NewWorldInstance {
    pub world_instance: Pubkey,
    pub instance_authority: Pubkey
}

#[event]
pub struct NewComponentRegistered {
    pub component: Pubkey,
    pub schema: String
}

#[event]
pub struct NewSystemRegistration {
    pub world_instance: Pubkey,
    pub component: Pubkey,
    pub system: Pubkey,
    pub system_registration: Pubkey
}

#[event]
pub struct NewUnitSpawned {
    pub instance: u64,
    pub tile: u64,
    pub player: u64,
    pub unit: u64,
}

#[event]
pub struct TroopMovement {
    pub instance: u64,
    pub from: u64,
    pub to: u64,
    pub unit: u64
}

#[event]
pub struct TileAttacked {
    pub instance:u64,
    pub attacker: u64,
    pub defender: u64,
    pub defending_tile: u64,
    pub damage: u64
}

#[event]
pub struct GameStateChanged {
    pub instance: u64,
    pub player: u64,
    pub new_state: PlayPhase
}

#[event]
pub struct ScoreChanged {
    pub instance: u64,
    pub player: u64,
    pub new_score: u64
}