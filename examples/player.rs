use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use serde_nbt::from_reader;
use std::fs::File;

// reads Minecraft's <player>.dat files:
// https://minecraft.fandom.com/wiki/Player.dat_format

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    #[serde(rename = "DataVersion")]
    data_version: i32,
    #[serde(rename = "playerGameType")]
    player_game_type: i32,
    #[serde(rename = "previousPlayerGameType")]
    previous_player_game_type: i32,
    #[serde(rename = "Score")]
    score: i32,
    // dimension:
    #[serde(rename = "SelectedItemSlot")]
    selected_item_slot: i32,
    // selected_item
    // spawn_dimension
    #[serde(rename = "SpawnX")]
    spawn_x: Option<i32>,
    #[serde(rename = "SpawnY")]
    spawn_y: Option<i32>,
    #[serde(rename = "SpawnZ")]
    spawn_z: Option<i32>,
    #[serde(rename = "SpawnForced")]
    spawn_forced: Option<bool>,
    #[serde(rename = "SleepTimer")]
    sleep_timer: i16,
    #[serde(rename = "foodLevel")]
    food_level: i32,
    #[serde(rename = "foodExhaustionLevel")]
    food_exhaustion_level: f32,
    #[serde(rename = "foodSaturationLevel")]
    food_saturation_level: f32,
    #[serde(rename = "foodTickTimer")]
    food_tick_timer: i32,
    #[serde(rename = "XpLevel")]
    xp_level: i32,
    #[serde(rename = "XpP")]
    xp_p: f32,
    #[serde(rename = "XpTotal")]
    xp_total: i32,
    #[serde(rename = "XpSeed")]
    xp_seed: i32,
    // inventory
    // ender_items
    // abilities
    // entered_nether_position
    // root_vehicle
    // shoulder_entity_left
    // shoulder_entity_right
    #[serde(rename = "seenCredits")]
    seen_credits: bool,
    // recipe_book
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];

    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);

    let player: Player = from_reader(decoder)?;
    println!("{:?}", player);

    Ok(())
}
