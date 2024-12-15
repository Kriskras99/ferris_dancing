pub mod json;
pub mod path;
pub mod plumbing;

// TODO: Remove pub use and replace uses with utils::path::
use std::{cmp::Ordering, ffi::OsStr, fmt::Display};

use clap::ValueEnum;
use dotstar_toolkit_utils::bytes::{
    primitives::{u32be, u32le},
    read::{BinaryDeserialize, ReadAtExt, ReadError},
    write::{BinarySerialize, WriteAt, WriteError},
};
pub use path::{PathId, SplitPath};
use serde::{Deserialize, Serialize};
use tracing::warn;
use ubiart_toolkit_shared_types::errors::ParserError;
pub use ubiart_toolkit_shared_types::{errors, Color, LocaleId};

pub struct InternedString;
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
            0x0288_3A7E => Ok("MusicTrackComponent_Template"),
            0x0579_E81B => Ok("PleoTextureGraphicComponent"),
            0x09C2_6AAB => Ok("CameraGraphicComponent_Template"),
            0x0C73_6497 => Ok("MasterTape_Template"),
            0x0E03_0DFB => Ok("JD_UIWidgetGroupHUD_Lyrics_Template"),
            0x0E35_5C68 => Ok("FXControllerComponent_Template"),
            0x1263_DAD9 => Ok("PleoComponent"),
            0x1445_31FF => Ok("navigation_default"),
            0x1576_B015 => Ok("navigation_row"),
            0x1642_9EB7 => Ok("ViewportUIComponent_Template"),
            0x1720_AB75 => Ok("stars_raceline_wdf"),
            0x1759_E29D => Ok("JD_AvatarDescComponent"),
            0x18AF_6D8D => Ok("avatar"),
            0x1A7E_999A => Ok("Mesh3DComponent"),
            0x1B85_7BCE => Ok("Actor_Template"),
            0x231F_27DE => Ok("TapeCase_Component"),
            0x24A3_7BF0 => Ok("TML_Motion"),
            0x24B7_AF0C => Ok("JD_PictoComponent_Template"),
            0x2520_774E => Ok("stamppageunlock"),
            0x27C6_D339 => Ok("JD_AvatarDescTemplate"),
            0x2810_2F02 => Ok("navigation_speed"),
            0x2949_932E => Ok("PleoTextureGraphicComponent_Template"),
            0x2C9C_B4F0 => Ok("JD_FixedCameraComponent_Template"),
            0x31D3_B347 => Ok("lyrics"),
            0x36F0_1B59 => Ok("JD_AsyncPlayerDesc_Template"),
            0x38F4_A3D5 => Ok("JD_UIWidgetElement_Template"),
            0x3D5D_EBA2 => Ok("JD_FixedCameraComponent"),
            0x3ED0_2533 => Ok("JD_BeatPulseComponent_Template"),
            0x4055_79FB => Ok("JD_SongDatabaseComponent"),
            0x40A1_5156 => Ok("menu_valid"),
            0x4166_F23C => Ok("snap"),
            0x418F_AF9A => Ok("menu_lstick_right"),
            0x4491_5DC4 => Ok("crowd"),
            0x4A24_BAD3 => Ok("JD_CMU_GenericStage_Component_Template"),
            0x4B04_9017 => Ok("ui_rollover_button"),
            0x4C55_6308 => Ok("navigation"),
            0x4E8C_DF75 => Ok("SingleInstanceMesh3DComponent_Template"),
            0x4FA4_0F09 => Ok("SubSceneActor"),
            0x51EA_2CD0 => Ok("JD_AutodanceComponent_Template"),
            0x5632_1EA5 => Ok("JD_GoldMoveComponent"),
            0x5A20_D4B1 => Ok("video_autodance"),
            0x5B64_8E44 => Ok("JD_BlockFlowTemplate"),
            0x64EC_D957 => Ok("JD_SongDatabaseTemplate"),
            0x64ED_9E36 => Ok("TexturePatcherComponent_Template"),
            0x6696_D39A => Ok("video"),
            0x677B_269B => Ok("MasterTape"),
            0x67B8_BB77 => Ok("JD_AutodanceComponent"),
            0x68ED_319A => Ok("Mesh3DComponent_Template"),
            0x693E_C811 => Ok("JD_UIWidgetGroupHUD_AutodanceRecorder_Template"),
            0x6DC2_DBB2 => Ok("menu_phone_right"),
            0x6F32_8BC1 => Ok("TexturePatcherComponent"),
            0x6F40_37D0 => Ok("coverflow"),
            0x7233_490C => Ok("menu_dpad_right"),
            0x72B6_1FC5 => Ok("MaterialGraphicComponent"),
            0x7411_331E => Ok("navigation_age"),
            0x7A7C_235B => Ok("MusicTrackComponent"),
            0x7C0C_114C => Ok("BezierTreeComponent_Template"),
            0x7DD8_643C => Ok("SoundComponent"),
            0x7F7A_3028 => Ok("UITextBox_Template"),
            0x8229_ABC3 => Ok("TapeCase_Template"),
            0x83B2_58E1 => Ok("gotodefault"),
            0x84B1_0A9F => Ok("ConvertedTmlTape_Template"),
            0x8AC2_B5C6 => Ok("JD_SongDescTemplate"),
            0x8D4F_FFB6 => Ok("FxControllerComponent"),
            0x8DA9_E375 => Ok("JD_BlockFlowComponent"),
            0x8E09_B64A => Ok("navigation_kids"),
            0x8F54_5995 => Ok("JD_RegistrationComponent_Template"),
            0x97CA_628B => Ok("Actor"),
            0x9A0A_4843 => Ok("PleoComponent_Template"),
            0x9CAE_4325 => Ok("TextureGraphicComponent_Template"),
            0x9CCE_B199 => Ok("checkbox"),
            0x9CD9_0BCB => Ok("theme"),
            0x9DF8_8CAE => Ok("drc"),
            0x9F87_350C => Ok("JD_UIWidgetGroupHUD_AutodanceRecorder"),
            0xA355_7351 => Err(ReadError::custom(
                "Unkown component with animation/.*.anm files".into(),
            )),
            0xA58D_BCC2 => Ok("JD_TransitionSceneConfig"),
            0xA9B9_1515 => Ok("JD_ChannelZappingComponent_Template"),
            0xAA55_B6BD => Ok("asyncplayervideo"),
            0xAA5B_5DAD => Ok("ClearColorComponent_Template"),
            0xAB6E_1718 => Ok("wdf_crowdloop"),
            0xABF3_773E => Ok("master"),
            0xAD1A_4447 => Ok("partymaster_coach"),
            0xAEBB_218B => Ok("ClearColorComponent"),
            0xAEC9_B9AE => Ok("JD_UIWidgetGroupHUD_Template"),
            0xB11F_C1B6 => Ok("prelobby"),
            0xB20E_35D5 => Ok("navigation_big_items"),
            0xBA69_7320 => Ok("JD_GoldMoveComponent_Template"),
            0xBEA8_2EB8 => Ok("JD_CreditsComponent_Template"),
            0xC316_BF34 => Ok("JD_PictoComponent"),
            0xC33B_4C02 => Ok("menu_phone_left"),
            0xC738_9490 => Ok("ui"),
            0xCD07_BB76 => Ok("ConvertedTmlTape_Component"),
            0xCE01_8EDB => Ok("JD_MapSceneConfig"),
            0xCE44_8441 => Ok("hud"),
            0xD10C_BEED => Ok("UITextBox"),
            0xD5B5_4597 => Ok("TML_Sequence"),
            0xD64E_0E2A => Ok("menu_dpad_left"),
            0xD94D_6C53 => Ok("SoundComponent_Template"),
            0xD9B1_E95C => Ok("menu_lstick_left"),
            0xDFEF_DBFB => Ok("decel"),
            0xE07F_CC3F => Ok("JD_SongDescComponent"),
            0xE0A2_4B6D => Ok("JD_RegistrationComponent"),
            0xE41C_9FC6 => Ok("UIItemTextField_Template"),
            0xE628_44B8 => Ok("JD_UIBannerSceneConfig"),
            0xEB53_7A60 => Ok("AMB"),
            0xF22C_9426 => Ok("JD_UIWidgetGroupHUD_Lyrics"),
            0xF878_DC2D => Ok("JD_SongDatabaseSceneConfig"),
            0xF9BE_082F => Ok("MaterialGraphicComponent_Template"),
            0xFA35_7DA1 => Ok("wiimote"),
            0xFD45_47AC => Ok("TML_Karaoke"),
            0xFFFF_FFFF => Ok(""),
            _ => Err(ReadError::custom(format!(
                "Unknown interned string id: 0x{string_id:08x}"
            ))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniqueGameId {
    pub game: Game,
    pub platform: Platform,
    pub id: u32,
}

impl UniqueGameId {
    pub const WIIU2015: Self = Self {
        game: Game::JustDance2015,
        platform: Platform::WiiU,
        id: 0xC563_9F58,
    };
    pub const WIIU2016: Self = Self {
        game: Game::JustDance2016,
        platform: Platform::WiiU,
        id: 0xF9D9_B22B,
    };
    pub const WIIU2017: Self = Self {
        game: Game::JustDance2017,
        platform: Platform::WiiU,
        id: 0x04A2_5379,
    };
    pub const WIN2017: Self = Self {
        game: Game::JustDance2017,
        platform: Platform::Win,
        id: 0x1D3A_4C30,
    };
    pub const NX2017: Self = Self {
        game: Game::JustDance2017,
        platform: Platform::Nx,
        id: 0x32F3_512A,
    };
    pub const NX2018: Self = Self {
        game: Game::JustDance2018,
        platform: Platform::Nx,
        id: 0x032E_71C5,
    };
    pub const NX2019V1: Self = Self {
        game: Game::JustDance2019,
        platform: Platform::Nx,
        id: 0x57A7_053C,
    };
    pub const NX2019V2: Self = Self {
        game: Game::JustDance2019,
        platform: Platform::Nx,
        id: 0xC781_A65B,
    };
    pub const NX2020: Self = Self {
        game: Game::JustDance2020,
        platform: Platform::Nx,
        id: 0x217A_94CE,
    };
    pub const NX_CHINA: Self = Self {
        game: Game::JustDanceChina,
        platform: Platform::Nx,
        id: 0xA155_8F87,
    };
    pub const NX2021: Self = Self {
        game: Game::JustDance2021,
        platform: Platform::Nx,
        id: 0xA4F0_18EE,
    };
    pub const NX2022: Self = Self {
        game: Game::JustDance2022,
        platform: Platform::Nx,
        id: 0x1DDB_2268,
    };
}

impl PartialOrd for UniqueGameId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Order by game first, if games are the same order by ID
        // A newer ID means a newer version of that game
        self.game
            .partial_cmp(&other.game)
            .map(|order| order.then(self.id.cmp(&other.id)))
    }
}

