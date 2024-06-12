//! # Avatars
//! Import all the avatars.
//!
//! Current implementation is a bit wonky. A better option would be too manually match all avatar ids
//! to names per game. Then Just Dance 2017 avatars can also be imported.
use std::{borrow::Cow, collections::HashMap, fs::File, io::Write, sync::OnceLock};

use anyhow::{anyhow, Context, Error};
use dotstar_toolkit_utils::{
    bytes::read::BinaryDeserialize,
    testing::{test_eq, test_not},
    vfs::{VirtualFile, VirtualPath},
};
use ubiart_toolkit::{
    cooked,
    json_types::{
        v17::AvatarDescription17, v1819::AvatarDescription1819, v22::AvatarDescription2022,
        AvatarsObjectives,
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
fn load_avatar_config(is: &ImportState<'_>) -> Result<HashMap<String, Avatar<'static>>, Error> {
    let avatars_config_path = is.dirs.avatars().join("avatars.json");
    if avatars_config_path.exists() {
        let file = File::open(&avatars_config_path)?;
        Ok(serde_json::from_reader(file)?)
    } else {
        Ok(HashMap::new())
    }
}

/// Save the avatar metadata to avatars.json
fn save_avatar_config(
    is: &ImportState<'_>,
    avatars: &HashMap<String, Avatar>,
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
    phone_image: &str,
) -> Result<(), Error> {
    let avatar_named_dir_path = is.dirs.avatars().join(name);
    std::fs::create_dir(&avatar_named_dir_path).with_context(|| {
        format!("Tried to create {avatar_named_dir_path:?}, but it already exists!")
    })?;
    let alt_actor_file = is
        .vfs
        .open(cook_path(actor_path, is.ugi.platform)?.as_ref())?;
    let alt_actor = cooked::act::Actor::deserialize_with(&alt_actor_file, is.ugi)?;

    let image_actor = alt_actor
        .components
        .first()
        .ok_or_else(|| anyhow!("No templates in {}", actor_path))?;
    let mtg = image_actor.material_graphic_component()?;

    // Save decooked image
    let image_path = mtg.files[0].to_string();
    test_not(image_path.is_empty())?;
    let cooked_image_path = cook_path(&image_path, is.ugi.platform)?;
    let decooked_image = decode_texture(&is.vfs.open(cooked_image_path.as_ref())?, is.ugi)
        .with_context(|| format!("Failed to decode {cooked_image_path}!"))?;
    let avatar_image_path = is.dirs.avatars().join(avatar.image_path.as_ref());
    decooked_image.save(&avatar_image_path)?;

    // Save phone image
    let avatar_image_phone_path = is.dirs.avatars().join(avatar.image_phone_path.as_ref());
    let mut avatar_image_phone_file = File::create(avatar_image_phone_path)?;
    avatar_image_phone_file.write_all(&is.vfs.open(phone_image.as_ref())?)?;

    Ok(())
}

/// Minimum information to correctly parse an avatar
struct MinAvatarDesc<'a> {
    /// The id in the current game
    pub avatar_id: u16,
    /// The sound effect
    pub sound_family: Cow<'a, str>,
    /// Unkown, so better keep it around
    pub status: u8,
    /// How is it unlocked in the game
    pub unlock_type: u8,
    /// Path to the actor
    pub actor_path: Cow<'a, str>,
    /// Path to the image for the phone controller
    pub phone_image: Cow<'a, str>,
}

impl<'a> From<AvatarDescription2022<'a>> for MinAvatarDesc<'a> {
    fn from(value: AvatarDescription2022<'a>) -> Self {
        Self {
            avatar_id: value.avatar_id,
            sound_family: value.sound_family,
            status: value.status,
            unlock_type: value.unlock_type,
            actor_path: value.actor_path,
            phone_image: value.phone_image,
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
            phone_image: value.phone_image,
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
            phone_image: value.phone_image,
        }
    }
}

/// Parse the avatar database scene for v20-v22
fn parse_actor_v20v22<'a>(
    is: &ImportState,
    file: &'a VirtualFile,
) -> Result<MinAvatarDesc<'a>, Error> {
    let template = cooked::json::parse_v22(file, is.lax)?;
    let mut actor_template = template.into_actor()?;
    test_eq(&actor_template.components.len(), &2).context("Not exactly two components in actor")?;
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
    let template = cooked::json::parse_v19(file, is.lax)?;
    let mut actor_template = template.into_actor()?;
    test_eq(&actor_template.components.len(), &2).context("Not exactly two components in actor")?;
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
    let template = cooked::json::parse_v17(file, is.lax)?;
    let mut actor_template = template.into_actor()?;
    test_eq(&actor_template.components.len(), &2).context("Not exactly two components in actor")?;
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

    let mut avatars = load_avatar_config(is)?;

    // Open the avatardb and avatarsobjectives (which might be empty)
    let avatardb_file = is
        .vfs
        .open(cook_path(avatardb_scene, is.ugi.platform)?.as_ref())?;
    let avatardb = cooked::isc::parse(&avatardb_file)?;
    let avatarsobjectives = avatarsobjectives.unwrap_or(&empty_objectives);
    let mut avatar_description_files = Vec::with_capacity(avatardb.scene.actors.len());

    for actor in avatardb.scene.actors {
        let actor = actor.actor()?;

        // Extract avatar description from template
        let file = is
            .vfs
            .open(cook_path(actor.lua.as_ref(), is.ugi.platform)?.as_ref())?;
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
            _ => todo!(),
        };

        let Ok(avatar_info) = get_name(is.ugi.game, avatar_desc.avatar_id) else {
            continue;
        };

        let name = avatar_info.name;

        // Only add new avatars
        if !avatars.contains_key(name) {
            let avatar_image_path = format!("{name}/avatar.png");
            let avatar_image_phone_path = format!("{name}/avatar_phone.png");

            let main_avatar = avatar_info.main_avatar().map(Cow::Borrowed);

            let avatar = Avatar {
                relative_song_name: Cow::Borrowed(avatar_info.map),
                sound_family: avatar_desc.sound_family,
                status: avatar_desc.status,
                unlock_type: UnlockType::from_unlock_type(
                    avatar_desc.unlock_type,
                    avatarsobjectives.get(&avatar_desc.avatar_id),
                )?,
                used_as_coach_map_name: Cow::Borrowed(avatar_info.map),
                used_as_coach_coach_id: avatar_info.coach,
                special_effect: avatar_info.special_effect,
                main_avatar,
                image_path: avatar_image_path.into(),
                image_phone_path: avatar_image_phone_path.into(),
                guessed: false,
            };

            save_images(
                is,
                name,
                &avatar,
                &avatar_desc.actor_path,
                &avatar_desc.phone_image,
            )?;

            avatars.insert(name.to_string(), avatar);
        }
    }

    import_unreferenced_avatars(is, &mut avatars)?;

    save_avatar_config(is, &avatars)?;

    Ok(())
}

/// Imports avatars that were not in the avatar database
fn import_unreferenced_avatars(
    is: &ImportState<'_>,
    avatars: &mut HashMap<String, Avatar>,
) -> Result<(), Error> {
    let import_path = cook_path("world/avatars/", is.ugi.platform)?;
    for avatar_id in is
        .vfs
        .walk_filesystem(import_path.as_ref())?
        .filter(|p| p.file_name().is_some_and(|s| s.ends_with("avatar.png.ckd")))
        .filter_map(VirtualPath::parent)
        .filter_map(VirtualPath::file_name)
        .flat_map(str::parse::<u16>)
    {
        let avatar_info = match get_name(is.ugi.game, avatar_id) {
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
            let avatar_image_phone_path = format!("{name}/avatar_phone.png");

            let main_avatar = avatar_info.main_avatar().map(Cow::Borrowed);

            let avatar = Avatar {
                relative_song_name: Cow::Borrowed(avatar_info.map),
                sound_family: Cow::Borrowed("AVTR_Common_Brand"),
                status: 1,
                unlock_type: UnlockType::Unlocked,
                used_as_coach_map_name: Cow::Borrowed(avatar_info.map),
                used_as_coach_coach_id: avatar_info.coach,
                special_effect: avatar_info.special_effect,
                main_avatar,
                image_path: avatar_image_path.into(),
                image_phone_path: avatar_image_phone_path.into(),
                guessed: true,
            };

            // Save decooked image
            let cooked_image_path = cook_path(
                &format!("world/avatars/{avatar_id:0>4}/avatar.png"),
                is.ugi.platform,
            )?;
            let decooked_image = decode_texture(&is.vfs.open(cooked_image_path.as_ref())?, is.ugi)?;
            let avatar_image_path = is.dirs.avatars().join(avatar.image_path.as_ref());
            decooked_image.save(&avatar_image_path)?;

            // Save the phone image if it exists, otherwise save the decooked image as the phone image
            let avatar_image_phone_path = is.dirs.avatars().join(avatar.image_phone_path.as_ref());
            if let Ok(file) = is
                .vfs
                .open(format!("word/avatars/{avatar_id:0>4}/avatar_phone.png").as_ref())
            {
                let mut avatar_image_phone_file = File::create(&avatar_image_phone_path)?;
                avatar_image_phone_file.write_all(&file)?;
            } else {
                decooked_image.save(&avatar_image_phone_path)?;
            }

            avatars.insert(name.to_string(), avatar);
        }
    }

    Ok(())
}

/// Maps avatar ids to their proper names for each game
static GAME_AVATAR_ID_NAME_MAP: OnceLock<HashMap<Game, HashMap<u16, AvatarInfo>>> = OnceLock::new();

/// Get the name for the `avatar_id` for `game`
fn get_name(game: Game, avatar_id: u16) -> Result<AvatarInfo, String> {
    get_map()
        .get(&game)
        .ok_or_else(|| format!("Unsupported game for avatar parsing: {game}"))?
        .get(&avatar_id)
        .copied()
        .ok_or_else(|| format!("Unknown Avatar ID: {avatar_id}"))
}

