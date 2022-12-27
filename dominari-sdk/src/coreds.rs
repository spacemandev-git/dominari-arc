use anchor_lang::prelude::Pubkey;
use core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX;

pub fn get_keys_from_id(registry_instance: Pubkey, ids: Vec<u64>) -> Vec<Pubkey> {
    let mut keys = vec![];
    for id in ids {
        keys.push(Pubkey::find_program_address(&[
            core_ds::constant::SEEDS_ENTITY_PREFIX,
            id.to_be_bytes().as_ref(),
            registry_instance.to_bytes().as_ref()
        ], &core_ds::id()).0);
    }
    keys
}

pub fn get_registry_instance(registry: Pubkey, instance:u64) -> Pubkey {
    Pubkey::find_program_address(&[
        SEEDS_REGISTRYINSTANCE_PREFIX,
        registry.to_bytes().as_ref(),
        instance.to_be_bytes().as_ref()
    ], &core_ds::id()).0 
}