impl Display for UniqueGameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{} for {}", self.game, self.platform)
    }
}

impl From<UniqueGameId> for u32 {
    fn from(value: UniqueGameId) -> Self {
        value.id
    }
}

impl TryFrom<u32> for UniqueGameId {
    type Error = ParserError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1C24_B91A => Ok(Self {
                game: Game::JustDance2014,
                platform: Platform::WiiU,
                id: value,
            }),
            0xC563_9F58 => Ok(Self {
                game: Game::JustDance2015,
                platform: Platform::WiiU,
                id: value,
            }),
            // Base       Update 1      Update 2
            0xDA14_5C61 | 0x8C9D_65E4 | 0xF9D_9B22B => Ok(Self {
                game: Game::JustDance2016,
                platform: Platform::WiiU,
                id: value,
            }),
            0x04A2_5379 => Ok(Self {
                game: Game::JustDance2017,
                platform: Platform::WiiU,
                id: value,
            }),
            0x1D3A_4C30 => Ok(Self {
                game: Game::JustDance2017,
                platform: Platform::Win,
                id: value,
            }),
            0x415E_6D8C | 0x32F3_512A => Ok(Self {
                game: Game::JustDance2017,
                platform: Platform::Nx,
                id: value,
            }),
            0x032E_71C5 => Ok(Self {
                game: Game::JustDance2018,
                platform: Platform::Nx,
                id: value,
            }),
            0x1F5E_E42F | 0xC781_A65B | 0x57A7_053C => Ok(Self {
                game: Game::JustDance2019,
                platform: Platform::Nx,
                id: value,
            }),
            0x72B4_2FF4 | 0xB292_FD08 | 0x217A_94CE => Ok(Self {
                game: Game::JustDance2020,
                platform: Platform::Nx,
                id: value,
            }),
            0xA155_8F87 => Ok(Self {
                game: Game::JustDanceChina,
                platform: Platform::Nx,
                id: value,
            }),
            0x4C8E_C5C5 => Ok(Self {
                game: Game::JustDance2020,
                platform: Platform::Wii,
                id: value,
            }),
            0xEB5D_504C | 0xA4F0_18EE => Ok(Self {
                game: Game::JustDance2021,
                platform: Platform::Nx,
                id: value,
            }),
            0x1DDB_2268 => Ok(Self {
                game: Game::JustDance2022,
                platform: Platform::Nx,
                id: value,
            }),
            _ => Err(ParserError::custom(format!(
                "Unknown game platform: {value:x}"
            ))),
        }
    }
}

