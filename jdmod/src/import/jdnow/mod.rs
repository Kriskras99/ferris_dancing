//! Import functionality for Just Dance Now data

use crate::import::jdnow::types::NowState;
use crate::import::TranscodeSettings;
use crate::types::localisation::LocaleId;
use crate::types::song::{Autodance, Clip, Color, GoldEffectClip, KaraokeClip, MenuArt, MenuArtTexture, MotionClip, MusicTrack, PhoneImage, PictogramClip, Signature, Song, SweatDifficulty, Timeline};
use crate::utils::{extract_audio, transcode};
use crate::{
    import::jdnow::types::NowTree,
    types::{song::SongDirectoryTree, DirectoryTree},
};
use anyhow::{anyhow, Error};
use bluestar_toolkit::{Moves, PictoAtlas, SongDetails};
use dotstar_toolkit_utils::vfs::{zipfs::ZipFs};
use hipstr::HipStr;
use image::{imageops, ImageFormat};
use std::collections::BTreeSet;
use std::fs::File;
use std::ops::Deref;
use std::path::Path;

mod types;

#[tracing::instrument(skip(dir_tree, transcode_settings))]
pub fn import_song(
    root: &Path,
    dir_tree: &DirectoryTree,
    map_name: &str,
    transcode_settings: TranscodeSettings,
) -> Result<(), Error> {
    let tree_jdn = NowTree::new(root, map_name);
    let tree_song = SongDirectoryTree::new(dir_tree.songs(), map_name);
    if tree_song.exists() {
        println!("Skipping {map_name}, song already imported!");
        return Ok(());
    }

    let details_file = std::fs::read(tree_jdn.detail())?;
    let details: SongDetails = serde_json::from_slice(&details_file)?;
    let zipfs = ZipFs::new(File::open(tree_jdn.bundle())?)?;

    let state = NowState {
        bundle: &zipfs,
        now: tree_jdn,
        song: tree_song,
        transcode: transcode_settings,
        details,
    };

    state.song.create_dir_all()?;
    create_autodance(&state)?;
    create_dance_timeline(&state)?;
    create_karaoke_timeline(&state)?;
    create_mainsequence(&state)?;
    create_musictrack(&state)?;
    let videofile = create_video(&state)?;
    let audiofile = create_audio(&state)?;
    create_menuart(&state)?;

    let song = Song {
        map_name: state.details.map_name,
        original_jd_version: state.details.original_jd_version,
        artist: state.details.artist,
        dancer_name: state.details.dancer_name,
        title: state.details.title,
        credits: state.details.credits,
        number_of_coaches: u32::from(state.details.num_coach).try_into()?,
        main_coach: None,
        difficulty: state.details.difficulty.try_into()?,
        sweat_difficulty: SweatDifficulty::Moderate,
        related_songs: vec![],
        status: state.details.status.try_into()?,
        tags: vec![],
        subtitle: LocaleId::default(),
        default_colors: (&state.details.default_colors).into(),
        audiofile: HipStr::from(audiofile),
        videofile: HipStr::borrowed(videofile),
    };

    let song_file = File::create(state.song.song_file())?;
    serde_json::to_writer_pretty(song_file, &song)?;

    Ok(())
}

/// Create an empty autodance
// TODO: Find out if autodance is actually used anywhere
fn create_autodance(state: &NowState) -> Result<(), Error> {
    let autodance = Autodance {
        record: vec![],
        song_start_position: 0.0,
        autodance_sound: "autodance.ogg".into(),
        duration: 30.0,
        playback_events: vec![],
    };

    let autodance_path = state.song.song().join("autodance.json");
    let autodance_file = File::create(autodance_path)?;
    serde_json::to_writer_pretty(autodance_file, &autodance)?;

    Ok(())
}

/// Import the pictos and classifiers
fn create_dance_timeline(state: &NowState) -> Result<(), Error> {
    let mut timeline = Timeline {
        timeline: BTreeSet::new(),
    };

    parse_picto_atlas(state)?;
    parse_pictos(state, &mut timeline)?;
    parse_classifiers(state, &mut timeline)?;

    let dance_timeline_path = state.song.song().join("dance_timeline.json");

    let timeline_file = File::create(dance_timeline_path)?;
    serde_json::to_writer_pretty(timeline_file, &timeline)?;

    Ok(())
}

