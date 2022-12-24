use std::str::FromStr;

use wasm_bindgen::prelude::*;
use anchor_lang::prelude::*;

#[wasm_bindgen]
pub struct ComponentIndex {
    pub registry: Pubkey,
    #[wasm_bindgen(skip)]
    pub index: bimap::BiHashMap<String, Pubkey>
}

#[wasm_bindgen]
impl ComponentIndex {
    #[wasm_bindgen(constructor)]
    pub fn new(id: &str) -> Self {
        ComponentIndex { 
            registry: Pubkey::from_str(id).unwrap(),
            index: bimap::BiHashMap::<String, Pubkey>::new() 
        }
    }

    pub fn insert_component_url(&mut self, schema:&str) {
        let pubkey = Pubkey::find_program_address(&[
            registry::constant::SEEDS_COMPONENTREGISTRATION,
            schema.as_bytes().as_ref(),
        ], &self.registry).0;

        self.index.insert(String::from(schema), pubkey);
    }

    pub fn get_component_pubkey(&self, schema:&str) -> Pubkey {
        self.index.get_by_left(&String::from(schema)).unwrap().clone()
    }

    pub fn get_component_url(&self, pubkey:&str) -> String {
        self.index.get_by_right(&Pubkey::from_str(pubkey).unwrap()).unwrap().clone()
    }
}

impl ComponentIndex {
    pub fn get_relevant_component_keys(&self) -> dominari::state::RelevantComponentKeys {
        dominari::state::RelevantComponentKeys {
            metadata: self.get_component_pubkey(&"metadata".to_string()),
            mapmeta: self.get_component_pubkey(&"mapmeta".to_string()),
            location: self.get_component_pubkey(&"location".to_string()),
            feature: self.get_component_pubkey(&"feature".to_string()),
            owner: self.get_component_pubkey(&"owner".to_string()),
            value: self.get_component_pubkey(&"value".to_string()),
            occupant: self.get_component_pubkey(&"occupant".to_string()),
            player_stats: self.get_component_pubkey(&"player_stats".to_string()),
            last_used: self.get_component_pubkey(&"last_used".to_string()),
            feature_rank: self.get_component_pubkey(&"feature_rank".to_string()),
            range: self.get_component_pubkey(&"range".to_string()),
            drop_table: self.get_component_pubkey(&"drop_table".to_string()),
            uses: self.get_component_pubkey(&"uses".to_string()),
            healing_power: self.get_component_pubkey(&"healing_power".to_string()),
            health: self.get_component_pubkey(&"health".to_string()),
            damage: self.get_component_pubkey(&"damage".to_string()),
            troop_class: self.get_component_pubkey(&"troop_class".to_string()),
            active: self.get_component_pubkey(&"active".to_string()),
            cost: self.get_component_pubkey(&"cost".to_string()),
            offchain_metadata: self.get_component_pubkey(&"offchain_metadata".to_string()),
        }
    }
}