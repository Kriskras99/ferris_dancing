//! Contains the parser implementation

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{f32be, i32be, u32be, u64be},
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    test_any, test_eq, test_ge, test_le,
};
use ubiart_toolkit_shared_types::Color;

use super::{
    AaBb, Actor, BeatPulseComponent, BoxInterpolatorComponent, CameraGraphicComponent, Carousel,
    CarouselAnimItemsDesc, CarouselBehaviour, CarouselBehaviourGoToElement,
    CarouselBehaviourNavigation, ClearColorComponent, Component, ConvertedTmlTapeComponent,
    CreditsComponent, FXControllerComponent, FixedCameraComponent, GFXMaterialSerializable,
    GFXMaterialSerializableParam, GFXMaterialTexturePathSet, GFXPrimitiveParam,
    MaterialGraphicComponent, PictoTimeline, PleoComponent, RegistrationComponent,
    SingleInstanceMesh3DComponent, SoundComponent, StopCondition, TextureGraphicComponent,
    UICarousel, UICarouselV16, UICarouselV1718, UICarouselV1922, UITextBox, UIWidgetElementDesc,
    UIWidgetGroupHUD, UIWidgetGroupHUDAutodanceRecorder, UIWidgetGroupHUDLyrics,
    UIWidgetGroupHUDPauseIcon, Unknown77F7D66C, ViewportUIComponent,
};
use crate::utils::{Game, SplitPath, UniqueGameId};

impl<'de> BinaryDeserialize<'de> for Actor<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk0 = reader.read_at::<u32be>(position)?;
        test_eq!(unk0, 1u32)?;
        let unk1 = reader.read_at::<f32be>(position)?;
        let unk2 = reader.read_at::<f32be>(position)?;
        let unk2_5 = reader.read_at::<f32be>(position)?;
        let unk3 = reader.read_at::<u64be>(position)?;
        test_eq!(unk3, 0u64)?;
        let unk3_5 = reader.read_at::<u32be>(position)?;
        test_any!(unk3_5, [0, 0xFFFF_FFFF])?;

        if ugi >= UniqueGameId::NX2019V1 {
            let unk4 = reader.read_at::<u64be>(position)?;
            test_eq!(unk4, 0x1_0000_0000u64)?;
        } else {
            let unk4 = reader.read_at::<u32be>(position)?;
            test_any!(unk4, [0x1u32, 0x0])?;
            if unk4 == 0x1 {
                let unk4_5 = reader.read_at::<u32be>(position)?;
                test_eq!(unk4_5, 0u32)?;
            }
        }

        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0u32)?;
        let unk6 = reader.read_at::<u64be>(position)?;
        test_eq!(unk6, 0u64)?;

        if ugi >= UniqueGameId::WIIU2016 {
            let unk6_5 = reader.read_at::<u32be>(position)?;
            test_eq!(unk6_5, 0x0)?;
        }

        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0xFFFF_FFFF)?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_eq!(unk8, 0u32)?;

        let lua = reader.read_at::<SplitPath>(position)?;
        test_eq!(lua.is_empty(), false)?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_eq!(unk9, 0u32)?;

        let components = reader
            .read_len_type_at_with::<u32be, Component>(position, ugi)?
            .collect::<Result<_, _>>()?;

        if let Ok(len) = reader.len() {
            if len != *position {
                reader.read_at_with::<UnknownFooter>(position, ugi)?;
            }
            test_eq!(len, *position)?;
        }

        Ok(Actor {
            lua,
            unk1,
            unk2,
            unk2_5,
            unk3_5,
            components,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Component<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        // String id of the class name of the template without the '_Template' but including 'JD_' if it is in the class name
        let component_type: u32 = reader.read_at::<u32be>(position)?;

        let component = match component_type {
            // JD_AutoDanceComponent
            0x67B8_BB77 => Component::AutodanceComponent,
            // JD_BeatPulseComponent
            0x7184_37A8 => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::BeatPulseComponent(
                    reader.read_at_with::<BeatPulseComponent>(position, ugi)?,
                )
            }
            // BoxInterpolatorComponent
            0xF513_60DA => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::BoxInterpolatorComponent(
                    reader.read_at_with::<BoxInterpolatorComponent>(position, ugi)?,
                )
            }
            // CameraGraphicComponent
            0xC760_4FA1 => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::CameraGraphicComponent(Box::new(
                    reader.read_at_with::<CameraGraphicComponent>(position, ugi)?,
                ))
            }
            // ClearColorComponent
            0xAEBB_218B => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::ClearColorComponent(
                    reader.read_at_with::<ClearColorComponent>(position, ugi)?,
                )
            }
            // ConvertedTmlTape_Component
            0xCD07_BB76 => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::ConvertedTmlTapeComponent(
                    reader.read_at_with::<ConvertedTmlTapeComponent>(position, ugi)?,
                )
            }
            // JD_CreditsComponent
            0x342E_A4FC => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::CreditsComponent(reader.read_at_with::<CreditsComponent>(position, ugi)?)
            }
            // JD_FixedCameraComponent
            0x3D5D_EBA2 => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::FixedCameraComponent(
                    reader.read_at_with::<FixedCameraComponent>(position, ugi)?,
                )
            }
            // FXControllerComponent
            0x8D4F_FFB6 => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::FXControllerComponent(
                    reader.read_at_with::<FXControllerComponent>(position, ugi)?,
                )
            }
            // MasterTape
            0x677B_269B => Component::MasterTape,
            // MaterialGraphicComponent
            0x72B6_1FC5 => Component::MaterialGraphicComponent(
                reader.read_at_with::<MaterialGraphicComponent>(position, (ugi, false))?,
            ),
            // JD_Carousel
            0x27E4_80C0 => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::Carousel(reader.read_at_with::<Carousel>(position, ugi)?)
            }
            // JD_PictoComponent
            0xC316_BF34 => Component::PictoComponent,
            // JD_PictoTimeline
            0xFA24_128D => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::PictoTimeline(reader.read_at_with::<PictoTimeline>(position, ugi)?)
            }
            // PleoComponent
            0x1263_DAD9 => Component::PleoComponent(reader.read_at::<PleoComponent>(position)?),
            // PleoTextureGraphicComponent
            0x0579_E81B => Component::PleoTextureGraphicComponent(
                reader.read_at_with::<MaterialGraphicComponent>(position, (ugi, true))?,
            ),
            // PropertyPatcher
            0xF719_B524 => {
                parse_property_patcher(reader, position, ugi)?;
                Component::PropertyPatcher
            }
            // JD_RegistrationComponent
            0xE0A2_4B6D => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::RegistrationComponent(reader.read_at::<RegistrationComponent>(position)?)
            }
            // SingleInstanceMesh3DComponent
            0x53E3_2AF7 => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::SingleInstanceMesh3DComponent(Box::new(
                    reader.read_at_with::<SingleInstanceMesh3DComponent>(position, ugi)?,
                ))
            }
            // JD_SongDatabaseComponent
            0x4055_79FB => Component::SongDatabaseComponent,
            // JD_SongDescComponent
            0xE07F_CC3F => Component::SongDescComponent,
            // SoundComponent
            0x7DD8_643C => {
                *position -= 4; // the deserialize implementation also checks the magic
                reader.read_at::<SoundComponent>(position)?;
                Component::SoundComponent
            }
            // TapeCase_Component
            0x231F_27DE => Component::TapeCaseComponent,
            // TextureGraphicComponent
            0x7B48_A9AE => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::TextureGraphicComponent(
                    reader.read_at_with::<TextureGraphicComponent>(position, ugi)?,
                )
            }
            // UICarousel
            0x8782_FE60 => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::UICarousel(reader.read_at_with::<UICarousel>(position, ugi)?)
            }
            // UITextBox
            0xD10C_BEED => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::UITextBox(reader.read_at_with::<UITextBox>(position, ugi)?)
            }
            // JD_UIWidgetGroupHUD
            0x1528_D94A => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::UIWidgetGroupHUD(reader.read_at_with::<UIWidgetGroupHUD>(position, ugi)?)
            }
            // JD_UIWidgetGroupHUD_AutodanceRecorder
            0x9F87_350C => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::UIWidgetGroupHUDAutodanceRecorder(
                    reader.read_at_with::<UIWidgetGroupHUDAutodanceRecorder>(position, ugi)?,
                )
            }
            // JD_UIWidgetGroupHUD_Lyrics
            0xF22C_9426 => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::UIWidgetGroupHUDLyrics(
                    reader.read_at_with::<UIWidgetGroupHUDLyrics>(position, ugi)?,
                )
            }
            // JD_UIWidgetGroupHUD_PauseIcon
            0x4866_6BB2 => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::UIWidgetGroupHUDPauseIcon(
                    reader.read_at_with::<UIWidgetGroupHUDPauseIcon>(position, ugi)?,
                )
            }
            // ViewportUIComponent
            0x6990_834C => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::ViewportUIComponent(
                    reader.read_at_with::<ViewportUIComponent>(position, ugi)?,
                )
            }
            // JD_AvatarDescComponent
            0x1759_E29D => Component::AvatarDescComponent,
            // JD_SkinDescComponent
            0x84EA_AE82 => Component::SkinDescComponent,
            // FxBankComponent
            0x966B_519D => {
                parse_fx_bank_component(reader, position)?;
                Component::FxBankComponent
            }
            // BezierTreeComponent
            0x3236_CF4C => {
                parse_bezier_tree_component(reader, position)?;
                Component::BezierTreeComponent
            }
            // AFXPostProcessComponent
            0x2B34_9E69 => {
                parse_afx_post_process_component(reader, position)?;
                Component::AFXPostProcessComponent
            }
            // JD_BlockFlowComponent
            0x8DA9_E375 => Component::BlockFlowComponent,
            // JD_GoldMoveComponent
            0x5632_1EA5 => Component::GoldMoveComponent,
            // JD_CameraFeedComponent
            0x499C_BAA4 => todo!("CameraFeedComponent"),
            // Something that looks like a graphic component
            0xA976_34C7 => todo!("Something that looks like a graphic component"),
            0x77F7_D66C => {
                *position -= 4; // the deserialize implementation also checks the magic
                Component::Unknown77F7D66C(reader.read_at_with::<Unknown77F7D66C>(position, ugi)?)
            }
            _ => {
                return Err(ReadError::custom(format!(
                    "Unknown component type: 0x{component_type:x}!"
                )));
            }
        };

        Ok(component)
    }
}

