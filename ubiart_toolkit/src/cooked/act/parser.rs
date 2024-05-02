//! Contains the parser implementation

#[cfg(not(feature = "fuzz"))]
use dotstar_toolkit_utils::testing::test_not;
use dotstar_toolkit_utils::{
    bytes::{
        primitives::{u32be, u64be},
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    testing::{test_any, test_eq, test_ge, test_le},
};

use super::{
    Actor, Component, CreditsComponent, MaterialGraphicComponent, PleoComponent, UITextBox,
};
use crate::utils::{Game, SplitPath, UniqueGameId};

pub fn parse<'de>(
    reader: &'de (impl ReadAtExt + ?Sized),
    position: &mut u64,
    gp: UniqueGameId,
) -> Result<Actor<'de>, ReadError> {
    let unk0 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk0, &1u32)?;
    let unk1 = reader.read_at::<u32be>(position)?.into();
    test_any(
        &unk1,
        &[
            0x0,
            0x3D23_D70A,
            0x3DCC_CCCD,
            0x3F66_6C4C,
            0x3F80_0000,
            0x4000_0000,
            0x4040_0000,
        ],
    )?;
    let unk2 = reader.read_at::<u32be>(position)?.into();
    test_any(
        &unk2,
        &[
            0x3EE7_720D,
            0x3F00_0000,
            0x3F4C_CCCD,
            0x3F80_0000,
            0x4240_0000,
            0x42F0_0000,
            0x4320_0000,
            0x43AF_0000,
            0x43C8_0000,
            0x4420_0000,
            0x4422_8000,
        ],
    )?;
    let unk2_5 = reader.read_at::<u32be>(position)?.into();
    test_any(
        &unk2_5,
        &[
            0x3EE7_720D,
            0x3F00_0000,
            0x3F4C_CCCD,
            0x3F80_0000,
            0x4120_0000,
            0x4180_0000,
            0x4240_0000,
            0x42F0_0000,
            0x4316_0000,
            0x4320_0000,
        ],
    )?;
    let unk3 = reader.read_at::<u64be>(position)?.into();
    test_eq(&unk3, &0u64)?;
    let unk3_5 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk3_5, &0u32)?;
    match gp.game {
        Game::JustDance2022 | Game::JustDance2021 | Game::JustDance2020 | Game::JustDance2019 => {
            let unk4 = reader.read_at::<u64be>(position)?.into();
            test_eq(&unk4, &0x1_0000_0000)?;
        }
        _ => {
            let unk4 = reader.read_at::<u32be>(position)?.into();
            test_any(&unk4, &[0x1u32, 0x0])?;
            if unk4 == 0x1 {
                let unk4_5 = reader.read_at::<u32be>(position)?.into();
                test_eq(&unk4_5, &0u32)?;
            }
        }
    };
    let unk5 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk5, &0u32)?;
    let unk6 = reader.read_at::<u64be>(position)?.into();
    test_eq(&unk6, &0)?;
    let unk7 = reader.read_at::<u64be>(position)?.into();
    test_eq(&unk7, &0xFFFF_FFFF)?;
    let unk8 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk8, &0u32)?;

    let tpl = reader.read_at::<SplitPath>(position)?;
    #[cfg(not(feature = "fuzz"))]
    {
        test_not(tpl.is_empty())?;
    }
    let unk9 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk9, &0u32)?;
    let actor_amount: usize = reader.read_at::<u32be>(position)?.try_into()?;

    let mut components = Vec::with_capacity(actor_amount);
    for _ in 0..actor_amount {
        let component = parse_component(reader, position, gp)?;
        components.push(component);
    }

    Ok(Actor {
        tpl,
        unk1,
        unk2,
        unk2_5,
        components,
    })
}

