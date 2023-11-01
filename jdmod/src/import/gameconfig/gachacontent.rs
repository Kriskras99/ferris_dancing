//! # Gacha Config
//! Import the gacha config if it does not exist in the mod yet
use std::fs::File;

use anyhow::Error;
use ubiart_toolkit::json_types;

use crate::types::{gameconfig::gachacontent::GachaConfig, ImportState};

/// Import the gacha config if it does not exist in the mod yet
pub fn import_v18v22(
    is: &ImportState<'_>,
    gachaconfig: &json_types::GachaConfig,
) -> Result<(), Error> {
    let gacha_config_path = is.dirs.config().join("gacha.json");

    if !gacha_config_path.exists() {
        println!("Importing gacha config...");

        let new_gacha_config = GachaConfig {
            price: gachaconfig.price,
            force_high_rarity_reward_count: gachaconfig.force_high_rarity_reward_count,
            numbers_of_maps_before_push_gacha_screen: gachaconfig
                .nb_maps_threshold_before_push_gacha_screen,
        };

        let file = File::create(gacha_config_path)?;
        serde_json::to_writer_pretty(file, &new_gacha_config)?;
    }

    Ok(())
}
