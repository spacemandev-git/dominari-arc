use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::*;
use std::collections::BTreeMap;

pub mod account;
pub mod context;
pub mod constant;
pub mod error;
pub mod event;
pub mod component;
pub mod state;

use account::*;
use context::*;
use constant::*;
use error::*;
use event::*;
use component::*;
use state::*;

use core_ds::account::MaxSize;
use core_ds::state::SerializedComponent;

declare_id!("3YdayPtujByJ1g1DWEUh7vpg78gZL49FWyD5rDGyof9T");

#[program]
pub mod dominari {
    use super::*;


    pub fn initialize(ctx: Context<Initialize>, component_keys: RelevantComponentKeys) -> Result<()> {
        ctx.accounts.config.authority = ctx.accounts.payer.key();
        ctx.accounts.config.components = component_keys;
        Ok(())
    }

    pub fn register_blueprint(ctx:Context<RegisterBlueprint>, name:String, components: BTreeMap<Pubkey, SerializedComponent>) -> Result<()> {
        ctx.accounts.blueprint.name = name;
        ctx.accounts.blueprint.components = components;
        Ok(())
    }

    pub fn system_init_map(ctx:Context<SystemInitMap>, entity_id:u64, max_x: u8, max_y: u8) -> Result<()> {
        let reference = &ctx.accounts.config.components;
        let config_seeds:&[&[u8]] = &[
            SEEDS_ABSIGNER,
            &[*ctx.bumps.get("config").unwrap()]
        ];
        let signer_seeds = &[config_seeds];

        let init_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::InitEntity{
                entity: ctx.accounts.map_entity.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                registry_instance: ctx.accounts.registry_instance.to_account_info(),
                registry_config: ctx.accounts.registry_config.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );

        let mut components: BTreeMap<Pubkey, SerializedComponent> = BTreeMap::new();
        // Map has Metadata and MapMeta Components
        let metadata_component = ComponentMetadata {
            name: format!("Map ({:#})", ctx.accounts.registry_instance.instance),
            entity_type: EntityType::Map,
            registry_instance: ctx.accounts.registry_instance.key(),
        }.try_to_vec().unwrap();
        components.insert(reference.metadata.key(), SerializedComponent { 
            max_size: ComponentMetadata::get_max_size(), 
            data:  metadata_component
        });

        let mapmeta_component = ComponentMapMeta {
            max_x,
            max_y,
        }.try_to_vec().unwrap();
        components.insert(reference.mapmeta.key(), SerializedComponent { 
            max_size: ComponentMapMeta::get_max_size(), 
            data: mapmeta_component 
        });

        // Mint Map Entity
        registry::cpi::init_entity(init_entity_ctx, entity_id, components)?;
        ctx.accounts.instance_index.map = entity_id; //ctx.accounts.map_entity.key();
        Ok(())
    }