impl BinaryDeserialize<'_> for AaBb {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let min_left = reader.read_at::<f32be>(position)?;
        let min_right = reader.read_at::<f32be>(position)?;
        let max_left = reader.read_at::<f32be>(position)?;
        let max_right = reader.read_at::<f32be>(position)?;
        Ok(Self {
            min: (min_left, min_right),
            max: (max_left, max_right),
        })
    }
}

impl<'de> BinaryDeserialize<'de> for BeatPulseComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x7184_37A8)?;
        let text = reader.read_len_string_at::<u32be>(position)?;
        let loc_id = reader.read_at::<u32be>(position)?;
        let elements = reader
            .read_len_type_at_with::<u32be, UIWidgetElementDesc>(position, ugi)?
            .collect::<Result<_, _>>()?;
        let model_name = reader.read_at::<InternedString>(position)?;
        let flag = reader.read_len_string_at::<u32be>(position)?;

        Ok(Self {
            text,
            loc_id,
            model_name,
            flag,
            elements,
        })
    }
}

impl BinaryDeserialize<'_> for BoxInterpolatorComponent {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xF513_60DA)?;
        let inner_box = reader.read_at_with::<AaBb>(position, ctx)?;
        let outer_box = reader.read_at_with::<AaBb>(position, ctx)?;

        Ok(Self {
            inner_box,
            outer_box,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for CameraGraphicComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xC760_4FA1)?;
        let primitive_parameters = reader.read_at_with::<GFXPrimitiveParam>(position, ctx)?;
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
        let anchor = reader.read_at::<u32be>(position)?;
        test_eq!(anchor, 1)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0)?;
        let material = reader.read_at_with::<GFXMaterialSerializable>(position, ctx)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0)?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_eq!(unk8, 0)?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_eq!(unk9, 0)?;
        let sinus_speed = reader.read_at::<f32be>(position)?;
        test_eq!(sinus_speed, 1.0)?;
        let unk11 = reader.read_at::<u32be>(position)?;
        test_eq!(unk11, 0)?;
        let unk12 = reader.read_at::<u32be>(position)?;
        test_eq!(unk12, 0)?;
        let old_anchor = reader.read_at::<u32be>(position)?;
        test_eq!(old_anchor, 1)?;
        Ok(Self {
            primitive_parameters,
            color_computer_tag_id: 0,
            render_in_target: 0,
            disable_light: 0,
            disable_shadow,
            atlas_index: 0,
            custom_anchor: (0.0, 0.0),
            sinus_amplitude: (0.0, 0.0, 0.0),
            sinus_speed,
            angle_x: 0.0,
            angle_y: 0.0,
            anchor,
            old_anchor,
            material,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Carousel<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x27E4_80C0)?;
        let main_anchor = reader.read_at::<u32be>(position)?;
        let validate_action = reader.read_at::<InternedString>(position)?;
        let carousel_data_id = reader.read_len_string_at::<u32be>(position)?;
        let manage_carousel_history = reader.read_at::<u32be>(position)?;
        let switch_speed = reader.read_at::<f32be>(position)?;
        let shortcuts_config_default = reader.read_len_string_at::<u32be>(position)?;
        let shortcuts_config_switch = reader.read_len_string_at::<u32be>(position)?;
        let shortcuts_config_ps4 = reader.read_len_string_at::<u32be>(position)?;
        let shortcuts_config_xb1 = reader.read_len_string_at::<u32be>(position)?;
        let shortcuts_config_pc = reader.read_len_string_at::<u32be>(position)?;
        let shortcuts_config_ggp = reader.read_len_string_at::<u32be>(position)?;
        let shortcuts_config_prospero = if ugi.game >= Game::JustDance2021 {
            Some(reader.read_len_string_at::<u32be>(position)?)
        } else {
            None
        };
        let shortcuts_config_scarlett = if ugi.game >= Game::JustDance2021 {
            Some(reader.read_len_string_at::<u32be>(position)?)
        } else {
            None
        };
        let shortcuts_from_center_instead_from_left = reader.read_at::<u32be>(position)?;
        test_eq!(shortcuts_from_center_instead_from_left, 0)?;
        let unk2 = reader.read_at::<u8>(position)?;
        test_eq!(unk2, 0)?;
        let initial_behaviour = reader.read_at::<InternedString>(position)?;
        let sound_context = reader.read_len_string_at::<u32be>(position)?;
        let behaviours = reader
            .read_len_type_at_with::<u32be, CarouselBehaviour>(position, ugi)?
            .collect::<Result<_, _>>()?;

        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0x7FFF_FFFF)?;

        let anim_items_desc = reader.read_at_with::<CarouselAnimItemsDesc>(position, ugi)?;

        let unk8 = reader.read_at::<u32be>(position)?;
        test_any!(unk8, [0x7, 0x9])?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_eq!(unk9, 0x1)?;

        Ok(Self {
            main_anchor,
            validate_action,
            carousel_data_id,
            manage_carousel_history,
            switch_speed,
            shortcuts_config_default,
            shortcuts_config_switch,
            shortcuts_config_ps4,
            shortcuts_config_xb1,
            shortcuts_config_pc,
            shortcuts_config_ggp,
            shortcuts_config_prospero,
            shortcuts_config_scarlett,
            shortcuts_from_center_instead_from_left,
            initial_behaviour,
            sound_context,
            behaviours,
            anim_items_desc,
        })
    }
}

