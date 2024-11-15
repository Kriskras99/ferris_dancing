//! # Aliases building
//! Build aliases
use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use ubiart_toolkit::{
    cooked,
    cooked::{isg::GameManagerConfigV22, json::LocalAliasesV2022},
    utils::UniqueGameId,
};

use crate::{
    build::{BuildFiles, BuildState},
    types::gameconfig::{aliases::Aliases, gachacontent::GachaItem},
    utils::cook_path,
};

/// Build aliases
pub fn build(
    bs: &BuildState,
    bf: &mut BuildFiles,
    gameconfig: &mut GameManagerConfigV22,
    gacha_items: &mut Vec<GachaItem>,
) -> Result<(), Error> {
    let aliases_file = bs
        .native_vfs
        .open(&bs.rel_tree.config().join("aliases.json"))?;
    let aliases = serde_json::from_slice::<Aliases>(&aliases_file)?.into_owned();

    let aliasesobjectives = &mut gameconfig.aliasesobjectives;
    aliasesobjectives.clear();

    let mut aliases_vec = Vec::with_capacity(aliases.aliases.len());
    for alias in aliases.aliases {
        let is_gacha = !alias.unlocked_by_default && alias.unlock_objective.is_none();
        let new_alias = alias.into_unlockable_alias_descriptor(aliasesobjectives);
        if is_gacha {
            gacha_items.push(GachaItem::Alias(new_alias.id));
        }
        aliases_vec.push(new_alias);
    }

    let local_aliases = LocalAliasesV2022 {
        class: Some(LocalAliasesV2022::CLASS),
        locked_color: aliases.locked_color.clone(),
        difficulty_colors: aliases
            .rarity_color
            .iter()
            .map(|(r, c)| (ubiart_toolkit::cooked::json::Rarity::from(*r), c.clone()))
            .collect(),
        aliases: aliases_vec,
    };

    let local_aliases_vec = cooked::json::create_vec_with_capacity_hint(&local_aliases, 230_000)?;
    bf.generated_files.add_file(
        cook_path(&gameconfig.alias_db_path, UniqueGameId::NX2022)?.into(),
        local_aliases_vec,
    )?;

    Ok(())
}
