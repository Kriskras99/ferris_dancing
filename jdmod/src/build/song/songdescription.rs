//! # Song Description Building
//! Builds the songdesc files
use std::{borrow::Cow, collections::HashMap};

use anyhow::Error;
use ubiart_toolkit::{cooked, json_types, utils::SplitPath};

use super::SongExportState;
use crate::{build::BuildFiles, types::song::Tag};

/// Builds the songdesc files
pub fn build(ses: &SongExportState<'_>, bf: &mut BuildFiles) -> Result<(), Error> {
    let map_path = ses.map_path;
    let cache_map_path = ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let mut phone_images = HashMap::new();
    phone_images.insert(
        Cow::Borrowed("cover"),
        Cow::Owned(format!(
            "{map_path}/menuart/textures/{lower_map_name}_cover_phone.jpg"
        )),
    );
    for i in 1..=(u8::from(ses.song.number_of_coaches)) {
        phone_images.insert(
            Cow::Owned(format!("coach{i}")),
            Cow::Owned(format!(
                "{map_path}/menuart/textures/{lower_map_name}_coach_{i}_phone.png"
            )),
        );
    }

    let song_desc_tpl = json_types::v22::Template22::Actor(json_types::v22::Actor22 {
        components: vec![json_types::v22::Template22::SongDescription(
            json_types::just_dance::SongDescription {
                class: None,
                map_name: ses.song.map_name.clone(),
                jd_version: 2022,
                original_jd_version: ses.song.original_jd_version,
                related_albums: ses.song.related_songs.clone(),
                artist: ses.song.artist.clone(),
                cn_lyrics: None,
                dancer_name: ses.song.dancer_name.clone(),
                title: ses.song.title.clone(),
                credits: ses.song.credits.clone(),
                sub_title: None,
                sub_credits: None,
                phone_images,
                num_coach: ses.song.number_of_coaches.into(),
                main_coach: ses
                    .song
                    .main_coach
                    .and_then(|n| i8::try_from(n).ok())
                    .unwrap_or(-1),
                difficulty: ses.song.difficulty.into(),
                sweat_difficulty: ses.song.sweat_difficulty.into(),
                background_type: 0,
                lyrics_type: 0,
                tags: ses.song.tags.iter().copied().map(Tag::to_cow).collect(),
                status: ses.song.status.normalize().into(),
                locale_id: ses.song.subtitle,
                mojo_value: 0,
                count_in_progression: 1,
                default_colors: (&ses.song.default_colors).into(),
                video_preview_path: Cow::Borrowed(""),
                double_scoring_type: None,
                paths: None,
                energy: None,
                score_with_both_controllers: None,
                jdm_attributes: None,
            },
        )],
        ..Default::default()
    });

    let song_desc_act = cooked::act::Actor {
        tpl: SplitPath {
            path: Cow::Borrowed(ses.map_path),
            filename: Cow::Borrowed("songdesc.tpl"),
        },
        unk1: 0,
        unk2: 0x3F80_0000,
        unk2_5: 0x3F80_0000,
        components: vec![cooked::act::Component::SongDescComponent],
    };

    let song_desc_tpl_vec = cooked::json::create_vec(&song_desc_tpl)?;
    let song_desc_act_vec = cooked::act::create_vec(&song_desc_act)?;

    bf.generated_files.add_file(
        format!("{cache_map_path}/songdesc.tpl.ckd").into(),
        song_desc_tpl_vec,
    )?;
    bf.generated_files.add_file(
        format!("{cache_map_path}/songdesc.act.ckd").into(),
        song_desc_act_vec,
    )?;

    Ok(())
}
