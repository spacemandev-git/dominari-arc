use dominari_sdk::gamestate::GameState;

#[tokio::main]
async fn main() {
    let mut gamestate = GameState::new(
        "http://localhost:8899",
        "3YdayPtujByJ1g1DWEUh7vpg78gZL49FWyD5rDGyof9T",
        "H5mieGWWK6qukHoNzbR6ysLxReeQC4JHZcNM6JkPQnm3",
        13712002795722475028_u64,
    );
    gamestate.load_state().await;
    println!("Tile ID: {:?}", gamestate.get_tile_id(0, 0));
    let tile_id:u64 = gamestate.get_tile_id(0, 0).parse().unwrap();
    let tile_entity = gamestate.get_entity(tile_id).unwrap();
    //println!("Tile Components: {:?}", tile_entity.components);
    let tile_feature = gamestate.get_entity_feature(&tile_id).unwrap();
    println!("Feature Component: {:?}", tile_feature);
}