impl BinaryDeserialize<'_> for CarouselAnimItemsDesc {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let number_of_anims_to_listen = reader.read_at::<u32be>(position)?;
        test_eq!(number_of_anims_to_listen, 0x0, "TODO!")?;
        let enable = reader.read_at::<u32be>(position)?;
        test_eq!(enable, 0x0)?;
        let show_items_at_init = reader.read_at::<u32be>(position)?;
        test_eq!(show_items_at_init, 0x0)?;
        let enable_carousel_on_anim_ends = reader.read_at::<u32be>(position)?;
        test_eq!(enable_carousel_on_anim_ends, 0x1)?;
        let check_items_visibility_on_anim_ends = reader.read_at::<u32be>(position)?;
        test_eq!(check_items_visibility_on_anim_ends, 0x1)?;

        Ok(Self {
            enable,
            show_items_at_init,
            enable_carousel_on_anim_ends,
            check_items_visibility_on_anim_ends,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for CarouselBehaviour<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let mut temp_position = *position;
        let magic = reader.read_at::<u32be>(&mut temp_position)?;
        match magic {
            0xD6F6_A73E => Ok(CarouselBehaviour::Navigation(
                reader.read_at_with::<CarouselBehaviourNavigation>(position, ctx)?,
            )),
            0xB45C_B89D => Ok(CarouselBehaviour::GoToElement(
                reader.read_at_with::<CarouselBehaviourGoToElement>(position, ctx)?,
            )),
            _ => Err(ReadError::custom(format!("Unknown magic: 0x{magic:08x}"))),
        }
    }
}

impl<'de> BinaryDeserialize<'de> for CarouselBehaviourNavigation<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xD6F6_A73E)?;
        let key = reader.read_at::<InternedString>(position)?;
        let sound_context = reader.read_len_string_at::<u32be>(position)?;
        let sound_notif_go_next = reader.read_len_string_at::<u32be>(position)?;
        let sound_notif_go_prev = reader.read_len_string_at::<u32be>(position)?;
        let stop_conditions = reader
            .read_len_type_at_with::<u32be, StopCondition>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let unk1 = reader.read_at::<InternedString>(position)?;
        test_any!(
            unk1,
            [
                "navigation_row",
                "navigation_kids",
                "navigation_default",
                "navigation_big_items",
                "navigation_age"
            ]
        )?;
        let decel_tape_label = reader.read_at::<InternedString>(position)?;
        let scroll_mode = reader.read_at::<u32be>(position)?;
        let time_between_steps = reader.read_at::<f32be>(position)?;
        let next_actions = reader
            .read_len_type_at::<u32be, InternedString>(position)?
            .collect::<Result<_, _>>()?;
        let prev_actions = reader
            .read_len_type_at::<u32be, InternedString>(position)?
            .collect::<Result<_, _>>()?;

        Ok(Self {
            key,
            sound_context,
            sound_notif_go_next,
            sound_notif_go_prev,
            stop_conditions,
            decel_tape_label,
            scroll_mode,
            time_between_steps,
            next_actions,
            prev_actions,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for CarouselBehaviourGoToElement<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xB45C_B89D)?;
        let key = reader.read_at::<InternedString>(position)?;
        let sound_context = reader.read_len_string_at::<u32be>(position)?;
        let sound_notif_go_next = reader.read_len_string_at::<u32be>(position)?;
        let sound_notif_go_prev = reader.read_len_string_at::<u32be>(position)?;
        let stop_conditions = reader
            .read_len_type_at_with::<u32be, StopCondition>(position, ctx)?
            .collect::<Result<_, _>>()?;
        let unk1 = reader.read_at::<InternedString>(position)?;
        test_any!(
            unk1,
            ["navigation_row", "navigation_default", "navigation_age"]
        )?;
        let decel_tape_label = reader.read_at::<InternedString>(position)?;
        let scroll_mode = reader.read_at::<u32be>(position)?;
        let time_between_steps = reader.read_at::<f32be>(position)?;

        Ok(Self {
            key,
            sound_context,
            sound_notif_go_next,
            sound_notif_go_prev,
            stop_conditions,
            decel_tape_label,
            scroll_mode,
            time_between_steps,
        })
    }
}

impl BinaryDeserialize<'_> for ClearColorComponent {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xAEBB_218B)?;
        let clear_color = reader.read_at::<Color>(position)?;
        let clear_front_light_color = reader.read_at::<Color>(position)?;
        let clear_back_light_color = reader.read_at::<Color>(position)?;

        Ok(Self {
            clear_color,
            clear_front_light_color,
            clear_back_light_color,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for ConvertedTmlTapeComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xCD07_BB76)?;
        let map_name = reader.read_len_string_at::<u32be>(position)?;

        Ok(Self { map_name })
    }
}

impl<'de> BinaryDeserialize<'de> for CreditsComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x342E_A4FC)?;
        let lines_number = reader.read_at::<u32be>(position)?;
        let name_font_size = reader.read_at::<f32be>(position)?;
        let title_font_size = reader.read_at::<f32be>(position)?;
        let big_title_font_size = reader.read_at::<f32be>(position)?;
        let very_big_title_font_size = reader.read_at::<f32be>(position)?;
        let anim_duration = reader.read_at::<f32be>(position)?;
        let lines_pos_offset = reader.read_at::<f32be>(position)?;
        let (min_anim_duration, speed_steps, bottom_spawn_y, top_spawn_y) =
            if ugi.game <= Game::JustDance2017 {
                (None, None, None, None)
            } else {
                let min_anim_duration = Some(reader.read_at::<f32be>(position)?);
                let speed_steps = Some(reader.read_at::<f32be>(position)?);
                let bottom_spawn_y = Some(reader.read_at::<f32be>(position)?);
                let top_spawn_y = Some(reader.read_at::<f32be>(position)?);
                (min_anim_duration, speed_steps, bottom_spawn_y, top_spawn_y)
            };
        let number_of_lines = usize::try_from(reader.read_at::<u32be>(position)?)?;
        let mut credits_lines = Vec::with_capacity(number_of_lines);
        for _ in 0..number_of_lines {
            let line = reader.read_len_string_at::<u32be>(position)?;
            credits_lines.push(line);
        }

        Ok(CreditsComponent {
            lines_number,
            name_font_size,
            title_font_size,
            big_title_font_size,
            very_big_title_font_size,
            anim_duration,
            lines_pos_offset,
            min_anim_duration,
            speed_steps,
            bottom_spawn_y,
            top_spawn_y,
            credits_lines,
        })
    }
}

