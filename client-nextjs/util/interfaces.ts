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
    name: String,
    id: String
}

interface WasmTroop {
    name: String,
    id: String
}

//dominari-sdk::wasm_wrappers::WasmPlayer
export interface WasmPlayer {
    name: String,
    image: String,
    score: String,
    kills: String,
    cards: String[]
}