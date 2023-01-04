use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use core_ds::{state::SerializedComponent, constant::SEEDS_ENTITY_PREFIX};
use core_ds::account::MaxSize;
use dominari::{component::*, constant::{SEEDS_BLUEPRINT, SEEDS_INSTANCEINDEX}, state::GameConfig};
use wasm_bindgen::prelude::*;
use std::{str::FromStr, collections::BTreeMap};
use anchor_lang::system_program::ID as system_program;
use crate::coreds::get_keys_from_id;
use crate::wasm_wrappers::GameConfigFile;
use crate::{component_schemas::ComponentIndex, blueprints::BlueprintConfig};

#[wasm_bindgen]
#[derive(Default)]
pub struct Dominari {
    pub program_id: Pubkey,
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

    pub fn initalize(&self, payer: &str, component_index:&ComponentIndex) -> JsValue {
        let payer = Pubkey::from_str(payer).unwrap();
        let config = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER
        ], &self.program_id).0;

        let ix = Instruction {
            program_id: self.program_id,
            accounts: dominari::accounts::Initialize {
                payer,
                system_program,
                config
            }.to_account_metas(None),
            data: dominari::instruction::Initialize {
                component_keys: component_index.get_relevant_component_keys()
            }.data()
        };
        serde_wasm_bindgen::to_value(&ix).unwrap()
    }

    pub fn register_blueprint(&self, payer:&str, name:&str, component_index:&ComponentIndex, blueprint_json: JsValue) -> JsValue {
        let blueprint: BlueprintConfig = serde_wasm_bindgen::from_value(blueprint_json).unwrap();
        let mut components: BTreeMap<Pubkey, SerializedComponent> = BTreeMap::new();
        let reference = component_index.get_relevant_component_keys();

        // Ignoring blueprint.metadata cause it'll get overwritten anyway
        // it's only used locally for front end

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
            // Convert the Drop Table into Pubkey Objects
            let drop_table_keys:Vec<Pubkey> = blueprint.drop_table.unwrap().drop_table.iter().map(|keystr: &String| {Pubkey::from_str(keystr.as_str()).unwrap()}).collect();
            let comp_drop_table = ComponentDropTable {
                drop_table: drop_table_keys
            };

            components.insert(reference.drop_table, SerializedComponent { 
                max_size: ComponentDropTable::get_max_size(), 
                data:  comp_drop_table.try_to_vec().unwrap()
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

        let ix = Instruction {
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
        };
        serde_wasm_bindgen::to_value(&ix).unwrap()
    }
    
    pub fn get_blueprint_key(&self, name:&str) -> String {
        let blueprint_key = Pubkey::find_program_address(&[
            SEEDS_BLUEPRINT,
            name.as_bytes(),
        ], &self.program_id).0;
        return blueprint_key.to_string()
    }

    pub fn create_game_instance(&self, payer:&str, instance: u64, game_config_json: JsValue) -> JsValue {
        let game_config_file:GameConfigFile = serde_wasm_bindgen::from_value(game_config_json).unwrap();
        let starting_cards_keys:Vec<Pubkey> = game_config_file.starting_cards.iter().map(|keystr: &String| {Pubkey::from_str(keystr.as_str()).unwrap()}).collect();
        let game_config = GameConfig { max_players: game_config_file.max_players, starting_cards: starting_cards_keys };

        let payer = Pubkey::from_str(payer).unwrap();
        let config = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER
        ], &self.program_id).0;
        
        let registry_instance = Pubkey::find_program_address(&[
            core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX,
            registry::id().to_bytes().as_ref(),
            instance.to_be_bytes().as_ref()
        ], &core_ds::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            SEEDS_INSTANCEINDEX,
            registry_instance.to_bytes().as_ref(),
        ], &self.program_id).0;

        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &registry::id()).0;

        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
        ], &self.program_id).0;

        let ab_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab_signer.to_bytes().as_ref()
        ], &registry::id()).0;

        let ix = Instruction {
            program_id: self.program_id,
            accounts: dominari::accounts::CreateGameInstance {
                payer,
                system_program,
                config,
                instance_index,
                registry_config,
                registry_program: registry::id(),
                ab_registration,
                registry_instance,
                coreds: core_ds::id()
            }.to_account_metas(Some(true)),
            data: dominari::instruction::CreateGameInstance {
                instance,
                game_config,
            }.data()
        };

        serde_wasm_bindgen::to_value(&ix).unwrap()
    }

    pub fn init_map(&self, payer:&str, instance:u64, entity_id:u64, max_x:u8, max_y:u8) -> JsValue {
        let payer = Pubkey::from_str(payer).unwrap();
        let config = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER
        ], &self.program_id).0;
        
        let registry_instance = Pubkey::find_program_address(&[
            core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX,
            registry::id().to_bytes().as_ref(),
            instance.to_be_bytes().as_ref()
        ], &core_ds::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            SEEDS_INSTANCEINDEX,
            registry_instance.to_bytes().as_ref(),
        ], &self.program_id).0;

        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &registry::id()).0;

        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
        ], &self.program_id).0;

        let ab_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab_signer.to_bytes().as_ref()
        ], &registry::id()).0;

        let map_entity = Pubkey::find_program_address(&[
            SEEDS_ENTITY_PREFIX,
            entity_id.to_be_bytes().as_ref(),
            registry_instance.to_bytes().as_ref(),
        ], &core_ds::id()).0;

        let ix = Instruction {
            program_id: self.program_id,
            accounts: dominari::accounts::SystemInitMap {
                payer,
                system_program,
                config,
                instance_index,
                registry_config,
                registry_program: registry::id(),
                ab_registration,
                coreds: core_ds::id(),
                registry_instance,
                map_entity
            }.to_account_metas(Some(true)),
            data: dominari::instruction::SystemInitMap {
                entity_id,
                max_x,
                max_y
            }.data()
        };

        serde_wasm_bindgen::to_value(&ix).unwrap()
    }

    pub fn init_tile(&self, payer:&str, instance:u64, entity_id:u64, x:u8, y:u8, cost:u64) -> JsValue {
        let payer = Pubkey::from_str(payer).unwrap();
        let config = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER
        ], &self.program_id).0;
        
        let registry_instance = Pubkey::find_program_address(&[
            core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX,
            registry::id().to_bytes().as_ref(),
            instance.to_be_bytes().as_ref()
        ], &core_ds::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            SEEDS_INSTANCEINDEX,
            registry_instance.to_bytes().as_ref(),
        ], &self.program_id).0;

        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &registry::id()).0;

        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
        ], &self.program_id).0;

        let ab_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab_signer.to_bytes().as_ref()
        ], &registry::id()).0;

        let tile_entity = Pubkey::find_program_address(&[
            SEEDS_ENTITY_PREFIX,
            entity_id.to_be_bytes().as_ref(),
            registry_instance.to_bytes().as_ref()
        ], &core_ds::id()).0;

        let ix = Instruction {
            program_id: self.program_id,
            accounts: dominari::accounts::SystemInitTile {
                payer,
                system_program,
                config,
                instance_index,
                registry_config,
                registry_program: registry::id(),
                ab_registration,
                coreds: core_ds::id(),
                registry_instance,
                tile_entity
            }.to_account_metas(Some(true)),
            data: dominari::instruction::SystemInitTile {
                entity_id,
                x,
                y,
                cost
            }.data()
        };

        serde_wasm_bindgen::to_value(&ix).unwrap()
    }

    pub fn init_player(&self, payer:&str, instance:u64, entity_id:u64, name: String, image: String) -> JsValue {
        let payer = Pubkey::from_str(payer).unwrap();
        let config = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER
        ], &self.program_id).0;
        
        let registry_instance = Pubkey::find_program_address(&[
            core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX,
            registry::id().to_bytes().as_ref(),
            instance.to_be_bytes().as_ref()
        ], &core_ds::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            SEEDS_INSTANCEINDEX,
            registry_instance.to_bytes().as_ref(),
        ], &self.program_id).0;

        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &registry::id()).0;

        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
        ], &self.program_id).0;

        let ab_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab_signer.to_bytes().as_ref()
        ], &registry::id()).0;

        let player_entity = Pubkey::find_program_address(&[
            SEEDS_ENTITY_PREFIX,
            entity_id.to_be_bytes().as_ref(),
            registry_instance.to_bytes().as_ref()
        ], &core_ds::id()).0;

        let ix = Instruction {
            program_id: self.program_id,
            accounts: dominari::accounts::SystemInitPlayer {
                payer,
                system_program,
                config,
                instance_index,
                registry_config,
                ab_registration,
                registry_program: registry::id(),
                coreds: core_ds::id(),
                registry_instance,
                player_entity,
            }.to_account_metas(Some(true)),
            data: dominari::instruction::SystemInitPlayer {
                entity_id,
                name,
                image,
            }.data()
        };

        serde_wasm_bindgen::to_value(&ix).unwrap()
    }

    pub fn spawn_unit(&self, payer:&str, instance:u64, player_id: u64, unit_id:u64, tile_id:u64, blueprint: &str) -> JsValue {
        let payer = Pubkey::from_str(payer).unwrap();
        let config = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER
        ], &self.program_id).0;
        
        let registry_instance = Pubkey::find_program_address(&[
            core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX,
            registry::id().to_bytes().as_ref(),
            instance.to_be_bytes().as_ref()
        ], &core_ds::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            SEEDS_INSTANCEINDEX,
            registry_instance.to_bytes().as_ref(),
        ], &self.program_id).0;

        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &registry::id()).0;

        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
        ], &self.program_id).0;

        let ab_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab_signer.to_bytes().as_ref()
        ], &registry::id()).0;

        let unit_blueprint = Pubkey::from_str(self.get_blueprint_key(blueprint).as_str()).unwrap();

        let player = get_keys_from_id(registry_instance, vec![player_id])[0];

        let tile = get_keys_from_id(registry_instance, vec![tile_id])[0];

        let unit = get_keys_from_id(registry_instance, vec![unit_id])[0];

        let ix = Instruction {
            program_id: self.program_id,
            accounts: dominari::accounts::SpawnUnit {
                payer,
                system_program,
                config,
                instance_index,
                registry_config,
                ab_registration,
                registry_program: registry::id(),
                coreds: core_ds::id(),
                registry_instance,
                unit_blueprint,
                player,
                tile,
                unit,
            }.to_account_metas(Some(true)),
            data: dominari::instruction::SpawnUnit {
                unit_id,
            }.data()
        };
        serde_wasm_bindgen::to_value(&ix).unwrap()
    }

    pub fn change_game_state(&self, payer:&str, instance:u64, player_id:u64, game_state_str:String) -> JsValue {
        let payer = Pubkey::from_str(payer).unwrap();
        let config = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER
        ], &self.program_id).0;
        
        let registry_instance = Pubkey::find_program_address(&[
            core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX,
            registry::id().to_bytes().as_ref(),
            instance.to_be_bytes().as_ref()
        ], &core_ds::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            SEEDS_INSTANCEINDEX,
            registry_instance.to_bytes().as_ref(),
        ], &self.program_id).0;

        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &registry::id()).0;

        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
        ], &self.program_id).0;

        let ab_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab_signer.to_bytes().as_ref()
        ], &registry::id()).0;

        let player = get_keys_from_id(registry_instance, vec![player_id])[0];

        let mut game_state: dominari::account::PlayPhase = dominari::account::PlayPhase::Paused;
        
        match game_state_str.as_str() {
            "Lobby" => game_state = dominari::account::PlayPhase::Lobby,
            "Build" => game_state = dominari::account::PlayPhase::Build,
            "Play" => game_state = dominari::account::PlayPhase::Play,
            "Paused" => game_state = dominari::account::PlayPhase::Paused,
            "Finished" => game_state = dominari::account::PlayPhase::Finished,
            &_=> {}
        }

        let ix = Instruction {
            program_id: self.program_id,
            accounts: dominari::accounts::ChangeGameState {
                payer,
                config,
                instance_index,
                registry_config,
                ab_registration,
                registry_program: registry::id(),
                coreds: core_ds::id(),
                registry_instance,
                player,
            }.to_account_metas(Some(true)),
            data: dominari::instruction::ChangeGameState {
                game_state
            }.data()
        };

        serde_wasm_bindgen::to_value(&ix).unwrap()
    }

    pub fn init_feature(&self, payer:&str, instance:u64, entity_id:u64, tile_id:u64, blueprint: String) -> JsValue {
        let payer = Pubkey::from_str(payer).unwrap();
        let config = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER
        ], &self.program_id).0;
        
        let registry_instance = Pubkey::find_program_address(&[
            core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX,
            registry::id().to_bytes().as_ref(),
            instance.to_be_bytes().as_ref()
        ], &core_ds::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            SEEDS_INSTANCEINDEX,
            registry_instance.to_bytes().as_ref(),
        ], &self.program_id).0;

        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &registry::id()).0;

        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
        ], &self.program_id).0;

        let ab_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab_signer.to_bytes().as_ref()
        ], &registry::id()).0;

        let blueprint = Pubkey::from_str(self.get_blueprint_key(blueprint.as_str()).as_str()).unwrap();
        let tile_entity = Pubkey::find_program_address(&[
            SEEDS_ENTITY_PREFIX,
            tile_id.to_be_bytes().as_ref(),
            registry_instance.to_bytes().as_ref()
        ], &core_ds::id()).0;

        let feature_entity = Pubkey::find_program_address(&[
            SEEDS_ENTITY_PREFIX,
            entity_id.to_be_bytes().as_ref(),
            registry_instance.to_bytes().as_ref()
        ], &core_ds::id()).0;


        let ix = Instruction {
            program_id: self.program_id,
            accounts: dominari::accounts::SystemInitFeature {
                payer,
                system_program,
                config,
                instance_index,
                registry_config,
                registry_program: registry::id(),
                ab_registration,
                coreds: core_ds::id(),
                registry_instance,
                blueprint,
                tile_entity,
                feature_entity,
            }.to_account_metas(Some(true)),
            data: dominari::instruction::SystemInitFeature {
                entity_id
            }.data()
        };
        serde_wasm_bindgen::to_value(&ix).unwrap()
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