impl BinaryDeserialize<'_> for FixedCameraComponent {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x3D5D_EBA2)?;
        let remote = reader.read_at::<u32be>(position)?;
        let offset_1 = reader.read_at::<f32be>(position)?;
        let offset_2 = reader.read_at::<f32be>(position)?;
        let offset_3 = reader.read_at::<f32be>(position)?;
        let start_as_main_cam = reader.read_at::<u32be>(position)?;
        Ok(Self {
            remote,
            offset: (offset_1, offset_2, offset_3),
            start_as_main_cam,
        })
    }
}

impl BinaryDeserialize<'_> for FXControllerComponent {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x8D4F_FFB6)?;
        let allow_bus_mix_events = reader.read_at::<u32be>(position)?;
        let allow_music_events = reader.read_at::<u32be>(position)?;
        Ok(Self {
            allow_bus_mix_events,
            allow_music_events,
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
        let atl_path = reader.read_at::<SplitPath>(position)?;
        let shader_path = reader.read_at::<SplitPath>(position)?;
        let material_params = reader.read_at::<GFXMaterialSerializableParam>(position)?;
        let stencil_test = reader.read_at::<u32be>(position)?;
        test_eq!(stencil_test, 0)?;
        let alpha_test = reader.read_at::<u32be>(position)?;
        test_eq!(alpha_test, u32::MAX)?;
        let alpha_ref = reader.read_at::<u32be>(position)?;
        test_eq!(alpha_ref, u32::MAX)?;

        Ok(Self {
            atl_channel,
            atl_path,
            shader_path,
            stencil_test,
            alpha_test,
            alpha_ref,
            texture_set,
            material_params,
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
            diffuse,
            back_light,
            normal,
            separate_alpha,
            diffuse_2,
            back_light_2,
            anim_impostor,
            diffuse_3,
            diffuse_4,
        })
    }
}

impl BinaryDeserialize<'_> for GFXPrimitiveParam {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let color_factor = reader.read_at::<Color>(position)?;
        test_eq!(
            color_factor,
            Color {
                color: (1.0, 1.0, 1.0, 1.0)
            }
        )?;
        let gfx_occlude_info = reader.read_at::<u32be>(position)?;
        test_eq!(gfx_occlude_info, 0x0)?;

        Ok(Self {
            color_factor,
            gfx_occlude_info,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for MaterialGraphicComponent<'de> {
    type Ctx = (UniqueGameId, bool);
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let (ugi, is_pleo) = ctx;
        for _ in 0..3 {
            let unk11 = reader.read_at::<u32be>(position)?;
            test_eq!(unk11, 0x3F80_0000u32)?;
        }
        let unk11_5 = reader.read_at::<u32be>(position)?;
        test_any!(unk11_5, [0x3F80_0000u32, 0x0])?;

        for _ in 0..2 {
            let _unk12: u64 = reader.read_at::<u64be>(position)?;
        }

        let unk13 = reader.read_at::<u32be>(position)?;
        test_any!(unk13, [0xFFFF_FFFFu32, 0x1])?;

        // <ENUM NAME="anchor" SEL="[0-9]" /> ?
        let unk14 = reader.read_at::<u64be>(position)?;
        test_ge!(unk14, 0u64).and(test_le!(unk14, 0x9u64))?;

        let unk15 = reader.read_at::<u64be>(position)?;
        test_any!(
            unk15,
            [
                0x0u64,
                0x3E2E_147B,
                0xC080_0000,
                0x3E99_999A_BDCC_CCCD,
                0xBDE1_47AE_3E61_47AE,
            ],
        )?;

        let mut files = [
            // diffuse
            SplitPath::default(),
            // back_light
            SplitPath::default(),
            // normal
            SplitPath::default(),
            // separateAlpha
            SplitPath::default(),
            // diffuse_2
            SplitPath::default(),
            // back_light_2
            SplitPath::default(),
            // anim_impostor
            SplitPath::default(),
            // diffuse_3
            SplitPath::default(),
            // diffuse_4
            SplitPath::default(),
            // ATL_Path
            SplitPath::default(),
            // shaderPath
            SplitPath::default(),
        ];

        for item in files.iter_mut().take(9) {
            let path = reader.read_at::<SplitPath>(position)?;
            *item = path;
        }

        let _unk19: u32 = reader.read_at::<u32be>(position)?;

        for item in files.iter_mut().skip(9) {
            let path = reader.read_at::<SplitPath>(position)?;
            *item = path;
        }

        match ugi.game {
            Game::JustDance2019
            | Game::JustDance2018
            | Game::JustDance2017
            | Game::JustDance2016 => {
                let _unk20: u64 = reader.read_at::<u64be>(position)?;
            }
            _ => {
                for _ in 0..4 {
                    let _unk20: u64 = reader.read_at::<u64be>(position)?;
                }

                let _unk20_5: u32 = reader.read_at::<u32be>(position)?;

                let _unk21: u32 = reader.read_at::<u32be>(position)?;
            }
        }

        if ugi.game == Game::JustDance2020 {
            // Just Dance 2020 sometimes has a 0u32 inbetween
            let unk21_5: u32 = reader.read_at::<u32be>(position)?;
            if unk21_5 != 0 {
                *position -= 4;
            }
        }

        let unk22 = reader.read_at::<u64be>(position)?;
        test_eq!(unk22, 0xFFFF_FFFF_FFFF_FFFFu64)?;

        for _ in 0..3 {
            let _unk23: u32 = reader.read_at::<u32be>(position)?;
        }

        let _unk24: u32 = reader.read_at::<u32be>(position)?;

        let _unk25: u64 = reader.read_at::<u64be>(position)?;

        // <ENUM NAME="oldAnchor" SEL="[0-9]" /> ?
        let unk26 = reader.read_at::<u32be>(position)?;
        test_ge!(unk26, 0).and(test_le!(unk26, 9))?;

        if is_pleo {
            let unk27 = reader.read_at::<u32be>(position)?;
            test_eq!(unk27, 0x0u32)?;
        }

        Ok(MaterialGraphicComponent {
            files,
            unk11_5,
            unk13,
            unk14,
            unk15,
            unk26,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for PictoTimeline<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xFA24_128D)?;
        let text = reader.read_len_string_at::<u32be>(position)?;
        let loc_id = reader.read_at::<u32be>(position)?;
        // probably an UIWidgetElementDesc list that is always zero
        let elements = reader.read_at::<u32be>(position)?;
        test_eq!(elements, 0)?;
        let model_name = reader.read_at::<InternedString>(position)?;
        let flag = reader.read_len_string_at::<u32be>(position)?;
        let relative_start_position_solo = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let relative_start_position_duo = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let relative_start_position_trio = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let relative_start_position_quatro = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let relative_start_position_sextet = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let shifting_position_solo = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let shifting_position_duo = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let shifting_position_trio = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let shifting_position_quatro = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let shifting_position_sextet = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let picto_track_offset = reader.read_at::<u32be>(position)?;
        let picto_scale = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );

        Ok(Self {
            text,
            loc_id,
            model_name,
            flag,
            relative_start_position_solo,
            relative_start_position_duo,
            relative_start_position_trio,
            relative_start_position_quatro,
            relative_start_position_sextet,
            shifting_position_solo,
            shifting_position_duo,
            shifting_position_trio,
            shifting_position_quatro,
            shifting_position_sextet,
            picto_track_offset,
            picto_scale,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for RegistrationComponent<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xE0A2_4B6D)?;
        let tag = reader.read_at::<InternedString>(position)?;
        let user_data = reader.read_len_string_at::<u32be>(position)?;
        Ok(Self { tag, user_data })
    }
}

impl<'de> BinaryDeserialize<'de> for SingleInstanceMesh3DComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x53E3_2AF7)?;
        test_le!(
            ctx.game,
            Game::JustDance2017,
            "SingleInstanceMesh3DComponent should not exist in newer games"
        )?;
        let primitive_parameters = reader.read_at_with::<GFXPrimitiveParam>(position, ctx)?;
        let color_computer_tag_id = reader.read_at::<u32be>(position)?;
        test_eq!(color_computer_tag_id, 0)?;
        let render_in_target = reader.read_at::<u32be>(position)?;
        test_eq!(render_in_target, 0)?;
        let disable_light = reader.read_at::<u32be>(position)?;
        test_eq!(disable_light, 0)?;
        let disable_shadow = reader.read_at::<u32be>(position)?;
        test_eq!(disable_shadow, 0xFFFF_FFFF)?;
        let animation_player_mode = reader.read_at::<u32be>(position)?;
        test_eq!(animation_player_mode, 0)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 1)?;
        let material = reader.read_at_with::<GFXMaterialSerializable>(position, ctx)?;
        let mesh_3d = reader.read_at::<SplitPath>(position)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0x0)?;
        let skeleton_3d = reader.read_at::<SplitPath>(position)?;
        let animation_3d = reader.read_at::<SplitPath>(position)?;
        let animation_node = reader.read_at::<SplitPath>(position)?;
        let orientation = [
            reader.read_at::<Color>(position)?,
            reader.read_at::<Color>(position)?,
            reader.read_at::<Color>(position)?,
            reader.read_at::<Color>(position)?,
        ];

        Ok(Self {
            primitive_parameters,
            color_computer_tag_id,
            render_in_target,
            disable_light,
            disable_shadow,
            scale_z: 0.0,
            mesh_3d,
            skeleton_3d,
            animation_3d,
            animation_node,
            orientation,
            material,
            animation_player_mode,
        })
    }
}

