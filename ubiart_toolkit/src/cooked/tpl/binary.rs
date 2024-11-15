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
use ubiart_toolkit_shared_types::{Color, LocaleId};

use crate::{
    cooked::tpl::types::{
        AaBb, Actor, AsyncPlayerDescTemplate, AutodanceComponent, AutodanceData,
        AutodanceRecordingStructure, AvatarDescription, AvatarDescription16, BlockDescriptor,
        BlockFlowTemplate, BlockReplacements, Country, DefaultColors, GFXMaterialSerializable,
        GFXMaterialSerializableParam, GFXMaterialTexturePathSet, MasterTape,
        MaterialGraphicComponent, MusicSection, MusicSignature, MusicTrackComponent,
        MusicTrackData, MusicTrackStructure, Paths, PhoneImages, PleoComponent,
        PleoTextureGraphicComponent, Record, SongDescription, SoundComponent, SoundDescriptor,
        SoundParams, TapeEntry, TapeGroup, Template,
    },
    shared_json_types::{
        AutoDanceFxDesc, AutodancePropData, AutodanceVideoStructure, GFXVector4, PlaybackEvent,
        PropEvent, PropPlayerConfig,
    },
    utils::{path::ExpectedPadding, InternedString, SplitPath, UniqueGameId},
};

impl<'de> BinaryDeserialize<'de> for Actor<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 1)?;
        // looks like a size? But is larger than the amount of bytes in the file
        // changes based on the content
        let _unk2 = reader.read_at::<u32be>(position)?;
        let class = reader.read_at::<InternedString>(position)?;
        test_eq!(class, "Actor_Template")?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_eq!(unk3, 0x6C)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 0)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0)?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_eq!(unk8, 0)?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_eq!(unk9, 0)?;
        let unk10 = reader.read_at::<u32be>(position)?;
        test_eq!(unk10, 0)?;
        let components = reader
            .read_len_type_at_with::<u32be, Template>(position, ctx)?
            .collect::<Result<_, _>>()?;

        Ok(Self {
            class: Self::CLASS,
            wip: 0,
            lowupdate: 0,
            update_layer: 0,
            procedural: 0,
            startpaused: 0,
            forceisenvironment: 0,
            components,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Template<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let class = reader.read_at::<InternedString>(position)?;
        match class {
            "JD_AsyncPlayerDesc_Template" => Ok(Template::AsyncPlayerDescTemplate(
                reader.read_at::<AsyncPlayerDescTemplate>(position)?,
            )),
            "JD_AutodanceComponent_Template" => Ok(Template::AutodanceComponent(
                reader.read_at::<AutodanceComponent>(position)?,
            )),
            "JD_AvatarDescTemplate" => Ok(Template::AvatarDescription(AvatarDescription::V16(
                reader.read_at::<AvatarDescription16>(position)?,
            ))),
            "JD_BlockFlowTemplate" => Ok(Template::BlockFlowTemplate(
                reader.read_at::<BlockFlowTemplate>(position)?,
            )),
            "JD_SongDescTemplate" => Ok(Template::SongDescription(
                reader.read_at_with::<SongDescription>(position, ctx)?,
            )),
            "MaterialGraphicComponent_Template" => Ok(Template::MaterialGraphicComponent(
                reader.read_at::<MaterialGraphicComponent>(position)?,
            )),
            "MasterTape_Template" => Ok(Template::MasterTape(
                reader.read_at::<MasterTape>(position)?,
            )),
            "MusicTrackComponent_Template" => Ok(Template::MusicTrackComponent(
                reader.read_at::<MusicTrackComponent>(position)?,
            )),
            "TapeCase_Template" => Ok(Template::TapeCase(reader.read_at::<MasterTape>(position)?)),
            "PleoComponent_Template" => Ok(Template::PleoComponent(
                reader.read_at_with::<PleoComponent>(position, ctx)?,
            )),
            "PleoTextureGraphicComponent_Template" => Ok(Template::PleoTextureGraphicComponent(
                reader.read_at::<PleoTextureGraphicComponent>(position)?,
            )),
            "SoundComponent_Template" => Ok(Template::SoundComponent(
                reader.read_at::<SoundComponent>(position)?,
            )),
            _ => todo!("{class}"),
        }
    }
}

impl BinaryDeserialize<'_> for AaBb<'static> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x10)?;
        let min = reader.read_at::<(f32be, f32be)>(position)?;
        let max = reader.read_at::<(f32be, f32be)>(position)?;

        Ok(Self {
            class: None,
            min,
            max,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for AsyncPlayerDescTemplate<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x08A8)?;
        let player_name = reader.read_len_string_at::<u32be>(position)?;
        let player_country = reader.read_at::<Country>(position)?;
        let player_age_bracket = reader.read_at::<u32be>(position)?;
        test_eq!(player_age_bracket, 0x3)?;
        let player_gender = reader.read_at::<u32be>(position)?;
        test_any!(player_gender, [0x0, 0x1])?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0x0)?;
        let avatar_id = reader.read_at::<u32be>(position)?;
        let thumbnails_path_len = reader.read_at::<u32be>(position)?;
        let mut thumbnails_path = Vec::with_capacity(usize::try_from(thumbnails_path_len)?);
        for _ in 0..thumbnails_path_len {
            thumbnails_path.push(HipStr::from(
                reader.read_at::<SplitPath>(position)?.to_string(),
            ));
        }

        Ok(Self {
            class: None,
            player_name,
            player_country,
            player_age_bracket,
            player_gender,
            avatar_id,
            thumbnails_path,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for AutodanceComponent<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x0714)?;
        let song = reader.read_len_string_at::<u32be>(position)?;
        let autodance_data = reader.read_at::<AutodanceData>(position)?;

        Ok(Self {
            class: None,
            song,
            autodance_data,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for AutodanceData<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x06F8)?;
        let recording_structure = reader.read_at::<AutodanceRecordingStructure>(position)?;
        let video_structure = reader.read_at::<AutodanceVideoStructure>(position)?;
        let autodance_sound_path = HipStr::from(reader.read_at::<SplitPath>(position)?.to_string());

        Ok(Self {
            class: None,
            recording_structure,
            video_structure,
            autodance_sound_path,
        })
    }
}

