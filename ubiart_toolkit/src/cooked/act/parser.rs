//! Contains the parser implementation

use std::borrow::Cow;

use anyhow::{anyhow, Context, Error};
use byteorder::BigEndian;

use crate::utils::{
    bytes::{read_path_at, read_string_at, read_u32_at, read_u64_at},
    Game, SplitPath,
};
use dotstar_toolkit_utils::testing::{test, test_any};

use super::{
    Actor, CreditsComponent, MaterialGraphicComponent, PleoComponent, Component, ComponentData,
    ComponentType,
};

/// Parse a bytearray-like source as a actor file
///
/// This will parse the source from start to end.
///
/// # Errors
/// This function will error when it encounters the following:
/// - Unexpected values (i.e. wrong magic)
/// - Invalid UTF-8 (i.e. in paths)
/// - Source has an unexpected size (i.e. not enough bytes, or too many bytes)
/// - If there are too many templates
pub fn parse(src: &[u8], game: Game) -> Result<Actor, anyhow::Error> {
    // Keep track of where we are
    let mut pos = 0;
    let unk0 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test(&unk0, &1)?;
    let unk1 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test_any(
        &unk1,
        &[
            0x0,
            0x3D23_D70A,
            0x3DCC_CCCD,
            0x3F66_6C4C,
            0x3F80_0000,
            0x4000_0000,
        ],
    )?;
    let unk2 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test_any(
        &unk2,
        &[
            0x3F00_0000,
            0x3F80_0000,
            0x4240_0000,
            0x4320_0000,
            0x4420_0000,
            0x4422_8000,
        ],
    )?;
    let unk2_5 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test_any(
        &unk2_5,
        &[
            0x3F00_0000,
            0x3F80_0000,
            0x4120_0000,
            0x4240_0000,
            0x4320_0000,
        ],
    )?;
    let unk3 = read_u64_at::<BigEndian>(src, &mut pos)?;
    test(&unk3, &0)?;
    let unk3_5 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test(&unk3_5, &0)?;
    match game {
        Game::JustDance2022 | Game::JustDance2021 | Game::JustDance2020 | Game::JustDance2019 => {
            let unk4 = read_u64_at::<BigEndian>(src, &mut pos)?;
            test(&unk4, &0x1_0000_0000)?;
        }
        _ => {
            let unk4 = read_u32_at::<BigEndian>(src, &mut pos)?;
            test_any(&unk4, &[0x1, 0x0])?;
            if unk4 == 0x1 {
                let unk4_5 = read_u32_at::<BigEndian>(src, &mut pos)?;
                test(&unk4_5, &0)?;
            }
        }
    };
    let unk5 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test(&unk5, &0)?;
    let unk6 = read_u64_at::<BigEndian>(src, &mut pos)?;
    test(&unk6, &0)?;
    let unk7 = read_u64_at::<BigEndian>(src, &mut pos)?;
    test(&unk7, &0xFFFF_FFFF)?;
    let unk8 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test(&unk8, &0)?;

    let tpl = read_path_at::<BigEndian>(src, &mut pos)?;
    test(&tpl.is_empty(), &false)?;
    let unk9 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test(&unk9, &0)?;
    let actor_amount = read_u32_at::<BigEndian>(src, &mut pos)?;

    let mut components = Vec::with_capacity(actor_amount.try_into()?);
    for _ in 0..actor_amount {
        // String id of the class name of the template without the '_Template' but including 'JD_' if it is in the class name
        let component_type_encoded = read_u32_at::<BigEndian>(src, &mut pos)?;

        let component_type = ComponentType::try_from(component_type_encoded)?;

        let component_data = match component_type {
            ComponentType::AutodanceComponent
            | ComponentType::MasterTape
            | ComponentType::SongDatabaseComponent
            | ComponentType::SongDescComponent
            | ComponentType::TapeCaseComponent
            | ComponentType::AvatarDescComponent
            | ComponentType::SkinDescComponent => ComponentData::None,
            ComponentType::BoxInterpolatorComponent => {
                parse_box_interpolator_component(src, game, &mut pos)?
            }
            ComponentType::ConvertedTmlTapeComponent => {
                parse_converted_tml_tape(src, game, &mut pos)?
            }
            ComponentType::CreditsComponent => parse_credits_component(src, game, &mut pos)?,
            ComponentType::FixedCameraComponent => {
                parse_fixed_camera_component(src, game, &mut pos)?
            }
            ComponentType::FXControllerComponent => parse_fx_controller(src, game, &mut pos)?,
            ComponentType::MaterialGraphicComponent => {
                parse_material_graphic_component(src, game, &mut pos, false)?
            }
            ComponentType::PleoTextureGraphicComponent => {
                parse_material_graphic_component(src, game, &mut pos, true)?
            }
            ComponentType::PleoComponent => parse_pleo_component(src, game, &mut pos)?,
            ComponentType::PropertyPatcher => parse_property_patcher(src, game, &mut pos)?,
            ComponentType::RegistrationComponent => {
                parse_registration_component(src, game, &mut pos)?
            }
            ComponentType::UITextBox => parse_ui_text_box(src, game, &mut pos)?,
            ComponentType::FxBankComponent => parse_fx_bank_component(src, game, &mut pos)?,
            ComponentType::BezierTreeComponent => parse_bezier_tree_component(src, game, &mut pos)?,
            ComponentType::AFXPostProcessComponent => {
                parse_afx_post_process_component(src, game, &mut pos)?
            }
            // TemplateType::TapeCase => parse_tape_case(src, game, &mut pos)?,
            // TemplateType::MusicTrackComponent => parse_music_track_component(src, game, path, &mut pos)?,
            ComponentType::BeatPulseComponent => todo!(),
            ComponentType::CameraGraphicComponent => todo!(),
            ComponentType::ClearColorComponent => todo!(),
            ComponentType::PictoComponent => todo!(),
            ComponentType::SingleInstanceMesh3DComponent => todo!(),
            ComponentType::SoundComponent => todo!(),
            ComponentType::UICarousel => todo!(),
            ComponentType::UIWdigetGroupHUDAutodanceRecorder => todo!(),
            ComponentType::UIWidgetGroupHUDLyrics => todo!(),
            ComponentType::ViewportUIComponent => todo!(),
            _ => {
                return Err(anyhow!("Unsupported component type {component_type:?}!"));
            }
        };
        components.push(Component {
            the_type: component_type,
            data: component_data,
        });
    }

    test(&pos, &src.len())?;

    Ok(Actor {
        tpl,
        unk1,
        unk2,
        unk2_5,
        components,
    })
}