impl BinaryDeserialize<'_> for SoundComponent {
    type Ctx = ();
    type Output = ();

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x7DD8_643C)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0)?;
        Ok(())
    }
}

impl BinaryDeserialize<'_> for StopCondition {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_any!(unk1, [0x0, 0x2])?;
        let waiting_time = reader.read_at::<f32be>(position)?;
        let count_to_reach = reader.read_at::<u32be>(position)?;
        let next_behaviour = reader.read_at::<InternedString>(position)?;
        let condition = reader.read_at::<u32be>(position)?;
        let anim_state = reader.read_at::<u32be>(position)?;
        Ok(Self {
            waiting_time,
            count_to_reach,
            next_behaviour,
            condition,
            anim_state,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for TextureGraphicComponent<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x7B48_A9AE)?;
        let primitive_parameters = reader.read_at_with::<GFXPrimitiveParam>(position, ctx)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0)?;
        let disable_shadow = reader.read_at::<u32be>(position)?;
        test_eq!(disable_shadow, u32::MAX)?;
        let sprite_index = reader.read_at::<u32be>(position)?;
        test_eq!(sprite_index, u32::MAX)?;
        let anchor = reader.read_at::<u32be>(position)?;
        test_eq!(anchor, 1)?;
        let material = reader.read_at_with::<GFXMaterialSerializable>(position, ctx)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0xFFFF_FFFF)?;

        Ok(Self {
            primitive_parameters,
            color_computer_tag_id: 0,
            render_in_target: 0,
            disable_light: 0,
            disable_shadow,
            sprite_index,
            anchor,
            material,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for UICarousel<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        match ugi.game {
            Game::JustDance2019
            | Game::JustDance2020
            | Game::JustDanceChina
            | Game::JustDance2021
            | Game::JustDance2022 => Ok(UICarousel::V1922(
                reader.read_at_with::<UICarouselV1922>(position, ugi)?,
            )),
            Game::JustDance2017 | Game::JustDance2018 => Ok(UICarousel::V1718(
                reader.read_at_with::<UICarouselV1718>(position, ugi)?,
            )),
            Game::JustDance2016 => Ok(UICarousel::V16(
                reader.read_at_with::<UICarouselV16>(position, ugi)?,
            )),
            _ => todo!("UICarousel for {ugi}"),
        }
    }
}

