//! # Menuart building
//! Builds the menuart textures and phone images
use std::borrow::Cow;

use anyhow::{bail, Error};
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use ubiart_toolkit::{cooked, utils::SplitPath};

use super::SongExportState;
use crate::{
    build::BuildFiles,
    types::song::{MenuArt, MenuArtTexture},
    utils::encode_texture,
};

/// Builds the menuart textures and phone images
pub fn build(
    ses: &SongExportState<'_>,
    bf: &mut BuildFiles,
) -> Result<cooked::isc::WrappedScene<'static>, Error> {
    let map_path = &ses.map_path;
    let cache_map_path = &ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let menuart_cache_dir = cache_map_path.join("menuart");
    let textures_cache_dir = menuart_cache_dir.join("textures");
    let actors_cache_dir = menuart_cache_dir.join("actors");
    let textures_dir = map_path.join("menuart/textures");

    let menuart_file = ses
        .native_vfs
        .open(&ses.dirs.menuart().join("menuart.json"))?;
    let menuart: Vec<MenuArt> = serde_json::from_slice(&menuart_file)?;

    let mut scene_actors = Vec::new();

    for menuart in menuart {
        match menuart {
            MenuArt::Texture(texture) => {
                let lower_name = texture.name.to_lowercase();

                let mgc_actor_vec = materialgraphiccomponent_actor(
                    ses,
                    &format!("{lower_map_name}_{lower_name}.tga"),
                )?;

                let scene_actor = materialgraphiccomponent_scene(ses, &texture);
                scene_actors.push(scene_actor);

                let from = ses.dirs.menuart().join(texture.filename.as_ref());
                let encoded = encode_texture(ses.native_vfs, &from)?;
                let to = textures_cache_dir.join(format!("{lower_map_name}_{lower_name}.tga.ckd"));

                let encoded_vec = cooked::png::create_vec(encoded)?;

                bf.generated_files.add_file(
                    actors_cache_dir.join(format!("{lower_map_name}_{lower_name}.act.ckd")),
                    mgc_actor_vec,
                )?;
                bf.generated_files.add_file(to, encoded_vec)?;
            }
            MenuArt::Phone(phone) => {
                let from = ses.dirs.menuart().join(phone.filename.as_ref());
                let to = match phone.name.as_ref() {
                    "cover" | "Cover" => {
                        textures_dir.join(format!("{lower_map_name}_cover_phone.jpg"))
                    }
                    "cover_kids" => {
                        textures_dir.join(format!("{lower_map_name}_cover_kids_phone.jpg"))
                    }
                    "coach1" => textures_dir.join(format!("{lower_map_name}_coach_1_phone.png")),
                    "coach2" => textures_dir.join(format!("{lower_map_name}_coach_2_phone.png")),
                    "coach3" => textures_dir.join(format!("{lower_map_name}_coach_3_phone.png")),
                    "coach4" => textures_dir.join(format!("{lower_map_name}_coach_4_phone.png")),
                    _ => bail!("Unknown phone image name: {phone:?}"),
                };
                bf.static_files.add_file(from, to)?;
            }
        }
    }

    let menuart_scene = menuart_scene(ses, scene_actors);
    let menuart_scene_vec = cooked::isc::create_vec_with_capacity_hint(&menuart_scene, 20_000)?;

    bf.generated_files.add_file(
        menuart_cache_dir.join(format!("{lower_map_name}_menuart.isc.ckd")),
        menuart_scene_vec,
    )?;

    Ok(cooked::isc::WrappedScene {
        scene: menuart_scene.scene,
    })
}

