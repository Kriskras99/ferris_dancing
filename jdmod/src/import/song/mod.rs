//! # Importing songs
//! Contains functionality for importing songs
use std::{borrow::Cow, fs::File};

use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use ubiart_toolkit::{cooked, utils::UniqueGameId};

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
        song::{Song, SongDirectoryTree, Tag},
        ImportState,
    },
    utils::cook_path,
};

/// State that is used a lot during the import
pub struct SongImportState<'a> {
    /// The filesystem of the game that is being pared
    pub vfs: &'a dyn VirtualFileSystem,
    /// The map codename in lowercase
    pub lower_map_name: &'a str,
    /// The directory structure for this song
    pub dirs: SongDirectoryTree,
    /// Game and platform
    pub ugi: UniqueGameId,
    /// Should we be lax with parsing
    pub lax: bool,
}

/// Import the song described at `songdesc_path``
pub fn import(is: &ImportState<'_>, songdesc_path: &str) -> Result<(), Error> {
    let songdesc_file = is
        .vfs
        .open(cook_path(songdesc_path, is.ugi.platform)?.as_ref())?;
    let mut actor = cooked::json::parse_v22(&songdesc_file, is.lax)?.into_actor()?;
    let songdesc = actor.components.swap_remove(0).into_song_description()?;

    let map_name = &songdesc.map_name;
    let lower_map_name = map_name.to_lowercase();
    let song_path = is.dirs.songs().join(map_name.as_ref());
    let dirs = SongDirectoryTree::new(&song_path);
    if dirs.exists() {
        println!("Skipping {map_name}, song already imported!");
        return Ok(());
    } else if songdesc.jdm_attributes.is_some()
        || songdesc.tags.contains(&Cow::Borrowed("dancemachine"))
    {
        // TODO: Support dance machine/dance lab
        println!("Warning! {map_name} is a dance machine map! Skipping!");
        return Ok(());
    } else if songdesc.tags.contains(&Cow::Borrowed("MASHUP")) {
        // TODO: Support mashups
        println!("Warning! {map_name} is a mashup! Skipping!");
        return Ok(());
    } else if songdesc.tags.contains(&Cow::Borrowed("doublescoring")) {
        // TODO: Support double rumble
        println!("Warning! {map_name} is a double rumble map! Skipping!");
        return Ok(());
    }

    dirs.create_all()?;

    let sis = SongImportState {
        vfs: is.vfs,
        lower_map_name: &lower_map_name,
        dirs,
        ugi: is.ugi,
        lax: is.lax,
    };

    println!("Parsing {map_name}");
    let main_scene_path = cook_path(
        &format!("world/maps/{lower_map_name}/{lower_map_name}_main_scene.isc"),
        is.ugi.platform,
    )?;
    let main_scene_file = is.vfs.open(main_scene_path.as_ref())?;
    let main_scene = cooked::isc::parse(&main_scene_file)?.scene;

    // Import the audio preview
    let autodance_path = main_scene
        .get_subscene_by_userfriendly_end("_AUTODANCE", sis.lax)?
        .wrapped_scene
        .scene
        .get_actor_by_userfriendly_end("_autodance", sis.lax)?
        .lua
        .as_ref();
    autodance::import(&sis, autodance_path)?;

    // Prepare for importing the timelines
    let timeline_scene = &main_scene
        .get_subscene_by_userfriendly_end("_TML", sis.lax)?
        .wrapped_scene
        .scene;

    // Import the dance timeline
    let dance_timeline_path = &timeline_scene
        .get_actor_by_userfriendly_end("_tml_dance", sis.lax)?
        .lua;
    dance_timeline::import(&sis, dance_timeline_path)?;

    // Import the karaoke timeline
    let karaoke_timeline_path = &timeline_scene
        .get_actor_by_userfriendly_end("_tml_karaoke", sis.lax)?
        .lua;
    karaoke_timeline::import(&sis, karaoke_timeline_path)?;

    // Import the mainsequence
    let mainsequence_path = &main_scene
        .get_subscene_by_userfriendly_end("_CINE", sis.lax)?
        .wrapped_scene
        .scene
        .get_actor_by_userfriendly_end("_MainSequence", sis.lax)?
        .lua;
    mainsequence::import(&sis, mainsequence_path)?;

    // Import the audio
    let musictrack_path = &main_scene
        .get_subscene_by_userfriendly_end("_AUDIO", sis.lax)?
        .wrapped_scene
        .scene
        .get_actor_by_userfriendly("MusicTrack")?
        .lua;
    let audiofile = musictrack::import(&sis, musictrack_path)?;

    // Import the video
    let video_actor = &main_scene
        .get_subscene_by_userfriendly_end("_VIDEO", sis.lax)?
        .wrapped_scene
        .scene
        .get_actor_by_userfriendly("VideoScreen")?;
    let videofile = video::import(&sis, video_actor)?;

    // Import menuart
    match (
        main_scene.get_subscene_by_userfriendly_end("_menuart", sis.lax),
        sis.lax,
    ) {
        (Ok(subscene), _) => {
            let menuart_scene = &subscene.wrapped_scene.scene;
            menuart::import(&sis, menuart_scene, &songdesc.phone_images)?;
        }
        (Err(_), true) => {
            println!("Warning! Could not find menuart subscene, trying to use scene file!");
            let cooked_path = cook_path(
                &format!(
                    "world/maps/{}/menuart/{}_menuart.isc",
                    sis.lower_map_name, sis.lower_map_name
                ),
                sis.ugi.platform,
            )?;
            let scene_file = sis.vfs.open(cooked_path.as_ref())?;
            let scene = cooked::isc::parse(&scene_file)?.scene;
            menuart::import(&sis, &scene, &songdesc.phone_images)?;
        }
        (Err(error), false) => return Err(error.into()),
    }

    let song_path = sis.dirs.song().join("song.json");

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
            .map(Cow::as_ref)
            .map(TryInto::<Tag>::try_into)
            .collect::<Result<_, _>>()?,
        subtitle: is.locale_id_map.get(songdesc.locale_id).unwrap_or_default(),
        default_colors: (&songdesc.default_colors).into(),
        audiofile: Cow::Owned(audiofile),
        videofile: Cow::Borrowed(videofile),
    };

    let song_file = File::create(song_path)?;
    serde_json::to_writer_pretty(song_file, &song)?;

    Ok(())
}