impl<'de> BinaryDeserialize<'de> for UICarouselV16<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x8782_FE60)?;

        let acceleration = reader.read_at::<f32be>(position)?;
        test_eq!(acceleration, 80.0)?;
        let deceleration = reader.read_at::<f32be>(position)?;
        test_eq!(deceleration, 7.0)?;
        let min_speed = reader.read_at::<f32be>(position)?;
        test_eq!(min_speed, 1.0)?;
        let max_speed = reader.read_at::<f32be>(position)?;
        test_any!(max_speed, [5.0, 10.0])?;

        let main_anchor = reader.read_at::<u32be>(position)?;
        test_any!(main_anchor, [0x0, 0x2, 0x3, 0x4])?;

        let min_deceleration_start_ratio = reader.read_at::<f32be>(position)?;
        test_eq!(min_deceleration_start_ratio, 0.8)?;
        let max_deceleration_start_ratio = reader.read_at::<f32be>(position)?;
        test_eq!(max_deceleration_start_ratio, 1.0)?;
        let next_actions = reader
            .read_len_type_at::<u32be, InternedString>(position)?
            .collect::<Result<Vec<_>, _>>()?;
        let prev_actions = reader
            .read_len_type_at::<u32be, InternedString>(position)?
            .collect::<Result<Vec<_>, _>>()?;

        let validate_action = reader.read_at::<InternedString>(position)?;
        let carousel_data_id = reader.read_len_string_at::<u32be>(position)?;

        let time_between_step = reader.read_at::<f32be>(position)?;
        test_eq!(time_between_step, 0.15)?;
        let sound_context = reader.read_len_string_at::<u32be>(position)?;
        test_eq!(sound_context.as_ref(), "Carousel")?;
        let sound_notif_go_next = reader.read_len_string_at::<u32be>(position)?;
        test_eq!(sound_notif_go_next.as_ref(), "Next")?;
        let sound_notif_go_prev = reader.read_len_string_at::<u32be>(position)?;
        test_eq!(sound_notif_go_prev.as_ref(), "Prev")?;

        let mode = reader.read_at::<i32be>(position)?;
        test_any!(mode, 1..=3)?;

        let anim_items_desc = reader.read_at_with::<CarouselAnimItemsDesc>(position, ugi)?;

        let force_loop = reader.read_at::<u32be>(position)?;
        test_any!(force_loop, 0x0..=0x1)?;
        let focus_anims_on_disabled_items = reader.read_at::<u32be>(position)?;
        test_eq!(focus_anims_on_disabled_items, 0x0)?;
        let manage_carousel_history = reader.read_at::<u32be>(position)?;
        test_any!(manage_carousel_history, 0x0..=0x1)?; // only 1 when unk7 is 1
        let min_nb_items_to_loop = reader.read_at::<u32be>(position)?;
        test_any!(min_nb_items_to_loop, [0x0, 0x9])?; // only 9 when unk7 is 1
        let auto_scroll = reader.read_at::<u32be>(position)?;
        test_eq!(auto_scroll, 0x0)?;

        let auto_scroll_pause_time = reader.read_at::<f32be>(position)?;
        test_eq!(auto_scroll_pause_time, 4.0)?;
        let auto_scroll_max_speed_ratio = reader.read_at::<f32be>(position)?;
        test_eq!(auto_scroll_max_speed_ratio, 1.0)?;

        Ok(Self {
            acceleration,
            deceleration,
            min_speed,
            max_speed,
            main_anchor,
            min_deceleration_start_ratio,
            max_deceleration_start_ratio,
            validate_action,
            carousel_data_id,
            time_between_step,
            sound_notif_go_next,
            sound_notif_go_prev,
            force_loop,
            focus_anims_on_disabled_items,
            manage_carousel_history,
            min_nb_items_to_loop,
            auto_scroll,
            auto_scroll_pause_time,
            auto_scroll_max_speed_ratio,
            sound_context,
            mode,
            next_actions,
            prev_actions,
            anim_items_desc,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for UICarouselV1718<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x8782_FE60)?;

        let main_anchor = reader.read_at::<u32be>(position)?;
        test_any!(main_anchor, [0x0, 0x2, 0x3, 0x4])?;

        let validate_action = reader.read_at::<InternedString>(position)?;
        let carousel_data_id = reader.read_len_string_at::<u32be>(position)?;

        let anim_items_desc = reader.read_at_with::<CarouselAnimItemsDesc>(position, ugi)?;
        let min_nb_items_to_loop = reader.read_at::<u32be>(position)?;
        let force_loop = reader.read_at::<u32be>(position)?;

        let manage_carousel_history = reader.read_at::<u32be>(position)?;
        let initial_behaviour = reader.read_at::<InternedString>(position)?;
        let sound_context = reader.read_len_string_at::<u32be>(position)?;
        let behaviours = reader
            .read_len_type_at_with::<u32be, CarouselBehaviour>(position, ugi)?
            .collect::<Result<_, _>>()?;

        let unk2 = reader.read_at::<u32be>(position)?;
        if ugi.game == Game::JustDance2018 {
            test_eq!(unk2, 0x7FFF_FFFF)?;
        } else {
            test_any!(
                unk2,
                [
                    0x0,
                    0x002E_006C,
                    0x002E_0077,
                    0x005C_006F,
                    0x0061_002E,
                    0x0063_0061,
                    0x006B_0063,
                    0x006C_0074,
                    0x0079_0000,
                    0x2E63_6972,
                    0x656C_6261,
                    0x6972_506F
                ]
            )?;
        }

        Ok(Self {
            main_anchor,
            validate_action,
            carousel_data_id,
            force_loop,
            manage_carousel_history,
            min_nb_items_to_loop,
            initial_behaviour,
            sound_context,
            behaviours,
            anim_items_desc,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for UICarouselV1922<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x8782_FE60)?;

        let main_anchor = reader.read_at::<u32be>(position)?;
        test_any!(main_anchor, [0x0, 0x2, 0x3, 0x4])?;

        let validate_action = reader.read_at::<InternedString>(position)?;
        let carousel_data_id = reader.read_len_string_at::<u32be>(position)?;

        let manage_carousel_history = reader.read_at::<u32be>(position)?;
        let initial_behaviour = reader.read_at::<InternedString>(position)?;
        let sound_context = reader.read_len_string_at::<u32be>(position)?;
        let behaviours = reader
            .read_len_type_at_with::<u32be, CarouselBehaviour>(position, ugi)?
            .collect::<Result<_, _>>()?;

        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0x7FFF_FFFF)?;

        let anim_items_desc = reader.read_at_with::<CarouselAnimItemsDesc>(position, ugi)?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_any!(unk8, [0x0, 0x7, 0x9])?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_any!(unk9, [0x0, 0x1])?;

        if ugi >= UniqueGameId::NX2019V2 {
            let unk10 = reader.read_at::<u32be>(position)?;
            test_any!(unk10, [0x4000_0000, 0x9000_0000])?;
        }

        Ok(Self {
            main_anchor,
            validate_action,
            carousel_data_id,
            manage_carousel_history,
            initial_behaviour,
            sound_context,
            behaviours,
            anim_items_desc,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for UITextBox<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xD10C_BEED)?;
        let style = reader.read_at::<u32be>(position)?;
        test_any!(style, 0..=3)?;
        let overriding_font_size = reader.read_at::<f32be>(position)?;
        test_any!(
            overriding_font_size,
            [-1.0, 20.0, 30.0, 32.0, 50.0, 70.0, 100.0, 150.0]
        )?;
        let text_case = reader.read_at::<i32be>(position)?;
        test_eq!(text_case, -1)?;
        let offset = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        test_eq!(offset, (0.0, 0.0))?;
        let scale = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        test_eq!(scale, (1.0, 1.0))?;
        let alpha = reader.read_at::<f32be>(position)?;
        test_any!(alpha, [0.0, 1.0])?;
        let max_width = reader.read_at::<f32be>(position)?;
        test_any!(max_width, [-1.0, 1200.0])?;
        let max_height = reader.read_at::<f32be>(position)?;
        test_eq!(max_height, -1.0)?;
        let area = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );

        let raw_text = reader.read_len_string_at::<u32be>(position)?;

        let use_lines_max_count = reader.read_at::<u32be>(position)?;
        test_eq!(use_lines_max_count, 0u32)?;
        let lines_max_count = reader.read_at::<u32be>(position)?;
        test_eq!(lines_max_count, 1u32)?;
        let loc_id = reader.read_at::<u32be>(position)?;
        let auto_scroll_speed = reader.read_at::<f32be>(position)?;
        test_any!(auto_scroll_speed, [-15.0, -10.0, 0.0, 12.0])?;
        let auto_scroll_speed_y = reader.read_at::<f32be>(position)?;
        test_eq!(auto_scroll_speed_y, 0.0)?;
        let auto_scroll_wait_time = reader.read_at::<f32be>(position)?;
        test_eq!(auto_scroll_wait_time, 0.0)?;
        let auto_scroll_wait_time_y = reader.read_at::<f32be>(position)?;
        test_eq!(auto_scroll_wait_time_y, 0.0)?;

        let auto_scroll_font_effect_name = reader.read_len_string_at::<u32be>(position)?;

        let auto_scroll_reset_on_inactive = reader.read_at::<u32be>(position)?;
        test_eq!(auto_scroll_reset_on_inactive, 0u32)?;
        let scroll_once = reader.read_at::<u32be>(position)?;
        test_eq!(scroll_once, 0x0u32)?;

        let unk2 = reader.read_at::<u32be>(position)?;
        test_any!(unk2, [0u32, 0x2, 0xFFFF_FFFF])?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_any!(unk3, [0u32, 0xFFFF_FFFF])?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_any!(unk4, [0u32, 0xBF80_0000])?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_any!(unk5, [0u32, 0xFFFF_FFFF])?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_any!(unk6, [0u32, 0xBF80_0000])?;

        let unk7 = reader.read_at::<i32be>(position)?;
        test_eq!(unk7, 0)?;
        let unk8 = reader.read_at::<i32be>(position)?;
        test_eq!(unk8, 0)?;
        let unk9 = reader.read_at::<i32be>(position)?;
        test_eq!(unk9, 0)?;
        let unk10 = reader.read_at::<i32be>(position)?;
        test_eq!(unk10, 0)?;

        let overriding_shadow_color = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );

        let overriding_shadow_offset = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );

        let overriding_line_spacing = reader.read_at::<f32be>(position)?;
        test_any!(overriding_line_spacing, [0.0, -6.0])?;
        let unk18 = reader.read_at::<u32be>(position)?;
        test_eq!(unk18, 0)?;

        if ugi >= UniqueGameId::NX2019V1 {
            let unk19 = reader.read_at::<i32be>(position)?;
            test_eq!(unk19, 0)?;
            let unk20 = reader.read_at::<u32be>(position)?;
            test_eq!(unk20, 0)?;
        }

        let overriding_font_size_min = reader.read_at::<f32be>(position)?;
        test_eq!(overriding_font_size_min, -1.0)?;
        let ending_dots = reader.read_at::<u32be>(position)?;
        test_eq!(ending_dots, 0)?;
        let colorize_icons = if ugi >= UniqueGameId::NX2020 {
            let colorize_icons = reader.read_at::<u32be>(position)?;
            test_eq!(colorize_icons, 0)?;
            Some(colorize_icons)
        } else {
            None
        };
        let unk25 = reader.read_at::<u32be>(position)?;
        test_any!(unk25, [0xFFFF_FFFFu32, 0x1, 0x2])?;
        let unk26 = reader.read_at::<u32be>(position)?;
        test_any!(unk26, [0xFFFF_FFFFu32, 0x1, 0x0])?;
        let overriding_anchor = reader.read_at::<i32be>(position)?;
        test_any!(overriding_anchor, -1..=8)?;
        Ok(UITextBox {
            style,
            overriding_font_size,
            offset,
            scale,
            alpha,
            max_width,
            max_height,
            area,
            raw_text,
            use_lines_max_count,
            lines_max_count,
            loc_id,
            auto_scroll_speed,
            auto_scroll_speed_y,
            auto_scroll_wait_time,
            auto_scroll_wait_time_y,
            auto_scroll_font_effect_name,
            auto_scroll_reset_on_inactive,
            scroll_once,
            overriding_shadow_color,
            overriding_shadow_offset,
            overriding_line_spacing,
            overriding_font_size_min,
            ending_dots,
            colorize_icons,
            overriding_anchor,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for UIWidgetElementDesc<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let element_path = reader.read_at::<SplitPath>(position)?;
        let name = reader.read_len_string_at::<u32be>(position)?;
        let flag = reader.read_len_string_at::<u32be>(position)?;
        let parent_index = reader.read_at::<i32be>(position)?;
        let bind_mode = reader.read_at::<u32be>(position)?;

        Ok(Self {
            element_path,
            name,
            flag,
            parent_index,
            bind_mode,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for UIWidgetGroupHUD<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x1528_D94A)?;
        let text = reader.read_len_string_at::<u32be>(position)?;
        let loc_id = reader.read_at::<u32be>(position)?;
        let elements = reader
            .read_len_type_at_with::<u32be, UIWidgetElementDesc>(position, ugi)?
            .collect::<Result<_, _>>()?;
        let model_name = reader.read_at::<InternedString>(position)?;
        let flag = reader.read_len_string_at::<u32be>(position)?;

        Ok(Self {
            text,
            loc_id,
            model_name,
            flag,
            elements,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for UIWidgetGroupHUDAutodanceRecorder<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x9F87_350C)?;
        let text = reader.read_len_string_at::<u32be>(position)?;
        let loc_id = reader.read_at::<u32be>(position)?;
        let elements = reader
            .read_len_type_at_with::<u32be, UIWidgetElementDesc>(position, ugi)?
            .collect::<Result<_, _>>()?;
        let model_name = reader.read_at::<InternedString>(position)?;
        let flag = reader.read_len_string_at::<u32be>(position)?;
        let icon_default_position = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let icon_relative_start_position_solo = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let icon_relative_start_position_duo = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let icon_relative_start_position_trio = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let icon_relative_start_position_quatro = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let icon_relative_start_position_sextet = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let icon_shifting_position_solo = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let icon_shifting_position_duo = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let icon_shifting_position_trio = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let icon_shifting_position_quatro = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );
        let icon_shifting_position_sextet = (
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
            reader.read_at::<f32be>(position)?,
        );

        Ok(Self {
            text,
            loc_id,
            model_name,
            flag,
            icon_default_position,
            icon_relative_start_position_solo,
            icon_relative_start_position_duo,
            icon_relative_start_position_trio,
            icon_relative_start_position_quatro,
            icon_relative_start_position_sextet,
            icon_shifting_position_solo,
            icon_shifting_position_duo,
            icon_shifting_position_trio,
            icon_shifting_position_quatro,
            icon_shifting_position_sextet,
            elements,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for UIWidgetGroupHUDLyrics<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0xF22C_9426)?;
        let text = reader.read_len_string_at::<u32be>(position)?;
        let loc_id = reader.read_at::<u32be>(position)?;
        let elements = reader
            .read_len_type_at_with::<u32be, UIWidgetElementDesc>(position, ugi)?
            .collect::<Result<_, _>>()?;
        let model_name = reader.read_at::<InternedString>(position)?;
        let flag = reader.read_len_string_at::<u32be>(position)?;

        Ok(Self {
            text,
            loc_id,
            model_name,
            flag,
            elements,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for UIWidgetGroupHUDPauseIcon<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x4866_6BB2)?;
        let text = reader.read_len_string_at::<u32be>(position)?;
        let loc_id = reader.read_at::<u32be>(position)?;
        let elements = reader
            .read_len_type_at_with::<u32be, UIWidgetElementDesc>(position, ugi)?
            .collect::<Result<_, _>>()?;
        let model_name = reader.read_at::<InternedString>(position)?;
        let flag = reader.read_len_string_at::<u32be>(position)?;

        Ok(Self {
            text,
            loc_id,
            model_name,
            flag,
            elements,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Unknown77F7D66C<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x77F7_D66C)?;
        let mapname = reader.read_len_string_at::<u32be>(position)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_any!(unk1, 2015..=2016)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_any!(unk3, [0, 1])?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 7)?;
        let unk5 = reader.read_slice_at(position, usize::try_from(unk4)?)?;
        let unk6 = reader.read_at::<f32be>(position)?;
        test_any!(
            unk6,
            [0.112503, 0.225006, 0.337508, 0.450011, 0.675017, 0.750019, 0.825021]
        )?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0x0)?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_eq!(unk8, 0x0)?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_eq!(unk9, 0xFFFF_FFFF)?;
        let unk10 = reader.read_at::<u32be>(position)?;
        test_eq!(unk10, 0x0)?;
        let unk11 = reader.read_at::<u32be>(position)?;
        test_eq!(unk11, 0x0)?;
        let unk12 = reader.read_at::<u32be>(position)?;
        test_eq!(unk12, 0x0)?;
        let unk13 = reader.read_at::<u32be>(position)?;
        test_eq!(unk13, 0x0)?;
        let unk14 = reader.read_at::<u32be>(position)?;
        test_eq!(unk14, 0xFFFF_FFFF)?;
        let unk15 = reader.read_at::<u32be>(position)?;
        test_eq!(unk15, 0x0)?;

        Ok(Self { mapname, unk5 })
    }
}

struct UnknownFooter;
impl BinaryDeserialize<'_> for UnknownFooter {
    type Ctx = UniqueGameId;
    type Output = ();

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x0)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0x0)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_eq!(unk3, 0xFFFF_FFFF)?;
        let unk4 = reader.read_at::<u32be>(position)?;
        test_eq!(unk4, 0x0)?;
        let unk5 = reader.read_at::<u32be>(position)?;
        test_eq!(unk5, 0x0)?;
        let unk6 = reader.read_at::<u32be>(position)?;
        test_eq!(unk6, 0x0)?;
        let unk7 = reader.read_at::<u32be>(position)?;
        test_eq!(unk7, 0x0)?;
        let unk8 = reader.read_at::<u32be>(position)?;
        test_eq!(unk8, 0x1)?;
        let unk9 = reader.read_at::<u32be>(position)?;
        test_eq!(unk9, 0x0)?;
        let unk10 = reader.read_at::<u32be>(position)?;
        test_eq!(unk10, 0x0)?;
        Ok(())
    }
}