impl BinaryDeserialize<'_> for UniqueGameId {
    type Ctx = bool;
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        lax: bool,
    ) -> Result<Self, ReadError> {
        let value = reader.read_at::<u32be>(position)?;
        let result = Self::try_from(value)
            .map_err(|_| ReadError::custom(format!("Unknown game platform: {value:x}")));
        if result.is_err() && lax {
            warn!("Unknown game platform: {value:x}");
            return Ok(Self {
                game: Game::Unknown,
                platform: Platform::Nx,
                id: value,
            });
        }
        result
    }
}

impl BinarySerialize for UniqueGameId {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, input.id)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Game {
    JustDance2014 = 20140,
    JustDance2015 = 20150,
    JustDance2016 = 20160,
    JustDance2017 = 20170,
    JustDance2018 = 20180,
    JustDance2019 = 20190,
    JustDance2020 = 20200,
    JustDanceChina = 20201,
    JustDance2021 = 20210,
    JustDance2022 = 20220,
    Unknown,
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if *self == Self::Unknown || *other == Self::Unknown {
            None
        } else {
            #[allow(
                clippy::as_conversions,
                reason = "the enum values are in the range of 20140-20220 so is always safe"
            )]
            (*self as u32).partial_cmp(&(*other as u32))
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JustDance2014 => std::write!(f, "Just Dance 2014"),
            Self::JustDance2015 => std::write!(f, "Just Dance 2015"),
            Self::JustDance2016 => std::write!(f, "Just Dance 2016"),
            Self::JustDance2017 => std::write!(f, "Just Dance 2017"),
            Self::JustDance2018 => std::write!(f, "Just Dance 2018"),
            Self::JustDance2019 => std::write!(f, "Just Dance 2019"),
            Self::JustDance2020 => std::write!(f, "Just Dance 2020"),
            Self::JustDanceChina => std::write!(f, "Just Dance China"),
            Self::JustDance2021 => std::write!(f, "Just Dance 2021"),
            Self::JustDance2022 => std::write!(f, "Just Dance 2022"),
            Self::Unknown => std::write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Platform {
    Win = 0x0,
    X360 = 0x1,
    Ps4 = 0x3,
    Wii = 0x5,
    WiiU = 0x8,
    Nx = 0xB,
}