impl BinaryDeserialize<'_> for AutoDanceFxDesc<'static> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x02E4)?;
        let opacity = reader.read_at::<f32be>(position)?;
        let color_low = reader.read_at::<GFXVector4>(position)?;
        let color_mid = reader.read_at::<GFXVector4>(position)?;
        let color_high = reader.read_at::<GFXVector4>(position)?;
        let low_to_mid = reader.read_at::<f32be>(position)?;
        let low_to_mid_width = reader.read_at::<f32be>(position)?;
        let mid_to_high = reader.read_at::<f32be>(position)?;
        let mid_to_high_width = reader.read_at::<f32be>(position)?;
        let sob_color = reader.read_at::<GFXVector4>(position)?;
        let out_color = reader.read_at::<GFXVector4>(position)?;
        let thick_middle = reader.read_at::<f32be>(position)?;
        let thick_inner = reader.read_at::<f32be>(position)?;
        let thick_smooth = reader.read_at::<f32be>(position)?;
        let shv_nb_frames = reader.read_at::<u32be>(position)?;
        test_eq!(shv_nb_frames, 0)?;
        let parts_scale = reader
            .read_len_type_at::<u32be, u32be>(position)?
            .collect::<Result<Vec<_>, _>>()?;
        test_eq!(parts_scale.len(), 5)?;
        let halftone_factor = reader.read_at::<u32be>(position)?;
        test_eq!(halftone_factor, 0)?;
        let halftone_cutout_levels = reader.read_at::<f32be>(position)?;
        let uv_blackout_factor = reader.read_at::<u32be>(position)?;
        test_eq!(uv_blackout_factor, 0)?;
        let uv_blackout_desaturation = reader.read_at::<f32be>(position)?;
        let uv_blackout_contrast = reader.read_at::<f32be>(position)?;
        let uv_blackout_brightness = reader.read_at::<u32be>(position)?;
        test_eq!(uv_blackout_brightness, 0)?;
        let uv_blackout_color = reader.read_at::<GFXVector4>(position)?;
        let toon_factor = reader.read_at::<f32be>(position)?;
        test_any!(toon_factor, [0.0, 1.0], "Position: {position}")?;
        let toon_cutout_levels = reader.read_at::<f32be>(position)?;
        let unk2 = reader
            .read_len_type_at::<u32be, u32be>(position)?
            .collect::<Result<Vec<_>, _>>()?;
        test_eq!(&unk2, &[0, 0, 0, 0, 0, 0])?;
        let refraction_factor = reader.read_at::<u32be>(position)?;
        test_eq!(refraction_factor, 0)?;
        let refraction_tint = reader.read_at::<GFXVector4>(position)?;
        let refraction_scale = reader.read_at::<GFXVector4>(position)?;
        let refraction_opacity = reader.read_at::<f32be>(position)?;
        test_eq!(refraction_opacity, 0.2)?;
        let colored_shiva_thresholds = reader.read_at::<GFXVector4>(position)?;
        let colored_shiva_color_0 = reader.read_at::<GFXVector4>(position)?;
        let colored_shiva_color_1 = reader.read_at::<GFXVector4>(position)?;
        let colored_shiva_color_2 = reader.read_at::<GFXVector4>(position)?;
        let saturation_modifier = reader.read_at::<f32be>(position)?;
        test_eq!(saturation_modifier, 0.0)?;
        let slime_factor = reader.read_at::<f32be>(position)?;
        test_eq!(slime_factor, 0.0)?;
        let slime_color = reader.read_at::<GFXVector4>(position)?;
        let slime_opacity = reader.read_at::<f32be>(position)?;
        test_eq!(slime_opacity, 0.2)?;
        let slime_ambient = reader.read_at::<f32be>(position)?;
        test_eq!(slime_ambient, 0.2)?;
        let slime_normal_tiling = reader.read_at::<f32be>(position)?;
        test_eq!(slime_normal_tiling, 7.0)?;
        let slime_light_angle = reader.read_at::<f32be>(position)?;
        test_eq!(slime_light_angle, 0.0)?;
        let slime_refraction = reader.read_at::<f32be>(position)?;
        let slime_refraction_index = reader.read_at::<f32be>(position)?;
        let slime_specular = reader.read_at::<f32be>(position)?;
        let slime_specular_power = reader.read_at::<f32be>(position)?;
        let overlay_blend_factor = reader.read_at::<f32be>(position)?;
        let overlay_blend_color = reader.read_at::<GFXVector4>(position)?;
        let background_sobel_factor = reader.read_at::<f32be>(position)?;
        let background_sobel_color = reader.read_at::<GFXVector4>(position)?;
        let player_glow_factor = reader.read_at::<f32be>(position)?;
        test_eq!(player_glow_factor, 0.0)?;
        let player_glow_color = reader.read_at::<GFXVector4>(position)?;
        let swap_head_with_player = reader
            .read_len_type_at::<u32be, u32be>(position)?
            .collect::<Result<_, _>>()?;
        test_eq!(&swap_head_with_player, &[0, 1, 2, 3, 4, 5])?;
        let animate_player_head = reader
            .read_len_type_at::<u32be, u32be>(position)?
            .collect::<Result<_, _>>()?;
        test_eq!(&animate_player_head, &[0, 0, 0, 0, 0, 0])?;
        let animated_head_total_time = reader.read_at::<f32be>(position)?;
        test_eq!(animated_head_total_time, 20.0)?;
        let animated_head_rest_time = reader.read_at::<f32be>(position)?;
        test_eq!(animated_head_rest_time, 16.0)?;
        let animated_head_frame_time = reader.read_at::<f32be>(position)?;
        test_eq!(animated_head_frame_time, 0.6)?;
        let animated_head_max_distance = reader.read_at::<f32be>(position)?;
        test_eq!(animated_head_max_distance, 1.25)?;
        let animated_head_max_angle = reader.read_at::<f32be>(position)?;
        test_eq!(animated_head_max_angle, 1.2)?;
        let screen_blend_inverse_alpha_factor = reader.read_at::<u32be>(position)?;
        test_eq!(screen_blend_inverse_alpha_factor, 0)?;
        let screen_blend_inverse_alpha_scale_x = reader.read_at::<f32be>(position)?;
        test_any!(screen_blend_inverse_alpha_scale_x, [0.0, 1.0])?;
        let screen_blend_inverse_alpha_scale_y = reader.read_at::<f32be>(position)?;
        test_any!(screen_blend_inverse_alpha_scale_y, [0.0, 1.0])?;
        let screen_blend_inverse_alpha_trans_x = reader.read_at::<u32be>(position)?;
        test_eq!(screen_blend_inverse_alpha_trans_x, 0)?;
        let screen_blend_inverse_alpha_trans_y = reader.read_at::<u32be>(position)?;
        test_eq!(screen_blend_inverse_alpha_trans_y, 0)?;
        let tint_mul_color_factor = reader.read_at::<u32be>(position)?;
        test_eq!(tint_mul_color_factor, 0)?;
        let tint_mul_color = reader.read_at::<GFXVector4>(position)?;
        let floor_plane_factor = reader.read_at::<f32be>(position)?;
        test_any!(floor_plane_factor, [0.0, 1.0])?;
        let floor_plane_tiles = reader.read_at::<GFXVector4>(position)?;
        let floor_speed_x = reader.read_at::<f32be>(position)?;
        test_any!(floor_speed_x, [0.0, 0.02])?;
        let floor_speed_y = reader.read_at::<f32be>(position)?;
        let floor_wave_speed = reader.read_at::<f32be>(position)?;
        let floor_blend_mode = reader.read_at::<u32be>(position)?;
        test_eq!(floor_blend_mode, 0)?;
        let floor_plane_image_id = reader.read_at::<u32be>(position)?;
        test_eq!(floor_plane_image_id, 0)?;
        let start_radius = reader.read_at::<f32be>(position)?;
        let end_radius = reader.read_at::<f32be>(position)?;
        let radius_variance = reader.read_at::<f32be>(position)?;
        let radius_noise_rate = reader.read_at::<u32be>(position)?;
        test_eq!(radius_noise_rate, 0)?;
        let radius_noise_amp = reader.read_at::<f32be>(position)?;
        test_eq!(radius_noise_amp, 0.0)?;
        let min_spin = reader.read_at::<f32be>(position)?;
        let max_spin = reader.read_at::<f32be>(position)?;
        let dir_angle = reader.read_at::<f32be>(position)?;
        test_eq!(dir_angle, 0.0)?;
        let min_wander_rate = reader.read_at::<f32be>(position)?;
        let max_wander_rate = reader.read_at::<f32be>(position)?;
        let min_wander_amp = reader.read_at::<f32be>(position)?;
        let max_wander_amp = reader.read_at::<f32be>(position)?;
        let min_speed = reader.read_at::<f32be>(position)?;
        let max_speed = reader.read_at::<f32be>(position)?;
        let motion_power = reader.read_at::<f32be>(position)?;
        let amount = reader.read_at::<f32be>(position)?;
        let image_id = reader.read_at::<u32be>(position)?;
        let start_r = reader.read_at::<f32be>(position)?;
        let start_g = reader.read_at::<f32be>(position)?;
        let start_b = reader.read_at::<f32be>(position)?;
        let end_r = reader.read_at::<f32be>(position)?;
        let end_g = reader.read_at::<f32be>(position)?;
        let end_b = reader.read_at::<f32be>(position)?;
        let start_alpha = reader.read_at::<f32be>(position)?;
        test_eq!(start_alpha, 1.0)?;
        let end_alpha = reader.read_at::<f32be>(position)?;
        let textured_outline_factor = reader.read_at::<u32be>(position)?;
        test_eq!(textured_outline_factor, 0)?;
        let textured_outline_tiling = reader.read_at::<f32be>(position)?;
        let triple_layer_background_factor = reader.read_at::<u32be>(position)?;
        test_eq!(triple_layer_background_factor, 0)?;
        let triple_layer_background_tint_color = reader.read_at::<GFXVector4>(position)?;
        let triple_layer_background_speed_x = reader.read_at::<u32be>(position)?;
        test_eq!(triple_layer_background_speed_x, 0)?;
        let triple_layer_background_speed_y = reader.read_at::<u32be>(position)?;
        test_eq!(triple_layer_background_speed_y, 0)?;
        let trail_effect_id = reader.read_at::<u32be>(position)?;
        test_eq!(trail_effect_id, 0)?;

        Ok(Self {
            class: None,
            opacity,
            color_low,
            color_mid,
            color_high,
            low_to_mid,
            low_to_mid_width,
            mid_to_high,
            mid_to_high_width,
            sob_color,
            out_color,
            thick_middle,
            thick_inner,
            thick_smooth,
            shv_nb_frames,
            parts_scale,
            halftone_factor,
            halftone_cutout_levels,
            uv_blackout_factor,
            uv_blackout_desaturation,
            uv_blackout_contrast,
            uv_blackout_brightness,
            uv_blackout_color,
            toon_factor,
            toon_cutout_levels,
            refraction_factor,
            refraction_tint,
            refraction_scale,
            refraction_opacity,
            colored_shiva_thresholds,
            colored_shiva_color_0,
            colored_shiva_color_1,
            colored_shiva_color_2,
            saturation_modifier,
            slime_factor,
            slime_color,
            slime_opacity,
            slime_ambient,
            slime_normal_tiling,
            slime_light_angle,
            slime_refraction,
            slime_refraction_index,
            slime_specular,
            slime_specular_power,
            overlay_blend_factor,
            overlay_blend_color,
            background_sobel_factor,
            background_sobel_color,
            player_glow_factor,
            player_glow_color,
            swap_head_with_player,
            animate_player_head,
            animated_head_total_time,
            animated_head_rest_time,
            animated_head_frame_time,
            animated_head_max_distance,
            animated_head_max_angle,
            screen_blend_inverse_alpha_factor,
            screen_blend_inverse_alpha_scale_x,
            screen_blend_inverse_alpha_scale_y,
            screen_blend_inverse_alpha_trans_x,
            screen_blend_inverse_alpha_trans_y,
            tint_mul_color_factor,
            tint_mul_color,
            floor_plane_factor,
            floor_plane_tiles,
            floor_speed_x,
            floor_speed_y,
            floor_wave_speed,
            floor_blend_mode,
            floor_plane_image_id,
            start_radius,
            end_radius,
            radius_variance,
            radius_noise_rate,
            radius_noise_amp,
            min_spin,
            max_spin,
            dir_angle,
            min_wander_rate,
            max_wander_rate,
            min_wander_amp,
            max_wander_amp,
            min_speed,
            max_speed,
            motion_power,
            amount,
            image_id,
            start_r,
            start_g,
            start_b,
            end_r,
            end_g,
            end_b,
            start_alpha,
            end_alpha,
            textured_outline_factor,
            textured_outline_tiling,
            triple_layer_background_factor,
            triple_layer_background_tint_color,
            triple_layer_background_speed_x,
            triple_layer_background_speed_y,
            trail_effect_id,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for AutodancePropData<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x18)?;
        let index = reader.read_at::<u32be>(position)?;
        test_eq!(index, 0x1)?;
        let pivot_x = reader.read_at::<f32be>(position)?;
        test_eq!(pivot_x, 0.5)?;
        let pivot_y = reader.read_at::<f32be>(position)?;
        let size = reader.read_at::<f32be>(position)?;
        let prop_part = reader.read_at::<u32be>(position)?;
        test_eq!(prop_part, 2)?;

        Ok(Self {
            class: None,
            index,
            pivot_x,
            pivot_y,
            size,
            fx_asset_id: None,
            prop_part,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for AutodanceRecordingStructure<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x10)?;
        let records = reader
            .read_len_type_at::<u32be, Record>(position)?
            .collect::<Result<_, _>>()?;

        Ok(Self {
            class: None,
            records,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for AutodanceVideoStructure<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x06C8)?;
        let gamemode = reader.read_at::<u32be>(position)?;
        test_any!(gamemode, 1..=2)?;
        let song_start_position = reader.read_at::<f32be>(position)?;
        let duration = reader.read_at::<f32be>(position)?;
        let thumbnail_time = reader.read_at::<u32be>(position)?;
        test_eq!(thumbnail_time, 0)?;
        let fade_out_duration = reader.read_at::<f32be>(position)?;
        let animated_frame_path = HipStr::from(
            reader
                .read_at_with::<SplitPath>(position, ExpectedPadding::None)?
                .to_string(),
        );
        let unk2 = reader.read_at::<u32be>(position)?;
        test_any!(unk2, [0x0, 0x22], "Position: {position}")?;
        let ground_plane_path = HipStr::from(
            reader
                .read_at_with::<SplitPath>(position, ExpectedPadding::None)?
                .to_string(),
        );
        let unk3 = reader.read_at::<u32be>(position)?;
        test_any!(unk3, [0x0, 0x22], "Position: {position}")?;
        let first_layer_triple_background_path = HipStr::from(
            reader
                .read_at_with::<SplitPath>(position, ExpectedPadding::None)?
                .to_string(),
        );
        let unk4 = reader.read_at::<u32be>(position)?;
        test_any!(unk4, [0x0, 0x22], "Position: {position}")?;
        let second_layer_triple_background_path = HipStr::from(
            reader
                .read_at_with::<SplitPath>(position, ExpectedPadding::None)?
                .to_string(),
        );
        let unk5 = reader.read_at::<u32be>(position)?;
        test_any!(unk5, [0x0, 0x22], "Position: {position}")?;
        let third_layer_triple_background_path = HipStr::from(
            reader
                .read_at_with::<SplitPath>(position, ExpectedPadding::None)?
                .to_string(),
        );
        let unk6 = reader.read_at::<u32be>(position)?;
        test_any!(unk6, [0x0, 0x22], "Position: {position}")?;
        let playback_events = reader
            .read_len_type_at::<u32be, PlaybackEvent>(position)?
            .collect::<Result<_, _>>()?;
        let background_effect = Box::new(reader.read_at::<AutoDanceFxDesc>(position)?);
        let _unk2 = reader
            .read_len_type_at::<u32be, UnknownC>(position)?
            .collect::<Result<Vec<_>, _>>()?;
        let player_effect = Box::new(reader.read_at::<AutoDanceFxDesc>(position)?);
        let _unk3 = reader
            .read_len_type_at::<u32be, UnknownC>(position)?
            .collect::<Result<Vec<_>, _>>()?;
        let prop_events = reader
            .read_len_type_at::<u32be, PropEvent>(position)?
            .collect::<Result<_, _>>()?;
        let props = reader
            .read_len_type_at::<u32be, AutodancePropData>(position)?
            .collect::<Result<_, _>>()?;
        let props_players_config = reader
            .read_len_type_at::<u32be, PropPlayerConfig>(position)?
            .collect::<Result<_, _>>()?;

        Ok(Self {
            class: None,
            game_mode: Some(gamemode),
            song_start_position,
            duration,
            thumbnail_time,
            fade_out_duration,
            animated_frame_path: Some(animated_frame_path),
            ground_plane_path,
            first_layer_triple_background_path,
            second_layer_triple_background_path,
            third_layer_triple_background_path,
            playback_events,
            background_effect,
            background_effect_events: vec![],
            player_effect,
            player_effect_events: vec![],
            prop_events,
            props,
            props_players_config,
        })
    }
}

struct UnknownC;
impl BinaryDeserialize<'_> for UnknownC {
    type Ctx = ();
    type Output = ();

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0xC)?;
        let _unk2 = reader.read_at::<f32be>(position)?;
        let _unk3 = reader.read_at::<f32be>(position)?;

        Ok(())
    }
}

