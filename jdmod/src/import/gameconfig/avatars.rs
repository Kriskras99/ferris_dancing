//! # Avatars
//! Import all the avatars.
//!
//! Current implementation is a bit wonky. A better option would be too manually match all avatar ids
//! to names per game. Then Just Dance 2017 avatars can also be imported.
use std::{collections::HashMap, fs::File};

use anyhow::{anyhow, Context, Error};
use dotstar_toolkit_utils::{
    bytes::read::BinaryDeserialize,
    vfs::{VirtualFile, VirtualPath},
};
use hipstr::HipStr;
use ownable::traits::IntoOwned;
use phf::phf_map;
use test_eq::test_eq;
use ubiart_toolkit::{
    cooked,
    cooked::{
        isg::AvatarsObjectives,
        tpl::types::{
            AvatarDescription, AvatarDescription16, AvatarDescription17, AvatarDescription1819,
            AvatarDescription2022,
        },
    },
    utils::Game,
};

use crate::{
    types::{
        gameconfig::avatars::{Avatar, UnlockType},
        ImportState,
    },
    utils::{cook_path, decode_texture},
};

/// Load existing avatars in the mod
fn load_avatar_config(
    is: &ImportState<'_>,
) -> Result<HashMap<HipStr<'static>, Avatar<'static>>, Error> {
    let avatars_config_path = is.dirs.avatars().join("avatars.json");
    let avatars_file = std::fs::read(&avatars_config_path).unwrap_or_else(|_| vec![b'{', b'}']);
    let avatars = serde_json::from_slice::<HashMap<HipStr, Avatar>>(&avatars_file)?;
    let avatars = avatars.into_owned();
    Ok(avatars)
}

/// Save the avatar metadata to avatars.json
fn save_avatar_config(
    is: &ImportState<'_>,
    avatars: &HashMap<HipStr, Avatar>,
) -> Result<(), Error> {
    let avatars_config_path = is.dirs.avatars().join("avatars.json");
    let file = File::create(avatars_config_path)?;
    serde_json::to_writer_pretty(file, &avatars)?;
    Ok(())
}

/// Decode and save the avatar images in the right location
fn save_images(
    is: &ImportState<'_>,
    name: &str,
    avatar: &Avatar,
    actor_path: &str,
) -> Result<(), Error> {
    let avatar_named_dir_path = is.dirs.avatars().join(name);
    std::fs::create_dir(&avatar_named_dir_path).with_context(|| {
        format!("Tried to create {avatar_named_dir_path:?}, but it already exists!")
    })?;
    let alt_actor_file = is.vfs.open(cook_path(actor_path, is.ugi)?.as_ref())?;
    let alt_actor = cooked::act::Actor::deserialize_with(&alt_actor_file, is.ugi)?;

    let image_actor = alt_actor
        .components
        .first()
        .ok_or_else(|| anyhow!("No templates in {}", actor_path))?;
    let mtg = image_actor.material_graphic_component()?;

    // Save decooked image
    let image_path = mtg.files[0].to_string();
    test_eq!(image_path.is_empty(), false)?;
    let cooked_image_path = cook_path(&image_path, is.ugi)?;
    let decooked_image = decode_texture(&is.vfs.open(cooked_image_path.as_ref())?, is.ugi)
        .with_context(|| format!("Failed to decode {cooked_image_path}!"))?;
    let avatar_image_path = is.dirs.avatars().join(avatar.image_path.as_str());
    decooked_image.save(&avatar_image_path)?;

    Ok(())
}

/// Minimum information to correctly parse an avatar
struct MinAvatarDesc<'a> {
    /// The id in the current game
    pub avatar_id: u32,
    /// The sound effect
    pub sound_family: HipStr<'a>,
    /// Unkown, so better keep it around
    pub status: u32,
    /// How is it unlocked in the game
    pub unlock_type: u32,
    /// Path to the actor
    pub actor_path: HipStr<'a>,
}

impl<'a> From<AvatarDescription2022<'a>> for MinAvatarDesc<'a> {
    fn from(value: AvatarDescription2022<'a>) -> Self {
        Self {
            avatar_id: value.avatar_id,
            sound_family: value.sound_family,
            status: value.status,
            unlock_type: value.unlock_type,
            actor_path: value.actor_path,
        }
    }
}

impl<'a> From<AvatarDescription1819<'a>> for MinAvatarDesc<'a> {
    fn from(value: AvatarDescription1819<'a>) -> Self {
        Self {
            avatar_id: value.avatar_id,
            sound_family: value.sound_family,
            status: value.status,
            unlock_type: value.unlock_type,
            actor_path: value.actor_path,
        }
    }
}

impl<'a> From<AvatarDescription17<'a>> for MinAvatarDesc<'a> {
    fn from(value: AvatarDescription17<'a>) -> Self {
        Self {
            avatar_id: value.avatar_id,
            sound_family: value.sound_family,
            status: value.status,
            unlock_type: value.unlock_type,
            actor_path: value.actor_path,
        }
    }
}

impl<'a> From<AvatarDescription16<'a>> for MinAvatarDesc<'a> {
    fn from(value: AvatarDescription16<'a>) -> Self {
        Self {
            avatar_id: value.avatar_id,
            sound_family: HipStr::default(),
            status: value.status,
            unlock_type: value.unlock_type,
            actor_path: value.actor_path,
        }
    }
}

impl<'a> From<AvatarDescription<'a>> for MinAvatarDesc<'a> {
    fn from(value: AvatarDescription<'a>) -> Self {
        match value {
            AvatarDescription::V16(desc) => desc.into(),
            AvatarDescription::V17(desc) => desc.into(),
            AvatarDescription::V1819(desc) => desc.into(),
            AvatarDescription::V2022(desc) => desc.into(),
        }
    }
}

/// Parse the avatar database scene for v20-v22
fn parse_actor_v20v22<'a>(
    is: &ImportState,
    file: &'a VirtualFile,
) -> Result<MinAvatarDesc<'a>, Error> {
    let mut actor_template = cooked::tpl::parse(file, is.ugi, is.lax)?;
    test_eq!(actor_template.components.len(), 2)?;
    let avatar_desc = actor_template
        .components
        .remove(1)
        .into_avatar_description()?;
    Ok(avatar_desc.into())
}

/// Parse the avatar database scene for v18-v19
fn parse_actor_v18v19<'a>(
    is: &ImportState,
    file: &'a VirtualFile,
) -> Result<MinAvatarDesc<'a>, Error> {
    let mut actor_template = cooked::tpl::parse(file, is.ugi, is.lax)?;
    test_eq!(actor_template.components.len(), 2)?;
    let avatar_desc = actor_template
        .components
        .remove(1)
        .into_avatar_description()?;
    Ok(avatar_desc.into())
}

/// Parse the avatar database for v17
fn parse_actor_v17<'a>(
    is: &ImportState,
    file: &'a VirtualFile,
) -> Result<MinAvatarDesc<'a>, Error> {
    let mut actor_template = cooked::tpl::parse(file, is.ugi, is.lax)?;
    test_eq!(actor_template.components.len(), 2)?;
    let avatar_desc = actor_template
        .components
        .remove(1)
        .into_avatar_description()?;
    Ok(avatar_desc.into())
}

/// Parse the avatar database for v16
fn parse_actor_v16<'a>(
    is: &ImportState,
    file: &'a VirtualFile,
) -> Result<MinAvatarDesc<'a>, Error> {
    let mut actor_template = cooked::tpl::parse(file, is.ugi, is.lax)?;
    test_eq!(actor_template.components.len(), 2)?;
    let avatar_desc = actor_template
        .components
        .remove(1)
        .into_avatar_description()?;
    Ok(avatar_desc.into())
}

/// Parse the avatar database and import all avatars
pub fn import(
    is: &ImportState,
    avatardb_scene: &str,
    avatarsobjectives: Option<&AvatarsObjectives>,
) -> Result<(), Error> {
    let empty_objectives = HashMap::new();
    println!("Importing avatars...");

    // Open the avatardb and avatarsobjectives (which might be empty)
    let avatardb_file = is.vfs.open(cook_path(avatardb_scene, is.ugi)?.as_ref())?;
    let avatardb = cooked::isc::parse(&avatardb_file, is.ugi)?;
    let avatarsobjectives = avatarsobjectives.unwrap_or(&empty_objectives);
    let mut avatar_description_files = Vec::with_capacity(avatardb.scene.actors.len());

    let mut avatars = load_avatar_config(is)?;

    for actor in avatardb.scene.actors {
        let actor = actor.actor()?;

        // Extract avatar description from template
        let file = is
            .vfs
            .open(cook_path(actor.lua.as_ref(), is.ugi)?.as_ref())?;
        avatar_description_files.push(file);
    }

    for file in &avatar_description_files {
        let avatar_desc = match is.ugi.game {
            Game::JustDance2022
            | Game::JustDance2021
            | Game::JustDance2020
            | Game::JustDanceChina => parse_actor_v20v22(is, file)?,
            Game::JustDance2019 | Game::JustDance2018 => parse_actor_v18v19(is, file)?,
            Game::JustDance2017 => parse_actor_v17(is, file)?,
            Game::JustDance2016 => parse_actor_v16(is, file)?,
            _ => todo!(),
        };

        let Ok(avatar_info) = get_name(avatar_desc.avatar_id) else {
            continue;
        };

        let name = avatar_info.name;

        // Only add new avatars
        if !avatars.contains_key(name) {
            let avatar_image_path = format!("{name}/avatar.png");

            let main_avatar = avatar_info.main_avatar().map(HipStr::borrowed);

            let avatar = Avatar {
                id: Some(avatar_desc.avatar_id),
                relative_song_name: HipStr::borrowed(avatar_info.map),
                sound_family: avatar_desc.sound_family,
                status: avatar_desc.status,
                unlock_type: UnlockType::from_unlock_type(
                    avatar_desc.unlock_type,
                    avatarsobjectives.get(&avatar_desc.avatar_id),
                )?,
                used_as_coach_map_name: HipStr::borrowed(avatar_info.map),
                used_as_coach_coach_id: avatar_info.coach,
                special_effect: avatar_info.special_effect,
                main_avatar,
                image_path: avatar_image_path.into(),
                guessed: false,
            };

            save_images(is, name, &avatar, &avatar_desc.actor_path)?;

            avatars.insert(HipStr::borrowed(name).into_owned(), avatar);
        }
    }

    import_unreferenced_avatars(is, &mut avatars)?;

    save_avatar_config(is, &avatars)?;

    Ok(())
}

/// Imports avatars that were not in the avatar database
fn import_unreferenced_avatars(
    is: &ImportState<'_>,
    avatars: &mut HashMap<HipStr, Avatar>,
) -> Result<(), Error> {
    let import_path = cook_path("world/avatars/", is.ugi)?;
    for avatar_id in is
        .vfs
        .walk_filesystem(import_path.as_ref())?
        .filter(|p| p.file_name().is_some_and(|s| s.ends_with("avatar.png.ckd")))
        .filter_map(VirtualPath::parent)
        .filter_map(VirtualPath::file_name)
        .flat_map(str::parse::<u32>)
    {
        let avatar_info = match get_name(avatar_id) {
            Ok(avatar_info) => avatar_info,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };

        let name = avatar_info.name;

        if !avatars.contains_key(name) {
            let avatar_named_dir_path = is.dirs.avatars().join(name);
            std::fs::create_dir(&avatar_named_dir_path).with_context(|| {
                format!("Tried to create {avatar_named_dir_path:?}, but it already exists!")
            })?;

            let avatar_image_path = format!("{name}/avatar.png");

            let main_avatar = avatar_info.main_avatar().map(HipStr::borrowed);

            let avatar = Avatar {
                id: Some(avatar_id),
                relative_song_name: HipStr::borrowed(avatar_info.map),
                sound_family: HipStr::borrowed("AVTR_Common_Brand"),
                status: 1,
                unlock_type: UnlockType::Unlocked,
                used_as_coach_map_name: HipStr::borrowed(avatar_info.map),
                used_as_coach_coach_id: avatar_info.coach,
                special_effect: avatar_info.special_effect,
                main_avatar,
                image_path: avatar_image_path.into(),
                guessed: true,
            };

            // Save decooked image
            let cooked_image_path =
                cook_path(&format!("world/avatars/{avatar_id:0>4}/avatar.png"), is.ugi)?;
            let decooked_image = decode_texture(&is.vfs.open(cooked_image_path.as_ref())?, is.ugi)?;
            let avatar_image_path = is.dirs.avatars().join(avatar.image_path.as_str());
            decooked_image.save(&avatar_image_path)?;

            avatars.insert(HipStr::borrowed(name).into_owned(), avatar);
        }
    }

    Ok(())
}

/// Get the name for the `avatar_id` for `game`
fn get_name(avatar_id: u32) -> Result<AvatarInfo, String> {
    AVATAR_ID_INFO_MAP
        .get(&avatar_id)
        .copied()
        .ok_or_else(|| format!("Unknown Avatar ID: {avatar_id}"))
}

#[derive(Debug, Clone, Copy)]
/// Documents avatar information which might be missing in the game
pub struct AvatarInfo {
    /// Unique name for the avatar
    pub name: &'static str,
    /// The map this avatar belongs to
    pub map: &'static str,
    /// Which coach this avatar is based on
    pub coach: u32,
    /// Is this a golden avatar
    pub special_effect: bool,
}

impl AvatarInfo {
    /// Create a `AvatarInfo`
    pub const fn new(
        name: &'static str,
        map: &'static str,
        coach: u32,
        special_effect: bool,
    ) -> Self {
        Self {
            name,
            map,
            coach,
            special_effect,
        }
    }

    /// Get the main avatar for this avatar if it exists
    pub fn main_avatar(&self) -> Option<&'static str> {
        self.name
            .ends_with("_Gold")
            .then(|| &self.name[0..self.name.len() - 5])
    }
}

