use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use serde_nbt::from_reader;
use std::fs::File;

// reads Minecraft's <player>.dat files:
// https://minecraft.fandom.com/wiki/Player.dat_format

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    // player related tags
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

    // mob related tags
    #[serde(rename = "AbsorptionAmount")]
    absorption_amount: f32,
    // active_effects
    // attributes
    // brain
    // death_loot_table: Option<>
    #[serde(rename = "DeathLootTableSeed")]
    death_loot_table_seed: Option<i64>,
    #[serde(rename = "DeathTime")]
    death_time: i16,
    #[serde(rename = "FallFlying")]
    fall_flying: bool,
    #[serde(rename = "Health")]
    health: f32,
    #[serde(rename = "HurtByTimestamp")]
    hurt_by_timestamp: i32,
    #[serde(rename = "HurtTime")]
    hurt_time: i16,
    // hand_items
    // #[serde(rename = "LeftHanded")]
    // left_handed: bool,
    // #[serde(rename = "NoAI")]
    // no_ai: bool,
    #[serde(rename = "SleepingX")]
    sleeping_x: Option<i32>,
    #[serde(rename = "SleepingY")]
    sleeping_y: Option<i32>,
    #[serde(rename = "SleepingZ")]
    sleeping_z: Option<i32>,
    // team
    // #[serde(rename = "TicksFrozen")]
    // ticks_frozen: i32,

    // entity related tags
    #[serde(rename = "Air")]
    air: i16,
    #[serde(rename = "FallDistance")]
    fall_distance: f32,
    #[serde(rename = "Fire")]
    fire: i16,
    // #[serde(rename = "Glowing")]
    // glowing: bool,
    #[serde(rename = "Invulnerable")]
    invulnerable: bool,
    // motion:
    // #[serde(rename = "NoGravity")]
    // no_gravity: bool,
    #[serde(rename = "OnGround")]
    on_ground: bool,
    // passengers
    #[serde(rename = "PortalCooldown")]
    portal_cooldown: i32,
    // pos
    // rotation
    #[serde(rename = "Silent")]
    silent: Option<bool>,
    // tags
    // uuid
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
