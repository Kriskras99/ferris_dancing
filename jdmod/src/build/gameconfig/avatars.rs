//! # Avatars
//! Build the avatars
use std::{borrow::Cow, collections::HashMap, fs::File, path::PathBuf};

use anyhow::{anyhow, Error};
use ubiart_toolkit::{
    cooked,
    json_types::{
        self,
        v22::{AvatarDescription22, GameManagerConfig22},
    },
    utils::SplitPath,
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
    gameconfig: &mut GameManagerConfig22<'_>,
    gacha_items: &mut Vec<GachaItem>,
) -> Result<(), Error> {
    let avatars: HashMap<Cow<'_, str>, Avatar> =
        serde_json::from_reader(File::open(bs.dirs.avatars().join("avatars.json"))?)?;

    // Avatars can refer to other avatars, so the IDs need to be known before we start the conversions
    let mut id_map = HashMap::with_capacity(avatars.len());
    for name in avatars.keys() {
        let id = generate_gacha_id();
        id_map.insert(name.clone(), id);
    }

    let avatarsobjectives = &mut gameconfig.avatarsobjectives;
    let mut scene_actors = Vec::with_capacity(avatars.len());

    for (name, avatar) in avatars {
        let id = *id_map
            .get(name.as_ref())
            .ok_or_else(|| anyhow!("Impossible!"))?;
        let avatar_dir = format!("world/avatars/{id:04}/");

        // Build the actor for the description
        let actor_path = format!("world/avatars/{id:04}/{id:04}.act");
        let actor_vec = desc_actor(&avatar_dir)?;

        // Encode the avatar image
        let cooked_image = encode_texture(&bs.dirs.avatars().join(avatar.image_path.as_ref()))?;
        let to = cook_path(&format!("world/avatars/{id:04}/avatar.png"), bs.platform)?;
        let cooked_image_vec = cooked::png::create_vec(&cooked_image)?;

        // Add the phone image for copying
        let phone_image = format!("world/avatars/{id:04}/avatar_phone.png");
        bf.static_files.add_file(
            bs.dirs.avatars().join(avatar.image_phone_path.as_ref()),
            PathBuf::from(phone_image.clone()),
        )?;

        // Add an avatar objective or add it to the gacha items
        let unlock_type = avatar.unlock_type.normalize();
        let unlock_type_u8 = u8::from(&unlock_type);
        if unlock_type == UnlockType::GiftMachine {
            gacha_items.push(GachaItem::Avatar(id));
        } else if let UnlockType::Quest(quest) = unlock_type {
            avatarsobjectives.insert(id, quest);
        }

        // Create the avatar description
        let tpl = json_types::v22::Template22::Actor(json_types::v22::Actor22 {
            components: vec![
                json_types::v22::Template22::MaterialGraphicComponent(
                    json_types::MaterialGraphicComponent {
                        shadow_size: (1.8, 0.3),
                        shadow_dist: 4,
                        ..Default::default()
                    },
                ),
                json_types::v22::Template22::AvatarDescription(AvatarDescription22 {
                    relative_song_name: avatar.used_as_coach_map_name.clone(),
                    sound_family: avatar.sound_family,
                    status: avatar.status,
                    unlock_type: unlock_type_u8,
                    actor_path: Cow::Owned(actor_path.clone()),
                    phone_image: Cow::Owned(phone_image),
                    avatar_id: id,
                    used_as_coach_map_name: avatar.used_as_coach_map_name,
                    used_as_coach_coach_id: avatar.used_as_coach_coach_id,
                    special_effect: Some(u8::from(avatar.special_effect)),
                    main_avatar_id: Some(
                        avatar
                            .main_avatar
                            .and_then(|name| id_map.get(name.as_ref()))
                            .copied()
                            .unwrap_or(u16::MAX),
                    ),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        });

        let desc_tpl_vec = cooked::json::create_vec_with_capacity_hint(&tpl, 3000)?;

        bf.generated_files.add_file(
            cook_path(&format!("world/avatars/{id:04}/desc.tpl"), bs.platform)?,
            desc_tpl_vec,
        );

        bf.generated_files.add_file(to, cooked_image_vec);

        bf.generated_files
            .add_file(cook_path(&actor_path, bs.platform)?, actor_vec);

        let scene = desc_scene(id);
        scene_actors.push(scene);
    }

    let avatardb_scene_vec =
        cooked::isc::create_vec_with_capacity_hint(&avatardb_scene(bs, scene_actors), 940_000)?;
    bf.generated_files.add_file(
        cook_path(&gameconfig.avatardb_scene, bs.platform)?,
        avatardb_scene_vec,
    );

    Ok(())
}

/// Build the avatar description
fn desc_actor(avatar_dir: &str) -> Result<Vec<u8>, Error> {
    let actor = cooked::act::Actor {
        tpl: SplitPath {
            path: Cow::Borrowed(avatar_dir),
            filename: Cow::Borrowed("desc.tpl"),
        },
        unk1: 0,
        unk2: 0x3F80_0000,
        unk2_5: 0x3F80_0000,
        components: vec![
            cooked::act::Component {
                the_type: cooked::act::ComponentType::MaterialGraphicComponent,
                data: cooked::act::ComponentData::MaterialGraphicComponent(Box::new(
                    cooked::act::MaterialGraphicComponent {
                        // TODO: Check values!
                        files: [
                            SplitPath {
                                path: Cow::Borrowed(avatar_dir),
                                filename: Cow::Borrowed("avatar.png"),
                            },
                            SplitPath::default(),
                            SplitPath::default(),
                            SplitPath::default(),
                            SplitPath::default(),
                            SplitPath::default(),
                            SplitPath::default(),
                            SplitPath::default(),
                            SplitPath::default(),
                            SplitPath {
                                path: Cow::Borrowed("world/ui/atlas/"),
                                filename: Cow::Borrowed("avatar.atl"),
                            },
                            SplitPath {
                                path: Cow::Borrowed("world/ui/materials/_common/"),
                                filename: Cow::Borrowed("alpha.msh"),
                            },
                        ],
                        ..Default::default()
                    },
                )),
            },
            cooked::act::Component {
                the_type: cooked::act::ComponentType::AvatarDescComponent,
                data: cooked::act::ComponentData::None,
            },
        ],
    };

    cooked::act::create_vec(&actor)
}

/// Build the description scene
fn desc_scene(id: u16) -> cooked::isc::WrappedActors<'static> {
    cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor {
        actor: cooked::isc::Actor {
            relativez: 0.000_122,
            userfriendly: Cow::Owned(format!("{id:04}")),
            pos2d: (-61.994_202, 37.990_768),
            lua: Cow::Owned(format!("world/avatars/{id:04}/desc.tpl")),
            components: vec![
                cooked::isc::WrappedComponent::MaterialGraphic(
                    cooked::isc::WrappedMaterialGraphicComponent {
                        material_graphic_component: cooked::isc::MaterialGraphicComponent {
                            material: cooked::isc::Material {
                                gfx_material_serializable: cooked::isc::GFXMaterialSerializable {
                                    atl_path: Cow::Borrowed("world/ui/atlas/avatar.atl"),
                                    shader_path: Cow::Borrowed(
                                        "world/ui/materials/_common/alpha.msh",
                                    ),
                                    texture_set: cooked::isc::TextureSet {
                                        gfx_material_texture_path_set:
                                            cooked::isc::GFXMaterialTexturePathSet {
                                                diffuse: Cow::Owned(format!(
                                                    "world/avatars/{id:04}/avatar.png"
                                                )),
                                                ..Default::default()
                                            },
                                    },
                                    ..Default::default()
                                },
                            },
                            ..Default::default()
                        },
                    },
                ),
                cooked::isc::WrappedComponent::AvatarDesc,
            ],
            ..Default::default()
        },
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
            scene_configs: cooked::isc::WrappedSceneConfigs {
                scene_configs: cooked::isc::SceneConfigs {
                    active_scene_config: 0,
                    jd_scene_config: vec![cooked::isc::WrappedJdSceneConfig::SongDatabase(
                        cooked::isc::WrappedSongDatabaseSceneConfig {
                            song_database_scene_config:
                                cooked::isc::SongDatabaseSceneConfig::default(),
                        },
                    )],
                },
            },
            ..Default::default()
        },
    }
}