impl Ord for Platform {
    #[allow(clippy::match_same_arms, reason = "Clearer this way")]
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Nx, Self::Nx) => Ordering::Equal,
            (Self::Nx, _) => Ordering::Greater,
            (_, Self::Nx) => Ordering::Less,
            (Self::WiiU, Self::WiiU) => Ordering::Equal,
            (Self::WiiU, _) => Ordering::Greater,
            (_, Self::WiiU) => Ordering::Less,
            (Self::Win, Self::Win) => Ordering::Equal,
            (Self::Win, _) => Ordering::Greater,
            (_, Self::Win) => Ordering::Less,
            (Self::Ps4, Self::Ps4) => Ordering::Equal,
            (Self::Ps4, _) => Ordering::Greater,
            (_, Self::Ps4) => Ordering::Less,
            (Self::X360, Self::X360) => Ordering::Equal,
            (Self::X360, _) => Ordering::Greater,
            (_, Self::X360) => Ordering::Less,
            (Self::Wii, Self::Wii) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Platform {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win => write!(f, "Windows"),
            Self::X360 => std::write!(f, "Xbox 360"),
            Self::Ps4 => std::write!(f, "PlayStation 4"),
            Self::Wii => std::write!(f, "Wii"),
            Self::WiiU => std::write!(f, "Wii U"),
            Self::Nx => std::write!(f, "Switch"),
        }
    }
}

