//! # Portrait borders building
//! Build the portrait borders
use std::collections::HashMap;

use anyhow::Error;
use dotstar_toolkit_utils::vfs::{VirtualFileSystem, VirtualPathBuf};
use ubiart_toolkit::{cooked, cooked::isg::GameManagerConfigV22, utils::UniqueGameId};

use crate::{
    build::{BuildFiles, BuildState},
    types::gameconfig::{gachacontent::GachaItem, portraitborders::PortraitBorder},
    utils::{cook_path, encode_texture},
};

/// Build the portrait borders
pub fn build(
    bs: &BuildState,
    bf: &mut BuildFiles,
    gameconfig: &GameManagerConfigV22<'_>,
    gacha_items: &mut Vec<GachaItem>,
) -> Result<(), Error> {
    let saved_portraitborders_file = bs
        .native_vfs
        .open(&bs.rel_tree.portraitborders().join("portraitborders.json"))?;
    let saved_portraitborders: HashMap<String, PortraitBorder> =
        serde_json::from_slice(&saved_portraitborders_file)?;

    let mut portrait_borders = Vec::with_capacity(saved_portraitborders.keys().len());

    for (name, pb) in &saved_portraitborders {
        let desc = pb.to_portrait_border_desc(name);

        gacha_items.push(GachaItem::PortraitBorder(desc.portrait_border_id));

        // Save the background and foreground textures and phone images (if they exist)
        let background_texture_encoded = encode_texture(
            bs.native_vfs,
            &bs.rel_tree
                .portraitborders()
                .join(pb.background_texture_path.as_str()),
        )?;
        let background_texture_vec = cooked::png::create_vec(background_texture_encoded)?;
        bf.generated_files.add_file(
            cook_path(&desc.background_texture_path, UniqueGameId::NX2022)?.into(),
            background_texture_vec,
        )?;

        if let Some(foreground_texture_path) = &pb.foreground_texture_path {
            let foreground_texture_encoded = encode_texture(
                bs.native_vfs,
                &bs.rel_tree
                    .portraitborders()
                    .join(foreground_texture_path.as_str()),
            )?;
            let foreground_texture_vec = cooked::png::create_vec(foreground_texture_encoded)?;
            bf.generated_files.add_file(
                cook_path(&desc.foreground_texture_path, UniqueGameId::NX2022)?.into(),
                foreground_texture_vec,
            )?;
        }

        bf.static_files.add_file(
            bs.rel_tree
                .portraitborders()
                .join(pb.background_texture_path.as_str()),
            VirtualPathBuf::from(desc.background_phone_path.as_str()),
        )?;

        if let Some(foreground_phone_path) = &pb.foreground_texture_path {
            bf.static_files.add_file(
                bs.rel_tree
                    .portraitborders()
                    .join(foreground_phone_path.as_str()),
                VirtualPathBuf::from(desc.foreground_phone_path.as_str()),
            )?;
        }

        portrait_borders.push(desc);
    }

    let template = cooked::isg::PortraitBordersDatabase {
        class: Some(cooked::isg::PortraitBordersDatabase::CLASS),
        portrait_borders,
    };

    let template_vec = cooked::json::create_vec(&template)?;
    bf.generated_files.add_file(
        cook_path(
            &gameconfig.config_files_path.portraitborders,
            UniqueGameId::NX2022,
        )?
        .into(),
        template_vec,
    )?;

    Ok(())
}