impl<'de> BinaryDeserialize<'de> for AvatarDescription16<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x54)?;
        let jd_version = reader.read_at::<u32be>(position)?;
        test_eq!(jd_version, 2015)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0)?;
        let actor_path = HipStr::from(reader.read_at::<SplitPath>(position)?.to_string());
        let unk3 = reader.read_at::<u32be>(position)?;
        test_any!(unk3, 2000..=2010)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 3)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0xFFFF_FFFF)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0xFFFF_FFFF)?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_eq!(unk8, 1)?;

        Ok(Self {
            class: None,
            jd_version,
            relative_song_name: HipStr::default(),
            relative_quest_id: HipStr::default(),
            relative_game_mode_name: HipStr::default(),
            actor_path,
            avatar_id: 0,
            phone_image: HipStr::default(),
            status: 0,
            unlock_type: 0,
            mojo_price: 0,
            wdf_level: 0,
            count_in_progression: 0,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for BlockDescriptor<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0xBC)?;
        let song_name = reader.read_len_string_at::<u32be>(position)?;
        let frst_beat = reader.read_at::<u32be>(position)?;
        let last_beat = reader.read_at::<u32be>(position)?;
        let song_switch = reader.read_at::<u32be>(position)?;
        test_any!(song_switch, 0x0..=1)?;
        let video_coach_offset = reader.read_at::<(f32be, f32be)>(position)?;
        let video_coach_scale = reader.read_at::<f32be>(position)?;
        let dance_step_name = reader.read_len_string_at::<u32be>(position)?;
        let playing_speed = reader.read_at::<f32be>(position)?;
        test_eq!(playing_speed, 1.0)?;
        let is_entry_point = reader.read_at::<u32be>(position)?;
        test_eq!(is_entry_point, 0x0)?;
        let is_empty_block = reader.read_at::<u32be>(position)?;
        test_any!(is_empty_block, 0..=1)?;
        let is_no_score_block = reader.read_at::<u32be>(position)?;
        test_eq!(is_no_score_block, 0x0)?;
        let guid = reader.read_len_string_at::<u32be>(position)?;

        Ok(Self {
            class: None,
            song_name,
            frst_beat,
            last_beat,
            song_switch,
            video_coach_offset,
            video_coach_scale,
            dance_step_name,
            playing_speed,
            is_entry_point,
            is_empty_block,
            is_no_score_block,
            guid,
            force_display_last_pictos: 0,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for BlockFlowTemplate<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x28)?;
        let is_mash_up = reader.read_at::<u32be>(position)?;
        test_any!(is_mash_up, 0..=1)?;
        let is_party_master = reader.read_at::<u32be>(position)?;
        test_any!(is_party_master, 0..=1)?;
        let block_descriptor_vector = reader
            .read_len_type_at::<u32be, BlockReplacements>(position)?
            .collect::<Result<_, _>>()?;

        Ok(Self {
            class: None,
            is_mash_up,
            is_party_master,
            block_descriptor_vector,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for BlockReplacements<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0xCC)?;
        let base_block = reader.read_at::<BlockDescriptor>(position)?;
        let alternative_blocks = reader
            .read_len_type_at::<u32be, BlockDescriptor>(position)?
            .collect::<Result<_, _>>()?;

        Ok(Self {
            class: None,
            base_block,
            alternative_blocks,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Country<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x2C)?;
        let country_id = reader.read_at::<u32be>(position)?;
        let country_code = reader.read_len_string_at::<u32be>(position)?;
        let country_name = reader.read_len_string_at::<u32be>(position)?;

        Ok(Self {
            class: None,
            country_id,
            country_code,
            country_name,
        })
    }
}

impl BinaryDeserialize<'_> for DefaultColors {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut theme = None;
        let mut lyrics = None;
        for default_color in reader.read_len_type_at::<u32be, (InternedString, Color)>(position)? {
            let (name, color) = default_color?;
            match name {
                "theme" => test_eq!(
                    theme.replace(color),
                    None,
                    "Found 'theme' twice for DefaultColors"
                )?,
                "lyrics" => test_eq!(
                    lyrics.replace(color),
                    None,
                    "Found 'lyrics' twice for DefaultColors"
                )?,
                _ => {
                    return Err(ReadError::custom(format!(
                        "Found unknown default color '{name}' for DefaultColors"
                    )))
                }
            }
        }

        let theme =
            theme.ok_or_else(|| ReadError::custom("Missing 'theme' for DefaultColors".into()))?;
        let lyrics =
            lyrics.ok_or_else(|| ReadError::custom("Missing 'lyrics' for DefaultColors".into()))?;
        Ok(Self {
            theme,
            lyrics,
            songcolor_1a: None,
            songcolor_1b: None,
            songcolor_2a: None,
            songcolor_2b: None,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for GFXMaterialSerializable<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x0000_0118)?;

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
            class: None,
            outlined_mask_params: None,
            atl_path: HipStr::borrowed(""),
            texture_set,
            atl_channel,
            shader_path: HipStr::from(shader_path.to_string()),
            material_params,
            stencil_test: Some(stencil_test),
            alpha_test,
            alpha_ref,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for GFXMaterialSerializableParam<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x0000_0018)?;

        let reflector_factor = reader.read_at::<u32be>(position)?;
        test_eq!(reflector_factor, 0)?;

        Ok(Self {
            class: None,
            reflector_factor,
        })
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
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x24)?;
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
            class: None,
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

impl BinaryDeserialize<'_> for GFXVector4<'static> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x10)?;
        let x = reader.read_at::<f32be>(position)?;
        let y = reader.read_at::<f32be>(position)?;
        let z = reader.read_at::<f32be>(position)?;
        let w = reader.read_at::<f32be>(position)?;

        Ok(Self {
            class: None,
            x,
            y,
            z,
            w,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for MasterTape<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x40)?;
        let tapes_rack = reader
            .read_len_type_at::<u32be, TapeGroup>(position)?
            .collect::<Result<_, _>>()?;

        Ok(Self {
            class: None,
            tapes_rack,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for MaterialGraphicComponent<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x0000_02C8)?;

        let core = reader.read_at::<CoreGraphicComponent>(position)?;

        Ok(Self {
            class: None,
            patch_level: core.patch_level,
            patch_h_level: core.patch_h_level,
            patch_v_level: core.patch_v_level,
            visual_aabb: core.visual_aabb,
            renderintarget: core.renderintarget,
            pos_offset: core.pos_offset,
            angle_offset: core.angle_offset,
            blendmode: core.blendmode,
            materialtype: core.materialtype,
            self_illum_color: core.self_illum_color,
            disable_light: core.disable_light,
            force_disable_light: core.force_disable_light,
            use_shadow: core.use_shadow,
            use_root_bone: core.use_root_bone,
            shadow_size: core.shadow_size,
            shadow_material: core.shadow_material,
            shadow_attenuation: core.shadow_attenuation,
            shadow_dist: core.shadow_dist,
            shadow_offset_pos: core.shadow_offset_pos,
            angle_limit: core.angle_limit,
            material: core.material,
            default_color: core.default_color,
            z_offset: core.z_offset,
        })
    }
}

