use dominari_sdk::gamestate::GameState;

#[tokio::main]
async fn main() {
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

