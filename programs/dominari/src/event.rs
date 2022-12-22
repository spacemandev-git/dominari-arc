use anchor_lang::prelude::*;

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