    pub fn system_init_tile(ctx:Context<SystemInitTile>, entity_id:u64, x:u8, y:u8, cost:u64) -> Result<()> {
        // Tile can only be instanced by Admin
        // So we can trust in the input
        let reference = &ctx.accounts.config.components;

        // Tile has Metadata, Location, Feature, Occupant, Owner and Cost components
        let mut components: BTreeMap<Pubkey, SerializedComponent> = BTreeMap::new();
        let metadata = ComponentMetadata {
            name: format!("Tile ({x}, {y})"),
            entity_type: EntityType::Tile,
            registry_instance: ctx.accounts.registry_instance.key(),
        }.try_to_vec().unwrap();
        components.insert(reference.metadata.key(), SerializedComponent { 
            max_size: ComponentMetadata::get_max_size(),
            data: metadata
        });

        let location = ComponentLocation {
            x,
            y,
        }.try_to_vec().unwrap();
        components.insert(reference.location.key(), SerializedComponent { 
            max_size: ComponentLocation::get_max_size(),
            data: location
        });

        let feature = ComponentFeature {
            feature_id: None,
        }.try_to_vec().unwrap();
        components.insert(reference.feature.key(), SerializedComponent { 
            max_size: ComponentFeature::get_max_size(),
            data: feature
        });

        let occupant = ComponentOccupant {
            occupant_id: None,
        }.try_to_vec().unwrap();
        components.insert(reference.occupant.key(), SerializedComponent { 
            max_size: ComponentOccupant::get_max_size(),
            data: occupant
        });

        let owner = ComponentOwner {
            owner: Some(ctx.accounts.payer.key()),
            player: None,
        }.try_to_vec().unwrap();
        components.insert(reference.owner.key(), SerializedComponent { 
            max_size: ComponentOwner::get_max_size(),
            data: owner
        });

        let cost_component = ComponentCost {
            lamports: cost,
        }.try_to_vec().unwrap();
        components.insert(reference.cost.key(), SerializedComponent { 
            max_size: ComponentCost::get_max_size(),
            data: cost_component
        });

        let config_seeds:&[&[u8]] = &[
            SEEDS_ABSIGNER,
            &[*ctx.bumps.get("config").unwrap()]
        ];
        let signer_seeds = &[config_seeds];

        let init_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::InitEntity{
                entity: ctx.accounts.tile_entity.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                registry_instance: ctx.accounts.registry_instance.to_account_info(),
                registry_config: ctx.accounts.registry_config.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );

        registry::cpi::init_entity(init_entity_ctx, entity_id, components)?;
        ctx.accounts.instance_index.tiles.push(entity_id);
        Ok(())
    }
    
    pub fn system_init_feature(ctx:Context<SystemInitFeature>, entity_id: u64) -> Result<()> {
        // Check to make sure tile can be modified by payer
        let reference = &ctx.accounts.config.components;
        let tile_owner_component = ctx.accounts.tile_entity.components.get(&reference.owner).unwrap();
        let tile_owner:ComponentOwner = ComponentOwner::try_from_slice(&tile_owner_component.data.as_slice()).unwrap();
        
        if tile_owner.owner.unwrap().key() != ctx.accounts.payer.key() {
            return err!(ComponentErrors::InvalidOwner)
        }

        // TODO: Check Blueprint 'cost' component and transfer that fee to build the Feature

        // Create Feature entity
        let mut components: BTreeMap<Pubkey, SerializedComponent> = BTreeMap::new();
        // Feature has Metadata, Location, Owner, Active, and ..Blueprint Components
        let metadata_component = ComponentMetadata {
            name: ctx.accounts.blueprint.name.clone(),
            entity_type: EntityType::Feature,
            registry_instance: ctx.accounts.registry_instance.key(),
        }.try_to_vec().unwrap();
        components.insert(reference.metadata.key(), SerializedComponent { 
            max_size: ComponentMetadata::get_max_size(), 
            data:  metadata_component
        });
        // Just copy the Tile Location component
        let tile_location = ctx.accounts.tile_entity.components.get(&reference.location).unwrap();
        components.insert(reference.location.key(), tile_location.clone());
        
        let owner = ComponentOwner {
            owner: tile_owner.owner,
            player: None,
        }.try_to_vec().unwrap();
        components.insert(reference.owner.key(), SerializedComponent { 
            max_size: ComponentOwner::get_max_size(),
            data: owner
        });

        let active = ComponentActive {
            active: true
        }.try_to_vec().unwrap();
        components.insert(reference.active.key(), SerializedComponent { 
            max_size: ComponentActive::get_max_size(),
            data: active
        });

        components.extend(ctx.accounts.blueprint.components.clone());

        //msg!("System Registration Components: {:?}", ctx.accounts.action_bundle_registration.components);
        //msg!("Feature Components: {:?}", components);


        let config_seeds:&[&[u8]] = &[
            SEEDS_ABSIGNER,
            &[*ctx.bumps.get("config").unwrap()]
        ];
        let signer_seeds = &[config_seeds];

        let init_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::InitEntity{
                entity: ctx.accounts.feature_entity.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                registry_instance: ctx.accounts.registry_instance.to_account_info(),
                registry_config: ctx.accounts.registry_config.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );

        registry::cpi::init_entity(init_entity_ctx, entity_id, components)?;
        ctx.accounts.instance_index.features.push(entity_id);

        // Modify the Tile Entity with the new Feature
        let tile_feature_component = ctx.accounts.tile_entity.components.get(&reference.feature).unwrap();
        let mut tile_feature:ComponentFeature = ComponentFeature::try_from_slice(&tile_feature_component.data.as_slice()).unwrap();
        tile_feature.feature_id = Some(entity_id);
        let data = tile_feature.try_to_vec().unwrap();

        //msg!("{}", ctx.accounts.config.components.feature.key());

        let modify_tile_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::ModifyComponent {
                registry_config: ctx.accounts.registry_config.to_account_info(),
                entity: ctx.accounts.tile_entity.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );
        registry::cpi::req_modify_component(modify_tile_ctx, vec![(reference.feature.key(), data)])?;
        Ok(())
    }

