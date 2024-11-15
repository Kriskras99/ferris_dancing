//! # Importing songs
//! Contains functionality for importing songs
use std::fs::File;

use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use hipstr::HipStr;
use tracing::error;
use ubiart_toolkit::{cooked, cooked::tpl::types::SongDescription, utils::UniqueGameId};

mod autodance;
mod dance_timeline;
mod karaoke_timeline;
mod mainsequence;
mod menuart;
mod montage;
mod musictrack;
mod video;

use crate::{
    types::{
        localisation::LocaleIdMap,
        song::{Song, SongDirectoryTree, Tag},
        ImportState,
    },
    utils::cook_path,
};

/// State that is used a lot during the import
pub struct SongImportState<'a> {
    /// The filesystem of the game that is being pared
    pub vfs: &'a dyn VirtualFileSystem,
    /// The map codename
    pub map_name: HipStr<'static>,
    /// The map codename in lowercase
    pub lower_map_name: &'a str,
    /// The directory structure for this song
    pub dirs: SongDirectoryTree,
    /// Game and platform
    pub ugi: UniqueGameId,
    /// Should we be lax with parsing
    pub lax: bool,
    /// Mapping of game locale id to mod locale id
    pub locale_id_map: &'a LocaleIdMap,
}

/// Import the song described at `songdesc_path``
pub fn import(is: &ImportState<'_>, songdesc_path: &str) -> Result<(), Error> {
    let songdesc_file = is.vfs.open(cook_path(songdesc_path, is.ugi)?.as_ref())?;
    let mut actor = cooked::tpl::parse(&songdesc_file, is.ugi, is.lax)?;
    let songdesc = actor.components.remove(0).into_song_description()?;

    let map_name = &songdesc.map_name;
    let lower_map_name = map_name.to_lowercase();
    let dirs = SongDirectoryTree::new(is.dirs.songs(), map_name);
    if dirs.exists() {
        println!("Skipping {map_name}, song already imported!");
        return Ok(());
    } else if songdesc.jdm_attributes.is_some()
        || songdesc.tags.contains(&HipStr::borrowed("dancemachine"))
    {
        // TODO: Support dance machine/dance lab
        println!("Warning! {map_name} is a dance machine map! Skipping!");
        return Ok(());
    } else if songdesc.tags.contains(&HipStr::borrowed("MASHUP"))
        || songdesc.tags.contains(&HipStr::borrowed("COMMUNITYMASHUP"))
    {
        // TODO: Support mashups
        println!("Warning! {map_name} is a mashup! Skipping!");
        return Ok(());
    } else if songdesc.tags.contains(&HipStr::borrowed("doublescoring")) {
        // TODO: Support double rumble
        println!("Warning! {map_name} is a double rumble map! Skipping!");
        return Ok(());
    } else if songdesc.tags.contains(&HipStr::borrowed("JUSTSHINE")) {
        // TODO: Support showtime maps
        println!("Warning! {map_name} is a showtime map! Skipping!");
        return Ok(());
    } else if songdesc.tags.contains(&HipStr::borrowed("PARTYMASTER")) {
        // TODO: Support party master maps
        println!("Warning! {map_name} is a party (puppet) master map! Skipping!");
        return Ok(());
    }

    let sis = SongImportState {
        vfs: is.vfs,
        map_name: map_name.clone().into_owned(),
        lower_map_name: &lower_map_name,
        dirs,
        ugi: is.ugi,
        lax: is.lax,
        locale_id_map: &is.locale_id_map,
    };

    match actual_import(&sis, songdesc) {
        Ok(()) => {}
        Err(err) => {
            error!("Failed to import {}: {err}", sis.map_name);
            sis.dirs.remove_dir_all()?;
            if !is.lax {
                return Err(err);
            }
        }
    }
    Ok(())
}