pub fn parse_component<'de>(
    reader: &'de (impl ReadAtExt + ?Sized),
    position: &mut u64,
    gp: UniqueGameId,
) -> Result<Component<'de>, ReadError> {
    // String id of the class name of the template without the '_Template' but including 'JD_' if it is in the class name
    let component_type: u32 = reader.read_at::<u32be>(position)?.into();

    let component = match component_type {
        // JD_AutoDanceComponent
        0x67B8_BB77 => Component::AutodanceComponent,
        // JD_BeatPulseComponent
        0x7184_37A8 => todo!("BeatPulseComponent"),
        // BoxInterpolatorComponent
        0xF513_60DA => {
            parse_box_interpolator_component(reader, position)?;
            Component::BoxInterpolatorComponent
        }
        // CameraGraphicComponent
        0xC760_4FA1 => todo!("CameraGraphicComponent"),
        // ClearColorComponent
        0xAEBB_218B => todo!("ClearColorComponent"),
        // ConvertedTmlTape_Component
        0xCD07_BB76 => {
            parse_converted_tml_tape(reader, position)?;
            Component::ConvertedTmlTapeComponent
        }
        // JD_CreditsComponent
        0x342E_A4FC => Component::CreditsComponent(parse_credits_component(reader, position, gp)?),
        // JD_FixedCameraComponent
        0x3D5D_EBA2 => {
            parse_fixed_camera_component(reader, position)?;
            Component::FixedCameraComponent
        }
        // FXControllerComponent
        0x8D4F_FFB6 => {
            parse_fx_controller(reader, position)?;
            Component::FXControllerComponent
        }
        // MasterTape
        0x677B_269B => Component::MasterTape,
        // MaterialGraphicComponent
        0x72B6_1FC5 => Component::MaterialGraphicComponent(parse_material_graphic_component(
            reader, position, gp, false,
        )?),
        // JD_Carousel
        0x27E4_80C0 => todo!("Carousel"),
        // JD_PictoComponent
        0xC316_BF34 => Component::PictoComponent,
        // PleoComponent
        0x1263_DAD9 => Component::PleoComponent(reader.read_at::<PleoComponent>(position)?),
        // PleoTextureGraphicComponent
        0x0579_E81B => Component::PleoTextureGraphicComponent(parse_material_graphic_component(
            reader, position, gp, true,
        )?),
        // PropertyPatcher
        0xF719_B524 => {
            parse_property_patcher(reader, position, gp)?;
            Component::PropertyPatcher
        }
        // JD_RegistrationComponent
        0xE0A2_4B6D => {
            parse_registration_component(reader, position)?;
            Component::RegistrationComponent
        }
        // SingleInstanceMesh3DComponent
        0x53E3_2AF7 => todo!("SingleInstanceMesh3DComponent"),
        // JD_SongDatabaseComponent
        0x4055_79FB => Component::SongDatabaseComponent,
        // JD_SongDescComponent
        0xE07F_CC3F => Component::SongDescComponent,
        // SoundComponent
        0x7DD8_643C => todo!("SoundComponent"),
        // TapeCase_Component
        0x231F_27DE => Component::TapeCaseComponent,
        // TextureGraphicComponent
        0x7B48_A9AE => todo!("TextureGraphicComponent"),
        // UICarousel
        0x8782_FE60 => todo!("UICarousel"),
        // UITextBox
        0xD10C_BEED => Component::UITextBox(parse_ui_text_box(reader, position, gp)?),
        // JD_UIWidgetGroupHUD_AutodanceRecorder
        0x9F87_350C => todo!("UIWidgetGroupHUDAutodanceRecorder"),
        // JD_UIWidgetGroupHUD_Lyrics
        0xF22C_9426 => todo!("UIWidgetGroupHUDLyrics"),
        // ViewportUIComponent
        0x6990_834C => todo!("ViewportUIComponent"),
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
        _ => {
            // 0x1528_D94A, 0x4866_6BB2, 0xA976_34C7
            return Err(ReadError::custom(format!(
                "Unknown component type: {component_type:x}!"
            )));
        }
    };

    Ok(component)
}

pub fn parse_credits_component<'de>(
    reader: &'de (impl ReadAtExt + ?Sized),
    position: &mut u64,
    gp: UniqueGameId,
) -> Result<CreditsComponent<'de>, ReadError> {
    let unk11 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk11, &[0xDu32, 0x17]).context(*position)?;
    let i = if gp.game == Game::JustDance2017 {
        6u32
    } else {
        10
    };
    for _ in 0..i {
        let unk12 = reader.read_at::<u32be>(position)?.into();
        test_any(
            &unk12,
            &[
                0x41C8_0000u32,
                0x41F0_0000,
                0x4220_0000,
                0x4248_0000,
                0x3F00_0000,
                0x3DCC_CCCD,
                0x4170_0000,
                0x4404_4000,
            ],
        )
        .context(*position)?;
    }
    let number_of_lines = usize::try_from(reader.read_at::<u32be>(position)?)?;
    let mut lines = Vec::with_capacity(number_of_lines);
    for _ in 0..number_of_lines {
        let line = reader.read_len_string_at::<u32be>(position)?;
        lines.push(line);
    }

    Ok(CreditsComponent { lines })
}

