//! Import functionality for Just Dance Now data

use std::{collections::BTreeSet, fs::File, ops::Deref, path::Path};

use anyhow::{anyhow, Context, Error};
use bluestar_toolkit::{Moves, PictoAtlas, SongDetails};
use dotstar_toolkit_utils::vfs::zipfs::ZipFs;
use hipstr::HipStr;
use image::{imageops, DynamicImage, ImageFormat};
use tracing::{debug, info, trace};

use crate::{
    import::{
        jdnow::types::{NowState, NowTree},
        TranscodeSettings,
    },
    types::{
        localisation::LocaleId,
        song::{
            Clip, Color, GoldEffectClip, HideUserInterfaceClip, KaraokeClip, MenuArt,
            MenuArtTexture, MotionClip, MusicTrack, PhoneImage, PictogramClip, Signature, Song,
            SongDirectoryTree, SweatDifficulty, Timeline,
        },
        DirectoryTree,
    },
    utils::{extract_audio, transcode},
};

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

    let details_file = std::fs::read(tree_jdn.detail())
        .with_context(|| format!("Could not open {}", tree_jdn.detail().display()))?;
    let details: SongDetails = serde_json::from_slice(&details_file)?;
    let zipfs = ZipFs::new(
        File::open(tree_jdn.bundle())
            .with_context(|| format!("Could not open {}", tree_jdn.bundle().display()))?,
    )?;

    let beats = details.beats.as_slice();
    let beats = if beats[0] == 0 { &beats[1..] } else { beats };
    let diff = (beats[1] * 48) - (beats[0] * 48);
    let n_to_prepend = div_round(beats[0] * 48, diff).max(1);
    let mut new_beats = Vec::with_capacity(usize::try_from(n_to_prepend)? + beats.len());
    for i in 0..n_to_prepend {
        new_beats.push(diff * i);
    }
    new_beats.extend(beats.iter().map(|b| b * 48));

    let mut state = NowState {
        bundle: &zipfs,
        now: tree_jdn,
        song: tree_song,
        transcode: transcode_settings,
        details,
        beats: new_beats,
    };

    state.song.create_dir_all()?;
    let mut min_time = create_dance_timeline(&mut state)?;
    min_time = min_time.min(create_karaoke_timeline(&mut state)?);
    create_mainsequence(&state, min_time)?;
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
        // TODO: Add kids tag for kids songs
        tags: vec![HipStr::borrowed("main")],
        subtitle: LocaleId::default(),
        default_colors: (&state.details.default_colors).into(),
        audiofile: HipStr::from(audiofile),
        videofile: HipStr::borrowed(videofile),
    };

    let song_file = File::create(state.song.song_file())?;
    serde_json::to_writer_pretty(song_file, &song)?;

    Ok(())
}

/// Convert a JDNow duration to the UbiArt time
fn convert_duration(beats: &mut Vec<u32>, n: u32) -> i32 {
    let max = beats.len();
    let n = n * 48;
    match beats.binary_search(&n) {
        Ok(index) => {
            let m = index * 24;
            i32::try_from(m).unwrap_or(if m > 0 { i32::MAX } else { i32::MIN })
        }
        Err(0) => {
            debug!("Duration out of bounds! n: {n}, first beat: {}", beats[0]);
            0
        }
        Err(index) if index == max => {
            debug!(
                "Duration out of bounds! n: {n}, last beat: {}",
                beats[max - 1]
            );
            let diff = beats[max - 1] - beats[max - 2];
            let mut beat = beats[max - 1];
            while n > beat {
                beat += diff;
                beats.push(beat);
            }
            convert_duration(beats, n)
        }
        Err(index) => {
            let x0 = beats[index - 1];
            let x1 = beats[index];
            let y0 = u32::try_from((index - 1) * 24).unwrap_or(u32::MAX);
            let y1 = u32::try_from(index * 24).unwrap_or(u32::MAX);

            let m = (y0 * (x1 - n) + (y1 * (n - x0))) / (x1 - x0);
            trace!("n: {n}, x0: {x0}, y0: {y0}, x1: {x1}, m: {m}");
            i32::try_from(m).unwrap_or(if m > 0 { i32::MAX } else { i32::MIN })
        }
    }
}

/// Import the pictos and classifiers
fn create_dance_timeline(state: &mut NowState) -> Result<i32, Error> {
    let mut timeline = Timeline {
        timeline: BTreeSet::new(),
    };

    parse_picto_atlas(state)?;
    let mut min_time = parse_pictos(state, &mut timeline);
    min_time = min_time.min(parse_classifiers(state, &mut timeline)?);

    let dance_timeline_path = state.song.song().join("dance_timeline.json");

    let timeline_file = File::create(dance_timeline_path)?;
    serde_json::to_writer_pretty(timeline_file, &timeline)?;

    Ok(min_time)
}

