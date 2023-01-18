use std::{collections::HashMap, str::FromStr};
use anchor_lang::prelude::*;
use wasm_bindgen::{prelude::*, throw_str};
use solana_client_wasm::WasmClient;
use dominari::account::{InstanceIndex, PlayPhase};
use core_ds::account::Entity;
use dominari::component::*;
use crate::{component_schemas::ComponentIndex, coreds::{get_registry_instance, get_keys_from_id}, wasm_wrappers::{WasmTile, WasmFeature, WasmTroop, WasmPlayer}, blueprints::BlueprintIndex};
//use web_sys::console;

#[wasm_bindgen]
pub struct GameState {
    pub dominari_program_id: Pubkey,
    pub registry_program_id: Pubkey,
    pub instance: u64, 
    #[wasm_bindgen(skip)]
    pub component_index: ComponentIndex,
    #[wasm_bindgen(skip)]
    pub client: WasmClient,
    #[wasm_bindgen(skip)]
    pub index: Option<InstanceIndex>,
    #[wasm_bindgen(skip)]
    pub entities: HashMap<u64, Entity>,
    #[wasm_bindgen(skip)]
    pub blueprint_index: BlueprintIndex,
    pub is_state_loaded: bool
}

#[wasm_bindgen]
impl GameState{
    #[wasm_bindgen(constructor)]
    pub fn new(rpc:&str, dominari_id:&str, registry_id:&str, instance:u64) -> Self {
        console_error_panic_hook::set_once();
        GameState { 
            dominari_program_id: Pubkey::from_str(dominari_id).unwrap(), 
            registry_program_id: Pubkey::from_str(registry_id).unwrap(), 
            instance,
            component_index: ComponentIndex::new(registry_id),
            client: WasmClient::new(rpc), 
            index: None, 
            entities: HashMap::new(),
            blueprint_index: BlueprintIndex::new(dominari_id),
            is_state_loaded: false
        }
    }

    pub fn add_blueprints(&mut self, blueprints_json: JsValue) {
        let blueprints: Vec<String> = serde_wasm_bindgen::from_value(blueprints_json).unwrap();
        for blueprint in blueprints {
            self.blueprint_index.insert_blueprint_name(blueprint);
        }
    }

    pub async fn update_instance_index(&mut self) {
        let registry_instance = Pubkey::find_program_address(&[
            core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX,
            self.registry_program_id.to_bytes().as_ref(),
            self.instance.to_be_bytes().as_ref()
        ], &core_ds::id()).0;

        let instance_index = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_INSTANCEINDEX,
            registry_instance.to_bytes().as_ref(),
        ], &self.dominari_program_id).0;

        let index:InstanceIndex = fetch_account(&self.client, &instance_index).await.unwrap();
        self.index = Some(index.clone());
    }

    pub async fn load_state(&mut self) {
        self.is_state_loaded = true;
        let registry_instance = get_registry_instance(registry::id(), self.instance);
        self.update_instance_index().await;
        
        let mut entities: HashMap<u64, Entity> = HashMap::new();
        entities.insert(
            self.index.as_ref().unwrap().map,
            fetch_accounts::<Entity>(
                &self.client,
                &&get_keys_from_id(
                    registry_instance,
                    vec![self.index.as_ref().unwrap().map]
                )
            ).await.get(0).unwrap().1.to_owned()
        );        
        
        let tile_entities:Vec<(Pubkey, Entity)> = fetch_accounts::<Entity>(&self.client, &get_keys_from_id(registry_instance, self.index.as_ref().unwrap().tiles.clone())).await;
        for (i, e) in tile_entities.iter().enumerate() {
            entities.insert(*self.index.as_ref().unwrap().tiles.get(i).unwrap(), e.1.to_owned());
        }

        let feature_entities:Vec<(Pubkey, Entity)> = fetch_accounts(&self.client, &get_keys_from_id(registry_instance, self.index.as_ref().unwrap().features.clone())).await;
        for (i, e) in feature_entities.iter().enumerate() {
            entities.insert(*self.index.as_ref().unwrap().features.get(i).unwrap(), e.1.to_owned());
        }

        let unit_entities:Vec<(Pubkey, Entity)> = fetch_accounts(&self.client, &get_keys_from_id(registry_instance, self.index.as_ref().unwrap().units.clone())).await;
        for (i, e) in unit_entities.iter().enumerate() {
            entities.insert(*self.index.as_ref().unwrap().units.get(i).unwrap(), e.1.to_owned());
        }

        let player_entities:Vec<(Pubkey, Entity)> = fetch_accounts(&self.client, &get_keys_from_id(registry_instance, self.index.as_ref().unwrap().players.clone())).await;
        for (i, e) in player_entities.iter().enumerate() {
            entities.insert(*self.index.as_ref().unwrap().players.get(i).unwrap(), e.1.to_owned());
        }

        self.entities = entities;
    }

    pub async fn update_entity(&mut self, entity_id:u64) {
        // Don't worry about finding this in index, just fetch the account and update the entities table
        let pubkey = get_keys_from_id(get_registry_instance(registry::id(), self.instance), vec![entity_id]);
        let entity:Entity = fetch_account(&self.client, &pubkey[0]).await.unwrap();
        self.entities.insert(entity_id, entity);
    }

    pub fn get_tile_id(&self, x:u8, y:u8) -> String {
        if self.index.is_none() {
            throw_str("Index isn't built yet!");
        }

        for id in &self.index.as_ref().unwrap().tiles {
            let location = self.get_entity_location(id).unwrap_throw();
            if location.x == x && location.y == y {
                return id.clone().to_string();
            }
        }
        throw_str("Tile Not Found!");
    }

    pub  fn get_wasm_tile(&self, tile_id:u64) -> JsValue {
        return serde_wasm_bindgen::to_value(&self.get_tile_info(tile_id)).unwrap()
    }

    pub fn get_map(&self) -> JsValue {
        if self.index.is_none() {
            throw_str("Load state first!")
        }
        let mut tiles: Vec<WasmTile> = vec![];

        for tile_id in self.index.as_ref().unwrap().tiles.iter() {
            tiles.push(self.get_tile_info(*tile_id));
        }

        serde_wasm_bindgen::to_value(&tiles).unwrap()
    }

    /**
     * @param user is Pubkey
     */
    pub fn get_player_info(&self, user: &str) -> JsValue {
        let pubkey = Pubkey::from_str(user).unwrap();
        // Search through Player Entities to see if any match 
        let player_id = self.get_player(pubkey);
        if player_id.is_none() {
            return serde_wasm_bindgen::to_value(&None::<WasmPlayer>).unwrap();
        } else {
            let stats = self.get_entity_player_stats(&player_id.unwrap()).unwrap();
            let cardnames = stats.cards.iter().map(|&cardkey| {return self.blueprint_index.get_blueprint_name(cardkey.to_string())}).collect(); 
            let player = WasmPlayer {
                id: player_id.unwrap().to_string(),
                name: stats.name,
                image: stats.image,
                score: stats.score.to_string(),
                kills: stats.kills.to_string(),
                cards: cardnames
            };
            return serde_wasm_bindgen::to_value(&player).unwrap();
        }
    }

    pub fn get_play_phase(&self) -> String {
        match self.index.as_ref().unwrap().play_phase {
            PlayPhase::Build => return String::from("Build"),
            PlayPhase::Lobby => return String::from("Lobby"),
            PlayPhase::Play => return String::from("Play"),
            PlayPhase::Paused => return String::from("Paused"),
            PlayPhase::Finished => return String::from("Finished"),
        }
    }
}

