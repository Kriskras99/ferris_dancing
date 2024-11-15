#![allow(
    clippy::case_sensitive_file_extension_comparisons,
    reason = "Extensions are always lower case in Just Dance"
)]

use dotstar_toolkit_utils::bytes::{
    primitives::{f32be, i32be, u32be},
    read::{BinaryDeserialize, ReadAtExt, ReadError},
};
use hipstr::HipStr;
use test_eq::{test_any, test_eq, test_or};
use ubiart_toolkit_shared_types::Color;

use super::Root;
use crate::{
    cooked::isc::{
        Actor, Autodance, BlockFlowComponent, CoverflowSkuSongs, CoverflowSong, Enum,
        GFXMaterialSerializable, GFXMaterialSerializableParam, GFXMaterialTexturePathSet,
        GFXPrimitiveParam, MapSceneConfig, MasterTape, Material, MaterialGraphicComponent,
        MaterialParams, MusicTrackComponent, PleoComponent, PleoTextureGraphicComponent,
        PrimitiveParameters, Scene, SceneConfigs, SongDatabase, SongDatabaseSceneConfig, SongDesc,
        SubSceneActor, TapeCase, TextureSet, WrappedActor, WrappedActors, WrappedComponent,
        WrappedJdSceneConfig, WrappedMapSceneConfig, WrappedMaterialGraphicComponent,
        WrappedPleoComponent, WrappedPleoTextureGraphicComponent, WrappedScene,
        WrappedSceneConfigs, WrappedSongDatabaseSceneConfig, WrappedSubSceneActor,
    },
    utils::{Game, InternedString, SplitPath, UniqueGameId},
};

impl<'de> BinaryDeserialize<'de> for Root<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Ok(Self {
            scene: reader.read_at_with::<Scene>(position, ctx)?,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Scene<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 1)?;
        let engine_version = reader.read_at::<u32be>(position)?;
        test_eq!(engine_version, 0x0002_6450)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_eq!(unk3, 0)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 0)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0)?;
        let actors = reader
            .read_len_type_at_with::<u32be, WrappedActors>(position, ctx)?
            .collect::<Result<Vec<_>, _>>()?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0)?;
        let scene_configs = reader.read_at_with::<SceneConfigs>(position, ctx)?;
        Ok(Self {
            engine_version,
            gridunit: 0.0,
            depth_separator: 0,
            near_separator: [
                Color {
                    color: (1.0, 0.0, 0.0, 0.0),
                },
                Color {
                    color: (0.0, 1.0, 0.0, 0.0),
                },
                Color {
                    color: (0.0, 0.0, 1.0, 0.0),
                },
                Color {
                    color: (0.0, 0.0, 0.0, 1.0),
                },
            ],
            far_separator: [
                Color {
                    color: (1.0, 0.0, 0.0, 0.0),
                },
                Color {
                    color: (0.0, 1.0, 0.0, 0.0),
                },
                Color {
                    color: (0.0, 0.0, 1.0, 0.0),
                },
                Color {
                    color: (0.0, 0.0, 0.0, 1.0),
                },
            ],
            view_family: false,
            is_popup: false,
            platform_filters: vec![],
            actors,
            scene_configs: WrappedSceneConfigs {
                wrapped: scene_configs,
            },
        })
    }
}

impl<'de> BinaryDeserialize<'de> for WrappedActors<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let name = reader.read_at::<InternedString>(position)?;
        match name {
            "Actor" => Ok(Self::Actor(WrappedActor {
                actor: Box::new(reader.read_at_with::<Actor>(position, ctx)?),
            })),
            "SubSceneActor" => Ok(Self::SubSceneActor(WrappedSubSceneActor {
                sub_scene_actor: Box::new(reader.read_at_with::<SubSceneActor>(position, ctx)?),
            })),
            _ => Err(ReadError::custom(format!("Unknown actor type: {name}"))),
        }
    }
}