impl BinaryDeserialize<'_> for MusicSection<'static> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x14)?;
        let marker = reader.read_at::<i32be>(position)?;
        let section_type = reader.read_at::<u32be>(position)?;
        test_any!(section_type, 0..=11)?;
        let comment = reader.read_at::<u32be>(position)?;
        test_eq!(comment, 0)?;

        Ok(Self {
            class: None,
            marker,
            section_type,
            comment: HipStr::default(),
        })
    }
}

impl BinaryDeserialize<'_> for MusicSignature<'static> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x8)?;
        let marker = reader.read_at::<i32be>(position)?;
        let beats = reader.read_at::<u32be>(position)?;

        Ok(Self {
            class: None,
            marker,
            beats,
            comment: None,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for MusicTrackComponent<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x80)?;
        let track_data = reader.read_at::<MusicTrackData>(position)?;

        Ok(Self {
            class: None,
            track_data,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for MusicTrackData<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x70)?;
        let structure = reader.read_at::<MusicTrackStructure>(position)?;
        let path = HipStr::from(reader.read_at::<SplitPath>(position)?.to_string());
        let unk2 = reader.read_at::<f32be>(position)?;
        test_any!(unk2, [0.0, -2.0, -3.1, -5.2, -5.3], "Position: {position}")?;

        Ok(Self {
            class: None,
            structure,
            path,
            url: HipStr::default(),
        })
    }
}