/// Mapping of avatar ID to the relevant metadata
static AVATAR_ID_INFO_MAP: phf::Map<u32, AvatarInfo> = phf_map! {
    1u32 => AvatarInfo::new("Dare_0", "Dare", 0, false),
    2u32 => AvatarInfo::new("DogsOut_0", "DogsOut", 0, false),
    3u32 => AvatarInfo::new("EyeOfTheTiger_0", "EyeOfTheTiger", 0, false),
    4u32 => AvatarInfo::new("GetAround_0", "GetAround", 0, false),
    5u32 => AvatarInfo::new("HotNCold_0", "HotNCold", 0, false),
    6u32 => AvatarInfo::new("ILikeToMoveIt_0", "ILikeToMoveIt", 0, false),
    7u32 => AvatarInfo::new("JinGoLoBa_0", "JinGoLoBa", 0, false),
    8u32 => AvatarInfo::new("RingMyBell_0", "RingMyBell", 0, false),
    9u32 => AvatarInfo::new("NineAfternoon_1", "NineAfternoon", 1, false),
    10u32 => AvatarInfo::new("BabyGirl_0", "BabyGirl", 0, false),
    11u32 => AvatarInfo::new("CallMe_0", "CallMe", 0, false),
    12u32 => AvatarInfo::new("ChickenPayback_0", "ChickenPayback", 0, false),
    13u32 => AvatarInfo::new("ComeOn_1", "ComeOn", 1, false),
    14u32 => AvatarInfo::new("CosmicGirl_0", "CosmicGirl", 0, false),
    15u32 => AvatarInfo::new("ElectroTribalDLC_0", "ElectroTribalDLC", 0, false),
    16u32 => AvatarInfo::new("Firework_0", "Firework", 0, false),
    23u32 => AvatarInfo::new("KattiKalandal_0", "KattiKalandal", 0, false),
    33u32 => AvatarInfo::new("Song2_0_C1", "Song2", 0, false),
    51u32 => AvatarInfo::new("HalloweenQUAT_2", "HalloweenQUAT", 2, false),
    52u32 => AvatarInfo::new("HalloweenQUAT_3", "HalloweenQUAT", 3, false),
    67u32 => AvatarInfo::new("BewareOf_0", "BewareOf", 0, false),
    72u32 => AvatarInfo::new("CrucifiedQUAT_3", "CrucifiedQUAT", 3, false),
    73u32 => AvatarInfo::new("CrucifiedQUAT_0", "CrucifiedQUAT", 0, false),
    76u32 => AvatarInfo::new("EverybodyNeeds_0", "EverybodyNeeds", 0, false),
    77u32 => AvatarInfo::new("FunHouseDLC_0_V1", "FunHouseDLC", 0, false),
    78u32 => AvatarInfo::new("GangnamStyleDLC_1_C1_V1", "GangnamStyleDLC", 1, false),
    80u32 => AvatarInfo::new("IstanbulQUAT_0", "IstanbulQUAT", 0, false),
    90u32 => AvatarInfo::new("RockLobster_0", "RockLobster", 0, false),
    97u32 => AvatarInfo::new("TimeWarpQUAT_1", "TimeWarpQUAT", 1, false),
    99u32 => AvatarInfo::new("WildWildWestQUAT_2", "WildWildWestQUAT", 2, false),
    106u32 => AvatarInfo::new("IWillSurvive_0", "IWillSurvive", 0, false),
    108u32 => AvatarInfo::new("SheWolf_0", "SheWolf", 0, false),
    115u32 => AvatarInfo::new("RobotRock_1", "RobotRock", 1, false),
    118u32 => AvatarInfo::new("Aquarius_1", "Aquarius", 1, false),
    127u32 => AvatarInfo::new("Gigolo_1", "Gigolo", 1, false),
    131u32 => AvatarInfo::new("Limbo_1", "Limbo", 1, false),
    170u32 => AvatarInfo::new("UbiSoftWatchDogs_AidenPearce", "UbiSoftWatchDogs", 0, false),
    171u32 => AvatarInfo::new("UbiSoftAssassins_Altair", "UbiSoftAssassins", 0, false),
    172u32 => AvatarInfo::new("UbiSoftGhostRecon", "UbiSoftGhostRecon", 0, false),
    173u32 => AvatarInfo::new("UbiSoftBeyondGood_Jade", "UbiSoftBeyondGood", 0, false),
    174u32 => AvatarInfo::new("UbiSoftBeyondGood_Peyj", "UbiSoftBeyondGood", 1, false),
    175u32 => AvatarInfo::new("UbiSoftPrinceOfPersia", "UbiSoftPrinceOfPersia", 0, false),
    176u32 => AvatarInfo::new("UbiSoftRabbids", "UbiSoftRabbids", 0, false),
    177u32 => AvatarInfo::new("UbiSoftSplinterCell_SamFisher", "UbiSoftSplinterCell", 0, false),
    178u32 => AvatarInfo::new("UbiSoftFarCry", "UbiSoftFarCry", 0, false),
    179u32 => AvatarInfo::new("UbiSoftZombiU", "UbiSoftZombiU", 0, false),
    180u32 => AvatarInfo::new("UbiSoftRayman_Rayman", "UbiSoftRayman", 0, false),
    181u32 => AvatarInfo::new("UbiSoftRayman_Barbara", "UbiSoftRayman", 1, false),
    182u32 => AvatarInfo::new("UbiSoftRayman_Globox", "UbiSoftRayman", 2, false),
    183u32 => AvatarInfo::new("UbiSoftRayman_Teensie", "UbiSoftRayman", 3, false),
    199u32 => AvatarInfo::new("YouNeverCan_1", "YouNeverCan", 1, false),
    239u32 => AvatarInfo::new("BollywoodXmas_0", "BollywoodXmas", 0, false),
    241u32 => AvatarInfo::new("Medal", "Medal", 0, false),
    250u32 => AvatarInfo::new("IGotAFeeling_0", "IGotAFeeling", 0, false),
    251u32 => AvatarInfo::new("Blame_0", "Blame", 0, false),
    252u32 => AvatarInfo::new("Animals_0", "Animals", 0, false),
    253u32 => AvatarInfo::new("LetsGroove_0", "LetsGroove", 0, false),
    254u32 => AvatarInfo::new("WilliamTell_0", "WilliamTell", 0, false),
    255u32 => AvatarInfo::new("BornThisWay_1", "BornThisWay", 1, false),
    256u32 => AvatarInfo::new("Hangover_0", "Hangover", 0, false),
    257u32 => AvatarInfo::new("Circus_2", "Circus", 2, false),
    258u32 => AvatarInfo::new("UptownFunk_0_C3", "UptownFunk", 0, false),
    259u32 => AvatarInfo::new("TheseBoots_0", "TheseBoots", 0, false),
    260u32 => AvatarInfo::new("MarioNX", "MarioNX", 0, false),
    263u32 => AvatarInfo::new("HitTheRoad_1", "HitTheRoad", 1, false),
    264u32 => AvatarInfo::new("SaintPatrick_0", "SaintPatrick", 0, false),
    265u32 => AvatarInfo::new("Teacher_0", "Teacher", 0, false),
    266u32 => AvatarInfo::new("TeacherALT_1", "TeacherALT", 1, false),
    267u32 => AvatarInfo::new("UptownFunkALT_1", "UptownFunkALT", 1, false),
    268u32 => AvatarInfo::new("WhenTheRain_1", "WhenTheRain", 1, false),
    269u32 => AvatarInfo::new("YoureTheOne_1", "YoureTheOne", 1, false),
    270u32 => AvatarInfo::new("AboutThatBass_0", "AboutThatBass", 0, false),
    271u32 => AvatarInfo::new("AboutThatBassALT_1", "AboutThatBassALT", 1, false),
    272u32 => AvatarInfo::new("AngryBirds_0", "AngryBirds", 0, false),
    273u32 => AvatarInfo::new("BoysBoys_1", "BoysBoys", 1, false),
    274u32 => AvatarInfo::new("Chiwawa_0", "Chiwawa", 0, false),
    275u32 => AvatarInfo::new("ElektroMambo_0", "ElektroMambo", 0, false),
    276u32 => AvatarInfo::new("HeyMama_1", "HeyMama", 1, false),
    277u32 => AvatarInfo::new("IGotAFeelingALT_0", "IGotAFeelingALT", 0, false),
    278u32 => AvatarInfo::new("LevanPolkka_0", "LevanPolkka", 0, false),
    279u32 => AvatarInfo::new("StuckOnAFeeling_0", "StuckOnAFeeling", 0, false),
    280u32 => AvatarInfo::new("TheChoice_0", "TheChoice", 0, false),
    281u32 => AvatarInfo::new("UnderTheSea_0", "UnderTheSea", 0, false),
    282u32 => AvatarInfo::new("JuntoATi_1", "JuntoATi", 1, false),
    283u32 => AvatarInfo::new("BornThisWayALT_0", "BornThisWayALT", 0, false),
    284u32 => AvatarInfo::new("Rabiosa_0", "Rabiosa", 0, false),
    285u32 => AvatarInfo::new("RabiosaALT_0", "RabiosaALT", 0, false),
    286u32 => AvatarInfo::new("Lights_0", "Lights", 0, false),
    287u32 => AvatarInfo::new("HeyMamaALT_1", "HeyMamaALT", 1, false),
    288u32 => AvatarInfo::new("WantToWantMe_0", "WantToWantMe", 0, false),
    289u32 => AvatarInfo::new("StadiumFlow_0", "StadiumFlow", 0, false),
    290u32 => AvatarInfo::new("KaboomPow_0", "KaboomPow", 0, false),
    291u32 => AvatarInfo::new("KungFunk_1", "KungFunk", 1, false),
    292u32 => AvatarInfo::new("Gibberish_1", "Gibberish", 1, false),
    293u32 => AvatarInfo::new("Fancy_2", "Fancy", 2, false),
    294u32 => AvatarInfo::new("CopaCabana_1", "CopaCabana", 1, false),
    295u32 => AvatarInfo::new("Albatraoz_0", "Albatraoz", 0, false),
    296u32 => AvatarInfo::new("AnimalsALT_0", "AnimalsALT", 0, false),
    297u32 => AvatarInfo::new("CircusALT_0", "CircusALT", 0, false),
    298u32 => AvatarInfo::new("FancyALT_0", "FancyALT", 0, false),
    299u32 => AvatarInfo::new("Heartbeat_0", "Heartbeat", 0, false),
    300u32 => AvatarInfo::new("HitTheRoadALT_2", "HitTheRoadALT", 2, false),
    301u32 => AvatarInfo::new("NoControl_1", "NoControl", 1, false),
    302u32 => AvatarInfo::new("Stargate_0", "Stargate", 0, false),
    303u32 => AvatarInfo::new("ThisIsHow_0", "ThisIsHow", 0, false),
    304u32 => AvatarInfo::new("ThisIsHowALT_0", "ThisIsHowALT", 0, false),
    305u32 => AvatarInfo::new("WantToWantMeALT_1", "WantToWantMeALT", 1, false),
    306u32 => AvatarInfo::new("Fun_0", "Fun", 0, false),
    307u32 => AvatarInfo::new("Coolos_0", "Coolos", 0, false),
    308u32 => AvatarInfo::new("UbiSoftAssassins_Cormac", "UbiSoftAssassins", 1, false),
    309u32 => AvatarInfo::new("UbiSoftAssassins_Arno", "UbiSoftAssassins", 2, false),
    310u32 => AvatarInfo::new("UbiSoftAssassins_Evie", "UbiSoftAssassins", 3, false),
    311u32 => AvatarInfo::new("UbiSoftValiantHearts_Emile", "UbiSoftValiantHearts", 0, false),
    312u32 => AvatarInfo::new("UbiSoftFarCry4_PaganMin", "UbiSoftFarCry4", 0, false),
    313u32 => AvatarInfo::new("UbiSoftGrowHome_BUD", "UbiSoftGrowHome", 0, false),
    314u32 => AvatarInfo::new("UbiSoftChildOfLight_Aurora", "UbiSoftChildOfLight", 0, false),
    315u32 => AvatarInfo::new("UbiSoftMightyQuest_SirPainhammer", "UbiSoftMightyQuest", 0, false),
    316u32 => AvatarInfo::new("DQ_Cake", "DQ", 0, false),
    317u32 => AvatarInfo::new("DQ_PalmTree", "DQ", 0, false),
    318u32 => AvatarInfo::new("DQ_IceCream", "DQ", 0, false),
    320u32 => AvatarInfo::new("DQ_Coconut", "DQ", 0, false),
    321u32 => AvatarInfo::new("DQ_Sunglasses", "DQ", 0, false),
    322u32 => AvatarInfo::new("DQ_Roller", "DQ", 0, false),
    323u32 => AvatarInfo::new("DQ_Keyboard", "DQ", 0, false),
    324u32 => AvatarInfo::new("DQ_Boombox", "DQ", 0, false),
    325u32 => AvatarInfo::new("DQ_DiscoBall", "DQ", 0, false),
    326u32 => AvatarInfo::new("DQ_Lightning", "DQ", 0, false),
    327u32 => AvatarInfo::new("DQ_Hurricane", "DQ", 0, false),
    328u32 => AvatarInfo::new("DQ_Sun", "DQ", 0, false),
    329u32 => AvatarInfo::new("DQ_Moon", "DQ", 0, false),
    330u32 => AvatarInfo::new("DQ_Comet", "DQ", 0, false),
    331u32 => AvatarInfo::new("DQ_Rocket", "DQ", 0, false),
    332u32 => AvatarInfo::new("DQ_YingYang", "DQ", 0, false),
    333u32 => AvatarInfo::new("DQ_Rainbow", "DQ", 0, false),
    334u32 => AvatarInfo::new("HoldingOut_FlyingCat", "HoldingOut", 4, false),
    335u32 => AvatarInfo::new("NeverGonna_Dinosaur", "NeverGonna", 0, false),
    336u32 => AvatarInfo::new("BollywoodXmas_Elephant", "BollywoodXmas", 4, false),
    345u32 => AvatarInfo::new("GangnamStyleDLC_1_C2", "GangnamStyleDLC", 1, false),
    346u32 => AvatarInfo::new("GangnamStyleDLC_1_C1_V2", "GangnamStyleDLC", 1, false),
    347u32 => AvatarInfo::new("GangnamStyleDLC_0", "GangnamStyleDLC", 0, false),
    353u32 => AvatarInfo::new("DieYoungDLC_1", "DieYoungDLC", 1, false),
    357u32 => AvatarInfo::new("TimberDLC_1", "TimberDLC", 1, false),
    360u32 => AvatarInfo::new("MovesLikeDLC_0_C1", "MovesLikeDLC", 0, false),
    362u32 => AvatarInfo::new("OneThingDLC_1", "OneThingDLC", 1, false),
    364u32 => AvatarInfo::new("KiloPapaDLC_0_V2", "KiloPapaDLC", 0, false),
    368u32 => AvatarInfo::new("BlurredLines_1", "BlurredLines", 1, false),
    369u32 => AvatarInfo::new("Gentleman_0_C2", "Gentleman", 0, false),
    370u32 => AvatarInfo::new("ThatPower_3", "ThatPower", 3, false),
    371u32 => AvatarInfo::new("KissYou_2", "KissYou", 2, false),
    372u32 => AvatarInfo::new("CmonDLC_0", "CmonDLC", 0, false),
    373u32 => AvatarInfo::new("WhatMakesYouBeautiful_0", "WhatMakesYouBeautiful", 0, false),
    374u32 => AvatarInfo::new("MovesLikeDLC_0_C4", "MovesLikeDLC", 0, false),
    375u32 => AvatarInfo::new("DynamiteQUAT_0", "DynamiteQUAT", 0, false),
    376u32 => AvatarInfo::new("DaFunk_1", "DaFunk", 1, false),
    378u32 => AvatarInfo::new("Pilgrim", "Pilgrim", 0, false),
    379u32 => AvatarInfo::new("SantaClaus", "SantaClaus", 0, false),
    380u32 => AvatarInfo::new("GetLucky_0", "GetLucky", 0, false),
    381u32 => AvatarInfo::new("MahNa_1", "MahNa", 1, false),
    382u32 => AvatarInfo::new("GangnamStyleDLC_Horse", "GangnamStyleDLC", 4, false),
    383u32 => AvatarInfo::new("AThousandDances_Penguin", "AThousandDances", 4, false),
    384u32 => AvatarInfo::new("Starships_Monster", "Starships", 4, false),
    385u32 => AvatarInfo::new("Lollipop_SockPuppet", "Lollipop", 4, false),
    386u32 => AvatarInfo::new("CaliforniaGurls_Raccoon", "CaliforniaGurls", 4, false),
    387u32 => AvatarInfo::new("UbiSoftForHonor_Samurai", "UbiSoftForHonor", 0, false),
    388u32 => AvatarInfo::new("UbiSoftForHonor_Viking", "UbiSoftForHonor", 1, false),
    389u32 => AvatarInfo::new("UbiSoftForHonor_Knight", "UbiSoftForHonor", 2, false),
    448u32 => AvatarInfo::new("Ghostbusters_0", "Ghostbusters", 0, false),
    478u32 => AvatarInfo::new("LeanOn_0", "LeanOn", 0, false),
    479u32 => AvatarInfo::new("LeanOn_1", "LeanOn", 1, false),
    480u32 => AvatarInfo::new("ScreamNShout_3", "ScreamNShout", 3, false),
    481u32 => AvatarInfo::new("ScreamNShoutALT_0", "ScreamNShoutALT", 0, false),
    498u32 => AvatarInfo::new("Hips_0", "Hips", 0, false),
    499u32 => AvatarInfo::new("WhatIsLove_0", "WhatIsLove", 0, false),
    500u32 => AvatarInfo::new("WorthItALT_2", "WorthItALT", 2, false),
    504u32 => AvatarInfo::new("DontStopMe_0_C1", "DontStopMe", 0, false),
    505u32 => AvatarInfo::new("Daddy_1", "Daddy", 1, false),
    506u32 => AvatarInfo::new("TicoTico_1", "TicoTico", 1, false),
    507u32 => AvatarInfo::new("Daddy_0", "Daddy", 0, false),
    516u32 => AvatarInfo::new("Samba_0", "Samba", 0, false),
    517u32 => AvatarInfo::new("Radical_0", "Radical", 0, false),
    518u32 => AvatarInfo::new("PoPiPo_2", "PoPiPo", 2, false),
    519u32 => AvatarInfo::new("LeanOnALT_0", "LeanOnALT", 0, false),
    520u32 => AvatarInfo::new("September_1", "September", 1, false),
    522u32 => AvatarInfo::new("ElTiki_1", "ElTiki", 1, false),
    524u32 => AvatarInfo::new("GhostInTheKeys_3", "GhostInTheKeys", 3, false),
    525u32 => AvatarInfo::new("WhatIsLoveALT_1", "WhatIsLoveALT", 1, false),
    526u32 => AvatarInfo::new("DontStopMeALT_0", "DontStopMeALT", 0, false),
    527u32 => AvatarInfo::new("WorthIt_0", "WorthIt", 0, false),
    528u32 => AvatarInfo::new("Titanium_0", "Titanium", 0, false),
    529u32 => AvatarInfo::new("Sorry_0", "Sorry", 0, false),
    539u32 => AvatarInfo::new("SeptemberALT_0", "SeptemberALT", 0, false),
    541u32 => AvatarInfo::new("DaddyALT_1", "DaddyALT", 1, false),
    542u32 => AvatarInfo::new("CheapThrills_0", "CheapThrills", 0, false),
    543u32 => AvatarInfo::new("RunTheNight_0", "RunTheNight", 0, false),
    544u32 => AvatarInfo::new("Oishii_0", "Oishii", 0, false),
    545u32 => AvatarInfo::new("CantFeelMyFace_0", "CantFeelMyFace", 0, false),
    546u32 => AvatarInfo::new("DragosteaDinTei_1", "DragosteaDinTei", 1, false),
    547u32 => AvatarInfo::new("Groove_1", "Groove", 1, false),
    550u32 => AvatarInfo::new("UbiSoftACCChina_ShaoJun", "UbiSoftACCChina", 0, false),
    551u32 => AvatarInfo::new("UbiSoftACCRussia_Nikolai", "UbiSoftACCRussia", 0, false),
    552u32 => AvatarInfo::new("UbiSoftACCIndia_Arbaaz", "UbiSoftACCIndia", 0, false),
    553u32 => AvatarInfo::new("UbiSoftFarCryPrimal_Takkar", "UbiSoftFarCry", 1, false),
    554u32 => AvatarInfo::new("UbiSoftRabbids_Disturbia", "UbiSoftRabbids", 0, false),
    555u32 => AvatarInfo::new("UbiSoftRainbow6Siege_IQ", "UbiSoftRainbow6Siege", 0, false),
    556u32 => AvatarInfo::new("UbiSoftTheDivision", "UbiSoftTheDivision", 0, false),
    557u32 => AvatarInfo::new("UbiSoftWatchDogs2_Marcus", "UbiSoftWatchDogs2", 0, false),
    559u32 => AvatarInfo::new("ColaSong_0", "ColaSong", 0, false),
    560u32 => AvatarInfo::new("DQ_Mask", "DQ", 0, false),
    561u32 => AvatarInfo::new("DQ_CoolKids", "DQ", 0, false),
    562u32 => AvatarInfo::new("DQ_AroundTheWorld", "DQ", 0, false),
    563u32 => AvatarInfo::new("DQ_NewHeroines", "DQ", 0, false),
    564u32 => AvatarInfo::new("DQ_Family", "DQ", 0, false),
    565u32 => AvatarInfo::new("DQ_HighEnergy", "DQ", 0, false),
    566u32 => AvatarInfo::new("DQ_YearRound", "DQ", 0, false),
    567u32 => AvatarInfo::new("DQ_KeepCalm", "DQ", 0, false),
    568u32 => AvatarInfo::new("DQ_StrongWoman", "DQ", 0, false),
    569u32 => AvatarInfo::new("ColaSongALT_0", "ColaSongALT", 0, false),
    570u32 => AvatarInfo::new("CakeByTheOceanALT_1", "CakeByTheOceanALT", 1, false),
    571u32 => AvatarInfo::new("ElTikiALT_1", "ElTikiALT", 1, false),
    572u32 => AvatarInfo::new("IntoYou_0", "IntoYou", 0, false),
    573u32 => AvatarInfo::new("CheapThrillsALT_0", "CheapThrillsALT", 0, false),
    574u32 => AvatarInfo::new("Bailar_0", "Bailar", 0, false),
    575u32 => AvatarInfo::new("WhereverIGo_0", "WhereverIGo", 0, false),
    576u32 => AvatarInfo::new("NaeNaeALT_1", "NaeNaeALT", 1, false),
    577u32 => AvatarInfo::new("UbiSoftSteep", "UbiSoftSteep", 0, false),
    578u32 => AvatarInfo::new("Bonbon_0", "Bonbon", 0, false),
    579u32 => AvatarInfo::new("LikeIWould_0", "LikeIWould", 0, false),
    580u32 => AvatarInfo::new("Leila_0", "Leila", 0, false),
    581u32 => AvatarInfo::new("ILoveRock_0", "ILoveRock", 0, false),
    582u32 => AvatarInfo::new("CakeByTheOcean_0", "CakeByTheOcean", 0, false),
    586u32 => AvatarInfo::new("DQ_Snack", "DQ", 0, false),
    587u32 => AvatarInfo::new("SorryALT_0", "SorryALT", 0, false),
    588u32 => AvatarInfo::new("TeDominar_1", "TeDominar", 1, false),
    589u32 => AvatarInfo::new("LastChristmas_0", "LastChristmas", 0, false),
    590u32 => AvatarInfo::new("AllAboutUs_2", "AllAboutUs", 2, false),
    591u32 => AvatarInfo::new("Bang_0", "Bang", 0, false),
    592u32 => AvatarInfo::new("Bicicleta_0", "Bicicleta", 0, false),
    593u32 => AvatarInfo::new("RedMangoose_0", "RedMangoose", 0, false),
    594u32 => AvatarInfo::new("SingleLadies_0", "SingleLadies", 0, false),
    595u32 => AvatarInfo::new("NaeNae_1", "NaeNae", 1, false),
    596u32 => AvatarInfo::new("Oishii_1", "Oishii", 1, false),
    597u32 => AvatarInfo::new("LastChristmas_1", "LastChristmas", 1, false),
    598u32 => AvatarInfo::new("HipsALT_1", "HipsALT", 1, false),
    599u32 => AvatarInfo::new("RadicalALT_1", "RadicalALT", 1, false),
    600u32 => AvatarInfo::new("LittleSwing_1", "LittleSwing", 1, false),
    606u32 => AvatarInfo::new("LegSongCHN_0", "LegSongCHN", 0, false),
    607u32 => AvatarInfo::new("KaraokeForeverCHN_0", "KaraokeForeverCHN", 0, false),
    608u32 => AvatarInfo::new("BigDreamerCHN_0", "BigDreamerCHN", 0, false),
    609u32 => AvatarInfo::new("BedtimeStoryCHN_0", "BedtimeStoryCHN", 0, false),
    639u32 => AvatarInfo::new("JDCBadGirl_0", "JDCBadGirl", 0, false),
    640u32 => AvatarInfo::new("JDCBangBangBang_1", "JDCBangBangBang", 1, false),
    641u32 => AvatarInfo::new("JDCDeep_0", "JDCDeep", 0, false),
    642u32 => AvatarInfo::new("JDCGrowl_0", "JDCGrowl", 0, false),
    646u32 => AvatarInfo::new("HowDeep_0", "HowDeep", 0, false),
    691u32 => AvatarInfo::new("WakaWaka_1", "WakaWaka", 1, false),
    694u32 => AvatarInfo::new("BlowYourMind_0", "BlowYourMind", 0, false),
    695u32 => AvatarInfo::new("AnotherOne_1", "AnotherOne", 1, false),
    697u32 => AvatarInfo::new("Carmen_0", "Carmen", 0, false),
    698u32 => AvatarInfo::new("Carmen_1", "Carmen", 1, false),
    699u32 => AvatarInfo::new("DaddyCool_0", "DaddyCool", 0, false),
    700u32 => AvatarInfo::new("Blue_0", "Blue", 0, false),
    704u32 => AvatarInfo::new("HappyFarmKids_0", "HappyFarmKids", 0, false),
    705u32 => AvatarInfo::new("SideTo_0", "SideTo", 0, false),
    706u32 => AvatarInfo::new("MakeItJingle_0", "MakeItJingle", 0, false),
    708u32 => AvatarInfo::new("GotThat_0", "GotThat", 0, false),
    709u32 => AvatarInfo::new("AnotherOneALT_0", "AnotherOneALT", 0, false),
    710u32 => AvatarInfo::new("SideToALT_0", "SideToALT", 0, false),
    711u32 => AvatarInfo::new("WakaWakaALT_0", "WakaWakaALT", 0, false),
    713u32 => AvatarInfo::new("8BitRetake_0", "8BitRetake", 0, false),
    714u32 => AvatarInfo::new("AutomatonALT_0", "AutomatonALT", 0, false),
    715u32 => AvatarInfo::new("TumBum_1", "TumBum", 1, false),
    716u32 => AvatarInfo::new("Rockabye_1", "Rockabye", 1, false),
    717u32 => AvatarInfo::new("FearlessPirateKids_0", "FearlessPirateKids", 0, false),
    718u32 => AvatarInfo::new("BubblePopALT_3", "BubblePopALT", 3, false),
    720u32 => AvatarInfo::new("Footloose_0_C3", "Footloose", 0, false),
    721u32 => AvatarInfo::new("24K_0", "24K", 0, false),
    722u32 => AvatarInfo::new("24KALT_0", "24KALT", 0, false),
    723u32 => AvatarInfo::new("Diggy_0", "Diggy", 0, false),
    724u32 => AvatarInfo::new("Chantaje_0", "Chantaje", 0, false),
    725u32 => AvatarInfo::new("LoveWard_1", "LoveWard", 1, false),
    726u32 => AvatarInfo::new("TumBumALT_0", "TumBumALT", 0, false),
    727u32 => AvatarInfo::new("SayonaraRetake_0", "SayonaraRetake", 0, false),
    729u32 => AvatarInfo::new("ChantajeALT_1", "ChantajeALT", 1, false),
    730u32 => AvatarInfo::new("Automaton_0", "Automaton", 0, false),
    731u32 => AvatarInfo::new("JohnWALT_0", "JohnWALT", 0, false),
    732u32 => AvatarInfo::new("BubblePop_2", "BubblePop", 2, false),
    738u32 => AvatarInfo::new("ItsyBitsyRetake_1", "ItsyBitsyRetake", 1, false),
    739u32 => AvatarInfo::new("ItsyBitsyRetake_0", "ItsyBitsyRetake", 0, false),
    741u32 => AvatarInfo::new("WakaWakaKids_0", "WakaWakaKids", 0, false),
    742u32 => AvatarInfo::new("MagicHalloweenKids_1", "MagicHalloweenKids", 1, false),
    743u32 => AvatarInfo::new("MagicHalloweenKids_0", "MagicHalloweenKids", 0, false),
    748u32 => AvatarInfo::new("BubblePopALT_2", "BubblePopALT", 2, false),
    749u32 => AvatarInfo::new("BubblePopALT_1", "BubblePopALT", 1, false),
    750u32 => AvatarInfo::new("BubblePopALT_0", "BubblePopALT", 0, false),
    753u32 => AvatarInfo::new("LoveIsAll_0", "LoveIsAll", 0, false),
    775u32 => AvatarInfo::new("FootlooseKids_0", "FootlooseKids", 0, false),
    776u32 => AvatarInfo::new("LoveWard_0", "LoveWard", 0, false),
    778u32 => AvatarInfo::new("KeepOn_0", "KeepOn", 0, false),
    780u32 => AvatarInfo::new("RiskyBusiness_0", "RiskyBusiness", 0, false),
    781u32 => AvatarInfo::new("Dharma_0", "Dharma", 0, false),
    782u32 => AvatarInfo::new("Cottonmouth_0", "Cottonmouth", 0, false),
    785u32 => AvatarInfo::new("DharmaALT_0", "DharmaALT", 0, false),
    786u32 => AvatarInfo::new("WDFGlitchy_0", "WDFGlitchy", 0, false),
    803u32 => AvatarInfo::new("UbiSoftRabbids_Apache", "UbiSoftRabbids", 0, false),
    804u32 => AvatarInfo::new("UbiSoftRabbids_Cotton", "UbiSoftRabbids", 1, false),
    805u32 => AvatarInfo::new("UbiSoftRabbids_SexyAndIKnowItDLC", "UbiSoftRabbids", 2, false),
    808u32 => AvatarInfo::new("ShapeOfYou_0", "ShapeOfYou", 0, false),
    809u32 => AvatarInfo::new("AnotherOne_2", "AnotherOne", 2, false),
    810u32 => AvatarInfo::new("BadLiar_0", "BadLiar", 0, false),
    811u32 => AvatarInfo::new("KissingStrangers_1", "KissingStrangers", 1, false),
    812u32 => AvatarInfo::new("MissAmazingKIDS_0", "MissAmazingKIDS", 0, false),
    813u32 => AvatarInfo::new("NewFace_1", "NewFace", 1, false),
    814u32 => AvatarInfo::new("SlumberParty_1", "SlumberParty", 1, false),
    815u32 => AvatarInfo::new("BeepBeep_0", "BeepBeep", 0, false),
    816u32 => AvatarInfo::new("FunkyRobotKids_0", "FunkyRobotKids", 0, false),
    817u32 => AvatarInfo::new("PixieLandKids_0", "PixieLandKids", 0, false),
    818u32 => AvatarInfo::new("Sidewinder_0", "Sidewinder", 0, false),
    820u32 => AvatarInfo::new("AllYouGotta_0", "AllYouGotta", 0, false),
    821u32 => AvatarInfo::new("AnotherOne_3", "AnotherOne", 3, false),
    822u32 => AvatarInfo::new("BubblePop_0", "BubblePop", 0, false),
    823u32 => AvatarInfo::new("BubblePop_1", "BubblePop", 1, false),
    824u32 => AvatarInfo::new("Copperhead_1", "Copperhead", 1, false),
    825u32 => AvatarInfo::new("Despacito_3", "Despacito", 3, false),
    826u32 => AvatarInfo::new("DespacitoALT_0", "DespacitoALT", 0, false),
    827u32 => AvatarInfo::new("HowFar_0", "HowFar", 0, false),
    828u32 => AvatarInfo::new("Instruction_0", "Instruction", 0, false),
    829u32 => AvatarInfo::new("JohnW_0", "JohnW", 0, false),
    830u32 => AvatarInfo::new("KissingStrangers_0", "KissingStrangers", 0, false),
    831u32 => AvatarInfo::new("KissingStrangersALT_1", "KissingStrangersALT", 1, false),
    832u32 => AvatarInfo::new("NaughtyGirl_0", "NaughtyGirl", 0, false),
    833u32 => AvatarInfo::new("SwishSwish_0", "SwishSwish", 0, false),
    834u32 => AvatarInfo::new("SwishSwish_1", "SwishSwish", 1, false),
    835u32 => AvatarInfo::new("SwishSwish_2", "SwishSwish", 2, false),
    836u32 => AvatarInfo::new("SwishSwish_3", "SwishSwish", 3, false),
    837u32 => AvatarInfo::new("WakaWakaALT_1", "WakaWakaALT", 1, false),
    883u32 => AvatarInfo::new("WhereAreYou_2", "WhereAreYou", 2, false),
    884u32 => AvatarInfo::new("ObsessionRetake_0", "ObsessionRetake", 0, false),
    885u32 => AvatarInfo::new("MamaMia_0", "MamaMia", 0, false),
    886u32 => AvatarInfo::new("GhostKids_0", "GhostKids", 0, false),
    887u32 => AvatarInfo::new("GhostKids_1", "GhostKids", 1, false),
    888u32 => AvatarInfo::new("JurassicKids_0", "JurassicKids", 0, false),
    889u32 => AvatarInfo::new("WorkWorkALT_0", "WorkWorkALT", 0, false),
    890u32 => AvatarInfo::new("ImStillStanding_0", "ImStillStanding", 0, false),
    891u32 => AvatarInfo::new("SpaceGirlKids_0", "SpaceGirlKids", 0, false),
    892u32 => AvatarInfo::new("Fire_0", "Fire", 0, false),
    893u32 => AvatarInfo::new("CaPlane_0", "CaPlane", 0, false),
    894u32 => AvatarInfo::new("Shaky_0", "Shaky", 0, false),
    895u32 => AvatarInfo::new("SaintPatrickKids_0", "SaintPatrickKids", 0, false),
    896u32 => AvatarInfo::new("SaintPatrickKids_1", "SaintPatrickKids", 1, false),
    897u32 => AvatarInfo::new("Adeyyo_0", "Adeyyo", 0, false),
    898u32 => AvatarInfo::new("MerryChristmasKids_0", "MerryChristmasKids", 0, false),
    899u32 => AvatarInfo::new("MerryChristmasKids_1", "MerryChristmasKids", 1, false),
    900u32 => AvatarInfo::new("NinjaKids_0", "NinjaKids", 0, false),
    901u32 => AvatarInfo::new("MadLove_0", "MadLove", 0, false),
    902u32 => AvatarInfo::new("LittlePartyALT_1", "LittlePartyALT", 1, false),
    903u32 => AvatarInfo::new("MiMiMi_0", "MiMiMi", 0, false),
    904u32 => AvatarInfo::new("MiMiMi_1", "MiMiMi", 1, false),
    905u32 => AvatarInfo::new("TheExplorerKids_0", "TheExplorerKids", 0, false),
    906u32 => AvatarInfo::new("WorkWork_0", "WorkWork", 0, false),
    907u32 => AvatarInfo::new("MonstersAcademyKids_0", "MonstersAcademyKids", 0, false),
    908u32 => AvatarInfo::new("Narco_0", "Narco", 0, false),
    909u32 => AvatarInfo::new("BumBumTamTamALT_0", "BumBumTamTamALT", 0, false),
    910u32 => AvatarInfo::new("Finesse_1", "Finesse", 1, false),
    911u32 => AvatarInfo::new("LittleParty_0", "LittleParty", 0, false),
    912u32 => AvatarInfo::new("Rhythm_0", "Rhythm", 0, false),
    913u32 => AvatarInfo::new("IFeelItComing_0", "IFeelItComing", 0, false),
    914u32 => AvatarInfo::new("Havana_0", "Havana", 0, false),
    915u32 => AvatarInfo::new("MakeMeFeel_0", "MakeMeFeel", 0, false),
    916u32 => AvatarInfo::new("WaterMe_0", "WaterMe", 0, false),
    917u32 => AvatarInfo::new("MiloscW_0", "MiloscW", 0, false),
    918u32 => AvatarInfo::new("FinesseALT_0", "FinesseALT", 0, false),
    919u32 => AvatarInfo::new("MiMiMiALT_0", "MiMiMiALT", 0, false),
    920u32 => AvatarInfo::new("MiMiMiALT_1", "MiMiMiALT", 1, false),
    921u32 => AvatarInfo::new("MiMiMiALT_2", "MiMiMiALT", 2, false),
    922u32 => AvatarInfo::new("MiMiMiALT_3", "MiMiMiALT", 3, false),
    923u32 => AvatarInfo::new("BumBumTamTamALT_1", "BumBumTamTamALT", 1, false),
    924u32 => AvatarInfo::new("MadLove_1", "MadLove", 1, false),
    925u32 => AvatarInfo::new("Finesse_0", "Finesse", 0, false),
    926u32 => AvatarInfo::new("Finesse_2", "Finesse", 2, false),
    927u32 => AvatarInfo::new("Finesse_3", "Finesse", 3, false),
    928u32 => AvatarInfo::new("ObsessionRetake_1", "ObsessionRetake", 1, false),
    929u32 => AvatarInfo::new("WhereAreYou_0", "WhereAreYou", 0, false),
    930u32 => AvatarInfo::new("WhereAreYou_1", "WhereAreYou", 1, false),
    931u32 => AvatarInfo::new("WorkWork_1", "WorkWork", 1, false),
    932u32 => AvatarInfo::new("WorkWork_2", "WorkWork", 2, false),
    933u32 => AvatarInfo::new("Fire_1", "Fire", 1, false),
    934u32 => AvatarInfo::new("MamaMia_1", "MamaMia", 1, false),
    935u32 => AvatarInfo::new("LittlePartyALT_0", "LittlePartyALT", 0, false),
    936u32 => AvatarInfo::new("Bang2019_0", "Bang2019", 0, false),
    937u32 => AvatarInfo::new("NewWorld_0", "NewWorld", 0, false),
    938u32 => AvatarInfo::new("NewReality_0", "NewReality", 0, false),
    939u32 => AvatarInfo::new("NotYourOrdinary_0", "NotYourOrdinary", 0, false),
    940u32 => AvatarInfo::new("NotYourOrdinary_1", "NotYourOrdinary", 1, false),
    941u32 => AvatarInfo::new("NotYourOrdinary_2", "NotYourOrdinary", 2, false),
    942u32 => AvatarInfo::new("NotYourOrdinary_3", "NotYourOrdinary", 3, false),
    943u32 => AvatarInfo::new("WhereAreYouALT_0", "WhereAreYouALT", 0, false),
    944u32 => AvatarInfo::new("WhereAreYouALT_1", "WhereAreYouALT", 1, false),
    945u32 => AvatarInfo::new("FireOnTheDancefloor_0", "FireOnTheDancefloor", 0, false),
    946u32 => AvatarInfo::new("LittlePartyALT_2", "LittlePartyALT", 2, false),
    947u32 => AvatarInfo::new("DameTu_0", "DameTu", 0, false),
    948u32 => AvatarInfo::new("WaterMeALT_0", "WaterMeALT", 0, false),
    949u32 => AvatarInfo::new("WaterMeALT_1", "WaterMeALT", 1, false),
    950u32 => AvatarInfo::new("NewRules_0", "NewRules", 0, false),
    951u32 => AvatarInfo::new("OMG_0", "OMG", 0, false),
    952u32 => AvatarInfo::new("OMG_1", "OMG", 1, false),
    953u32 => AvatarInfo::new("OMG_2", "OMG", 2, false),
    954u32 => AvatarInfo::new("OMGALT_0", "OMGALT", 0, false),
    955u32 => AvatarInfo::new("BumBumTamTam_0", "BumBumTamTam", 0, false),
    956u32 => AvatarInfo::new("BumBumTamTam_1", "BumBumTamTam", 1, false),
    959u32 => AvatarInfo::new("SweetLittle_0", "SweetLittle", 0, false),
    961u32 => AvatarInfo::new("NewRulesALT_0", "NewRulesALT", 0, false),
    962u32 => AvatarInfo::new("UbiSoftRainbowSixSiege_Ela", "UbiSoftRainbowSixSiege", 0, false),
    963u32 => AvatarInfo::new("UbiSoftRainbowSixSiege_Tachanka", "UbiSoftRainbowSixSiege", 1, false),
    964u32 => AvatarInfo::new("UbiSoftRainbowSixSiege_Ash", "UbiSoftRainbowSixSiege", 2, false),
    965u32 => AvatarInfo::new("UbiSoftRainbowSixSiege_Dokkaebi", "UbiSoftRainbowSixSiege", 3, false),
    966u32 => AvatarInfo::new("UbiSoftACOdyssey_Alexios", "UbiSoftACOdyssey", 0, false),
    967u32 => AvatarInfo::new("UbiSoftACOdyssey_Kassandra", "UbiSoftACOdyssey", 1, false),
    968u32 => AvatarInfo::new("UbiSoftWatchDogs2_Sitara", "UbiSoftWatchDogs2", 0, false),
    969u32 => AvatarInfo::new("UbiSoftWatchDogs2_Wrench", "UbiSoftWatchDogs2", 1, false),
    982u32 => AvatarInfo::new("PocoLoco_0", "PocoLoco", 0, false),
    983u32 => AvatarInfo::new("PacMan_0", "PacMan", 0, false),
    984u32 => AvatarInfo::new("PacMan_1", "PacMan", 1, false),
    985u32 => AvatarInfo::new("PacMan_2", "PacMan", 2, false),
    986u32 => AvatarInfo::new("PacMan_3", "PacMan", 3, false),
    987u32 => AvatarInfo::new("UbiSoftTheCrew2", "UbiSoftTheCrew2", 0, false),
    988u32 => AvatarInfo::new("MadLoveALT_0", "MadLoveALT", 0, false),
    989u32 => AvatarInfo::new("Familiar_0", "Familiar", 0, false),
    990u32 => AvatarInfo::new("UbiSoftUnknown_0", "UbiSoftUnknown", 0, false),
    991u32 => AvatarInfo::new("TOY_0", "TOY", 0, false),
    992u32 => AvatarInfo::new("SangriaWine_0", "SangriaWine", 0, false),
    993u32 => AvatarInfo::new("Bang2019ALT_0", "Bang2019ALT", 0, false),
    994u32 => AvatarInfo::new("HavanaALT_0", "HavanaALT", 0, false),
    995u32 => AvatarInfo::new("HavanaALT_1", "HavanaALT", 1, false),
    996u32 => AvatarInfo::new("NoTearsLeft_0", "NoTearsLeft", 0, false),
    997u32 => AvatarInfo::new("NoTearsLeft_1", "NoTearsLeft", 1, false),
    998u32 => AvatarInfo::new("OneKiss_0", "OneKiss", 0, false),
    999u32 => AvatarInfo::new("Calypso_0", "Calypso", 0, false),
    1000u32 => AvatarInfo::new("SweetSensation_0", "SweetSensation", 0, false),
    1001u32 => AvatarInfo::new("SweetSensation_1", "SweetSensation", 1, false),
    1002u32 => AvatarInfo::new("SweetSensation_2", "SweetSensation", 2, false),
    1003u32 => AvatarInfo::new("SweetSensation_3", "SweetSensation", 3, false),
    1004u32 => AvatarInfo::new("NiceForWhat_0", "NiceForWhat", 0, false),
    1005u32 => AvatarInfo::new("NiceForWhat_1", "NiceForWhat", 1, false),
    1006u32 => AvatarInfo::new("NiceForWhat_2", "NiceForWhat", 2, false),
    1007u32 => AvatarInfo::new("NiceForWhat_3", "NiceForWhat", 3, false),
    1008u32 => AvatarInfo::new("Mayores_0", "Mayores", 0, false),
    1009u32 => AvatarInfo::new("RaveIn_0", "RaveIn", 0, false),
    1010u32 => AvatarInfo::new("RaveIn_1", "RaveIn", 1, false),
    1011u32 => AvatarInfo::new("RaveIn_2", "RaveIn", 2, false),
    1012u32 => AvatarInfo::new("RaveIn_3", "RaveIn", 3, false),
    1013u32 => AvatarInfo::new("DDUDU_0", "DDUDU", 0, false),
    1014u32 => AvatarInfo::new("DDUDU_1", "DDUDU", 1, false),
    1015u32 => AvatarInfo::new("DDUDU_2", "DDUDU", 2, false),
    1016u32 => AvatarInfo::new("DDUDU_3", "DDUDU", 3, false),
    1019u32 => AvatarInfo::new("Sugar_0", "Sugar", 0, false),
    1020u32 => AvatarInfo::new("Sugar_1", "Sugar", 1, false),
    1021u32 => AvatarInfo::new("Sugar_2", "Sugar", 2, false),
    1022u32 => AvatarInfo::new("Sugar_3", "Sugar", 3, false),
    1023u32 => AvatarInfo::new("Sugar_4", "Sugar", 4, false),
    1024u32 => AvatarInfo::new("Sugar_5", "Sugar", 5, false),
    1025u32 => AvatarInfo::new("Sugar_6", "Sugar", 6, false),
    1026u32 => AvatarInfo::new("Sugar_7", "Sugar", 7, false),
    1027u32 => AvatarInfo::new("Sugar_8", "Sugar", 8, false),
    1028u32 => AvatarInfo::new("Sugar_9", "Sugar", 9, false),
    1029u32 => AvatarInfo::new("NiceForWhat_4", "NiceForWhat", 4, false),
    1030u32 => AvatarInfo::new("NiceForWhat_5", "NiceForWhat", 5, false),
    1031u32 => AvatarInfo::new("JD_Panda", "JD", 0, false),
    1058u32 => AvatarInfo::new("FreezeKids_0", "FreezeKids", 0, false),
    1059u32 => AvatarInfo::new("FreezeKids_1", "FreezeKids", 1, false),
    1060u32 => AvatarInfo::new("SchoolyardKids_0", "SchoolyardKids", 0, false),
    1061u32 => AvatarInfo::new("SchoolyardKids_1", "SchoolyardKids", 1, false),
    1062u32 => AvatarInfo::new("MedievalKids_0", "MedievalKids", 0, false),
    1063u32 => AvatarInfo::new("MedievalKids_1", "MedievalKids", 1, false),
    1064u32 => AvatarInfo::new("ChefKids_0", "ChefKids", 0, false),
    1065u32 => AvatarInfo::new("AdventurerKids_0", "AdventurerKids", 0, false),
    1066u32 => AvatarInfo::new("WizardKids_0", "WizardKids", 0, false),
    1067u32 => AvatarInfo::new("BandmasterKids_0", "BandmasterKids", 0, false),
    1068u32 => AvatarInfo::new("BirthdayKids_0", "BirthdayKids", 0, false),
    1083u32 => AvatarInfo::new("BalMasque_0", "BalMasque", 0, false),
    1084u32 => AvatarInfo::new("BalMasque_1", "BalMasque", 1, false),
    1085u32 => AvatarInfo::new("BalMasque_2", "BalMasque", 2, false),
    1086u32 => AvatarInfo::new("BalMasque_3", "BalMasque", 3, false),
    1087u32 => AvatarInfo::new("FitButYouKnow_0", "FitButYouKnow", 0, false),
    1092u32 => AvatarInfo::new("IAmTheBest_1", "IAmTheBest", 1, false),
    1093u32 => AvatarInfo::new("IAmTheBest_0", "IAmTheBest", 0, false),
    1095u32 => AvatarInfo::new("Vodovorot_0", "Vodovorot", 0, false),
    1096u32 => AvatarInfo::new("TheTime_0", "TheTime", 0, false),
    1097u32 => AvatarInfo::new("TheTime_1", "TheTime", 1, false),
    1098u32 => AvatarInfo::new("TheTime_2", "TheTime", 2, false),
    1099u32 => AvatarInfo::new("TheTime_3", "TheTime", 3, false),
    1101u32 => AvatarInfo::new("AlwaysLookOn_0", "AlwaysLookOn", 0, false),
    1102u32 => AvatarInfo::new("AlwaysLookOn_1", "AlwaysLookOn", 1, false),
    1103u32 => AvatarInfo::new("AlwaysLookOn_2", "AlwaysLookOn", 2, false),
    1104u32 => AvatarInfo::new("AlwaysLookOn_3", "AlwaysLookOn", 3, false),
    1105u32 => AvatarInfo::new("Policeman_0", "Policeman", 0, false),
    1106u32 => AvatarInfo::new("Policeman_1", "Policeman", 1, false),
    1107u32 => AvatarInfo::new("Policeman_2", "Policeman", 2, false),
    1108u32 => AvatarInfo::new("RainOverMe_0", "RainOverMe", 0, false),
    1109u32 => AvatarInfo::new("BalMasque_1_Gold", "BalMasque", 1, true),
    1110u32 => AvatarInfo::new("FitButYouKnow_0_Gold", "FitButYouKnow", 0, true),
    1111u32 => AvatarInfo::new("IAmTheBest_1_Gold", "IAmTheBest", 1, true),
    1112u32 => AvatarInfo::new("Vodovorot_0_Gold", "Vodovorot", 0, true),
    1113u32 => AvatarInfo::new("TheTime_0_Gold", "TheTime", 0, true),
    1114u32 => AvatarInfo::new("AlwaysLookOn_1_Gold", "AlwaysLookOn", 1, true),
    1115u32 => AvatarInfo::new("Policeman_0_Gold", "Policeman", 0, true),
    1116u32 => AvatarInfo::new("RainOverMe_0_Gold", "RainOverMe", 0, true),
    1117u32 => AvatarInfo::new("KeepInTouch_0", "KeepInTouch", 0, false),
    1118u32 => AvatarInfo::new("BadBoy_0", "BadBoy", 0, false),
    1119u32 => AvatarInfo::new("BadBoy_1", "BadBoy", 1, false),
    1120u32 => AvatarInfo::new("BadBoy_0_Gold", "BadBoy", 0, true),
    1121u32 => AvatarInfo::new("KeepInTouch_0_Gold", "KeepInTouch", 0, true),
    1122u32 => AvatarInfo::new("BabyShark_0", "BabyShark", 0, false),
    1123u32 => AvatarInfo::new("BabyShark_1", "BabyShark", 1, false),
    1124u32 => AvatarInfo::new("GodIsAWoman_0", "GodIsAWoman", 0, false),
    1125u32 => AvatarInfo::new("BabyShark_1_Gold", "BabyShark", 1, true),
    1126u32 => AvatarInfo::new("GodIsAWoman_0_Gold", "GodIsAWoman", 0, true),
    1127u32 => AvatarInfo::new("GetAround_0", "GetAround", 0, false),
    1128u32 => AvatarInfo::new("Firework_0", "Firework", 0, false),
    1129u32 => AvatarInfo::new("HeyYa_0", "HeyYa", 0, false),
    1130u32 => AvatarInfo::new("KattiKalandal_1", "KattiKalandal", 1, false),
    1131u32 => AvatarInfo::new("MonsterMash_0", "MonsterMash", 0, false),
    1132u32 => AvatarInfo::new("CaliforniaGurls_0", "CaliforniaGurls", 0, false),
    1133u32 => AvatarInfo::new("GotMeDancing_0", "GotMeDancing", 0, false),
    1134u32 => AvatarInfo::new("HeyBoy_0", "HeyBoy", 0, false),
    1135u32 => AvatarInfo::new("WhatYouWait_0", "WhatYouWait", 0, false),
    1136u32 => AvatarInfo::new("Disturbia_0", "Disturbia", 0, false),
    1137u32 => AvatarInfo::new("IstanbulQUAT_2", "IstanbulQUAT", 2, false),
    1138u32 => AvatarInfo::new("RockNRoll_0", "RockNRoll", 0, false),
    1139u32 => AvatarInfo::new("YouMakeMeFeelDLC_0", "YouMakeMeFeelDLC", 0, false),
    1140u32 => AvatarInfo::new("YouReTheFirst_0", "YouReTheFirst", 0, false),
    1141u32 => AvatarInfo::new("RobotRock_0", "RobotRock", 0, false),
    1142u32 => AvatarInfo::new("SheWolf_0", "SheWolf", 0, false),
    1143u32 => AvatarInfo::new("WhereHaveYou_0", "WhereHaveYou", 0, false),
    1144u32 => AvatarInfo::new("Starships_0", "Starships", 0, false),
    1145u32 => AvatarInfo::new("CmonDLC_1", "CmonDLC", 1, false),
    1146u32 => AvatarInfo::new("ThatPower_1", "ThatPower", 1, false),
    1147u32 => AvatarInfo::new("TurnUpTheLove_0", "TurnUpTheLove", 0, false),
    1148u32 => AvatarInfo::new("GetLucky_1", "GetLucky", 1, false),
    1149u32 => AvatarInfo::new("LimaGolf1_0", "LimaGolf1", 0, false),
    1150u32 => AvatarInfo::new("Luftballons_0", "Luftballons", 0, false),
    1151u32 => AvatarInfo::new("TheFox_1", "TheFox", 1, false),
    1152u32 => AvatarInfo::new("BuiltForThis_0", "BuiltForThis", 0, false),
    1153u32 => AvatarInfo::new("Happy_0", "Happy", 0, false),
    1154u32 => AvatarInfo::new("Summer_0", "Summer", 0, false),
    1155u32 => AvatarInfo::new("WalkThisWay_1", "WalkThisWay", 1, false),
    1156u32 => AvatarInfo::new("Tetris_3", "Tetris", 3, false),
    1157u32 => AvatarInfo::new("Macarena_3", "Macarena", 3, false),
    1158u32 => AvatarInfo::new("Birthday_0", "Birthday", 0, false),
    1159u32 => AvatarInfo::new("HoldingOut_0", "HoldingOut", 0, false),
    1160u32 => AvatarInfo::new("PoundTheAlarm_0", "PoundTheAlarm", 0, false),
    1161u32 => AvatarInfo::new("Blame_0", "Blame", 0, false),
    1162u32 => AvatarInfo::new("Animals_0", "Animals", 0, false),
    1163u32 => AvatarInfo::new("WilliamTell_0", "WilliamTell", 0, false),
    1164u32 => AvatarInfo::new("BornThisWay_1", "BornThisWay", 1, false),
    1165u32 => AvatarInfo::new("Circus_2_C3", "Circus", 2, false),
    1166u32 => AvatarInfo::new("UptownFunk_0_C3", "UptownFunk", 0, false),
    1167u32 => AvatarInfo::new("Chiwawa_0", "Chiwawa", 0, false),
    1168u32 => AvatarInfo::new("ElectroMambo_0", "ElectroMambo", 0, false),
    1169u32 => AvatarInfo::new("HeyMama_1", "HeyMama", 1, false),
    1170u32 => AvatarInfo::new("KaboomPow_0", "KaboomPow", 0, false),
    1171u32 => AvatarInfo::new("ThisIsHow_0", "ThisIsHow", 0, false),
    1172u32 => AvatarInfo::new("DieYoungDLC_0", "DieYoungDLC", 0, false),
    1173u32 => AvatarInfo::new("BoomDLC_0", "BoomDLC", 0, false),
    1174u32 => AvatarInfo::new("Cotton_0", "Cotton", 0, false),
    1175u32 => AvatarInfo::new("Fame_0", "Fame", 0, false),
    1176u32 => AvatarInfo::new("TribalDance_0", "TribalDance", 0, false),
    1177u32 => AvatarInfo::new("GoodFeeling_0", "GoodFeeling", 0, false),
    1178u32 => AvatarInfo::new("JaiHo_0", "JaiHo", 0, false),
    1179u32 => AvatarInfo::new("KetchupSong_0", "KetchupSong", 0, false),
    1180u32 => AvatarInfo::new("Kurio_1", "Kurio", 1, false),
    1181u32 => AvatarInfo::new("Lollipop_0", "Lollipop", 0, false),
    1182u32 => AvatarInfo::new("ThatsTheWay_0", "ThatsTheWay", 0, false),
    1183u32 => AvatarInfo::new("TheFinalCountdown_0", "TheFinalCountdown", 0, false),
    1184u32 => AvatarInfo::new("ET_0", "ET", 0, false),
    1185u32 => AvatarInfo::new("CrazyLittle_1", "CrazyLittle", 1, false),
    1186u32 => AvatarInfo::new("CryingBlood_0", "CryingBlood", 0, false),
    1187u32 => AvatarInfo::new("GirlsJustWant_0", "GirlsJustWant", 0, false),
    1188u32 => AvatarInfo::new("ScreamNShoutALT_0", "ScreamNShoutALT", 0, false),
    1189u32 => AvatarInfo::new("WhatIsLove_0", "WhatIsLove", 0, false),
    1190u32 => AvatarInfo::new("DontStopMe_0", "DontStopMe", 0, false),
    1191u32 => AvatarInfo::new("PoPiPo_2", "PoPiPo", 2, false),
    1192u32 => AvatarInfo::new("September_1", "September", 1, false),
    1193u32 => AvatarInfo::new("WorthIt_0", "WorthIt", 0, false),
    1194u32 => AvatarInfo::new("Titanium_0", "Titanium", 0, false),
    1195u32 => AvatarInfo::new("DragosteaDinTei_1", "DragosteaDinTei", 1, false),
    1196u32 => AvatarInfo::new("Bailar_0", "Bailar", 0, false),
    1197u32 => AvatarInfo::new("HappyFarmKids_0", "HappyFarmKids", 0, false),
    1198u32 => AvatarInfo::new("TumBum_1", "TumBum", 1, false),
    1199u32 => AvatarInfo::new("Footloose_0", "Footloose", 0, false),
    1200u32 => AvatarInfo::new("24K_0", "24K", 0, false),
    1201u32 => AvatarInfo::new("Automaton_0", "Automaton", 0, false),
    1202u32 => AvatarInfo::new("BubblePopALT_1", "BubblePopALT", 1, false),
    1203u32 => AvatarInfo::new("LoveWard_0", "LoveWard", 0, false),
    1204u32 => AvatarInfo::new("AllYouGotta_0", "AllYouGotta", 0, false),
    1205u32 => AvatarInfo::new("AnotherOne_3", "AnotherOne", 3, false),
    1206u32 => AvatarInfo::new("BubblePop_1", "BubblePop", 1, false),
    1207u32 => AvatarInfo::new("Despacito_3", "Despacito", 3, false),
    1208u32 => AvatarInfo::new("SwishSwish_2", "SwishSwish", 2, false),
    1209u32 => AvatarInfo::new("SwishSwish_3", "SwishSwish", 3, false),
    1210u32 => AvatarInfo::new("MamaMia_0", "MamaMia", 0, false),
    1211u32 => AvatarInfo::new("WorkWorkALT_0", "WorkWorkALT", 0, false),
    1212u32 => AvatarInfo::new("Adeyyo_0", "Adeyyo", 0, false),
    1213u32 => AvatarInfo::new("MerryChristmasKids_1", "MerryChristmasKids", 1, false),
    1214u32 => AvatarInfo::new("MiMiMi_1", "MiMiMi", 1, false),
    1215u32 => AvatarInfo::new("MonstersAcademyKids_0", "MonstersAcademyKids", 0, false),
    1216u32 => AvatarInfo::new("Finesse_1", "Finesse", 1, false),
    1217u32 => AvatarInfo::new("IFeelItComing_0", "IFeelItComing", 0, false),
    1218u32 => AvatarInfo::new("MiloscW_0", "MiloscW", 0, false),
    1219u32 => AvatarInfo::new("MadLove_1", "MadLove", 1, false),
    1220u32 => AvatarInfo::new("OMG_2", "OMG", 2, false),
    1221u32 => AvatarInfo::new("NoTearsLeft_0_C1", "NoTearsLeft", 0, false),
    1222u32 => AvatarInfo::new("OneKiss_0", "OneKiss", 0, false),
    1223u32 => AvatarInfo::new("SweetSensation_3", "SweetSensation", 3, false),
    1224u32 => AvatarInfo::new("DDUDU_3", "DDUDU", 3, false),
    1225u32 => AvatarInfo::new("Sugar_0", "Sugar", 0, false),
    1226u32 => AvatarInfo::new("Sugar_1", "Sugar", 1, false),
    1227u32 => AvatarInfo::new("ConCalma_0", "ConCalma", 0, false),
    1228u32 => AvatarInfo::new("ConCalma_1", "ConCalma", 1, false),
    1229u32 => AvatarInfo::new("ConCalma_1_Gold", "ConCalma", 1, true),
    1231u32 => AvatarInfo::new("Sushii_0", "Sushii", 0, false),
    1232u32 => AvatarInfo::new("Sushii_0_Gold", "Sushii", 0, true),
    1236u32 => AvatarInfo::new("SoyYoALT_0", "SoyYoALT", 0, false),
    1237u32 => AvatarInfo::new("SoyYoALT_1", "SoyYoALT", 1, false),
    1238u32 => AvatarInfo::new("SoyYoALT_0_Gold", "SoyYoALT", 0, true),
    1243u32 => AvatarInfo::new("Bangarang_0", "Bangarang", 0, false),
    1244u32 => AvatarInfo::new("Bangarang_0_Gold", "Bangarang", 0, true),
    1245u32 => AvatarInfo::new("365_0", "365", 0, false),
    1246u32 => AvatarInfo::new("365_0_Gold", "365", 0, true),
    1247u32 => AvatarInfo::new("TheTimeALT_0", "TheTimeALT", 0, false),
    1248u32 => AvatarInfo::new("TheTimeALT_0_Gold", "TheTimeALT", 0, true),
    1249u32 => AvatarInfo::new("IAmTheBestALT_0", "IAmTheBestALT", 0, false),
    1250u32 => AvatarInfo::new("IAmTheBestALT_0_Gold", "IAmTheBestALT", 0, true),
    1251u32 => AvatarInfo::new("RainOverMeALT_0", "RainOverMeALT", 0, false),
    1252u32 => AvatarInfo::new("RainOverMeALT_0_Gold", "RainOverMeALT", 0, true),
    1253u32 => AvatarInfo::new("UbiSoftSpaceJunkies_Bugout", "UbiSoftSpaceJunkies", 0, false),
    1254u32 => AvatarInfo::new("UbiSoftSpaceJunkies_Subaru", "UbiSoftSpaceJunkies", 1, false),
    1255u32 => AvatarInfo::new("UbiSoftFarCryNewDawn_Micky", "UbiSoftFarCry", 2, false),
    1256u32 => AvatarInfo::new("UbiSoftFarCryNewDawn_Lou", "UbiSoftFarCry", 3, false),
    1257u32 => AvatarInfo::new("UbiSoftTrailsRising", "UbiSoftTrailsRising", 0, false),
    1258u32 => AvatarInfo::new("ILikeIt_0", "ILikeIt", 0, false),
    1259u32 => AvatarInfo::new("ILikeIt_1", "ILikeIt", 1, false),
    1260u32 => AvatarInfo::new("ILikeIt_2", "ILikeIt", 2, false),
    1261u32 => AvatarInfo::new("ILikeIt_1_Gold", "ILikeIt", 1, true),
    1262u32 => AvatarInfo::new("UbiSoftTheDivision_BrianJohnson", "UbiSoftTheDivision", 2, false),
    1263u32 => AvatarInfo::new("GodIsAWomanALT_0", "GodIsAWomanALT", 0, false),
    1264u32 => AvatarInfo::new("GodIsAWomanALT_0_Gold", "GodIsAWomanALT", 0, true),
    1265u32 => AvatarInfo::new("SoyYo_0", "SoyYo", 0, false),
    1266u32 => AvatarInfo::new("SoyYo_0_Gold", "SoyYo", 0, true),
    1267u32 => AvatarInfo::new("TelAviv_0", "TelAviv", 0, false),
    1268u32 => AvatarInfo::new("TelAviv_1", "TelAviv", 1, false),
    1269u32 => AvatarInfo::new("TelAviv_2", "TelAviv", 2, false),
    1270u32 => AvatarInfo::new("TelAviv_2_Gold", "TelAviv", 2, true),
    1271u32 => AvatarInfo::new("Skibidi_0", "Skibidi", 0, false),
    1272u32 => AvatarInfo::new("Skibidi_1", "Skibidi", 1, false),
    1273u32 => AvatarInfo::new("Skibidi_0_Gold", "Skibidi", 0, true),
    1274u32 => AvatarInfo::new("TakiTaki_0", "TakiTaki", 0, false),
    1275u32 => AvatarInfo::new("TakiTaki_0_Gold", "TakiTaki", 0, true),
    1279u32 => AvatarInfo::new("UbiSoftUnknown_1", "UbiSoftUnknown", 1, false),
    1280u32 => AvatarInfo::new("UbiSoftUnknown_2", "UbiSoftUnknown", 2, false),
    1281u32 => AvatarInfo::new("Swag_0", "Swag", 0, false),
    1282u32 => AvatarInfo::new("Swag_0_Gold", "Swag", 0, true),
    1283u32 => AvatarInfo::new("SushiiALT_0", "SushiiALT", 0, false),
    1284u32 => AvatarInfo::new("SushiiALT_0_Gold", "SushiiALT", 0, true),
    1285u32 => AvatarInfo::new("BangarangALT_0", "BangarangALT", 0, false),
    1286u32 => AvatarInfo::new("BangarangALT_0_Gold", "BangarangALT", 0, true),
    1287u32 => AvatarInfo::new("TakiTakiALT_0", "TakiTakiALT", 0, false),
    1288u32 => AvatarInfo::new("TakiTakiALT_1", "TakiTakiALT", 1, false),
    1289u32 => AvatarInfo::new("TakiTakiALT_1_Gold", "TakiTakiALT", 1, true),
    1290u32 => AvatarInfo::new("GetBusy_0", "GetBusy", 0, false),
    1291u32 => AvatarInfo::new("GetBusy_1", "GetBusy", 1, false),
    1292u32 => AvatarInfo::new("GetBusy_1_Gold", "GetBusy", 1, true),
    1293u32 => AvatarInfo::new("HighHopes_0", "HighHopes", 0, false),
    1294u32 => AvatarInfo::new("HighHopes_1", "HighHopes", 1, false),
    1295u32 => AvatarInfo::new("HighHopes_2", "HighHopes", 2, false),
    1296u32 => AvatarInfo::new("HighHopes_3", "HighHopes", 3, false),
    1297u32 => AvatarInfo::new("HighHopes_0_Gold", "HighHopes", 0, true),
    1298u32 => AvatarInfo::new("KillThisLove_0", "KillThisLove", 0, false),
    1299u32 => AvatarInfo::new("KillThisLove_1", "KillThisLove", 1, false),
    1300u32 => AvatarInfo::new("KillThisLove_2", "KillThisLove", 2, false),
    1301u32 => AvatarInfo::new("KillThisLove_3", "KillThisLove", 3, false),
    1302u32 => AvatarInfo::new("KillThisLove_2_Gold", "KillThisLove", 2, true),
    1303u32 => AvatarInfo::new("Talk_0", "Talk", 0, false),
    1304u32 => AvatarInfo::new("Talk_0_Gold", "Talk", 0, true),
    1306u32 => AvatarInfo::new("MaItu_0", "MaItu", 0, false),
    1307u32 => AvatarInfo::new("MaItu_0_Gold", "MaItu", 0, true),
    1308u32 => AvatarInfo::new("Everybody_0", "Everybody", 0, false),
    1309u32 => AvatarInfo::new("Everybody_1", "Everybody", 1, false),
    1310u32 => AvatarInfo::new("Everybody_2", "Everybody", 2, false),
    1311u32 => AvatarInfo::new("Everybody_3", "Everybody", 3, false),
    1312u32 => AvatarInfo::new("Everybody_2_Gold", "Everybody", 2, true),
    1313u32 => AvatarInfo::new("LaRespuesta_0", "LaRespuesta", 0, false),
    1314u32 => AvatarInfo::new("LaRespuesta_1", "LaRespuesta", 1, false),
    1315u32 => AvatarInfo::new("LaRespuesta_0_Gold", "LaRespuesta", 0, true),
    1316u32 => AvatarInfo::new("TalkALT_0", "TalkALT", 0, false),
    1317u32 => AvatarInfo::new("TalkALT_0_Gold", "TalkALT", 0, true),
    1318u32 => AvatarInfo::new("OldTownRoad_0", "OldTownRoad", 0, false),
    1319u32 => AvatarInfo::new("OldTownRoad_0_Gold", "OldTownRoad", 0, true),
    1320u32 => AvatarInfo::new("NotYourOrdinary_3_Gold", "NotYourOrdinary", 3, true),
    1321u32 => AvatarInfo::new("ILikeIt_0_Gold", "ILikeIt", 0, true),
    1322u32 => AvatarInfo::new("7Rings_0", "7Rings", 0, false),
    1323u32 => AvatarInfo::new("7Rings_1", "7Rings", 1, false),
    1324u32 => AvatarInfo::new("7Rings_2", "7Rings", 2, false),
    1325u32 => AvatarInfo::new("7Rings_1_Gold", "7Rings", 1, true),
    1326u32 => AvatarInfo::new("BadGuy_0", "BadGuy", 0, false),
    1327u32 => AvatarInfo::new("BadGuy_0_Gold", "BadGuy", 0, true),
    1328u32 => AvatarInfo::new("Footwork_0", "Footwork", 0, false),
    1329u32 => AvatarInfo::new("Footwork_0_Gold", "Footwork", 0, true),
    1330u32 => AvatarInfo::new("OldTownRoadALT_0", "OldTownRoadALT", 0, false),
    1331u32 => AvatarInfo::new("OldTownRoadALT_1", "OldTownRoadALT", 1, false),
    1332u32 => AvatarInfo::new("OldTownRoadALT_2", "OldTownRoadALT", 2, false),
    1333u32 => AvatarInfo::new("OldTownRoadALT_2_Gold", "OldTownRoadALT", 2, true),
    1334u32 => AvatarInfo::new("JustAnIllusion_0", "JustAnIllusion", 0, false),
    1335u32 => AvatarInfo::new("JustAnIllusion_1", "JustAnIllusion", 1, false),
    1336u32 => AvatarInfo::new("JustAnIllusion_1_Gold", "JustAnIllusion", 1, true),
    1337u32 => AvatarInfo::new("StopMovin_0", "StopMovin", 0, false),
    1338u32 => AvatarInfo::new("StopMovin_1", "StopMovin", 1, false),
    1339u32 => AvatarInfo::new("StopMovin_2", "StopMovin", 2, false),
    1340u32 => AvatarInfo::new("StopMovin_1_Gold", "StopMovin", 1, true),
    1342u32 => AvatarInfo::new("7RingsALT_0", "7RingsALT", 0, false),
    1343u32 => AvatarInfo::new("7RingsALT_0_Gold", "7RingsALT", 0, true),
    1344u32 => AvatarInfo::new("KillThisLoveALT_0", "KillThisLoveALT", 0, false),
    1345u32 => AvatarInfo::new("KillThisLoveALT_0_Gold", "KillThisLoveALT", 0, true),
    1346u32 => AvatarInfo::new("BassaSababa_0", "BassaSababa", 0, false),
    1347u32 => AvatarInfo::new("BassaSababa_0_Gold", "BassaSababa", 0, true),
    1348u32 => AvatarInfo::new("FancyTwice_0", "FancyTwice", 0, false),
    1349u32 => AvatarInfo::new("FancyTwice_1", "FancyTwice", 1, false),
    1350u32 => AvatarInfo::new("FancyTwice_2", "FancyTwice", 2, false),
    1351u32 => AvatarInfo::new("UglyBeauty_0", "UglyBeauty", 0, false),
    1352u32 => AvatarInfo::new("UglyBeauty_1", "UglyBeauty", 1, false),
    1353u32 => AvatarInfo::new("UglyBeauty_2", "UglyBeauty", 2, false),
    1354u32 => AvatarInfo::new("DoCarnaval_0", "DoCarnaval", 0, false),
    1355u32 => AvatarInfo::new("DoCarnaval_0_Gold", "DoCarnaval", 0, true),
    1356u32 => AvatarInfo::new("ConAltura_0", "ConAltura", 0, false),
    1357u32 => AvatarInfo::new("ConAltura_0_Gold", "ConAltura", 0, true),
    1358u32 => AvatarInfo::new("IDontCare_0", "IDontCare", 0, false),
    1359u32 => AvatarInfo::new("IDontCare_0_Gold", "IDontCare", 0, true),
    1360u32 => AvatarInfo::new("UglyBeauty_0_Gold", "UglyBeauty", 0, true),
    1361u32 => AvatarInfo::new("FancyTwice_2_Gold", "FancyTwice", 2, true),
    1362u32 => AvatarInfo::new("CanCan_0", "CanCan", 0, false),
    1363u32 => AvatarInfo::new("CanCan_1", "CanCan", 1, false),
    1364u32 => AvatarInfo::new("CanCan_2", "CanCan", 2, false),
    1365u32 => AvatarInfo::new("CanCan_3", "CanCan", 3, false),
    1366u32 => AvatarInfo::new("CanCan_4", "CanCan", 4, false),
    1367u32 => AvatarInfo::new("CanCan_1_Gold", "CanCan", 1, true),
    1368u32 => AvatarInfo::new("BoyWithLuv_0", "BoyWithLuv", 0, false),
    1369u32 => AvatarInfo::new("BoyWithLuv_1", "BoyWithLuv", 1, false),
    1370u32 => AvatarInfo::new("BoyWithLuv_2", "BoyWithLuv", 2, false),
    1371u32 => AvatarInfo::new("BoyWithLuv_2_Gold", "BoyWithLuv", 2, true),
    1393u32 => AvatarInfo::new("BadAssPrincessKids_0", "BadAssPrincessKids", 0, false),
    1394u32 => AvatarInfo::new("SpyKids_0", "SpyKids", 0, false),
    1395u32 => AvatarInfo::new("ChasmonauteKids_0", "ChasmonauteKids", 0, false),
    1396u32 => AvatarInfo::new("BubblesKids_0", "BubblesKids", 0, false),
    1397u32 => AvatarInfo::new("FiremenKids_0", "FiremenKids", 0, false),
    1398u32 => AvatarInfo::new("FiremenKids_1", "FiremenKids", 1, false),
    1399u32 => AvatarInfo::new("EcoloKids_1", "EcoloKids", 1, false),
    1400u32 => AvatarInfo::new("EcoloKids_0", "EcoloKids", 0, false),
    1402u32 => AvatarInfo::new("BalletKids_0", "BalletKids", 0, false),
    1403u32 => AvatarInfo::new("BalletKids_1", "BalletKids", 1, false),
    1407u32 => AvatarInfo::new("CarpetKids_0", "CarpetKids", 0, false),
    1408u32 => AvatarInfo::new("Senorita_0", "Senorita", 0, false),
    1409u32 => AvatarInfo::new("Senorita_1", "Senorita", 1, false),
    1411u32 => AvatarInfo::new("Senorita_1_Gold", "Senorita", 1, true),
    1412u32 => AvatarInfo::new("Bailando1997_0", "Bailando1997", 0, false),
    1413u32 => AvatarInfo::new("Bailando1997_0_Gold", "Bailando1997", 0, true),
    1414u32 => AvatarInfo::new("HabibiYaeni_0", "HabibiYaeni", 0, false),
    1415u32 => AvatarInfo::new("HabibiYaeni_1", "HabibiYaeni", 1, false),
    1416u32 => AvatarInfo::new("HabibiYaeni_2", "HabibiYaeni", 2, false),
    1417u32 => AvatarInfo::new("HabibiYaeni_0_Gold", "HabibiYaeni", 0, true),
    1421u32 => AvatarInfo::new("QueTirePaLante_0", "QueTirePaLante", 0, false),
    1422u32 => AvatarInfo::new("QueTirePaLante_1", "QueTirePaLante", 1, false),
    1423u32 => AvatarInfo::new("QueTirePaLante_2", "QueTirePaLante", 2, false),
    1424u32 => AvatarInfo::new("QueTirePaLante_3", "QueTirePaLante", 3, false),
    1425u32 => AvatarInfo::new("QueTirePaLante_1_Gold", "QueTirePaLante", 1, true),
    1426u32 => AvatarInfo::new("InTheNavy_0", "InTheNavy", 0, false),
    1427u32 => AvatarInfo::new("InTheNavy_1", "InTheNavy", 1, false),
    1428u32 => AvatarInfo::new("InTheNavy_2", "InTheNavy", 2, false),
    1429u32 => AvatarInfo::new("InTheNavy_3", "InTheNavy", 3, false),
    1430u32 => AvatarInfo::new("InTheNavy_3_Gold", "InTheNavy", 3, true),
    1435u32 => AvatarInfo::new("Georgia_0", "Georgia", 0, false),
    1436u32 => AvatarInfo::new("Georgia_0_Gold", "Georgia", 0, true),
    1437u32 => AvatarInfo::new("Buscando_0", "Buscando", 0, false),
    1438u32 => AvatarInfo::new("Buscando_0_Gold", "Buscando", 0, true),
    1439u32 => AvatarInfo::new("SambaDeJaneiro_0", "SambaDeJaneiro", 0, false),
    1440u32 => AvatarInfo::new("SambaDeJaneiro_1", "SambaDeJaneiro", 1, false),
    1441u32 => AvatarInfo::new("SambaDeJaneiro_2", "SambaDeJaneiro", 2, false),
    1442u32 => AvatarInfo::new("SambaDeJaneiro_1_Gold", "SambaDeJaneiro", 1, true),
    1443u32 => AvatarInfo::new("TheWeekend_0", "TheWeekend", 0, false),
    1444u32 => AvatarInfo::new("TheWeekend_1", "TheWeekend", 1, false),
    1445u32 => AvatarInfo::new("TheWeekend_1_Gold", "TheWeekend", 1, true),
    1446u32 => AvatarInfo::new("WithoutMe_0", "WithoutMe", 0, false),
    1447u32 => AvatarInfo::new("WithoutMe_1", "WithoutMe", 1, false),
    1448u32 => AvatarInfo::new("WithoutMe_2", "WithoutMe", 2, false),
    1449u32 => AvatarInfo::new("WithoutMe_2_Gold", "WithoutMe", 2, true),
    1450u32 => AvatarInfo::new("DibbyDibby_0", "DibbyDibby", 0, false),
    1451u32 => AvatarInfo::new("DibbyDibby_1", "DibbyDibby", 1, false),
    1452u32 => AvatarInfo::new("DibbyDibby_0_Gold", "DibbyDibby", 0, true),
    1453u32 => AvatarInfo::new("Alexandrie_0", "Alexandrie", 0, false),
    1454u32 => AvatarInfo::new("Alexandrie_1", "Alexandrie", 1, false),
    1455u32 => AvatarInfo::new("Alexandrie_2", "Alexandrie", 2, false),
    1456u32 => AvatarInfo::new("Alexandrie_1_Gold", "Alexandrie", 1, true),
    1457u32 => AvatarInfo::new("SweetEscape_0", "SweetEscape", 0, false),
    1458u32 => AvatarInfo::new("SweetEscape_0_Gold", "SweetEscape", 0, true),
    1459u32 => AvatarInfo::new("HeatSeeker_0", "HeatSeeker", 0, false),
    1460u32 => AvatarInfo::new("HeatSeeker_0_Gold", "HeatSeeker", 0, true),
    1461u32 => AvatarInfo::new("Juice_0", "Juice", 0, false),
    1462u32 => AvatarInfo::new("Juice_1", "Juice", 1, false),
    1463u32 => AvatarInfo::new("Juice_2", "Juice", 2, false),
    1464u32 => AvatarInfo::new("Juice_1_Gold", "Juice", 1, true),
    1465u32 => AvatarInfo::new("Kuliki_0", "Kuliki", 0, false),
    1466u32 => AvatarInfo::new("Kuliki_0_Gold", "Kuliki", 0, true),
    1467u32 => AvatarInfo::new("WhoRun_0", "WhoRun", 0, false),
    1468u32 => AvatarInfo::new("WhoRun_1", "WhoRun", 1, false),
    1469u32 => AvatarInfo::new("WhoRun_2", "WhoRun", 2, false),
    1470u32 => AvatarInfo::new("WhoRun_1_Gold", "WhoRun", 1, true),
    1471u32 => AvatarInfo::new("WithoutMeALTRETAKE_0", "WithoutMeALTRETAKE", 0, false),
    1472u32 => AvatarInfo::new("WithoutMeALT_0_Gold", "WithoutMeALT", 0, true),
    1473u32 => AvatarInfo::new("RareALT_0", "RareALT", 0, false),
    1474u32 => AvatarInfo::new("RareALT_0_Gold", "RareALT", 0, true),
    1475u32 => AvatarInfo::new("DanceMonkey_0", "DanceMonkey", 0, false),
    1476u32 => AvatarInfo::new("DanceMonkey_0_Gold", "DanceMonkey", 0, true),
    1477u32 => AvatarInfo::new("HabibiYaeniALT_0", "HabibiYaeniALT", 0, false),
    1478u32 => AvatarInfo::new("HabibiYaeniALT_0_Gold", "HabibiYaeniALT", 0, true),
    1479u32 => AvatarInfo::new("DontStartALT_0", "DontStartALT", 0, false),
    1480u32 => AvatarInfo::new("DontStartALT_0_Gold", "DontStartALT", 0, true),
    1481u32 => AvatarInfo::new("JuiceALT_0", "JuiceALT", 0, false),
    1482u32 => AvatarInfo::new("JuiceALT_1", "JuiceALT", 1, false),
    1483u32 => AvatarInfo::new("JuiceALT_2", "JuiceALT", 2, false),
    1484u32 => AvatarInfo::new("JuiceALT_2_Gold", "JuiceALT", 2, true),
    1485u32 => AvatarInfo::new("Zenit_0", "Zenit", 0, false),
    1486u32 => AvatarInfo::new("Zenit_0_Gold", "Zenit", 0, true),
    1487u32 => AvatarInfo::new("BuscandoALT_0", "BuscandoALT", 0, false),
    1488u32 => AvatarInfo::new("BuscandoALT_0_Gold", "BuscandoALT", 0, true),
    1489u32 => AvatarInfo::new("SambaDeJaneiroALT_0", "SambaDeJaneiroALT", 0, false),
    1490u32 => AvatarInfo::new("SambaDeJaneiroALT_1", "SambaDeJaneiroALT", 1, false),
    1491u32 => AvatarInfo::new("SambaDeJaneiroALT_1_Gold", "SambaDeJaneiroALT", 1, true),
    1492u32 => AvatarInfo::new("Joone_0", "Joone", 0, false),
    1493u32 => AvatarInfo::new("Joone_1", "Joone", 1, false),
    1494u32 => AvatarInfo::new("Joone_1_Gold", "Joone", 1, true),
    1500u32 => AvatarInfo::new("AdoreYou_0", "AdoreYou", 0, false),
    1501u32 => AvatarInfo::new("AdoreYou_0_Gold", "AdoreYou", 0, true),
    1502u32 => AvatarInfo::new("FeelSpecialALT_0", "FeelSpecialALT", 0, false),
    1503u32 => AvatarInfo::new("FeelSpecialALT_0_Gold", "FeelSpecialALT", 0, true),
    1504u32 => AvatarInfo::new("Runaway_0", "Runaway", 0, false),
    1505u32 => AvatarInfo::new("Runaway_1", "Runaway", 1, false),
    1506u32 => AvatarInfo::new("Runaway_0_Gold", "Runaway", 0, true),
    1507u32 => AvatarInfo::new("Volar_0", "Volar", 0, false),
    1508u32 => AvatarInfo::new("Volar_0_Gold", "Volar", 0, true),
    1509u32 => AvatarInfo::new("GetGetDown_0", "GetGetDown", 0, false),
    1510u32 => AvatarInfo::new("GetGetDown_1", "GetGetDown", 1, false),
    1511u32 => AvatarInfo::new("GetGetDown_2", "GetGetDown", 2, false),
    1512u32 => AvatarInfo::new("GetGetDown_3", "GetGetDown", 3, false),
    1513u32 => AvatarInfo::new("GetGetDown_3_Gold", "GetGetDown", 3, true),
    1517u32 => AvatarInfo::new("WhoRunALTRETAKE_0", "WhoRunALTRETAKE", 0, false),
    1518u32 => AvatarInfo::new("WhoRunALT_0_Gold", "WhoRunALT", 0, true),
    1522u32 => AvatarInfo::new("AllTheGoodGirls_0", "AllTheGoodGirls", 0, false),
    1523u32 => AvatarInfo::new("AllTheGoodGirls_0_Gold", "AllTheGoodGirls", 0, true),
    1524u32 => AvatarInfo::new("PacaDance_0", "PacaDance", 0, false),
    1525u32 => AvatarInfo::new("PacaDance_1", "PacaDance", 1, false),
    1526u32 => AvatarInfo::new("PacaDance_1_Gold", "PacaDance", 1, true),
    1527u32 => AvatarInfo::new("Rare_0", "Rare", 0, false),
    1528u32 => AvatarInfo::new("Rare_0_Gold", "Rare", 0, true),
    1529u32 => AvatarInfo::new("BoyYouCan_0", "BoyYouCan", 0, false),
    1530u32 => AvatarInfo::new("BoyYouCan_0_Gold", "BoyYouCan", 0, true),
    1531u32 => AvatarInfo::new("FeelSpecial_0", "FeelSpecial", 0, false),
    1532u32 => AvatarInfo::new("FeelSpecial_1", "FeelSpecial", 1, false),
    1533u32 => AvatarInfo::new("FeelSpecial_2", "FeelSpecial", 2, false),
    1534u32 => AvatarInfo::new("FeelSpecial_1_Gold", "FeelSpecial", 1, true),
    1535u32 => AvatarInfo::new("InTheNavy_2_Gold", "InTheNavy", 2, true),
    1536u32 => AvatarInfo::new("WithoutMe_0_Gold", "WithoutMe", 0, true),
    1537u32 => AvatarInfo::new("Magenta_0", "Magenta", 0, false),
    1538u32 => AvatarInfo::new("Magenta_0_Gold", "Magenta", 0, true),
    1539u32 => AvatarInfo::new("Lacrimosa_0", "Lacrimosa", 0, false),
    1540u32 => AvatarInfo::new("Lacrimosa_0_Gold", "Lacrimosa", 0, true),
    1541u32 => AvatarInfo::new("TillTheWorldEndsALT_0", "TillTheWorldEndsALT", 0, false),
    1542u32 => AvatarInfo::new("TillTheWorldEndsALT_0_Gold", "TillTheWorldEndsALT", 0, true),
    1543u32 => AvatarInfo::new("TillTheWorldEnds_0", "TillTheWorldEnds", 0, false),
    1544u32 => AvatarInfo::new("TillTheWorldEnds_0_Gold", "TillTheWorldEnds", 0, true),
    1545u32 => AvatarInfo::new("DontStart_0", "DontStart", 0, false),
    1546u32 => AvatarInfo::new("DontStart_0_Gold", "DontStart", 0, true),
    1547u32 => AvatarInfo::new("KickItALT_0", "KickItALT", 0, false),
    1548u32 => AvatarInfo::new("KickItALT_0_Gold", "KickItALT", 0, true),
    1549u32 => AvatarInfo::new("BlindingLightsALT_0", "BlindingLightsALT", 0, false),
    1550u32 => AvatarInfo::new("BlindingLightsALT_0_Gold", "BlindingLightsALT", 0, true),
    1551u32 => AvatarInfo::new("KickIt_0", "KickIt", 0, false),
    1552u32 => AvatarInfo::new("KickIt_1", "KickIt", 1, false),
    1553u32 => AvatarInfo::new("KickIt_2", "KickIt", 2, false),
    1554u32 => AvatarInfo::new("KickIt_3", "KickIt", 3, false),
    1555u32 => AvatarInfo::new("KickIt_1_Gold", "KickIt", 1, true),
    1556u32 => AvatarInfo::new("FriendInMe_0", "FriendInMe", 0, false),
    1557u32 => AvatarInfo::new("FriendInMe_0_Gold", "FriendInMe", 0, true),
    1558u32 => AvatarInfo::new("Sorbet_0", "Sorbet", 0, false),
    1559u32 => AvatarInfo::new("Sorbet_1", "Sorbet", 1, false),
    1560u32 => AvatarInfo::new("Sorbet_2", "Sorbet", 2, false),
    1561u32 => AvatarInfo::new("Sorbet_3", "Sorbet", 3, false),
    1562u32 => AvatarInfo::new("Sorbet_3_Gold", "Sorbet", 3, true),
    1563u32 => AvatarInfo::new("Uno_0", "Uno", 0, false),
    1564u32 => AvatarInfo::new("Uno_1", "Uno", 1, false),
    1565u32 => AvatarInfo::new("Uno_2", "Uno", 2, false),
    1566u32 => AvatarInfo::new("Uno_3", "Uno", 3, false),
    1567u32 => AvatarInfo::new("Uno_0_Gold", "Uno", 0, true),
    1568u32 => AvatarInfo::new("OtherSideSZA_0", "OtherSideSZA", 0, false),
    1569u32 => AvatarInfo::new("OtherSideSZA_0_Gold", "OtherSideSZA", 0, true),
    1570u32 => AvatarInfo::new("BlindingLights_0", "BlindingLights", 0, false),
    1571u32 => AvatarInfo::new("BlindingLights_0_Gold", "BlindingLights", 0, true),
    1572u32 => AvatarInfo::new("YoLeLlego_0", "YoLeLlego", 0, false),
    1573u32 => AvatarInfo::new("YoLeLlego_1", "YoLeLlego", 1, false),
    1574u32 => AvatarInfo::new("YoLeLlego_0_Gold", "YoLeLlego", 0, true),
    1575u32 => AvatarInfo::new("Temperature_0", "Temperature", 0, false),
    1576u32 => AvatarInfo::new("Temperature_1", "Temperature", 1, false),
    1577u32 => AvatarInfo::new("Temperature_2", "Temperature", 2, false),
    1578u32 => AvatarInfo::new("Temperature_3", "Temperature", 3, false),
    1579u32 => AvatarInfo::new("Temperature_2_Gold", "Temperature", 2, true),
    1580u32 => AvatarInfo::new("RainOnMe_0", "RainOnMe", 0, false),
    1581u32 => AvatarInfo::new("RainOnMe_1", "RainOnMe", 1, false),
    1582u32 => AvatarInfo::new("RainOnMe_2", "RainOnMe", 2, false),
    1583u32 => AvatarInfo::new("RainOnMe_3", "RainOnMe", 3, false),
    1584u32 => AvatarInfo::new("RainOnMe_1_Gold", "RainOnMe", 1, true),
    1585u32 => AvatarInfo::new("SaySo_0", "SaySo", 0, false),
    1586u32 => AvatarInfo::new("SaySo_1", "SaySo", 1, false),
    1587u32 => AvatarInfo::new("SaySo_0_Gold", "SaySo", 0, true),
    1594u32 => AvatarInfo::new("TemperatureALT_0", "TemperatureALT", 0, false),
    1595u32 => AvatarInfo::new("TemperatureALT_0_Gold", "TemperatureALT", 0, true),
    1619u32 => AvatarInfo::new("FreedFromDesire_0", "FreedFromDesire", 0, false),
    1620u32 => AvatarInfo::new("FreedFromDesire_0_Gold", "FreedFromDesire", 0, true),
    1633u32 => AvatarInfo::new("China_0", "China", 0, false),
    1634u32 => AvatarInfo::new("China_1", "China", 1, false),
    1635u32 => AvatarInfo::new("China_2", "China", 2, false),
    1636u32 => AvatarInfo::new("China_2_Gold", "China", 2, true),
    1638u32 => AvatarInfo::new("BreakMyHeart_0", "BreakMyHeart", 0, false),
    1639u32 => AvatarInfo::new("BreakMyHeart_0_Gold", "BreakMyHeart", 0, true),
    1641u32 => AvatarInfo::new("FlashPose_0", "FlashPose", 0, false),
    1642u32 => AvatarInfo::new("FlashPose_1", "FlashPose", 1, false),
    1643u32 => AvatarInfo::new("FlashPose_2", "FlashPose", 2, false),
    1644u32 => AvatarInfo::new("FlashPose_0_Gold", "FlashPose", 0, true),
    1645u32 => AvatarInfo::new("Siargo_0", "Siargo", 0, false),
    1646u32 => AvatarInfo::new("Siargo_0_Gold", "Siargo", 0, true),
    1647u32 => AvatarInfo::new("Human_0", "Human", 0, false),
    1648u32 => AvatarInfo::new("Human_0_Gold", "Human", 0, true),
    1649u32 => AvatarInfo::new("YouCanDance_0", "YouCanDance", 0, false),
    1650u32 => AvatarInfo::new("YouCanDance_0_Gold", "YouCanDance", 0, true),
    1651u32 => AvatarInfo::new("Funk_0", "Funk", 0, false),
    1652u32 => AvatarInfo::new("Funk_1", "Funk", 1, false),
    1653u32 => AvatarInfo::new("Funk_0_Gold", "Funk", 0, true),
    1654u32 => AvatarInfo::new("Believer_0", "Believer", 0, false),
    1655u32 => AvatarInfo::new("Believer_1", "Believer", 1, false),
    1656u32 => AvatarInfo::new("Believer_0_Gold", "Believer", 0, true),
    1657u32 => AvatarInfo::new("TGIF_0", "TGIF", 0, false),
    1658u32 => AvatarInfo::new("TGIF_0_Gold", "TGIF", 0, true),
    1659u32 => AvatarInfo::new("Jopping_0", "Jopping", 0, false),
    1660u32 => AvatarInfo::new("Jopping_1", "Jopping", 1, false),
    1661u32 => AvatarInfo::new("Jopping_2", "Jopping", 2, false),
    1662u32 => AvatarInfo::new("Jopping_2_Gold", "Jopping", 2, true),
    1668u32 => AvatarInfo::new("LevelUp_0", "LevelUp", 0, false),
    1669u32 => AvatarInfo::new("LevelUp_1", "LevelUp", 1, false),
    1670u32 => AvatarInfo::new("LevelUp_2", "LevelUp", 2, false),
    1671u32 => AvatarInfo::new("LevelUp_1_Gold", "LevelUp", 1, true),
    1672u32 => AvatarInfo::new("BossWitch_0", "BossWitch", 0, false),
    1673u32 => AvatarInfo::new("BossWitch_0_Gold", "BossWitch", 0, true),
    1674u32 => AvatarInfo::new("FollowTheWhiteRabbit_0", "FollowTheWhiteRabbit", 0, false),
    1675u32 => AvatarInfo::new("FollowTheWhiteRabbit_0_Gold", "FollowTheWhiteRabbit", 0, true),
    1676u32 => AvatarInfo::new("Popstars_0", "Popstars", 0, false),
    1677u32 => AvatarInfo::new("Popstars_1", "Popstars", 1, false),
    1678u32 => AvatarInfo::new("Popstars_2", "Popstars", 2, false),
    1679u32 => AvatarInfo::new("Popstars_3", "Popstars", 3, false),
    1680u32 => AvatarInfo::new("Popstars_3_Gold", "Popstars", 3, true),
    1681u32 => AvatarInfo::new("Mood_0", "Mood", 0, false),
    1682u32 => AvatarInfo::new("Mood_1", "Mood", 1, false),
    1683u32 => AvatarInfo::new("Mood_1_Gold", "Mood", 1, true),
    1684u32 => AvatarInfo::new("JoppingALT_0", "JoppingALT", 0, false),
    1685u32 => AvatarInfo::new("JoppingALT_0_Gold", "JoppingALT", 0, true),
    1686u32 => AvatarInfo::new("BlackMam_0", "BlackMam", 0, false),
    1687u32 => AvatarInfo::new("BlackMam_1", "BlackMam", 1, false),
    1688u32 => AvatarInfo::new("BlackMam_2", "BlackMam", 2, false),
    1689u32 => AvatarInfo::new("BlackMam_1_Gold", "BlackMam", 1, true),
    1690u32 => AvatarInfo::new("BlackMamALT_0", "BlackMamALT", 0, false),
    1691u32 => AvatarInfo::new("BlackMamALT_0_Gold", "BlackMamALT", 0, true),
    1692u32 => AvatarInfo::new("ImOuttaLove_0", "ImOuttaLove", 0, false),
    1693u32 => AvatarInfo::new("ImOuttaLove_0_Gold", "ImOuttaLove", 0, true),
    1694u32 => AvatarInfo::new("Boombayah_0", "Boombayah", 0, false),
    1695u32 => AvatarInfo::new("Boombayah_1", "Boombayah", 1, false),
    1696u32 => AvatarInfo::new("Boombayah_2", "Boombayah", 2, false),
    1697u32 => AvatarInfo::new("Boombayah_3", "Boombayah", 3, false),
    1698u32 => AvatarInfo::new("Boombayah_2_Gold", "Boombayah", 2, true),
    1699u32 => AvatarInfo::new("Baiana_0", "Baiana", 0, false),
    1700u32 => AvatarInfo::new("Baiana_0_Gold", "Baiana", 0, true),
    1702u32 => AvatarInfo::new("Malibu_0", "Malibu", 0, false),
    1703u32 => AvatarInfo::new("Malibu_1", "Malibu", 1, false),
    1704u32 => AvatarInfo::new("Malibu_2", "Malibu", 2, false),
    1705u32 => AvatarInfo::new("RockYourBody_0", "RockYourBody", 0, false),
    1706u32 => AvatarInfo::new("RockYourBody_0_Gold", "RockYourBody", 0, true),
    1707u32 => AvatarInfo::new("Jerusalema_0", "Jerusalema", 0, false),
    1708u32 => AvatarInfo::new("Jerusalema_1", "Jerusalema", 1, false),
    1709u32 => AvatarInfo::new("Jerusalema_2", "Jerusalema", 2, false),
    1710u32 => AvatarInfo::new("Jerusalema_3", "Jerusalema", 3, false),
    1711u32 => AvatarInfo::new("Jerusalema_3_Gold", "Jerusalema", 3, true),
    1712u32 => AvatarInfo::new("Judas_0", "Judas", 0, false),
    1713u32 => AvatarInfo::new("Judas_0_Gold", "Judas", 0, true),
    1714u32 => AvatarInfo::new("Buttons_0", "Buttons", 0, false),
    1716u32 => AvatarInfo::new("Buttons_2", "Buttons", 2, false),
    1718u32 => AvatarInfo::new("ChinaALT_0", "ChinaALT", 0, false),
    1719u32 => AvatarInfo::new("ChinaALT_1", "ChinaALT", 1, false),
    1720u32 => AvatarInfo::new("ChinaALT_1_Gold", "ChinaALT", 1, true),
    1721u32 => AvatarInfo::new("LoveStory_0", "LoveStory", 0, false),
    1722u32 => AvatarInfo::new("LoveStory_1", "LoveStory", 1, false),
    1723u32 => AvatarInfo::new("LoveStory_0_Gold", "LoveStory", 0, true),
    1724u32 => AvatarInfo::new("GirlLikeMe_0", "GirlLikeMe", 0, false),
    1725u32 => AvatarInfo::new("GirlLikeMe_1", "GirlLikeMe", 1, false),
    1726u32 => AvatarInfo::new("GirlLikeMe_2", "GirlLikeMe", 2, false),
    1727u32 => AvatarInfo::new("GirlLikeMe_3", "GirlLikeMe", 3, false),
    1728u32 => AvatarInfo::new("GirlLikeMe_3_Gold", "GirlLikeMe", 3, true),
    1729u32 => AvatarInfo::new("Chandelier_0", "Chandelier", 0, false),
    1730u32 => AvatarInfo::new("Chandelier_0_Gold", "Chandelier", 0, true),
    1731u32 => AvatarInfo::new("SuaCara_0", "SuaCara", 0, false),
    1732u32 => AvatarInfo::new("SuaCara_0_Gold", "SuaCara", 0, true),
    1733u32 => AvatarInfo::new("BoombayahALT_0", "BoombayahALT", 0, false),
    1734u32 => AvatarInfo::new("BoombayahALT_0_Gold", "BoombayahALT", 0, true),
    1737u32 => AvatarInfo::new("GirlLikeMeALT_0", "GirlLikeMeALT", 0, false),
    1738u32 => AvatarInfo::new("GirlLikeMeALT_0_Gold", "GirlLikeMeALT", 0, true),
    1739u32 => AvatarInfo::new("StopDropAndRoll_0", "StopDropAndRoll", 0, false),
    1740u32 => AvatarInfo::new("StopDropAndRoll_1", "StopDropAndRoll", 1, false),
    1741u32 => AvatarInfo::new("StopDropAndRoll_2", "StopDropAndRoll", 2, false),
    1742u32 => AvatarInfo::new("StopDropAndRoll_3", "StopDropAndRoll", 3, false),
    1743u32 => AvatarInfo::new("StopDropAndRoll_2_Gold", "StopDropAndRoll", 2, true),
    1744u32 => AvatarInfo::new("MightyReal_0", "MightyReal", 0, false),
    1745u32 => AvatarInfo::new("MightyReal_0_Gold", "MightyReal", 0, true),
    1746u32 => AvatarInfo::new("MrBlueSky_0", "MrBlueSky", 0, false),
    1747u32 => AvatarInfo::new("MrBlueSky_0_Gold", "MrBlueSky", 0, true),
    1748u32 => AvatarInfo::new("ThinkAboutThings_0", "ThinkAboutThings", 0, false),
    1749u32 => AvatarInfo::new("ThinkAboutThings_0_Gold", "ThinkAboutThings", 0, true),
    1750u32 => AvatarInfo::new("Chacarron_0", "Chacarron", 0, false),
    1751u32 => AvatarInfo::new("Chacarron_0_Gold", "Chacarron", 0, true),
    1752u32 => AvatarInfo::new("SuaCaraALT_0", "SuaCaraALT", 0, false),
    1753u32 => AvatarInfo::new("SuaCaraALT_1", "SuaCaraALT", 1, false),
    1754u32 => AvatarInfo::new("SuaCaraALT_0_Gold", "SuaCaraALT", 0, true),
    1755u32 => AvatarInfo::new("Boombayah_3_Gold", "Boombayah", 3, true),
    1756u32 => AvatarInfo::new("Believer_0_Gold", "Believer", 0, true),
    1757u32 => AvatarInfo::new("ChinaALT_0_Gold", "ChinaALT", 0, true),
    1758u32 => AvatarInfo::new("SmalltownBoy_0", "SmalltownBoy", 0, false),
    1759u32 => AvatarInfo::new("SmalltownBoy_0_Gold", "SmalltownBoy", 0, true),
    1760u32 => AvatarInfo::new("BlackMam_2_Gold", "BlackMam", 2, true),
    1761u32 => AvatarInfo::new("SaveYourTears_0", "SaveYourTears", 0, false),
    1762u32 => AvatarInfo::new("SaveYourTears_1", "SaveYourTears", 1, false),
    1763u32 => AvatarInfo::new("SaveYourTears_0_Gold", "SaveYourTears", 0, true),
    1764u32 => AvatarInfo::new("Levitating_0", "Levitating", 0, false),
    1765u32 => AvatarInfo::new("Levitating_0_Gold", "Levitating", 0, true),
    1766u32 => AvatarInfo::new("NailsHips_0", "NailsHips", 0, false),
    1767u32 => AvatarInfo::new("NailsHips_0_Gold", "NailsHips", 0, true),
    1768u32 => AvatarInfo::new("NailsHipsJD_0", "NailsHipsJD", 0, false),
    1769u32 => AvatarInfo::new("NailsHipsJD_0_Gold", "NailsHipsJD", 0, true),
    1770u32 => AvatarInfo::new("HappierThanEver_0", "HappierThanEver", 0, false),
    1771u32 => AvatarInfo::new("HappierThanEver_0_Gold", "HappierThanEver", 0, true),
    1772u32 => AvatarInfo::new("BuildAB_0", "BuildAB", 0, false),
    1773u32 => AvatarInfo::new("BuildAB_0_Gold", "BuildAB", 0, true),
    1774u32 => AvatarInfo::new("ChandelierALT_0", "ChandelierALT", 0, false),
    1775u32 => AvatarInfo::new("ChandelierALT_0_Gold", "ChandelierALT", 0, true),
    1776u32 => AvatarInfo::new("LevitatingALT_0", "LevitatingALT", 0, false),
    1777u32 => AvatarInfo::new("LevitatingALT_0_Gold", "LevitatingALT", 0, true),
};
