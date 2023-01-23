use dominari_sdk::gamestate::GameState;
use anchor_lang::prelude::*;
use clockwork_sdk::ThreadProgram;


async fn main1() {
    let mut gamestate = GameState::new(
        "http://localhost:8899",
        "3YdayPtujByJ1g1DWEUh7vpg78gZL49FWyD5rDGyof9T",
        "H5mieGWWK6qukHoNzbR6ysLxReeQC4JHZcNM6JkPQnm3",
        11058116549897703620_u64,
    );
    gamestate.load_state().await;
    let defender = gamestate.get_entity(2681892547010935764_u64).unwrap();
    println!("Defender Components: {:?}", defender.components);
    let defender_active = gamestate.get_entity_active(&2681892547010935764_u64).unwrap();
    println!("Active Component: {:?}", defender_active);
}

#[tokio::main]
async fn main(){
    const instance:u64 = 2553213103686576020;
    const SEEDS_INSTANCEINDEX:&[u8;14] = b"instance_index";


    let registry_instance = Pubkey::find_program_address(&[
        core_ds::constant::SEEDS_REGISTRYINSTANCE_PREFIX,
        registry::id().to_bytes().as_ref(),
        instance.to_be_bytes().as_ref()
    ], &core_ds::id()).0;

    let instance_index = Pubkey::find_program_address(&[
        SEEDS_INSTANCEINDEX,
        registry_instance.to_bytes().as_ref(),
    ], &dominari::ID).0;

    let thread = Pubkey::find_program_address(&[
        b"thread",
       instance_index.key().as_ref(), 
       instance.to_string().as_bytes(),
    ], &ThreadProgram::id()).0;

    println!("{}", thread.to_string());
}