/// Get the static map which maps avatar ids to their proper names for each game
fn get_map() -> &'static HashMap<Game, HashMap<u16, AvatarInfo>> {
    GAME_AVATAR_ID_NAME_MAP.get_or_init(|| {
        HashMap::from([
            (
                Game::JustDance2022,
                HashMap::from([
                    (1, AvatarInfo::new("Dare_0", "Dare", 0, false)),
                    (2, AvatarInfo::new("DogsOut_0", "DogsOut", 0, false)),
                    (
                        3,
                        AvatarInfo::new("EyeOfTheTiger_0", "EyeOfTheTiger", 0, false),
                    ),
                    (4, AvatarInfo::new("GetAround_0", "GetAround", 0, false)),
                    (5, AvatarInfo::new("HotNCold_0", "HotNCold", 0, false)),
                    (
                        6,
                        AvatarInfo::new("ILikeToMoveIt_0", "ILikeToMoveIt", 0, false),
                    ),
                    (7, AvatarInfo::new("JinGoLoBa_0", "JinGoLoBa", 0, false)),
                    (8, AvatarInfo::new("RingMyBell_0", "RingMyBell", 0, false)),
                    (
                        9,
                        AvatarInfo::new("NineAfternoon_0", "NineAfternoon", 0, false),
                    ),
                    (10, AvatarInfo::new("BabyGirl_0", "BabyGirl", 0, false)),
                    (11, AvatarInfo::new("CallMe_0", "CallMe", 0, false)),
                    (
                        12,
                        AvatarInfo::new("ChickenPayback_0", "ChickenPayback", 0, false),
                    ),
                    (13, AvatarInfo::new("ComeOn_1", "ComeOn", 1, false)),
                    (14, AvatarInfo::new("CosmicGirl_0", "CosmicGirl", 0, false)),
                    (
                        15,
                        AvatarInfo::new("ElectroTribalDLC_0", "ElectroTribalDLC", 0, false),
                    ),
                    (16, AvatarInfo::new("Firework_0", "Firework", 0, false)),
                    (
                        76,
                        AvatarInfo::new("EverybodyNeeds_0", "EverybodyNeeds", 0, false),
                    ),
                    (
                        347,
                        AvatarInfo::new("GangnamStyleDLC_0", "GangnamStyleDLC", 0, false),
                    ),
                    (
                        717,
                        AvatarInfo::new("FearlessPirateKids_0", "FearlessPirateKids", 0, false),
                    ),
                    (
                        816,
                        AvatarInfo::new("FunkyRobotKids_0", "FunkyRobotKids", 0, false),
                    ),
                    (
                        907,
                        AvatarInfo::new("MonstersAcademyKids_0", "MonstersAcademyKids", 0, false),
                    ),
                    (922, AvatarInfo::new("MiMiMiALT_3", "MiMiMiALT", 3, false)),
                    (933, AvatarInfo::new("Fire_1", "Fire", 1, false)),
                    (953, AvatarInfo::new("OMG_2", "OMG", 2, false)),
                    (
                        1062,
                        AvatarInfo::new("MedievalKids_0", "MedievalKids", 0, false),
                    ),
                    (
                        1063,
                        AvatarInfo::new("MedievalKids_1", "MedievalKids", 1, false),
                    ),
                    (1064, AvatarInfo::new("ChefKids_0", "ChefKids", 0, false)),
                    (
                        1065,
                        AvatarInfo::new("AdventurerKids_0", "AdventurerKids", 0, false),
                    ),
                    (
                        1397,
                        AvatarInfo::new("FiremenKids_0", "FiremenKids", 0, false),
                    ),
                    (
                        1398,
                        AvatarInfo::new("FiremenKids_1", "FiremenKids", 1, false),
                    ),
                    (
                        1402,
                        AvatarInfo::new("BalletKids_0", "BalletKids", 0, false),
                    ),
                    (
                        1403,
                        AvatarInfo::new("BalletKids_1", "BalletKids", 1, false),
                    ),
                    (1467, AvatarInfo::new("WhoRun_0", "WhoRun", 0, false)),
                    (1468, AvatarInfo::new("WhoRun_1", "WhoRun", 1, false)),
                    (1469, AvatarInfo::new("WhoRun_2", "WhoRun", 2, false)),
                    (1470, AvatarInfo::new("WhoRun_1_Gold", "WhoRun", 1, true)),
                    (
                        1517,
                        AvatarInfo::new("WhoRunALTRETAKE_0", "WhoRunALTRETAKE", 0, false),
                    ),
                    (
                        1518,
                        AvatarInfo::new("WhoRunALT_0_Gold", "WhoRunALT", 0, true),
                    ),
                    (
                        1619,
                        AvatarInfo::new("FreedFromDesire_0", "FreedFromDesire", 0, false),
                    ),
                    (
                        1620,
                        AvatarInfo::new("FreedFromDesire_0_Gold", "FreedFromDesire", 0, true),
                    ),
                    (1633, AvatarInfo::new("China_0", "China", 0, false)),
                    (1634, AvatarInfo::new("China_1", "China", 1, false)),
                    (1635, AvatarInfo::new("China_2", "China", 2, false)),
                    (1636, AvatarInfo::new("China_2_Gold", "China", 2, true)),
                    (
                        1638,
                        AvatarInfo::new("BreakMyHeart_0", "BreakMyHeart", 0, false),
                    ),
                    (
                        1639,
                        AvatarInfo::new("BreakMyHeart_0_Gold", "BreakMyHeart", 0, true),
                    ),
                    (1641, AvatarInfo::new("FlashPose_0", "FlashPose", 0, false)),
                    (1642, AvatarInfo::new("FlashPose_1", "FlashPose", 1, false)),
                    (1643, AvatarInfo::new("FlashPose_2", "FlashPose", 2, false)),
                    (
                        1644,
                        AvatarInfo::new("FlashPose_0_Gold", "FlashPose", 0, true),
                    ),
                    (1645, AvatarInfo::new("Siargo_0", "Siargo", 0, false)),
                    (1646, AvatarInfo::new("Siargo_0_Gold", "Siargo", 0, true)),
                    (1647, AvatarInfo::new("Human_0", "Human", 0, false)),
                    (1648, AvatarInfo::new("Human_0_Gold", "Human", 0, true)),
                    (
                        1649,
                        AvatarInfo::new("YouCanDance_0", "YouCanDance", 0, false),
                    ),
                    (
                        1650,
                        AvatarInfo::new("YouCanDance_0_Gold", "YouCanDance", 0, true),
                    ),
                    (1651, AvatarInfo::new("Funk_0", "Funk", 0, false)),
                    (1652, AvatarInfo::new("Funk_1", "Funk", 1, false)),
                    (1653, AvatarInfo::new("Funk_0_Gold", "Funk", 0, true)),
                    (1654, AvatarInfo::new("Believer_0", "Believer", 0, false)),
                    (1655, AvatarInfo::new("Believer_1", "Believer", 1, false)),
                    (
                        1656,
                        AvatarInfo::new("Believer_0_Gold", "Believer", 0, true),
                    ),
                    (1657, AvatarInfo::new("TGIF_0", "TGIF", 0, false)),
                    (1658, AvatarInfo::new("TGIF_0_Gold", "TGIF", 0, true)),
                    (1659, AvatarInfo::new("Jopping_0", "Jopping", 0, false)),
                    (1660, AvatarInfo::new("Jopping_1", "Jopping", 1, false)),
                    (1661, AvatarInfo::new("Jopping_2", "Jopping", 2, false)),
                    (1662, AvatarInfo::new("Jopping_2_Gold", "Jopping", 2, true)),
                    (1668, AvatarInfo::new("LevelUp_0", "LevelUp", 0, false)),
                    (1669, AvatarInfo::new("LevelUp_1", "LevelUp", 1, false)),
                    (1670, AvatarInfo::new("LevelUp_2", "LevelUp", 2, false)),
                    (1671, AvatarInfo::new("LevelUp_1_Gold", "LevelUp", 1, true)),
                    (1672, AvatarInfo::new("BossWitch_0", "BossWitch", 0, false)),
                    (
                        1673,
                        AvatarInfo::new("BossWitch_0_Gold", "BossWitch", 0, true),
                    ),
                    (
                        1674,
                        AvatarInfo::new("FollowTheWhiteRabbit_0", "FollowTheWhiteRabbit", 0, false),
                    ),
                    (
                        1675,
                        AvatarInfo::new(
                            "FollowTheWhiteRabbit_0_Gold",
                            "FollowTheWhiteRabbit",
                            0,
                            true,
                        ),
                    ),
                    (1676, AvatarInfo::new("Popstars_0", "Popstars", 0, false)),
                    (1677, AvatarInfo::new("Popstars_1", "Popstars", 1, false)),
                    (1678, AvatarInfo::new("Popstars_2", "Popstars", 2, false)),
                    (1679, AvatarInfo::new("Popstars_3", "Popstars", 3, false)),
                    (
                        1680,
                        AvatarInfo::new("Popstars_3_Gold", "Popstars", 3, true),
                    ),
                    (1681, AvatarInfo::new("Mood_0", "Mood", 0, false)),
                    (1682, AvatarInfo::new("Mood_1", "Mood", 1, false)),
                    (1683, AvatarInfo::new("Mood_1_Gold", "Mood", 1, true)),
                    (
                        1684,
                        AvatarInfo::new("JoppingALT_0", "JoppingALT", 0, false),
                    ),
                    (
                        1685,
                        AvatarInfo::new("JoppingALT_0_Gold", "JoppingALT", 0, true),
                    ),
                    (1686, AvatarInfo::new("BlackMam_0", "BlackMam", 0, false)),
                    (1687, AvatarInfo::new("BlackMam_1", "BlackMam", 1, false)),
                    (1688, AvatarInfo::new("BlackMam_2", "BlackMam", 2, false)),
                    (
                        1689,
                        AvatarInfo::new("BlackMam_1_Gold", "BlackMam", 1, true),
                    ),
                    (
                        1690,
                        AvatarInfo::new("BlackMamALT_0", "BlackMamALT", 0, false),
                    ),
                    (
                        1691,
                        AvatarInfo::new("BlackMamALT_0_Gold", "BlackMamALT", 0, true),
                    ),
                    (
                        1692,
                        AvatarInfo::new("ImOuttaLove_0", "ImOuttaLove", 0, false),
                    ),
                    (
                        1693,
                        AvatarInfo::new("ImOuttaLove_0_Gold", "ImOuttaLove", 0, true),
                    ),
                    (1694, AvatarInfo::new("Boombayah_0", "Boombayah", 0, false)),
                    (1695, AvatarInfo::new("Boombayah_1", "Boombayah", 1, false)),
                    (1696, AvatarInfo::new("Boombayah_2", "Boombayah", 2, false)),
                    (1697, AvatarInfo::new("Boombayah_3", "Boombayah", 3, false)),
                    (
                        1698,
                        AvatarInfo::new("Boombayah_2_Gold", "Boombayah", 2, true),
                    ),
                    (1699, AvatarInfo::new("Baiana_0", "Baiana", 0, false)),
                    (1700, AvatarInfo::new("Baiana_0_Gold", "Baiana", 0, true)),
                    (1702, AvatarInfo::new("Malibu_0", "Malibu", 0, false)),
                    (1703, AvatarInfo::new("Malibu_1", "Malibu", 1, false)),
                    (1704, AvatarInfo::new("Malibu_2", "Malibu", 2, false)),
                    (
                        1705,
                        AvatarInfo::new("RockYourBody_0", "RockYourBody", 0, false),
                    ),
                    (
                        1706,
                        AvatarInfo::new("RockYourBody_0_Gold", "RockYourBody", 0, true),
                    ),
                    (
                        1707,
                        AvatarInfo::new("Jerusalema_0", "Jerusalema", 0, false),
                    ),
                    (
                        1708,
                        AvatarInfo::new("Jerusalema_1", "Jerusalema", 1, false),
                    ),
                    (
                        1709,
                        AvatarInfo::new("Jerusalema_2", "Jerusalema", 2, false),
                    ),
                    (
                        1710,
                        AvatarInfo::new("Jerusalema_3", "Jerusalema", 3, false),
                    ),
                    (
                        1711,
                        AvatarInfo::new("Jerusalema_3_Gold", "Jerusalema", 3, true),
                    ),
                    (1712, AvatarInfo::new("Judas_0", "Judas", 0, false)),
                    (1713, AvatarInfo::new("Judas_0_Gold", "Judas", 0, true)),
                    (1714, AvatarInfo::new("Buttons_0", "Buttons", 0, false)),
                    (1716, AvatarInfo::new("Buttons_2", "Buttons", 2, false)),
                    (1718, AvatarInfo::new("ChinaALT_0", "ChinaALT", 0, false)),
                    (1719, AvatarInfo::new("ChinaALT_1", "ChinaALT", 1, false)),
                    (
                        1720,
                        AvatarInfo::new("ChinaALT_1_Gold", "ChinaALT", 1, true),
                    ),
                    (1721, AvatarInfo::new("LoveStory_0", "LoveStory", 0, false)),
                    (1722, AvatarInfo::new("LoveStory_1", "LoveStory", 1, false)),
                    (
                        1723,
                        AvatarInfo::new("LoveStory_0_Gold", "LoveStory", 0, true),
                    ),
                    (
                        1724,
                        AvatarInfo::new("GirlLikeMe_0", "GirlLikeMe", 0, false),
                    ),
                    (
                        1725,
                        AvatarInfo::new("GirlLikeMe_1", "GirlLikeMe", 1, false),
                    ),
                    (
                        1726,
                        AvatarInfo::new("GirlLikeMe_2", "GirlLikeMe", 2, false),
                    ),
                    (
                        1727,
                        AvatarInfo::new("GirlLikeMe_3", "GirlLikeMe", 3, false),
                    ),
                    (
                        1728,
                        AvatarInfo::new("GirlLikeMe_3_Gold", "GirlLikeMe", 3, true),
                    ),
                    (
                        1729,
                        AvatarInfo::new("Chandelier_0", "Chandelier", 0, false),
                    ),
                    (
                        1730,
                        AvatarInfo::new("Chandelier_0_Gold", "Chandelier", 0, true),
                    ),
                    (1731, AvatarInfo::new("SuaCara_0", "SuaCara", 0, false)),
                    (1732, AvatarInfo::new("SuaCara_0_Gold", "SuaCara", 0, true)),
                    (
                        1733,
                        AvatarInfo::new("BoombayahALT_0", "BoombayahALT", 0, false),
                    ),
                    (
                        1734,
                        AvatarInfo::new("BoombayahALT_0_Gold", "BoombayahALT", 0, true),
                    ),
                    (
                        1737,
                        AvatarInfo::new("GirlLikeMeALT_0", "GirlLikeMeALT", 0, false),
                    ),
                    (
                        1738,
                        AvatarInfo::new("GirlLikeMeALT_0_Gold", "GirlLikeMeALT", 0, true),
                    ),
                    (
                        1739,
                        AvatarInfo::new("StopDropAndRoll_0", "StopDropAndRoll", 0, false),
                    ),
                    (
                        1740,
                        AvatarInfo::new("StopDropAndRoll_1", "StopDropAndRoll", 1, false),
                    ),
                    (
                        1741,
                        AvatarInfo::new("StopDropAndRoll_2", "StopDropAndRoll", 2, false),
                    ),
                    (
                        1742,
                        AvatarInfo::new("StopDropAndRoll_3", "StopDropAndRoll", 3, false),
                    ),
                    (
                        1743,
                        AvatarInfo::new("StopDropAndRoll_2_Gold", "StopDropAndRoll", 2, true),
                    ),
                    (
                        1744,
                        AvatarInfo::new("MightyReal_0", "MightyReal", 0, false),
                    ),
                    (
                        1745,
                        AvatarInfo::new("MightyReal_0_Gold", "MightyReal", 0, true),
                    ),
                    (1746, AvatarInfo::new("MrBlueSky_0", "MrBlueSky", 0, false)),
                    (
                        1747,
                        AvatarInfo::new("MrBlueSky_0_Gold", "MrBlueSky", 0, true),
                    ),
                    (
                        1748,
                        AvatarInfo::new("ThinkAboutThings_0", "ThinkAboutThings", 0, false),
                    ),
                    (
                        1749,
                        AvatarInfo::new("ThinkAboutThings_0_Gold", "ThinkAboutThings", 0, true),
                    ),
                    (1750, AvatarInfo::new("Chacarron_0", "Chacarron", 0, false)),
                    (
                        1751,
                        AvatarInfo::new("Chacarron_0_Gold", "Chacarron", 0, true),
                    ),
                    (
                        1752,
                        AvatarInfo::new("SuaCaraALT_0", "SuaCaraALT", 0, false),
                    ),
                    (
                        1753,
                        AvatarInfo::new("SuaCaraALT_1", "SuaCaraALT", 1, false),
                    ),
                    (
                        1754,
                        AvatarInfo::new("SuaCaraALT_0_Gold", "SuaCaraALT", 0, true),
                    ),
                    (
                        1755,
                        AvatarInfo::new("Boombayah_3_Gold", "Boombayah", 3, true),
                    ),
                    (
                        1756,
                        AvatarInfo::new("Believer_0_Gold", "Believer", 0, true),
                    ),
                    (
                        1757,
                        AvatarInfo::new("ChinaALT_0_Gold", "ChinaALT", 0, true),
                    ),
                    (
                        1758,
                        AvatarInfo::new("SmalltownBoy_0", "SmalltownBoy", 0, false),
                    ),
                    (
                        1759,
                        AvatarInfo::new("SmalltownBoy_0_Gold", "SmalltownBoy", 0, true),
                    ),
                    (
                        1760,
                        AvatarInfo::new("BlackMam_2_Gold", "BlackMam", 2, true),
                    ),
                    (
                        1761,
                        AvatarInfo::new("SaveYourTears_0", "SaveYourTears", 0, false),
                    ),
                    (
                        1762,
                        AvatarInfo::new("SaveYourTears_1", "SaveYourTears", 1, false),
                    ),
                    (
                        1763,
                        AvatarInfo::new("SaveYourTears_0_Gold", "SaveYourTears", 0, true),
                    ),
                    (
                        1764,
                        AvatarInfo::new("Levitating_0", "Levitating", 0, false),
                    ),
                    (
                        1765,
                        AvatarInfo::new("Levitating_0_Gold", "Levitating", 0, true),
                    ),
                    (1766, AvatarInfo::new("NailsHips_0", "NailsHips", 0, false)),
                    (
                        1767,
                        AvatarInfo::new("NailsHips_0_Gold", "NailsHips", 0, true),
                    ),
                    (
                        1768,
                        AvatarInfo::new("NailsHipsJD_0", "NailsHipsJD", 0, false),
                    ),
                    (
                        1769,
                        AvatarInfo::new("NailsHipsJD_0_Gold", "NailsHipsJD", 0, true),
                    ),
                    (
                        1770,
                        AvatarInfo::new("HappierThanEver_0", "HappierThanEver", 0, false),
                    ),
                    (
                        1771,
                        AvatarInfo::new("HappierThanEver_0_Gold", "HappierThanEver", 0, true),
                    ),
                    (1772, AvatarInfo::new("BuildAB_0", "BuildAB", 0, false)),
                    (1773, AvatarInfo::new("BuildAB_0_Gold", "BuildAB", 0, true)),
                    (
                        1774,
                        AvatarInfo::new("ChandelierALT_0", "ChandelierALT", 0, false),
                    ),
                    (
                        1775,
                        AvatarInfo::new("ChandelierALT_0_Gold", "ChandelierALT", 0, true),
                    ),
                    (
                        1776,
                        AvatarInfo::new("LevitatingALT_0", "LevitatingALT", 0, false),
                    ),
                    (
                        1777,
                        AvatarInfo::new("LevitatingALT_0_Gold", "LevitatingALT", 0, true),
                    ),
                ]),
            ),
            (
                Game::JustDance2021,
                HashMap::from([
                    (1, AvatarInfo::new("Dare_0", "Dare", 0, false)),
                    (2, AvatarInfo::new("DogsOut_0", "DogsOut", 0, false)),
                    (
                        3,
                        AvatarInfo::new("EyeOfTheTiger_0", "EyeOfTheTiger", 0, false),
                    ),
                    (4, AvatarInfo::new("GetAround_0", "GetAround", 0, false)),
                    (5, AvatarInfo::new("HotNCold_0", "HotNCold", 0, false)),
                    (
                        6,
                        AvatarInfo::new("ILikeToMoveIt_0", "ILikeToMoveIt", 0, false),
                    ),
                    (7, AvatarInfo::new("JinGoLoBa_0", "JinGoLoBa", 0, false)),
                    (8, AvatarInfo::new("RingMyBell_0", "RingMyBell", 0, false)),
                    (
                        9,
                        AvatarInfo::new("NineAfternoon_0", "NineAfternoon", 0, false),
                    ),
                    (10, AvatarInfo::new("BabyGirl_0", "BabyGirl", 0, false)),
                    (11, AvatarInfo::new("CallMe_0", "CallMe", 0, false)),
                    (
                        12,
                        AvatarInfo::new("ChickenPayback_0", "ChickenPayback", 0, false),
                    ),
                    (13, AvatarInfo::new("ComeOn_1", "ComeOn", 1, false)),
                    (14, AvatarInfo::new("CosmicGirl_0", "CosmicGirl", 0, false)),
                    (
                        15,
                        AvatarInfo::new("ElectroTribalDLC_0", "ElectroTribalDLC", 0, false),
                    ),
                    (16, AvatarInfo::new("Firework_0", "Firework", 0, false)),
                    (
                        76,
                        AvatarInfo::new("EverybodyNeeds_0", "EverybodyNeeds", 0, false),
                    ),
                    (
                        347,
                        AvatarInfo::new("GangnamStyleDLC_0", "GangnamStyleDLC", 0, false),
                    ),
                    (922, AvatarInfo::new("MiMiMiALT_3", "MiMiMiALT", 3, false)),
                    (933, AvatarInfo::new("Fire_1", "Fire", 1, false)),
                    (953, AvatarInfo::new("OMG_2", "OMG", 2, false)),
                    (
                        1393,
                        AvatarInfo::new("BadAssPrincessKids_0", "BadAssPrincessKids", 0, false),
                    ),
                    (1394, AvatarInfo::new("SpyKids_0", "SpyKids", 0, false)),
                    (
                        1395,
                        AvatarInfo::new("ChasmonauteKids_0", "ChasmonauteKids", 0, false),
                    ),
                    (
                        1396,
                        AvatarInfo::new("BubblesKids_0", "BubblesKids", 0, false),
                    ),
                    (
                        1397,
                        AvatarInfo::new("FiremenKids_0", "FiremenKids", 0, false),
                    ),
                    (
                        1398,
                        AvatarInfo::new("FiremenKids_1", "FiremenKids", 1, false),
                    ),
                    (1399, AvatarInfo::new("EcoloKids_1", "EcoloKids", 1, false)),
                    (1400, AvatarInfo::new("EcoloKids_0", "EcoloKids", 0, false)),
                    (
                        1402,
                        AvatarInfo::new("BalletKids_0", "BalletKids", 0, false),
                    ),
                    (
                        1403,
                        AvatarInfo::new("BalletKids_1", "BalletKids", 1, false),
                    ),
                    (
                        1407,
                        AvatarInfo::new("CarpetKids_0", "CarpetKids", 0, false),
                    ),
                    (1408, AvatarInfo::new("Senorita_0", "Senorita", 0, false)),
                    (1409, AvatarInfo::new("Senorita_1", "Senorita", 1, false)),
                    (
                        1411,
                        AvatarInfo::new("Senorita_1_Gold", "Senorita", 1, true),
                    ),
                    (
                        1412,
                        AvatarInfo::new("Bailando1997_0", "Bailando1997", 0, false),
                    ),
                    (
                        1413,
                        AvatarInfo::new("Bailando1997_0_Gold", "Bailando1997", 0, true),
                    ),
                    (
                        1414,
                        AvatarInfo::new("HabibiYaeni_0", "HabibiYaeni", 0, false),
                    ),
                    (
                        1415,
                        AvatarInfo::new("HabibiYaeni_1", "HabibiYaeni", 1, false),
                    ),
                    (
                        1416,
                        AvatarInfo::new("HabibiYaeni_2", "HabibiYaeni", 2, false),
                    ),
                    (
                        1417,
                        AvatarInfo::new("HabibiYaeni_0_Gold", "HabibiYaeni", 0, true),
                    ),
                    (
                        1421,
                        AvatarInfo::new("QueTirePaLante_0", "QueTirePaLante", 0, false),
                    ),
                    (
                        1422,
                        AvatarInfo::new("QueTirePaLante_1", "QueTirePaLante", 1, false),
                    ),
                    (
                        1423,
                        AvatarInfo::new("QueTirePaLante_2", "QueTirePaLante", 2, false),
                    ),
                    (
                        1424,
                        AvatarInfo::new("QueTirePaLante_3", "QueTirePaLante", 3, false),
                    ),
                    (
                        1425,
                        AvatarInfo::new("QueTirePaLante_1_Gold", "QueTirePaLante", 1, true),
                    ),
                    (1426, AvatarInfo::new("InTheNavy_0", "InTheNavy", 0, false)),
                    (1427, AvatarInfo::new("InTheNavy_1", "InTheNavy", 1, false)),
                    (1428, AvatarInfo::new("InTheNavy_2", "InTheNavy", 2, false)),
                    (1429, AvatarInfo::new("InTheNavy_3", "InTheNavy", 3, false)),
                    (
                        1430,
                        AvatarInfo::new("InTheNavy_3_Gold", "InTheNavy", 3, true),
                    ),
                    (1435, AvatarInfo::new("Georgia_0", "Georgia", 0, false)),
                    (1436, AvatarInfo::new("Georgia_0_Gold", "Georgia", 0, true)),
                    (1437, AvatarInfo::new("Buscando_0", "Buscando", 0, false)),
                    (
                        1438,
                        AvatarInfo::new("Buscando_0_Gold", "Buscando", 0, true),
                    ),
                    (
                        1439,
                        AvatarInfo::new("SambaDeJaneiro_0", "SambaDeJaneiro", 0, false),
                    ),
                    (
                        1440,
                        AvatarInfo::new("SambaDeJaneiro_1", "SambaDeJaneiro", 1, false),
                    ),
                    (
                        1441,
                        AvatarInfo::new("SambaDeJaneiro_2", "SambaDeJaneiro", 2, false),
                    ),
                    (
                        1442,
                        AvatarInfo::new("SambaDeJaneiro_1_Gold", "SambaDeJaneiro", 1, true),
                    ),
                    (
                        1443,
                        AvatarInfo::new("TheWeekend_0", "TheWeekend", 0, false),
                    ),
                    (
                        1444,
                        AvatarInfo::new("TheWeekend_1", "TheWeekend", 1, false),
                    ),
                    (
                        1445,
                        AvatarInfo::new("TheWeekend_1_Gold", "TheWeekend", 1, true),
                    ),
                    (1446, AvatarInfo::new("WithoutMe_0", "WithoutMe", 0, false)),
                    (1447, AvatarInfo::new("WithoutMe_1", "WithoutMe", 1, false)),
                    (1473, AvatarInfo::new("RareALT_0", "RareALT", 0, false)),
                    (1474, AvatarInfo::new("RareALT_0_Gold", "RareALT", 0, true)),
                    (1448, AvatarInfo::new("WithoutMe_2", "WithoutMe", 2, false)),
                    (
                        1449,
                        AvatarInfo::new("WithoutMe_2_Gold", "WithoutMe", 2, true),
                    ),
                    (
                        1450,
                        AvatarInfo::new("DibbyDibby_0", "DibbyDibby", 0, false),
                    ),
                    (
                        1451,
                        AvatarInfo::new("DibbyDibby_1", "DibbyDibby", 1, false),
                    ),
                    (
                        1452,
                        AvatarInfo::new("DibbyDibby_0_Gold", "DibbyDibby", 0, true),
                    ),
                    (
                        1453,
                        AvatarInfo::new("Alexandrie_0", "Alexandrie", 0, false),
                    ),
                    (
                        1454,
                        AvatarInfo::new("Alexandrie_1", "Alexandrie", 1, false),
                    ),
                    (
                        1455,
                        AvatarInfo::new("Alexandrie_2", "Alexandrie", 2, false),
                    ),
                    (
                        1456,
                        AvatarInfo::new("Alexandrie_1_Gold", "Alexandrie", 1, true),
                    ),
                    (
                        1457,
                        AvatarInfo::new("SweetEscape_0", "SweetEscape", 0, false),
                    ),
                    (
                        1458,
                        AvatarInfo::new("SweetEscape_0_Gold", "SweetEscape", 0, true),
                    ),
                    (
                        1459,
                        AvatarInfo::new("HeatSeeker_0", "HeatSeeker", 0, false),
                    ),
                    (
                        1460,
                        AvatarInfo::new("HeatSeeker_0_Gold", "HeatSeeker", 0, true),
                    ),
                    (1461, AvatarInfo::new("Juice_0", "Juice", 0, false)),
                    (1462, AvatarInfo::new("Juice_1", "Juice", 1, false)),
                    (1463, AvatarInfo::new("Juice_2", "Juice", 2, false)),
                    (1464, AvatarInfo::new("Juice_1_Gold", "Juice", 1, true)),
                    (1465, AvatarInfo::new("Kuliki_0", "Kuliki", 0, false)),
                    (1466, AvatarInfo::new("Kuliki_0_Gold", "Kuliki", 0, true)),
                    (1467, AvatarInfo::new("WhoRun_0", "WhoRun", 0, false)),
                    (1468, AvatarInfo::new("WhoRun_1", "WhoRun", 1, false)),
                    (1469, AvatarInfo::new("WhoRun_2", "WhoRun", 2, false)),
                    (1470, AvatarInfo::new("WhoRun_1_Gold", "WhoRun", 1, true)),
                    (
                        1471,
                        AvatarInfo::new("WithoutMeALTRETAKE_0", "WithoutMeALTRETAKE", 0, false),
                    ),
                    (
                        1472,
                        AvatarInfo::new("WithoutMeALT_0_Gold", "WithoutMeALT", 0, true),
                    ),
                    (1473, AvatarInfo::new("RareALT_0", "RareALT", 0, false)),
                    (1474, AvatarInfo::new("RareALT_0_Gold", "RareALT", 0, true)),
                    (
                        1475,
                        AvatarInfo::new("DanceMonkey_0", "DanceMonkey", 0, false),
                    ),
                    (
                        1476,
                        AvatarInfo::new("DanceMonkey_0_Gold", "DanceMonkey", 0, true),
                    ),
                    (
                        1477,
                        AvatarInfo::new("HabibiYaeniALT_0", "HabibiYaeniALT", 0, false),
                    ),
                    (
                        1478,
                        AvatarInfo::new("HabibiYaeniALT_0_Gold", "HabibiYaeniALT", 0, true),
                    ),
                    (
                        1479,
                        AvatarInfo::new("DontStartALT_0", "DontStartALT", 0, false),
                    ),
                    (
                        1480,
                        AvatarInfo::new("DontStartALT_0_Gold", "DontStartALT", 0, true),
                    ),
                    (1481, AvatarInfo::new("JuiceALT_0", "JuiceALT", 0, false)),
                    (1482, AvatarInfo::new("JuiceALT_1", "JuiceALT", 1, false)),
                    (1483, AvatarInfo::new("JuiceALT_2", "JuiceALT", 2, false)),
                    (
                        1484,
                        AvatarInfo::new("JuiceALT_2_Gold", "JuiceALT", 2, true),
                    ),
                    (1485, AvatarInfo::new("Zenit_0", "Zenit", 0, false)),
                    (1486, AvatarInfo::new("Zenit_0_Gold", "Zenit", 0, true)),
                    (
                        1487,
                        AvatarInfo::new("BuscandoALT_0", "BuscandoALT", 0, false),
                    ),
                    (
                        1488,
                        AvatarInfo::new("BuscandoALT_0_Gold", "BuscandoALT", 0, true),
                    ),
                    (
                        1489,
                        AvatarInfo::new("SambaDeJaneiroALT_0", "SambaDeJaneiroALT", 0, false),
                    ),
                    (
                        1490,
                        AvatarInfo::new("SambaDeJaneiroALT_1", "SambaDeJaneiroALT", 1, false),
                    ),
                    (
                        1491,
                        AvatarInfo::new("SambaDeJaneiroALT_1_Gold", "SambaDeJaneiroALT", 1, true),
                    ),
                    (1492, AvatarInfo::new("Joone_0", "Joone", 0, false)),
                    (1493, AvatarInfo::new("Joone_1", "Joone", 1, false)),
                    (1494, AvatarInfo::new("Joone_1_Gold", "Joone", 1, true)),
                    (1500, AvatarInfo::new("AdoreYou_0", "AdoreYou", 0, false)),
                    (
                        1501,
                        AvatarInfo::new("AdoreYou_0_Gold", "AdoreYou", 0, true),
                    ),
                    (
                        1502,
                        AvatarInfo::new("FeelSpecialALT_0", "FeelSpecialALT", 0, false),
                    ),
                    (
                        1503,
                        AvatarInfo::new("FeelSpecialALT_0_Gold", "FeelSpecialALT", 0, true),
                    ),
                    (1504, AvatarInfo::new("Runaway_0", "Runaway", 0, false)),
                    (1505, AvatarInfo::new("Runaway_1", "Runaway", 1, false)),
                    (1506, AvatarInfo::new("Runaway_0_Gold", "Runaway", 0, true)),
                    (1507, AvatarInfo::new("Volar_0", "Volar", 0, false)),
                    (1508, AvatarInfo::new("Volar_0_Gold", "Volar", 0, true)),
                    (
                        1509,
                        AvatarInfo::new("GetGetDown_0", "GetGetDown", 0, false),
                    ),
                    (
                        1510,
                        AvatarInfo::new("GetGetDown_1", "GetGetDown", 1, false),
                    ),
                    (
                        1511,
                        AvatarInfo::new("GetGetDown_2", "GetGetDown", 2, false),
                    ),
                    (
                        1512,
                        AvatarInfo::new("GetGetDown_3", "GetGetDown", 3, false),
                    ),
                    (
                        1513,
                        AvatarInfo::new("GetGetDown_3_Gold", "GetGetDown", 3, true),
                    ),
                    (
                        1517,
                        AvatarInfo::new("WhoRunALTRETAKE_0", "WhoRunALTRETAKE", 0, false),
                    ),
                    (
                        1518,
                        AvatarInfo::new("WhoRunALT_0_Gold", "WhoRunALT", 0, true),
                    ),
                    (
                        1522,
                        AvatarInfo::new("AllTheGoodGirls_0", "AllTheGoodGirls", 0, false),
                    ),
                    (
                        1523,
                        AvatarInfo::new("AllTheGoodGirls_0_Gold", "AllTheGoodGirls", 0, true),
                    ),
                    (1524, AvatarInfo::new("PacaDance_0", "PacaDance", 0, false)),
                    (1525, AvatarInfo::new("PacaDance_1", "PacaDance", 1, false)),
                    (
                        1526,
                        AvatarInfo::new("PacaDance_1_Gold", "PacaDance", 1, true),
                    ),
                    (1527, AvatarInfo::new("Rare_0", "Rare", 0, false)),
                    (1528, AvatarInfo::new("Rare_0_Gold", "Rare", 0, true)),
                    (1529, AvatarInfo::new("BoyYouCan_0", "BoyYouCan", 0, false)),
                    (
                        1530,
                        AvatarInfo::new("BoyYouCan_0_Gold", "BoyYouCan", 0, true),
                    ),
                    (
                        1531,
                        AvatarInfo::new("FeelSpecial_0", "FeelSpecial", 0, false),
                    ),
                    (
                        1532,
                        AvatarInfo::new("FeelSpecial_1", "FeelSpecial", 1, false),
                    ),
                    (
                        1533,
                        AvatarInfo::new("FeelSpecial_2", "FeelSpecial", 2, false),
                    ),
                    (
                        1534,
                        AvatarInfo::new("FeelSpecial_1_Gold", "FeelSpecial", 1, true),
                    ),
                    (
                        1535,
                        AvatarInfo::new("InTheNavy_2_Gold", "InTheNavy", 2, true),
                    ),
                    (
                        1536,
                        AvatarInfo::new("WithoutMe_0_Gold", "WithoutMe", 0, true),
                    ),
                    (1537, AvatarInfo::new("Magenta_0", "Magenta", 0, false)),
                    (1538, AvatarInfo::new("Magenta_0_Gold", "Magenta", 0, true)),
                    (1539, AvatarInfo::new("Lacrimosa_0", "Lacrimosa", 0, false)),
                    (
                        1540,
                        AvatarInfo::new("Lacrimosa_0_Gold", "Lacrimosa", 0, true),
                    ),
                    (
                        1541,
                        AvatarInfo::new("TillTheWorldEndsALT_0", "TillTheWorldEndsALT", 0, false),
                    ),
                    (
                        1542,
                        AvatarInfo::new(
                            "TillTheWorldEndsALT_0_Gold",
                            "TillTheWorldEndsALT",
                            0,
                            true,
                        ),
                    ),
                    (
                        1543,
                        AvatarInfo::new("TillTheWorldEnds_0", "TillTheWorldEnds", 0, false),
                    ),
                    (
                        1544,
                        AvatarInfo::new("TillTheWorldEnds_0_Gold", "TillTheWorldEnds", 0, true),
                    ),
                    (1545, AvatarInfo::new("DontStart_0", "DontStart", 0, false)),
                    (
                        1546,
                        AvatarInfo::new("DontStart_0_Gold", "DontStart", 0, true),
                    ),
                    (1547, AvatarInfo::new("KickItALT_0", "KickItALT", 0, false)),
                    (
                        1548,
                        AvatarInfo::new("KickItALT_0_Gold", "KickItALT", 0, true),
                    ),
                    (
                        1549,
                        AvatarInfo::new("BlindingLightsALT_0", "BlindingLightsALT", 0, false),
                    ),
                    (
                        1550,
                        AvatarInfo::new("BlindingLightsALT_0_Gold", "BlindingLightsALT", 0, true),
                    ),
                    (1551, AvatarInfo::new("KickIt_0", "KickIt", 0, false)),
                    (1552, AvatarInfo::new("KickIt_1", "KickIt", 1, false)),
                    (1553, AvatarInfo::new("KickIt_2", "KickIt", 2, false)),
                    (1554, AvatarInfo::new("KickIt_3", "KickIt", 3, false)),
                    (1555, AvatarInfo::new("KickIt_1_Gold", "KickIt", 1, true)),
                    (
                        1556,
                        AvatarInfo::new("FriendInMe_0", "FriendInMe", 0, false),
                    ),
                    (
                        1557,
                        AvatarInfo::new("FriendInMe_0_Gold", "FriendInMe", 0, true),
                    ),
                    (1558, AvatarInfo::new("Sorbet_0", "Sorbet", 0, false)),
                    (1559, AvatarInfo::new("Sorbet_1", "Sorbet", 1, false)),
                    (1560, AvatarInfo::new("Sorbet_2", "Sorbet", 2, false)),
                    (1561, AvatarInfo::new("Sorbet_3", "Sorbet", 3, false)),
                    (1562, AvatarInfo::new("Sorbet_3_Gold", "Sorbet", 3, true)),
                    (1563, AvatarInfo::new("Uno_0", "Uno", 0, false)),
                    (1564, AvatarInfo::new("Uno_1", "Uno", 1, false)),
                    (1565, AvatarInfo::new("Uno_2", "Uno", 2, false)),
                    (1566, AvatarInfo::new("Uno_3", "Uno", 3, false)),
                    (1567, AvatarInfo::new("Uno_0_Gold", "Uno", 0, true)),
                    (
                        1568,
                        AvatarInfo::new("OtherSideSZA_0", "OtherSideSZA", 0, false),
                    ),
                    (
                        1569,
                        AvatarInfo::new("OtherSideSZA_0_Gold", "OtherSideSZA", 0, true),
                    ),
                    (
                        1570,
                        AvatarInfo::new("BlindingLights_0", "BlindingLights", 0, false),
                    ),
                    (
                        1571,
                        AvatarInfo::new("BlindingLights_0_Gold", "BlindingLights", 0, true),
                    ),
                    (1572, AvatarInfo::new("YoLeLlego_0", "YoLeLlego", 0, false)),
                    (1573, AvatarInfo::new("YoLeLlego_1", "YoLeLlego", 1, false)),
                    (
                        1574,
                        AvatarInfo::new("YoLeLlego_0_Gold", "YoLeLlego", 0, true),
                    ),
                    (
                        1575,
                        AvatarInfo::new("Temperature_0", "Temperature", 0, false),
                    ),
                    (
                        1576,
                        AvatarInfo::new("Temperature_1", "Temperature", 1, false),
                    ),
                    (
                        1577,
                        AvatarInfo::new("Temperature_2", "Temperature", 2, false),
                    ),
                    (
                        1578,
                        AvatarInfo::new("Temperature_3", "Temperature", 3, false),
                    ),
                    (
                        1579,
                        AvatarInfo::new("Temperature_2_Gold", "Temperature", 2, true),
                    ),
                    (1580, AvatarInfo::new("RainOnMe_0", "RainOnMe", 0, false)),
                    (1581, AvatarInfo::new("RainOnMe_1", "RainOnMe", 1, false)),
                    (1582, AvatarInfo::new("RainOnMe_2", "RainOnMe", 2, false)),
                    (1583, AvatarInfo::new("RainOnMe_3", "RainOnMe", 3, false)),
                    (
                        1584,
                        AvatarInfo::new("RainOnMe_1_Gold", "RainOnMe", 1, true),
                    ),
                    (1585, AvatarInfo::new("SaySo_0", "SaySo", 0, false)),
                    (1586, AvatarInfo::new("SaySo_1", "SaySo", 1, false)),
                    (1587, AvatarInfo::new("SaySo_0_Gold", "SaySo", 0, true)),
                    (
                        1594,
                        AvatarInfo::new("TemperatureALT_0", "TemperatureALT", 0, false),
                    ),
                    (
                        1595,
                        AvatarInfo::new("TemperatureALT_0_Gold", "TemperatureALT", 0, true),
                    ),
                ]),
            ),
            (
                Game::JustDanceChina,
                HashMap::from([
                    (1, AvatarInfo::new("Dare_0", "Dare", 0, false)),
                    (2, AvatarInfo::new("DogsOut_0", "DogsOut", 0, false)),
                    (
                        3,
                        AvatarInfo::new("EyeOfTheTiger_0", "EyeOfTheTiger", 0, false),
                    ),
                    (4, AvatarInfo::new("GetAround_0", "GetAround", 0, false)),
                    (5, AvatarInfo::new("HotNCold_0", "HotNCold", 0, false)),
                    (
                        6,
                        AvatarInfo::new("ILikeToMoveIt_0", "ILikeToMoveIt", 0, false),
                    ),
                    (7, AvatarInfo::new("JinGoLoBa_0", "JinGoLoBa", 0, false)),
                    (8, AvatarInfo::new("RingMyBell_0", "RingMyBell", 0, false)),
                    (
                        9,
                        AvatarInfo::new("NineAfternoon_0", "NineAfternoon", 0, false),
                    ),
                    (10, AvatarInfo::new("BabyGirl_0", "BabyGirl", 0, false)),
                    (11, AvatarInfo::new("CallMe_0", "CallMe", 0, false)),
                    (
                        12,
                        AvatarInfo::new("ChickenPayback_0", "ChickenPayback", 0, false),
                    ),
                    (13, AvatarInfo::new("ComeOn_1", "ComeOn", 1, false)),
                    (14, AvatarInfo::new("CosmicGirl_0", "CosmicGirl", 0, false)),
                    (
                        15,
                        AvatarInfo::new("ElectroTribalDLC_0", "ElectroTribalDLC", 0, false),
                    ),
                    (16, AvatarInfo::new("Firework_0", "Firework", 0, false)),
                    (
                        76,
                        AvatarInfo::new("EverybodyNeeds_0", "EverybodyNeeds", 0, false),
                    ),
                    (
                        347,
                        AvatarInfo::new("GangnamStyleDLC_0", "GangnamStyleDLC", 0, false),
                    ),
                    // (922, "?"),
                    // (933, "?"),
                    (953, AvatarInfo::new("OMG_2", "OMG", 2, false)),
                    (
                        1058,
                        AvatarInfo::new("FreezeKids_0", "FreezeKids", 0, false),
                    ),
                    (
                        1059,
                        AvatarInfo::new("FreezeKids_1", "FreezeKids", 1, false),
                    ),
                    (
                        1060,
                        AvatarInfo::new("SchoolyardKids_0", "SchoolyardKids", 0, false),
                    ),
                    (
                        1061,
                        AvatarInfo::new("SchoolyardKids_1", "SchoolyardKids", 1, false),
                    ),
                    (
                        1062,
                        AvatarInfo::new("MedievalKids_0", "MedievalKids", 0, false),
                    ),
                    (
                        1063,
                        AvatarInfo::new("MedievalKids_1", "MedievalKids", 1, false),
                    ),
                    (1064, AvatarInfo::new("ChefKids_0", "ChefKids", 0, false)),
                    (
                        1065,
                        AvatarInfo::new("AdventurerKids_0", "AdventurerKids", 0, false),
                    ),
                    (
                        1066,
                        AvatarInfo::new("WizardKids_0", "WizardKids", 0, false),
                    ),
                    (
                        1067,
                        AvatarInfo::new("BandmasterKids_0", "BandmasterKids", 0, false),
                    ),
                    (
                        1068,
                        AvatarInfo::new("BirthdayKids_0", "BirthdayKids", 0, false),
                    ),
                    (1083, AvatarInfo::new("BalMasque_0", "BalMasque", 0, false)),
                    (1084, AvatarInfo::new("BalMasque_1", "BalMasque", 1, false)),
                    (1085, AvatarInfo::new("BalMasque_2", "BalMasque", 2, false)),
                    (1086, AvatarInfo::new("BalMasque_3", "BalMasque", 3, false)),
                    (
                        1087,
                        AvatarInfo::new("FitButYouKnow_0", "FitButYouKnow", 0, false),
                    ),
                    (
                        1092,
                        AvatarInfo::new("IAmTheBest_1", "IAmTheBest", 1, false),
                    ),
                    (
                        1093,
                        AvatarInfo::new("IAmTheBest_0", "IAmTheBest", 0, false),
                    ),
                    (1095, AvatarInfo::new("Vodovorot_0", "Vodovorot", 0, false)),
                    (1096, AvatarInfo::new("TheTime_0", "TheTime", 0, false)),
                    (1097, AvatarInfo::new("TheTime_1", "TheTime", 1, false)),
                    (1098, AvatarInfo::new("TheTime_2", "TheTime", 2, false)),
                    (1099, AvatarInfo::new("TheTime_3", "TheTime", 3, false)),
                    (
                        1101,
                        AvatarInfo::new("AlwaysLookOn_0", "AlwaysLookOn", 0, false),
                    ),
                    (
                        1102,
                        AvatarInfo::new("AlwaysLookOn_1", "AlwaysLookOn", 1, false),
                    ),
                    (
                        1103,
                        AvatarInfo::new("AlwaysLookOn_2", "AlwaysLookOn", 2, false),
                    ),
                    (
                        1104,
                        AvatarInfo::new("AlwaysLookOn_3", "AlwaysLookOn", 3, false),
                    ),
                    (1105, AvatarInfo::new("Policeman_0", "Policeman", 0, false)),
                    (1106, AvatarInfo::new("Policeman_1", "Policeman", 1, false)),
                    (1107, AvatarInfo::new("Policeman_2", "Policeman", 2, false)),
                    (
                        1108,
                        AvatarInfo::new("RainOverMe_0", "RainOverMe", 0, false),
                    ),
                    (
                        1109,
                        AvatarInfo::new("BalMasque_1_Gold", "BalMasque", 1, true),
                    ),
                    (
                        1110,
                        AvatarInfo::new("FitButYouKnow_0_Gold", "FitButYouKnow", 0, true),
                    ),
                    (
                        1111,
                        AvatarInfo::new("IAmTheBest_1_Gold", "IAmTheBest", 1, true),
                    ),
                    (
                        1112,
                        AvatarInfo::new("Vodovorot_0_Gold", "Vodovorot", 0, true),
                    ),
                    (1113, AvatarInfo::new("TheTime_0_Gold", "TheTime", 0, true)),
                    (
                        1114,
                        AvatarInfo::new("AlwaysLookOn_1_Gold", "AlwaysLookOn", 1, true),
                    ),
                    (
                        1115,
                        AvatarInfo::new("Policeman_0_Gold", "Policeman", 0, true),
                    ),
                    (
                        1116,
                        AvatarInfo::new("RainOverMe_0_Gold", "RainOverMe", 0, true),
                    ),
                    (
                        1117,
                        AvatarInfo::new("KeepInTouch_0", "KeepInTouch", 0, false),
                    ),
                    (1118, AvatarInfo::new("BadBoy_0", "BadBoy", 0, false)),
                    (1119, AvatarInfo::new("BadBoy_1", "BadBoy", 1, false)),
                    (1120, AvatarInfo::new("BadBoy_0_Gold", "BadBoy", 0, true)),
                    (
                        1121,
                        AvatarInfo::new("KeepInTouch_0_Gold", "KeepInTouch", 0, true),
                    ),
                    (1122, AvatarInfo::new("BabyShark_0", "BabyShark", 0, false)),
                    (1123, AvatarInfo::new("BabyShark_1", "BabyShark", 1, false)),
                    (
                        1124,
                        AvatarInfo::new("GodIsAWoman_0", "GodIsAWoman", 0, false),
                    ),
                    (
                        1125,
                        AvatarInfo::new("BabyShark_1_Gold", "BabyShark", 1, true),
                    ),
                    (
                        1126,
                        AvatarInfo::new("GodIsAWoman_0_Gold", "GodIsAWoman", 0, true),
                    ),
                    (1127, AvatarInfo::new("GetAround_0", "GetAround", 0, false)),
                    (1128, AvatarInfo::new("Firework_0", "Firework", 0, false)),
                    (1129, AvatarInfo::new("HeyYa_0", "HeyYa", 0, false)),
                    (
                        1130,
                        AvatarInfo::new("KattiKalandal_1", "KattiKalandal", 1, false),
                    ),
                    (
                        1131,
                        AvatarInfo::new("MonsterMash_0", "MonsterMash", 0, false),
                    ),
                    (
                        1132,
                        AvatarInfo::new("CaliforniaGurls_0", "CaliforniaGurls", 0, false),
                    ),
                    (
                        1133,
                        AvatarInfo::new("GotMeDancing_0", "GotMeDancing", 0, false),
                    ),
                    (1134, AvatarInfo::new("HeyBoy_0", "HeyBoy", 0, false)),
                    (
                        1135,
                        AvatarInfo::new("WhatYouWait_0", "WhatYouWait", 0, false),
                    ),
                    (1136, AvatarInfo::new("Disturbia_0", "Disturbia", 0, false)),
                    (
                        1137,
                        AvatarInfo::new("IstanbulQUAT_2", "IstanbulQUAT", 2, false),
                    ),
                    (1138, AvatarInfo::new("RockNRoll_0", "RockNRoll", 0, false)),
                    (
                        1139,
                        AvatarInfo::new("YouMakeMeFeelDLC_0", "YouMakeMeFeelDLC", 0, false),
                    ),
                    (
                        1140,
                        AvatarInfo::new("YouReTheFirst_0", "YouReTheFirst", 0, false),
                    ),
                    (1141, AvatarInfo::new("RobotRock_0", "RobotRock", 0, false)),
                    (1142, AvatarInfo::new("SheWolf_0", "SheWolf", 0, false)),
                    (
                        1143,
                        AvatarInfo::new("WhereHaveYou_0", "WhereHaveYou", 0, false),
                    ),
                    (1144, AvatarInfo::new("Starships_0", "Starships", 0, false)),
                    (1145, AvatarInfo::new("CmonDLC_1", "CmonDLC", 1, false)),
                    (1146, AvatarInfo::new("ThatPower_1", "ThatPower", 1, false)),
                    (
                        1147,
                        AvatarInfo::new("TurnUpTheLove_0", "TurnUpTheLove", 0, false),
                    ),
                    (1148, AvatarInfo::new("GetLucky_1", "GetLucky", 1, false)),
                    (1149, AvatarInfo::new("LimaGolf1_0", "LimaGolf1", 0, false)),
                    (
                        1150,
                        AvatarInfo::new("Luftballons_0", "Luftballons", 0, false),
                    ),
                    (1151, AvatarInfo::new("TheFox_1", "TheFox", 1, false)),
                    (
                        1152,
                        AvatarInfo::new("BuiltForThis_0", "BuiltForThis", 0, false),
                    ),
                    (1153, AvatarInfo::new("Happy_0", "Happy", 0, false)),
                    (1154, AvatarInfo::new("Summer_0", "Summer", 0, false)),
                    (
                        1155,
                        AvatarInfo::new("WalkThisWay_1", "WalkThisWay", 1, false),
                    ),
                    (1156, AvatarInfo::new("Tetris_3", "Tetris", 3, false)),
                    (1157, AvatarInfo::new("Macarena_3", "Macarena", 3, false)),
                    (1158, AvatarInfo::new("Birthday_0", "Birthday", 0, false)),
                    (
                        1159,
                        AvatarInfo::new("HoldingOut_0", "HoldingOut", 0, false),
                    ),
                    (
                        1160,
                        AvatarInfo::new("PoundTheAlarm_0", "PoundTheAlarm", 0, false),
                    ),
                    (1161, AvatarInfo::new("Blame_0", "Blame", 0, false)),
                    (1162, AvatarInfo::new("Animals_0", "Animals", 0, false)),
                    // (1163, "?"),
                    (
                        1164,
                        AvatarInfo::new("BornThisWay_1", "BornThisWay", 1, false),
                    ),
                    (1165, AvatarInfo::new("Circus_2_C3", "Circus", 2, false)),
                    (
                        1166,
                        AvatarInfo::new("UptownFunk_0", "UptownFunk", 0, false),
                    ),
                    (1167, AvatarInfo::new("Chiwawa_0", "Chiwawa", 0, false)),
                    (
                        1168,
                        AvatarInfo::new("ElectroMambo_0", "ElectroMambo", 0, false),
                    ),
                    (1169, AvatarInfo::new("HeyMama_1", "HeyMama", 1, false)),
                    (1170, AvatarInfo::new("KaboomPow_0", "KaboomPow", 0, false)),
                    (1171, AvatarInfo::new("ThisIsHow_0", "ThisIsHow", 0, false)),
                    (
                        1172,
                        AvatarInfo::new("DieYoungDLC_0", "DieYoungDLC", 0, false),
                    ),
                    (1173, AvatarInfo::new("BoomDLC_0", "BoomDLC", 0, false)),
                    (1174, AvatarInfo::new("Cotton_0", "Cotton", 0, false)),
                    (1175, AvatarInfo::new("Fame_0", "Fame", 0, false)),
                    (
                        1176,
                        AvatarInfo::new("TribalDance_0", "TribalDance", 0, false),
                    ),
                    (
                        1177,
                        AvatarInfo::new("GoodFeeling_0", "GoodFeeling", 0, false),
                    ),
                    (1178, AvatarInfo::new("JaiHo_0", "JaiHo", 0, false)),
                    (
                        1179,
                        AvatarInfo::new("KetchupSong_0", "KetchupSong", 0, false),
                    ),
                    (1180, AvatarInfo::new("Kurio_1", "Kurio", 1, false)),
                    (1181, AvatarInfo::new("Lollipop_0", "Lollipop", 0, false)),
                    (
                        1182,
                        AvatarInfo::new("ThatsTheWay_0", "ThatsTheWay", 0, false),
                    ),
                    (
                        1183,
                        AvatarInfo::new("TheFinalCountdown_0", "TheFinalCountdown", 0, false),
                    ),
                    (1184, AvatarInfo::new("ET_0", "ET", 0, false)),
                    (
                        1185,
                        AvatarInfo::new("CrazyLittle_1", "CrazyLittle", 1, false),
                    ),
                    (
                        1186,
                        AvatarInfo::new("CryingBlood_0", "CryingBlood", 0, false),
                    ),
                    (
                        1187,
                        AvatarInfo::new("GirlsJustWant_0", "GirlsJustWant", 0, false),
                    ),
                    (
                        1188,
                        AvatarInfo::new("ScreamNShoutALT_0", "ScreamNShoutALT", 0, false),
                    ),
                    (
                        1189,
                        AvatarInfo::new("WhatIsLove_0", "WhatIsLove", 0, false),
                    ),
                    (
                        1190,
                        AvatarInfo::new("DontStopMe_0", "DontStopMe", 0, false),
                    ),
                    (1191, AvatarInfo::new("PoPiPo_2", "PoPiPo", 2, false)),
                    (1192, AvatarInfo::new("September_1", "September", 1, false)),
                    (1193, AvatarInfo::new("WorthIt_0", "WorthIt", 0, false)),
                    (1194, AvatarInfo::new("Titanium_0", "Titanium", 0, false)),
                    (
                        1195,
                        AvatarInfo::new("DragosteaDinTei_1", "DragosteaDinTei", 1, false),
                    ),
                    (1196, AvatarInfo::new("Bailar_0", "Bailar", 0, false)),
                    (
                        1197,
                        AvatarInfo::new("HappyFarmKids_0", "HappyFarmKids", 0, false),
                    ),
                    (1198, AvatarInfo::new("TumBum_1", "TumBum", 1, false)),
                    (1199, AvatarInfo::new("Footloose_0", "Footloose", 0, false)),
                    (1200, AvatarInfo::new("24K_0", "24K", 0, false)),
                    (1201, AvatarInfo::new("Automaton_0", "Automaton", 0, false)),
                    (
                        1202,
                        AvatarInfo::new("BubblePopALT_1", "BubblePopALT", 1, false),
                    ),
                    (1203, AvatarInfo::new("LoveWard_0", "LoveWard", 0, false)),
                    (
                        1204,
                        AvatarInfo::new("AllYouGotta_0", "AllYouGotta", 0, false),
                    ),
                    (
                        1205,
                        AvatarInfo::new("AnotherOne_3", "AnotherOne", 3, false),
                    ),
                    (1206, AvatarInfo::new("BubblePop_1", "BubblePop", 1, false)),
                    (1207, AvatarInfo::new("Despacito_3", "Despacito", 3, false)),
                    (
                        1208,
                        AvatarInfo::new("SwishSwish_2", "SwishSwish", 2, false),
                    ),
                    (
                        1209,
                        AvatarInfo::new("SwishSwish_3", "SwishSwish", 3, false),
                    ),
                    (1210, AvatarInfo::new("MamaMia_0", "MamaMia", 0, false)),
                    (
                        1211,
                        AvatarInfo::new("WorkWorkALT_0", "WorkWorkALT", 0, false),
                    ),
                    (1212, AvatarInfo::new("Adeyyo_0", "Adeyyo", 0, false)),
                    (
                        1213,
                        AvatarInfo::new("MerryChristmasKids_1", "MerryChristmasKids", 1, false),
                    ),
                    (1214, AvatarInfo::new("MiMiMi_1", "MiMiMi", 1, false)),
                    (
                        1215,
                        AvatarInfo::new("MonstersAcademyKids_0", "MonstersAcademyKids", 0, false),
                    ),
                    (1216, AvatarInfo::new("Finesse_1", "Finesse", 1, false)),
                    (
                        1217,
                        AvatarInfo::new("IFeelItComing_0", "IFeelItComing", 0, false),
                    ),
                    (1218, AvatarInfo::new("MiloscW_0", "MiloscW", 0, false)),
                    (1219, AvatarInfo::new("MadLove_1", "MadLove", 1, false)),
                    (1220, AvatarInfo::new("OMG_2", "OMG", 2, false)),
                    (
                        1221,
                        AvatarInfo::new("NoTearsLeft_0", "NoTearsLeft", 0, false),
                    ), // C1
                    (1222, AvatarInfo::new("OneKiss_0", "OneKiss", 0, false)),
                    (
                        1223,
                        AvatarInfo::new("SweetSensation_3", "SweetSensation", 3, false),
                    ),
                    (1224, AvatarInfo::new("DDUDU_3", "DDUDU", 3, false)),
                    (1225, AvatarInfo::new("Sugar_0", "Sugar", 0, false)),
                    (1226, AvatarInfo::new("Sugar_1", "Sugar", 1, false)),
                    (1227, AvatarInfo::new("ConCalma_0", "ConCalma", 0, false)),
                    (1228, AvatarInfo::new("ConCalma_1", "ConCalma", 1, false)),
                    (
                        1229,
                        AvatarInfo::new("ConCalma_1_Gold", "ConCalma", 1, true),
                    ),
                    (1231, AvatarInfo::new("Sushii_0", "Sushii", 0, false)),
                    (1232, AvatarInfo::new("Sushii_0_Gold", "Sushii", 0, true)),
                    (1236, AvatarInfo::new("SoyYoALT_0", "SoyYoALT", 0, false)),
                    (1237, AvatarInfo::new("SoyYoALT_1", "SoyYoALT", 1, false)),
                    (
                        1238,
                        AvatarInfo::new("SoyYoALT_0_Gold", "SoyYoALT", 0, true),
                    ),
                    (1243, AvatarInfo::new("Bangarang_0", "Bangarang", 0, false)),
                    (
                        1244,
                        AvatarInfo::new("Bangarang_0_Gold", "Bangarang", 0, true),
                    ),
                    (1245, AvatarInfo::new("365_0", "365", 0, false)),
                    (1246, AvatarInfo::new("365_0_Gold", "365", 0, true)),
                    (
                        1247,
                        AvatarInfo::new("TheTimeALT_0", "TheTimeALT", 0, false),
                    ),
                    (
                        1248,
                        AvatarInfo::new("TheTimeALT_0_Gold", "TheTimeALT", 0, true),
                    ),
                    (
                        1249,
                        AvatarInfo::new("IAmTheBestALT_0", "IAmTheBestALT", 0, false),
                    ),
                    (
                        1250,
                        AvatarInfo::new("IAmTheBestALT_0_Gold", "IAmTheBestALT", 0, true),
                    ),
                    (
                        1251,
                        AvatarInfo::new("RainOverMeALT_0", "RainOverMeALT", 0, false),
                    ),
                    (
                        1252,
                        AvatarInfo::new("RainOverMeALT_0_Gold", "RainOverMeALT", 0, true),
                    ),
                    // (1253, "?"), // Ubisoft
                    // (1254, "?"), // Ubisoft
                    // (1255, "?"), // Ubisoft
                    // (1256, "?"), // Ubisoft
                    // (1257, "?"), // Ubisoft
                    (1258, AvatarInfo::new("ILikeIt_0", "ILikeIt", 0, false)),
                    (1259, AvatarInfo::new("ILikeIt_1", "ILikeIt", 1, false)),
                    (1260, AvatarInfo::new("ILikeIt_2", "ILikeIt", 2, false)),
                    (1261, AvatarInfo::new("ILikeIt_1_Gold", "ILikeIt", 1, true)),
                    // (1262, "?"), // Ubisoft
                    (
                        1263,
                        AvatarInfo::new("GodIsAWomanALT_0", "GodIsAWomanALT", 0, false),
                    ),
                    (
                        1264,
                        AvatarInfo::new("GodIsAWomanALT_0_Gold", "GodIsAWomanALT", 0, true),
                    ),
                    (1265, AvatarInfo::new("SoyYo_0", "SoyYo", 0, false)),
                    (1266, AvatarInfo::new("SoyYo_0_Gold", "SoyYo", 0, true)),
                    (1267, AvatarInfo::new("TelAviv_0", "TelAviv", 0, false)),
                    (1268, AvatarInfo::new("TelAviv_1", "TelAviv", 1, false)),
                    (1269, AvatarInfo::new("TelAviv_2", "TelAviv", 2, false)),
                    (1270, AvatarInfo::new("TelAviv_2_Gold", "TelAviv", 2, true)),
                    (1271, AvatarInfo::new("Skibidi_0", "Skibidi", 0, false)),
                    (1272, AvatarInfo::new("Skibidi_1", "Skibidi", 1, false)),
                    (1273, AvatarInfo::new("Skibidi_0_Gold", "Skibidi", 0, true)),
                    (1274, AvatarInfo::new("TakiTaki_0", "TakiTaki", 0, false)),
                    (
                        1275,
                        AvatarInfo::new("TakiTaki_0_Gold", "TakiTaki", 0, true),
                    ),
                    // (1279, "?"), // Ubisoft
                    // (1280, "?"), // Ubisoft
                    (1281, AvatarInfo::new("Swag_0", "Swag", 0, false)),
                    (1282, AvatarInfo::new("Swag_0_Gold", "Swag", 0, true)),
                    (1283, AvatarInfo::new("SushiiALT_0", "SushiiALT", 0, false)),
                    (
                        1284,
                        AvatarInfo::new("SushiiALT_0_Gold", "SushiiALT", 0, true),
                    ),
                    (
                        1285,
                        AvatarInfo::new("BangarangALT_0", "BangarangALT", 0, false),
                    ),
                    (
                        1286,
                        AvatarInfo::new("BangarangALT_0_Gold", "BangarangALT", 0, true),
                    ),
                    (
                        1287,
                        AvatarInfo::new("TakiTakiALT_0", "TakiTakiALT", 0, false),
                    ),
                    (
                        1288,
                        AvatarInfo::new("TakiTakiALT_1", "TakiTakiALT", 1, false),
                    ),
                    (
                        1289,
                        AvatarInfo::new("TakiTakiALT_1_Gold", "TakiTakiALT", 1, true),
                    ),
                    (1290, AvatarInfo::new("GetBusy_0", "GetBusy", 0, false)),
                    (1291, AvatarInfo::new("GetBusy_1", "GetBusy", 1, false)),
                    (1292, AvatarInfo::new("GetBusy_1_Gold", "GetBusy", 1, true)),
                    (1293, AvatarInfo::new("HighHopes_0", "HighHopes", 0, false)),
                    (1294, AvatarInfo::new("HighHopes_1", "HighHopes", 1, false)),
                    (1295, AvatarInfo::new("HighHopes_2", "HighHopes", 2, false)),
                    (1296, AvatarInfo::new("HighHopes_3", "HighHopes", 3, false)),
                    (
                        1297,
                        AvatarInfo::new("HighHopes_0_Gold", "HighHopes", 0, true),
                    ),
                    (
                        1298,
                        AvatarInfo::new("KillThisLove_0", "KillThisLove", 0, false),
                    ),
                    (
                        1299,
                        AvatarInfo::new("KillThisLove_1", "KillThisLove", 1, false),
                    ),
                    (
                        1300,
                        AvatarInfo::new("KillThisLove_2", "KillThisLove", 2, false),
                    ),
                    (
                        1301,
                        AvatarInfo::new("KillThisLove_3", "KillThisLove", 3, false),
                    ),
                    (
                        1302,
                        AvatarInfo::new("KillThisLove_2_Gold", "KillThisLove", 2, true),
                    ),
                    (1303, AvatarInfo::new("Talk_0", "Talk", 0, false)),
                    (1304, AvatarInfo::new("Talk_0_Gold", "Talk", 0, true)),
                    (1306, AvatarInfo::new("MaItu_0", "MaItu", 0, false)),
                    (1307, AvatarInfo::new("MaItu_0_Gold", "MaItu", 0, true)),
                    (1308, AvatarInfo::new("Everybody_0", "Everybody", 0, false)),
                    (1309, AvatarInfo::new("Everybody_1", "Everybody", 1, false)),
                    (1310, AvatarInfo::new("Everybody_2", "Everybody", 2, false)),
                    (1311, AvatarInfo::new("Everybody_3", "Everybody", 3, false)),
                    (
                        1312,
                        AvatarInfo::new("Everybody_2_Gold", "Everybody", 2, true),
                    ),
                    (1316, AvatarInfo::new("TalkALT_0", "TalkALT", 0, false)),
                    (1317, AvatarInfo::new("TalkALT_0_Gold", "TalkALT", 0, true)),
                    (
                        1318,
                        AvatarInfo::new("OldTownRoad_0", "OldTownRoad", 0, false),
                    ),
                    (
                        1319,
                        AvatarInfo::new("OldTownRoad_0_Gold", "OldTownRoad", 0, true),
                    ),
                    (
                        1320,
                        AvatarInfo::new("NotYourOrdinary_3_Gold", "NotYourOrdinary", 3, true),
                    ),
                    (1321, AvatarInfo::new("ILikeIt_0_Gold", "ILikeIt", 0, true)),
                    (1322, AvatarInfo::new("7Rings_0", "7Rings", 0, false)),
                    (1323, AvatarInfo::new("7Rings_1", "7Rings", 1, false)),
                    (1324, AvatarInfo::new("7Rings_2", "7Rings", 2, false)),
                    (1325, AvatarInfo::new("7Rings_1_Gold", "7Rings", 1, true)),
                    (1326, AvatarInfo::new("BadGuy_0", "BadGuy", 0, false)),
                    (1327, AvatarInfo::new("BadGuy_0_Gold", "BadGuy", 0, true)),
                    (1328, AvatarInfo::new("Footwork_0", "Footwork", 0, false)),
                    (
                        1329,
                        AvatarInfo::new("Footwork_0_Gold", "Footwork", 0, true),
                    ),
                    (
                        1330,
                        AvatarInfo::new("OldTownRoadALT_0", "OldTownRoadALT", 0, false),
                    ),
                    (
                        1331,
                        AvatarInfo::new("OldTownRoadALT_1", "OldTownRoadALT", 1, false),
                    ),
                    (
                        1332,
                        AvatarInfo::new("OldTownRoadALT_2", "OldTownRoadALT", 2, false),
                    ),
                    (
                        1333,
                        AvatarInfo::new("OldTownRoadALT_2_Gold", "OldTownRoadALT", 2, true),
                    ),
                    (
                        1334,
                        AvatarInfo::new("JustAnIllusion_0", "JustAnIllusion", 0, false),
                    ),
                    (
                        1335,
                        AvatarInfo::new("JustAnIllusion_1", "JustAnIllusion", 1, false),
                    ),
                    (
                        1336,
                        AvatarInfo::new("JustAnIllusion_1_Gold", "JustAnIllusion", 1, true),
                    ),
                    (1337, AvatarInfo::new("StopMovin_0", "StopMovin", 0, false)),
                    (1338, AvatarInfo::new("StopMovin_1", "StopMovin", 1, false)),
                    (1339, AvatarInfo::new("StopMovin_2", "StopMovin", 2, false)),
                    (
                        1340,
                        AvatarInfo::new("StopMovin_1_Gold", "StopMovin", 1, true),
                    ),
                    (1342, AvatarInfo::new("7RingsALT_0", "7RingsALT", 0, false)),
                    (
                        1343,
                        AvatarInfo::new("7RingsALT_0_Gold", "7RingsALT", 0, true),
                    ),
                    (
                        1344,
                        AvatarInfo::new("KillThisLoveALT_0", "KillThisLoveALT", 0, false),
                    ),
                    (
                        1345,
                        AvatarInfo::new("KillThisLoveALT_0_Gold", "KillThisLoveALT", 0, true),
                    ),
                    (
                        1346,
                        AvatarInfo::new("BassaSababa_0", "BassaSababa", 0, false),
                    ),
                    (
                        1347,
                        AvatarInfo::new("BassaSababa_0_Gold", "BassaSababa", 0, true),
                    ),
                    (
                        1348,
                        AvatarInfo::new("FancyTwice_0", "FancyTwice", 0, false),
                    ),
                    (
                        1349,
                        AvatarInfo::new("FancyTwice_1", "FancyTwice", 1, false),
                    ),
                    (
                        1350,
                        AvatarInfo::new("FancyTwice_2", "FancyTwice", 2, false),
                    ),
                    (
                        1351,
                        AvatarInfo::new("UglyBeauty_0", "UglyBeauty", 0, false),
                    ),
                    (
                        1352,
                        AvatarInfo::new("UglyBeauty_1", "UglyBeauty", 1, false),
                    ),
                    (
                        1353,
                        AvatarInfo::new("UglyBeauty_2", "UglyBeauty", 2, false),
                    ),
                    (
                        1354,
                        AvatarInfo::new("DoCarnaval_0", "DoCarnaval", 0, false),
                    ),
                    (
                        1355,
                        AvatarInfo::new("DoCarnaval_0_Gold", "DoCarnaval", 0, true),
                    ),
                    (1356, AvatarInfo::new("ConAltura_0", "ConAltura", 0, false)),
                    (
                        1357,
                        AvatarInfo::new("ConAltura_0_Gold", "ConAltura", 0, true),
                    ),
                    (1358, AvatarInfo::new("IDontCare_0", "IDontCare", 0, false)),
                    (
                        1359,
                        AvatarInfo::new("IDontCare_0_Gold", "IDontCare", 0, true),
                    ),
                    (
                        1360,
                        AvatarInfo::new("UglyBeauty_0_Gold", "UglyBeauty", 0, true),
                    ),
                    (
                        1361,
                        AvatarInfo::new("FancyTwice_2_Gold", "FancyTwice", 2, true),
                    ),
                    (1362, AvatarInfo::new("CanCan_0", "CanCan", 0, false)),
                    (1363, AvatarInfo::new("CanCan_1", "CanCan", 1, false)),
                    (1364, AvatarInfo::new("CanCan_2", "CanCan", 2, false)),
                    (1365, AvatarInfo::new("CanCan_3", "CanCan", 3, false)),
                    (1366, AvatarInfo::new("CanCan_4", "CanCan", 4, false)),
                    (1367, AvatarInfo::new("CanCan_1_Gold", "CanCan", 1, true)),
                    (
                        1368,
                        AvatarInfo::new("BoyWithLuv_0", "BoyWithLuv", 0, false),
                    ),
                    (
                        1369,
                        AvatarInfo::new("BoyWithLuv_1", "BoyWithLuv", 1, false),
                    ),
                    (
                        1370,
                        AvatarInfo::new("BoyWithLuv_2", "BoyWithLuv", 2, false),
                    ),
                    (
                        1371,
                        AvatarInfo::new("BoyWithLuv_2_Gold", "BoyWithLuv", 2, true),
                    ),
                ]),
            ),
            (
                Game::JustDance2020,
                HashMap::from([
                    (1, AvatarInfo::new("Dare_0", "Dare", 0, false)),
                    (2, AvatarInfo::new("DogsOut_0", "DogsOut", 0, false)),
                    (
                        3,
                        AvatarInfo::new("EyeOfTheTiger_0", "EyeOfTheTiger", 0, false),
                    ),
                    (4, AvatarInfo::new("GetAround_0", "GetAround", 0, false)),
                    (5, AvatarInfo::new("HotNCold_0", "HotNCold", 0, false)),
                    (
                        6,
                        AvatarInfo::new("ILikeToMoveIt_0", "ILikeToMoveIt", 0, false),
                    ),
                    (7, AvatarInfo::new("JinGoLoBa_0", "JinGoLoBa", 0, false)),
                    (8, AvatarInfo::new("RingMyBell_0", "RingMyBell", 0, false)),
                    (
                        9,
                        AvatarInfo::new("NineAfternoon_0", "NineAfternoon", 0, false),
                    ),
                    (10, AvatarInfo::new("BabyGirl_0", "BabyGirl", 0, false)),
                    (11, AvatarInfo::new("CallMe_0", "CallMe", 0, false)),
                    (
                        12,
                        AvatarInfo::new("ChickenPayback_0", "ChickenPayback", 0, false),
                    ),
                    (13, AvatarInfo::new("ComeOn_1", "ComeOn", 1, false)),
                    (14, AvatarInfo::new("CosmicGirl_0", "CosmicGirl", 0, false)),
                    (
                        15,
                        AvatarInfo::new("ElectroTribalDLC_0", "ElectroTribalDLC", 0, false),
                    ),
                    (16, AvatarInfo::new("Firework_0", "Firework", 0, false)),
                    (
                        76,
                        AvatarInfo::new("EverybodyNeeds_0", "EverybodyNeeds", 0, false),
                    ),
                    (
                        347,
                        AvatarInfo::new("GangnamStyleDLC_0", "GangnamStyleDLC", 0, false),
                    ),
                    (922, AvatarInfo::new("MiMiMiALT_3", "MiMiMiALT", 3, false)),
                    (933, AvatarInfo::new("Fire_1", "Fire", 1, false)),
                    (953, AvatarInfo::new("OMG_2", "OMG", 2, false)),
                    (
                        1058,
                        AvatarInfo::new("FreezeKids_0", "FreezeKids", 0, false),
                    ),
                    (
                        1059,
                        AvatarInfo::new("FreezeKids_1", "FreezeKids", 1, false),
                    ),
                    (
                        1060,
                        AvatarInfo::new("SchoolyardKids_0", "SchoolyardKids", 0, false),
                    ),
                    (
                        1061,
                        AvatarInfo::new("SchoolyardKids_1", "SchoolyardKids", 1, false),
                    ),
                    (
                        1062,
                        AvatarInfo::new("MedievalKids_0", "MedievalKids", 0, false),
                    ),
                    (
                        1063,
                        AvatarInfo::new("MedievalKids_1", "MedievalKids", 1, false),
                    ),
                    (1064, AvatarInfo::new("ChefKids_0", "ChefKids", 0, false)),
                    (
                        1065,
                        AvatarInfo::new("AdventurerKids_0", "AdventurerKids", 0, false),
                    ),
                    (
                        1066,
                        AvatarInfo::new("WizardKids_0", "WizardKids", 0, false),
                    ),
                    (
                        1067,
                        AvatarInfo::new("BandmasterKids_0", "BandmasterKids", 0, false),
                    ),
                    (
                        1068,
                        AvatarInfo::new("BirthdayKids_0", "BirthdayKids", 0, false),
                    ),
                    (1083, AvatarInfo::new("BalMasque_0", "BalMasque", 0, false)),
                    (1084, AvatarInfo::new("BalMasque_1", "BalMasque", 1, false)),
                    (1085, AvatarInfo::new("BalMasque_2", "BalMasque", 2, false)),
                    (1086, AvatarInfo::new("BalMasque_3", "BalMasque", 3, false)),
                    (
                        1087,
                        AvatarInfo::new("FitButYouKnow_0", "FitButYouKnow", 0, false),
                    ),
                    (
                        1092,
                        AvatarInfo::new("IAmTheBest_1", "IAmTheBest", 1, false),
                    ),
                    (
                        1093,
                        AvatarInfo::new("IAmTheBest_0", "IAmTheBest", 0, false),
                    ),
                    (1095, AvatarInfo::new("Vodovorot_0", "Vodovorot", 0, false)),
                    (1096, AvatarInfo::new("TheTime_0", "TheTime", 0, false)),
                    (1097, AvatarInfo::new("TheTime_1", "TheTime", 1, false)),
                    (1098, AvatarInfo::new("TheTime_2", "TheTime", 2, false)),
                    (1099, AvatarInfo::new("TheTime_3", "TheTime", 3, false)),
                    (
                        1101,
                        AvatarInfo::new("AlwaysLookOn_0", "AlwaysLookOn", 0, false),
                    ),
                    (
                        1102,
                        AvatarInfo::new("AlwaysLookOn_1", "AlwaysLookOn", 1, false),
                    ),
                    (
                        1103,
                        AvatarInfo::new("AlwaysLookOn_2", "AlwaysLookOn", 2, false),
                    ),
                    (
                        1104,
                        AvatarInfo::new("AlwaysLookOn_3", "AlwaysLookOn", 3, false),
                    ),
                    (1105, AvatarInfo::new("Policeman_0", "Policeman", 0, false)),
                    (1106, AvatarInfo::new("Policeman_1", "Policeman", 1, false)),
                    (1107, AvatarInfo::new("Policeman_2", "Policeman", 2, false)),
                    (
                        1108,
                        AvatarInfo::new("RainOverMe_0", "RainOverMe", 0, false),
                    ),
                    (
                        1109,
                        AvatarInfo::new("BalMasque_1_Gold", "BalMasque", 1, true),
                    ),
                    (
                        1110,
                        AvatarInfo::new("FitButYouKnow_0_Gold", "FitButYouKnow", 0, true),
                    ),
                    (
                        1111,
                        AvatarInfo::new("IAmTheBest_1_Gold", "IAmTheBest", 1, true),
                    ),
                    (
                        1112,
                        AvatarInfo::new("Vodovorot_0_Gold", "Vodovorot", 0, true),
                    ),
                    (1113, AvatarInfo::new("TheTime_0_Gold", "TheTime", 0, true)),
                    (
                        1114,
                        AvatarInfo::new("AlwaysLookOn_1_Gold", "AlwaysLookOn", 1, true),
                    ),
                    (
                        1115,
                        AvatarInfo::new("Policeman_0_Gold", "Policeman", 0, true),
                    ),
                    (
                        1116,
                        AvatarInfo::new("RainOverMe_0_Gold", "RainOverMe", 0, true),
                    ),
                    (
                        1117,
                        AvatarInfo::new("KeepInTouch_0", "KeepInTouch", 0, false),
                    ),
                    (1118, AvatarInfo::new("BadBoy_0", "BadBoy", 0, false)),
                    (1119, AvatarInfo::new("BadBoy_1", "BadBoy", 1, false)),
                    (1120, AvatarInfo::new("BadBoy_0_Gold", "BadBoy", 0, true)),
                    (
                        1121,
                        AvatarInfo::new("KeepInTouch_0_Gold", "KeepInTouch", 0, true),
                    ),
                    (1122, AvatarInfo::new("BabyShark_0", "BabyShark", 0, false)),
                    (1123, AvatarInfo::new("BabyShark_1", "BabyShark", 1, false)),
                    (
                        1124,
                        AvatarInfo::new("GodIsAWoman_0", "GodIsAWoman", 0, false),
                    ),
                    (
                        1125,
                        AvatarInfo::new("BabyShark_1_Gold", "BabyShark", 1, true),
                    ),
                    (
                        1126,
                        AvatarInfo::new("GodIsAWoman_0_Gold", "GodIsAWoman", 0, true),
                    ),
                    (1127, AvatarInfo::new("GetAround_0", "GetAround", 0, false)),
                    (1128, AvatarInfo::new("Firework_0", "Firework", 0, false)),
                    (1129, AvatarInfo::new("HeyYa_0", "HeyYa", 0, false)),
                    (
                        1130,
                        AvatarInfo::new("KattiKalandal_1", "KattiKalandal", 1, false),
                    ),
                    (
                        1131,
                        AvatarInfo::new("MonsterMash_0", "MonsterMash", 0, false),
                    ),
                    (
                        1132,
                        AvatarInfo::new("CaliforniaGurls_0", "CaliforniaGurls", 0, false),
                    ),
                    (
                        1133,
                        AvatarInfo::new("GotMeDancing_0", "GotMeDancing", 0, false),
                    ),
                    (1134, AvatarInfo::new("HeyBoy_0", "HeyBoy", 0, false)),
                    (
                        1135,
                        AvatarInfo::new("WhatYouWait_0", "WhatYouWait", 0, false),
                    ),
                    (1136, AvatarInfo::new("Disturbia_0", "Disturbia", 0, false)),
                    (
                        1137,
                        AvatarInfo::new("IstanbulQUAT_2", "IstanbulQUAT", 2, false),
                    ),
                    (1138, AvatarInfo::new("RockNRoll_0", "RockNRoll", 0, false)),
                    (
                        1139,
                        AvatarInfo::new("YouMakeMeFeelDLC_0", "YouMakeMeFeelDLC", 0, false),
                    ),
                    (
                        1140,
                        AvatarInfo::new("YouReTheFirst_0", "YouReTheFirst", 0, false),
                    ),
                    (1141, AvatarInfo::new("RobotRock_0", "RobotRock", 0, false)),
                    (1142, AvatarInfo::new("SheWolf_0", "SheWolf", 0, false)),
                    (
                        1143,
                        AvatarInfo::new("WhereHaveYou_0", "WhereHaveYou", 0, false),
                    ),
                    (1144, AvatarInfo::new("Starships_0", "Starships", 0, false)),
                    (1145, AvatarInfo::new("CmonDLC_1", "CmonDLC", 1, false)),
                    (1146, AvatarInfo::new("ThatPower_1", "ThatPower", 1, false)),
                    (
                        1147,
                        AvatarInfo::new("TurnUpTheLove_0", "TurnUpTheLove", 0, false),
                    ),
                    (1148, AvatarInfo::new("GetLucky_1", "GetLucky", 1, false)),
                    (1149, AvatarInfo::new("LimaGolf1_0", "LimaGolf1", 0, false)),
                    (
                        1150,
                        AvatarInfo::new("Luftballons_0", "Luftballons", 0, false),
                    ),
                    (1151, AvatarInfo::new("TheFox_1", "TheFox", 1, false)),
                    (
                        1152,
                        AvatarInfo::new("BuiltForThis_0", "BuiltForThis", 0, false),
                    ),
                    (1153, AvatarInfo::new("Happy_0", "Happy", 0, false)),
                    (1154, AvatarInfo::new("Summer_0", "Summer", 0, false)),
                    (
                        1155,
                        AvatarInfo::new("WalkThisWay_1", "WalkThisWay", 1, false),
                    ),
                    (1156, AvatarInfo::new("Tetris_3", "Tetris", 3, false)),
                    (1157, AvatarInfo::new("Macarena_3", "Macarena", 3, false)),
                    (1158, AvatarInfo::new("Birthday_0", "Birthday", 0, false)),
                    (
                        1159,
                        AvatarInfo::new("HoldingOut_0", "HoldingOut", 0, false),
                    ),
                    (
                        1160,
                        AvatarInfo::new("PoundTheAlarm_0", "PoundTheAlarm", 0, false),
                    ),
                    (1161, AvatarInfo::new("Blame_0", "Blame", 0, false)),
                    (1162, AvatarInfo::new("Animals_0", "Animals", 0, false)),
                    (
                        1163,
                        AvatarInfo::new("WilliamTell_0", "WilliamTell", 0, false),
                    ),
                    (
                        1164,
                        AvatarInfo::new("BornThisWay_1", "BornThisWay", 1, false),
                    ),
                    (1165, AvatarInfo::new("Circus_2", "Circus", 2, false)),
                    (
                        1166,
                        AvatarInfo::new("UptownFunk_0_C3", "UptownFunk", 0, false),
                    ),
                    (1167, AvatarInfo::new("Chiwawa_0", "Chiwawa", 0, false)),
                    (
                        1168,
                        AvatarInfo::new("ElectroMambo_0", "ElectroMambo", 0, false),
                    ),
                    (1169, AvatarInfo::new("HeyMama_1", "HeyMama", 1, false)),
                    (1170, AvatarInfo::new("KaboomPow_0", "KaboomPow", 0, false)),
                    (1171, AvatarInfo::new("ThisIsHow_0", "ThisIsHow", 0, false)),
                    (
                        1172,
                        AvatarInfo::new("DieYoungDLC_0", "DieYoungDLC", 0, false),
                    ),
                    (1173, AvatarInfo::new("BoomDLC_0", "BoomDLC", 0, false)),
                    (1174, AvatarInfo::new("Cotton_0", "Cotton", 0, false)),
                    (1175, AvatarInfo::new("Fame_0", "Fame", 0, false)),
                    (
                        1176,
                        AvatarInfo::new("TribalDance_0", "TribalDance", 0, false),
                    ),
                    (
                        1177,
                        AvatarInfo::new("GoodFeeling_0", "GoodFeeling", 0, false),
                    ),
                    (1178, AvatarInfo::new("JaiHo_0", "JaiHo", 0, false)),
                    (
                        1179,
                        AvatarInfo::new("KetchupSong_0", "KetchupSong", 0, false),
                    ),
                    (1180, AvatarInfo::new("Kurio_1", "Kurio", 1, false)),
                    (1181, AvatarInfo::new("Lollipop_0", "Lollipop", 0, false)),
                    (
                        1182,
                        AvatarInfo::new("ThatsTheWay_0", "ThatsTheWay", 0, false),
                    ),
                    (
                        1183,
                        AvatarInfo::new("TheFinalCountdown_0", "TheFinalCountdown", 0, false),
                    ),
                    (1184, AvatarInfo::new("ET_0", "ET", 0, false)),
                    (
                        1185,
                        AvatarInfo::new("CrazyLittle_1", "CrazyLittle", 1, false),
                    ),
                    (
                        1186,
                        AvatarInfo::new("CryingBlood_0", "CryingBlood", 0, false),
                    ),
                    (
                        1187,
                        AvatarInfo::new("GirlsJustWant_0", "GirlsJustWant", 0, false),
                    ),
                    (
                        1188,
                        AvatarInfo::new("ScreamNShoutALT_0", "ScreamNShoutALT", 0, false),
                    ),
                    (
                        1189,
                        AvatarInfo::new("WhatIsLove_0", "WhatIsLove", 0, false),
                    ),
                    (
                        1190,
                        AvatarInfo::new("DontStopMe_0", "DontStopMe", 0, false),
                    ),
                    (1191, AvatarInfo::new("PoPiPo_2", "PoPiPo", 2, false)),
                    (1192, AvatarInfo::new("September_1", "September", 1, false)),
                    (1193, AvatarInfo::new("WorthIt_0", "WorthIt", 0, false)),
                    (1194, AvatarInfo::new("Titanium_0", "Titanium", 0, false)),
                    (
                        1195,
                        AvatarInfo::new("DragosteaDinTei_1", "DragosteaDinTei", 1, false),
                    ),
                    (1196, AvatarInfo::new("Bailar_0", "Bailar", 0, false)),
                    (
                        1197,
                        AvatarInfo::new("HappyFarmKids_0", "HappyFarmKids", 0, false),
                    ),
                    (1198, AvatarInfo::new("TumBum_1", "TumBum", 1, false)),
                    (1199, AvatarInfo::new("Footloose_0", "Footloose", 0, false)),
                    (1200, AvatarInfo::new("24K_0", "24K", 0, false)),
                    (1201, AvatarInfo::new("Automaton_0", "Automaton", 0, false)),
                    (
                        1202,
                        AvatarInfo::new("BubblePopALT_1", "BubblePopALT", 1, false),
                    ),
                    (1203, AvatarInfo::new("LoveWard_0", "LoveWard", 0, false)),
                    (
                        1204,
                        AvatarInfo::new("AllYouGotta_0", "AllYouGotta", 0, false),
                    ),
                    (
                        1205,
                        AvatarInfo::new("AnotherOne_3", "AnotherOne", 3, false),
                    ),
                    (1206, AvatarInfo::new("BubblePop_1", "BubblePop", 1, false)),
                    (1207, AvatarInfo::new("Despacito_3", "Despacito", 3, false)),
                    (
                        1208,
                        AvatarInfo::new("SwishSwish_2", "SwishSwish", 2, false),
                    ),
                    (
                        1209,
                        AvatarInfo::new("SwishSwish_3", "SwishSwish", 3, false),
                    ),
                    (1210, AvatarInfo::new("MamaMia_0", "MamaMia", 0, false)),
                    (
                        1211,
                        AvatarInfo::new("WorkWorkALT_0", "WorkWorkALT", 0, false),
                    ),
                    (1212, AvatarInfo::new("Adeyyo_0", "Adeyyo", 0, false)),
                    (
                        1213,
                        AvatarInfo::new("MerryChristmasKids_1", "MerryChristmasKids", 1, false),
                    ),
                    (1214, AvatarInfo::new("MiMiMi_1", "MiMiMi", 1, false)),
                    (
                        1215,
                        AvatarInfo::new("MonstersAcademyKids_0", "MonstersAcademyKids", 0, false),
                    ),
                    (1216, AvatarInfo::new("Finesse_1", "Finesse", 1, false)),
                    (
                        1217,
                        AvatarInfo::new("IFeelItComing_0", "IFeelItComing", 0, false),
                    ),
                    (1218, AvatarInfo::new("MiloscW_0", "MiloscW", 0, false)),
                    (1219, AvatarInfo::new("MadLove_1", "MadLove", 1, false)),
                    (1220, AvatarInfo::new("OMG_2", "OMG", 2, false)),
                    (
                        1221,
                        AvatarInfo::new("NoTearsLeft_0_C1", "NoTearsLeft", 0, false),
                    ),
                    (1222, AvatarInfo::new("OneKiss_0", "OneKiss", 0, false)),
                    (
                        1223,
                        AvatarInfo::new("SweetSensation_3", "SweetSensation", 3, false),
                    ),
                    (1224, AvatarInfo::new("DDUDU_3", "DDUDU", 3, false)),
                    (1225, AvatarInfo::new("Sugar_0", "Sugar", 0, false)),
                    (1226, AvatarInfo::new("Sugar_1", "Sugar", 1, false)),
                    (1227, AvatarInfo::new("ConCalma_0", "ConCalma", 0, false)),
                    (1228, AvatarInfo::new("ConCalma_1", "ConCalma", 1, false)),
                    (
                        1229,
                        AvatarInfo::new("ConCalma_1_Gold", "ConCalma", 1, true),
                    ),
                    (1231, AvatarInfo::new("Sushii_0", "Sushii", 0, false)),
                    (1232, AvatarInfo::new("Sushii_0_Gold", "Sushii", 0, true)),
                    (1236, AvatarInfo::new("SoyYoALT_0", "SoyYoALT", 0, false)),
                    (1237, AvatarInfo::new("SoyYoALT_1", "SoyYoALT", 1, false)),
                    (
                        1238,
                        AvatarInfo::new("SoyYoALT_0_Gold", "SoyYoALT", 0, true),
                    ),
                    (1243, AvatarInfo::new("Bangarang_0", "Bangarang", 0, false)),
                    (
                        1244,
                        AvatarInfo::new("Bangarang_0_Gold", "Bangarang", 0, true),
                    ),
                    (1245, AvatarInfo::new("365_0", "365", 0, false)),
                    (1246, AvatarInfo::new("365_0_Gold", "365", 0, true)),
                    (
                        1247,
                        AvatarInfo::new("TheTimeALT_0", "TheTimeALT", 0, false),
                    ),
                    (
                        1248,
                        AvatarInfo::new("TheTimeALT_0_Gold", "TheTimeALT", 0, true),
                    ),
                    (
                        1249,
                        AvatarInfo::new("IAmTheBestALT_0", "IAmTheBestALT", 0, false),
                    ),
                    (
                        1250,
                        AvatarInfo::new("IAmTheBestALT_0_Gold", "IAmTheBestALT", 0, true),
                    ),
                    (
                        1251,
                        AvatarInfo::new("RainOverMeALT_0", "RainOverMeALT", 0, false),
                    ),
                    (
                        1252,
                        AvatarInfo::new("RainOverMeALT_0_Gold", "RainOverMeALT", 0, true),
                    ),
                    // (1253, "?"), // Ubisoft
                    // (1254, "?"), // Ubisoft
                    // (1255, "?"), // Ubisoft
                    // (1256, "?"), // Ubisoft
                    // (1257, "?"), // Ubisoft
                    (1258, AvatarInfo::new("ILikeIt_0", "ILikeIt", 0, false)),
                    (1259, AvatarInfo::new("ILikeIt_1", "ILikeIt", 1, false)),
                    (1260, AvatarInfo::new("ILikeIt_2", "ILikeIt", 2, false)),
                    (1261, AvatarInfo::new("ILikeIt_1_Gold", "ILikeIt", 1, true)),
                    // (1262, "?"), // Ubisoft
                    (
                        1263,
                        AvatarInfo::new("GodIsAWomanALT_0", "GodIsAWomanALT", 0, false),
                    ),
                    (
                        1264,
                        AvatarInfo::new("GodIsAWomanALT_0_Gold", "GodIsAWomanALT", 0, true),
                    ),
                    (1265, AvatarInfo::new("SoyYo_0", "SoyYo", 0, false)),
                    (1266, AvatarInfo::new("SoyYo_0_Gold", "SoyYo", 0, true)),
                    (1267, AvatarInfo::new("TelAviv_0", "TelAviv", 0, false)),
                    (1268, AvatarInfo::new("TelAviv_1", "TelAviv", 1, false)),
                    (1269, AvatarInfo::new("TelAviv_2", "TelAviv", 2, false)),
                    (1270, AvatarInfo::new("TelAviv_2_Gold", "TelAviv", 2, true)),
                    (1271, AvatarInfo::new("Skibidi_0", "Skibidi", 0, false)),
                    (1272, AvatarInfo::new("Skibidi_1", "Skibidi", 1, false)),
                    (1273, AvatarInfo::new("Skibidi_0_Gold", "Skibidi", 0, true)),
                    (1274, AvatarInfo::new("TakiTaki_0", "TakiTaki", 0, false)),
                    (
                        1275,
                        AvatarInfo::new("TakiTaki_0_Gold", "TakiTaki", 0, true),
                    ),
                    // (1279, "?"), // Ubisoft
                    // (1280, "?"), // Ubisoft
                    (1281, AvatarInfo::new("Swag_0", "Swag", 0, false)),
                    (1282, AvatarInfo::new("Swag_0_Gold", "Swag", 0, true)),
                    (1283, AvatarInfo::new("SushiiALT_0", "SushiiALT", 0, false)),
                    (
                        1284,
                        AvatarInfo::new("SushiiALT_0_Gold", "SushiiALT", 0, true),
                    ),
                    (
                        1285,
                        AvatarInfo::new("BangarangALT_0", "BangarangALT", 0, false),
                    ),
                    (
                        1286,
                        AvatarInfo::new("BangarangALT_0_Gold", "BangarangALT", 0, true),
                    ),
                    (
                        1287,
                        AvatarInfo::new("TakiTakiALT_0", "TakiTakiALT", 0, false),
                    ),
                    (
                        1288,
                        AvatarInfo::new("TakiTakiALT_1", "TakiTakiALT", 1, false),
                    ),
                    (
                        1289,
                        AvatarInfo::new("TakiTakiALT_1_Gold", "TakiTakiALT", 1, true),
                    ),
                    (1290, AvatarInfo::new("GetBusy_0", "GetBusy", 0, false)),
                    (1291, AvatarInfo::new("GetBusy_1", "GetBusy", 1, false)),
                    (1292, AvatarInfo::new("GetBusy_1_Gold", "GetBusy", 1, true)),
                    (1293, AvatarInfo::new("HighHopes_0", "HighHopes", 0, false)),
                    (1294, AvatarInfo::new("HighHopes_1", "HighHopes", 1, false)),
                    (1295, AvatarInfo::new("HighHopes_2", "HighHopes", 2, false)),
                    (1296, AvatarInfo::new("HighHopes_3", "HighHopes", 3, false)),
                    (
                        1297,
                        AvatarInfo::new("HighHopes_0_Gold", "HighHopes", 0, true),
                    ),
                    (
                        1298,
                        AvatarInfo::new("KillThisLove_0", "KillThisLove", 0, false),
                    ),
                    (
                        1299,
                        AvatarInfo::new("KillThisLove_1", "KillThisLove", 1, false),
                    ),
                    (
                        1300,
                        AvatarInfo::new("KillThisLove_2", "KillThisLove", 2, false),
                    ),
                    (
                        1301,
                        AvatarInfo::new("KillThisLove_3", "KillThisLove", 3, false),
                    ),
                    (
                        1302,
                        AvatarInfo::new("KillThisLove_2_Gold", "KillThisLove", 2, true),
                    ),
                    (1303, AvatarInfo::new("Talk_0", "Talk", 0, false)),
                    (1304, AvatarInfo::new("Talk_0_Gold", "Talk", 0, true)),
                    (1306, AvatarInfo::new("MaItu_0", "MaItu", 0, false)),
                    (1307, AvatarInfo::new("MaItu_0_Gold", "MaItu", 0, true)),
                    (1308, AvatarInfo::new("Everybody_0", "Everybody", 0, false)),
                    (1309, AvatarInfo::new("Everybody_1", "Everybody", 1, false)),
                    (1310, AvatarInfo::new("Everybody_2", "Everybody", 2, false)),
                    (1311, AvatarInfo::new("Everybody_3", "Everybody", 3, false)),
                    (
                        1312,
                        AvatarInfo::new("Everybody_2_Gold", "Everybody", 2, true),
                    ),
                    (
                        1313,
                        AvatarInfo::new("LaRespuesta_0", "LaRespuesta", 0, false),
                    ),
                    (
                        1314,
                        AvatarInfo::new("LaRespuesta_1", "LaRespuesta", 1, false),
                    ),
                    (
                        1315,
                        AvatarInfo::new("LaRespuesta_0_Gold", "LaRespuesta", 0, true),
                    ),
                    (1316, AvatarInfo::new("TalkALT_0", "TalkALT", 0, false)),
                    (1317, AvatarInfo::new("TalkALT_0_Gold", "TalkALT", 0, true)),
                    (
                        1318,
                        AvatarInfo::new("OldTownRoad_0", "OldTownRoad", 0, false),
                    ),
                    (
                        1319,
                        AvatarInfo::new("OldTownRoad_0_Gold", "OldTownRoad", 0, true),
                    ),
                    (
                        1320,
                        AvatarInfo::new("NotYourOrdinary_3_Gold", "NotYourOrdinary", 3, true),
                    ),
                    (1321, AvatarInfo::new("ILikeIt_0_Gold", "ILikeIt", 0, true)),
                    (1322, AvatarInfo::new("7Rings_0", "7Rings", 0, false)),
                    (1323, AvatarInfo::new("7Rings_1", "7Rings", 1, false)),
                    (1324, AvatarInfo::new("7Rings_2", "7Rings", 2, false)),
                    (1325, AvatarInfo::new("7Rings_1_Gold", "7Rings", 1, true)),
                    (1326, AvatarInfo::new("BadGuy_0", "BadGuy", 0, false)),
                    (1327, AvatarInfo::new("BadGuy_0_Gold", "BadGuy", 0, true)),
                    (1328, AvatarInfo::new("Footwork_0", "Footwork", 0, false)),
                    (
                        1329,
                        AvatarInfo::new("Footwork_0_Gold", "Footwork", 0, true),
                    ),
                    (
                        1330,
                        AvatarInfo::new("OldTownRoadALT_0", "OldTownRoadALT", 0, false),
                    ),
                    (
                        1331,
                        AvatarInfo::new("OldTownRoadALT_1", "OldTownRoadALT", 1, false),
                    ),
                    (
                        1332,
                        AvatarInfo::new("OldTownRoadALT_2", "OldTownRoadALT", 2, false),
                    ),
                    (
                        1333,
                        AvatarInfo::new("OldTownRoadALT_2_Gold", "OldTownRoadALT", 2, true),
                    ),
                    (
                        1334,
                        AvatarInfo::new("JustAnIllusion_0", "JustAnIllusion", 0, false),
                    ),
                    (
                        1335,
                        AvatarInfo::new("JustAnIllusion_1", "JustAnIllusion", 1, false),
                    ),
                    (
                        1336,
                        AvatarInfo::new("JustAnIllusion_1_Gold", "JustAnIllusion", 1, true),
                    ),
                    (1337, AvatarInfo::new("StopMovin_0", "StopMovin", 0, false)),
                    (1338, AvatarInfo::new("StopMovin_1", "StopMovin", 1, false)),
                    (1339, AvatarInfo::new("StopMovin_2", "StopMovin", 2, false)),
                    (
                        1340,
                        AvatarInfo::new("StopMovin_1_Gold", "StopMovin", 1, true),
                    ),
                    (1342, AvatarInfo::new("7RingsALT_0", "7RingsALT", 0, false)),
                    (
                        1343,
                        AvatarInfo::new("7RingsALT_0_Gold", "7RingsALT", 0, true),
                    ),
                    (
                        1344,
                        AvatarInfo::new("KillThisLoveALT_0", "KillThisLoveALT", 0, false),
                    ),
                    (
                        1345,
                        AvatarInfo::new("KillThisLoveALT_0_Gold", "KillThisLoveALT", 0, true),
                    ),
                    (
                        1346,
                        AvatarInfo::new("BassaSababa_0", "BassaSababa", 0, false),
                    ),
                    (
                        1347,
                        AvatarInfo::new("BassaSababa_0_Gold", "BassaSababa", 0, true),
                    ),
                    (
                        1348,
                        AvatarInfo::new("FancyTwice_0", "FancyTwice", 0, false),
                    ),
                    (
                        1349,
                        AvatarInfo::new("FancyTwice_1", "FancyTwice", 1, false),
                    ),
                    (
                        1350,
                        AvatarInfo::new("FancyTwice_2", "FancyTwice", 2, false),
                    ),
                    (
                        1351,
                        AvatarInfo::new("UglyBeauty_0", "UglyBeauty", 0, false),
                    ),
                    (
                        1352,
                        AvatarInfo::new("UglyBeauty_1", "UglyBeauty", 1, false),
                    ),
                    (
                        1353,
                        AvatarInfo::new("UglyBeauty_2", "UglyBeauty", 2, false),
                    ),
                    (
                        1354,
                        AvatarInfo::new("DoCarnaval_0", "DoCarnaval", 0, false),
                    ),
                    (
                        1355,
                        AvatarInfo::new("DoCarnaval_0_Gold", "DoCarnaval", 0, true),
                    ),
                    (1356, AvatarInfo::new("ConAltura_0", "ConAltura", 0, false)),
                    (
                        1357,
                        AvatarInfo::new("ConAltura_0_Gold", "ConAltura", 0, true),
                    ),
                    (1358, AvatarInfo::new("IDontCare_0", "IDontCare", 0, false)),
                    (
                        1359,
                        AvatarInfo::new("IDontCare_0_Gold", "IDontCare", 0, true),
                    ),
                    (
                        1360,
                        AvatarInfo::new("UglyBeauty_0_Gold", "UglyBeauty", 0, true),
                    ),
                    (
                        1361,
                        AvatarInfo::new("FancyTwice_2_Gold", "FancyTwice", 2, true),
                    ),
                    (1362, AvatarInfo::new("CanCan_0", "CanCan", 0, false)),
                    (1363, AvatarInfo::new("CanCan_1", "CanCan", 1, false)),
                    (1364, AvatarInfo::new("CanCan_2", "CanCan", 2, false)),
                    (1365, AvatarInfo::new("CanCan_3", "CanCan", 3, false)),
                    (1366, AvatarInfo::new("CanCan_4", "CanCan", 4, false)),
                    (1367, AvatarInfo::new("CanCan_1_Gold", "CanCan", 1, true)),
                    (
                        1368,
                        AvatarInfo::new("BoyWithLuv_0", "BoyWithLuv", 0, false),
                    ),
                    (
                        1369,
                        AvatarInfo::new("BoyWithLuv_1", "BoyWithLuv", 1, false),
                    ),
                    (
                        1370,
                        AvatarInfo::new("BoyWithLuv_2", "BoyWithLuv", 2, false),
                    ),
                    (
                        1371,
                        AvatarInfo::new("BoyWithLuv_2_Gold", "BoyWithLuv", 2, true),
                    ),
                ]),
            ),
            (
                Game::JustDance2019,
                HashMap::from([
                    (1, AvatarInfo::new("Dare_0", "Dare", 0, false)),
                    (2, AvatarInfo::new("DogsOut_0", "DogsOut", 0, false)),
                    (
                        3,
                        AvatarInfo::new("EyeOfTheTiger_0", "EyeOfTheTiger", 0, false),
                    ),
                    (4, AvatarInfo::new("GetAround_0", "GetAround", 0, false)),
                    (5, AvatarInfo::new("HotNCold_0", "HotNCold", 0, false)),
                    (
                        6,
                        AvatarInfo::new("ILikeToMoveIt_0", "ILikeToMoveIt", 0, false),
                    ),
                    (7, AvatarInfo::new("JinGoLoBa_0", "JinGoLoBa", 0, false)),
                    (8, AvatarInfo::new("RingMyBell_0", "RingMyBell", 0, false)),
                    (
                        9,
                        AvatarInfo::new("NineAfternoon_0", "NineAfternoon", 0, false),
                    ),
                    (10, AvatarInfo::new("BabyGirl_0", "BabyGirl", 0, false)),
                    (11, AvatarInfo::new("CallMe_0", "CallMe", 0, false)),
                    (
                        12,
                        AvatarInfo::new("ChickenPayback_0", "ChickenPayback", 0, false),
                    ),
                    (13, AvatarInfo::new("ComeOn_1", "ComeOn", 1, false)),
                    (14, AvatarInfo::new("CosmicGirl_0", "CosmicGirl", 0, false)),
                    (
                        15,
                        AvatarInfo::new("ElectroTribalDLC_0", "ElectroTribalDLC", 0, false),
                    ),
                    (16, AvatarInfo::new("Firework_0", "Firework", 0, false)),
                    (
                        76,
                        AvatarInfo::new("EverybodyNeeds_0", "EverybodyNeeds", 0, false),
                    ),
                    (
                        77,
                        AvatarInfo::new("FunHouseDLC_0", "FunHouseDLC", 0, false),
                    ),
                    (252, AvatarInfo::new("Animals_0", "Animals", 0, false)),
                    (
                        347,
                        AvatarInfo::new("GangnamStyleDLC_0", "GangnamStyleDLC", 0, false),
                    ),
                    (
                        694,
                        AvatarInfo::new("BlowYourMind_0", "BlowYourMind", 0, false),
                    ),
                    (695, AvatarInfo::new("AnotherOne_1", "AnotherOne", 1, false)),
                    (697, AvatarInfo::new("Carmen_0", "Carmen", 0, false)),
                    (700, AvatarInfo::new("Blue_0", "Blue", 0, false)),
                    (
                        704,
                        AvatarInfo::new("HappyFarmKids_0", "HappyFarmKids", 0, false),
                    ),
                    (705, AvatarInfo::new("SideTo_0", "SideTo", 0, false)),
                    (
                        706,
                        AvatarInfo::new("MakeItJingle_0", "MakeItJingle", 0, false),
                    ),
                    (
                        709,
                        AvatarInfo::new("AnotherOneALT_0", "AnotherOneALT", 0, false),
                    ),
                    (710, AvatarInfo::new("SideToALT_0", "SideToALT", 0, false)),
                    (
                        711,
                        AvatarInfo::new("WakaWakaALT_0", "WakaWakaALT", 0, false),
                    ),
                    (713, AvatarInfo::new("8BitRetake_0", "8BitRetake", 0, false)),
                    (
                        714,
                        AvatarInfo::new("AutomatonALT_0", "AutomatonALT", 0, false),
                    ),
                    (716, AvatarInfo::new("Rockabye_1", "Rockabye", 1, false)),
                    (
                        718,
                        AvatarInfo::new("BubblePopALT_3", "BubblePopALT", 3, false),
                    ),
                    (720, AvatarInfo::new("Footloose_0", "Footloose", 0, false)), // C3
                    (722, AvatarInfo::new("24KALT_0", "24KALT", 0, false)),
                    (723, AvatarInfo::new("Diggy_0", "Diggy", 0, false)),
                    (724, AvatarInfo::new("Chantaje_0", "Chantaje", 0, false)),
                    (725, AvatarInfo::new("LoveWard_1", "LoveWard", 1, false)),
                    (726, AvatarInfo::new("TumBumALT_0", "TumBumALT", 0, false)),
                    (
                        727,
                        AvatarInfo::new("SayonaraRetake_0", "SayonaraRetake", 0, false),
                    ),
                    (
                        729,
                        AvatarInfo::new("ChantajeALT_1", "ChantajeALT", 1, false),
                    ),
                    (730, AvatarInfo::new("Automaton_0", "Automaton", 0, false)),
                    (732, AvatarInfo::new("BubblePop_2", "BubblePop", 2, false)),
                    (
                        738,
                        AvatarInfo::new("ItsyBitsyRetake_1", "ItsyBitsyRetake", 1, false),
                    ),
                    (
                        739,
                        AvatarInfo::new("ItsyBitsyRetake_0", "ItsyBitsyRetake", 0, false),
                    ),
                    (
                        741,
                        AvatarInfo::new("WakaWakaKids_0", "WakaWakaKids", 0, false),
                    ),
                    (
                        742,
                        AvatarInfo::new("MagicHalloweenKids_1", "MagicHalloweenKids", 1, false),
                    ),
                    (
                        743,
                        AvatarInfo::new("MagicHalloweenKids_0", "MagicHalloweenKids", 0, false),
                    ),
                    (
                        748,
                        AvatarInfo::new("BubblePopALT_2", "BubblePopALT", 2, false),
                    ),
                    (
                        749,
                        AvatarInfo::new("BubblePopALT_1", "BubblePopALT", 1, false),
                    ),
                    (
                        750,
                        AvatarInfo::new("BubblePopALT_0", "BubblePopALT", 0, false),
                    ),
                    (
                        775,
                        AvatarInfo::new("FootlooseKids_0", "FootlooseKids", 0, false),
                    ),
                    (776, AvatarInfo::new("LoveWard_0", "LoveWard", 0, false)),
                    (778, AvatarInfo::new("KeepOn_0", "KeepOn", 0, false)),
                    (
                        780,
                        AvatarInfo::new("RiskyBusiness_0", "RiskyBusiness", 0, false),
                    ),
                    (781, AvatarInfo::new("Dharma_0", "Dharma", 0, false)),
                    (
                        782,
                        AvatarInfo::new("Cottonmouth_0", "Cottonmouth", 0, false),
                    ),
                    (785, AvatarInfo::new("DharmaALT_0", "DharmaALT", 0, false)),
                    (808, AvatarInfo::new("ShapeOfYou_0", "ShapeOfYou", 0, false)),
                    (809, AvatarInfo::new("AnotherOne_2", "AnotherOne", 2, false)),
                    (810, AvatarInfo::new("BadLiar_0", "BadLiar", 0, false)),
                    (
                        811,
                        AvatarInfo::new("KissingStrangers_1", "KissingStrangers", 1, false),
                    ),
                    (
                        812,
                        AvatarInfo::new("MissAmazingKIDS_0", "MissAmazingKIDS", 0, false),
                    ),
                    (813, AvatarInfo::new("NewFace_1", "NewFace", 1, false)),
                    (
                        814,
                        AvatarInfo::new("SlumberParty_1", "SlumberParty", 1, false),
                    ),
                    (
                        816,
                        AvatarInfo::new("FunkyRobotKids_0", "FunkyRobotKids", 0, false),
                    ),
                    (
                        817,
                        AvatarInfo::new("PixieLandKids_0", "PixieLandKids", 0, false),
                    ),
                    (818, AvatarInfo::new("Sidewinder_0", "Sidewinder", 0, false)),
                    (
                        820,
                        AvatarInfo::new("AllYouGotta_0", "AllYouGotta", 0, false),
                    ),
                    (821, AvatarInfo::new("AnotherOne_3", "AnotherOne", 3, false)),
                    (822, AvatarInfo::new("BubblePop_0", "BubblePop", 0, false)),
                    (823, AvatarInfo::new("BubblePop_1", "BubblePop", 1, false)),
                    (824, AvatarInfo::new("Copperhead_1", "Copperhead", 1, false)),
                    (825, AvatarInfo::new("Despacito_3", "Despacito", 3, false)),
                    (
                        826,
                        AvatarInfo::new("DespacitoALT_0", "DespacitoALT", 0, false),
                    ),
                    (827, AvatarInfo::new("HowFar_0", "HowFar", 0, false)),
                    (
                        828,
                        AvatarInfo::new("Instruction_0", "Instruction", 0, false),
                    ),
                    (829, AvatarInfo::new("JohnW_0", "JohnW", 0, false)),
                    (
                        830,
                        AvatarInfo::new("KissingStrangers_0", "KissingStrangers", 0, false),
                    ),
                    (
                        831,
                        AvatarInfo::new("KissingStrangersALT_1", "KissingStrangersALT", 1, false),
                    ),
                    (
                        832,
                        AvatarInfo::new("NaughtyGirl_0", "NaughtyGirl", 0, false),
                    ),
                    (833, AvatarInfo::new("SwishSwish_0", "SwishSwish", 0, false)),
                    (834, AvatarInfo::new("SwishSwish_1", "SwishSwish", 1, false)),
                    (835, AvatarInfo::new("SwishSwish_2", "SwishSwish", 2, false)),
                    (836, AvatarInfo::new("SwishSwish_3", "SwishSwish", 3, false)),
                    (
                        837,
                        AvatarInfo::new("WakaWakaALT_1", "WakaWakaALT", 1, false),
                    ),
                    (
                        883,
                        AvatarInfo::new("WhereAreYou_2", "WhereAreYou", 2, false),
                    ),
                    (
                        884,
                        AvatarInfo::new("ObsessionRetake_0", "ObsessionRetake", 0, false),
                    ),
                    (885, AvatarInfo::new("MamaMia_0", "MamaMia", 0, false)),
                    (886, AvatarInfo::new("GhostKids_0", "GhostKids", 0, false)),
                    (887, AvatarInfo::new("GhostKids_1", "GhostKids", 1, false)),
                    (
                        888,
                        AvatarInfo::new("JurassicKids_0", "JurassicKids", 0, false),
                    ),
                    (
                        889,
                        AvatarInfo::new("WorkWorkALT_0", "WorkWorkALT", 0, false),
                    ),
                    (
                        890,
                        AvatarInfo::new("ImStillStanding_0", "ImStillStanding", 0, false),
                    ),
                    (
                        891,
                        AvatarInfo::new("SpaceGirlKids_0", "SpaceGirlKids", 0, false),
                    ),
                    (892, AvatarInfo::new("Fire_0", "Fire", 0, false)),
                    (893, AvatarInfo::new("CaPlane_0", "CaPlane", 0, false)),
                    (894, AvatarInfo::new("Shaky_0", "Shaky", 0, false)),
                    (
                        895,
                        AvatarInfo::new("SaintPatrickKids_0", "SaintPatrickKids", 0, false),
                    ),
                    (
                        896,
                        AvatarInfo::new("SaintPatrickKids_1", "SaintPatrickKids", 1, false),
                    ),
                    (897, AvatarInfo::new("Adeyyo_0", "Adeyyo", 0, false)),
                    (
                        898,
                        AvatarInfo::new("MerryChristmasKids_0", "MerryChristmasKids", 0, false),
                    ),
                    (
                        899,
                        AvatarInfo::new("MerryChristmasKids_1", "MerryChristmasKids", 1, false),
                    ),
                    (900, AvatarInfo::new("NinjaKids_0", "NinjaKids", 0, false)),
                    (901, AvatarInfo::new("MadLove_0", "MadLove", 0, false)),
                    (
                        902,
                        AvatarInfo::new("LittlePartyALT_1", "LittlePartyALT", 1, false),
                    ),
                    (903, AvatarInfo::new("MiMiMi_0", "MiMiMi", 0, false)),
                    (904, AvatarInfo::new("MiMiMi_1", "MiMiMi", 1, false)),
                    (
                        905,
                        AvatarInfo::new("TheExplorerKids_0", "TheExplorerKids", 0, false),
                    ),
                    (906, AvatarInfo::new("WorkWork_0", "WorkWork", 0, false)),
                    (
                        907,
                        AvatarInfo::new("MonstersAcademyKids_0", "MonstersAcademyKids", 0, false),
                    ),
                    (908, AvatarInfo::new("Narco_0", "Narco", 0, false)),
                    (
                        909,
                        AvatarInfo::new("BumBumTamTamALT_0", "BumBumTamTamALT", 0, false),
                    ),
                    (910, AvatarInfo::new("Finesse_1", "Finesse", 1, false)),
                    (
                        911,
                        AvatarInfo::new("LittleParty_0", "LittleParty", 0, false),
                    ),
                    (912, AvatarInfo::new("Rhythm_0", "Rhythm", 0, false)),
                    (
                        913,
                        AvatarInfo::new("IFeelItComing_0", "IFeelItComing", 0, false),
                    ),
                    (914, AvatarInfo::new("Havana_0", "Havana", 0, false)),
                    (915, AvatarInfo::new("MakeMeFeel_0", "MakeMeFeel", 0, false)),
                    (916, AvatarInfo::new("WaterMe_0", "WaterMe", 0, false)),
                    (917, AvatarInfo::new("MiloscW_0", "MiloscW", 0, false)),
                    (918, AvatarInfo::new("FinesseALT_0", "FinesseALT", 0, false)),
                    (919, AvatarInfo::new("MiMiMiALT_0", "MiMiMiALT", 0, false)),
                    (920, AvatarInfo::new("MiMiMiALT_1", "MiMiMiALT", 1, false)),
                    (921, AvatarInfo::new("MiMiMiALT_2", "MiMiMiALT", 2, false)),
                    (922, AvatarInfo::new("MiMiMiALT_3", "MiMiMiALT", 3, false)),
                    (
                        923,
                        AvatarInfo::new("BumBumTamTamALT_1", "BumBumTamTamALT", 1, false),
                    ),
                    (924, AvatarInfo::new("MadLove_1", "MadLove", 1, false)),
                    (925, AvatarInfo::new("Finesse_0", "Finesse", 0, false)),
                    (926, AvatarInfo::new("Finesse_2", "Finesse", 2, false)),
                    (927, AvatarInfo::new("Finesse_3", "Finesse", 3, false)),
                    (
                        928,
                        AvatarInfo::new("ObsessionRetake_1", "ObsessionRetake", 1, false),
                    ),
                    (
                        929,
                        AvatarInfo::new("WhereAreYou_0", "WhereAreYou", 0, false),
                    ),
                    (
                        930,
                        AvatarInfo::new("WhereAreYou_1", "WhereAreYou", 1, false),
                    ),
                    (931, AvatarInfo::new("WorkWork_1", "WorkWork", 1, false)),
                    (932, AvatarInfo::new("WorkWork_2", "WorkWork", 2, false)),
                    (933, AvatarInfo::new("Fire_1", "Fire", 1, false)),
                    (934, AvatarInfo::new("MamaMia_1", "MamaMia", 1, false)),
                    (
                        935,
                        AvatarInfo::new("LittlePartyALT_0", "LittlePartyALT", 0, false),
                    ),
                    (936, AvatarInfo::new("Bang2019_0", "Bang2019", 0, false)),
                    (937, AvatarInfo::new("NewWorld_0", "NewWorld", 0, false)),
                    (938, AvatarInfo::new("NewReality_0", "NewReality", 0, false)),
                    (
                        939,
                        AvatarInfo::new("NotYourOrdinary_0", "NotYourOrdinary", 0, false),
                    ),
                    (
                        940,
                        AvatarInfo::new("NotYourOrdinary_1", "NotYourOrdinary", 1, false),
                    ),
                    (
                        941,
                        AvatarInfo::new("NotYourOrdinary_2", "NotYourOrdinary", 2, false),
                    ),
                    (
                        942,
                        AvatarInfo::new("NotYourOrdinary_3", "NotYourOrdinary", 3, false),
                    ),
                    (
                        943,
                        AvatarInfo::new("WhereAreYouALT_0", "WhereAreYouALT", 0, false),
                    ),
                    (
                        944,
                        AvatarInfo::new("WhereAreYouALT_1", "WhereAreYouALT", 1, false),
                    ),
                    (
                        945,
                        AvatarInfo::new("FireOnTheDancefloor_0", "FireOnTheDancefloor", 0, false),
                    ),
                    (
                        946,
                        AvatarInfo::new("LittlePartyALT_2", "LittlePartyALT", 2, false),
                    ),
                    (947, AvatarInfo::new("DameTu_0", "DameTu", 0, false)),
                    (948, AvatarInfo::new("WaterMeALT_0", "WaterMeALT", 0, false)),
                    (949, AvatarInfo::new("WaterMeALT_1", "WaterMeALT", 1, false)),
                    (950, AvatarInfo::new("NewRules_0", "NewRules", 0, false)),
                    (951, AvatarInfo::new("OMG_0", "OMG", 0, false)),
                    (952, AvatarInfo::new("OMG_1", "OMG", 1, false)),
                    (953, AvatarInfo::new("OMG_2", "OMG", 2, false)),
                    (954, AvatarInfo::new("OMGALT_0", "OMGALT", 0, false)),
                    (
                        955,
                        AvatarInfo::new("BumBumTamTam_0", "BumBumTamTam", 0, false),
                    ),
                    (
                        956,
                        AvatarInfo::new("BumBumTamTam_1", "BumBumTamTam", 1, false),
                    ),
                    (
                        959,
                        AvatarInfo::new("SweetLittle_0", "SweetLittle", 0, false),
                    ),
                    (
                        961,
                        AvatarInfo::new("NewRulesALT_0", "NewRulesALT", 0, false),
                    ),
                    (
                        962,
                        AvatarInfo::new(
                            "UbiSoftRainbowSixSiegeEla",
                            "UbiSoftRainbowSixSiege",
                            0,
                            false,
                        ),
                    ),
                    (
                        963,
                        AvatarInfo::new(
                            "UbiSoftRainbowSixSiegeTachanka",
                            "UbiSoftRainbowSixSiege",
                            1,
                            false,
                        ),
                    ),
                    (
                        964,
                        AvatarInfo::new(
                            "UbiSoftRainbowSixSiegeAsh",
                            "UbiSoftRainbowSixSiege",
                            2,
                            false,
                        ),
                    ),
                    (
                        965,
                        AvatarInfo::new(
                            "UbiSoftRainbowSixSiegeDokkaebi",
                            "UbiSoftRainbowSixSiege",
                            3,
                            false,
                        ),
                    ),
                    (
                        966,
                        AvatarInfo::new("UbiSoftACOdysseyAlexios", "UbiSoftACOdyssey", 0, false),
                    ),
                    (
                        967,
                        AvatarInfo::new("UbiSoftACOdysseyKassandra", "UbiSoftACOdyssey", 1, false),
                    ),
                    (
                        968,
                        AvatarInfo::new("UbiSoftWatchDogs2Sitara", "UbiSoftWatchDogs2", 0, false),
                    ),
                    (
                        969,
                        AvatarInfo::new("UbiSoftWatchDogs2Wrench", "UbiSoftWatchDogs2", 1, false),
                    ),
                    (982, AvatarInfo::new("PocoLoco_0", "PocoLoco", 0, false)),
                    (983, AvatarInfo::new("PacMan_0", "PacMan", 0, false)),
                    (984, AvatarInfo::new("PacMan_1", "PacMan", 1, false)),
                    (985, AvatarInfo::new("PacMan_2", "PacMan", 2, false)),
                    (986, AvatarInfo::new("PacMan_3", "PacMan", 3, false)),
                    (
                        987,
                        AvatarInfo::new("UbiSoftTheCrew2", "UbiSoftTheCrew2", 0, false),
                    ),
                    (988, AvatarInfo::new("MadLoveALT_0", "MadLoveALT", 0, false)),
                    (989, AvatarInfo::new("Familiar_0", "Familiar", 0, false)),
                    // (990, "UbiSoftJD2019Unknown", "UbiSoftJD2019Unknown", 0, false),
                    (991, AvatarInfo::new("TOY_0", "TOY", 0, false)),
                    (
                        992,
                        AvatarInfo::new("SangriaWine_0", "SangriaWine", 0, false),
                    ),
                    (
                        993,
                        AvatarInfo::new("Bang2019ALT_0", "Bang2019ALT", 0, false),
                    ),
                    (994, AvatarInfo::new("HavanaALT_0", "HavanaALT", 0, false)),
                    (995, AvatarInfo::new("HavanaALT_1", "HavanaALT", 1, false)),
                    (
                        996,
                        AvatarInfo::new("NoTearsLeft_0", "NoTearsLeft", 0, false),
                    ),
                    (
                        997,
                        AvatarInfo::new("NoTearsLeft_1", "NoTearsLeft", 1, false),
                    ),
                    (998, AvatarInfo::new("OneKiss_0", "OneKiss", 0, false)),
                    (999, AvatarInfo::new("Calypso_0", "Calypso", 0, false)),
                    (
                        1000,
                        AvatarInfo::new("SweetSensation_0", "SweetSensation", 0, false),
                    ),
                    (
                        1001,
                        AvatarInfo::new("SweetSensation_1", "SweetSensation", 1, false),
                    ),
                    (
                        1002,
                        AvatarInfo::new("SweetSensation_2", "SweetSensation", 2, false),
                    ),
                    (
                        1003,
                        AvatarInfo::new("SweetSensation_3", "SweetSensation", 3, false),
                    ),
                    (
                        1004,
                        AvatarInfo::new("NiceForWhat_0", "NiceForWhat", 0, false),
                    ),
                    (
                        1005,
                        AvatarInfo::new("NiceForWhat_1", "NiceForWhat", 1, false),
                    ),
                    (
                        1006,
                        AvatarInfo::new("NiceForWhat_2", "NiceForWhat", 2, false),
                    ),
                    (
                        1007,
                        AvatarInfo::new("NiceForWhat_3", "NiceForWhat", 3, false),
                    ),
                    (1008, AvatarInfo::new("Mayores_0", "Mayores", 0, false)),
                    (1009, AvatarInfo::new("RaveIn_0", "RaveIn", 0, false)),
                    (1010, AvatarInfo::new("RaveIn_1", "RaveIn", 1, false)),
                    (1011, AvatarInfo::new("RaveIn_2", "RaveIn", 2, false)),
                    (1012, AvatarInfo::new("RaveIn_3", "RaveIn", 3, false)),
                    (1013, AvatarInfo::new("DDUDU_0", "DDUDU", 0, false)),
                    (1014, AvatarInfo::new("DDUDU_1", "DDUDU", 1, false)),
                    (1015, AvatarInfo::new("DDUDU_2", "DDUDU", 2, false)),
                    (1016, AvatarInfo::new("DDUDU_3", "DDUDU", 3, false)),
                    (1019, AvatarInfo::new("Sugar_0", "Sugar", 0, false)),
                    (1020, AvatarInfo::new("Sugar_1", "Sugar", 1, false)),
                    (1021, AvatarInfo::new("Sugar_2", "Sugar", 2, false)),
                    (1022, AvatarInfo::new("Sugar_3", "Sugar", 3, false)),
                    (1023, AvatarInfo::new("Sugar_4", "Sugar", 4, false)),
                    (1024, AvatarInfo::new("Sugar_5", "Sugar", 5, false)),
                    (1025, AvatarInfo::new("Sugar_6", "Sugar", 6, false)),
                    (1026, AvatarInfo::new("Sugar_7", "Sugar", 7, false)),
                    (1027, AvatarInfo::new("Sugar_8", "Sugar", 8, false)),
                    (1028, AvatarInfo::new("Sugar_9", "Sugar", 9, false)),
                    (
                        1029,
                        AvatarInfo::new("NiceForWhat_4", "NiceForWhat", 4, false),
                    ),
                    (
                        1030,
                        AvatarInfo::new("NiceForWhat_5", "NiceForWhat", 5, false),
                    ),
                    (1031, AvatarInfo::new("JD_Panda", "JD", 0, false)),
                ]),
            ),
            (
                Game::JustDance2018,
                HashMap::from([
                    (1, AvatarInfo::new("Dare_0", "Dare", 0, false)),
                    (2, AvatarInfo::new("DogsOut_0", "DogsOut", 0, false)),
                    (
                        3,
                        AvatarInfo::new("EyeOfTheTiger_0", "EyeOfTheTiger", 0, false),
                    ),
                    (4, AvatarInfo::new("GetAround_0", "GetAround", 0, false)),
                    (5, AvatarInfo::new("HotNCold_0", "HotNCold", 0, false)),
                    (
                        6,
                        AvatarInfo::new("ILikeToMoveIt_0", "ILikeToMoveIt", 0, false),
                    ),
                    (7, AvatarInfo::new("JinGoLoBa_0", "JinGoLoBa", 0, false)),
                    (8, AvatarInfo::new("RingMyBell_0", "RingMyBell", 0, false)),
                    (
                        9,
                        AvatarInfo::new("NineAfternoon_0", "NineAfternoon", 0, false),
                    ),
                    (10, AvatarInfo::new("BabyGirl_0", "BabyGirl", 0, false)),
                    (11, AvatarInfo::new("CallMe_0", "CallMe", 0, false)),
                    (
                        12,
                        AvatarInfo::new("ChickenPayback_0", "ChickenPayback", 0, false),
                    ),
                    (13, AvatarInfo::new("ComeOn_1", "ComeOn", 1, false)),
                    (14, AvatarInfo::new("CosmicGirl_0", "CosmicGirl", 0, false)),
                    (
                        15,
                        AvatarInfo::new("ElectroTribalDLC_0", "ElectroTribalDLC", 0, false),
                    ),
                    (16, AvatarInfo::new("Firework_0", "Firework", 0, false)),
                    (
                        76,
                        AvatarInfo::new("EverybodyNeeds_0", "EverybodyNeeds", 0, false),
                    ),
                    (
                        77,
                        AvatarInfo::new("FunHouseDLC_0", "FunHouseDLC", 0, false),
                    ),
                    (252, AvatarInfo::new("Animals_0", "Animals", 0, false)),
                    (
                        347,
                        AvatarInfo::new("GangnamStyleDLC_0", "GangnamStyleDLC", 0, false),
                    ),
                    (
                        448,
                        AvatarInfo::new("Ghostbusters_0", "Ghostbusters", 0, false),
                    ),
                    (691, AvatarInfo::new("WakaWaka_1", "WakaWaka", 1, false)),
                    (
                        694,
                        AvatarInfo::new("BlowYourMind_0", "BlowYourMind", 0, false),
                    ),
                    (695, AvatarInfo::new("AnotherOne_2", "AnotherOne", 2, false)),
                    (697, AvatarInfo::new("Carmen_0", "Carmen", 0, false)),
                    (698, AvatarInfo::new("Carmen_1", "Carmen", 1, false)),
                    (699, AvatarInfo::new("DaddyCool_0", "DaddyCool", 0, false)),
                    (700, AvatarInfo::new("Blue_0", "Blue", 0, false)),
                    (
                        704,
                        AvatarInfo::new("HappyFarmKids_0", "HappyFarmKids", 0, false),
                    ),
                    (705, AvatarInfo::new("SideTo_0", "SideTo", 0, false)),
                    (
                        706,
                        AvatarInfo::new("MakeItJingle_0", "MakeItJingle", 0, false),
                    ),
                    (708, AvatarInfo::new("GotThat_0", "GotThat", 0, false)),
                    (
                        709,
                        AvatarInfo::new("AnotherOneALT_0", "AnotherOneALT", 0, false),
                    ),
                    (710, AvatarInfo::new("SideToALT_0", "SideToALT", 0, false)),
                    (
                        711,
                        AvatarInfo::new("WakaWakaALT_0", "WakaWakaALT", 0, false),
                    ),
                    (713, AvatarInfo::new("8BitRetake_0", "8BitRetake", 0, false)),
                    (
                        714,
                        AvatarInfo::new("AutomatonALT_0", "AutomatonALT", 0, false),
                    ),
                    (715, AvatarInfo::new("TumBum_1", "TumBum", 1, false)),
                    (716, AvatarInfo::new("Rockabye_1", "Rockabye", 1, false)),
                    (
                        717,
                        AvatarInfo::new("FearlessPirateKids_0", "FearlessPirateKids", 0, false),
                    ),
                    (
                        718,
                        AvatarInfo::new("BubblePopALT_3", "BubblePopALT", 3, false),
                    ),
                    (720, AvatarInfo::new("Footloose_0", "Footloose", 0, false)),
                    (721, AvatarInfo::new("24K_0", "24K", 0, false)),
                    (722, AvatarInfo::new("24KALT_0", "24KALT", 0, false)),
                    (723, AvatarInfo::new("Diggy_0", "Diggy", 0, false)),
                    (724, AvatarInfo::new("Chantaje_0", "Chantaje", 0, false)),
                    (725, AvatarInfo::new("LoveWard_1", "LoveWard", 1, false)),
                    (726, AvatarInfo::new("TumBumALT_0", "TumBumALT", 0, false)),
                    (
                        727,
                        AvatarInfo::new("SayonaraRetake_0", "SayonaraRetake", 0, false),
                    ),
                    (
                        729,
                        AvatarInfo::new("ChantajeALT_1", "ChantajeALT", 1, false),
                    ),
                    (730, AvatarInfo::new("Automaton_0", "Automaton", 0, false)),
                    (731, AvatarInfo::new("JohnWALT_0", "JohnWALT", 0, false)),
                    (732, AvatarInfo::new("BubblePop_2", "BubblePop", 2, false)),
                    (
                        738,
                        AvatarInfo::new("ItsyBitsyRetake_1", "ItsyBitsyRetake", 1, false),
                    ),
                    (
                        739,
                        AvatarInfo::new("ItsyBitsyRetake_0", "ItsyBitsyRetake", 0, false),
                    ),
                    (
                        741,
                        AvatarInfo::new("WakaWakaKids_0", "WakaWakaKids", 0, false),
                    ),
                    (
                        742,
                        AvatarInfo::new("MagicHalloweenKids_1", "MagicHalloweenKids", 1, false),
                    ),
                    (
                        743,
                        AvatarInfo::new("MagicHalloweenKids_0", "MagicHalloweenKids", 0, false),
                    ),
                    (
                        748,
                        AvatarInfo::new("BubblePopALT_2", "BubblePopALT", 2, false),
                    ),
                    (
                        749,
                        AvatarInfo::new("BubblePopALT_1", "BubblePopALT", 1, false),
                    ),
                    (
                        750,
                        AvatarInfo::new("BubblePopALT_0", "BubblePopALT", 0, false),
                    ),
                    (753, AvatarInfo::new("LoveIsAll_0", "LoveIsAll", 0, false)),
                    (
                        775,
                        AvatarInfo::new("FootlooseKids_0", "FootlooseKids", 0, false),
                    ),
                    (776, AvatarInfo::new("LoveWard_0", "LoveWard", 0, false)),
                    (778, AvatarInfo::new("KeepOn_0", "KeepOn", 0, false)),
                    (
                        780,
                        AvatarInfo::new("RiskyBusiness_0", "RiskyBusiness", 0, false),
                    ),
                    (781, AvatarInfo::new("Dharma_0", "Dharma", 0, false)),
                    (
                        782,
                        AvatarInfo::new("Cottonmouth_0", "Cottonmouth", 0, false),
                    ),
                    (785, AvatarInfo::new("DharmaALT_0", "DharmaALT", 0, false)),
                    (786, AvatarInfo::new("WDFGlitchy_0", "WDFGlitchy", 0, false)),
                    (
                        803,
                        AvatarInfo::new("UbiSoftRabbidsApache", "UbiSoftRabbids", 0, false),
                    ),
                    (
                        804,
                        AvatarInfo::new("UbiSoftRabbidsCotton", "UbiSoftRabbids", 1, false),
                    ),
                    (
                        805,
                        AvatarInfo::new(
                            "UbiSoftRabbidsSexyAndIKnowItDLC",
                            "UbiSoftRabbids",
                            2,
                            false,
                        ),
                    ),
                    (808, AvatarInfo::new("ShapeOfYou_0", "ShapeOfYou", 0, false)),
                    (809, AvatarInfo::new("AnotherOne_2", "AnotherOne", 2, false)),
                    (810, AvatarInfo::new("BadLiar_0", "BadLiar", 0, false)),
                    (
                        811,
                        AvatarInfo::new("KissingStrangers_1", "KissingStrangers", 1, false),
                    ),
                    (
                        812,
                        AvatarInfo::new("MissAmazingKIDS_0", "MissAmazingKIDS", 0, false),
                    ),
                    (813, AvatarInfo::new("NewFace_1", "NewFace", 1, false)),
                    (
                        814,
                        AvatarInfo::new("SlumberParty_1", "SlumberParty", 1, false),
                    ),
                    (815, AvatarInfo::new("BeepBeep_0", "BeepBeep", 0, false)),
                    (
                        816,
                        AvatarInfo::new("FunkyRobotKids_0", "FunkyRobotKids", 0, false),
                    ),
                    (
                        817,
                        AvatarInfo::new("PixieLandKids_0", "PixieLandKids", 0, false),
                    ),
                    (818, AvatarInfo::new("Sidewinder_0", "Sidewinder", 0, false)),
                    (
                        820,
                        AvatarInfo::new("AllYouGotta_0", "AllYouGotta", 0, false),
                    ),
                    (821, AvatarInfo::new("AnotherOne_3", "AnotherOne", 3, false)),
                    (822, AvatarInfo::new("BubblePop_0", "BubblePop", 0, false)),
                    (823, AvatarInfo::new("BubblePop_1", "BubblePop", 1, false)),
                    (824, AvatarInfo::new("Copperhead_0", "Copperhead", 0, false)),
                    (825, AvatarInfo::new("Despacito_3", "Despacito", 3, false)),
                    (
                        826,
                        AvatarInfo::new("DespacitoALT_0", "DespacitoALT", 0, false),
                    ),
                    (827, AvatarInfo::new("HowFar_0", "HowFar", 0, false)),
                    (
                        828,
                        AvatarInfo::new("Instruction_0", "Instruction", 0, false),
                    ),
                    (829, AvatarInfo::new("JohnW_0", "JohnW", 0, false)),
                    (
                        830,
                        AvatarInfo::new("KissingStrangers_0", "KissingStrangers", 0, false),
                    ),
                    (
                        831,
                        AvatarInfo::new("KissingStrangersALT_1", "KissingStrangersALT", 1, false),
                    ),
                    (
                        832,
                        AvatarInfo::new("NaughtyGirl_0", "NaughtyGirl", 0, false),
                    ),
                    (833, AvatarInfo::new("SwishSwish_0", "SwishSwish", 0, false)),
                    (834, AvatarInfo::new("SwishSwish_1", "SwishSwish", 1, false)),
                    (835, AvatarInfo::new("SwishSwish_2", "SwishSwish", 2, false)),
                    (836, AvatarInfo::new("SwishSwish_3", "SwishSwish", 3, false)),
                    (
                        837,
                        AvatarInfo::new("WakaWakaALT_1", "WakaWakaALT", 1, false),
                    ),
                ]),
            ),
            (
                Game::JustDance2017,
                HashMap::from([
                    (1, AvatarInfo::new("Dare_0", "Dare", 0, false)),
                    (2, AvatarInfo::new("DogsOut_0", "DogsOut", 0, false)),
                    (
                        3,
                        AvatarInfo::new("EyeOfTheTiger_0", "EyeOfTheTiger", 0, false),
                    ),
                    (4, AvatarInfo::new("GetAround_0", "GetAround", 0, false)),
                    (5, AvatarInfo::new("HotNCold_0", "HotNCold", 0, false)),
                    (
                        6,
                        AvatarInfo::new("ILikeToMoveIt_0", "ILikeToMoveIt", 0, false),
                    ),
                    (7, AvatarInfo::new("JinGoLoBa_0", "JinGoLoBa", 0, false)),
                    (8, AvatarInfo::new("RingMyBell_0", "RingMyBell", 0, false)),
                    (
                        9,
                        AvatarInfo::new("NineAfternoon_0", "NineAfternoon", 0, false),
                    ),
                    (10, AvatarInfo::new("BabyGirl_0", "BabyGirl", 0, false)),
                    (11, AvatarInfo::new("CallMe_0", "CallMe", 0, false)),
                    (
                        12,
                        AvatarInfo::new("ChickenPayback_0", "ChickenPayback", 0, false),
                    ),
                    (13, AvatarInfo::new("ComeOn_1", "ComeOn", 1, false)),
                    (14, AvatarInfo::new("CosmicGirl_0", "CosmicGirl", 0, false)),
                    (
                        15,
                        AvatarInfo::new("ElectroTribalDLC_0", "ElectroTribalDLC", 0, false),
                    ),
                    (16, AvatarInfo::new("Firework_0", "Firework", 0, false)),
                    (
                        23,
                        AvatarInfo::new("KattiKalandal_0", "KattiKalandal", 0, false),
                    ),
                    (33, AvatarInfo::new("Song2_0", "Song2", 0, false)), // C1
                    (
                        51,
                        AvatarInfo::new("HalloweenQUAT_2", "HalloweenQUAT", 2, false),
                    ),
                    (
                        52,
                        AvatarInfo::new("HalloweenQUAT_3", "HalloweenQUAT", 3, false),
                    ),
                    (67, AvatarInfo::new("BewareOf_0", "BewareOf", 0, false)),
                    (
                        72,
                        AvatarInfo::new("CrucifiedQUAT_3", "CrucifiedQUAT", 3, false),
                    ),
                    (
                        73,
                        AvatarInfo::new("CrucifiedQUAT_0", "CrucifiedQUAT", 0, false),
                    ),
                    (
                        76,
                        AvatarInfo::new("EverybodyNeeds_0", "EverybodyNeeds", 0, false),
                    ),
                    (
                        77,
                        AvatarInfo::new("FunHouseDLC_0", "FunHouseDLC", 0, false),
                    ),
                    (
                        78,
                        AvatarInfo::new("GangnamStyleDLC_1_C1_V1", "GangnamStyleDLC", 1, false),
                    ),
                    (
                        80,
                        AvatarInfo::new("IstanbulQUAT_0", "IstanbulQUAT", 0, false),
                    ),
                    (
                        90,
                        AvatarInfo::new("RockLobster_0", "RockLobster", 0, false),
                    ),
                    (
                        97,
                        AvatarInfo::new("TimeWarpQUAT_1", "TimeWarpQUAT", 1, false),
                    ),
                    (
                        99,
                        AvatarInfo::new("WildWildWestQUAT_2", "WildWildWestQUAT", 2, false),
                    ),
                    (
                        106,
                        AvatarInfo::new("IWillSurvive_0", "IWillSurvive", 0, false),
                    ),
                    (115, AvatarInfo::new("RobotRock_1", "RobotRock", 1, false)),
                    (118, AvatarInfo::new("Aquarius_1", "Aquarius", 1, false)),
                    (127, AvatarInfo::new("Gigolo_1", "Gigolo", 1, false)),
                    (131, AvatarInfo::new("Limbo_1", "Limbo", 1, false)),
                    (
                        175,
                        AvatarInfo::new("UbiSoftFarCry", "UbiSoftFarCry", 0, false),
                    ),
                    (
                        179,
                        AvatarInfo::new("UbiSoftZombiU", "UbiSoftZombiU", 0, false),
                    ),
                    (
                        180,
                        AvatarInfo::new("UbiSoftRayman", "UbiSoftRayman", 0, false),
                    ),
                    (
                        239,
                        AvatarInfo::new("BollywoodXmas_0", "BollywoodXmas", 0, false),
                    ),
                    (252, AvatarInfo::new("Animals_0", "Animals", 0, false)),
                    (253, AvatarInfo::new("LetsGroove_0", "LetsGroove", 0, false)),
                    (
                        254,
                        AvatarInfo::new("WilliamTell_0", "WilliamTell", 0, false),
                    ),
                    (260, AvatarInfo::new("MarioNX", "MarioNX", 0, false)),
                    (274, AvatarInfo::new("Chiwawa_0", "Chiwawa", 0, false)),
                    (
                        309,
                        AvatarInfo::new("UbiSoftACUnityArno", "UbiSoftACUnityArno", 0, false),
                    ),
                    (
                        314,
                        AvatarInfo::new(
                            "UbiSoftChildOfLightAurora",
                            "UbiSoftChildOfLightAurora",
                            0,
                            false,
                        ),
                    ),
                    (
                        345,
                        AvatarInfo::new("GangnamStyleDLC_1_C2", "GangnamStyleDLC", 1, false),
                    ),
                    (
                        346,
                        AvatarInfo::new("GangnamStyleDLC_1_C1_V2", "GangnamStyleDLC", 1, false),
                    ),
                    (
                        347,
                        AvatarInfo::new("GangnamStyleDLC_0", "GangnamStyleDLC", 0, false),
                    ),
                    (
                        353,
                        AvatarInfo::new("DieYoungDLC_1", "DieYoungDLC", 1, false),
                    ),
                    (357, AvatarInfo::new("TimberDLC_1", "TimberDLC", 1, false)),
                    (
                        360,
                        AvatarInfo::new("MovesLikeDLC_0_C1", "MovesLikeDLC", 0, false),
                    ),
                    (
                        362,
                        AvatarInfo::new("OneThingDLC_1", "OneThingDLC", 1, false),
                    ),
                    (
                        364,
                        AvatarInfo::new("KiloPapaDLC_0_V2", "KiloPapaDLC", 0, false),
                    ),
                    (
                        368,
                        AvatarInfo::new("BlurredLines_1", "BlurredLines", 1, false),
                    ),
                    (
                        369,
                        AvatarInfo::new("Gentleman_0_C2", "Gentleman", 0, false),
                    ),
                    (370, AvatarInfo::new("ThatPower_3", "ThatPower", 3, false)),
                    (371, AvatarInfo::new("KissYou_2", "KissYou", 2, false)),
                    (372, AvatarInfo::new("CmonDLC_0", "CmonDLC", 0, false)),
                    (
                        373,
                        AvatarInfo::new(
                            "WhatMakesYouBeautiful_0",
                            "WhatMakesYouBeautiful",
                            0,
                            false,
                        ),
                    ),
                    (
                        374,
                        AvatarInfo::new("MovesLikeDLC_0_C4", "MovesLikeDLC", 0, false),
                    ),
                    (
                        375,
                        AvatarInfo::new("DynamiteQUAT_0", "DynamiteQUAT", 0, false),
                    ),
                    (376, AvatarInfo::new("DaFunk_1", "DaFunk", 1, false)),
                    (378, AvatarInfo::new("Pilgrim", "Pilgrim", 0, false)),
                    (379, AvatarInfo::new("SantaClaus", "SantaClaus", 0, false)),
                    (380, AvatarInfo::new("GetLucky_0", "GetLucky", 0, false)),
                    (381, AvatarInfo::new("MahNa_1", "MahNa", 1, false)),
                    (
                        382,
                        AvatarInfo::new("GangnamStyleDLC_Horse", "GangnamStyleDLC", 4, false),
                    ),
                    (
                        383,
                        AvatarInfo::new("AThousandDances_Penguin", "AThousandDances", 4, false),
                    ),
                    (
                        385,
                        AvatarInfo::new("Lollipop_SockPuppet", "Lollipop", 4, false),
                    ),
                    (478, AvatarInfo::new("LeanOn_0", "LeanOn", 0, false)),
                    (479, AvatarInfo::new("LeanOn_1", "LeanOn", 1, false)),
                    (
                        480,
                        AvatarInfo::new("ScreamNShout_3", "ScreamNShout", 3, false),
                    ),
                    (
                        481,
                        AvatarInfo::new("ScreamNShoutALT_0", "ScreamNShoutALT", 0, false),
                    ),
                    (498, AvatarInfo::new("Hips_0", "Hips", 0, false)),
                    (499, AvatarInfo::new("WhatIsLove_0", "WhatIsLove", 0, false)),
                    (500, AvatarInfo::new("WorthItALT_2", "WorthItALT", 2, false)),
                    (504, AvatarInfo::new("DontStopMe_0", "DontStopMe", 0, false)), // C1
                    (505, AvatarInfo::new("Daddy_1", "Daddy", 1, false)),
                    (506, AvatarInfo::new("TicoTico_1", "TicoTico", 1, false)),
                    (507, AvatarInfo::new("Daddy_0", "Daddy", 0, false)),
                    (516, AvatarInfo::new("Samba_0", "Samba", 0, false)),
                    (517, AvatarInfo::new("Radical_0", "Radical", 0, false)),
                    (518, AvatarInfo::new("PoPiPo_2", "PoPiPo", 2, false)),
                    (519, AvatarInfo::new("LeanOnALT_0", "LeanOnALT", 0, false)),
                    (520, AvatarInfo::new("September_1", "September", 1, false)),
                    (522, AvatarInfo::new("ElTiki_1", "ElTiki", 1, false)),
                    (
                        524,
                        AvatarInfo::new("GhostInTheKeys_3", "GhostInTheKeys", 3, false),
                    ),
                    (
                        525,
                        AvatarInfo::new("WhatIsLoveALT_1", "WhatIsLoveALT", 1, false),
                    ),
                    (
                        526,
                        AvatarInfo::new("DontStopMeALT_0", "DontStopMeALT", 0, false),
                    ),
                    (527, AvatarInfo::new("WorthIt_0", "WorthIt", 0, false)),
                    (528, AvatarInfo::new("Titanium_0", "Titanium", 0, false)),
                    (529, AvatarInfo::new("Sorry_0", "Sorry", 0, false)),
                    (
                        539,
                        AvatarInfo::new("SeptemberALT_0", "SeptemberALT", 0, false),
                    ),
                    (541, AvatarInfo::new("DaddyALT_1", "DaddyALT", 1, false)),
                    (
                        542,
                        AvatarInfo::new("CheapThrills_0", "CheapThrills", 0, false),
                    ),
                    (
                        543,
                        AvatarInfo::new("RunTheNight_0", "RunTheNight", 0, false),
                    ),
                    (544, AvatarInfo::new("Oishii_0", "Oishii", 0, false)),
                    (
                        545,
                        AvatarInfo::new("CantFeelMyFace_0", "CantFeelMyFace", 0, false),
                    ),
                    (
                        546,
                        AvatarInfo::new("DragosteaDinTei_1", "DragosteaDinTei", 1, false),
                    ),
                    (547, AvatarInfo::new("Groove_1", "Groove", 1, false)),
                    (
                        550,
                        AvatarInfo::new("UbiSoftACCChinaShaoJun", "UbiSoftACCChina", 0, false),
                    ),
                    (
                        551,
                        AvatarInfo::new("UbiSoftACCRussiaNikolai", "UbiSoftACCRussia", 0, false),
                    ),
                    (
                        552,
                        AvatarInfo::new("UbiSoftACCIndiaArbaaz", "UbiSoftACCIndia", 0, false),
                    ),
                    (
                        554,
                        AvatarInfo::new("UbiSoftRabbidsDisturbia", "UbiSoftRabbids", 0, false),
                    ),
                    (
                        555,
                        AvatarInfo::new("UbiSoftRainbow6SiegeIQ", "UbiSoftRainbow6Siege", 0, false),
                    ),
                    (
                        557,
                        AvatarInfo::new("UbiSoftWatchDogs2Marcus", "UbiSoftWatchDogs2", 0, false),
                    ),
                    (559, AvatarInfo::new("ColaSong_0", "ColaSong", 0, false)),
                    (560, AvatarInfo::new("DQ_Mask", "DQ", 0, false)),
                    (561, AvatarInfo::new("DQ_CoolKids", "DQ", 0, false)),
                    (562, AvatarInfo::new("DQ_AroundTheWorld", "DQ", 0, false)),
                    (563, AvatarInfo::new("DQ_NewHeroines", "DQ", 0, false)),
                    (564, AvatarInfo::new("DQ_Family", "DQ", 0, false)),
                    (565, AvatarInfo::new("DQ_HighEnergy", "DQ", 0, false)),
                    (566, AvatarInfo::new("DQ_YearRound", "DQ", 0, false)),
                    (567, AvatarInfo::new("DQ_KeepCalm", "DQ", 0, false)),
                    (568, AvatarInfo::new("DQ_StrongWoman", "DQ", 0, false)),
                    (
                        569,
                        AvatarInfo::new("ColaSongALT_0", "ColaSongALT", 0, false),
                    ),
                    (
                        570,
                        AvatarInfo::new("CakeByTheOceanALT_1", "CakeByTheOceanALT", 1, false),
                    ),
                    (571, AvatarInfo::new("ElTikiALT_1", "ElTikiALT", 1, false)),
                    (572, AvatarInfo::new("IntoYou_0", "IntoYou", 0, false)),
                    (
                        573,
                        AvatarInfo::new("CheapThrillsALT_0", "CheapThrillsALT", 0, false),
                    ),
                    (574, AvatarInfo::new("Bailar_0", "Bailar", 0, false)),
                    (
                        575,
                        AvatarInfo::new("WhereverIGo_0", "WhereverIGo", 0, false),
                    ),
                    (576, AvatarInfo::new("NaeNaeALT_1", "NaeNaeALT", 1, false)),
                    (578, AvatarInfo::new("Bonbon_0", "Bonbon", 0, false)),
                    (580, AvatarInfo::new("Leila_0", "Leila", 0, false)),
                    (581, AvatarInfo::new("ILoveRock_0", "ILoveRock", 0, false)),
                    (
                        582,
                        AvatarInfo::new("CakeByTheOcean_0", "CakeByTheOcean", 0, false),
                    ),
                    (586, AvatarInfo::new("DQ_Snack", "DQ", 0, false)),
                    (587, AvatarInfo::new("SorryALT_0", "SorryALT", 0, false)),
                    (588, AvatarInfo::new("TeDominar_1", "TeDominar", 1, false)),
                    (
                        589,
                        AvatarInfo::new("LastChristmas_0", "LastChristmas", 0, false),
                    ),
                    (590, AvatarInfo::new("AllAboutUs_2", "AllAboutUs", 2, false)),
                    (591, AvatarInfo::new("Bang_0", "Bang", 0, false)),
                    (592, AvatarInfo::new("Bicicleta_0", "Bicicleta", 0, false)),
                    (
                        593,
                        AvatarInfo::new("RedMangoose_0", "RedMangoose", 0, false),
                    ),
                    (
                        594,
                        AvatarInfo::new("SingleLadies_0", "SingleLadies", 0, false),
                    ),
                    (595, AvatarInfo::new("NaeNae_1", "NaeNae", 1, false)),
                    (596, AvatarInfo::new("Oishii_1", "Oishii", 1, false)),
                    (598, AvatarInfo::new("HipsALT_1", "HipsALT", 1, false)),
                    (599, AvatarInfo::new("RadicalALT_1", "RadicalALT", 1, false)),
                    (
                        600,
                        AvatarInfo::new("LittleSwing_1", "LittleSwing", 1, false),
                    ),
                    (606, AvatarInfo::new("LegSongCHN_0", "LegSongCHN", 0, false)),
                    (
                        607,
                        AvatarInfo::new("KaraokeForeverCHN_0", "KaraokeForeverCHN", 0, false),
                    ),
                    (
                        608,
                        AvatarInfo::new("BigDreamerCHN_0", "BigDreamerCHN", 0, false),
                    ),
                    (
                        609,
                        AvatarInfo::new("BedtimeStoryCHN_0", "BedtimeStoryCHN", 0, false),
                    ),
                    (639, AvatarInfo::new("JDCBadGirl_0", "JDCBadGirl", 0, false)),
                    (
                        640,
                        AvatarInfo::new("JDCBangBangBang_1", "JDCBangBangBang", 1, false),
                    ),
                    (641, AvatarInfo::new("JDCDeep_0", "JDCDeep", 0, false)),
                    (642, AvatarInfo::new("JDCGrowl_0", "JDCGrowl", 0, false)),
                    (646, AvatarInfo::new("HowDeep_0", "HowDeep", 0, false)),
                ]),
            ),
        ])
    })
}

#[derive(Debug, Clone, Copy)]
/// Documents avatar information which might be missing in the game
pub struct AvatarInfo {
    /// Unique name for the avatar
    pub name: &'static str,
    /// The map this avatar belongs to
    pub map: &'static str,
    /// Which coach this avatar is based on
    pub coach: u8,
    /// Is this a golden avatar
    pub special_effect: bool,
}

impl AvatarInfo {
    /// Create a `AvatarInfo`
    pub const fn new(
        name: &'static str,
        map: &'static str,
        coach: u8,
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