impl<'de> BinaryDeserialize<'de> for Actor<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let relativez = reader.read_at::<f32be>(position)?;
        let scale = reader.read_at::<(f32be, f32be)>(position)?;
        let xflipped = reader.read_at::<u32be>(position)?;
        test_any!(xflipped, 0..=1)?;
        let userfriendly = reader.read_len_string_at::<u32be>(position)?;
        let pos2d = reader.read_at::<(f32be, f32be)>(position)?;
        let unk3 = reader.read_at::<f32be>(position)?;
        test_any!(unk3, 0.0..=6.283_185_5, "Position: {position}")?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 0)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0xFFFF_FFFF)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0)?;
        let lua = reader.read_at::<SplitPath>(position)?;
        let _unk8 = reader
            .read_len_type_at::<u32be, ActorUnknown2>(position)?
            .collect::<Result<Vec<_>, _>>()?;
        let components = reader
            .read_len_type_at_with::<u32be, WrappedComponent>(position, ctx)?
            .collect::<Result<_, _>>()?;

        Ok(Self {
            relativez,
            scale,
            x_flipped: xflipped == 1,
            userfriendly,
            marker: None,
            defaultenable: None,
            is_enabled: None,
            pos2d,
            angle: 0.0,
            instancedatafile: HipStr::default(),
            lua: HipStr::from(lua.to_string()),
            components,
            parent_bind: None,
            markers: vec![],
        })
    }
}

impl<'de> BinaryDeserialize<'de> for SubSceneActor<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let relative_z = reader.read_at::<f32be>(position)?;
        let scale = reader.read_at::<(f32be, f32be)>(position)?;
        let xflipped = reader.read_at::<u32be>(position)?;
        test_any!(xflipped, 0..=1)?;
        let userfriendly = reader.read_len_string_at::<u32be>(position)?;
        let pos2d = reader.read_at::<(f32be, f32be)>(position)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_eq!(unk3, 0)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 0)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0xFFFF_FFFF)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0)?;
        let lua = reader.read_at::<SplitPath>(position)?;
        // let _unk8 = reader.read_len_type_at::<u32be, Unknown2>(position)?.collect::<Result<Vec<_>,_>>()?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_eq!(unk8, 0)?;
        let components = reader
            .read_len_type_at_with::<u32be, WrappedComponent>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let relative_path = reader.read_at::<SplitPath>(position)?;
        let embed_scene = reader.read_at::<u32be>(position)?;
        test_eq!(embed_scene, 1)?;
        let is_single_piece = reader.read_at::<u32be>(position)?;
        test_eq!(is_single_piece, 0)?;
        let zforced = reader.read_at::<u32be>(position)?;
        test_eq!(zforced, 1)?;
        let direct_picking = reader.read_at::<u32be>(position)?;
        test_eq!(direct_picking, 1)?;
        let view_type = reader.read_at::<i32be>(position)?;
        test_any!(view_type, [2, 3])?;
        let scene = reader.read_at_with::<Scene>(position, ctx)?;

        Ok(Self {
            relativez: relative_z,
            scale,
            x_flipped: xflipped == 1,
            userfriendly,
            marker: None,
            defaultenable: None,
            is_enabled: None,
            pos2d,
            angle: 0.0,
            instancedatafile: HipStr::default(),
            lua: HipStr::from(lua.to_string()),
            relativepath: HipStr::from(relative_path.to_string()),
            embed_scene: false,
            is_single_piece: false,
            zforced: false,
            direct_picking: false,
            ignore_save: false,
            enums: vec![Enum {
                name: HipStr::borrowed("viewType"),
                selection: view_type,
            }],
            wrapped_scene: WrappedScene { wrapped: scene },
            components,
            parent_bind: None,
            markers: vec![],
        })
    }
}