/// Build the menuart scene
fn menuart_scene<'a>(
    ses: &SongExportState<'_>,
    actors: Vec<cooked::isc::WrappedActors<'a>>,
) -> cooked::isc::Root<'a> {
    cooked::isc::Root {
        scene: cooked::isc::Scene {
            engine_version: ses.engine_version,
            gridunit: 0.5,
            depth_separator: 0,
            near_separator: [
                (1.0, 0.0, 0.0, 0.0),
                (0.0, 1.0, 0.0, 0.0),
                (0.0, 0.0, 1.0, 0.0),
                (0.0, 0.0, 0.0, 1.0),
            ],
            far_separator: [
                (1.0, 0.0, 0.0, 0.0),
                (0.0, 1.0, 0.0, 0.0),
                (0.0, 0.0, 1.0, 0.0),
                (0.0, 0.0, 0.0, 1.0),
            ],
            view_family: true,
            is_popup: false,
            platform_filters: Vec::new(),
            actors,
            scene_configs: cooked::isc::SceneConfigs::default().into(),
        },
    }
}

/// Build the MaterialGraphicComponent actor
fn materialgraphiccomponent_actor(ses: &SongExportState<'_>, tga: &str) -> Result<Vec<u8>, Error> {
    let lower_map_name = ses.lower_map_name;
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
                files: [
                    SplitPath::new(
                        Cow::Owned(format!("world/maps/{lower_map_name}/menuart/textures/")),
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
                unk14: if tga.ends_with("_albumcoach.tga") || tga.contains("_coach_") {
                    6
                } else {
                    1
                },
                ..Default::default()
            },
        )],
    };

    Ok(cooked::act::create_vec(actor)?)
}

/// Build the MaterialGraphicComponent scene
fn materialgraphiccomponent_scene(
    ses: &SongExportState<'_>,
    texture: &MenuArtTexture<'_>,
) -> cooked::isc::WrappedActors<'static> {
    let map_name = ses.song.map_name.as_ref();
    let lower_map_name = ses.lower_map_name;
    let lower_name = texture.name.to_lowercase();
    let name = texture.name.as_ref();

    cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor {
        actor: Box::new(cooked::isc::Actor {
            scale: texture.scale,
            userfriendly: Cow::Owned(format!("{map_name}_{name}")),
            pos2d: texture.pos2d,
            lua: Cow::Borrowed("enginedata/actortemplates/tpl_materialgraphiccomponent2d.tpl"),
            components: vec![
                cooked::isc::WrappedComponent::MaterialGraphic(cooked::isc::MaterialGraphicComponent {
                    color_computer_tag_id: 0,
                    render_in_target: false,
                    disable_light: false,
                    disable_shadow: texture.disable_shadow,
                    atlas_index: 0,
                    custom_anchor: (0.0, 0.0),
                    sinus_amplitude: (0.0, 0.0, 0.0),
                    sinus_speed: 1.0,
                    angle_x: 0.0,
                    angle_y: 0.0,
                    primitive_parameters: cooked::isc::PrimitiveParameters {
                        gfx_primitive_param: cooked::isc::GFXPrimitiveParam { color_factor: (0.0, 0.0, 0.0, 0.0), enums: vec![cooked::isc::Enum { name: Cow::Borrowed("gfxOccludeInfo"), selection: 0 }] } 
                    },
                    enums: vec![cooked::isc::Enum { name: Cow::Borrowed("anchor"), selection: texture.anchor }, cooked::isc::Enum { name: Cow::Borrowed("anchor"), selection: texture.anchor }],
                    material: cooked::isc::Material { gfx_material_serializable: cooked::isc::GFXMaterialSerializable {
                        atl_channel: 0,
                        atl_path: Cow::Borrowed(""),
                        shader_path: Cow::Borrowed("world/_common/matshader/multitexture_1layer.msh"),
                        stencil_test: None,
                        alpha_test: 4_294_967_295,
                        alpha_ref: 4_294_967_295,
                        texture_set: cooked::isc::TextureSet { gfx_material_texture_path_set: cooked::isc::GFXMaterialTexturePathSet {
                            diffuse: Cow::Owned(format!("world/maps/{lower_map_name}/menuart/textures/{lower_map_name}_{lower_name}.tga")),
                            ..Default::default()
                        }},
                        material_params: cooked::isc::MaterialParams::default(),
                        outlined_mask_params: Some(cooked::isc::OutlinedMaskParams::default())
                    }}
                }.into())
            ],
            ..Default::default()
        })
    })
}
