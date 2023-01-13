use anchor_lang::prelude::*;
use core_ds::account::MaxSize;
use std::collections::BTreeMap;
use crate::{account::*, state::GameConfig};
use crate::constant::*;
use crate::state::{DependentMaxSize};

use core_ds::{
    state::SerializedComponent, 
    account::{RegistryInstance, Entity},
    program::CoreDs
};
use registry::{
    program::Registry, 
    account::{RegistryConfig, ActionBundleRegistration},
    constant::SEEDS_REGISTRYSIGNER
};

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer=payer,
        seeds=[SEEDS_ABSIGNER],
        bump,
        space= 8 + Config::get_max_size() as usize,
    )]
    pub config: Account<'info, Config>
}

#[derive(Accounts)]
#[instruction(name:String, components: BTreeMap<Pubkey, SerializedComponent>)]
pub struct RegisterBlueprint <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        constraint = config.authority.key() == payer.key(),
        seeds=[SEEDS_ABSIGNER],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer=payer,
        seeds=[
            SEEDS_BLUEPRINT,
            name.as_bytes().as_ref()
        ],
        bump,
        space= 8 + STRING_MAX_SIZE as usize + compute_comp_arr_max_size(&components.values().cloned().collect())
    )]
    pub blueprint: Account<'info, Blueprint>,
}

#[derive(Accounts)]
pub struct SystemInitMap<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    
    // Action Bundle
    #[account(
        seeds=[SEEDS_ABSIGNER],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        mut,
        seeds=[
            SEEDS_INSTANCEINDEX,
            registry_instance.key().as_ref()
        ],
        bump,
    )]
    pub instance_index: Box<Account<'info, InstanceIndex>>,    

    //Registry
    #[account(
        seeds = [SEEDS_REGISTRYSIGNER],
        bump,
        seeds::program = registry_instance.registry.key()
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    pub registry_program: Program<'info, Registry>,
    pub ab_registration: Box<Account<'info, ActionBundleRegistration>>,

    //CoreDs
    pub coreds: Program<'info, CoreDs>, 
    pub registry_instance: Account<'info, RegistryInstance>,    

    /// CHECK: Initalized through CPI
    #[account(mut)]
    pub map_entity: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SystemInitTile<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    // Action Bundle
    #[account(
        seeds=[SEEDS_ABSIGNER],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        mut,
        realloc = instance_index.to_account_info().data_len() + ENTITY_ID_SIZE,
        realloc::payer = payer,
        realloc::zero = false,
        seeds=[
            SEEDS_INSTANCEINDEX,
            registry_instance.key().as_ref()
        ],
        bump,
    )]
    pub instance_index: Box<Account<'info, InstanceIndex>>,    

    //Registry
    #[account(
        seeds = [SEEDS_REGISTRYSIGNER],
        bump,
        seeds::program = registry_instance.registry.key()
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    pub registry_program: Program<'info, Registry>,
    pub ab_registration: Box<Account<'info, ActionBundleRegistration>>,

    //CoreDs
    pub coreds: Program<'info, CoreDs>, 
    /// CHECK: Created via CPI
    pub registry_instance: Account<'info, RegistryInstance>,

    /// CHECK: Initalized through CPI
    #[account(mut)]
    pub tile_entity: AccountInfo<'info>,

}


#[derive(Accounts)]
pub struct SystemInitFeature<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    // Action Bundle
    #[account(
        seeds=[SEEDS_ABSIGNER],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        mut,
        realloc = instance_index.to_account_info().data_len() + ENTITY_ID_SIZE,
        realloc::payer = payer,
        realloc::zero = false,
        seeds=[
            SEEDS_INSTANCEINDEX,
            registry_instance.key().as_ref()
        ],
        bump,
    )]
    pub instance_index: Box<Account<'info, InstanceIndex>>,    
    pub blueprint: Box<Account<'info, Blueprint>>,

    //Registry
    #[account(
        seeds = [SEEDS_REGISTRYSIGNER.as_slice()],
        bump,
        seeds::program = registry_instance.registry.key()
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    pub registry_program: Program<'info, Registry>,
    pub ab_registration: Box<Account<'info, ActionBundleRegistration>>,

    //CoreDs
    pub coreds: Program<'info, CoreDs>, 
    pub registry_instance: Account<'info, RegistryInstance>,
    #[account(mut)]
    pub tile_entity: Box<Account<'info, Entity>>,
    /// CHECK: Initalized through CPI
    #[account(mut)]
    pub feature_entity: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SystemInitPlayer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    // Action Bundle
    #[account(
        seeds=[SEEDS_ABSIGNER],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        mut,
        realloc = instance_index.to_account_info().data_len() + ENTITY_ID_SIZE,
        realloc::payer = payer,
        realloc::zero = false,
        seeds=[
            SEEDS_INSTANCEINDEX,
            registry_instance.key().as_ref()
        ],
        bump,
    )]
    pub instance_index: Box<Account<'info, InstanceIndex>>,    

    //Registry
    #[account(
        seeds = [SEEDS_REGISTRYSIGNER.as_slice()],
        bump,
        seeds::program = registry_instance.registry.key()
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    pub registry_program: Program<'info, Registry>,
    pub ab_registration: Box<Account<'info, ActionBundleRegistration>>,

    //CoreDs
    pub coreds: Program<'info, CoreDs>, 
    pub registry_instance: Account<'info, RegistryInstance>,

    /// CHECK: Created via CPI
    #[account(mut)]
    pub player_entity: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(instance:u64, game_config: GameConfig)]
