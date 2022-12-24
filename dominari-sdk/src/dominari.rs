use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use core_ds::state::SerializedComponent;
use core_ds::account::MaxSize;
use dominari::{component::*, constant::SEEDS_BLUEPRINT};
use wasm_bindgen::prelude::*;
use std::{str::FromStr, collections::BTreeMap};
use anchor_lang::system_program::ID as system_program;

use crate::{component_schemas::ComponentIndex, blueprints::BlueprintConfig};

#[wasm_bindgen]
#[derive(Default)]
pub struct Dominari {
    pub program_id: Pubkey
}

#[wasm_bindgen]
impl Dominari {
    #[wasm_bindgen(constructor)]
    pub fn new(id:&str) -> Self {
        console_error_panic_hook::set_once();
        Dominari {
            program_id: Pubkey::from_str(id).unwrap()
        }
    }

    pub fn initalize(&self, payer: &str, component_index:&ComponentIndex) -> Instruction {
        let payer = Pubkey::from_str(payer).unwrap();
        let config = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER
        ], &self.program_id).0;

        Instruction {
            program_id: self.program_id,
            accounts: dominari::accounts::Initialize {
                payer,
                system_program,
                config
            }.to_account_metas(None),
            data: dominari::instruction::Initialize {
                component_keys: component_index.get_relevant_component_keys()
            }.data()
        }
    }

    pub fn register_blueprint(&self, payer:&str, name:&str, component_index:&ComponentIndex, blueprint_json: JsValue) -> Instruction {
        let blueprint: BlueprintConfig = serde_wasm_bindgen::from_value(blueprint_json).unwrap();
        let mut components: BTreeMap<Pubkey, SerializedComponent> = BTreeMap::new();
        let reference = component_index.get_relevant_component_keys();

        if blueprint.mapmeta.is_some() {
            components.insert(reference.metadata, SerializedComponent { 
                max_size: ComponentMapMeta::get_max_size(), 
                data:  blueprint.mapmeta.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.location.is_some() {
            components.insert(reference.location, SerializedComponent { 
                max_size: ComponentLocation::get_max_size(), 
                data:  blueprint.location.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.feature.is_some() {
            components.insert(reference.feature, SerializedComponent { 
                max_size: ComponentFeature::get_max_size(), 
                data:  blueprint.feature.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.owner.is_some() {
            components.insert(reference.owner, SerializedComponent { 
                max_size: ComponentOwner::get_max_size(), 
                data:  blueprint.owner.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.value.is_some() {
            components.insert(reference.value, SerializedComponent { 
                max_size: ComponentValue::get_max_size(), 
                data:  blueprint.value.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.occupant.is_some() {
            components.insert(reference.occupant, SerializedComponent { 
                max_size: ComponentOccupant::get_max_size(), 
                data:  blueprint.occupant.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.player_stats.is_some() {
            components.insert(reference.player_stats, SerializedComponent { 
                max_size: ComponentPlayerStats::get_max_size(), 
                data:  blueprint.player_stats.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.last_used.is_some() {
            components.insert(reference.last_used, SerializedComponent { 
                max_size: ComponentLastUsed::get_max_size(), 
                data:  blueprint.last_used.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.feature_rank.is_some() {
            components.insert(reference.feature_rank, SerializedComponent { 
                max_size: ComponentFeatureRank::get_max_size(), 
                data:  blueprint.feature_rank.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.range.is_some() {
            components.insert(reference.range, SerializedComponent { 
                max_size: ComponentRange::get_max_size(), 
                data:  blueprint.range.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.drop_table.is_some() {
            components.insert(reference.drop_table, SerializedComponent { 
                max_size: ComponentDropTable::get_max_size(), 
                data:  blueprint.drop_table.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.uses.is_some() {
            components.insert(reference.uses, SerializedComponent { 
                max_size: ComponentUses::get_max_size(), 
                data:  blueprint.uses.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.healing_power.is_some() {
            components.insert(reference.healing_power, SerializedComponent { 
                max_size: ComponentHealingPower::get_max_size(), 
                data:  blueprint.healing_power.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.health.is_some() {
            components.insert(reference.health, SerializedComponent { 
                max_size: ComponentHealth::get_max_size(), 
                data:  blueprint.health.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.damage.is_some() {
            components.insert(reference.damage, SerializedComponent { 
                max_size: ComponentDamage::get_max_size(), 
                data:  blueprint.damage.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.troop_class.is_some() {
            components.insert(reference.troop_class, SerializedComponent { 
                max_size: ComponentTroopClass::get_max_size(), 
                data:  blueprint.troop_class.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.active.is_some() {
            components.insert(reference.active, SerializedComponent { 
                max_size: ComponentActive::get_max_size(), 
                data:  blueprint.active.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.cost.is_some() {
            components.insert(reference.cost, SerializedComponent { 
                max_size: ComponentCost::get_max_size(), 
                data:  blueprint.cost.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        if blueprint.offchain_metadata.is_some() {
            components.insert(reference.offchain_metadata, SerializedComponent { 
                max_size: ComponentOffchainMetadata::get_max_size(), 
                data:  blueprint.offchain_metadata.as_ref().unwrap().try_to_vec().unwrap()
            });
        }

        let payer = Pubkey::from_str(payer).unwrap();
        let config = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER
        ], &self.program_id).0;

        let blueprint_key = Pubkey::find_program_address(&[
            SEEDS_BLUEPRINT,
            name.as_bytes(),
        ], &self.program_id).0;

        Instruction {
            program_id: self.program_id,
            accounts: dominari::accounts::RegisterBlueprint {
                payer,
                system_program,
                config,
                blueprint: blueprint_key
            }.to_account_metas(None),
            data: dominari::instruction::RegisterBlueprint {
                name: String::from(name),
                components,
            }.data()
        }
    }
}   

/*
Initialization
0. Create a ComponentIndex and fill with URLs
1. Initalize (Consumes reference to ComponentIndex)
    -> Create AB Signer
2. Register Blueprints

Game Loop
1. Create Game
    -> Create Instance Index
    -> Add Instance to AB Registration
    -> Initalize Map
    -> Initalize Tiles
2. Toggle Game State
3. Game Actions
*/