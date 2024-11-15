//! # Portraitborders
//! Import all portraitborders
use std::{collections::HashMap, fs::File};

use anyhow::Error;
use cooked::isg;
use ubiart_toolkit::{cooked, cooked::isg::PortraitBordersDatabase};

use crate::{
    regex,
    types::{gameconfig::portraitborders::PortraitBorder, ImportState},
    utils::{cook_path, decode_texture, hipstr_regex_single_capture},
};

/// Import all portraitborders
pub fn import_v20v22(is: &ImportState<'_>, portraitborders_path: &str) -> Result<(), Error> {
    println!("Importing portrait borders...");

    let regex = regex!(r".*/[0-9]+_([a-z_]*)/pb_back.png$");

    let new_portraitborders = is
        .vfs
        .open(cook_path(portraitborders_path, is.ugi)?.as_ref())?;
    let portrait_borders_database =
        isg::parse::<PortraitBordersDatabase>(&new_portraitborders, is.lax)?;

    // Load existing avatars in the mod
    let pb_config_path = is.dirs.portraitborders().join("portraitborders.json");
    let pb_config_file = std::fs::read(&pb_config_path).unwrap_or_else(|_| vec![b'{', b'}']);
    let mut portraitborders: HashMap<String, PortraitBorder> =
        serde_json::from_slice(&pb_config_file)?;

    for desc in &portrait_borders_database.portrait_borders {
        let name = hipstr_regex_single_capture(regex, &desc.background_texture_path)?;

        if !portraitborders.contains_key(name.as_str()) {
            let pb = PortraitBorder::from_portrait_border_desc(desc, name.as_ref())?;
            std::fs::create_dir(is.dirs.portraitborders().join(name.as_str()))?;

            // Save the background and foreground textures and phone images (if they exist)
            let background_texture = is
                .vfs
                .open(cook_path(&desc.background_texture_path, is.ugi)?.as_ref())?;
            let background_texture_decoded = decode_texture(&background_texture, is.ugi)?;
            background_texture_decoded.save(
                is.dirs
                    .portraitborders()
                    .join(pb.background_texture_path.as_str()),
            )?;

            if let Some(foreground_texture_path) = &pb.foreground_texture_path {
                let foreground_texture = is
                    .vfs
                    .open(cook_path(&desc.foreground_texture_path, is.ugi)?.as_ref())?;
                let foreground_texture_decoded = decode_texture(&foreground_texture, is.ugi)?;
                foreground_texture_decoded.save(
                    is.dirs
                        .portraitborders()
                        .join(foreground_texture_path.as_str()),
                )?;
            }

            portraitborders.insert(name.to_string(), pb);
        }
    }

    let file = File::create(pb_config_path)?;
    serde_json::to_writer_pretty(file, &portraitborders)?;

    Ok(())
}