pub struct CreateGameInstance<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    // Action Bundle
    #[account(
        seeds=[SEEDS_ABSIGNER],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,
    
    #[account(
        init,
        payer=payer,
        seeds=[
            SEEDS_INSTANCEINDEX,
            registry_instance.key().as_ref()
        ],
        bump,
        space= 8 + InstanceIndex::get_max_size() as usize + game_config.get_max_size() as usize
    )]
    pub instance_index: Box<Account<'info, InstanceIndex>>,    

    //Registry
    #[account(
        seeds = [SEEDS_REGISTRYSIGNER.as_slice()],
        bump,
        seeds::program = registry::id()
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    pub registry_program: Program<'info, Registry>,
    #[account(mut)]
    pub ab_registration: Box<Account<'info, ActionBundleRegistration>>,


    //CoreDs
    pub coreds: Program<'info, CoreDs>, 
    /// CHECK: Created via CPI in the coreds program
    #[account(mut)]
    pub registry_instance: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ChangeGameState<'info> {
    pub payer: Signer<'info>,

    // Action Bundle
    #[account(
        seeds=[SEEDS_ABSIGNER],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        mut,
        seeds=[
            SEEDS_INSTANCEINDEX,
            registry_instance.key().as_ref()
        ],
        bump,
    )]
    pub instance_index: Box<Account<'info, InstanceIndex>>,

    // Registry
    #[account(
        seeds = [SEEDS_REGISTRYSIGNER.as_slice()],
        bump,
        seeds::program = registry_instance.registry.key()
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    pub registry_program: Program<'info, Registry>,
    pub ab_registration: Box<Account<'info, ActionBundleRegistration>>,

    // CoreDs
    pub coreds: Program<'info, CoreDs>, 
    pub registry_instance: Account<'info, RegistryInstance>,

    // Entities Required
    pub player: Box<Account<'info, Entity>>,
}

#[derive(Accounts)]
pub struct SpawnUnit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    //Action Bundle
    #[account(
        seeds=[SEEDS_ABSIGNER],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        mut,
        realloc = instance_index.to_account_info().data_len() + ENTITY_ID_SIZE,
        realloc::payer = payer,
        realloc::zero = false,
        seeds=[
            SEEDS_INSTANCEINDEX,
            registry_instance.key().as_ref()
        ],
        bump,
    )]
    pub instance_index: Box<Account<'info, InstanceIndex>>,    

    pub unit_blueprint: Account<'info, Blueprint>,

    //Registry
    #[account(
        seeds = [SEEDS_REGISTRYSIGNER.as_slice()],
        bump,
        seeds::program = registry_instance.registry.key()
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    pub registry_program: Program<'info, Registry>,
    pub ab_registration: Box<Account<'info, ActionBundleRegistration>>,

    //CoreDs
    pub coreds: Program<'info, CoreDs>, 
    pub registry_instance: Account<'info, RegistryInstance>,


    #[account(mut)]
    pub player: Box<Account<'info, Entity>>,
    #[account(mut)]
    pub tile: Box<Account<'info, Entity>>,
    /// CHECK: Initalized through CPI
    #[account(mut)]
    pub unit: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MoveUnit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    //Action Bundle
    #[account(
        seeds=[SEEDS_ABSIGNER],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,
    pub instance_index: Box<Account<'info, InstanceIndex>>,    

    //Registry
    #[account(
        seeds = [SEEDS_REGISTRYSIGNER.as_slice()],
        bump,
        seeds::program = registry_instance.registry.key()
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    pub registry_program: Program<'info, Registry>,
    pub ab_registration: Box<Account<'info, ActionBundleRegistration>>,

    //CoreDs
    pub coreds: Program<'info, CoreDs>, 
    pub registry_instance: Account<'info, RegistryInstance>,

    #[account(
        mut,
        constraint = from.instance == registry_instance.instance
    )]
    pub from: Box<Account<'info, Entity>>,
    #[account(
        mut,
        constraint = to.instance == registry_instance.instance
    )]
    pub to: Box<Account<'info, Entity>>,
    #[account(
        mut,
        constraint = unit.instance == registry_instance.instance
    )]
    pub unit: Box<Account<'info, Entity>>,
}

#[derive(Accounts)]
pub struct AttackTile <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    //Action Bundle
    #[account(
        seeds=[SEEDS_ABSIGNER],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,
    pub instance_index: Box<Account<'info, InstanceIndex>>,    

    //Registry
    #[account(
        seeds = [SEEDS_REGISTRYSIGNER.as_slice()],
        bump,
        seeds::program = registry_instance.registry.key()
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    pub registry_program: Program<'info, Registry>,
    pub ab_registration: Box<Account<'info, ActionBundleRegistration>>,

    //CoreDs
    pub coreds: Program<'info, CoreDs>, 
    pub registry_instance: Account<'info, RegistryInstance>,

    #[account(
        mut,
        constraint = attacker.instance == registry_instance.instance
    )]
    pub attacker: Box<Account<'info, Entity>>,
    #[account(
        mut,
        constraint = defender.instance == registry_instance.instance
    )]
    pub defender: Box<Account<'info, Entity>>,
    #[account(
        mut,
        constraint = defending_tile.instance == registry_instance.instance
    )]
    pub defending_tile: Box<Account<'info, Entity>>,
    
}

#[derive(Accounts)]
pub struct ReclaimSol<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    // Action Bundle

    // Registry

    // CoreDs
}


/********************************************UTIL Fns */
pub fn compute_comp_arr_max_size(components: &Vec<SerializedComponent>) -> usize {
    let mut max_size:usize = 0;
    for comp in components {
                    // Max Size + 32 (Pubkey) + Other fields in SerializedComponent
        max_size += comp.max_size as usize + 32 + SerializedComponent::get_max_size() as usize;
    }
    return max_size;
}