/// Add the pictos to the timeline
fn parse_pictos(state: &NowState, timeline: &mut Timeline) -> Result<(), Error> {
    let video_offset = i32::try_from(state.details.video_offset)?;

    for picto in &state.details.pictos {
        let start_time = (picto.time - video_offset) / 20;
        let duration = picto.duration / 20;
        let clip = Clip::Pictogram(PictogramClip {
            is_active: true,
            start_time,
            duration,
            picto_filename: HipStr::from(format!("{}.png", picto.name)),
        });

        timeline.timeline.insert(clip);

        if picto.name.ends_with("_gold") || picto.gold_move.is_some() {
            let clip = Clip::GoldEffect(GoldEffectClip {
                is_active: true,
                start_time,
                duration,
                effect_type: 1,
            });

            timeline.timeline.insert(clip);
        }
    }

    Ok(())
}

/// Extract the pictos from the atlas
fn parse_picto_atlas(state: &NowState) -> Result<(), Error> {
    let atlas_desc_file = std::fs::read(state.now.atlas_desc())?;
    let atlas_desc: PictoAtlas = serde_json::from_slice(&atlas_desc_file)?;
    let width = atlas_desc.image_size.width;
    let height = atlas_desc.image_size.height;

    let image = image::open(state.now.atlas())?.into_rgba8();

    for (name, (x, y)) in atlas_desc.images {
        let picto = imageops::crop_imm(&image, x, y, width, height).to_image();
        let mut path = state.song.pictos().join(&name);
        path.set_extension("png");
        picto.save_with_format(path, ImageFormat::Png)?;
    }

    Ok(())
}

/// Add the classifiers to the timeline
fn parse_classifiers(state: &NowState, timeline: &mut Timeline) -> Result<(), Error> {
    let video_offset = i32::try_from(state.details.video_offset)?;
    for coach in 0..state.details.num_coach {
        let desc_file = state.bundle.open(&state.now.bundle_tree().classifiers_desc(coach))?;
        let desc: Moves = serde_json::from_slice(&desc_file)?;

        for class in desc {
            let start_time = (class.time - video_offset) / 20;
            let duration = class.duration / 20;
            let clip = Clip::Motion(MotionClip {
                is_active: true,
                start_time,
                duration,
                classifier_filename: HipStr::from(format!("{}.msm", class.name)),
                gold_move: class.gold_move.is_some(),
                coach_id: coach,
                color: Color::default(),
            });

            timeline.timeline.insert(clip);
        }
    }

    for classifier_path in state.bundle.read_dir(state.now.bundle_tree().classifiers())? {
        let classifier_file = state.bundle.open(classifier_path)?;
        let dest = state.song.moves().join(classifier_path.file_name().ok_or_else(|| anyhow!("No filename in {classifier_path}"))?);
        std::fs::write(dest, classifier_file.deref())?
    }

    Ok(())
}

/// Import the lyrics
fn create_karaoke_timeline(state: &NowState) -> Result<(), Error> {
    let mut timeline = Timeline {
        timeline: BTreeSet::new(),
    };

    let video_offset = i32::try_from(state.details.video_offset)?;

    for lyric in &state.details.lyrics {
        let clip = Clip::Karaoke(KaraokeClip {
            is_active: true,
            start_time: (lyric.time - video_offset) / 20,
            duration: lyric.duration / 20,
            pitch: 0.0,
            lyrics: lyric.text.clone(),
            is_end_of_line: lyric.is_line_ending == 1,
            content_type: 1,

        });
        timeline.timeline.insert(clip);
    }

    let karaoke_timeline_path = state.song.song().join("karaoke_timeline.json");

    let timeline_file = File::create(karaoke_timeline_path)?;
    serde_json::to_writer_pretty(timeline_file, &timeline)?;

    Ok(())
}

/// Create empty mainsequence
fn create_mainsequence(state: &NowState) -> Result<(), Error> {
    let timeline = Timeline {
        timeline: BTreeSet::new(),
    };

    let mainsequence_path = state.song.song().join("mainsequence.json");

    let mainsequence_file = File::create(mainsequence_path)?;
    serde_json::to_writer_pretty(mainsequence_file, &timeline)?;

    Ok(())
}

/// Create the musictrack
// TODO: Figure out start/end beat, sections
fn create_musictrack(state: &NowState) -> Result<(), Error> {
    let video_offset = state.details.video_offset;
    let start_beat = state.details.audio_preview.coverflow.startbeat;
    let musictrack = MusicTrack {
        start_beat: 0,
        end_beat: 0,
        video_start_time: -1.0 * ((video_offset as f32) / 1000.0),
        preview_entry: start_beat,
        preview_loop_start: start_beat,
        preview_loop_end: u32::try_from(state.details.beats.len()).unwrap_or(u32::MAX).min(start_beat + 150),
        signatures: vec![Signature {
            marker: 0,
            beats: 4,
        }],
        sections: vec![],
        markers: state.details.beats.iter().copied().map(|b| (b - video_offset) * 48).collect(),
    };

    let musictrack_path = state.song.song().join("musictrack.json");

    let musictrack_file = File::create(musictrack_path)?;
    serde_json::to_writer_pretty(musictrack_file, &musictrack)?;

    Ok(())
}

