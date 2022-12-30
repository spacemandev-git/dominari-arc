use std::str::FromStr;

use wasm_bindgen::prelude::*;
use anchor_lang::prelude::*;
use serde::Deserialize;

use crate::wasm_wrappers::{ComponentDropTableWASM, ComponentMetadataWASM};

#[derive(Deserialize, Debug)]
pub struct BlueprintConfig {
    pub metadata: Option<ComponentMetadataWASM>, // Uses Pubkey
    pub mapmeta: Option<dominari::component::ComponentMapMeta>,
    pub location: Option<dominari::component::ComponentLocation>,
    pub feature: Option<dominari::component::ComponentFeature>,
    pub owner: Option<dominari::component::ComponentOwner>, // Uses Pubkey
    pub value: Option<dominari::component::ComponentValue>,
    pub occupant: Option<dominari::component::ComponentOccupant>,
    pub player_stats: Option<dominari::component::ComponentPlayerStats>, // Uses Pubkey
    pub last_used: Option<dominari::component::ComponentLastUsed>,
    pub feature_rank: Option<dominari::component::ComponentFeatureRank>,
    pub range: Option<dominari::component::ComponentRange>,
    pub drop_table: Option<ComponentDropTableWASM>, // Uses Pubkey
    pub uses: Option<dominari::component::ComponentUses>,
    pub healing_power: Option<dominari::component::ComponentHealingPower>,
    pub health: Option<dominari::component::ComponentHealth>,
    pub damage: Option<dominari::component::ComponentDamage>,
    pub troop_class: Option<dominari::component::ComponentTroopClass>,
    pub active: Option<dominari::component::ComponentActive>,
    pub cost: Option<dominari::component::ComponentCost>,
    pub offchain_metadata: Option<dominari::component::ComponentOffchainMetadata>,
}

#[wasm_bindgen]
pub struct BlueprintIndex {
    #[wasm_bindgen(skip)]
    pub dominari: Pubkey,
    #[wasm_bindgen(skip)]
    pub index: bimap::BiHashMap<String, Pubkey>
}

#[wasm_bindgen]
impl BlueprintIndex {
    pub fn new(dominari: &str) -> Self {
        BlueprintIndex { dominari: Pubkey::from_str(dominari).unwrap(), index: bimap::BiHashMap::new() }
    }

    pub fn insert_blueprint_name(&mut self, blueprint: String) {
        let pubkey = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_BLUEPRINT,
            blueprint.as_str().as_bytes().as_ref(),
        ], &self.dominari).0;

        self.index.insert(blueprint, pubkey);
    }

    /**
     * Returns the pubkey if no matching name is found
     * Basically "unkown" Blueprint
     */
    pub fn get_blueprint_name(&self, pubkey:String) -> String {
        let key = Pubkey::from_str(pubkey.as_str()).unwrap();
        let name = self.index.get_by_right(&key);
        if name.is_none() {
            return pubkey;
        } else {
            return name.unwrap().to_owned();
        }
    }

    pub fn get_blueprint_key(&self, blueprint: String) -> String {
        let key = self.index.get_by_left(&blueprint);
        if key.is_none() {
            let pubkey = Pubkey::find_program_address(&[
                dominari::constant::SEEDS_BLUEPRINT,
                blueprint.as_str().as_bytes().as_ref(),
            ], &self.dominari).0;
            return pubkey.to_string();
        } else {
            return key.unwrap().to_string();
        }
    }
}