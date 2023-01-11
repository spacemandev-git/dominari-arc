export interface ConfigFileInterface {
    config: GameConfig,
    map: MapConfig
}

//dominari::state::GameConfig
interface GameConfig {
    max_players: number, //u16
    starting_cards: string[] //Vec<Pubkey>    
}

interface MapConfig {
    cost_per_tile: bigint, //u64
    mapmeta: MapMeta,
    features: Feature[]
}

//dominari::component::ComponentMapMeta
interface MapMeta {
    max_x: number, //u8
    max_y: number, //u8
}

interface Feature {
    x: number,
    y: number,
    feature: string, //blueprint name
}

//dominari-sdk::wasm_wrappers::WasmTile
export interface WasmTile {
    x: number,
    y: number,
    feature: WasmFeature,
    troop: WasmTroop,
}

interface WasmFeature {
    name: string,
    id: string
}

interface WasmTroop {
    name: string,
    id: string,
    troop_owner_player_id: string,
    troop_owner_player_key: string,
    min_damage: string,
    max_damage: string,
    bonus_infantry: string,
    bonus_armor: string,
    bonus_aircraft: string,
    bonus_feature: string,
    health: string,
    class: any, // Enum that'll end up as an object key
    movement: number,
    attack_range: number,
    last_used: string,
    recovery: string,
    value: string
}

//dominari-sdk::wasm_wrappers::WasmPlayer
export interface WasmPlayer {
    id: string,
    name: string,
    image: string,
    score: string,
    kills: string,
    cards: string[]
}


export enum NavEnum {
    Settings,
    Map
}

export interface Blueprints {
    [key: string]: any
}

export type PlayPauseState = "Lobby" | "Build" | "Play" | "Paused" | "Finished";