/// Transcode the video and put it in the song directory
fn create_video(state: &NowState) -> Result<&'static str, Error> {
    let filename = "main_video.webm";
    let from = state.now.video();
    let to_path = state.song.song().join(filename);

    println!("Transcoding video for {}", state.details.map_name);
    transcode(from, &to_path, state.transcode)?;

    Ok("main_video.webm")
}

/// Transcode the audio and put it in the song directory
fn create_audio(state: &NowState) -> Result<String, Error> {
    let filename = format!("{}.opus", state.details.map_name);
    let from = state.now.video();
    let to_path = state.song.song().join(&filename);

    extract_audio(from, &to_path)?;

    Ok(filename)

}

/// Create the menuart
fn create_menuart(state: &NowState) -> Result<(), Error> {
    let mut menuart = Vec::new();

    let cover = state.now.cover();
    std::fs::copy(cover, state.song.menuart().join("cover_generic.png"))?;
    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("cover_generic"),
        filename: HipStr::borrowed("cover_generic.png"),
        scale: (0.3, 0.3),
        pos2d: (266.08755, 197.62996),
        disable_shadow: 4294967295,
        anchor: 1,
    }));
    std::fs::copy(cover, state.song.menuart().join("cover_online_Kids.png"))?;
    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("cover_online_Kids"),
        filename: HipStr::borrowed("cover_online_Kids.png"),
        scale: (0.3, 0.3),
        pos2d: (-150.0, 0.0),
        disable_shadow: 4294967295,
        anchor: 1,
    }));
    std::fs::copy(cover, state.song.menuart().join("cover_online.png"))?;
    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("cover_online"),
        filename: HipStr::borrowed("cover_online.png"),
        scale: (0.3, 0.3),
        pos2d: (-150.0, 0.0),
        disable_shadow: 4294967295,
        anchor: 1,
    }));
    std::fs::copy(cover, state.song.menuart().join("cover_phone.png"))?;
    menuart.push(MenuArt::Phone(PhoneImage {
        name: HipStr::borrowed("cover_phone"),
        filename: HipStr::borrowed("cover_phone.png"),
    }));


    let coach_1 = state.now.coach(1);
    std::fs::copy(&coach_1, state.song.menuart().join("coach_1.png"))?;
    std::fs::copy(&coach_1, state.song.menuart().join("coach1_phone.png"))?;

    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("coach_1"),
        filename: HipStr::borrowed("coach_1.png"),
        scale: (0.290211, 0.290211),
        pos2d: (212.7845, 663.6802),
        disable_shadow: 4294967295,
        anchor: 6,
    }));
    menuart.push(MenuArt::Phone(PhoneImage {
        name: HipStr::borrowed("coach1_phone"),
        filename: HipStr::borrowed("coach1_phone.png"),
    }));

    for coach in 2..=state.details.num_coach {
        let path = state.now.coach(coach);
        std::fs::copy(&path, state.song.menuart().join(format!("coach_{coach}.png")))?;
        std::fs::copy(path, state.song.menuart().join(format!("coach{coach}_phone.png")))?;

        menuart.push(MenuArt::Texture(MenuArtTexture {
            name: HipStr::from(format!("coach_{coach}")),
            filename: HipStr::from(format!("coach_{coach}.png")),
            scale: (0.290211, 0.290211),
            pos2d: (524.3811, 670.82983),
            disable_shadow: 4294967295,
            anchor: 6,
        }));
        menuart.push(MenuArt::Phone(PhoneImage {
            name: HipStr::from(format!("coach{coach}_phone")),
            filename: HipStr::from(format!("coach_{coach}_phone.png")),
        }));
    }

    std::fs::copy(coach_1, state.song.menuart().join("cover_albumcoach.png"))?;
    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("cover_albumcoach"),
        filename: HipStr::borrowed("cover_albumcoach.png"),
        scale: (0.3, 0.3),
        pos2d: (738.1063, 359.61203),
        disable_shadow: 4294967295,
        anchor: 1,
    }));

    let map_bkg = state.now.map_bkg();
    std::fs::copy(&map_bkg, state.song.menuart().join("map_bkg.png"))?;
    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("map_bkg"),
        filename: HipStr::borrowed("map_bkg.png"),
        scale: (256.0, 128.0),
        pos2d: (1487.41, 350.0),
        disable_shadow: 1,
        anchor: 1,
    }));

    let menuart_path = state.song.menuart().join("menuart.json");
    let menuart_file = File::create(menuart_path)?;
    serde_json::to_writer_pretty(menuart_file, &menuart)?;

    Ok(())
}