impl BinaryDeserialize<'_> for ViewportUIComponent {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ugi: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, 0x6990_834C)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, u32::MAX)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0)?;
        let focale = reader.read_at::<f32be>(position)?;
        test_eq!(focale, std::f32::consts::FRAC_PI_4)?;
        let far_plane = reader.read_at::<f32be>(position)?;
        test_eq!(far_plane, 1000.0)?;
        let position_0 = reader.read_at::<f32be>(position)?;
        test_eq!(position_0, 0.21)?;
        let position_1 = reader.read_at::<f32be>(position)?;
        test_eq!(position_1, 0.049)?;
        let size_0 = reader.read_at::<f32be>(position)?;
        test_eq!(size_0, 0.585)?;
        let size_1 = reader.read_at::<f32be>(position)?;
        test_eq!(size_1, 0.585)?;

        Ok(Self {
            active: 0,
            focale,
            far_plane,
            position: (position_0, position_1),
            size: (size_0, size_1),
            view_mask: 0,
        })
    }
}

/// Parse the AFX post process component of an actor
fn parse_afx_post_process_component(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    let unk16 = reader.read_at::<u64be>(position)?;
    test_eq!(unk16, 8u64)?;
    let unk17 = reader.read_at::<u32be>(position)?;
    test_eq!(unk17, 1u32)?;
    Ok(())
}