/**
 * Non WASM Endpoints
 */
impl GameState {
    
    pub fn get_tile_info(&self, tile_id:u64) -> WasmTile {
        // All tiles have these four components
        let location = self.get_entity_location(&tile_id).unwrap();
        let feature = self.get_entity_feature(&tile_id).unwrap().feature_id;
        let troop = self.get_entity_occupant(&tile_id).unwrap().occupant_id;

        let mut tile = WasmTile {
            x: location.x,
            y: location.y,
            feature: None,
            troop: None,
        };

        if feature.is_some() {
            let f_id = feature.unwrap();
            let feature_metadata = self.get_entity_metadata(&f_id).unwrap();
            tile.feature = Some(WasmFeature {
                name: feature_metadata.name,
                id: f_id.to_string()
            })
        }

        if troop.is_some() {
            let t_id = troop.unwrap();
            let troop_metadata = self.get_entity_metadata(&t_id).unwrap();
            let troop_player = self.get_entity_owner(&t_id).unwrap();
            let damage = self.get_entity_damage(&t_id).unwrap();
            let health = self.get_entity_health(&t_id).unwrap();
            let class = self.get_entity_troop_class(&t_id).unwrap();
            let range = self.get_entity_range(&t_id).unwrap();
            let last_used = self.get_entity_last_used(&t_id).unwrap();
            let value = self.get_entity_value(&t_id).unwrap();


            tile.troop = Some(WasmTroop {
                name: troop_metadata.name,
                id: t_id.to_string(),
                troop_owner_player_id: troop_player.player.unwrap().to_string(),
                troop_owner_player_key: troop_player.owner.unwrap().to_string(),
                min_damage: damage.min_damage.to_string(),
                max_damage: damage.max_damage.to_string(),
                bonus_infantry: damage.bonus_infantry.to_string(),
                bonus_armor: damage.bonus_armor.to_string(),
                bonus_aircraft: damage.bonus_aircraft.to_string(),
                bonus_feature: damage.bonus_feature.to_string(),
                health: health.health.to_string(),
                class: class.class,
                movement: range.movement,
                attack_range: range.attack_range,
                last_used: last_used.last_used.to_string(),
                recovery: last_used.recovery.to_string(),
                value: value.value.to_string()
            })
        }
        tile
    }

    pub fn get_player(&self, player: Pubkey) -> Option<u64> {
        if self.index.is_none() {
            return None;
        }

        for player_id in self.index.as_ref().unwrap().players.iter() {
            let player_key = self.get_entity_player_stats(&player_id).unwrap().key;
            if player.key() == player_key.key() {
                return Some(player_id.to_owned());
            }
        }

        return None;
    }