struct ActorUnknown2;
impl BinaryDeserialize<'_> for ActorUnknown2 {
    type Ctx = ();
    type Output = ();

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader
            .read_len_type_at::<u32be, u32be>(position)?
            .collect::<Result<Vec<_>, _>>()?;
        test_any!(unk1.as_slice(), [&[][..], &[0, 1]])?;
        let _unk2 = reader.read_len_string_at::<u32be>(position)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_eq!(unk3, 0)?;
        let _unk5 = reader.read_len_string_at::<u32be>(position)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0)?;
        let unk8 = reader.read_at::<f32be>(position)?;
        test_any!(unk8, -0.367_015..=6.159_785)?;
        let unk9 = reader.read_at::<f32be>(position)?;
        test_any!(unk9, -1.410_983..=1.633_319)?;
        let unk10 = reader.read_at::<f32be>(position)?;
        test_any!(unk10, -1.556_146..=0.15151)?;
        let unk11 = reader.read_at::<f32be>(position)?;
        test_any!(unk11, 0.0..=0.169_997)?;
        let unk12 = reader.read_at::<f32be>(position)?;
        test_any!(unk12, 0.000_435..=1.099_898)?;
        let unk13 = reader.read_at::<f32be>(position)?;
        test_any!(unk13, 0.0..=1.298_109)?;
        let unk14 = reader.read_at::<u32be>(position)?;
        test_eq!(unk14, 1, "Position: {position}")?;
        let unk15 = reader.read_at::<u32be>(position)?;
        test_eq!(unk15, 2)?;
        let unk16 = reader.read_at::<u32be>(position)?;
        test_eq!(unk16, 0)?;
        let unk17 = reader.read_at::<u32be>(position)?;
        test_eq!(unk17, 0)?;
        let unk18 = reader.read_at::<u32be>(position)?;
        test_eq!(unk18, 0)?;

        Ok(())
    }
}

impl<'de> BinaryDeserialize<'de> for WrappedComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let name = reader.read_at::<InternedString>(position)?;
        match name {
            "JD_AutodanceComponent" => Ok(WrappedComponent::Autodance(Autodance { wrapped: () })),
            "JD_BlockFlowComponent" => {
                Ok(WrappedComponent::BlockFlowComponent(BlockFlowComponent {
                    wrapped: (),
                }))
            }
            "JD_SongDatabaseComponent" => {
                Ok(WrappedComponent::SongDatabase(SongDatabase { wrapped: () }))
            }
            "JD_SongDescComponent" => Ok(WrappedComponent::SongDesc(SongDesc { wrapped: () })),
            "MasterTape" => Ok(WrappedComponent::MasterTape(MasterTape { wrapped: () })),
            "MaterialGraphicComponent" => Ok(WrappedComponent::MaterialGraphic(
                WrappedMaterialGraphicComponent {
                    wrapped: reader.read_at_with::<MaterialGraphicComponent>(position, ctx)?,
                },
            )),
            "MusicTrackComponent" => Ok(WrappedComponent::MusicTrack(MusicTrackComponent {
                wrapped: (),
            })),
            "PleoComponent" => Ok(WrappedComponent::Pleo(WrappedPleoComponent {
                wrapped: reader.read_at_with::<PleoComponent>(position, ctx)?,
            })),
            "PleoTextureGraphicComponent" => Ok(WrappedComponent::PleoTextureGraphic(
                WrappedPleoTextureGraphicComponent {
                    wrapped: reader.read_at_with::<PleoTextureGraphicComponent>(position, ctx)?,
                },
            )),
            "TapeCase_Component" => Ok(WrappedComponent::TapeCase(TapeCase { wrapped: () })),
            _ => todo!("{name}"),
        }
    }
}

impl<'de> BinaryDeserialize<'de> for PleoComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let video = reader.read_at::<SplitPath>(position)?;
        let dash_mpd = if ctx.game <= Game::JustDance2015 {
            SplitPath::default()
        } else {
            reader.read_at::<SplitPath>(position)?
        };
        let channel_id = reader.read_len_string_at::<u32be>(position)?;
        Ok(PleoComponent {
            video: HipStr::from(video.to_string()),
            dash_mpd: HipStr::from(dash_mpd.to_string()),
            channel_id,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for MaterialGraphicComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let primitive_parameters = PrimitiveParameters {
            gfx_primitive_param: reader.read_at_with::<GFXPrimitiveParam>(position, ctx)?,
        };
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_eq!(unk3, 0)?;
        let disable_shadow = reader.read_at::<u32be>(position)?;
        test_eq!(disable_shadow, u32::MAX)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 0)?;
        let anchor = reader.read_at::<i32be>(position)?;
        test_any!(anchor, 0..=9)?;
        let unk5 = reader.read_at::<f32be>(position)?;
        test_any!(unk5, -0.12..=0.0, "Position: {position}")?;
        let unk6 = reader.read_at::<f32be>(position)?;
        test_any!(unk6, -1.0..=1.2)?;
        let material = reader.read_at_with::<GFXMaterialSerializable>(position, ctx)?;
        let sinus_amplitude = reader.read_at::<(f32be, f32be, f32be)>(position)?;
        let sinus_speed = reader.read_at::<f32be>(position)?;
        let angle_y = reader.read_at::<f32be>(position)?;
        test_eq!(angle_y, 0.0)?;
        let angle_x = reader.read_at::<f32be>(position)?;
        test_eq!(angle_x, 0.0)?;
        let old_anchor = reader.read_at::<i32be>(position)?;
        test_any!(old_anchor, 0..=9)?;

        let enums = vec![
            Enum {
                name: HipStr::borrowed("anchor"),
                selection: anchor,
            },
            Enum {
                name: HipStr::borrowed("oldAnchor"),
                selection: old_anchor,
            },
        ];

        Ok(Self {
            primitive_parameters,
            color_computer_tag_id: 0,
            render_in_target: false,
            disable_light: false,
            disable_shadow,
            atlas_index: 0,
            custom_anchor: (0.0, 0.0),
            sinus_amplitude,
            sinus_speed,
            angle_x,
            angle_y,
            enums,
            material: Material {
                gfx_material_serializable: material,
            },
        })
    }
}