impl<'de> BinaryDeserialize<'de> for MusicTrackStructure<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x4C)?;
        let markers = reader
            .read_len_type_at::<u32be, u32be>(position)?
            .collect::<Result<_, _>>()?;
        let signatures = reader
            .read_len_type_at::<u32be, MusicSignature>(position)?
            .collect::<Result<_, _>>()?;
        let sections = reader
            .read_len_type_at::<u32be, MusicSection>(position)?
            .collect::<Result<_, _>>()?;
        let start_beat = reader.read_at::<i32be>(position)?;
        let end_beat = reader.read_at::<u32be>(position)?;
        let video_start_time = reader.read_at::<f32be>(position)?;
        let volume = reader.read_at::<f32be>(position)?;
        test_eq!(volume, 0.0)?;

        Ok(Self {
            class: None,
            markers,
            signatures,
            sections,
            start_beat,
            end_beat,
            fade_start_beat: 0,
            use_fade_start_beat: false,
            fade_end_beat: 0,
            use_fade_end_beat: false,
            video_start_time,
            preview_entry: 0.0,
            preview_loop_start: 0.0,
            preview_loop_end: 0.0,
            volume,
            fade_in_duration: 0,
            fade_in_type: 0,
            fade_out_duration: 0,
            fade_out_type: 0,
            entry_points: vec![],
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Paths<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let avatars_len = reader.read_at::<u32be>(position)?;
        let mut avatars = Vec::with_capacity(usize::try_from(avatars_len)?);
        for _ in 0..avatars_len {
            avatars.push(HipStr::from(
                reader.read_at::<SplitPath>(position)?.to_string(),
            ));
        }

        let asyncplayers_len = reader.read_at::<u32be>(position)?;
        let mut asyncplayers = Vec::with_capacity(usize::try_from(asyncplayers_len)?);
        for _ in 0..asyncplayers_len {
            asyncplayers.push(HipStr::from(
                reader.read_at::<SplitPath>(position)?.to_string(),
            ));
        }

        Ok(Self {
            asyncplayers: Some(asyncplayers),
            avatars: Some(avatars),
        })
    }
}

