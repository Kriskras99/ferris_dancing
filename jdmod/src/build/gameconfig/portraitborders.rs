//! # Portrait borders building
//! Build the portrait borders
use std::{collections::HashMap, fs::File, path::PathBuf};

use anyhow::Error;
use ubiart_toolkit::{
    cooked,
    json_types::{self, isg::PortraitBordersDatabase, v22::GameManagerConfig22},
};

use crate::{
    build::{BuildFiles, BuildState},
    types::gameconfig::{gachacontent::GachaItem, portraitborders::PortraitBorder},
    utils::{cook_path, encode_texture},
};

/// Build the portrait borders
pub fn build(
    bs: &BuildState,
    bf: &mut BuildFiles,
    gameconfig: &GameManagerConfig22<'_>,
    gacha_items: &mut Vec<GachaItem>,
) -> Result<(), Error> {
    let saved_portraitborders: HashMap<String, PortraitBorder> = serde_json::from_reader(
        File::open(bs.dirs.portraitborders().join("portraitborders.json"))?,
    )?;

    let mut portrait_borders = Vec::with_capacity(saved_portraitborders.keys().len());

    for (name, pb) in &saved_portraitborders {
        let desc = pb.to_portrait_border_desc(name);

        gacha_items.push(GachaItem::PortraitBorder(desc.portrait_border_id));

        // Save the background and foreground textures and phone images (if they exist)
        let background_texture_encoded = encode_texture(
            &bs.dirs
                .portraitborders()
                .join(pb.background_texture_path.as_ref()),
        )?;
        let background_texture_vec = cooked::png::create_vec(&background_texture_encoded)?;
        bf.generated_files.add_file(
            cook_path(&desc.background_texture_path, bs.platform)?,
            background_texture_vec,
        )?;

        if let Some(foreground_texture_path) = &pb.foreground_texture_path {
            let foreground_texture_encoded = encode_texture(
                &bs.dirs
                    .portraitborders()
                    .join(foreground_texture_path.as_ref()),
            )?;
            let foreground_texture_vec = cooked::png::create_vec(&foreground_texture_encoded)?;
            bf.generated_files.add_file(
                cook_path(&desc.foreground_texture_path, bs.platform)?,
                foreground_texture_vec,
            )?;
        }

        bf.static_files.add_file(
            bs.dirs
                .portraitborders()
                .join(pb.background_phone_path.as_ref()),
            PathBuf::from(desc.background_phone_path.as_ref()),
        )?;

        if let Some(foreground_phone_path) = &pb.foreground_phone_path {
            bf.static_files.add_file(
                bs.dirs
                    .portraitborders()
                    .join(foreground_phone_path.as_ref()),
                PathBuf::from(desc.foreground_phone_path.as_ref()),
            )?;
        }

        portrait_borders.push(desc);
    }

    let template = json_types::v22::Template22::PortraitBordersDatabase(PortraitBordersDatabase {
        class: None,
        portrait_borders,
    });

    let template_vec = cooked::json::create_vec(&template)?;
    bf.generated_files.add_file(
        cook_path(&gameconfig.config_files_path.portraitborders, bs.platform)?,
        template_vec,
    )?;

    Ok(())
}