impl<'de> BinaryDeserialize<'de> for MapSceneConfig<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0)?;
        let pause_level = reader.read_at::<i32be>(position)?;
        test_eq!(pause_level, 6)?;
        let r#type = reader.read_at::<i32be>(position)?;
        test_eq!(r#type, 1)?;
        let musicscore = reader.read_at::<i32be>(position)?;
        test_eq!(musicscore, 2)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0)?;

        Ok(Self {
            name: HipStr::default(),
            sound_context: HipStr::default(),
            hud: 0,
            phone_title_loc_id: None,
            phone_image: None,
            enums: vec![
                Enum {
                    name: HipStr::borrowed("Pause_Level"),
                    selection: pause_level,
                },
                Enum {
                    name: HipStr::borrowed("type"),
                    selection: r#type,
                },
                Enum {
                    name: HipStr::borrowed("musicscore"),
                    selection: musicscore,
                },
            ],
        })
    }
}

impl<'de> BinaryDeserialize<'de> for PleoTextureGraphicComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let core = reader.read_at_with::<MaterialGraphicComponent>(position, ctx)?;
        let channel_id = reader.read_len_string_at::<u32be>(position)?;

        Ok(Self {
            primitive_parameters: core.primitive_parameters,
            color_computer_tag_id: core.color_computer_tag_id,
            render_in_target: core.render_in_target,
            disable_light: core.disable_light,
            disable_shadow: core.disable_shadow,
            atlas_index: core.atlas_index,
            custom_anchor: core.custom_anchor,
            sinus_amplitude: core.sinus_amplitude,
            sinus_speed: core.sinus_speed,
            angle_x: core.angle_x,
            angle_y: core.angle_y,
            enums: core.enums,
            material: core.material,
            channel_id,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for SceneConfigs<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let jd_scene_config = reader
            .read_len_type_at_with::<u32be, WrappedJdSceneConfig>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let active_scene_config = reader.read_at::<u32be>(position)?;
        test_any!(active_scene_config, 0..=1)?; // not so sure about this one

        Ok(Self {
            active_scene_config,
            jd_scene_config,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for SongDatabaseSceneConfig<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let sku = reader.read_len_string_at::<u32be>(position)?;
        let territory = reader.read_len_string_at::<u32be>(position)?;
        let rating_ui = HipStr::from(reader.read_at::<SplitPath>(position)?.to_string());
        let coverflow_sku_songs = reader
            .read_len_type_at::<u32be, CoverflowSong>(position)?
            .map(|cs| cs.map(|cs| CoverflowSkuSongs { coverflow_song: cs }))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            name: HipStr::default(),
            sku,
            territory,
            rating_ui,
            enums: vec![],
            coverflow_sku_songs,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for CoverflowSong<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let name = reader.read_len_string_at::<u32be>(position)?;
        let cover_path = HipStr::from(reader.read_at::<SplitPath>(position)?.to_string());
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0)?;

        Ok(Self { name, cover_path })
    }
}

impl<'de> BinaryDeserialize<'de> for WrappedJdSceneConfig<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let name = reader.read_at::<InternedString>(position)?;
        match name {
            "JD_MapSceneConfig" => Ok(WrappedJdSceneConfig::Map(WrappedMapSceneConfig {
                wrapped: reader.read_at_with::<MapSceneConfig>(position, ctx)?,
            })),
            "JD_SongDatabaseSceneConfig" => Ok(WrappedJdSceneConfig::SongDatabase(
                WrappedSongDatabaseSceneConfig {
                    wrapped: reader.read_at::<SongDatabaseSceneConfig>(position)?,
                },
            )),
            "JD_TransitionSceneConfig" => todo!("JD_TransitionSceneConfig"),
            "JD_UIBannerSceneConfig" => todo!("JD_UIBannerSceneConfig"),
            _ => Err(ReadError::custom(format!("Unknown SceneConfig: {name}"))),
        }
    }
}