impl BinaryDeserialize<'_> for PlaybackEvent<'static> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x18)?;
        let clip_number = reader.read_at::<u32be>(position)?;
        let start_clip = reader.read_at::<f32be>(position)?;
        let start_time = reader.read_at::<f32be>(position)?;
        let duration = reader.read_at::<f32be>(position)?;
        let speed = reader.read_at::<f32be>(position)?;

        Ok(Self {
            class: None,
            clip_number,
            start_clip,
            start_time,
            duration,
            speed,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for PleoComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x88)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0x0)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_eq!(unk3, 0x0)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 0xFFFF_FFFF)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0x0)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0x0)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0x0)?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_any!(unk8, [0x0, 0x1])?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_eq!(unk9, 0x1)?;
        let unk10 = reader.read_at::<u32be>(position)?;
        test_eq!(unk10, 0x1)?;
        let unk11 = reader.read_at::<u32be>(position)?;
        test_eq!(unk11, 0x0)?;
        let unk12 = reader.read_at::<u32be>(position)?;
        test_eq!(unk12, 0x1)?;
        let channel_id = reader.read_len_string_at::<u32be>(position)?;
        let unk13 = reader.read_at::<u32be>(position)?;
        test_eq!(unk13, 0x0)?;
        let unk14 = reader.read_at::<u32be>(position)?;
        test_eq!(unk14, 0x1)?;
        let unk15 = reader.read_at::<u32be>(position)?;
        test_eq!(unk15, 0x0)?;
        let unk16 = reader.read_at::<u32be>(position)?;
        test_eq!(unk16, 0x0)?;
        let unk17 = reader.read_at::<u32be>(position)?;
        test_eq!(unk17, 0xFFFF_FFFF)?;
        let unk18 = reader.read_at::<u32be>(position)?;
        test_eq!(unk18, 0x0)?;
        let audio_bus = reader.read_at::<InternedString>(position)?;
        test_eq!(audio_bus, "video")?;

        Ok(Self {
            class: None,
            video: HipStr::default(),
            video_url: HipStr::default(),
            auto_play: 0,
            play_from_memory: 0,
            play_to_texture: 0,
            loop_it: 0,
            clean_loop: None,
            alpha: 0,
            sound: 0,
            channel_id,
            adaptive: 0,
            auto_stop_at_the_end: 0,
            auto_discard_after_stop: None,
            dash_mpd: HipStr::default(),
            audio_bus: HipStr::borrowed(audio_bus),
            loop_frame: None,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for PleoTextureGraphicComponent<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x0000_02DC)?;

        let core = reader.read_at::<CoreGraphicComponent>(position)?;

        let channel_id = reader.read_len_string_at::<u32be>(position)?;
        let auto_activate = reader.read_at::<u32be>(position)?;
        test_any!(auto_activate, [0, 1])?;
        let use_conductor = reader.read_at::<u32be>(position)?;
        test_eq!(use_conductor, 1)?;

        Ok(Self {
            class: None,
            patch_level: core.patch_level,
            patch_h_level: core.patch_h_level,
            patch_v_level: core.patch_v_level,
            visual_aabb: core.visual_aabb,
            renderintarget: core.renderintarget,
            pos_offset: core.pos_offset,
            angle_offset: core.angle_offset,
            blendmode: core.blendmode,
            materialtype: core.materialtype,
            self_illum_color: core.self_illum_color,
            disable_light: core.disable_light,
            force_disable_light: core.force_disable_light,
            use_shadow: core.use_shadow,
            use_root_bone: core.use_root_bone,
            shadow_size: core.shadow_size,
            shadow_material: core.shadow_material,
            shadow_attenuation: core.shadow_attenuation,
            shadow_dist: core.shadow_dist,
            shadow_offset_pos: core.shadow_offset_pos,
            angle_limit: core.angle_limit,
            material: core.material,
            default_color: core.default_color,
            z_offset: core.z_offset,
            channel_id,
            auto_activate,
            use_conductor,
        })
    }
}

impl BinaryDeserialize<'_> for PropEvent<'static> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x1C)?;
        let start_time = reader.read_at::<u32be>(position)?;
        test_eq!(start_time, 0x0)?;
        let duration = reader.read_at::<f32be>(position)?;
        let associated_props = reader
            .read_len_type_at::<u32be, u32be>(position)?
            .collect::<Result<_, _>>()?;
        test_eq!(associated_props, &[1, 1, 1, 1, 1, 1])?;

        Ok(Self {
            class: None,
            start_time,
            duration,
            associated_props,
        })
    }
}

impl BinaryDeserialize<'_> for PropPlayerConfig<'static> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x18)?;
        let index = reader.read_at::<u32be>(position)?;
        test_eq!(index, 0x1)?;
        let active_props = reader
            .read_len_type_at::<u32be, u32be>(position)?
            .collect::<Result<_, _>>()?;
        test_eq!(active_props, &[1])?;

        Ok(Self {
            class: None,
            index,
            active_props,
        })
    }
}