/// Parse the fx bank component of an actor
fn parse_fx_bank_component(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    for _ in 0..4 {
        let unk11 = reader.read_at::<u32be>(position)?;
        test_eq!(unk11, 0x3F80_0000u32)?;
    }
    for _ in 0..3 {
        let unk12 = reader.read_at::<u32be>(position)?;
        test_eq!(unk12, 0u32)?;
    }
    let unk13 = reader.read_at::<u32be>(position)?;
    test_any!(unk13, [0x0u32, 0xFFFF_FFFF])?;
    let unk14 = reader.read_at::<u32be>(position)?;
    test_eq!(unk14, 0xFFFF_FFFFu32)?;
    Ok(())
}

/// Parse the bezier tree component of an actor
fn parse_bezier_tree_component(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    let unk18 = reader.read_at::<u32be>(position)?;
    test_eq!(unk18, 2u32)?;
    for _ in 0..4 {
        let unk19 = reader.read_at::<u32be>(position)?;
        test_eq!(unk19, 0u32)?;
    }
    for _ in 0..2 {
        let unk20 = reader.read_at::<u32be>(position)?;
        test_eq!(unk20, 0x3F80_0000u32)?;
    }
    for _ in 0..2 {
        let unk21 = reader.read_at::<u32be>(position)?;
        test_eq!(unk21, 0u32)?;
    }
    let unk22 = reader.read_at::<u32be>(position)?;
    test_eq!(unk22, 0x4040_0000u32)?;
    for _ in 0..2 {
        let unk23 = reader.read_at::<u32be>(position)?;
        test_eq!(unk23, 0u32)?;
    }
    for _ in 0..2 {
        let unk24 = reader.read_at::<u32be>(position)?;
        test_eq!(unk24, 0x3F80_0000u32)?;
    }
    for _ in 0..3 {
        let unk25 = reader.read_at::<u32be>(position)?;
        test_eq!(unk25, 0u32)?;
    }
    let unk26 = reader.read_at::<u32be>(position)?;
    test_eq!(unk26, 1u32)?;
    Ok(())
}

impl<'de> BinaryDeserialize<'de> for PleoComponent<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let video = reader.read_at::<SplitPath>(position)?;
        let dash_mpd = reader.read_at::<SplitPath>(position)?;
        let channel_id = reader.read_len_string_at::<u32be>(position)?;
        let channel_id = if channel_id.is_empty() {
            None
        } else {
            Some(channel_id)
        };
        Ok(PleoComponent {
            video,
            dash_mpd,
            channel_id,
        })
    }
}

/// Parse the property patcher component of an actor
fn parse_property_patcher(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
    gp: UniqueGameId,
) -> Result<(), ReadError> {
    let unk11 = reader.read_at::<u32be>(position)?;
    test_eq!(unk11, 1u32)?;
    let unk12 = reader.read_at::<u32be>(position)?;
    test_eq!(unk12, 0u32)?;
    if gp.game != Game::JustDance2017 {
        let unk13 = reader.read_at::<u32be>(position)?;
        test_eq!(unk13, 0u32)?;
    }
    Ok(())
}

struct InternedString;
impl BinaryDeserialize<'_> for InternedString {
    type Ctx = ();
    type Output = &'static str;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let string_id = reader.read_at::<u32be>(position)?;
        match string_id {
            0x1445_31FF => Ok("navigation_default"),
            0x1576_B015 => Ok("navigation_row"),
            0x2810_2F02 => Ok("navigation_speed"),
            0x40A1_5156 => Ok("menu_valid"),
            0x418F_AF9A => Ok("menu_lstick_right"),
            0x4C55_6308 => Ok("navigation"),
            0x6DC2_DBB2 => Ok("menu_phone_right"),
            0x7233_490C => Ok("menu_dpad_right"),
            0x7411_331E => Ok("navigation_age"),
            0x83B2_58E1 => Ok("gotodefault"),
            0x8E09_B64A => Ok("navigation_kids"),
            0xAA55_B6BD => Ok("asyncplayervideo"),
            0xB20E_35D5 => Ok("navigation_big_items"),
            0xC33B_4C02 => Ok("menu_phone_left"),
            0xD64E_0E2A => Ok("menu_dpad_left"),
            0xD9B1_E95C => Ok("menu_lstick_left"),
            0xDFEF_DBFB => Ok("decel"),
            0xFFFF_FFFF => Ok(""),
            _ => Err(ReadError::custom(format!(
                "Unknown interned string id: 0x{string_id:08x}"
            ))),
        }
    }
}
