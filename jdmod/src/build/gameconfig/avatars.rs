//! # Avatars
//! Build the avatars
use std::collections::HashMap;

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::vfs::{VirtualFileSystem, VirtualPathBuf};
use hipstr::HipStr;
use ownable::traits::IntoOwned;
use ubiart_toolkit::{
    cooked,
    cooked::{
        isc::AvatarDesc,
        tpl::types::{AvatarDescription, AvatarDescription2022},
    },
    json_types::v22::GameManagerConfig22,
    utils::{SplitPath, UniqueGameId},
};

use crate::{
    build::{BuildFiles, BuildState},
    types::gameconfig::{
        avatars::{Avatar, UnlockType},
        gachacontent::GachaItem,
        generate_gacha_id,
    },
    utils::{cook_path, encode_texture},
};

/// Build the avatars
pub fn build(
    bs: &BuildState,
    bf: &mut BuildFiles,
    gameconfig: &mut GameManagerConfig22,
    gacha_items: &mut Vec<GachaItem>,
) -> Result<(), Error> {
    let avatars_file = bs
        .native_vfs
        .open(&bs.rel_tree.avatars().join("avatars.json"))?;

    let avatars =
        serde_json::from_slice::<HashMap<HipStr<'_>, Avatar>>(&avatars_file)?.into_owned();

    // Avatars can refer to other avatars, so the IDs need to be known before we start the conversions
    let mut id_map = HashMap::with_capacity(avatars.len());
    for name in avatars.keys() {
        let id = generate_gacha_id();
        id_map.insert(name.clone(), id);
    }

    let avatarsobjectives = &mut gameconfig.avatarsobjectives;
    avatarsobjectives.clear();
    let mut scene_actors = Vec::with_capacity(avatars.len());

    for (name, avatar) in avatars {
        let id = *id_map
            .get(name.as_str())
            .ok_or_else(|| anyhow!("Impossible!"))?;
        let avatar_dir = format!("world/avatars/{id:04}/");

        // Build the actor for the description
        let actor_path = format!("world/avatars/{id:04}/{id:04}.act");
        let actor_vec = desc_actor(&avatar_dir)?;

        // Encode the avatar image
        let cooked_image = encode_texture(
            bs.native_vfs,
            &bs.rel_tree.avatars().join(avatar.image_path.as_str()),
        )?;
        let to = cook_path(
            &format!("world/avatars/{id:04}/avatar.png"),
            UniqueGameId::NX2022,
        )?;
        let cooked_image_vec = cooked::png::create_vec(cooked_image)?;

        // Add the phone image for copying
        let phone_image = format!("world/avatars/{id:04}/avatar_phone.png");
        bf.static_files.add_file(
            bs.rel_tree.avatars().join(avatar.image_phone_path.as_str()),
            VirtualPathBuf::from(phone_image.clone()),
        )?;

        // Add an avatar objective or add it to the gacha items
        let unlock_type = avatar.unlock_type.normalize();
        let unlock_type_u32 = u32::from(&unlock_type);
        if unlock_type == UnlockType::GiftMachine {
            gacha_items.push(GachaItem::Avatar(id));
        } else if let UnlockType::Quest(quest) = unlock_type {
            avatarsobjectives.insert(id, quest);
        }

        // Create the avatar description
        let tpl = cooked::tpl::types::Actor {
            components: vec![
                cooked::tpl::types::Template::MaterialGraphicComponent(
                    cooked::tpl::types::MaterialGraphicComponent {
                        shadow_size: (1.8, 0.3),
                        shadow_dist: 4.0,
                        ..Default::default()
                    },
                ),
                cooked::tpl::types::Template::AvatarDescription(AvatarDescription::V2022(
                    AvatarDescription2022 {
                        relative_song_name: avatar.used_as_coach_map_name.clone(),
                        sound_family: avatar.sound_family,
                        status: avatar.status,
                        unlock_type: unlock_type_u32,
                        actor_path: HipStr::from(actor_path.clone()),
                        phone_image: HipStr::from(phone_image),
                        avatar_id: id,
                        used_as_coach_map_name: avatar.used_as_coach_map_name,
                        used_as_coach_coach_id: avatar.used_as_coach_coach_id,
                        special_effect: u32::from(avatar.special_effect),
                        main_avatar_id: avatar
                            .main_avatar
                            .and_then(|name| id_map.get(name.as_str()))
                            .copied()
                            .unwrap_or_else(|| u32::from(u16::MAX)),
                        ..Default::default()
                    },
                )),
            ],
            ..Default::default()
        };

        let desc_tpl_vec = cooked::json::create_vec_with_capacity_hint(&tpl, 3000)?;

        bf.generated_files.add_file(
            cook_path(
                &format!("world/avatars/{id:04}/desc.tpl"),
                UniqueGameId::NX2022,
            )?
            .into(),
            desc_tpl_vec,
        )?;

        bf.generated_files.add_file(to.into(), cooked_image_vec)?;

        bf.generated_files.add_file(
            cook_path(&actor_path, UniqueGameId::NX2022)?.into(),
            actor_vec,
        )?;

        let scene = desc_scene(id);
        scene_actors.push(scene);
    }

    let avatardb_scene_vec =
        cooked::isc::create_vec_with_capacity_hint(&avatardb_scene(bs, scene_actors), 940_000)?;
    bf.generated_files.add_file(
        cook_path(&gameconfig.avatardb_scene, UniqueGameId::NX2022)?.into(),
        avatardb_scene_vec,
    )?;

    Ok(())
}