    pub fn create_game_instance(ctx:Context<CreateGameInstance>, instance:u64, game_config: GameConfig) -> Result<()> {
        let config_seeds:&[&[u8]] = &[
            SEEDS_ABSIGNER,
            &[*ctx.bumps.get("config").unwrap()]
        ];
        let signer_seeds = &[config_seeds];

        // Instance the World
        let instance_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::InstanceRegistry {
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                registry_config: ctx.accounts.registry_config.to_account_info(),
                registry_instance: ctx.accounts.registry_instance.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                ab_signer: ctx.accounts.config.to_account_info(),
            },
            signer_seeds
        );

        registry::cpi::instance_registry(instance_ctx, instance)?;
        // Set up Instance Index
        ctx.accounts.instance_index.config = game_config; 
        ctx.accounts.instance_index.authority = ctx.accounts.payer.key();
        Ok(())
    }

    pub fn system_init_player(ctx:Context<SystemInitPlayer>, entity_id: u64, name:String, image: String ) -> Result <()> {
        let reference = &ctx.accounts.config.components;
        // Optional: Fail if too many players already in the instance
        if ctx.accounts.instance_index.config.max_players == ctx.accounts.instance_index.players.len() as u16 {
            return err!(DominariError::PlayerCountExceeded)
        }

        if name.len() > STRING_MAX_SIZE as usize || image.len() > STRING_MAX_SIZE as usize {
            return err!(ComponentErrors::StringTooLong)
        }

        // Create Player Entity
        // Player has: Metadata and Player Stats
        let mut components: BTreeMap<Pubkey, SerializedComponent> = BTreeMap::new();
        // Feature has Metadata, Location, Owner, Active, and ..Blueprint Components
        let metadata_component = ComponentMetadata {
            name: ctx.accounts.payer.key().to_string(),
            entity_type: EntityType::Player,
            registry_instance: ctx.accounts.registry_instance.key(),
        }.try_to_vec().unwrap();
        components.insert(reference.metadata.key(), SerializedComponent { 
            max_size: ComponentMetadata::get_max_size(), 
            data:  metadata_component
        });

        let player_stats_component = ComponentPlayerStats {
            name,
            image, 
            key: ctx.accounts.payer.key(),
            score: 0,
            kills: 0,
            // Give them Starting Card
            cards: ctx.accounts.instance_index.config.starting_cards.clone()
        }.try_to_vec().unwrap();
        components.insert(reference.player_stats.key(), SerializedComponent { 
            max_size: ComponentPlayerStats::get_max_size(), 
            data:  player_stats_component
        });

        let config_seeds:&[&[u8]] = &[
            SEEDS_ABSIGNER,
            &[*ctx.bumps.get("config").unwrap()]
        ];
        let signer_seeds = &[config_seeds];

        let init_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::InitEntity{
                entity: ctx.accounts.player_entity.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                registry_instance: ctx.accounts.registry_instance.to_account_info(),
                registry_config: ctx.accounts.registry_config.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );

        registry::cpi::init_entity(init_entity_ctx, entity_id, components)?;
        
        // Add player entity to instance index
        ctx.accounts.instance_index.players.push(entity_id);

        Ok(())
    }

    /**
     * Can only be called by a player that's in the game
     */
    pub fn change_game_state(ctx:Context<ChangeGameState>, game_state: PlayPhase) -> Result<()> {
        if !ctx.accounts.instance_index.players.contains(&ctx.accounts.player.entity_id) {
            return err!(DominariError::InvalidPlayer)
        }

        ctx.accounts.instance_index.play_phase = game_state.clone();

        emit!(GameStateChanged {
            instance: ctx.accounts.registry_instance.instance,
            player: ctx.accounts.player.entity_id,
            new_state: game_state
        });
        Ok(())
    }

    pub fn spawn_unit(ctx:Context<SpawnUnit>, unit_id: u64) -> Result<()> {
        let reference = &ctx.accounts.config.components;
        // Check if the game is paused
        if ctx.accounts.instance_index.play_phase != PlayPhase::Play {
            return err!(DominariError::GamePaused)
        }

        // Check player belongs to payer
        let player_stats_component = ctx.accounts.player.components.get(&reference.player_stats).unwrap();
        let mut player_stats = ComponentPlayerStats::try_from_slice(&player_stats_component.data.as_slice()).unwrap();
        if player_stats.key.key() != ctx.accounts.payer.key() {
            return err!(ComponentErrors::InvalidOwner)
        }

        // Check that the Tile is Empty
        let tile_occupant_component = ctx.accounts.tile.components.get(&reference.occupant).unwrap();
        let mut tile_occupant = ComponentOccupant::try_from_slice(&tile_occupant_component.data.as_slice()).unwrap();
        if tile_occupant.occupant_id.is_some() {
            return err!(ComponentErrors::TileOccupied)
        }

        // Check the Blueprint is in Player Hand
        let card_idx = player_stats.cards.iter().position(|&card| card.key() == ctx.accounts.unit_blueprint.key());

        if card_idx.is_none() {
            return err!(ComponentErrors::InvalidCard)
        }

        // Modify Player Hand to remove Blueprint
        player_stats.cards.swap_remove(card_idx.unwrap());

        // Create Unit Entity
        let mut components: BTreeMap<Pubkey, SerializedComponent> = BTreeMap::new();
        // Add Metadata, Owner, Location, Active + Blueprint components
        let metadata_component = ComponentMetadata {
            name: ctx.accounts.unit_blueprint.name.clone(),
            entity_type: EntityType::Unit,
            registry_instance: ctx.accounts.registry_instance.key()

        }.try_to_vec().unwrap();
        components.insert(reference.metadata.key(), SerializedComponent {
            max_size: ComponentMetadata::get_max_size(),
            data: metadata_component
        });
        let owner_component = ComponentOwner {  
            owner: Some(ctx.accounts.payer.key()),
            player: Some(ctx.accounts.player.entity_id)
        }.try_to_vec().unwrap();
        components.insert(reference.owner.key(), SerializedComponent {
            max_size: ComponentOwner::get_max_size(),
            data: owner_component
        });
        let active_component = ComponentActive {
            active: true
        }.try_to_vec().unwrap();
        components.insert(reference.active.key(), SerializedComponent{
            max_size: ComponentActive::get_max_size(),
            data: active_component
        });

        // Clone the Tile's location component to the Unit
        components.insert(
            reference.location.key(),
            ctx.accounts.tile.components.get(&reference.location).unwrap().clone()
        );
        
        components.extend(ctx.accounts.unit_blueprint.components.clone());
        
        let config_seeds:&[&[u8]] = &[
            SEEDS_ABSIGNER,
            &[*ctx.bumps.get("config").unwrap()]
        ];
        let signer_seeds = &[config_seeds];

        let init_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::InitEntity{
                entity: ctx.accounts.unit.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                registry_instance: ctx.accounts.registry_instance.to_account_info(),
                registry_config: ctx.accounts.registry_config.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );

        registry::cpi::init_entity(init_entity_ctx, unit_id, components)?;
        // Add the new Unit Entity to Instance index
        ctx.accounts.instance_index.units.push(unit_id);

        // Modify Tile to point to Unit Entity
        tile_occupant.occupant_id = Some(unit_id);
        let data = tile_occupant.try_to_vec().unwrap();
        let modify_tile_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::ModifyComponent {
                registry_config: ctx.accounts.registry_config.to_account_info(),
                entity: ctx.accounts.tile.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );
        registry::cpi::req_modify_component(modify_tile_ctx, vec![(ctx.accounts.config.components.occupant.key(), data)])?;

        // Update Player Stats to no longer have that card
        let data = player_stats.try_to_vec().unwrap();
        let modify_player_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::ModifyComponent {
                registry_config: ctx.accounts.registry_config.to_account_info(),
                entity: ctx.accounts.player.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );
        registry::cpi::req_modify_component(modify_player_ctx, vec![(ctx.accounts.config.components.player_stats.key(), data)])?;

        emit!(NewUnitSpawned {
            instance: ctx.accounts.registry_instance.instance,
            tile: ctx.accounts.tile.entity_id,
            player: ctx.accounts.player.entity_id,
            unit: unit_id
        });

        Ok(())
    }

    pub fn move_unit(ctx:Context<MoveUnit>) -> Result<()> {
        let reference = &ctx.accounts.config.components;
        // Check if the game is paused
        if ctx.accounts.instance_index.play_phase != PlayPhase::Play {
            return err!(DominariError::GamePaused)
        }

        // From.Occupant must be Unit
        let from_occupant_component = ctx.accounts.from.components.get(&reference.occupant).unwrap();
        let mut from_occupant = ComponentOccupant::try_from_slice(&from_occupant_component.data.as_slice()).unwrap();
        if from_occupant.occupant_id.unwrap() != ctx.accounts.unit.entity_id {
            return err!(ComponentErrors::InvalidUnit)
        }
        
        // Unit must be active
        let active_component = ctx.accounts.unit.components.get(&reference.active).unwrap();
        let active = ComponentActive::try_from_slice(&active_component.data.as_slice()).unwrap();
        if active.active == false {
            return err!(ComponentErrors::UnitDead)
        }

        // To.Occupant must be Empty
        let to_occupant_component = ctx.accounts.to.components.get(&reference.occupant).unwrap();
        let mut to_occupant = ComponentOccupant::try_from_slice(&to_occupant_component.data.as_slice()).unwrap();
        if to_occupant.occupant_id.is_some() {
            return err!(ComponentErrors::TileOccupied)
        }

        // Unit must be Owned by Player        
        let unit_owner_component = ctx.accounts.unit.components.get(&reference.owner).unwrap();
        let unit_owner = ComponentOwner::try_from_slice(&unit_owner_component.data.as_slice()).unwrap();
        if unit_owner.owner.unwrap() != ctx.accounts.payer.key() {
            return err!(ComponentErrors::InvalidOwner)
        }
        
        // Unit must be recovered from last used
        let clock = Clock::get().unwrap();
        let unit_last_used_component = ctx.accounts.unit.components.get(&reference.last_used).unwrap();
        let mut unit_last_used = ComponentLastUsed::try_from_slice(&unit_last_used_component.data.as_slice()).unwrap();
        if unit_last_used.last_used != 0 && (unit_last_used.last_used + unit_last_used.recovery) >= clock.slot {
            return err!(ComponentErrors::UnitRecovering)
        }

        // Distance between From and To must be < Unit's Movement
        let from_location_c = ctx.accounts.from.components.get(&reference.location).unwrap();
        let from_location = ComponentLocation::try_from_slice(&from_location_c.data.as_slice()).unwrap();

        let to_location_c = ctx.accounts.to.components.get(&reference.location).unwrap();
        let to_location = ComponentLocation::try_from_slice(&to_location_c.data.as_slice()).unwrap();
        
        let distance:f64 = (((to_location.x as f64 - from_location.x as f64).powf(2_f64) + (to_location.y as f64 - from_location.y as f64).powf(2_f64)) as f64).sqrt();
        let unit_range_component = ctx.accounts.unit.components.get(&reference.range).unwrap();
        let unit_range = ComponentRange::try_from_slice(&unit_range_component.data.as_slice()).unwrap();
        if unit_range.movement < distance as u8 {
            return err!(ComponentErrors::UnitLacksMovement)
        }

        let config_seeds:&[&[u8]] = &[
            SEEDS_ABSIGNER,
            &[*ctx.bumps.get("config").unwrap()]
        ];
        let signer_seeds = &[config_seeds];

        // Modify Unit's last_used & location
        unit_last_used.last_used = clock.slot;

        let modify_unit_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::ModifyComponent {
                registry_config: ctx.accounts.registry_config.to_account_info(),
                entity: ctx.accounts.unit.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );
        registry::cpi::req_modify_component(modify_unit_ctx, vec![
                (ctx.accounts.config.components.last_used.key(), unit_last_used.try_to_vec().unwrap()),
                (ctx.accounts.config.components.location.key(),  to_location_c.data.clone())    
            ])?;

        // Modify From Occupant to be None
        from_occupant.occupant_id = None;
        let modify_from_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::ModifyComponent {
                registry_config: ctx.accounts.registry_config.to_account_info(),
                entity: ctx.accounts.from.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );
        registry::cpi::req_modify_component(modify_from_ctx, vec![(ctx.accounts.config.components.occupant.key(),from_occupant.try_to_vec().unwrap())])?;    

        // Modify To Occupant to be Unit
        to_occupant.occupant_id = Some(ctx.accounts.unit.entity_id);
        let modify_to_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::ModifyComponent {
                registry_config: ctx.accounts.registry_config.to_account_info(),
                entity: ctx.accounts.to.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );
        registry::cpi::req_modify_component(modify_to_ctx, vec![(ctx.accounts.config.components.occupant.key(), to_occupant.try_to_vec().unwrap())])?;
        // Emit Troop Movement
        emit!(TroopMovement {
            instance: ctx.accounts.registry_instance.instance,
            from: ctx.accounts.from.entity_id,
            to: ctx.accounts.to.entity_id,
            unit: ctx.accounts.unit.entity_id
        });

        Ok(())
    }
    
    pub fn attack_tile(ctx:Context<AttackTile>) -> Result<()> {
        // Attacker could be Feature or Unit (just needs Damage Component)
        let attacker = &ctx.accounts.attacker;
        let defender = &ctx.accounts.defender;
        let reference = &ctx.accounts.config.components;

        // Check if the game is paused
        if ctx.accounts.instance_index.play_phase != PlayPhase::Play {
            return err!(DominariError::GamePaused)
        }
        
        // Check that attacker is owned by Payer
        let attacker_owner_c = attacker.components.get(&reference.owner).unwrap();
        let attacker_owner = ComponentOwner::try_from_slice(&attacker_owner_c.data.as_slice()).unwrap();
        if attacker_owner.owner != Some(ctx.accounts.payer.key()) {
            return err!(ComponentErrors::InvalidOwner)
        }
        
        // Check that attacker is active
        let attacker_active_c = attacker.components.get(&reference.active).unwrap();
        let attacker_active = ComponentActive::try_from_slice(&attacker_active_c.data.as_slice()).unwrap();
        if attacker_active.active == false {
            return err!(ComponentErrors::UnitDead)
        }

        // Check that defender is NOT owned by Payer
        let defender_owner_c = defender.components.get(&reference.owner).unwrap();
        let defender_owner = ComponentOwner::try_from_slice(&defender_owner_c.data.as_slice()).unwrap();
        if defender_owner.player == attacker_owner.player {
            return err!(ComponentErrors::FriendlyFire)
        }

        // Check attacker has damage component
        let attacker_damage_c = attacker.components.get(&reference.damage).unwrap();
        let attacker_damage = ComponentDamage::try_from_slice(&attacker_damage_c.data.as_slice()).unwrap();

        // Check defender is active and has health component
        let defender_active_c = defender.components.get(&reference.active).unwrap();
        let mut defender_active = ComponentActive::try_from_slice(&defender_active_c.data.as_slice()).unwrap();
        if defender_active.active == false {
            return err!(ComponentErrors::UnitDead)
        }
        let defender_health_c = defender.components.get(&reference.health);
        if defender_health_c.is_none() {
            return err!(ComponentErrors::NoHealthComponent)
        }
        let mut defender_health = ComponentHealth::try_from_slice(&defender_health_c.unwrap().data.as_slice()).unwrap();

        // Defender must be in Range of Attacker
        let attacker_location_c = attacker.components.get(&reference.location).unwrap();
        let attacker_location = ComponentLocation::try_from_slice(&attacker_location_c.data.as_slice()).unwrap();
        let defender_location_c = defender.components.get(&reference.location).unwrap();
        let defender_location = ComponentLocation::try_from_slice(&defender_location_c.data.as_slice()).unwrap();
        
        let distance:f64 = (((defender_location.x as f64 - attacker_location.x as f64).powf(2_f64) + (defender_location.y as f64 - attacker_location.y as f64 ).powf(2_f64)) as f64).sqrt();
        let attacker_range_c = attacker.components.get(&reference.range).unwrap();
        let attacker_range = ComponentRange::try_from_slice(&attacker_range_c.data.as_slice()).unwrap();
        if distance as u8 > attacker_range.attack_range {
            return err!(ComponentErrors::OutOfRange)
        }

        // Check attacker last used isn't violated
        let clock = Clock::get().unwrap();
        let attacker_last_used_c = attacker.components.get(&reference.last_used).unwrap();
        let mut attacker_last_used = ComponentLastUsed::try_from_slice(&attacker_last_used_c.data.as_slice()).unwrap();
        if attacker_last_used.last_used != 0 && (attacker_last_used.last_used + attacker_last_used.recovery) >= clock.slot {
            return err!(ComponentErrors::UnitRecovering)
        }
        attacker_last_used.last_used = clock.slot;        
        
        let config_seeds:&[&[u8]] = &[
            SEEDS_ABSIGNER,
            &[*ctx.bumps.get("config").unwrap()]
        ];
        let signer_seeds = &[config_seeds];

        // Modify attacker last used
        let modify_attacker_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::ModifyComponent {
                registry_config: ctx.accounts.registry_config.to_account_info(),
                entity: ctx.accounts.attacker.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );
        registry::cpi::req_modify_component(modify_attacker_ctx, vec![(reference.last_used.key(),attacker_last_used.try_to_vec().unwrap())])?;

        // Roll Damage for Attacker, apply modifiers 
        let mut dmg = get_random_u64(attacker_damage.max_damage); 
        
        // check if defender is Feature, if not, look for it's TroopClass
        let defender_metadata_c = defender.components.get(&reference.metadata).unwrap();
        let defender_metadata = ComponentMetadata::try_from_slice(&defender_metadata_c.data.as_slice()).unwrap();

        if defender_metadata.entity_type == EntityType::Feature {
            dmg += attacker_damage.bonus_feature as u64;
        } else {
            let defender_troop_class_c = defender.components.get(&reference.troop_class).unwrap();
            let defender_troop_class = ComponentTroopClass::try_from_slice(&defender_troop_class_c.data.as_slice()).unwrap();
            match defender_troop_class.class {
                TroopClass::Aircraft => dmg += attacker_damage.bonus_aircraft as u64,
                TroopClass::Infantry => dmg += attacker_damage.bonus_infantry as u64,
                TroopClass::Armor => dmg += attacker_damage.bonus_armor as u64,
            }
        }

        if dmg < attacker_damage.min_damage {
            dmg = attacker_damage.min_damage;
        }

        if dmg >= defender_health.health {
            defender_health.health = 0;
            defender_active.active = false;

            // Modify the defending tile to remove the defender
            let defending_tile = &ctx.accounts.defending_tile;
            // Require Defender Location and Defending Tile Location are the same
            let defending_tile_loc_c = defending_tile.components.get(&reference.location).unwrap();
            let defending_tile_loc = ComponentLocation::try_from_slice(&defending_tile_loc_c.data.as_slice()).unwrap();
            if defending_tile_loc.x != defender_location.x && defending_tile_loc.y != defender_location.y {
                return err!(ComponentErrors::InvalidLocation)
            }

            if defender_metadata.entity_type == EntityType::Feature {
                let tile_feature_c = defending_tile.components.get(&reference.feature).unwrap();
                let mut tile_feature = ComponentFeature::try_from_slice(&tile_feature_c.data.as_slice()).unwrap();
                tile_feature.feature_id = None;
                let modify_tile_ctx = CpiContext::new_with_signer(
                    ctx.accounts.registry_program.to_account_info(),
                    registry::cpi::accounts::ModifyComponent {
                        registry_config: ctx.accounts.registry_config.to_account_info(),
                        entity: ctx.accounts.defending_tile.to_account_info(),
                        action_bundle: ctx.accounts.config.to_account_info(),
                        action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                        core_ds: ctx.accounts.coreds.to_account_info(),
                    },
                    signer_seeds
                );
                registry::cpi::req_modify_component(modify_tile_ctx, vec![(reference.feature.key(), tile_feature.try_to_vec().unwrap())])?;

            } else {
                let tile_occupant_c = defending_tile.components.get(&reference.occupant).unwrap();
                let mut tile_occupant = ComponentOccupant::try_from_slice(&tile_occupant_c.data.as_slice()).unwrap();
                tile_occupant.occupant_id = None;
                let modify_tile_ctx = CpiContext::new_with_signer(
                    ctx.accounts.registry_program.to_account_info(),
                    registry::cpi::accounts::ModifyComponent {
                        registry_config: ctx.accounts.registry_config.to_account_info(),
                        entity: ctx.accounts.defending_tile.to_account_info(),
                        action_bundle: ctx.accounts.config.to_account_info(),
                        action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                        core_ds: ctx.accounts.coreds.to_account_info(),
                    },
                    signer_seeds
                );
                registry::cpi::req_modify_component(modify_tile_ctx, vec![(reference.occupant.key(),tile_occupant.try_to_vec().unwrap())])?;
            }
        } else {
            defender_health.health -= dmg;
        }

        // Modify defender health
            // If defender health at 0, Modify active as well
        let modify_defender_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::ModifyComponent {
                registry_config: ctx.accounts.registry_config.to_account_info(),
                entity: ctx.accounts.defender.to_account_info(),
                action_bundle: ctx.accounts.config.to_account_info(),
                action_bundle_registration: ctx.accounts.ab_registration.to_account_info(),
                core_ds: ctx.accounts.coreds.to_account_info(),
            },
            signer_seeds
        );
        registry::cpi::req_modify_component(modify_defender_ctx, vec![
                (reference.health.key(), defender_health.try_to_vec().unwrap()),
                (reference.active.key(), defender_active.try_to_vec().unwrap()),
            ])?;
    

        emit!(TileAttacked{
            instance: ctx.accounts.registry_instance.instance,
            attacker: attacker.entity_id,
            defender: defender.entity_id,
            defending_tile: ctx.accounts.defending_tile.entity_id,
            damage: dmg
        });

        Ok(())
    }
    //pub fn modify_unit(ctx:Context<ModUnit>) -> Result<()> {}

    //pub fn build_feature(ctx:Context<BuildFeature>) -> Result<()> {}
    //pub fn use_[feature](ctx:Context<UseFeature>) -> Result<()> {}

    // Pass in multiple entities through remaining accounts; will iterate and remove them if they are marked inactive
    //pub fn reclaim_entity_sol(ctx:Context<ReclaimSol>) -> Result<()> {}
    pub fn reclaim_sol(ctx:Context<ReclaimSol>) -> Result<()> {
        // Can Close *Instance Index* if it's Empty
        // Can Close Map, Tiles, Features if Leader
        // Can Close Troops if Troop Owner
        Ok(())
    }
}

pub fn get_random_u64(max: u64) -> u64 {
    let clock = Clock::get().unwrap();
    let slice = &hash(&clock.slot.to_be_bytes()).to_bytes()[0..8];
    let num: u64 = u64::from_be_bytes(slice.try_into().unwrap());
    let target = num/(u64::MAX/max);
    return target;
}