impl BinaryDeserialize<'_> for Record<'static> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x08)?;
        let start = reader.read_at::<f32be>(position)?;
        let duration = reader.read_at::<f32be>(position)?;
        Ok(Self {
            class: None,
            start,
            duration,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for SongDescription<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0xF4)?;
        let map_name = reader.read_len_string_at::<u32be>(position)?;
        let jd_version = reader.read_at::<u32be>(position)?;
        test_eq!(jd_version, 2015)?;
        // maybe original_jd_version?
        let original_jd_version = reader.read_at::<u32be>(position)?;
        test_any!(original_jd_version, [0x5, 2015, 0xFFFF_FFFF])?;
        let related_albums_len = reader.read_at::<u32be>(position)?;
        test_any!(related_albums_len, 0x0..=0x1)?;
        let mut related_albums = Vec::with_capacity(usize::try_from(related_albums_len)?);
        for _ in 0..related_albums_len {
            related_albums.push(reader.read_len_string_at::<u32be>(position)?);
        }
        let _unk3 = reader
            .read_len_type_at::<u32be, Unknown58>(position)?
            .collect::<Result<Vec<_>, _>>()?;

        let artist = reader.read_len_string_at::<u32be>(position)?;
        let dancer_name = reader.read_len_string_at::<u32be>(position)?;
        let title = reader.read_len_string_at::<u32be>(position)?;
        let num_coach = reader.read_at::<u32be>(position)?;
        test_any!(num_coach, 0..=4)?;
        let main_coach = reader.read_at::<i32be>(position)?;
        test_any!(main_coach, -1..=0)?;
        let difficulty = reader.read_at::<u32be>(position)?;
        test_any!(difficulty, 1..=3)?;
        let background_type = reader.read_at::<u32be>(position)?;
        test_any!(background_type, [0x0, 0x3, 0x4, 0x5])?;
        let lyrics_type = reader.read_at::<i32be>(position)?;
        test_any!(lyrics_type, -1..=2)?;
        let energy = reader.read_at::<u32be>(position)?;
        test_eq!(energy, 0x1)?;
        let unk17 = reader.read_at::<f32be>(position)?;
        test_any!(unk17, [0.0, 0.5])?;
        let tags = reader
            .read_len_type_at::<u32be, Unknown10>(position)?
            .collect::<Result<_, _>>()?;
        let default_colors = reader.read_at::<DefaultColors>(position)?;
        let paths = reader.read_at::<Paths>(position)?;

        Ok(Self {
            class: None,
            map_name,
            jd_version,
            original_jd_version: jd_version,
            related_albums,
            artist,
            cn_lyrics: None,
            dancer_name,
            title,
            credits: HipStr::default(),
            sub_title: None,
            sub_credits: None,
            phone_images: PhoneImages::default(),
            num_coach,
            main_coach,
            double_scoring_type: None,
            difficulty,
            sweat_difficulty: 1,
            background_type,
            lyrics_type,
            energy: Some(energy),
            tags,
            jdm_attributes: None,
            status: 0,
            locale_id: LocaleId::default(),
            mojo_value: 0,
            count_in_progression: 0,
            default_colors,
            video_preview_path: HipStr::default(),
            score_with_both_controllers: None,
            paths: Some(paths),
        })
    }
}

struct Unknown58;
impl BinaryDeserialize<'_> for Unknown58 {
    type Ctx = ();
    type Output = ();

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x58)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_any!(unk2, 0x0..=0x2)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_any!(unk3, [0, 1, 2, 6])?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_any!(unk4, [0x1, 0x2, 0x7])?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_or!(
            test_any!(unk5, 0x2D35..=0x2E0F),
            test_eq!(unk5, 0xFFFF_FFFF)
        )?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_any!(unk6, 0x1..=0x4)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_any!(unk7, [0x0, 0x14])?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_eq!(unk8, 0x0)?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_eq!(unk9, 0x0)?;
        let unk10 = reader.read_at::<u32be>(position)?;
        test_eq!(unk10, 0x0)?;
        let unk11 = reader.read_at::<u32be>(position)?;
        test_eq!(unk11, 0xFFFF_FFFF)?;
        let unk12 = reader.read_at::<u32be>(position)?;
        test_eq!(unk12, 0x0)?;
        let unk13 = reader.read_at::<u32be>(position)?;
        test_any!(unk13, 0x0..=0x1)?;

        Ok(())
    }
}

struct Unknown10;
impl BinaryDeserialize<'_> for Unknown10 {
    type Ctx = ();
    type Output = HipStr<'static>;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x10)?;
        let tag = reader.read_at::<InternedString>(position)?;
        let unk21 = reader.read_at::<u32be>(position)?;
        test_any!(unk21, 0..=514, "Position: {position}")?;
        let unk22 = reader.read_at::<u32be>(position)?;
        test_any!(unk22, 0..=570)?;

        Ok(HipStr::borrowed(tag))
    }
}

