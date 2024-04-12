//! # Gacha Machine building
//! Build the gacha machine
use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use ubiart_toolkit::{
    cooked,
    json_types::{self, v22::GameManagerConfig22},
};

use crate::{
    build::{BuildFiles, BuildState},
    types::gameconfig::gachacontent::{GachaConfig, GachaItem},
    utils::cook_path,
};

/// Build the gacha machine
pub fn build(
    bs: &BuildState,
    bf: &mut BuildFiles,
    gameconfig: &mut GameManagerConfig22<'_>,
    gacha_items: Vec<GachaItem>,
) -> Result<(), Error> {
    let gacha_config_file = bs
        .native_vfs
        .open(&bs.rel_tree.config().join("gacha.json"))?;
    let gacha_config: GachaConfig = serde_json::from_slice(&gacha_config_file)?;

    gameconfig.gachaconfig.force_high_rarity_reward_count =
        gacha_config.force_high_rarity_reward_count;
    gameconfig.gachaconfig.price = gacha_config.price;
    gameconfig
        .gachaconfig
        .nb_maps_threshold_before_push_gacha_screen =
        gacha_config.numbers_of_maps_before_push_gacha_screen;

    let gacha_content_database =
        json_types::v22::Template22::GachaContentDatabase(json_types::isg::GachaContentDatabase {
            class: None,
            collectibles: gacha_items.into_iter().map(Into::into).collect(),
        });

    let gacha_content_database_vec =
        cooked::json::create_vec_with_capacity_hint(&gacha_content_database, 16_000)?;
    bf.generated_files.add_file(
        cook_path(&gameconfig.config_files_path.gachacontent, bs.platform)?.into(),
        gacha_content_database_vec,
    )?;

    Ok(())
}
