use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use registry::constant::SEEDS_COMPONENTREGISTRATION;
use wasm_bindgen::prelude::*;
use std::str::FromStr;
use anchor_lang::system_program::ID as system_program;


#[wasm_bindgen]
#[derive(Default)]
pub struct Registry {
    pub program_id: Pubkey
}

#[wasm_bindgen]
impl Registry {
    #[wasm_bindgen(constructor)]
    pub fn new(id:&str) -> Self {
        console_error_panic_hook::set_once();
        Registry { 
            program_id: Pubkey::from_str(id).unwrap()
        }
    }
    
    // Initialize Registry (Creates Registry Config)
    pub fn initialize(&self, payer_str: &str) -> JsValue {
        let payer = Pubkey::from_str(payer_str).unwrap();
        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &self.program_id).0;

        let accounts = registry::accounts::Initialize {
            payer,
            system_program,
            registry_config,
        };

        let ix = Instruction {
            program_id: self.program_id,
            accounts: accounts.to_account_metas(None),
            data: registry::instruction::Initialize {
                core_ds: core_ds::id(),
            }.data(),
        };

        serde_wasm_bindgen::to_value(&ix).unwrap()
    }

    // Register Components
    pub fn register_component(&self, payer_str: &str, schema: &str,) -> JsValue {
        let payer = Pubkey::from_str(payer_str).unwrap();
        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &self.program_id).0;

        let component = Pubkey::find_program_address(&[
            SEEDS_COMPONENTREGISTRATION,
            schema.as_bytes().as_ref(),
        ], &self.program_id).0;

        let ix = Instruction {
            program_id: self.program_id,
            accounts: registry::accounts::RegisterComponent {
                payer,
                system_program,
                component,
                registry_config,
            }.to_account_metas(None),
            data: registry::instruction::RegisterComponent {
                schema: String::from_str(schema).unwrap(),
            }.data()
        };
        serde_wasm_bindgen::to_value(&ix).unwrap()
    }
    
    pub fn register_action_bundle(&self, payer:&str, ab:&str) -> JsValue {
        let payer = Pubkey::from_str(payer).unwrap();
        let ab = Pubkey::from_str(ab).unwrap();

        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
        ], &ab).0;

        let action_bundle_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab_signer.to_bytes().as_ref()
        ], &self.program_id).0;

        let ix = Instruction {
            program_id: self.program_id,
            accounts: registry::accounts::RegisterAB {
                payer,
                system_program,
                action_bundle_registration,
                action_bundle: ab_signer,
            }.to_account_metas(None),
            data: registry::instruction::RegisterActionBundle {}.data()
        };
        serde_wasm_bindgen::to_value(&ix).unwrap()
    }

    pub fn add_components_for_action_bundle(&self, payer: &str, ab:&str, components:JsValue) -> JsValue {
        let components_str:Vec<String> = serde_wasm_bindgen::from_value(components).unwrap();

        let components:Vec<Pubkey> = components_str.iter().map(|comp_str| {
            Pubkey::find_program_address(&[
                SEEDS_COMPONENTREGISTRATION,
                comp_str.as_bytes().as_ref(),
            ], &self.program_id).0    
        }).collect();

        let payer = Pubkey::from_str(payer).unwrap();
        let ab = Pubkey::from_str(ab).unwrap();
        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
        ], &ab).0;

        let action_bundle_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab_signer.to_bytes().as_ref()
        ], &self.program_id).0;

        let config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &self.program_id).0;

        
        let ix = Instruction {
            program_id: self.program_id,
            accounts: registry::accounts::AddComponentsToActionBundleRegistration {
                payer,
                system_program,
                action_bundle_registration,
                config,
            }.to_account_metas(None),
            data: registry::instruction::AddComponentsToActionBundleRegistration {
                components                
            }.data()
        };
        serde_wasm_bindgen::to_value(&ix).unwrap()
    }
}

/*
1. Deploy 3 Programs
2. Initalize Registry
3. Register Components with Registry
4. Register Action Bundle
5. Register AB w/ Components

Dominari
6. Generate Game Instance -> Instance Registry via CPI
 */