fn parse_registration_component<'a>(
    src: &'a [u8],
    _game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    // if game == Game::JustDance2018 {
    let unk11 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk11, &[0xAA55_B6BD, 0xFFFF_FFFF])?;
    let unk12 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk12, &0x0)?;
    // }
    Ok(ComponentData::None)
}

fn parse_box_interpolator_component<'a>(
    src: &'a [u8],
    _game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    for _ in 0..2 {
        let unk11 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk11, &0xBF00_0000)?;
    }
    for _ in 0..2 {
        let unk12 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk12, &0x3F00_0000)?;
    }
    for _ in 0..2 {
        let unk13 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk13, &0xBF80_0000)?;
    }
    for _ in 0..2 {
        let unk14 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk14, &0x3F80_0000)?;
    }
    Ok(ComponentData::None)
}

fn parse_afx_post_process_component<'a>(
    src: &'a [u8],
    _game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    let unk16 = read_u64_at::<BigEndian>(src, pos)?;
    test(&unk16, &8)?;
    let unk17 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk17, &1)?;
    Ok(ComponentData::None)
}

fn parse_converted_tml_tape<'a>(
    src: &'a [u8],
    _game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    let unk11 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk11, &0)?;
    Ok(ComponentData::None)
}

fn parse_credits_component<'a>(
    src: &'a [u8],
    game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    let unk11 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk11, &[0xD, 0x17]).context(*pos)?;
    let i = if game == Game::JustDance2017 { 6 } else { 10 };
    for _ in 0..i {
        let unk12 = read_u32_at::<BigEndian>(src, pos)?;
        test_any(
            &unk12,
            &[
                0x41C8_0000,
                0x41F0_0000,
                0x4220_0000,
                0x4248_0000,
                0x3F00_0000,
                0x3DCC_CCCD,
                0x4170_0000,
                0x4404_4000,
            ],
        )
        .context(*pos)?;
    }
    let number_of_lines = read_u32_at::<BigEndian>(src, pos)?;
    let mut lines = Vec::with_capacity(number_of_lines.try_into()?);
    for _ in 0..number_of_lines {
        let line = Cow::Borrowed(read_string_at::<BigEndian>(src, pos)?);
        lines.push(line);
    }

    Ok(ComponentData::CreditsComponent(CreditsComponent { lines }))
}

