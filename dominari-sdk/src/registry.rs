use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
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
    pub fn initialize(&self, payer_str: &str) -> Instruction {
        let payer = Pubkey::from_str(payer_str).unwrap();
        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &self.program_id).0;

        let accounts = registry::accounts::Initialize {
            payer,
            system_program,
            registry_config,
        };

        Instruction {
            program_id: self.program_id,
            accounts: accounts.to_account_metas(None),
            data: registry::instruction::Initalize {
                core_ds: core_ds::id(),
            }.data(),
        }
    }

    // Register Components
    pub fn register_component(&self, schema: &str, payer_str: &str) -> Instruction {
        let payer = Pubkey::from_str(payer_str).unwrap();
        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &self.program_id).0;

        let component = Pubkey::find_program_address(&[
            schema.as_bytes().as_ref(),
        ], &self.program_id).0;

        Instruction {
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
        }
    }
    
    pub fn register_action_bundle(&self, payer:&str, ab:&str) -> Instruction {
        let payer = Pubkey::from_str(payer).unwrap();
        let ab = Pubkey::from_str(ab).unwrap();

        let action_bundle_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab.to_bytes().as_ref()
        ], &self.program_id).0;

        let action_bundle = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
            ab.to_bytes().as_ref()
        ], &self.program_id).0;

        Instruction {
            program_id: self.program_id,
            accounts: registry::accounts::RegisterAB {
                payer,
                system_program,
                action_bundle_registration,
                action_bundle,
            }.to_account_metas(None),
            data: registry::instruction::RegisterActionBundle {}.data()
        }
    }

    pub fn add_components_for_action_bundle(&self, payer: &str, ab:&str, components:JsValue) -> Instruction {
        let components_str:Vec<String> = serde_wasm_bindgen::from_value(components).unwrap();
        let components:Vec<Pubkey> = components_str.iter().map(|comp_str| {
            Pubkey::from_str(comp_str.as_str()).unwrap()
        }).collect();

        let payer = Pubkey::from_str(payer).unwrap();
        let ab = Pubkey::from_str(ab).unwrap();

        let action_bundle_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab.to_bytes().as_ref()
        ], &self.program_id).0;

        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
            ab.to_bytes().as_ref()
        ], &self.program_id).0;

        
        Instruction {
            program_id: self.program_id,
            accounts: registry::accounts::AddComponentsToActionBundleRegistration {
                payer,
                system_program,
                action_bundle_registration,
                ab_signer,
            }.to_account_metas(None),
            data: registry::instruction::AddComponentsToActionBundleRegistration {
                components                
            }.data()
        }
    }

    pub fn add_instance_for_action_bundle(&self, payer: &str, ab:&str, instance:u64) -> Instruction {
        let payer = Pubkey::from_str(payer).unwrap();
        let ab = Pubkey::from_str(ab).unwrap();

        let action_bundle_registration = Pubkey::find_program_address(&[
            registry::constant::SEEDS_ACTIONBUNDLEREGISTRATION,
            ab.to_bytes().as_ref()
        ], &self.program_id).0;

        let ab_signer = Pubkey::find_program_address(&[
            dominari::constant::SEEDS_ABSIGNER,
            ab.to_bytes().as_ref()
        ], &self.program_id).0;

        
        Instruction {
            program_id: self.program_id,
            accounts: registry::accounts::AddInstancesToActionBundleRegistration {
                payer,
                system_program,
                action_bundle_registration,
                ab_signer,
            }.to_account_metas(None),
            data: registry::instruction::AddInstancesToActionBundleRegistration {
                instances: vec![instance]                
            }.data()
        }
    }

    // Instance Registry
    pub fn instance_registry(&self, payer_str: &str, instance:u64) -> Instruction {
        let payer = Pubkey::from_str(payer_str).unwrap();
        let registry_config = Pubkey::find_program_address(&[
            registry::constant::SEEDS_REGISTRYSIGNER,
        ], &self.program_id).0;


        let registry_instance = Pubkey::find_program_address(&[
            core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX,
            &self.program_id.as_ref(),
            instance.to_be_bytes().as_ref()
        ], &core_ds::id()).0;

        let instance_authority = Pubkey::find_program_address(&[
            registry::constant::SEEDS_INSTANCEAUTHORITY,
            registry_instance.to_bytes().as_ref()
        ], &self.program_id).0;

        Instruction {
            program_id: self.program_id,
            accounts: registry::accounts::InstanceRegistry {
                payer,
                system_program,
                registry_config, // Should be a CPI Signer
                registry_instance,
                instance_authority,
                core_ds: core_ds::id(),
            }.to_account_metas(Some(true)), // This will CPI into Universe program, so some of these accounts are signers
            data: registry::instruction::InstanceRegistry {
                instance
            }.data()
        }
    }

}

/*
1. Deploy 3 Programs
2. Initalize Registry
3. Register Components with Registry
4. Register Action Bundle
5. Register AB w/ Components

Dominari
6. Generate Game Instance -> Add instance to AB Registration & Create New Instance
 */