/// The actual import logic for a song, so that the [`import`] function can catch any errors
fn actual_import(sis: &SongImportState<'_>, songdesc: SongDescription<'_>) -> Result<(), Error> {
    println!("Parsing {}", sis.map_name);
    let lower_map_name = sis.lower_map_name;

    sis.dirs.create_dir_all()?;
    let main_scene_path = cook_path(
        &format!("world/maps/{lower_map_name}/{lower_map_name}_main_scene.isc"),
        sis.ugi,
    )?;
    let main_scene_file = sis.vfs.open(main_scene_path.as_ref())?;
    let main_scene = cooked::isc::parse(&main_scene_file, sis.ugi)?.scene;

    // Import the audio preview
    let autodance_path = main_scene
        .get_subscene_by_userfriendly_end("_AUTODANCE", sis.lax)?
        .wrapped_scene
        .as_ref()
        .get_actor_by_userfriendly_end("_autodance", sis.lax)?
        .lua
        .as_ref();
    autodance::import(sis, autodance_path)?;

    // Prepare for importing the timelines
    let timeline_scene = &main_scene
        .get_subscene_by_userfriendly_end("_TML", sis.lax)?
        .wrapped_scene
        .as_ref();

    // Import the dance timeline
    let dance_timeline_path = &timeline_scene
        .get_actor_by_userfriendly_end("_tml_dance", sis.lax)?
        .lua;
    dance_timeline::import(sis, dance_timeline_path)?;

    // Import the karaoke timeline
    let karaoke_timeline_path = &timeline_scene
        .get_actor_by_userfriendly_end("_tml_karaoke", sis.lax)?
        .lua;
    karaoke_timeline::import(sis, karaoke_timeline_path)?;

    // Import the mainsequence
    let mainsequence_path = &main_scene
        .get_subscene_by_userfriendly_end("_CINE", sis.lax)?
        .wrapped_scene
        .as_ref()
        .get_actor_by_userfriendly_end("_MainSequence", sis.lax)?
        .lua;
    mainsequence::import(sis, mainsequence_path)?;

    // Import the audio
    let musictrack_path = &main_scene
        .get_subscene_by_userfriendly_end("_AUDIO", sis.lax)?
        .wrapped_scene
        .as_ref()
        .get_actor_by_userfriendly("MusicTrack")?
        .lua;
    let audiofile = musictrack::import(sis, musictrack_path)?;

    // Import the video
    let video_actor = &main_scene
        .get_subscene_by_userfriendly_end("_VIDEO", sis.lax)?
        .wrapped_scene
        .as_ref()
        .get_actor_by_userfriendly("VideoScreen")?;
    let videofile = video::import(sis, video_actor)?;

    // Import menuart
    match (
        main_scene.get_subscene_by_userfriendly_end("_MENUART", true),
        sis.lax,
    ) {
        (Ok(subscene), _) => {
            menuart::import(sis, subscene.wrapped_scene.as_ref(), &songdesc.phone_images)?;
        }
        (Err(_), true) => {
            println!("Warning! Could not find menuart subscene, trying to use scene file!");
            let cooked_path = cook_path(
                &format!(
                    "world/maps/{}/menuart/{}_menuart.isc",
                    sis.lower_map_name, sis.lower_map_name
                ),
                sis.ugi,
            )?;
            let scene_file = sis.vfs.open(cooked_path.as_ref())?;
            let scene = cooked::isc::parse(&scene_file, sis.ugi)?.scene;
            menuart::import(sis, &scene, &songdesc.phone_images)?;
        }
        (Err(error), false) => return Err(error.into()),
    }

    let song = Song {
        map_name: songdesc.map_name,
        original_jd_version: songdesc.original_jd_version,
        artist: songdesc.artist,
        dancer_name: songdesc.dancer_name,
        title: songdesc.title,
        credits: songdesc.credits,
        number_of_coaches: songdesc.num_coach.try_into()?,
        main_coach: (songdesc.main_coach >= 0).then(|| songdesc.main_coach.abs_diff(0)),
        difficulty: songdesc.difficulty.try_into()?,
        sweat_difficulty: songdesc.sweat_difficulty.try_into()?,
        related_songs: songdesc.related_albums,
        status: songdesc.status.try_into()?,
        tags: songdesc
            .tags
            .iter()
            .map(HipStr::as_str)
            .map(TryInto::<Tag>::try_into)
            .collect::<Result<_, _>>()?,
        subtitle: sis
            .locale_id_map
            .get(songdesc.locale_id)
            .unwrap_or_default(),
        default_colors: (&songdesc.default_colors).into(),
        audiofile: HipStr::from(audiofile),
        videofile: HipStr::borrowed(videofile),
    };

    let song_file = File::create(sis.dirs.song_file())?;
    serde_json::to_writer_pretty(song_file, &song)?;

    Ok(())
}