fn parse_fixed_camera_component<'a>(
    src: &'a [u8],
    _game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    let unk11 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk11, &[0x0, 0x1])?;
    let unk12 = read_u64_at::<BigEndian>(src, pos)?;
    test(&unk12, &0)?;
    let unk13 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk13, &0x4120_0000)?;
    let unk14 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk14, &[0x0, 0x1])?;
    Ok(ComponentData::None)
}

fn parse_fx_controller<'a>(
    src: &'a [u8],
    _game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    let unk11 = read_u64_at::<BigEndian>(src, pos)?;
    test(&unk11, &0)?;
    Ok(ComponentData::None)
}

fn parse_fx_bank_component<'a>(
    src: &'a [u8],
    _game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    for _ in 0..4 {
        let unk11 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk11, &0x3F80_0000)?;
    }
    for _ in 0..3 {
        let unk12 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk12, &0)?;
    }
    let unk13 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk13, &[0x0, 0xFFFF_FFFF])?;
    let unk14 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk14, &0xFFFF_FFFF)?;
    Ok(ComponentData::None)
}

fn parse_bezier_tree_component<'a>(
    src: &'a [u8],
    _game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    let unk18 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk18, &2)?;
    for _ in 0..4 {
        let unk19 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk19, &0)?;
    }
    for _ in 0..2 {
        let unk20 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk20, &0x3F80_0000)?;
    }
    for _ in 0..2 {
        let unk21 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk21, &0)?;
    }
    let unk22 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk22, &0x4040_0000)?;
    for _ in 0..2 {
        let unk23 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk23, &0)?;
    }
    for _ in 0..2 {
        let unk24 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk24, &0x3F80_0000)?;
    }
    for _ in 0..3 {
        let unk25 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk25, &0)?;
    }
    let unk26 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk26, &1)?;
    Ok(ComponentData::None)
}

fn parse_material_graphic_component<'a>(
    src: &'a [u8],
    game: Game,
    pos: &mut usize,
    is_pleo: bool,
) -> Result<ComponentData<'a>, Error> {
    for _ in 0..3 {
        let unk11 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk11, &0x3F80_0000).with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;
    }
    let unk11_5 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk11_5, &[0x3F80_0000, 0x0])
        .with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;

    for _ in 0..2 {
        let unk12 = read_u64_at::<BigEndian>(src, pos)?;
        test_any(&unk12, &[0x0, 0xFFFF_FFFF])
            .with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;
    }

    let unk13 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk13, &[0xFFFF_FFFF, 0x1])
        .with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;

    // <ENUM NAME="anchor" SEL="[0-9]" /> ?
    let unk14 = read_u64_at::<BigEndian>(src, pos)?;
    test_any(&unk14, &[0x1, 0x2, 0x3, 0x6, 0x9])
        .with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;

    let unk15 = read_u64_at::<BigEndian>(src, pos)?;
    test_any(
        &unk15,
        &[
            0x0,
            0x3E2E_147B,
            0xC080_0000,
            0x3E99_999A_BDCC_CCCD,
            0xBDE1_47AE_3E61_47AE,
        ],
    )
    .with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;

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
        let path = read_path_at::<BigEndian>(src, pos)
            .with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;
        *item = path;
    }

    let unk19 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk19, &0x0).with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;

    for item in files.iter_mut().skip(9) {
        let path = read_path_at::<BigEndian>(src, pos)
            .with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;
        *item = path;
    }

    if game == Game::JustDance2019 || game == Game::JustDance2018 || game == Game::JustDance2017 {
        let unk20 = read_u64_at::<BigEndian>(src, pos)?;
        test(&unk20, &0x0).with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;
    } else {
        for _ in 0..4 {
            let unk20 = read_u64_at::<BigEndian>(src, pos)?;
            test(&unk20, &0x0).with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;
        }

        let unk20_5 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk20_5, &0x0).with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;

        let unk21 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk21, &0x3F80_0000).with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;
    }

    if game == Game::JustDance2020 {
        // Just Dance 2020 sometimes has a 0u32 inbetween
        let unk21_5 = read_u32_at::<BigEndian>(src, pos)?;
        if unk21_5 != 0 {
            *pos -= 4;
        }
    }

    let unk22 = read_u64_at::<BigEndian>(src, pos)?;
    test(&unk22, &0xFFFF_FFFF_FFFF_FFFF)
        .with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;

    for _ in 0..3 {
        let unk23 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk23, &0x0).with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;
    }

    let unk24 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk24, &0x3F80_0000).with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;

    let unk25 = read_u64_at::<BigEndian>(src, pos)?;
    test(&unk25, &0x0).with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;

    // <ENUM NAME="oldAnchor" SEL="[0-9]" /> ?
    let unk26 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk26, &[0x1, 0x2, 0x3, 0x6, 0x9])
        .with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;

    if is_pleo {
        let unk27 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk27, &0x0).with_context(|| format!("Pos: {pos}, is_pleo: {is_pleo}"))?;
    }

    Ok(ComponentData::MaterialGraphicComponent(Box::new(
        MaterialGraphicComponent {
            files,
            unk11_5,
            unk13,
            unk14,
            unk15,
            unk26,
        },
    )))
}