/// Parse the registration component of an actor
fn parse_registration_component(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    let unk11 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk11, &[0xAA55_B6BDu32, 0xFFFF_FFFF])?;
    let unk12 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk12, &0x0u32)?;
    Ok(())
}

/// Parse the box interpolator component of an actor
fn parse_box_interpolator_component(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    for _ in 0..2 {
        let unk11 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk11, &0xBF00_0000u32)?;
    }
    for _ in 0..2 {
        let unk12 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk12, &0x3F00_0000u32)?;
    }
    for _ in 0..2 {
        let unk13 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk13, &0xBF80_0000u32)?;
    }
    for _ in 0..2 {
        let unk14 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk14, &0x3F80_0000u32)?;
    }
    Ok(())
}

/// Parse the AFX post process component of an actor
fn parse_afx_post_process_component(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    let unk16 = reader.read_at::<u64be>(position)?.into();
    test_eq(&unk16, &8u64)?;
    let unk17 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk17, &1u32)?;
    Ok(())
}

/// Parse the converted tml tape component of an actor
fn parse_converted_tml_tape(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    let unk11 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk11, &0u32)?;
    Ok(())
}

/// Parse the fixed camera component of an actor
fn parse_fixed_camera_component(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    let unk11 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk11, &[0x0u32, 0x1])?;
    let unk12 = reader.read_at::<u64be>(position)?.into();
    test_eq(&unk12, &0u64)?;
    let unk13 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk13, &0x4120_0000u32)?;
    let unk14 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk14, &[0x0u32, 0x1])?;
    Ok(())
}

/// Parse the fx controller component of an actor
fn parse_fx_controller(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    let unk11 = reader.read_at::<u64be>(position)?.into();
    test_eq(&unk11, &0u64)?;
    Ok(())
}

/// Parse the fx bank component of an actor
fn parse_fx_bank_component(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    for _ in 0..4 {
        let unk11 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk11, &0x3F80_0000u32)?;
    }
    for _ in 0..3 {
        let unk12 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk12, &0u32)?;
    }
    let unk13 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk13, &[0x0u32, 0xFFFF_FFFF])?;
    let unk14 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk14, &0xFFFF_FFFFu32)?;
    Ok(())
}

/// Parse the bezier tree component of an actor
fn parse_bezier_tree_component(
    reader: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<(), ReadError> {
    let unk18 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk18, &2u32)?;
    for _ in 0..4 {
        let unk19 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk19, &0u32)?;
    }
    for _ in 0..2 {
        let unk20 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk20, &0x3F80_0000u32)?;
    }
    for _ in 0..2 {
        let unk21 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk21, &0u32)?;
    }
    let unk22 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk22, &0x4040_0000u32)?;
    for _ in 0..2 {
        let unk23 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk23, &0u32)?;
    }
    for _ in 0..2 {
        let unk24 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk24, &0x3F80_0000u32)?;
    }
    for _ in 0..3 {
        let unk25 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk25, &0u32)?;
    }
    let unk26 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk26, &1u32)?;
    Ok(())
}

