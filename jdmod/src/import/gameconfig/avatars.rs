//! # Avatars
//! Import all the avatars.
//!
//! Current implementation is a bit wonky. A better option would be too manually match all avatar ids
//! to names per game. Then Just Dance 2017 avatars can also be imported.
use std::{borrow::Cow, collections::HashMap, fs::File, io::Write, sync::OnceLock};

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

/// Import all the avatars (Just Dance 2018-2022)
pub fn import_v18v22(
    is: &ImportState<'_>,
    avatardb_scene: &str,
    avatarsobjectives: Option<&AvatarsObjectives>,
) -> Result<(), Error> {
    let empty_objectives = HashMap::new();
    println!("Importing avatars...");

    // Load existing avatars in the mod
    let avatars_config_path = is.dirs.avatars().join("avatars.json");
    let mut avatars: HashMap<String, Avatar> = if avatars_config_path.exists() {
        let file = File::open(&avatars_config_path)?;
        serde_json::from_reader(file)?
    } else {
        HashMap::new()
    };

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
        let actor_template = template.actor()?;
        test(&actor_template.components.len(), &2)
            .context("Not exactly two components in actor")?;
        let avatar_desc = actor_template.components[1].avatar_description()?;

        // Create a (hopefully) unique name for the avatar
        // TODO: Maybe just use a static mapping for each game?
        let name = format!(
            "{}_{}{}{}",
            avatar_desc.used_as_coach_map_name.as_ref(),
            avatar_desc.used_as_coach_coach_id,
            if avatar_desc.special_effect == Some(1) {
                "_Gold"
            } else {
                ""
            },
            if avatar_desc.unlock_type == 22 {
                "_Unlimited"
            } else {
                ""
            }
        );
        println!("Mapping {} to {}", avatar_desc.avatar_id, name);
        // Add the name to the id map, so we can look it up later
        id_map.insert(avatar_desc.avatar_id, name.clone());

        // Collect the avatar descriptions so we don't have the parse isc and tpls again
        avatar_descriptions.push(avatar_desc.to_owned());
    }

    for avatar_desc in avatar_descriptions {
        let name = &id_map[&avatar_desc.avatar_id];
        // Only add new avatars
        if !avatars.contains_key(name) {
            let avatar_named_dir_path = is.dirs.avatars().join(name);
            let avatar_image_path = format!("{name}/avatar.png");
            let avatar_image_phone_path = format!("{name}/avatar_phone.png");
            let avatar = Avatar {
                relative_song_name: if avatar_desc.relative_song_name.is_empty() {
                    None
                } else {
                    Some(Cow::Owned(avatar_desc.relative_song_name))
                },
                sound_family: Cow::Owned(avatar_desc.sound_family),
                status: avatar_desc.status,
                unlock_type: UnlockType::from_unlock_type(
                    avatar_desc.unlock_type,
                    avatarsobjectives.get(&avatar_desc.avatar_id),
                )?,
                used_as_coach_map_name: Cow::Owned(avatar_desc.used_as_coach_map_name),
                used_as_coach_coach_id: avatar_desc.used_as_coach_coach_id,
                special_effect: avatar_desc.special_effect == Some(1),
                main_avatar: avatar_desc.main_avatar_id.and_then(|main_id| {
                    if main_id == u16::MAX {
                        None
                    } else {
                        let main = id_map.get(&main_id).map(String::as_str).map(Cow::Borrowed);
                        if main.is_none() {
                            println!("Warning! Avatar id {main_id} does not exist!");
                            println!("Failed to add main avatar for {name}");
                        } else {
                            println!("Adding main avatar {main:?} for {name}");
                        }
                        main
                    }
                }),
                image_path: Cow::Owned(avatar_image_path),
                image_phone_path: Cow::Owned(avatar_image_phone_path),
            };
            std::fs::create_dir(&avatar_named_dir_path)?;
            let alt_actor_file = is
                .vfs
                .open(cook_path(avatar_desc.actor_path.as_ref(), is.platform)?.as_ref())?;
            let alt_actor = cooked::act::parse(&alt_actor_file, is.game)?;

            let image_actor = alt_actor
                .components
                .first()
                .ok_or_else(|| anyhow!("No templates in {}", avatar_desc.actor_path))?;
            let mtg = image_actor.data.material_graphics_component()?;

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
            avatar_image_phone_file.write_all(&is.vfs.open(avatar_desc.phone_image.as_ref())?)?;

            avatars.insert(name.clone(), avatar);
        }
    }

    // TODO: Detect unreferenced avatars and try to import them?

    let file = File::create(avatars_config_path)?;
    serde_json::to_writer_pretty(file, &avatars)?;

    Ok(())
}

/// Maps avatar ids to their proper names for each game
static GAME_AVATAR_ID_NAME_MAP: OnceLock<HashMap<Game, HashMap<u16, &'static str>>> =
    OnceLock::new();

/// Get the name for the `avatar_id` for `game`
fn get_name(game: Game, avatar_id: u16) -> Result<&'static str, Error> {
    get_map()
        .get(&game)
        .ok_or_else(|| anyhow!("Unsupported game: {game}"))?
        .get(&avatar_id)
        .copied()
        .ok_or_else(|| anyhow!("Unknown ID: {avatar_id}"))
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
                    (5, "HotNCold"),
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
                    (5, "HotNCold"),
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
                    // (347, "?"),
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
                    (5, "HotNCold"),
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
                    // (347, "?"),
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
                    (1163, "?"),
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
        ])
    })
}