// fn parse_music_track_component<'a>(src: 'a '[u8], _game: Game, path: &Path, pos: &mut usize) -> Result<TemplateData<'a>, Error> {
//     let unk11 = read_u32_at::<BigEndian>(src, pos)?;
//     test(
//         unk11 == 0x3,
//         anyhow!("unk11 is 0x{unk11:08x}"),
//     )?;
//     let unk12 = read_u32_at::<BigEndian>(src, pos)?;
//     test(
//         unk12 == 0x40A1_5156 || unk12 == 0xFFFF_FFFF,
//         anyhow!("unk12 is 0x{unk12:08x}"),
//     )?;
//     let unk13 = read_string_at::<BigEndian>(src, pos)?;
//     let unk14 = read_u32_at::<BigEndian>(src, pos)?;
//     test(
//         unk14 == 0x1,
//         anyhow!("unk14 is 0x{unk14:08x}"),
//     )?;
//     let unk15 = read_u32_at::<BigEndian>(src, pos)?;
//     test(
//         unk15 == 0x4000_0000,
//         anyhow!("unk15 is 0x{unk15:08x}"),
//     )?;
//     for _ in 0..8 {
//         let unk16 = read_u32_at::<BigEndian>(src, pos)?;
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
//     let unk19 = read_u32_at::<BigEndian>(src, pos)?;
//     test(
//         unk19 == 0x4C55_6308,
//         anyhow!("unk19 is 0x{unk19:08x}"),
//     )?;
//     let unk20 = read_string_at::<BigEndian>(src, pos)?;
//     let unk21 = read_u32_at::<BigEndian>(src, pos)?;
//     test(
//         unk21 == 0x2 || unk21 == 0x3,
//         anyhow!("unk21 is 0x{unk21:08x}"),
//     )?;
//     let unk22 = read_u32_at::<BigEndian>(src, pos)?;
//     test(
//         unk22 == 0xD6F6_A73E,
//         anyhow!("unk22 is 0x{unk22:08x}"),
//     )?;
//     let unk23 = read_u32_at::<BigEndian>(src, pos)?;
//     test(
//         unk23 == 0x4C55_6308 || unk23 == 0x2810_2F02,
//         anyhow!("unk23 is 0x{unk23:08x}"),
//     )?;
//     let unk24 = read_u32_at::<BigEndian>(src, pos)?;
//     test(
//         unk24 == 0x0,
//         anyhow!("unk24 is 0x{unk24:08x}"),
//     )?;
//     let unk25 = read_string_at::<BigEndian>(src, pos)?;
//     let unk26 = read_string_at::<BigEndian>(src, pos)?;
//     todo!()
// }

fn parse_pleo_component<'a>(
    src: &'a [u8],
    _game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    let video = read_path_at::<BigEndian>(src, pos)?;
    let dash_mpd = read_path_at::<BigEndian>(src, pos)?;
    let channel_id = Cow::Borrowed(read_string_at::<BigEndian>(src, pos)?);
    let channel_id = if channel_id.is_empty() { None } else { Some(channel_id) };
    Ok(ComponentData::PleoComponent(PleoComponent {
        video,
        dash_mpd,
        channel_id,
    }))
}

