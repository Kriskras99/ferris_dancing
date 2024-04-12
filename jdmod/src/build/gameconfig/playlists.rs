//! # Playlists Building
//! Build the playlists
use std::{borrow::Cow, collections::HashMap, path::Path};

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::{testing::test_eq, vfs::VirtualFileSystem};
use ubiart_toolkit::{
    cooked,
    json_types::{self, v22::GameManagerConfig22},
    utils::SplitPath,
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
    let saved_playlists: HashMap<Cow<'_, str>, Playlist> =
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
        let file_stem = AsRef::<Path>::as_ref(cover.as_ref())
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or_else(|| anyhow!("Failure parsing filename!"))?;
        let offline_playlist = playlist.into_offline_playlist()?;

        let tga = format!("{file_stem}.tga");
        let cover_actor_vec = cover_actor(&tga)?;

        let cooked_cover =
            encode_texture(bs.native_vfs, &bs.rel_tree.playlists().join(cover.as_ref()))?;
        let cooked_cover_vec = cooked::png::create_vec(&cooked_cover)?;
        bf.generated_files.add_file(
            cook_path(offline_playlist.cover_path.as_ref(), bs.platform)?.into(),
            cover_actor_vec,
        )?;
        bf.generated_files.add_file(
            cook_path(
                &format!("world/ui/textures/covers/playlists_offline/{tga}"),
                bs.platform,
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
        cook_path(&gameconfig.config_files_path.playlist, bs.platform)?.into(),
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
    let carousel_rules_path = cook_path(carousel_rules, bs.platform)?;
    let template_file = bs.patched_base_vfs.open(carousel_rules_path.as_ref())?;
    let mut carousel_rules = cooked::json::parse_v22(&template_file, false)?
        .carousel_rules()?
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
    test_eq(&carousel_rule.categories.len(), &1)
        .context("More than one category in carousel_rule!")?;
    let recommended = carousel_rule
        .categories
        .first()
        .ok_or_else(|| anyhow!("Recommended not in categories"))?;
    requests.retain(|c| !recommended.requests.contains(c));

    // Add a new playlist carousel called 'Themed Playlists' for all other playlists
    // TODO: Investigate adding a carousel per Game
    let category_rule = json_types::isg::CategoryRule {
        class: Some(json_types::isg::CategoryRule::CLASS),
        act: Cow::Borrowed("ui_carousel"),
        isc: Cow::Borrowed("grp_row"),
        title: Cow::Borrowed("Themed"),
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
        tpl: SplitPath::new(
            Cow::Borrowed("enginedata/actortemplates/"),
            Cow::Borrowed("tpl_materialgraphiccomponent2d.tpl"),
        )?,
        unk1: 0,
        unk2: 0x3F80_0000,
        unk2_5: 0x3F80_0000,
        components: vec![cooked::act::Component::MaterialGraphicComponent(
            cooked::act::MaterialGraphicComponent {
                // TODO: Check values!
                files: [
                    SplitPath::new(
                        Cow::Borrowed("world/ui/textures/covers/playlists_offline/"),
                        Cow::Borrowed(tga),
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
                        Cow::Borrowed("world/_common/matshader/"),
                        Cow::Borrowed("multitexture_1layer.msh"),
                    )?,
                ],
                ..Default::default()
            },
        )],
    };

    Ok(cooked::act::create_vec(&actor)?)
}