impl<'de> BinaryDeserialize<'de> for SoundComponent<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x118)?;
        let sound_list = reader
            .read_len_type_at::<u32be, SoundDescriptor>(position)?
            .collect::<Result<_, _>>()?;

        Ok(Self {
            class: None,
            sound_list,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for SoundDescriptor<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0xF8)?;

        // stringid of name, for example amb_ineedyourlovedlc_outro
        // doesn't work for wiiu2015/dlc7/tpl.ckd/de2de3743d26a22624854df4c9d86dc9.tpl.ckd
        let _name = reader.read_at::<u32be>(position)?;
        let volume = reader.read_at::<f32be>(position)?;
        let category = reader.read_at::<InternedString>(position)?;
        let limit_category = reader.read_at::<InternedString>(position)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_any!(unk2, 0..=1, "Position: {position}")?;
        let unk3 = reader.read_at::<i32be>(position)?;
        test_any!(unk3, -1..=10)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 0)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0)?;
        let files_len = reader.read_at::<u32be>(position)?;
        let mut files = Vec::with_capacity(usize::try_from(files_len)?);
        for _ in 0..files_len {
            let file = reader.read_at_with::<SplitPath>(position, ExpectedPadding::None)?;
            let padding = reader.read_at::<u32be>(position)?;
            test_any!(padding, [0x0, 0x2])?;
            files.push(HipStr::from(file.to_string()));
        }
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0)?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_eq!(unk8, 0)?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_eq!(unk9, 0)?;
        let unk10 = reader.read_at::<u32be>(position)?;
        test_eq!(unk10, 0)?;
        let params = reader.read_at::<SoundParams>(position)?;
        let unk11 = reader.read_at::<i32be>(position)?;
        test_any!(unk11, -1..=2)?;
        let unk12 = reader.read_at::<u32be>(position)?;
        test_eq!(unk12, 0xFFFF_FFFF)?;
        let unk13 = reader.read_at::<u32be>(position)?;
        test_any!(unk13, [0, 1])?;
        let unk14 = reader.read_at::<u32be>(position)?;
        test_any!(unk14, [0, 0x21C00])?;
        let unk15 = reader.read_at::<u32be>(position)?;
        test_eq!(unk15, 0)?;

        let pause_insensitive_flags = reader.read_at::<u32be>(position)?;
        test_eq!(pause_insensitive_flags, 0x0)?;
        let out_devices = reader.read_at::<u32be>(position)?;
        test_eq!(out_devices, 0xFFFF_FFFF)?;
        let sound_play_after_destroy = reader.read_at::<u32be>(position)?;
        test_any!(sound_play_after_destroy, 0x0..=0x1)?;

        let name = files
            .first()
            .and_then(|n| n.rsplit_once('/'))
            .unwrap_or_default()
            .1;
        let name = name
            .split_once('.')
            .unwrap_or((name, HipStr::borrowed("")))
            .0;

        Ok(Self {
            class: None,
            name,
            volume,
            category: HipStr::borrowed(category),
            limit_category: HipStr::borrowed(limit_category),
            limit_mode: 0,
            max_instances: 0,
            files,
            serial_playing_mode: 0,
            serial_stopping_mode: 0,
            params,
            pause_insensitive_flags,
            out_devices,
            sound_play_after_destroy,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for SoundParams<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x60)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_any!(unk2, 0x1..=0x2)?;
        let loop_it = reader.read_at::<u32be>(position)?;
        test_any!(loop_it, 0x0..=0x2)?;
        let play_mode = reader.read_at::<u32be>(position)?;
        test_any!(play_mode, 0x0..=0x2)?;
        let play_mode_input = reader.read_at::<u32be>(position)?;
        test_eq!(play_mode_input, 0xFFFF_FFFF)?;
        let random_vol_min = reader.read_at::<f32be>(position)?;
        let random_vol_max = reader.read_at::<f32be>(position)?;
        test_eq!(random_vol_max, 0.0)?;
        let delay = reader.read_at::<u32be>(position)?;
        test_eq!(delay, 0)?;
        let random_delay = reader.read_at::<u32be>(position)?;
        test_eq!(random_delay, 0)?;
        let random_pitch_min = reader.read_at::<f32be>(position)?;
        let random_pitch_max = reader.read_at::<f32be>(position)?;
        let fade_in_time = reader.read_at::<f32be>(position)?;
        let fade_out_time = reader.read_at::<f32be>(position)?;
        let filter_frequency = reader.read_at::<u32be>(position)?;
        test_eq!(filter_frequency, 0)?;
        let filter_type = reader.read_at::<u32be>(position)?;
        test_eq!(filter_type, 2)?;

        Ok(Self {
            class: None,
            loop_it,
            play_mode,
            play_mode_input: HipStr::default(),
            random_vol_min,
            random_vol_max,
            delay,
            random_delay,
            pitch: 1.0,
            random_pitch_min,
            random_pitch_max,
            fade_in_time,
            fade_out_time,
            filter_frequency,
            filter_type,
            transition_sample_offset: 0,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for TapeEntry<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x28)?;
        let label = HipStr::borrowed(reader.read_at::<InternedString>(position)?);
        let path = HipStr::from(reader.read_at::<SplitPath>(position)?.to_string());

        Ok(Self {
            class: None,
            label,
            path,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for TapeGroup<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x10)?;
        let entries = reader
            .read_len_type_at::<u32be, TapeEntry>(position)?
            .collect::<Result<_, _>>()?;

        Ok(Self {
            class: None,
            entries,
        })
    }
}

/// Struct for deserializing the shared parts of the various `GraphicComponent`s
struct CoreGraphicComponent<'a> {
    pub patch_level: u32,
    pub patch_h_level: u32,
    pub patch_v_level: u32,
    pub visual_aabb: AaBb<'a>,
    pub renderintarget: u32,
    pub pos_offset: (u32, u32),
    pub angle_offset: f32,
    pub blendmode: u32,
    pub materialtype: u32,
    pub self_illum_color: Color,
    pub disable_light: u32,
    pub force_disable_light: u32,
    pub use_shadow: u32,
    pub use_root_bone: u32,
    pub shadow_size: (f32, f32),
    pub shadow_material: Box<GFXMaterialSerializable<'a>>,
    pub shadow_attenuation: f32,
    pub shadow_dist: f32,
    pub shadow_offset_pos: (u32, u32, u32),
    pub angle_limit: u32,
    pub material: Box<GFXMaterialSerializable<'a>>,
    pub default_color: Color,
    pub z_offset: u32,
}

impl<'de> BinaryDeserialize<'de> for CoreGraphicComponent<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let patch_level = reader.read_at::<u32be>(position)?;
        test_eq!(patch_level, 0)?;
        let patch_h_level = reader.read_at::<u32be>(position)?;
        test_eq!(patch_h_level, 2)?;
        let patch_v_level = reader.read_at::<u32be>(position)?;
        test_eq!(patch_v_level, 2)?;
        let visual_aabb = reader.read_at::<AaBb>(position)?;
        let renderintarget = reader.read_at::<u32be>(position)?;
        test_eq!(renderintarget, 0)?;
        let pos_offset = reader.read_at::<(u32be, u32be)>(position)?;
        test_eq!(pos_offset, (0, 0))?;
        let angle_offset = reader.read_at::<f32be>(position)?;
        test_eq!(angle_offset, 0.0)?;
        let blendmode = reader.read_at::<u32be>(position)?;
        test_eq!(blendmode, 2)?;
        let materialtype = reader.read_at::<u32be>(position)?;
        test_eq!(materialtype, 0)?;

        let self_illum_color = reader.read_at::<Color>(position)?;
        test_eq!(
            self_illum_color,
            Color {
                color: (0.0, 0.0, 0.0, 0.0)
            }
        )?;
        let disable_light = reader.read_at::<u32be>(position)?;
        test_eq!(disable_light, 0)?;
        let force_disable_light = reader.read_at::<u32be>(position)?;
        test_eq!(force_disable_light, 0)?;
        let use_shadow = reader.read_at::<u32be>(position)?;
        test_eq!(use_shadow, 0)?;
        let use_root_bone = reader.read_at::<u32be>(position)?;
        test_eq!(use_root_bone, 0)?;
        let shadow_size = reader.read_at::<(f32be, f32be)>(position)?;
        let shadow_material = Box::new(reader.read_at::<GFXMaterialSerializable>(position)?);
        let shadow_attenuation = reader.read_at::<f32be>(position)?;
        test_eq!(shadow_attenuation, 1.0)?;
        let shadow_dist = reader.read_at::<f32be>(position)?;
        test_eq!(shadow_dist, 4.0)?;
        let shadow_offset_pos = reader.read_at::<(u32be, u32be, u32be)>(position)?;
        test_eq!(shadow_offset_pos, (0x0, 0x0, 0x0))?;
        let angle_limit = reader.read_at::<u32be>(position)?;
        test_eq!(angle_limit, 0x0)?;
        let material = Box::new(reader.read_at::<GFXMaterialSerializable>(position)?);
        let default_color = reader.read_at::<Color>(position)?;
        test_eq!(
            default_color,
            Color {
                color: (1.0, 1.0, 1.0, 1.0)
            }
        )?;
        let z_offset = reader.read_at::<u32be>(position)?;
        test_eq!(z_offset, 0)?;

        Ok(Self {
            patch_level,
            patch_h_level,
            patch_v_level,
            visual_aabb,
            renderintarget,
            pos_offset,
            angle_offset,
            blendmode,
            materialtype,
            self_illum_color,
            disable_light,
            force_disable_light,
            use_shadow,
            use_root_bone,
            shadow_size,
            shadow_material,
            shadow_attenuation,
            shadow_dist,
            shadow_offset_pos,
            angle_limit,
            material,
            default_color,
            z_offset,
        })
    }
}