fn parse_property_patcher<'a>(
    src: &'a [u8],
    game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    let unk11 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk11, &1)?;
    let unk12 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk12, &0)?;
    if game != Game::JustDance2017 {
        let unk13 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk13, &0)?;
    }
    Ok(ComponentData::None)
}

fn parse_ui_text_box<'a>(
    src: &'a [u8],
    game: Game,
    pos: &mut usize,
) -> Result<ComponentData<'a>, Error> {
    let unk11 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk11, &[0x0, 0x2, 0x3])?;
    let unk12 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(
        &unk12,
        &[
            0xbf80_0000,
            0x41a0_0000,
            0x4200_0000,
            0x428C_0000,
            0x42C8_0000,
            0x4316_0000,
        ],
    )?;
    let unk13 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk13, &0xffff_ffff)?;
    let unk14 = read_u64_at::<BigEndian>(src, pos)?;
    test(&unk14, &0)?;
    for _ in 0..3 {
        let unk15 = read_u32_at::<BigEndian>(src, pos)?;
        test_any(&unk15, &[0x0, 0x3f80_0000])?;
    }
    for _ in 0..2 {
        let unk17 = read_u32_at::<BigEndian>(src, pos)?;
        test_any(&unk17, &[0x4496_0000, 0xbf80_0000])?;
    }
    let unk18 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk18, &[0xbf80_0000, 0x443b_8000, 0x458C_A000]).context(*pos)?;
    let unk19 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk19, &0xbf80_0000).context(*pos)?;
    let string1 = Cow::Borrowed(read_string_at::<BigEndian>(src, pos).context(*pos)?);
    let string1 = if string1.is_empty() {
        None
    } else {
        Some(string1)
    };
    let unk20 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk20, &0).context(*pos)?;
    let unk21 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk21, &1).context(*pos)?;
    let unk22 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk22, &[0xffff_ffff, 0x317a, 0x3b]).context(*pos)?;
    let unk23_1 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk23_1, &[0x0, 0x4140_0000, 0xC170_0000, 0xC120_0000]).context(*pos)?;
    let unk23_2 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk23_2, &0).context(*pos)?;
    let unk23_3 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk23_3, &0).context(*pos)?;
    let unk23_4 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk23_4, &0).context(*pos)?;
    let string2 = Cow::Borrowed(read_string_at::<BigEndian>(src, pos).context(*pos)?);
    let string2 = if string2.is_empty() {
        None
    } else {
        Some(string2)
    };
    let unk23_6 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk23_6, &0).context(*pos)?;
    let unk23_7 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk23_7, &0x0).context(*pos)?;
    let unk23_8 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk23_8, &[0, 0x2, 0xFFFF_FFFF]).context(*pos)?;
    let unk24 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk24, &[0, 0xFFFF_FFFF]).context(*pos)?;
    let unk25 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk25, &[0, 0xBF80_0000]).context(*pos)?;
    let unk26 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk26, &[0, 0xFFFF_FFFF]).context(*pos)?;
    let unk27 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk27, &[0, 0xbf80_0000]).context(*pos)?;
    let i = if game == Game::JustDance2018 || game == Game::JustDance2017 {
        6
    } else {
        7
    };
    for _ in 0..i {
        let unk28 = read_u64_at::<BigEndian>(src, pos)?;
        test(&unk28, &0).context(*pos)?;
    }
    let unk29 = read_u32_at::<BigEndian>(src, pos)?;
    test(&unk29, &0xbf80_0000).context(*pos)?;
    if game == Game::JustDance2019 || game == Game::JustDance2018 || game == Game::JustDance2017 {
        let unk30 = read_u32_at::<BigEndian>(src, pos)?;
        test(&unk30, &0).context(*pos)?;
    } else {
        let unk30 = read_u64_at::<BigEndian>(src, pos)?;
        test(&unk30, &0).context(*pos)?;
    }
    let unk31 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk31, &[0xffff_ffff, 0x1]).context(*pos)?;
    let unk32 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk32, &[0xffff_ffff, 0x1]).context(*pos)?;
    let unk33 = read_u32_at::<BigEndian>(src, pos)?;
    test_any(&unk33, &[1, 4, 0xFFFF_FFFF]).context(*pos)?;
    Ok(ComponentData::UITextBox(super::UITextBox {
        string1,
        string2,
    }))
}
