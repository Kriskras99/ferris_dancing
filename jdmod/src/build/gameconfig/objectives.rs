//! # Objectives Building
//! Build the objectives
use std::{borrow::Cow, collections::HashMap};

use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use ubiart_toolkit::{cooked, json_types};

use crate::{
    build::{BuildFiles, BuildState},
    types::gameconfig::objectives::Objective,
    utils::cook_path,
};

/// Build the objectives
pub fn build(bs: &BuildState, bf: &mut BuildFiles) -> Result<(), Error> {
    let name_map_file = bs
        .native_vfs
        .open(&bs.rel_tree.config().join("objectives.json"))?;
    let name_map: HashMap<Cow<'_, str>, Objective> = serde_json::from_slice(&name_map_file)?;
    let objective_descs = name_map
        .into_iter()
        .map(|(name, descriptor)| (name, descriptor.into()))
        .collect();

    let objective_database =
        json_types::v22::Template22::ObjectivesDatabase(json_types::isg::ObjectivesDatabase {
            class: None,
            objective_descs,
        });

    let objective_database_vec = cooked::json::create_vec(&objective_database)?;
    bf.generated_files.add_file(
        cook_path("enginedata/gameconfig/objectives.isg", bs.platform)?.into(),
        objective_database_vec,
    )?;

    Ok(())
}