impl TryFrom<u32> for Platform {
    type Error = ParserError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Self::Win),
            0x1 => Ok(Self::X360),
            0x3 => Ok(Self::Ps4),
            0x5 => Ok(Self::Wii),
            0x8 => Ok(Self::WiiU),
            0xB => Ok(Self::Nx),
            _ => Err(ParserError::custom(format!("Unknown platform id {value}!"))),
        }
    }
}

impl From<Platform> for u32 {
    #[allow(
        clippy::as_conversions,
        reason = "Platform is repr(u32) thus this is always safe"
    )]
    fn from(value: Platform) -> Self {
        value as Self
    }
}

impl BinaryDeserialize<'_> for Platform {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Self::try_from(reader.read_at::<u32be>(position)?)
            .map_err(|e| ReadError::custom(format!("{e:?}")))
    }
}

impl BinarySerialize for Platform {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, u32::from(input))?;
        Ok(())
    }
}

// Calculates the Ubisoft string id for a given string.
//
// Implementation based on the Python implementation by github.com/InvoxiPlayGames
#[must_use]
pub fn string_id(string: &str) -> u32 {
    let bytes = string.as_bytes();
    let mut upper = Vec::with_capacity(bytes.len());
    // Convert lowercase chars to uppercase
    for byte in bytes {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    ubi_crc(&upper)
}

/// Calculates the Ubisoft string id for a given os string.
///
/// Implementation based on the Python implementation by github.com/InvoxiPlayGames
#[must_use]
pub fn os_string_id(string: &OsStr) -> u32 {
    let bytes = string.as_encoded_bytes();
    let mut upper = Vec::with_capacity(bytes.len());
    // Convert lowercase chars to uppercase
    for byte in bytes {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    ubi_crc(&upper)
}

/// Calculates the Ubisoft string id for two strings.
///
/// Implementation based on the Python implementation by github.com/InvoxiPlayGames
#[must_use]
pub fn string_id_2(one: &str, two: &str) -> u32 {
    let bytes_one = one.as_bytes();
    let bytes_two = two.as_bytes();
    let mut upper = Vec::with_capacity(bytes_one.len() + bytes_two.len());
    // Convert lowercase chars to uppercase
    for byte in bytes_one {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    for byte in bytes_two {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    ubi_crc(&upper)
}

#[must_use]
/// Implementation of the UbiArt CRC function
#[allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "Truncating is wanted"
)]
pub fn ubi_crc(data: &[u8]) -> u32 {
    let length = data.len();
    let mut a: u32 = 0x9E37_79B9;
    let mut b: u32 = 0x9E37_79B9;
    let mut c: u32 = 0;

    let mut pos = 0u64;
    while (pos as usize) + 12 <= length {
        a = a.wrapping_add(
            data.read_at::<u32le>(&mut pos)
                .unwrap_or_else(|_| unreachable!()),
        );
        b = b.wrapping_add(
            data.read_at::<u32le>(&mut pos)
                .unwrap_or_else(|_| unreachable!()),
        );
        c = c.wrapping_add(
            data.read_at::<u32le>(&mut pos)
                .unwrap_or_else(|_| unreachable!()),
        );
        (a, b, c) = shifter(a, b, c);
    }

    let pos = pos as usize;
    c = c.wrapping_add(length as u32);
    let left = length - pos;

    if left > 0 {
        if left >= 11 {
            c = c.wrapping_add(u32::from(data[pos + 10]) << 24);
        }
        if left >= 10 {
            c = c.wrapping_add(u32::from(data[pos + 9]) << 16);
        }
        if left >= 9 {
            c = c.wrapping_add(u32::from(data[pos + 8]) << 8);
        }
        if left >= 8 {
            b = b.wrapping_add(u32::from(data[pos + 7]) << 24);
        }
        if left >= 7 {
            b = b.wrapping_add(u32::from(data[pos + 6]) << 16);
        }
        if left >= 6 {
            b = b.wrapping_add(u32::from(data[pos + 5]) << 8);
        }
        if left >= 5 {
            b = b.wrapping_add(u32::from(data[pos + 4]));
        }
        if left >= 4 {
            a = a.wrapping_add(u32::from(data[pos + 3]) << 24);
        }
        if left >= 3 {
            a = a.wrapping_add(u32::from(data[pos + 2]) << 16);
        }
        if left >= 2 {
            a = a.wrapping_add(u32::from(data[pos + 1]) << 8);
        }
        if left >= 1 {
            a = a.wrapping_add(u32::from(data[pos]));
        }
    }

    (_, _, c) = shifter(a, b, c);
    c
}

/// Shifting implementation for ubicrc
const fn shifter(mut a: u32, mut b: u32, mut c: u32) -> (u32, u32, u32) {
    a = (a.wrapping_sub(b).wrapping_sub(c)) ^ (c >> 0xD);
    b = (b.wrapping_sub(a).wrapping_sub(c)) ^ (a << 0x8);
    c = (c.wrapping_sub(a).wrapping_sub(b)) ^ (b >> 0xD);
    a = (a.wrapping_sub(c).wrapping_sub(b)) ^ (c >> 0xC);
    let d = (b.wrapping_sub(a).wrapping_sub(c)) ^ (a << 0x10);
    c = (c.wrapping_sub(a).wrapping_sub(d)) ^ (d >> 0x5);
    a = (a.wrapping_sub(c).wrapping_sub(d)) ^ (c >> 0x3);
    b = (d.wrapping_sub(a).wrapping_sub(c)) ^ (a << 0xA);
    c = (c.wrapping_sub(a).wrapping_sub(b)) ^ (b >> 0xF);
    (a, b, c)
}

#[allow(clippy::missing_panics_doc)]
#[cfg(test)]
mod tests {
    use dotstar_toolkit_utils::vfs::VirtualPathBuf;
    use hipstr::HipStr;

    use super::{string_id, PathId, SplitPath};

    #[test]
    fn test_string_id() {
        assert_eq!(
            string_id("world/maps/adoreyou/videoscoach/adoreyou.vp9.720.webm"),
            0x45CC_A9CA
        );
    }

    #[test]
    fn test_splitpath_try_from_path() {
        let path = VirtualPathBuf::from("world/maps/adoreyou/videoscoach/adoreyou.vp9.720.webm");
        let sp = SplitPath::try_from(path.as_path()).unwrap();
        assert_eq!(&PathId::from(&sp), &PathId::from(0x45CC_A9CA));
    }

    #[test]
    fn test_splitpath_starts_with() {
        let split_path = SplitPath::new(
            HipStr::borrowed("cache/itf_cooked/nx/"),
            HipStr::borrowed("atlascontainer.ckd"),
        )
        .unwrap();
        assert!(split_path.starts_with("cache"));
        assert!(split_path.starts_with("cache/itf_cooked/nx/"));
        assert!(split_path.starts_with("cache/itf_cooked/nx/atlas"));
        assert!(split_path.starts_with("cache/itf_cooked/nx/atlascontainer.ckd"));
    }
}
