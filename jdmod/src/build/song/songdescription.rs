//! # Song Description Building
//! Builds the songdesc files
use std::collections::HashMap;

use anyhow::Error;
use hipstr::HipStr;
use ubiart_toolkit::{cooked, utils::SplitPath};

use super::SongExportState;
use crate::{build::BuildFiles, types::song::Tag};

/// Builds the songdesc files
pub fn build(ses: &SongExportState<'_>, bf: &mut BuildFiles) -> Result<(), Error> {
    let map_path = ses.map_path;
    let cache_map_path = ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let mut phone_images = HashMap::new();
    phone_images.insert(
        HipStr::borrowed("cover"),
        HipStr::from(
            map_path
                .join(format!("menuart/textures/{lower_map_name}_cover_phone.jpg"))
                .into_string(),
        ),
    );
    for i in 1..=u32::from(ses.song.number_of_coaches) {
        phone_images.insert(
            HipStr::from(format!("coach{i}")),
            HipStr::from(
                map_path
                    .join(format!(
                        "menuart/textures/{lower_map_name}_coach_{i}_phone.png"
                    ))
                    .into_string(),
            ),
        );
    }

    let song_desc_tpl = cooked::tpl::types::Actor {
        components: vec![cooked::tpl::types::Template::SongDescription(
            cooked::tpl::types::SongDescription {
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
                    .and_then(|n| i32::try_from(n).ok())
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
                video_preview_path: HipStr::borrowed(""),
                double_scoring_type: None,
                paths: None,
                energy: None,
                score_with_both_controllers: None,
                jdm_attributes: None,
            },
        )],
        ..Default::default()
    };

    let song_desc_act = cooked::act::Actor {
        lua: SplitPath::new(
            HipStr::borrowed(ses.map_path.as_str()),
            HipStr::borrowed("songdesc.tpl"),
        )?,
        unk1: 0.0,
        unk2: 1.0,
        unk2_5: 1.0,
        unk3_5: 0,
        components: vec![cooked::act::Component::SongDescComponent],
    };

    let song_desc_tpl_vec = cooked::json::create_vec(&song_desc_tpl)?;
    let song_desc_act_vec = cooked::act::create_vec(song_desc_act)?;

    bf.generated_files
        .add_file(cache_map_path.join("songdesc.tpl.ckd"), song_desc_tpl_vec)?;
    bf.generated_files
        .add_file(cache_map_path.join("songdesc.act.ckd"), song_desc_act_vec)?;

    Ok(())
}
