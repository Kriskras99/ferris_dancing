//! # Aliases building
//! Build aliases
use std::fs::File;

use anyhow::Error;
use ubiart_toolkit::{
    cooked,
    json_types::{
        isg::LocalAliases,
        v22::{GameManagerConfig22, Template22},
    },
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
    gameconfig: &mut GameManagerConfig22<'_>,
    gacha_items: &mut Vec<GachaItem>,
) -> Result<(), Error> {
    let aliases: Aliases =
        serde_json::from_reader(File::open(bs.dirs.config().join("aliases.json"))?)?;

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

    let local_aliases = Template22::LocalAliases(LocalAliases {
        class: None,
        locked_color: aliases.locked_color.clone(),
        difficulty_colors: aliases
            .rarity_color
            .iter()
            .map(|(r, c)| (ubiart_toolkit::json_types::isg::Rarity::from(*r), c.clone()))
            .collect(),
        aliases: aliases_vec,
    });

    let local_aliases_vec = cooked::json::create_vec_with_capacity_hint(&local_aliases, 230_000)?;
    bf.generated_files.add_file(
        cook_path(&gameconfig.alias_db_path, bs.platform)?.into(),
        local_aliases_vec,
    )?;

    Ok(())
}
