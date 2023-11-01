//! # Avatars
//! Import all the avatars.
//!
//! Current implementation is a bit wonky. A better option would be too manually match all avatar ids
//! to names per game. Then Just Dance 2017 avatars can also be imported.
use std::{borrow::Cow, collections::HashMap, fs::File, io::Write};

use anyhow::{anyhow, Error};
use ubiart_toolkit::{cooked, json_types::AvatarsObjectives};

use crate::{
    types::{
        gameconfig::avatars::{Avatar, UnlockType},
        ImportState,
    },
    utils::{cook_path, decode_texture},
};

/// Import all the avatars (Just Dance 2018-2022)
pub fn import_v18v22(
    is: &ImportState<'_>,
    avatardb_scene: &str,
    avatarsobjectives: Option<&AvatarsObjectives>,
) -> Result<(), Error> {
    let empty_objectives = HashMap::new();
    println!("Importing avatars...");

    // Load existing avatars in the mod
    let avatars_config_path = is.dirs.avatars().join("avatars.json");
    let mut avatars: HashMap<String, Avatar> = if avatars_config_path.exists() {
        let file = File::open(&avatars_config_path)?;
        serde_json::from_reader(file)?
    } else {
        HashMap::new()
    };

    // Open the avatardb and avatarsobjectives (which might be empty)
    let avatardb_file = is
        .vfs
        .open(cook_path(avatardb_scene, is.platform)?.as_ref())?;
    let avatardb = cooked::isc::parse(&avatardb_file)?;
    let avatarsobjectives = avatarsobjectives.unwrap_or(&empty_objectives);

    // maps the id of an avatar to an avatar name, so related avatars can be linked by name
    let mut id_map = HashMap::with_capacity(avatardb.scene.actors.len());
    // stores the avatar descriptions so we don't have to iter through the avatardb again
    let mut avatar_descriptions = Vec::with_capacity(avatardb.scene.actors.len());

    for actor in avatardb.scene.actors {
        let actor = actor.actor()?;

        // Extract avatar description from template
        let file = is
            .vfs
            .open(cook_path(actor.lua.as_ref(), is.platform)?.as_ref())?;
        let template = cooked::json::parse_v22(&file)?;
        let actor_template = template.actor()?;
        assert!(
            actor_template.components.len() == 2,
            "Not exactly two components in actor"
        );
        let avatar_desc = actor_template.components[1].avatar_description()?;

        // Create a (hopefully) unique name for the avatar
        // TODO: Maybe just use a static mapping for each game?
        let name = format!(
            "{}_{}{}{}",
            avatar_desc.used_as_coach_map_name.as_ref(),
            avatar_desc.used_as_coach_coach_id,
            if avatar_desc.special_effect == Some(1) {
                "_Gold"
            } else {
                ""
            },
            if avatar_desc.unlock_type == 22 {
                "_Unlimited"
            } else {
                ""
            }
        );
        // Add the name to the id map, so we can look it up later
        id_map.insert(avatar_desc.avatar_id, name.clone());

        // Collect the avatar descriptions so we don't have the parse isc and tpls again
        avatar_descriptions.push(avatar_desc.to_owned());
    }

    for avatar_desc in avatar_descriptions {
        let name = &id_map[&avatar_desc.avatar_id];
        // Only add new avatars
        if !avatars.contains_key(name) {
            let avatar_named_dir_path = is.dirs.avatars().join(name);
            let avatar_image_path = format!("{name}/avatar.png");
            let avatar_image_phone_path = format!("{name}/avatar_phone.png");
            let avatar = Avatar {
                relative_song_name: if avatar_desc.relative_song_name.is_empty() {
                    None
                } else {
                    Some(Cow::Owned(avatar_desc.relative_song_name))
                },
                sound_family: Cow::Owned(avatar_desc.sound_family),
                status: avatar_desc.status,
                unlock_type: UnlockType::from_unlock_type(
                    avatar_desc.unlock_type,
                    avatarsobjectives.get(&avatar_desc.avatar_id),
                )?,
                used_as_coach_map_name: Cow::Owned(avatar_desc.used_as_coach_map_name),
                used_as_coach_coach_id: avatar_desc.used_as_coach_coach_id,
                special_effect: avatar_desc.special_effect == Some(1),
                main_avatar: avatar_desc.main_avatar_id.and_then(|main_id| {
                    if main_id == u16::MAX {
                        None
                    } else {
                        let main = id_map.get(&main_id).map(String::as_str).map(Cow::Borrowed);
                        if main.is_none() {
                            println!("Warning! Avatar id {main_id} does not exist!");
                        }
                        main
                    }
                }),
                image_path: Cow::Owned(avatar_image_path),
                image_phone_path: Cow::Owned(avatar_image_phone_path),
            };
            std::fs::create_dir(&avatar_named_dir_path)?;
            let alt_actor_file = is
                .vfs
                .open(cook_path(avatar_desc.actor_path.as_ref(), is.platform)?.as_ref())?;
            let alt_actor = cooked::act::parse(&alt_actor_file, is.game)?;

            let image_actor = alt_actor
                .templates
                .first()
                .ok_or_else(|| anyhow!("No templates in {}", avatar_desc.actor_path))?;
            let mtg = image_actor.data.material_graphics_component()?;

            // Save decooked image
            let cooked_image_path = cook_path(
                &mtg.files[0]
                    .as_ref()
                    .ok_or_else(|| anyhow!("No image path in {:?}", mtg.files))?
                    .to_string(),
                is.platform,
            )?;
            let decooked_image = decode_texture(&is.vfs.open(cooked_image_path.as_ref())?)?;
            let avatar_image_path = is.dirs.avatars().join(avatar.image_path.as_ref());
            decooked_image.save(&avatar_image_path)?;

            // Save phone image
            let avatar_image_phone_path = is.dirs.avatars().join(avatar.image_phone_path.as_ref());
            let mut avatar_image_phone_file = File::create(&avatar_image_phone_path)?;
            avatar_image_phone_file.write_all(&is.vfs.open(avatar_desc.phone_image.as_ref())?)?;

            avatars.insert(name.clone(), avatar);
        }
    }

    // TODO: Detect unreferenced avatars and try to import them?

    let file = File::create(avatars_config_path)?;
    serde_json::to_writer_pretty(file, &avatars)?;

    Ok(())
}