    pub fn get_entity(&self, entity_id:u64) -> Option<&Entity> {
        return self.entities.get(&entity_id)
    }

    /** COMPONENT GETTERS */
    pub fn get_entity_metadata(&self, entity_id: &u64) -> Option<ComponentMetadata> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().metadata.key());
        if sc.is_none() { return None };
        Some(ComponentMetadata::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_mapmeta(&self, entity_id: &u64) -> Option<ComponentMapMeta> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().mapmeta.key());
        if sc.is_none() { return None };
        Some(ComponentMapMeta::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_location(&self, entity_id: &u64) -> Option<ComponentLocation> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().location.key());
        if sc.is_none() { return None };
        Some(ComponentLocation::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_feature(&self, entity_id: &u64) -> Option<ComponentFeature> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().feature.key());
        if sc.is_none() { return None };
        Some(ComponentFeature::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_owner(&self, entity_id: &u64) -> Option<ComponentOwner> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().owner.key());
        if sc.is_none() { return None };
        Some(ComponentOwner::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_value(&self, entity_id: &u64) -> Option<ComponentValue> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().value.key());
        if sc.is_none() { return None };
        Some(ComponentValue::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_occupant(&self, entity_id: &u64) -> Option<ComponentOccupant> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().occupant.key());
        if sc.is_none() { return None };
        Some(ComponentOccupant::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_player_stats(&self, entity_id: &u64) -> Option<ComponentPlayerStats> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().player_stats.key());
        if sc.is_none() { return None };
        Some(ComponentPlayerStats::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_last_used(&self, entity_id: &u64) -> Option<ComponentLastUsed> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().last_used.key());
        if sc.is_none() { return None };
        Some(ComponentLastUsed::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_feature_rank(&self, entity_id: &u64) -> Option<ComponentFeatureRank> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().feature_rank.key());
        if sc.is_none() { return None };
        Some(ComponentFeatureRank::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_range(&self, entity_id: &u64) -> Option<ComponentRange> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().range.key());
        if sc.is_none() { return None };
        Some(ComponentRange::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_drop_table(&self, entity_id: &u64) -> Option<ComponentDropTable> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().drop_table.key());
        if sc.is_none() { return None };
        Some(ComponentDropTable::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_uses(&self, entity_id: &u64) -> Option<ComponentUses> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().uses.key());
        if sc.is_none() { return None };
        Some(ComponentUses::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_healing_power(&self, entity_id: &u64) -> Option<ComponentHealingPower> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().healing_power.key());
        if sc.is_none() { return None };
        Some(ComponentHealingPower::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_health(&self, entity_id: &u64) -> Option<ComponentHealth> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().health.key());
        if sc.is_none() { return None };
        Some(ComponentHealth::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_damage(&self, entity_id: &u64) -> Option<ComponentDamage> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().damage.key());
        if sc.is_none() { return None };
        Some(ComponentDamage::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_troop_class(&self, entity_id: &u64) -> Option<ComponentTroopClass> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().troop_class.key());
        if sc.is_none() { return None };
        Some(ComponentTroopClass::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_active(&self, entity_id: &u64) -> Option<ComponentActive> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().active.key());
        if sc.is_none() { return None };
        Some(ComponentActive::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_cost(&self, entity_id: &u64) -> Option<ComponentCost> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().cost.key());
        if sc.is_none() { return None };
        Some(ComponentCost::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
    pub fn get_entity_offchain_metadata(&self, entity_id: &u64) -> Option<ComponentOffchainMetadata> {
        let serialized_components = &self.entities.get(&entity_id).unwrap().components;
        let sc = serialized_components.get(&self.component_index.get_relevant_component_keys().offchain_metadata.key());
        if sc.is_none() { return None };
        Some(ComponentOffchainMetadata::try_from_slice(&sc.unwrap().data.as_slice()).unwrap())
    }
}

pub async fn fetch_account<T: AccountDeserialize>(client: &WasmClient, pubkey: &Pubkey) -> Result<T> {
    let mut data:&[u8] = &client.get_account(pubkey).await.unwrap().data;
    let result: Result<T> = deserialize_account(&mut data).await;
    return result;
}

/**
 * Makes the assumption that the accounts returned are in the same order as the keys passed in
 * This is because the acocunts returned don't have the pubkey attached to them.
 */
pub async fn fetch_accounts<T: AccountDeserialize>(client: &WasmClient, pubkeys: &Vec<Pubkey>) -> Vec<(Pubkey,T)> {
    let accounts = &client.get_multiple_accounts(pubkeys).await.unwrap();
    let mut results = vec![];
    for (i, account) in accounts.iter().enumerate() {
        let result: Result<T> = deserialize_account(&account.as_ref().unwrap().data).await;
        results.push((pubkeys.get(i).unwrap().clone(), result.unwrap()));
    }
    return results;
}

pub async fn deserialize_account<T: AccountDeserialize>(mut data: &[u8]) -> Result<T> {
    let result = T::try_deserialize(&mut data).map_err(Into::into);
    return result;
}