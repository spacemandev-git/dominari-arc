use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BlueprintConfig {
    pub metadata: Option<dominari::component::ComponentMetadata>, // Uses Pubkey
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

#[derive(Deserialize, Debug)]
pub struct ComponentDropTableWASM {
    pub drop_table: Vec<String>
}