/// Add the pictos to the timeline
fn parse_pictos(state: &mut NowState, timeline: &mut Timeline) -> i32 {
    let mut min_time = 0;

    for picto in &state.details.pictos {
        let start_time = convert_duration(&mut state.beats, picto.time);
        let duration = convert_duration(&mut state.beats, picto.duration);
        let clip = Clip::Pictogram(PictogramClip {
            is_active: true,
            start_time,
            duration,
            picto_filename: HipStr::from(format!("{}.png", picto.name)),
        });
        min_time = min_time.min(clip.start_time());

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

    min_time
}

/// Extract the pictos from the atlas
fn parse_picto_atlas(state: &NowState) -> Result<(), Error> {
    let atlas_desc_file = std::fs::read(state.now.atlas_desc())?;
    let atlas_desc: PictoAtlas = serde_json::from_slice(&atlas_desc_file)?;
    let width = atlas_desc.image_size.width;
    let height = atlas_desc.image_size.height;

    let image = image::open(state.now.atlas())?;

    for (name, (x, y)) in atlas_desc.images {
        let picto = DynamicImage::from(imageops::crop_imm(&image, x, y, width, height).to_image());
        let picto = picto.resize(1024, 512, imageops::FilterType::Lanczos3);
        let mut path = state.song.pictos().join(&name);
        path.set_extension("png");
        picto.save_with_format(path, ImageFormat::Png)?;
    }

    Ok(())
}

/// Add the classifiers to the timeline
fn parse_classifiers(state: &mut NowState, timeline: &mut Timeline) -> Result<i32, Error> {
    let mut min_time = 0;
    for coach in 0..state.details.num_coach {
        let desc_file = state
            .bundle
            .open(&state.now.bundle_tree().classifiers_desc(coach))?;
        let desc: Moves = serde_json::from_slice(&desc_file)?;

        for class in desc {
            let start_time = convert_duration(&mut state.beats, class.time);
            let duration = convert_duration(&mut state.beats, class.duration);
            let clip = Clip::Motion(MotionClip {
                is_active: true,
                start_time,
                duration,
                classifier_filename: HipStr::from(format!("{}.msm", class.name)),
                gold_move: class.gold_move.is_some(),
                coach_id: coach,
                color: Color::default(),
            });

            min_time = min_time.min(clip.start_time());

            timeline.timeline.insert(clip);
        }
    }

    for classifier_path in state
        .bundle
        .read_dir(state.now.bundle_tree().classifiers())?
    {
        let classifier_file = state.bundle.open(classifier_path)?;
        let dest = state.song.moves().join(
            classifier_path
                .file_name()
                .ok_or_else(|| anyhow!("No filename in {classifier_path}"))?,
        );
        std::fs::write(dest, classifier_file.deref())?;
    }

    Ok(min_time)
}

/// Import the lyrics
fn create_karaoke_timeline(state: &mut NowState) -> Result<i32, Error> {
    let mut timeline = Timeline {
        timeline: BTreeSet::new(),
    };

    let mut min_time = 0;

    for lyric in &state.details.lyrics {
        let start_time = convert_duration(&mut state.beats, lyric.time);
        let duration = convert_duration(&mut state.beats, lyric.duration);
        let clip = Clip::Karaoke(KaraokeClip {
            is_active: true,
            start_time,
            duration,
            pitch: 0.0,
            lyrics: lyric.text.clone(),
            is_end_of_line: lyric.is_line_ending == 1,
            content_type: 1,
        });
        min_time = min_time.min(clip.start_time());
        timeline.timeline.insert(clip);
    }

    let karaoke_timeline_path = state.song.song().join("karaoke_timeline.json");

    let timeline_file = File::create(karaoke_timeline_path)?;
    serde_json::to_writer_pretty(timeline_file, &timeline)?;

    Ok(min_time)
}

/// Create empty mainsequence
fn create_mainsequence(state: &NowState, min_time: i32) -> Result<(), Error> {
    let mut timeline = Timeline {
        timeline: BTreeSet::new(),
    };

    if min_time > 100 {
        let clip = Clip::HideUserInterface(HideUserInterfaceClip {
            is_active: true,
            start_time: 0,
            duration: min_time - 100,
            event_type: 18,
        });
        timeline.timeline.insert(clip);
    }

    let mainsequence_path = state.song.song().join("mainsequence.json");

    let mainsequence_file = File::create(mainsequence_path)?;
    serde_json::to_writer_pretty(mainsequence_file, &timeline)?;

    Ok(())
}

/// Create the musictrack
// TODO: Figure out start/end beat, sections
fn create_musictrack(state: &NowState) -> Result<(), Error> {
    let start_beat = state.details.audio_preview.coverflow.startbeat;
    let beats_len = u32::try_from(state.details.beats.len()).unwrap_or(u32::MAX) * 24;
    let musictrack = MusicTrack {
        start_beat: 0,
        end_beat: beats_len,
        video_start_time: 0.0,
        preview_entry: start_beat,
        preview_loop_start: start_beat,
        preview_loop_end: beats_len.min(start_beat + 150),
        signatures: vec![Signature {
            marker: 0,
            beats: 4,
        }],
        sections: vec![],
        markers: state
            .details
            .beats
            .iter()
            .copied()
            .map(|b| b * 48)
            .collect(),
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
    let to_path = state.song.audio().join(&filename);

    extract_audio(from, &to_path)?;

    Ok(filename)
}

/// Create the menuart
fn create_menuart(state: &NowState) -> Result<(), Error> {
    let mut menuart = Vec::new();

    let orig_cover = image::open(state.now.cover())?;
    let cover = orig_cover.resize(1024, 1024, imageops::FilterType::Lanczos3);
    cover.save(state.song.menuart().join("cover_generic.png"))?;
    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("cover_generic"),
        filename: HipStr::borrowed("cover_generic.png"),
        scale: (0.3, 0.3),
        pos2d: (266.08755, 197.62996),
        disable_shadow: 4_294_967_295,
        anchor: 1,
    }));
    cover.save(state.song.menuart().join("cover_online_Kids.png"))?;
    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("cover_online_Kids"),
        filename: HipStr::borrowed("cover_online_Kids.png"),
        scale: (0.3, 0.3),
        pos2d: (-150.0, 0.0),
        disable_shadow: 4_294_967_295,
        anchor: 1,
    }));
    cover.save(state.song.menuart().join("cover_online.png"))?;
    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("cover_online"),
        filename: HipStr::borrowed("cover_online.png"),
        scale: (0.3, 0.3),
        pos2d: (-150.0, 0.0),
        disable_shadow: 4_294_967_295,
        anchor: 1,
    }));
    let cover = orig_cover.resize(1024, 1024, imageops::FilterType::Lanczos3);
    cover.save(state.song.menuart().join("cover_phone.jpg"))?;
    menuart.push(MenuArt::Phone(PhoneImage {
        name: HipStr::borrowed("cover"),
        filename: HipStr::borrowed("cover_phone.jpg"),
    }));

    let orig_coach_1 = image::open(state.now.coach(1))?;
    let coach_1 = orig_coach_1.resize(1024, 1024, imageops::FilterType::Lanczos3);
    coach_1.save(state.song.menuart().join("coach_1.png"))?;
    let coach_1 = orig_coach_1.resize(256, 256, imageops::FilterType::Lanczos3);
    coach_1.save(state.song.menuart().join("coach1_phone.png"))?;

    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("coach_1"),
        filename: HipStr::borrowed("coach_1.png"),
        scale: (0.290_211, 0.290_211),
        pos2d: (212.7845, 663.6802),
        disable_shadow: 4_294_967_295,
        anchor: 6,
    }));
    menuart.push(MenuArt::Phone(PhoneImage {
        name: HipStr::borrowed("coach1"),
        filename: HipStr::borrowed("coach1_phone.png"),
    }));

    for coach in 2..=state.details.num_coach {
        let orig_coach = image::open(state.now.coach(coach))?;
        let coach_image = orig_coach.resize(1024, 1024, imageops::FilterType::Lanczos3);
        coach_image.save(state.song.menuart().join(format!("coach_{coach}.png")))?;
        let coach_image = orig_coach.resize(256, 256, imageops::FilterType::Lanczos3);
        coach_image.save(state.song.menuart().join(format!("coach{coach}_phone.png")))?;

        menuart.push(MenuArt::Texture(MenuArtTexture {
            name: HipStr::from(format!("coach_{coach}")),
            filename: HipStr::from(format!("coach_{coach}.png")),
            scale: (0.290_211, 0.290_211),
            pos2d: (524.3811, 670.82983),
            disable_shadow: 4_294_967_295,
            anchor: 6,
        }));
        menuart.push(MenuArt::Phone(PhoneImage {
            name: HipStr::from(format!("coach{coach}")),
            filename: HipStr::from(format!("coach{coach}_phone.png")),
        }));
    }

    let cover_albumcoach = orig_coach_1.resize(1024, 1024, imageops::FilterType::Lanczos3);
    cover_albumcoach.save(state.song.menuart().join("cover_albumcoach.png"))?;
    menuart.push(MenuArt::Texture(MenuArtTexture {
        name: HipStr::borrowed("cover_albumcoach"),
        filename: HipStr::borrowed("cover_albumcoach.png"),
        scale: (0.3, 0.3),
        pos2d: (738.1063, 359.61203),
        disable_shadow: 4_294_967_295,
        anchor: 1,
    }));

    if let Ok(map_bkg) = image::open(state.now.map_bkg()) {
        map_bkg.save(state.song.menuart().join("map_bkg.png"))?;
        menuart.push(MenuArt::Texture(MenuArtTexture {
            name: HipStr::borrowed("map_bkg"),
            filename: HipStr::borrowed("map_bkg.png"),
            scale: (256.0, 128.0),
            pos2d: (1487.41, 350.0),
            disable_shadow: 1,
            anchor: 1,
        }));
    } else {
        info!("Failed to import map_bkg for {}", state.details.map_name);
    }

    let menuart_path = state.song.menuart().join("menuart.json");
    let menuart_file = File::create(menuart_path)?;
    serde_json::to_writer_pretty(menuart_file, &menuart)?;

    Ok(())
}

/// Divide the divisor by the dividend, rounding the result to
/// the nearest whole integer
const fn div_round(divisor: u32, dividend: u32) -> u32 {
    (dividend + (divisor >> 1)) / divisor
}