/// Parse the material graphic component of an actor
fn parse_material_graphic_component<'de>(
    reader: &'de (impl ReadAtExt + ?Sized),
    position: &mut u64,
    gp: UniqueGameId,
    is_pleo: bool,
) -> Result<MaterialGraphicComponent<'de>, ReadError> {
    let game = gp.game;
    for _ in 0..3 {
        let unk11 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk11, &0x3F80_0000u32)?;
    }
    let unk11_5 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk11_5, &[0x3F80_0000u32, 0x0])?;

    for _ in 0..2 {
        let _unk12 = reader.read_at::<u64be>(position)?;
    }

    let unk13 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk13, &[0xFFFF_FFFFu32, 0x1])?;

    // <ENUM NAME="anchor" SEL="[0-9]" /> ?
    let unk14 = reader.read_at::<u64be>(position)?.into();
    test_ge(&unk14, &0u64).and(test_le(&unk14, &0x9u64))?;

    let unk15 = reader.read_at::<u64be>(position)?.into();
    test_any(
        &unk15,
        &[
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

    let _unk19 = reader.read_at::<u32be>(position)?;

    for item in files.iter_mut().skip(9) {
        let path = reader.read_at::<SplitPath>(position)?;
        *item = path;
    }

    if game == Game::JustDance2019 || game == Game::JustDance2018 || game == Game::JustDance2017 {
        let _unk20 = reader.read_at::<u64be>(position)?;
    } else {
        for _ in 0..4 {
            let _unk20 = reader.read_at::<u64be>(position)?;
        }

        let _unk20_5 = reader.read_at::<u32be>(position)?;

        let _unk21 = reader.read_at::<u32be>(position)?;
    }

    if game == Game::JustDance2020 {
        // Just Dance 2020 sometimes has a 0u32 inbetween
        let unk21_5 = u32::from(reader.read_at::<u32be>(position)?);
        if unk21_5 != 0 {
            *position -= 4;
        }
    }

    let unk22 = reader.read_at::<u64be>(position)?.into();
    test_eq(&unk22, &0xFFFF_FFFF_FFFF_FFFF)?;

    for _ in 0..3 {
        let _unk23 = reader.read_at::<u32be>(position)?;
    }

    let _unk24 = reader.read_at::<u32be>(position)?;

    let _unk25 = reader.read_at::<u64be>(position)?;

    // <ENUM NAME="oldAnchor" SEL="[0-9]" /> ?
    let unk26 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk26, &[0x1, 0x2, 0x3, 0x6, 0x9])?;

    if is_pleo {
        let unk27 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk27, &0x0u32)?;
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

// /// Parse the music track component of an actor
// fn parse_music_track_component<'a>(src: 'a '[u8], _game: Game, path: &Path, pos: &mut usize) -> Result<TemplateData<'a>, Error> {
//     let unk11 = reader.read_at::<u32be>(position)?.into();
//     test(
//         unk11 == 0x3,
//         anyhow!("unk11 is 0x{unk11:08x}"),
//     )?;
//     let unk12 = reader.read_at::<u32be>(position)?.into();
//     test(
//         unk12 == 0x40A1_5156 || unk12 == 0xFFFF_FFFF,
//         anyhow!("unk12 is 0x{unk12:08x}"),
//     )?;
//     let unk13 = read_string_at::<BigEndian>(src, pos)?;
//     let unk14 = reader.read_at::<u32be>(position)?.into();
//     test(
//         unk14 == 0x1,
//         anyhow!("unk14 is 0x{unk14:08x}"),
//     )?;
//     let unk15 = reader.read_at::<u32be>(position)?.into();
//     test(
//         unk15 == 0x4000_0000,
//         anyhow!("unk15 is 0x{unk15:08x}"),
//     )?;
//     for _ in 0..8 {
//         let unk16 = reader.read_at::<u32be>(position)?.into();
//         test(
//             unk16 == 0x1,
//             anyhow!("unk16 is 0x{unk16:08x}"),
//         )?;
//         let unk17 = src.read_u8_at(*offset)?;
//         test(
//             unk17 == 0x2d,
//             anyhow!("unk17 is 0x{unk17:08x}"),
//         )?;
//     }
//     let unk18 = src.read_u8_at(*offset)?;
//     test(
//         unk18 == 0x0,
//         anyhow!("unk18 is 0x{unk18:08x}"),
//     )?;
//     let unk19 = reader.read_at::<u32be>(position)?.into();
//     test(
//         unk19 == 0x4C55_6308,
//         anyhow!("unk19 is 0x{unk19:08x}"),
//     )?;
//     let unk20 = read_string_at::<BigEndian>(src, pos)?;
//     let unk21 = reader.read_at::<u32be>(position)?.into();
//     test(
//         unk21 == 0x2 || unk21 == 0x3,
//         anyhow!("unk21 is 0x{unk21:08x}"),
//     )?;
//     let unk22 = reader.read_at::<u32be>(position)?.into();
//     test(
//         unk22 == 0xD6F6_A73E,
//         anyhow!("unk22 is 0x{unk22:08x}"),
//     )?;
//     let unk23 = reader.read_at::<u32be>(position)?.into();
//     test(
//         unk23 == 0x4C55_6308 || unk23 == 0x2810_2F02,
//         anyhow!("unk23 is 0x{unk23:08x}"),
//     )?;
//     let unk24 = reader.read_at::<u32be>(position)?.into();
//     test(
//         unk24 == 0x0,
//         anyhow!("unk24 is 0x{unk24:08x}"),
//     )?;
//     let unk25 = read_string_at::<BigEndian>(src, pos)?;
//     let unk26 = read_string_at::<BigEndian>(src, pos)?;
//     todo!()
// }

impl<'de> BinaryDeserialize<'de> for PleoComponent<'de> {
    fn deserialize_at(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
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
    let unk11 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk11, &1u32)?;
    let unk12 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk12, &0u32)?;
    if gp.game != Game::JustDance2017 {
        let unk13 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk13, &0u32)?;
    }
    Ok(())
}