impl<'de> BinaryDeserialize<'de> for GFXPrimitiveParam<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let color_factor = reader.read_at::<Color>(position)?;
        let gfx_occlude_info = reader.read_at::<i32be>(position)?;
        test_eq!(gfx_occlude_info, 0x0)?;

        Ok(Self {
            color_factor,
            enums: vec![Enum {
                name: HipStr::borrowed("gfxOccludeInfo"),
                selection: gfx_occlude_info,
            }],
        })
    }
}

impl<'de> BinaryDeserialize<'de> for GFXMaterialSerializable<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let texture_set = reader.read_at::<GFXMaterialTexturePathSet>(position)?;
        let atl_channel = reader.read_at::<u32be>(position)?;
        test_eq!(atl_channel, 0)?;
        let shader_path = reader.read_at::<SplitPath>(position)?;
        test_or!(
            test_eq!(shader_path.is_empty(), true),
            test_eq!(shader_path.filename().ends_with(".msh"), true)
        )?;
        let material_params = reader.read_at::<GFXMaterialSerializableParam>(position)?;
        let stencil_test = reader.read_at::<u32be>(position)?;
        test_eq!(stencil_test, 0)?;
        let alpha_test = reader.read_at::<u32be>(position)?;
        test_eq!(alpha_test, u32::MAX)?;
        let alpha_ref = reader.read_at::<u32be>(position)?;
        test_eq!(alpha_ref, u32::MAX)?;

        Ok(Self {
            atl_channel,
            atl_path: HipStr::borrowed(""),
            shader_path: HipStr::from(shader_path.to_string()),
            stencil_test: Some(stencil_test),
            alpha_test,
            alpha_ref,
            texture_set: TextureSet {
                gfx_material_texture_path_set: texture_set,
            },
            material_params: MaterialParams {
                gfx_material_serializable_param: material_params,
            },
            outlined_mask_params: None,
        })
    }
}

impl BinaryDeserialize<'_> for GFXMaterialSerializableParam {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let reflector_factor = reader.read_at::<f32be>(position)?;
        test_eq!(reflector_factor, 0.0)?;

        Ok(Self { reflector_factor })
    }
}

impl<'de> BinaryDeserialize<'de> for GFXMaterialTexturePathSet<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let diffuse = reader.read_at::<SplitPath>(position)?;
        let back_light = reader.read_at::<SplitPath>(position)?;
        let normal = reader.read_at::<SplitPath>(position)?;
        let separate_alpha = reader.read_at::<SplitPath>(position)?;
        let diffuse_2 = reader.read_at::<SplitPath>(position)?;
        let back_light_2 = reader.read_at::<SplitPath>(position)?;
        let anim_impostor = reader.read_at::<SplitPath>(position)?;
        let diffuse_3 = reader.read_at::<SplitPath>(position)?;
        let diffuse_4 = reader.read_at::<SplitPath>(position)?;

        Ok(Self {
            diffuse: HipStr::from(diffuse.to_string()),
            back_light: HipStr::from(back_light.to_string()),
            normal: HipStr::from(normal.to_string()),
            separate_alpha: HipStr::from(separate_alpha.to_string()),
            diffuse_2: HipStr::from(diffuse_2.to_string()),
            back_light_2: HipStr::from(back_light_2.to_string()),
            anim_impostor: HipStr::from(anim_impostor.to_string()),
            diffuse_3: HipStr::from(diffuse_3.to_string()),
            diffuse_4: HipStr::from(diffuse_4.to_string()),
        })
    }
}