/// Build the avatar description
fn desc_actor(avatar_dir: &str) -> Result<Vec<u8>, Error> {
    let actor = cooked::act::Actor {
        lua: SplitPath::new(HipStr::borrowed(avatar_dir), HipStr::borrowed("desc.tpl"))?,
        unk1: 0.0,
        unk2: 1.0,
        unk2_5: 1.0,
        unk3_5: 0,
        components: vec![
            cooked::act::Component::MaterialGraphicComponent(
                cooked::act::MaterialGraphicComponent {
                    // TODO: Check values!
                    files: [
                        SplitPath::new(
                            HipStr::borrowed(avatar_dir),
                            HipStr::borrowed("avatar.png"),
                        )?,
                        SplitPath::default(),
                        SplitPath::default(),
                        SplitPath::default(),
                        SplitPath::default(),
                        SplitPath::default(),
                        SplitPath::default(),
                        SplitPath::default(),
                        SplitPath::default(),
                        SplitPath::new(
                            HipStr::borrowed("world/ui/atlas/"),
                            HipStr::borrowed("avatar.atl"),
                        )?,
                        SplitPath::new(
                            HipStr::borrowed("world/ui/materials/_common/"),
                            HipStr::borrowed("alpha.msh"),
                        )?,
                    ],
                    ..Default::default()
                },
            ),
            cooked::act::Component::AvatarDescComponent,
        ],
    };

    Ok(cooked::act::create_vec(actor)?)
}

/// Build the description scene
fn desc_scene(id: u32) -> cooked::isc::WrappedActors<'static> {
    cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor {
        actor: Box::new(cooked::isc::Actor {
            relativez: 0.000_122,
            userfriendly: HipStr::from(format!("{id:04}")),
            pos2d: (-61.994_2, 37.990_77),
            lua: HipStr::from(format!("world/avatars/{id:04}/desc.tpl")),
            components: vec![
                cooked::isc::WrappedComponent::MaterialGraphic(
                    cooked::isc::MaterialGraphicComponent {
                        material: cooked::isc::Material {
                            gfx_material_serializable: cooked::isc::GFXMaterialSerializable {
                                atl_path: HipStr::borrowed("world/ui/atlas/avatar.atl"),
                                shader_path: HipStr::borrowed(
                                    "world/ui/materials/_common/alpha.msh",
                                ),
                                texture_set: cooked::isc::TextureSet {
                                    gfx_material_texture_path_set:
                                        cooked::isc::GFXMaterialTexturePathSet {
                                            diffuse: HipStr::from(format!(
                                                "world/avatars/{id:04}/avatar.png"
                                            )),
                                            ..Default::default()
                                        },
                                },
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    }
                    .into(),
                ),
                cooked::isc::WrappedComponent::AvatarDesc(AvatarDesc::default()),
            ],
            ..Default::default()
        }),
    })
}

/// Build the avatar database scene
fn avatardb_scene<'a>(
    bs: &BuildState,
    actors: Vec<cooked::isc::WrappedActors<'a>>,
) -> cooked::isc::Root<'a> {
    cooked::isc::Root {
        scene: cooked::isc::Scene {
            engine_version: bs.engine_version,
            view_family: true,
            actors,
            scene_configs: cooked::isc::SceneConfigs {
                active_scene_config: 0,
                jd_scene_config: vec![cooked::isc::WrappedJdSceneConfig::SongDatabase(
                    cooked::isc::SongDatabaseSceneConfig::default().into(),
                )],
            }
            .into(),
            ..Default::default()
        },
    }
}