/// Parse the ui textbox component of an actor
fn parse_ui_text_box<'de>(
    reader: &'de (impl ReadAtExt + ?Sized),
    position: &mut u64,
    gp: UniqueGameId,
) -> Result<UITextBox<'de>, ReadError> {
    let game = gp.game;
    let unk11 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk11, &[0x0u32, 0x1, 0x2, 0x3])?;
    let unk12 = reader.read_at::<u32be>(position)?.into();
    test_any(
        &unk12,
        &[
            0xBF80_0000u32,
            0x41A0_0000,
            0x41F0_0000,
            0x4200_0000,
            0x4248_0000,
            0x428C_0000,
            0x42C8_0000,
            0x4316_0000,
        ],
    )?;
    let unk13 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk13, &0xFFFF_FFFFu32)?;
    let unk14 = reader.read_at::<u64be>(position)?.into();
    test_eq(&unk14, &0u64)?;
    for _ in 0..3 {
        let unk15 = reader.read_at::<u32be>(position)?.into();
        test_any(&unk15, &[0x0u32, 0x3F80_0000])?;
    }
    for _ in 0..2 {
        let unk17 = reader.read_at::<u32be>(position)?.into();
        test_any(&unk17, &[0x4348_0000, 0x4496_0000u32, 0xBF80_0000])?;
    }
    let unk18 = reader.read_at::<u32be>(position)?.into();
    test_any(
        &unk18,
        &[0x4348_0000u32, 0x443B_8000, 0x458C_A000, 0xBF80_0000],
    )?;
    let unk19 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk19, &0xBF80_0000u32)?;
    let string1 = reader.read_len_string_at::<u32be>(position)?;
    let string1 = if string1.is_empty() {
        None
    } else {
        Some(string1)
    };
    let unk20 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk20, &0u32)?;
    let unk21 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk21, &1u32)?;
    let unk22 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk22, &[0xFFFF_FFFFu32, 0x317A, 0x1B7C, 0x3B])?;
    let unk23_1 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk23_1, &[0x0u32, 0x4140_0000, 0xC170_0000, 0xC120_0000])?;
    let unk23_2 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk23_2, &0u32)?;
    let unk23_3 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk23_3, &0u32)?;
    let unk23_4 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk23_4, &0u32)?;
    let string2 = reader.read_len_string_at::<u32be>(position)?;
    let string2 = if string2.is_empty() {
        None
    } else {
        Some(string2)
    };
    let unk23_6 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk23_6, &0u32)?;
    let unk23_7 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk23_7, &0x0u32)?;
    let unk23_8 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk23_8, &[0u32, 0x2, 0xFFFF_FFFF])?;
    let unk24 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk24, &[0u32, 0xFFFF_FFFF])?;
    let unk25 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk25, &[0u32, 0xBF80_0000])?;
    let unk26 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk26, &[0u32, 0xFFFF_FFFF])?;
    let unk27 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk27, &[0u32, 0xBF80_0000])?;
    let i = if game == Game::JustDance2018 || game == Game::JustDance2017 {
        6
    } else {
        7
    };
    for _ in 0..i {
        let unk28 = reader.read_at::<u64be>(position)?.into();
        test_eq(&unk28, &0u64)?;
    }
    let unk29 = reader.read_at::<u32be>(position)?.into();
    test_eq(&unk29, &0xBF80_0000u32)?;
    if game == Game::JustDance2019 || game == Game::JustDance2018 || game == Game::JustDance2017 {
        let unk30 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk30, &0u32)?;
    } else {
        let unk30 = reader.read_at::<u64be>(position)?.into();
        test_eq(&unk30, &0u64)?;
    }
    let unk31 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk31, &[0xFFFF_FFFFu32, 0x1])?;
    let unk32 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk32, &[0xFFFF_FFFFu32, 0x1])?;
    let unk33 = reader.read_at::<u32be>(position)?.into();
    test_any(&unk33, &[1u32, 4, 0xFFFF_FFFF])?;
    Ok(UITextBox { string1, string2 })
}
