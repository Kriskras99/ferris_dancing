//! # Avatars
//! Import all the avatars.
//!
//! Current implementation is a bit wonky. A better option would be too manually match all avatar ids
//! to names per game. Then Just Dance 2017 avatars can also be imported.
use std::{
    borrow::Cow, collections::HashMap, ffi::OsStr, fs::File, io::Write, path::Path, sync::OnceLock,
};

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::testing::test;
use ubiart_toolkit::{cooked, json_types::AvatarsObjectives, utils::Game};

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

fn save_avatar_config(is: &ImportState<'_>, avatars: HashMap<String, Avatar>) -> Result<(), Error> {
    let avatars_config_path = is.dirs.avatars().join("avatars.json");
    let file = File::create(avatars_config_path)?;
    serde_json::to_writer_pretty(file, &avatars)?;
    Ok(())
}

fn save_images(
    is: &ImportState<'_>,
    name: &str,
    avatar: &Avatar,
    actor_path: &str,
    phone_image: &str,
) -> Result<(), Error> {
    let avatar_named_dir_path = is.dirs.avatars().join(name);
    std::fs::create_dir(&avatar_named_dir_path)?;
    let alt_actor_file = is
        .vfs
        .open(cook_path(actor_path.as_ref(), is.platform)?.as_ref())?;
    let alt_actor = cooked::act::parse(&alt_actor_file, &mut 0, is.unique_game_id)?;

    let image_actor = alt_actor
        .components
        .first()
        .ok_or_else(|| anyhow!("No templates in {}", actor_path))?;
    let mtg = image_actor.material_graphic_component()?;

    // Save decooked image
    let image_path = mtg.files[0].to_string();
    test(&image_path.is_empty(), &false)?;
    let cooked_image_path = cook_path(&image_path, is.platform)?;
    let decooked_image = decode_texture(&is.vfs.open(cooked_image_path.as_ref())?)?;
    let avatar_image_path = is.dirs.avatars().join(avatar.image_path.as_ref());
    decooked_image.save(&avatar_image_path)?;

    // Save phone image
    let avatar_image_phone_path = is.dirs.avatars().join(avatar.image_phone_path.as_ref());
    let mut avatar_image_phone_file = File::create(&avatar_image_phone_path)?;
    avatar_image_phone_file.write_all(&is.vfs.open(phone_image.as_ref())?)?;

    Ok(())
}

