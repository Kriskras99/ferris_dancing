//! # Portraitborders
//! Import all portraitborders
use std::{collections::HashMap, fs::File, io::Write};

use anyhow::Error;
use ubiart_toolkit::cooked;

use crate::{
    regex,
    types::{gameconfig::portraitborders::PortraitBorder, ImportState},
    utils::{cook_path, cow_regex_single_capture, decode_texture},
};

/// Import all portraitborders
pub fn import_v20v22(is: &ImportState<'_>, portraitborders_path: &str) -> Result<(), Error> {
    println!("Importing portrait borders...");

    let regex = regex!(r".*/[0-9]+_([a-z_]*)/pb_back.png$");

    let new_portraitborders = is
        .vfs
        .open(cook_path(portraitborders_path, is.ugi.platform)?.as_ref())?;
    let template = cooked::json::parse_v22(&new_portraitborders, is.lax)?;
    let portrait_borders_database = template.into_portrait_borders_database()?;

    // Load existing avatars in the mod
    let pb_config_path = is.dirs.portraitborders().join("portraitborders.json");
    let mut portraitborders: HashMap<String, PortraitBorder> = if pb_config_path.exists() {
        let file = File::open(&pb_config_path)?;
        serde_json::from_reader(file)?
    } else {
        HashMap::new()
    };

    for desc in &portrait_borders_database.portrait_borders {
        let name = cow_regex_single_capture(regex, desc.background_texture_path.clone())?;

        if !portraitborders.contains_key(name.as_ref()) {
            let pb = PortraitBorder::from_portrait_border_desc(desc, name.as_ref())?;
            std::fs::create_dir(is.dirs.portraitborders().join(name.as_ref()))?;

            // Save the background and foreground textures and phone images (if they exist)
            let background_texture = is
                .vfs
                .open(cook_path(&desc.background_texture_path, is.ugi.platform)?.as_ref())?;
            let background_texture_decoded = decode_texture(&background_texture, is.ugi)?;
            background_texture_decoded.save(
                is.dirs
                    .portraitborders()
                    .join(pb.background_texture_path.as_ref()),
            )?;

            if let Some(foreground_texture_path) = &pb.foreground_texture_path {
                let foreground_texture = is
                    .vfs
                    .open(cook_path(&desc.foreground_texture_path, is.ugi.platform)?.as_ref())?;
                let foreground_texture_decoded = decode_texture(&foreground_texture, is.ugi)?;
                foreground_texture_decoded.save(
                    is.dirs
                        .portraitborders()
                        .join(foreground_texture_path.as_ref()),
                )?;
            }

            let background_phone = is.vfs.open(desc.background_phone_path.as_ref().as_ref())?;
            let mut file = File::create(
                is.dirs
                    .portraitborders()
                    .join(pb.background_phone_path.as_ref()),
            )?;
            file.write_all(&background_phone)?;

            if let Some(foreground_phone_path) = &pb.foreground_phone_path {
                let foreground_phone = is.vfs.open(desc.foreground_phone_path.as_ref().as_ref())?;
                let mut file = File::create(
                    is.dirs
                        .portraitborders()
                        .join(foreground_phone_path.as_ref()),
                )?;
                file.write_all(&foreground_phone)?;
            }

            portraitborders.insert(name.to_string(), pb);
        }
    }

    let file = File::create(pb_config_path)?;
    serde_json::to_writer_pretty(file, &portraitborders)?;

    Ok(())
}
