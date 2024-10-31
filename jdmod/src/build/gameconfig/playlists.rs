//! # Playlists Building
//! Build the playlists
use std::collections::HashMap;

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::vfs::{VirtualFileSystem, VirtualPath};
use hipstr::HipStr;
use test_eq::test_eq;
use ubiart_toolkit::{
    cooked,
    json_types::{self, v22::GameManagerConfig22},
    utils::{SplitPath, UniqueGameId},
};

use crate::{
    build::{BuildFiles, BuildState},
    types::gameconfig::playlists::Playlist,
    utils::{cook_path, encode_texture},
};

/// Build the playlists
pub fn build(
    bs: &BuildState,
    bf: &mut BuildFiles,
    gameconfig: &GameManagerConfig22<'_>,
) -> Result<(), Error> {
    let saved_playlists_file = bs
        .native_vfs
        .open(&bs.rel_tree.playlists().join("playlists.json"))?;
    let saved_playlists: HashMap<HipStr<'_>, Playlist> =
        serde_json::from_slice(&saved_playlists_file)?;

    let mut playlists = HashMap::with_capacity(saved_playlists.len());
    let mut requests = Vec::new();

    for (name, playlist) in saved_playlists {
        requests.push(json_types::isg::CarouselRequestDesc::Playlists(
            json_types::isg::CarouselPlaylistsRequestDesc {
                playlist_id: name.clone(),
                ..Default::default()
            },
        ));

        let cover = playlist.cover.clone();
        let file_stem = VirtualPath::new(cover.as_str())
            .file_stem()
            .ok_or_else(|| anyhow!("Failure parsing filename!"))?;
        let offline_playlist = playlist.into_offline_playlist()?;

        let tga = format!("{file_stem}.tga");
        let cover_actor_vec = cover_actor(&tga)?;

        let cooked_cover =
            encode_texture(bs.native_vfs, &bs.rel_tree.playlists().join(cover.as_str()))?;
        let cooked_cover_vec = cooked::png::create_vec(cooked_cover)?;
        bf.generated_files.add_file(
            cook_path(offline_playlist.cover_path.as_str(), UniqueGameId::NX2022)?.into(),
            cover_actor_vec,
        )?;
        bf.generated_files.add_file(
            cook_path(
                &format!("world/ui/textures/covers/playlists_offline/{tga}"),
                UniqueGameId::NX2022,
            )?
            .into(),
            cooked_cover_vec,
        )?;

        playlists.insert(name, offline_playlist);
    }

    let template =
        json_types::v22::Template22::PlaylistDatabase(json_types::isg::PlaylistDatabase {
            class: None,
            playlists,
        });

    let template_vec = cooked::json::create_vec(&template)?;
    bf.generated_files.add_file(
        cook_path(&gameconfig.config_files_path.playlist, UniqueGameId::NX2022)?.into(),
        template_vec,
    )?;

    build_carousel(bs, bf, requests, &gameconfig.carousel_rules)?;

    Ok(())
}

/// Build the carousel
fn build_carousel(
    bs: &BuildState,
    bf: &mut BuildFiles,
    mut requests: Vec<json_types::isg::CarouselRequestDesc<'_>>,
    carousel_rules: &str,
) -> Result<(), Error> {
    let carousel_rules_path = cook_path(carousel_rules, UniqueGameId::NX2022)?;
    let template_file = bs.patched_base_vfs.open(carousel_rules_path.as_ref())?;
    let mut carousel_rules = cooked::json::parse_v22(&template_file, false)?
        .into_carousel_rules()?
        .clone();

    // Remove existing playlist carousels except for 'Recommended for me',
    // then remove any playlist that's in 'Recommended for me' from requests so that there are no dupes.
    let carousel_rule = carousel_rules
        .rules
        .get_mut("/jd2022-playlists")
        .ok_or_else(|| anyhow!("Playlist rule not found in carousel"))?;
    carousel_rule
        .categories
        .retain(|c| c.title == "Recommended for me");
    test_eq!(carousel_rule.categories.len(), 1)?;
    let recommended = carousel_rule
        .categories
        .first()
        .ok_or_else(|| anyhow!("Recommended not in categories"))?;
    requests.retain(|c| !recommended.requests.contains(c));

    // Add a new playlist carousel called 'Themed Playlists' for all other playlists
    // TODO: Investigate adding a carousel per Game
    let category_rule = json_types::isg::CategoryRule {
        class: Some(json_types::isg::CategoryRule::CLASS),
        act: HipStr::borrowed("ui_carousel"),
        isc: HipStr::borrowed("grp_row"),
        title: HipStr::borrowed("Themed"),
        title_id: 0x39AB, // 'Themed Playlists'
        requests,
        filters: Vec::new(),
    };

    carousel_rule.categories.push(category_rule);

    let carousel_vec = cooked::json::create_vec_with_capacity_hint(
        &json_types::v22::Template22::CarouselRules(carousel_rules),
        100_000,
    )?;
    bf.generated_files
        .add_file(carousel_rules_path.into(), carousel_vec)?;

    Ok(())
}

/// Build the cover actor
fn cover_actor(tga: &str) -> Result<Vec<u8>, Error> {
    let actor = cooked::act::Actor {
        lua: SplitPath::new(
            HipStr::borrowed("enginedata/actortemplates/"),
            HipStr::borrowed("tpl_materialgraphiccomponent2d.tpl"),
        )?,
        unk1: 0.0,
        unk2: 1.0,
        unk2_5: 1.0,
        unk3_5: 0,
        components: vec![cooked::act::Component::MaterialGraphicComponent(
            cooked::act::MaterialGraphicComponent {
                // TODO: Check values!
                files: [
                    SplitPath::new(
                        HipStr::borrowed("world/ui/textures/covers/playlists_offline/"),
                        HipStr::borrowed(tga),
                    )?,
                    SplitPath::default(),
                    SplitPath::default(),
                    SplitPath::default(),
                    SplitPath::default(),
                    SplitPath::default(),
                    SplitPath::default(),
                    SplitPath::default(),
                    SplitPath::default(),
                    SplitPath::default(),
                    SplitPath::new(
                        HipStr::borrowed("world/_common/matshader/"),
                        HipStr::borrowed("multitexture_1layer.msh"),
                    )?,
                ],
                ..Default::default()
            },
        )],
    };

    Ok(cooked::act::create_vec(actor)?)
}