/// Import all the avatars (Just Dance 2020-2022)
pub fn import_v20v22(
    is: &ImportState<'_>,
    avatardb_scene: &str,
    avatarsobjectives: Option<&AvatarsObjectives>,
) -> Result<(), Error> {
    let empty_objectives = HashMap::new();
    println!("Importing avatars...");

    let mut avatars = load_avatar_config(is)?;

    // Open the avatardb and avatarsobjectives (which might be empty)
    let avatardb_file = is
        .vfs
        .open(cook_path(avatardb_scene, is.platform)?.as_ref())?;
    let avatardb = cooked::isc::parse(&avatardb_file)?;
    let avatarsobjectives = avatarsobjectives.unwrap_or(&empty_objectives);

    // maps the id of an avatar to an avatar name, so related avatars can be linked by name
    let mut id_map = HashMap::with_capacity(avatardb.scene.actors.len());
    // stores the avatar descriptions so we don't have to iter through the avatardb again
    let mut avatar_descriptions = Vec::with_capacity(avatardb.scene.actors.len());

    for actor in avatardb.scene.actors {
        let actor = actor.actor()?;

        // Extract avatar description from template
        let file = is
            .vfs
            .open(cook_path(actor.lua.as_ref(), is.platform)?.as_ref())?;
        let template = cooked::json::parse_v22(&file, is.lax)?;
        let mut actor_template = template.actor()?;
        test(&actor_template.components.len(), &2)
            .context("Not exactly two components in actor")?;
        let avatar_desc = actor_template.components.remove(1).avatar_description()?;

        let name = match get_name(is.game, avatar_desc.avatar_id) {
            Ok(name) => name,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };

        // Add the name to the id map, so we can look it up later
        id_map.insert(avatar_desc.avatar_id, name);

        // Collect the avatar descriptions so we don't have the parse isc and tpls again
        avatar_descriptions.push(avatar_desc.into_owned());
    }

    for avatar_desc in avatar_descriptions {
        let name = id_map[&avatar_desc.avatar_id];
        let main_avatar = if avatar_desc.main_avatar_id == u16::MAX {
            None
        } else {
            let main = id_map
                .get(&avatar_desc.main_avatar_id)
                .map(|s| Cow::Borrowed(*s));
            if main.is_none() {
                println!(
                    "Warning! Avatar id {} does not exist!",
                    avatar_desc.main_avatar_id
                );
                println!("Failed to add main avatar for {name}");
            }
            main
        };
        // Only add new avatars
        if !avatars.contains_key(name) {
            let avatar_image_path = format!("{name}/avatar.png");
            let avatar_image_phone_path = format!("{name}/avatar_phone.png");
            let avatar = Avatar {
                relative_song_name: if avatar_desc.relative_song_name.is_empty() {
                    None
                } else {
                    Some(avatar_desc.relative_song_name)
                },
                sound_family: avatar_desc.sound_family,
                status: avatar_desc.status,
                unlock_type: UnlockType::from_unlock_type(
                    avatar_desc.unlock_type,
                    avatarsobjectives.get(&avatar_desc.avatar_id),
                )?,
                used_as_coach_map_name: avatar_desc.used_as_coach_map_name,
                used_as_coach_coach_id: avatar_desc.used_as_coach_coach_id,
                special_effect: avatar_desc.special_effect == 1,
                main_avatar,
                image_path: avatar_image_path.into(),
                image_phone_path: avatar_image_phone_path.into(),
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

    save_avatar_config(is, avatars)?;

    Ok(())
}

/// Import all the avatars (Just Dance 2017-2019)
pub fn import_v17v19(
    is: &ImportState<'_>,
    avatardb_scene: &str,
    avatarsobjectives: Option<&AvatarsObjectives>,
) -> Result<(), Error> {
    let empty_objectives = HashMap::new();
    println!("Importing avatars...");

    let mut avatars = load_avatar_config(is)?;

    // Open the avatardb and avatarsobjectives (which might be empty)
    let avatardb_file = is
        .vfs
        .open(cook_path(avatardb_scene, is.platform)?.as_ref())?;
    let avatardb = cooked::isc::parse(&avatardb_file)?;
    let avatarsobjectives = avatarsobjectives.unwrap_or(&empty_objectives);

    // maps the id of an avatar to an avatar name, so related avatars can be linked by name
    let mut id_map = HashMap::with_capacity(avatardb.scene.actors.len());
    // stores the avatar descriptions so we don't have to iter through the avatardb again
    let mut avatar_descriptions = Vec::with_capacity(avatardb.scene.actors.len());

    for actor in avatardb.scene.actors {
        let actor = actor.actor()?;

        // Extract avatar description from template
        let file = is
            .vfs
            .open(cook_path(actor.lua.as_ref(), is.platform)?.as_ref())?;
        let template = cooked::json::parse_v19(&file, is.lax)?;
        let mut actor_template = template.actor()?;
        test(&actor_template.components.len(), &2)
            .context("Not exactly two components in actor")?;
        let avatar_desc = actor_template.components.remove(1).avatar_description()?;

        let name = match get_name(is.game, avatar_desc.avatar_id) {
            Ok(name) => name,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };

        // Add the name to the id map, so we can look it up later
        id_map.insert(avatar_desc.avatar_id, name);

        // Collect the avatar descriptions so we don't have the parse isc and tpls again
        avatar_descriptions.push(avatar_desc.into_owned());
    }

    for avatar_desc in avatar_descriptions {
        let name = id_map[&avatar_desc.avatar_id];
        let mut split = name.split('_');
        let map_name = Cow::Borrowed(split.next().unwrap());
        let coach_id = Cow::Borrowed(split.next().unwrap());
        let gold = split.next() == Some("Gold");

        let main_avatar = avatar_desc
            .main_avatar_id
            .and_then(|main_avatar_id| {
                if main_avatar_id == u16::MAX {
                    None
                } else {
                    let main = id_map.get(&main_avatar_id).map(|s| Cow::Borrowed(*s));
                    if main.is_none() {
                        println!("Warning! Avatar id {} does not exist!", main_avatar_id);
                        println!("Failed to add main avatar for {name}");
                    }
                    main
                }
            })
            .or_else(|| {
                if gold {
                    Some(Cow::Owned(format!("{map_name}_{coach_id}")))
                } else {
                    None
                }
            });

        // Only add new avatars
        if !avatars.contains_key(name) {
            let avatar_image_path = format!("{name}/avatar.png");
            let avatar_image_phone_path = format!("{name}/avatar_phone.png");
            let avatar = Avatar {
                relative_song_name: if avatar_desc.relative_song_name.is_empty() {
                    None
                } else {
                    Some(avatar_desc.relative_song_name)
                },
                sound_family: avatar_desc.sound_family,
                status: avatar_desc.status,
                unlock_type: UnlockType::from_unlock_type(
                    avatar_desc.unlock_type,
                    avatarsobjectives.get(&avatar_desc.avatar_id),
                )?,
                used_as_coach_map_name: avatar_desc.used_as_coach_map_name.unwrap_or(map_name),
                used_as_coach_coach_id: avatar_desc
                    .used_as_coach_coach_id
                    .unwrap_or(coach_id.parse::<u8>()?),
                special_effect: avatar_desc.special_effect.map(|s| s == 1).unwrap_or(gold),
                main_avatar,
                image_path: avatar_image_path.into(),
                image_phone_path: avatar_image_phone_path.into(),
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

    save_avatar_config(is, avatars)?;

    Ok(())
}

fn import_unreferenced_avatars(
    is: &ImportState<'_>,
    avatars: &mut HashMap<String, Avatar>,
) -> Result<(), Error> {
    let import_path = cook_path("world/avatars/", is.platform)?;
    for avatar_id in is
        .vfs
        .walk_filesystem(import_path.as_ref())?
        .filter(|p| p.ends_with("avatar.png.ckd"))
        .map(Path::parent)
        .flatten()
        .map(Path::file_name)
        .flatten()
        .map(OsStr::to_str)
        .flatten()
        .map(str::parse::<u16>)
        .flatten()
    {
        let name = match get_name(is.game, avatar_id) {
            Ok(name) => name,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };
        if !avatars.contains_key(name) {
            let avatar_named_dir_path = is.dirs.avatars().join(name);
            let avatar_image_path = format!("{name}/avatar.png");
            let avatar_image_phone_path = format!("{name}/avatar_phone.png");
            let mut split = name.split('_');
            let map_name = Cow::Borrowed(split.next().unwrap());
            let coach_id = Cow::Borrowed(split.next().unwrap_or("0"));
            let gold = split.next() == Some("Gold");
            let avatar = Avatar {
                relative_song_name: None,
                sound_family: Cow::Borrowed("AVTR_Common_Brand"),
                status: 1,
                unlock_type: UnlockType::Unlocked,
                used_as_coach_map_name: map_name.clone(),
                used_as_coach_coach_id: coach_id.as_ref().parse()?,
                special_effect: gold,
                main_avatar: if gold {
                    Some(Cow::Owned(format!("{map_name}_{coach_id}")))
                } else {
                    None
                },
                image_path: avatar_image_path.into(),
                image_phone_path: avatar_image_phone_path.into(),
            };
            std::fs::create_dir(&avatar_named_dir_path)?;

            // Save decooked image
            let cooked_image_path = cook_path(
                &format!("world/avatars/{avatar_id:0>4}/avatar.png"),
                is.platform,
            )?;
            let decooked_image = decode_texture(&is.vfs.open(cooked_image_path.as_ref())?)?;
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
static GAME_AVATAR_ID_NAME_MAP: OnceLock<HashMap<Game, HashMap<u16, &'static str>>> =
    OnceLock::new();

/// Get the name for the `avatar_id` for `game`
fn get_name(game: Game, avatar_id: u16) -> Result<&'static str, String> {
    get_map()
        .get(&game)
        .ok_or_else(|| format!("Unsupported game: {game}"))?
        .get(&avatar_id)
        .copied()
        .ok_or_else(|| format!("Unknown ID: {avatar_id}"))
}

/// Get the static map which maps avatar ids to their proper names for each game
fn get_map() -> &'static HashMap<Game, HashMap<u16, &'static str>> {
    GAME_AVATAR_ID_NAME_MAP.get_or_init(|| {
        HashMap::from([
            (
                Game::JustDance2022,
                HashMap::from([
                    (1, "Dare_0"),
                    (2, "DogsOut_0"),
                    (3, "EyeOfTheTiger_0"),
                    (4, "GetAround_0"),
                    (5, "HotNCold_0"),
                    (6, "ILikeToMoveIt_0"),
                    (7, "JinGoLoBa_0"),
                    (8, "RingMyBell_0"),
                    (9, "NineAfternoon_0"),
                    (10, "BabyGirl_0"),
                    (11, "CallMe_0"),
                    (12, "ChickenPayback_0"),
                    (13, "ComeOn_1"),
                    (14, "CosmicGirl_0"),
                    (15, "ElectroTribalDLC_0"),
                    (16, "Firework_0"),
                    (76, "EverybodyNeeds_0"),
                    // (347, "?""),
                    (717, "FearlessPirateKids_0"),
                    (816, "FunkyRobotKids_0"),
                    (907, "MonstersAcademyKids_0"),
                    (1062, "MedievalKids_0"),
                    (1063, "MedievalKids_1"),
                    (1064, "ChefKids_0"),
                    (1065, "AdventurerKids_0"),
                    (1397, "FiremenKids_0"),
                    (1398, "FiremenKids_1"),
                    (1402, "BalletKids_0"),
                    (1403, "BalletKids_1"),
                    (1467, "WhoRun_0"),
                    (1468, "WhoRun_1"),
                    (1469, "WhoRun_2"),
                    (1470, "WhoRun_1_Gold"),
                    (1517, "WhoRunALTRETAKE_0"),
                    (1518, "WhoRunALT_0_Gold"),
                    (1619, "FreedfromDesire_0"),
                    (1620, "FreedfromDesire_0_Gold"),
                    (1633, "China_0"),
                    (1634, "China_1"),
                    (1635, "China_2"),
                    (1636, "China_2_Gold"),
                    (1638, "BreakMyHeart_0"),
                    (1639, "BreakMyHeart_0_Gold"),
                    (1641, "FlashPose_0"),
                    (1642, "FlashPose_1"),
                    (1643, "FlashPose_2"),
                    (1644, "FlashPose_0_Gold"),
                    (1645, "Siargo_0"),
                    (1646, "Siargo_0_Gold"),
                    (1647, "Human_0"),
                    (1648, "Human_0_Gold"),
                    (1649, "YouCanDance_0"),
                    (1650, "YouCanDance_0_Gold"),
                    (1651, "Funk_0"),
                    (1652, "Funk_1"),
                    (1653, "Funk_0_Gold"),
                    (1654, "Believer_0"),
                    (1655, "Believer_1"),
                    (1656, "Believer_0_Gold"),
                    (1657, "TGIF_0"),
                    (1658, "TGIF_0_Gold"),
                    (1659, "Jopping_0"),
                    (1660, "Jopping_1"),
                    (1661, "Jopping_2"),
                    (1662, "Jopping_2_Gold"),
                    (1668, "Levelup_0"),
                    (1669, "Levelup_1"),
                    (1670, "Levelup_2"),
                    (1671, "Levelup_1_Gold"),
                    (1672, "BossWitch_0"),
                    (1673, "BossWitch_0_Gold"),
                    (1674, "FollowTheWhiteRabbit_0"),
                    (1675, "FollowTheWhiteRabbit_0_Gold"),
                    (1676, "Popstars_0"),
                    (1677, "Popstars_1"),
                    (1678, "Popstars_2"),
                    (1679, "Popstars_3"),
                    (1680, "Popstars_3_Gold"),
                    (1681, "Mood_0"),
                    (1682, "Mood_1"),
                    (1683, "Mood_1_Gold"),
                    (1684, "JoppingAlt_0"),
                    (1685, "JoppingAlt_0_Gold"),
                    (1686, "BlackMam_0"),
                    (1687, "BlackMam_1"),
                    (1688, "BlackMam_2"),
                    (1689, "BlackMam_1_Gold"),
                    (1690, "BlackMamALT_0"),
                    (1691, "BlackMamALT_0_Gold"),
                    (1692, "ImOuttaLove_0"),
                    (1693, "ImOuttaLove_0_Gold"),
                    (1694, "Boombayah_0"),
                    (1695, "Boombayah_1"),
                    (1696, "Boombayah_2"),
                    (1697, "Boombayah_3"),
                    (1698, "Boombayah_2_Gold"),
                    (1699, "Baiana_0"),
                    (1700, "Baiana_0_Gold"),
                    (1702, "Malibu_0"),
                    (1703, "Malibu_1"),
                    (1704, "Malibu_2"),
                    (1705, "RockYourBody_0"),
                    (1706, "RockYourBody_0_Gold"),
                    (1707, "Jerusalema_0"),
                    (1708, "Jerusalema_1"),
                    (1709, "Jerusalema_2"),
                    (1710, "Jerusalema_3"),
                    (1711, "Jerusalema_3_Gold"),
                    (1712, "Judas_0"),
                    (1713, "Judas_0_Gold"),
                    (1714, "Buttons_0"),
                    (1716, "Buttons_2"),
                    (1718, "ChinaALT_0"),
                    (1719, "ChinaALT_1"),
                    (1720, "ChinaALT_1_Gold"),
                    (1721, "LoveStory_0"),
                    (1722, "LoveStory_1"),
                    (1723, "LoveStory_0_Gold"),
                    (1724, "GirlLikeMe_0"),
                    (1725, "GirlLikeMe_1"),
                    (1726, "GirlLikeMe_2"),
                    (1727, "GirlLikeMe_3"),
                    (1728, "GirlLikeMe_3_Gold"),
                    (1729, "Chandelier_0"),
                    (1730, "Chandelier_0_Gold"),
                    (1731, "SuaCara_0"),
                    (1732, "SuaCara_0_Gold"),
                    (1733, "BoombayahALT_0"),
                    (1734, "BoombayahALT_0_Gold"),
                    (1737, "GirlLikeMeALT_0"),
                    (1738, "GirlLikeMeALT_0_Gold"),
                    (1739, "StopDropAndRoll_0"),
                    (1740, "StopDropAndRoll_1"),
                    (1741, "StopDropAndRoll_2"),
                    (1742, "StopDropAndRoll_3"),
                    (1743, "StopDropAndRoll_2_Gold"),
                    (1744, "MightyReal_0"),
                    (1745, "MightyReal_0_Gold"),
                    (1746, "MrBlueSky_0"),
                    (1747, "MrBlueSky_0_Gold"),
                    (1748, "ThinkAboutThings_0"),
                    (1749, "ThinkAboutThings_0_Gold"),
                    (1750, "Chacarron_0"),
                    (1751, "Chacarron_0_Gold"),
                    (1752, "SuaCaraALT_0"),
                    (1753, "SuaCaraALT_1"),
                    (1754, "SuaCaraALT_0_Gold"),
                    (1755, "Boombayah_3_Gold_Unlimited"),
                    (1756, "Believer_0_Gold_Unlimited"),
                    (1757, "ChinaALT_0_Gold"),
                    (1758, "SmalltownBoy_0"),
                    (1759, "SmalltownBoy_0_Gold"),
                    (1760, "BlackMam_2_Gold"),
                    (1761, "SaveYourTears_0"),
                    (1762, "SaveYourTears_1"),
                    (1763, "SaveYourTears_0_Gold"),
                    (1764, "Levitating_0"),
                    (1765, "Levitating_0_Gold"),
                    (1766, "NailsHips_0"),
                    (1767, "NailsHips_0_Gold"),
                    (1768, "NailsHipsJD_0"),
                    (1769, "NailsHipsJD_0_Gold"),
                    (1770, "HappierThanEver_0"),
                    (1771, "HappierThanEver_0_Gold"),
                    (1772, "BuildAB_0"),
                    (1773, "BuildAB_0_Gold"),
                    (1774, "ChandelierALT_0"),
                    (1775, "ChandelierALT_0_Gold"),
                    (1776, "LevitatingALT_0"),
                    (1777, "LevitatingALT_0_Gold"),
                ]),
            ),
            (
                Game::JustDance2021,
                HashMap::from([
                    (1, "Dare_0"),
                    (2, "DogsOut_0"),
                    (3, "EyeOfTheTiger_0"),
                    (4, "GetAround_0"),
                    (5, "HotNCold_0"),
                    (6, "ILikeToMoveIt_0"),
                    (7, "JinGoLoBa_0"),
                    (8, "RingMyBell_0"),
                    (9, "NineAfternoon_0"),
                    (10, "BabyGirl_0"),
                    (11, "CallMe_0"),
                    (12, "ChickenPayback_0"),
                    (13, "ComeOn_1"),
                    (14, "CosmicGirl_0"),
                    (15, "ElectroTribalDLC_0"),
                    (16, "Firework_0"),
                    (76, "EverybodyNeeds_0"),
                    (347, "GangnamStyleDLC_0"),
                    // (922, "?"),
                    // (933, "?"),
                    (953, "OMG_2"),
                    (1393, "BadAssPrincessKids_0"),
                    (1394, "SpyKids_0"),
                    (1395, "ChasmonauteKids_0"),
                    (1396, "BubblesKids_0"),
                    (1397, "FiremenKids_0"),
                    (1398, "FiremenKids_1"),
                    (1399, "EcoloKids_1"),
                    (1400, "EcoloKids_0"),
                    (1402, "BalletKids_0"),
                    (1403, "BalletKids_1"),
                    (1407, "CarpetKids_0"),
                    (1408, "Senorita_0"),
                    (1409, "Senorita_1"),
                    (1411, "Senorita_1_Gold"),
                    (1412, "Bailando1997_0"),
                    (1413, "Bailando1997_0_Gold"),
                    (1414, "HabibiYaeni_0"),
                    (1415, "HabibiYaeni_1"),
                    (1416, "HabibiYaeni_2"),
                    (1417, "HabibiYaeni_0_Gold"),
                    (1421, "QueTirePaLante_0"),
                    (1422, "QueTirePaLante_1"),
                    (1423, "QueTirePaLante_2"),
                    (1424, "QueTirePaLante_3"),
                    (1425, "QueTirePaLante_1_Gold"),
                    (1426, "InTheNavy_0"),
                    (1427, "InTheNavy_1"),
                    (1428, "InTheNavy_2"),
                    (1429, "InTheNavy_3"),
                    (1430, "InTheNavy_3_Gold"),
                    (1435, "Georgia_0"),
                    (1436, "Georgia_0_Gold"),
                    (1437, "Buscando_0"),
                    (1438, "Buscando_0_Gold"),
                    (1439, "SambaDeJaneiro_0"),
                    (1440, "SambaDeJaneiro_1"),
                    (1441, "SambaDeJaneiro_2"),
                    (1442, "SambaDeJaneiro_1_Gold"),
                    (1443, "TheWeekend_0"),
                    (1444, "TheWeekend_1"),
                    (1445, "TheWeekend_1_Gold"),
                    (1446, "WithoutMe_0"),
                    (1447, "WithoutMe_1"),
                    (1473, "RareALT_0"),
                    (1474, "RareALT_0_Gold"),
                    (1448, "WithoutMe_2"),
                    (1449, "WithoutMe_2_Gold"),
                    (1450, "DibbyDibby_0"),
                    (1451, "DibbyDibby_1"),
                    (1452, "DibbyDibby_0_Gold"),
                    (1453, "Alexandrie_0"),
                    (1454, "Alexandrie_1"),
                    (1455, "Alexandrie_2"),
                    (1456, "Alexandrie_1_Gold"),
                    (1457, "SweetEscape_0"),
                    (1458, "SweetEscape_0_Gold"),
                    (1459, "HeatSeeker_0"),
                    (1460, "HeatSeeker_0_Gold"),
                    (1461, "Juice_0"),
                    (1462, "Juice_1"),
                    (1463, "Juice_2"),
                    (1464, "Juice_1_Gold"),
                    (1465, "Kuliki_0"),
                    (1466, "Kuliki_0_Gold"),
                    (1467, "WhoRun_0"),
                    (1468, "WhoRun_1"),
                    (1469, "WhoRun_2"),
                    (1470, "WhoRun_1_Gold"),
                    (1471, "WithoutMeALTRETAKE_0"),
                    (1472, "WithoutMeALT_0_Gold"),
                    (1473, "RareALT_0"),
                    (1474, "RareALT_0_Gold"),
                    (1475, "DanceMonkey_0"),
                    (1476, "DanceMonkey_0_Gold"),
                    (1477, "HabibiYaeniALT_0"),
                    (1478, "HabibiYaeniALT_0_Gold"),
                    (1479, "DontStartALT_0"),
                    (1480, "DontStartALT_0_Gold"),
                    (1481, "JuiceALT_0"),
                    (1482, "JuiceALT_1"),
                    (1483, "JuiceALT_2"),
                    (1484, "JuiceALT_2_Gold"),
                    (1485, "Zenit_0"),
                    (1486, "Zenit_0_Gold"),
                    (1487, "BuscandoALT_0"),
                    (1488, "BuscandoALT_0_Gold"),
                    (1489, "SambaDeJaneiroALT_0"),
                    (1490, "SambaDeJaneiroALT_1"),
                    (1491, "SambaDeJaneiroALT_1_Gold"),
                    (1492, "Joone_0"),
                    (1493, "Joone_1"),
                    (1494, "Joone_1_Gold"),
                    (1500, "AdoreYou_0"),
                    (1501, "AdoreYou_0_Gold"),
                    (1502, "FeelSpecialALT_0"),
                    (1503, "FeelSpecialALT_0_Gold"),
                    (1504, "Runaway_0"),
                    (1505, "Runaway_1"),
                    (1506, "Runaway_0_Gold"),
                    (1507, "Volar_0"),
                    (1508, "Volar_0_Gold"),
                    (1509, "GetGetDown_0"),
                    (1510, "GetGetDown_1"),
                    (1511, "GetGetDown_2"),
                    (1512, "GetGetDown_3"),
                    (1513, "GetGetDown_3_Gold"),
                    (1517, "WhoRunALTRETAKE_0"),
                    (1518, "WhoRunALT_0_Gold"),
                    (1522, "AllTheGoodGirls_0"),
                    (1523, "AllTheGoodGirls_0_Gold"),
                    (1524, "PacaDance_0"),
                    (1525, "PacaDance_1"),
                    (1526, "PacaDance_1_Gold"),
                    (1527, "Rare_0"),
                    (1528, "Rare_0_Gold"),
                    (1529, "BoyYouCan_0"),
                    (1530, "BoyYouCan_0_Gold"),
                    (1531, "FeelSpecial_0"),
                    (1532, "FeelSpecial_1"),
                    (1533, "FeelSpecial_2"),
                    (1534, "FeelSpecial_1_Gold"),
                    (1535, "InTheNavy_2_Gold_Unlimited"),
                    (1536, "WithoutMe_0_Gold_Unlimited"),
                    (1537, "Magenta_0"),
                    (1538, "Magenta_0_Gold"),
                    (1539, "Lacrimosa_0"),
                    (1540, "Lacrimosa_0_Gold"),
                    (1541, "TillTheWorldEndsALT_0"),
                    (1542, "TillTheWorldEndsALT_0_Gold"),
                    (1543, "TillTheWorldEnds_0"),
                    (1544, "TillTheWorldEnds_0_Gold"),
                    (1545, "DontStart_0"),
                    (1546, "DontStart_0_Gold"),
                    (1547, "KickItALT_0"),
                    (1548, "KickItALT_0_Gold"),
                    (1549, "BlindingLightsALT_0"),
                    (1550, "BlindingLightsALT_0_Gold"),
                    (1551, "KickIt_0"),
                    (1552, "KickIt_1"),
                    (1553, "KickIt_2"),
                    (1554, "KickIt_3"),
                    (1555, "KickIt_1_Gold"),
                    (1556, "FriendInMe_0"),
                    (1557, "FriendInMe_0_Gold"),
                    (1558, "Sorbet_0"),
                    (1559, "Sorbet_1"),
                    (1560, "Sorbet_2"),
                    (1561, "Sorbet_3"),
                    (1562, "Sorbet_3_Gold"),
                    (1563, "Uno_0"),
                    (1564, "Uno_1"),
                    (1565, "Uno_2"),
                    (1566, "Uno_3"),
                    (1567, "Uno_0_Gold"),
                    (1568, "OtherSideSZA_0"),
                    (1569, "OtherSideSZA_0_Gold"),
                    (1570, "BlindingLights_0"),
                    (1571, "BlindingLights_0_Gold"),
                    (1572, "YoLeLlego_0"),
                    (1573, "YoLeLlego_1"),
                    (1574, "YoLeLlego_0_Gold"),
                    (1575, "Temperature_0"),
                    (1576, "Temperature_1"),
                    (1577, "Temperature_2"),
                    (1578, "Temperature_3"),
                    (1579, "Temperature_2_Gold"),
                    (1580, "RainOnMe_0"),
                    (1581, "RainOnMe_1"),
                    (1582, "RainOnMe_2"),
                    (1583, "RainOnMe_3"),
                    (1584, "RainOnMe_1_Gold"),
                    (1585, "SaySo_0"),
                    (1586, "SaySo_1"),
                    (1587, "SaySo_0_Gold"),
                    (1594, "TemperatureALT_0"),
                    (1595, "TemperatureALT_0_Gold"),
                ]),
            ),
            (
                Game::JustDance2020,
                HashMap::from([
                    (1, "Dare_0"),
                    (2, "DogsOut_0"),
                    (3, "EyeOfTheTiger_0"),
                    (4, "GetAround_0"),
                    (5, "HotNCold_0"),
                    (6, "ILikeToMoveIt_0"),
                    (7, "JinGoLoBa_0"),
                    (8, "RingMyBell_0"),
                    (9, "NineAfternoon_0"),
                    (10, "BabyGirl_0"),
                    (11, "CallMe_0"),
                    (12, "ChickenPayback_0"),
                    (13, "ComeOn_1"),
                    (14, "CosmicGirl_0"),
                    (15, "ElectroTribalDLC_0"),
                    (16, "Firework_0"),
                    (76, "EverybodyNeeds_0"),
                    (347, "GangnamStyleDLC_0"),
                    // (922, "?"),
                    // (933, "?"),
                    (953, "OMG_2"),
                    (1058, "FreezeKids_0"),
                    (1059, "FreezeKids_1"),
                    (1060, "SchoolyardKids_0"),
                    (1061, "SchoolyardKids_1"),
                    (1062, "MedievalKids_0"),
                    (1063, "MedievalKids_1"),
                    (1064, "ChefKids_0"),
                    (1065, "AdventurerKids_0"),
                    (1066, "WizardKids_0"),
                    (1067, "BandmasterKids_0"),
                    (1068, "BirthdayKids_0"),
                    (1083, "BalMasque_0"),
                    (1084, "BalMasque_1"),
                    (1085, "BalMasque_2"),
                    (1086, "BalMasque_3"),
                    (1087, "FitButYouKnow_0"),
                    (1092, "IAmTheBest_1"),
                    (1093, "IAmTheBest_0"),
                    (1095, "Vodovorot_0"),
                    (1096, "TheTime_0"),
                    (1097, "TheTime_1"),
                    (1098, "TheTime_2"),
                    (1099, "TheTime_3"),
                    (1101, "AlwaysLookOn_0"),
                    (1102, "AlwaysLookOn_1"),
                    (1103, "AlwaysLookOn_2"),
                    (1104, "AlwaysLookOn_3"),
                    (1105, "Policeman_0"),
                    (1106, "Policeman_1"),
                    (1107, "Policeman_2"),
                    (1108, "RainOverMe_0"),
                    (1109, "BalMasque_1_Gold"),
                    (1110, "FitButYouKnow_0_Gold"),
                    (1111, "IAmTheBest_1_Gold"),
                    (1112, "Vodovorot_0_Gold"),
                    (1113, "TheTime_0_Gold"),
                    (1114, "AlwaysLookOn_1_Gold"),
                    (1115, "Policeman_0_Gold"),
                    (1116, "RainOverMe_0_Gold"),
                    (1117, "KeepInTouch_0"),
                    (1118, "BadBoy_0"),
                    (1119, "BadBoy_1"),
                    (1120, "BadBoy_0_Gold"),
                    (1121, "KeepInTouch_0_Gold"),
                    (1122, "BabyShark_0"),
                    (1123, "BabyShark_1"),
                    (1124, "GodIsAWoman_0"),
                    (1125, "BabyShark_1_Gold"),
                    (1126, "GodIsAWoman_0_Gold"),
                    (1127, "GetAround_0"),
                    (1128, "Firework_0"),
                    (1129, "HeyYa_0"),
                    (1130, "KattiKalandal_1"),
                    (1131, "MonsterMash_0"),
                    (1132, "CaliforniaGurls_0"),
                    (1133, "GotMeDancing_0"),
                    (1134, "HeyBoy_0"),
                    (1135, "WhatYouWait_0"),
                    (1136, "Disturbia_0"),
                    (1137, "IstanbulQUAT_2"),
                    (1138, "RockNRoll_0"),
                    (1139, "YouMakeMeFeelDLC_0"),
                    (1140, "YouReTheFirst_0"),
                    (1141, "RobotRock_0"),
                    (1142, "SheWolf_0"),
                    (1143, "WhereHaveYou_0"),
                    (1144, "Starships_0"),
                    (1145, "CmonDLC_1"),
                    (1146, "ThatPower_1"),
                    (1147, "TurnUpTheLove_0"),
                    (1148, "GetLucky_1"),
                    (1149, "LimaGolf1_0"),
                    (1150, "Luftballons_0"),
                    (1151, "TheFox_1"),
                    (1152, "BuiltForThis_0"),
                    (1153, "Happy_0"),
                    (1154, "Summer_0"),
                    (1155, "WalkThisWay_1"),
                    (1156, "Tetris_3"),
                    (1157, "Macarena_3"),
                    (1158, "Birthday_0"),
                    (1159, "HoldingOut_0"),
                    (1160, "PoundTheAlarm_0"),
                    (1161, "Blame_0"),
                    (1162, "Animals_0"),
                    // (1163, "?"),
                    (1164, "BornThisWay_1"),
                    (1165, "Circus_2"),
                    (1166, "UptownFunk_0"), // C3
                    (1167, "Chiwawa_0"),
                    (1168, "ElectroMambo_0"),
                    (1169, "HeyMama_1"),
                    (1170, "KaboomPow_0"),
                    (1171, "ThisIsHow_0"),
                    (1172, "DieYoungDLC_0"),
                    (1173, "BoomDLC_0"),
                    (1174, "Cotton_0"),
                    (1175, "Fame_0"),
                    (1176, "TribalDance_0"),
                    (1177, "GoodFeeling_0"),
                    (1178, "JaiHo_0"),
                    (1179, "KetchupSong_0"),
                    (1180, "Kurio_1"),
                    (1181, "Lollipop_0"),
                    (1182, "ThatsTheWay_0"),
                    (1183, "TheFinalCountdown_0"),
                    (1184, "ET_0"),
                    (1185, "CrazyLittle_1"),
                    (1186, "CryingBlood_0"),
                    (1187, "GirlsJustWant_0"),
                    (1188, "ScreamNShoutALT_0"),
                    (1189, "WhatIsLove_0"),
                    (1190, "DontStopMe_0"),
                    (1191, "PoPiPo_2"),
                    (1192, "September_1"),
                    (1193, "WorthIt_0"),
                    (1194, "Titanium_0"),
                    (1195, "DragosteaDinTei_1"),
                    (1196, "Bailar_0"),
                    (1197, "HappyFarmKids_0"),
                    (1198, "TumBum_1"),
                    (1199, "Footloose_0"),
                    (1200, "24K_0"),
                    (1201, "Automaton_0"),
                    (1202, "BubblePopALT_1"),
                    (1203, "LoveWard_0"),
                    (1204, "AllYouGotta_0"),
                    (1205, "AnotherOne_3"),
                    (1206, "BubblePop_1"),
                    (1207, "Despacito_3"),
                    (1208, "SwishSwish_2"),
                    (1209, "SwishSwish_3"),
                    (1210, "MamaMia_0"),
                    (1211, "WorkWorkALT_0"),
                    (1212, "Adeyyo_0"),
                    (1213, "MerryChristmasKids_1"),
                    (1214, "MiMiMi_1"),
                    (1215, "MonstersAcademyKids_0"),
                    (1216, "Finesse_1"),
                    (1217, "IFeelItComing_0"),
                    (1218, "MiloscW_0"),
                    (1219, "MadLove_1"),
                    (1220, "OMG_2"),
                    (1221, "NoTearsLeft_0"), // C1
                    (1222, "OneKiss_0"),
                    (1223, "SweetSensation_3"),
                    (1224, "DDUDU_3"),
                    (1225, "Sugar_0"),
                    (1226, "Sugar_1"),
                    (1227, "ConCalma_0"),
                    (1228, "ConCalma_1"),
                    (1229, "ConCalma_1_Gold"),
                    (1231, "Sushii_0"),
                    (1232, "Sushii_0_Gold"),
                    (1236, "SoyYoALT_0"),
                    (1237, "SoyYoALT_1"),
                    (1238, "SoyYoALT_0_Gold"),
                    (1243, "Bangarang_0"),
                    (1244, "Bangarang_0_Gold"),
                    (1245, "365_0"),
                    (1246, "365_0_Gold"),
                    (1247, "TheTimeALT_0"),
                    (1248, "TheTimeALT_0_Gold"),
                    (1249, "IAmTheBestALT_0"),
                    (1250, "IAmTheBestALT_0_Gold"),
                    (1251, "RainOverMeALT_0"),
                    (1252, "RainOverMeALT_0_Gold"),
                    (1253, "?"), // Ubisoft
                    (1254, "?"), // Ubisoft
                    (1255, "?"), // Ubisoft
                    (1256, "?"), // Ubisoft
                    (1257, "?"), // Ubisoft
                    (1258, "ILikeIt_0"),
                    (1259, "ILikeIt_1"),
                    (1260, "ILikeIt_2"),
                    (1261, "ILikeIt_1_Gold"),
                    (1262, "?"), // Ubisoft
                    (1263, "GodIsAWomanALT_0"),
                    (1264, "GodIsAWomanALT_0_Gold"),
                    (1265, "SoyYo_0"),
                    (1266, "SoyYo_0_Gold"),
                    (1267, "TelAviv_0"),
                    (1268, "TelAviv_1"),
                    (1269, "TelAviv_2"),
                    (1270, "TelAviv_2_Gold"),
                    (1271, "Skibidi_0"),
                    (1272, "Skibidi_1"),
                    (1273, "Skibidi_0_Gold"),
                    (1274, "TakiTaki_0"),
                    (1275, "TakiTaki_0_Gold"),
                    (1279, "?"), // Ubisoft
                    (1280, "?"), // Ubisoft
                    (1281, "Swag_0"),
                    (1282, "Swag_0_Gold"),
                    (1283, "SushiiALT_0"),
                    (1284, "SushiiALT_0_Gold"),
                    (1285, "BangarangALT_0"),
                    (1286, "BangarangALT_0_Gold"),
                    (1287, "TakiTakiALT_0"),
                    (1288, "TakiTakiALT_1"),
                    (1289, "TakiTakiALT_1_Gold"),
                    (1290, "GetBusy_0"),
                    (1291, "GetBusy_1"),
                    (1292, "GetBusy_1_Gold"),
                    (1293, "HighHopes_0"),
                    (1294, "HighHopes_1"),
                    (1295, "HighHopes_2"),
                    (1296, "HighHopes_3"),
                    (1297, "HighHopes_0_Gold"),
                    (1298, "KillThisLove_0"),
                    (1299, "KillThisLove_1"),
                    (1300, "KillThisLove_2"),
                    (1301, "KillThisLove_3"),
                    (1302, "KillThisLove_2_Gold"),
                    (1303, "Talk_0"),
                    (1304, "Talk_0_Gold"),
                    (1306, "MaItu_0"),
                    (1307, "MaItu_0_Gold"),
                    (1308, "Everybody_0"),
                    (1309, "Everybody_1"),
                    (1310, "Everybody_2"),
                    (1311, "Everybody_3"),
                    (1312, "Everybody_2_Gold"),
                    (1316, "TalkALT_0"),
                    (1317, "TalkALT_0_Gold"),
                    (1318, "OldTownRoad_0"),
                    (1319, "OldTownRoad_0_Gold"),
                    (1320, "NotYourOrdinary_Gold_Unlimited"),
                    (1321, "ILikeIt_0_Gold_Unlimited"),
                    (1322, "7Rings_0"),
                    (1323, "7Rings_1"),
                    (1324, "7Rings_2"),
                    (1325, "7Rings_1_Gold"),
                    (1326, "BadGuy_0"),
                    (1327, "BadGuy_0_Gold"),
                    (1328, "Footwork_0"),
                    (1329, "Footwork_0_Gold"),
                    (1330, "OldTownRoadALT_0"),
                    (1331, "OldTownRoadALT_1"),
                    (1332, "OldTownRoadALT_2"),
                    (1333, "OldTownRoadALT_2_Gold"),
                    (1334, "JustAnIllusion_0"),
                    (1335, "JustAnIllusion_1"),
                    (1336, "JustAnIllusion_1_Gold"),
                    (1337, "StopMovin_0"),
                    (1338, "StopMovin_1"),
                    (1339, "StopMovin_2"),
                    (1340, "StopMovin_1_Gold"),
                    (1342, "7RingsALT_0"),
                    (1343, "7RingsALT_0_Gold"),
                    (1344, "KillThisLoveALT_0"),
                    (1345, "KillThisLoveALT_0_Gold"),
                    (1346, "BassaSababa_0"),
                    (1347, "BassaSababa_0_Gold"),
                    (1348, "FancyTwice_0"),
                    (1349, "FancyTwice_1"),
                    (1350, "FancyTwice_2"),
                    (1351, "UglyBeauty_0"),
                    (1352, "UglyBeauty_1"),
                    (1353, "UglyBeauty_2"),
                    (1354, "DoCarnaval_0"),
                    (1355, "DoCarnaval_0_Gold"),
                    (1356, "ConAltura_0"),
                    (1357, "ConAltura_0_Gold"),
                    (1358, "IDontCare_0"),
                    (1359, "IDontCare_0_Gold"),
                    (1360, "UglyBeauty_0_Gold"),
                    (1361, "FancyTwice_2_Gold"),
                    (1362, "CanCan_0"),
                    (1363, "CanCan_1"),
                    (1364, "CanCan_2"),
                    (1365, "CanCan_3"),
                    (1366, "CanCan_4"),
                    (1367, "CanCan_1_Gold"),
                    (1368, "BoyWithLuv_0"),
                    (1369, "BoyWithLuv_1"),
                    (1370, "BoyWithLuv_2"),
                    (1371, "BoyWithLuv_2_Gold"),
                ]),
            ),
            (
                Game::JustDance2019,
                HashMap::from([
                    (1, "Dare_0"),
                    (2, "DogsOut_0"),
                    (3, "EyeOfTheTiger_0"),
                    (4, "GetAround_0"),
                    (5, "HotNCold_0"),
                    (6, "ILikeToMoveIt_0"),
                    (7, "JinGoLoBa_0"),
                    (8, "RingMyBell_0"),
                    (9, "NineAfternoon_0"),
                    (10, "BabyGirl_0"),
                    (11, "CallMe_0"),
                    (12, "ChickenPayback_0"),
                    (13, "ComeOn_1"),
                    (14, "CosmicGirl_0"),
                    (15, "ElectroTribalDLC_0"),
                    (16, "Firework_0"),
                    (76, "EverybodyNeeds_0"),
                    (77, "FunHouseDLC_0"),
                    (252, "Animals_0"),
                    (347, "GangnamStyleDLC_0"),
                    (694, "BlowYourMind_0"),
                    (695, "AnotherOne_1"),
                    (697, "Carmen_0"),
                    (700, "Blue_0"),
                    (704, "HappyFarmKids_0"),
                    (705, "SideTo_0"),
                    (706, "MakeItJingle_0"),
                    (709, "AnotherOneALT_0"),
                    (710, "SideToALT_0"),
                    (711, "WakaWakaALT_0"),
                    (713, "8BitRetake_0"),
                    (714, "AutomatonALT_0"),
                    (716, "Rockabye_1"),
                    (718, "BubblePopALT_3"),
                    (720, "Footloose_0"), // C3
                    (722, "24KALT_0"),
                    (723, "Diggy_0"),
                    (724, "Chantaje_0"),
                    (725, "LoveWard_1"),
                    (726, "TumBumALT_0"),
                    (727, "SayonaraRetake_0"),
                    (729, "ChantajeALT_1"),
                    (730, "Automaton_0"),
                    (732, "BubblePop_2"),
                    (738, "ItsyBitsyRetake_1"),
                    (739, "ItsyBitsyRetake_0"),
                    (741, "WakaWakaKids_0"),
                    (742, "MagicHalloweenKids_1"),
                    (743, "MagicHalloweenKids_0"),
                    (748, "BubblePopALT_2"),
                    (749, "BubblePopALT_1"),
                    (750, "BubblePopALT_0"),
                    (775, "FootlooseKids_0"),
                    (776, "LoveWard_0"),
                    (778, "KeepOn_0"),
                    (780, "RiskyBusiness_0"),
                    (781, "Dharma_0"),
                    (782, "Cottonmouth_0"),
                    (785, "DharmaALT_0"),
                    (808, "ShapeOfYou_0"),
                    (809, "AnotherOne_2"),
                    (810, "BadLiar_0"),
                    (811, "KissingStrangers_1"),
                    (812, "MissAmazingKIDS_0"),
                    (813, "NewFace_1"),
                    (814, "SlumberParty_1"),
                    (816, "FunkyRobotKids_0"),
                    (817, "PixieLandKids_0"),
                    (818, "Sidewinder_0"),
                    (820, "AllYouGotta_0"),
                    (821, "AnotherOne_3"),
                    (822, "BubblePop_0"),
                    (823, "BubblePop_1"),
                    (824, "Copperhead_1"),
                    (825, "Despacito_3"),
                    (826, "DespacitoALT_0"),
                    (827, "HowFar_0"),
                    (828, "Instruction_0"),
                    (829, "JohnW_0"),
                    (830, "KissingStrangers_0"),
                    (831, "KissingStrangersALT_1"),
                    (832, "NaughtyGirl_0"),
                    (833, "SwishSwish_0"),
                    (834, "SwishSwish_1"),
                    (835, "SwishSwish_2"),
                    (836, "SwishSwish_3"),
                    (837, "WakaWakaALT_1"),
                    (883, "WhereAreYou_2"),
                    (884, "ObsessionRetake_0"),
                    (885, "MamaMia_0"),
                    (886, "GhostKids_0"),
                    (887, "GhostKids_1"),
                    (888, "JurassicKids_0"),
                    (889, "WorkWorkALT_0"),
                    (890, "ImStillStanding_0"),
                    (891, "SpaceGirlKids_0"),
                    (892, "Fire_0"),
                    (893, "CaPlane_0"),
                    (894, "Shaky_0"),
                    (895, "SaintPatrickKids_0"),
                    (896, "SaintPatrickKids_1"),
                    (897, "Adeyyo_0"),
                    (898, "MerryChristmasKids_0"),
                    (899, "MerryChristmasKids_1"),
                    (900, "NinjaKids_0"),
                    (901, "MadLove_0"),
                    (902, "LittlePartyALT_1"),
                    (903, "MiMiMi_0"),
                    (904, "MiMiMi_1"),
                    (905, "TheExplorerKids_0"),
                    (906, "WorkWork_0"),
                    (907, "MonstersAcademyKids_0"),
                    (908, "Narco_0"),
                    (909, "BumBumTamTamALT_0"),
                    (910, "Finesse_1"),
                    (911, "LittleParty_0"),
                    (912, "Rhythm_0"),
                    (913, "IFeelItComing_0"),
                    (914, "Havana_0"),
                    (915, "MakeMeFeel_0"),
                    (916, "WaterMe_0"),
                    (917, "MiloscW_0"),
                    (918, "FinesseALT_0"),
                    (919, "MiMiMiALT_0"),
                    (920, "MiMiMiALT_1"),
                    (921, "MiMiMiALT_2"),
                    (922, "MiMiMiALT_3"),
                    (923, "BumBumTamTamALT_1"),
                    (924, "MadLove_1"),
                    (925, "Finesse_0"),
                    (926, "Finesse_2"),
                    (927, "Finesse_3"),
                    (928, "ObsessionRetake_1"),
                    (929, "WhereAreYou_0"),
                    (930, "WhereAreYou_1"),
                    (931, "WorkWork_1"),
                    (932, "WorkWork_2"),
                    (933, "Fire_1"),
                    (934, "MamaMia_1"),
                    (935, "LittlePartyALT_0"),
                    (936, "Bang2019_0"),
                    (937, "NewWorld_0"),
                    (938, "NewReality_0"),
                    (939, "NotYourOrdinary_0"),
                    (940, "NotYourOrdinary_1"),
                    (941, "NotYourOrdinary_2"),
                    (942, "NotYourOrdinary_3"),
                    (943, "WhereAreYouALT_0"),
                    (944, "WhereAreYouALT_1"),
                    (945, "FireOnTheDancefloor_0"),
                    (946, "LittlePartyALT_2"),
                    (947, "DameTu_0"),
                    (948, "WaterMeALT_0"),
                    (949, "WaterMeALT_1"),
                    (950, "NewRules_0"),
                    (951, "OMG_0"),
                    (952, "OMG_1"),
                    (953, "OMG_2"),
                    (954, "OMGALT_0"),
                    (955, "BumBumTamTam_0"),
                    (956, "BumBumTamTam_1"),
                    (959, "SweetLittle_0"),
                    (961, "NewRulesALT_0"),
                    (962, "UbiSoftRainbowSixSiegeEla"),
                    (963, "UbiSoftRainbowSixSiegeTachanka"),
                    (964, "UbiSoftRainbowSixSiegeAsh"),
                    (965, "UbiSoftRainbowSixSiegeDokkaebi"),
                    (966, "UbiSoftACOdysseyAlexios"),
                    (967, "UbiSoftACOdysseyKassandra"),
                    (968, "UbiSoftWatchDogs2Sitara"),
                    (969, "UbiSoftWatchDogs2Wrench"),
                    (982, "PocoLoco_0"),
                    (983, "PacMan_0"),
                    (984, "PacMan_1"),
                    (985, "PacMan_2"),
                    (986, "PacMan_3"),
                    (987, "UbiSoftTheCrew2"),
                    (988, "MadLoveALT_0"),
                    (989, "Familiar_0"),
                    (990, "UbiSoftJD2019Unknown"),
                    (991, "TOY_0"),
                    (992, "SangriaWine_0"),
                    (993, "Bang2019ALT_0"),
                    (994, "HavanaALT_0"),
                    (995, "HavanaALT_1"),
                    (996, "NoTearsLeft_0"),
                    (997, "NoTearsLeft_1"),
                    (998, "OneKiss_0"),
                    (999, "Calypso_0"),
                    (1000, "SweetSensation_0"),
                    (1001, "SweetSensation_1"),
                    (1002, "SweetSensation_2"),
                    (1003, "SweetSensation_3"),
                    (1004, "NiceForWhat_0"),
                    (1005, "NiceForWhat_1"),
                    (1006, "NiceForWhat_2"),
                    (1007, "NiceForWhat_3"),
                    (1008, "Mayores_0"),
                    (1009, "RaveIn_0"),
                    (1010, "RaveIn_1"),
                    (1011, "RaveIn_2"),
                    (1012, "RaveIn_3"),
                    (1013, "DDUDU_0"),
                    (1014, "DDUDU_1"),
                    (1015, "DDUDU_2"),
                    (1016, "DDUDU_3"),
                    (1019, "Sugar_0"),
                    (1020, "Sugar_1"),
                    (1021, "Sugar_2"),
                    (1022, "Sugar_3"),
                    (1023, "Sugar_4"),
                    (1024, "Sugar_5"),
                    (1025, "Sugar_6"),
                    (1026, "Sugar_7"),
                    (1027, "Sugar_8"),
                    (1028, "Sugar_9"),
                    (1029, "NiceForWhat_4"),
                    (1030, "NiceForWhat_5"),
                    (1031, "Panda"),
                ]),
            ),
            (
                Game::JustDance2018,
                HashMap::from([
                    (1, "Dare_0"),
                    (2, "DogsOut_0"),
                    (3, "EyeOfTheTiger_0"),
                    (4, "GetAround_0"),
                    (5, "HotNCold_0"),
                    (6, "ILikeToMoveIt_0"),
                    (7, "JinGoLoBa_0"),
                    (8, "RingMyBell_0"),
                    (9, "NineAfternoon_0"),
                    (10, "BabyGirl_0"),
                    (11, "CallMe_0"),
                    (12, "ChickenPayback_0"),
                    (13, "ComeOn_1"),
                    (14, "CosmicGirl_0"),
                    (15, "ElectroTribalDLC_0"),
                    (16, "Firework_0"),
                    (76, "EverybodyNeeds_0"),
                    (77, "FunHouseDLC_0"),
                    (252, "Animals_0"),
                    (347, "GangnamStyleDLC_0"),
                    (448, "Ghostbusters"),
                    (691, "WakaWaka_1"),
                    (694, "BlowYourMind"),
                    (695, "AnotherOne_2"),
                    (697, "Carmen"),
                    (698, "Carmen_1"),
                    (699, "DaddyCool"),
                    (700, "Blue"),
                    (704, "HappyFarmKids"),
                    (705, "SideTo"),
                    (706, "MakeItJingle"),
                    (708, "GotThat"),
                    (709, "AnotherOneALT"),
                    (710, "SideToALT"),
                    (711, "WakaWakaALT"),
                    (713, "8BitRetake"),
                    (714, "AutomatonALT"),
                    (715, "TumBum_1"),
                    (716, "Rockabye_1"),
                    (717, "FearlessPirateKids"),
                    (718, "BubblePopALT_3"),
                    (720, "Footloose"),
                    (721, "24K"),
                    (722, "24KALT"),
                    (723, "Diggy"),
                    (724, "Chantaje"),
                    (725, "LoveWard_1"),
                    (726, "TumBumALT"),
                    (727, "SayonaraRetake"),
                    (729, "ChantajeALT_1"),
                    (730, "Automaton"),
                    (731, "JohnWALT"),
                    (732, "BubblePop_2"),
                    (738, "ItsyBitsyRetake_1"),
                    (739, "ItsyBitsyRetake"),
                    (741, "WakaWakaKids"),
                    (742, "MagicHalloweenKids_1"),
                    (743, "MagicHalloweenKids"),
                    (748, "BubblePopALT_2"),
                    (749, "BubblePopALT_1"),
                    (750, "BubblePopALT"),
                    (753, "LoveIsAll"),
                    (775, "FootlooseKids"),
                    (776, "LoveWard"),
                    (778, "KeepOn"),
                    (780, "RiskyBusiness"),
                    (781, "Dharma"),
                    (782, "Cottonmouth"),
                    (785, "DharmaALT"),
                    (786, "WDF_Glitchy"),
                    (803, "UbiSoftRabbidsApache"),
                    (804, "UbiSoftRabbidsCotton"),
                    (805, "UbiSoftRabbidsSexyAndIKnowItDLC"),
                    (808, "ShapeOfYou"),
                    (809, "AnotherOne_2"),
                    (810, "BadLiar"),
                    (811, "KissingStrangers_1"),
                    (812, "MissAmazingKids"),
                    (813, "NewFace_1"),
                    (814, "SlumberParty_1"),
                    (815, "BeepBeep"),
                    (816, "FunkyRobotKids"),
                    (817, "PixieLandKids"),
                    (818, "Sidewinder"),
                    (820, "AllYouGotta"),
                    (821, "AnotherOne_3"),
                    (822, "BubblePop"),
                    (823, "BubblePop_1"),
                    (824, "Copperhead"),
                    (825, "Despacito_3"),
                    (826, "DespacitoALT"),
                    (827, "HowFar"),
                    (828, "Instruction"),
                    (829, "JohnW"),
                    (830, "KissingStrangers"),
                    (831, "KissingStrangersALT_1"),
                    (832, "NaughtyGirl"),
                    (833, "SwishSwish"),
                    (834, "SwishSwish_1"),
                    (835, "SwishSwish_2"),
                    (836, "SwishSwish_3"),
                    (837, "WakaWakaALT_1"),
                ]),
            ),
            (
                Game::JustDance2017,
                HashMap::from([
                    (1, "Dare_0"),
                    (2, "DogsOut_0"),
                    (3, "EyeOfTheTiger_0"),
                    (4, "GetAround_0"),
                    (5, "HotNCold_0"),
                    (6, "ILikeToMoveIt_0"),
                    (7, "JinGoLoBa_0"),
                    (8, "RingMyBell_0"),
                    (9, "NineAfternoon_0"),
                    (10, "BabyGirl_0"),
                    (11, "CallMe_0"),
                    (12, "ChickenPayback_0"),
                    (13, "ComeOn_1"),
                    (14, "CosmicGirl_0"),
                    (15, "ElectroTribalDLC_0"),
                    (16, "Firework_0"),
                    (23, "KattiKalandal_0"),
                    (33, "Song2_0"), // C1
                    (51, "HalloweenQUAT_2"),
                    (52, "HalloweenQUAT_3"),
                    (67, "BewareOf_0"),
                    (72, "CrucifiedQUAT_3"),
                    (73, "CrucifiedQUAT_0"),
                    (76, "EverybodyNeeds_0"),
                    (77, "FunHouseDLC_0"),
                    (78, "GangnamStyleDLC_1_C1_V1"), // C1
                    (80, "IstanbulQUAT_0"),
                    (90, "RockLobster_0"),
                    (97, "TimeWarpQUAT_1"),
                    (99, "WildWildWestQUAT_2"),
                    (106, "IWillSurvive_0"),
                    (115, "RobotRock_1"),
                    (118, "Aquarius_1"),
                    (127, "Gigolo_1"),
                    (131, "Limbo_1"),
                    (175, "UbiSoftFarCry"),
                    (179, "UbiSoftZombiU"),
                    (180, "UbiSoftRayman"),
                    (239, "BollywoodXmas_0"),
                    (252, "Animals_0"),
                    (253, "LetsGroove_0"),
                    (254, "WilliamTell_0"),
                    (260, "Nintendo_Mario"),
                    (274, "Chiwawa_0"),
                    (309, "UbiSoftACUnityArno"),
                    (314, "UbiSoftChildOfLightAurora"),
                    (345, "GangnamStyleDLC_1_C2"),    // C2
                    (346, "GangnamStyleDLC_1_C1_V2"), // C1
                    (347, "GangnamStyleDLC_0"),
                    (353, "DieYoungDLC_1"),
                    (357, "TimberDLC_1"),
                    (360, "MovesLikeDLC_0_C1"),
                    (362, "OneThingDLC_1"),
                    (364, "KiloPapaDLC_0_V2"),
                    (368, "BlurredLines_1"),
                    (369, "Gentleman_0_C2"),
                    (370, "ThatPower_3"),
                    (371, "KissYou_2"),
                    (372, "CmonDLC_0"),
                    (373, "WhatMakesYouBeautiful_0"),
                    (374, "MovesLikeDLC_0_C4"),
                    (375, "DynamiteQUAT_0"),
                    (376, "DaFunk_1"),
                    (378, "Pilgrim"),
                    (379, "SantaClaus"),
                    (380, "GetLucky_0"),
                    (381, "MahNa_1"),
                    (382, "GangnamStyleDLC_Horse"),
                    (383, "AThousandDances_Penguin"),
                    (385, "Lollipop_SockPuppet"),
                    (478, "LeanOn_0"),
                    (479, "LeanOn_1"),
                    (480, "ScreamNShout_3"),
                    (481, "ScreamNShoutALT_0"),
                    (498, "Hips_0"),
                    (499, "WhatIsLove_0"),
                    (500, "WorthItALT_2"),
                    (504, "DontStopMe_0"), // C1
                    (505, "Daddy_1"),
                    (506, "TicoTico_1"),
                    (507, "Daddy_0"),
                    (516, "Samba_0"),
                    (517, "Radical_0"),
                    (518, "PoPiPo_2"),
                    (519, "LeanOnALT_0"),
                    (520, "September_1"),
                    (522, "ElTiki_1"),
                    (524, "GhostInTheKeys_3"),
                    (525, "WhatIsLoveALT_1"),
                    (526, "DontStopMeALT_0"),
                    (527, "WorthIt_0"),
                    (528, "Titanium_0"),
                    (529, "Sorry_0"),
                    (539, "SeptemberALT_0"),
                    (541, "DaddyALT_1"),
                    (542, "CheapThrills_0"),
                    (543, "RunTheNight_0"),
                    (544, "Oishii_0"),
                    (545, "CantFeelMyFace_0"),
                    (546, "DragosteaDinTei_1"),
                    (547, "Groove_1"),
                    (550, "UbiSoftACCChineShaoJun"),
                    (551, "UbiSoftACCRussiaNikolai"),
                    (552, "UbiSoftACCIndiaArbaaz"),
                    (554, "UbiSoftRabbidsDisturbia"),
                    (555, "UbiSoftRainbow6SiegeIQ"),
                    (557, "UbiSoftWatchDogs2Marcus"),
                    (559, "ColaSong_0"),
                    (560, "DQ_Mask"),
                    (561, "DQ_CoolKids"),
                    (562, "DQ_AroundTheWorld"),
                    (563, "DQ_NewHeroines"),
                    (564, "DQ_Family"),
                    (565, "DQ_HighEnergy"),
                    (566, "DQ_YearRound"),
                    (567, "DQ_KeepCalm"),
                    (568, "DQ_StrongWoman"),
                    (569, "ColaSongALT_0"),
                    (570, "CakeByTheOceanALT_1"),
                    (571, "ElTikiALT_1"),
                    (572, "IntoYou_0"),
                    (573, "CheapThrillsALT_0"),
                    (574, "Bailar_0"),
                    (575, "WhereverIGo_0"),
                    (576, "NaeNaeALT_1"),
                    (578, "Bonbon_0"),
                    (580, "Leila_0"),
                    (581, "ILoveRock_0"),
                    (582, "CakeByTheOcean_0"),
                    (586, "DQ_Snack"),
                    (587, "SorryALT_0"),
                    (588, "TeDominar_1"),
                    (589, "LastChristmas_0"),
                    (590, "AllAboutUs_2"),
                    (591, "Bang_0"),
                    (592, "Bicicleta_0"),
                    (593, "RedMangoose_0"),
                    (594, "SingleLadies_0"),
                    (595, "NaeNae_1"),
                    (596, "Oishii_1"),
                    (598, "HipsALT_1"),
                    (599, "RadicalALT_1"),
                    (600, "LittleSwing_1"),
                    // (606, "?"),
                    // (607, "?"),
                    // (608, "?"),
                    // (609, "?"),
                    // (639, "?"),
                    // (640, "?"),
                    // (641, "?"),
                    // (642, "?"),
                    (646, "HowDeep_0"),
                ]),
            ),
        ])
    })
}
