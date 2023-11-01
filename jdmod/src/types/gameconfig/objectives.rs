//! # Objectives
//! Types for objectives (as used by scheduled quests)
use std::{
    borrow::Cow,
    collections::{
        hash_map::{DefaultHasher, Entry},
        HashMap,
    },
    hash::{Hash, Hasher},
};

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::testing::test;
use serde::{Deserialize, Serialize};
use ubiart_toolkit::json_types;

use crate::types::{
    localisation::{LocaleId, LocaleIdMap},
    song::{NumberOfCoaches, Tag},
};

// TODO: Replace with BiMap which allows mapping multiple strings to one objective
/// Represents all the objectives in the mod
///
/// Note: an objective can have multiple names but a name cannot refer to multiple objectives
#[derive(Debug, Clone, Default)]
pub struct Objectives<'a> {
    /// Mapping from a objective to a objective name
    pub objective_map: HashMap<Objective<'a>, String>,
    /// Mapping from objective name to the objective
    pub name_map: HashMap<String, Objective<'a>>,
}

/// Describes an objective
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Objective<'a> {
    /// What does a user need to do
    pub objective_type: ObjectiveType<'a>,
    /// Description of the objective
    pub description: LocaleId,
    /// Description of the objective as a string (empty most of the time)
    pub description_raw: Cow<'a, str>,
    /// Can this objective be achieved multiple times
    pub is_static: bool,
    /// Don't sync with online?
    pub exclude_from_upload: bool,
}

impl<'a> Objectives<'a> {
    /// Add an objective that does not have a name.
    ///
    /// This will generate a name for the objective that is deterministic but won't match
    /// names provided by the game itself.
    ///
    /// # Errors
    /// Will error if an objective already exists for the generated name but does not match `objective`
    pub fn add_objective(&mut self, objective: Objective<'a>) -> Result<String, Error> {
        if let Some(name) = self.objective_map.get(&objective) {
            Ok(name.clone())
        } else {
            let name = objective.generate_name();
            if self.name_map.contains_key(&name) {
                return Err(anyhow!(
                    "Generated name already in Objectives but Objective is not in object_map!"
                ));
            }
            self.name_map.insert(name.clone(), objective.clone());
            self.objective_map.insert(objective, name.clone());
            Ok(name)
        }
    }

    /// Add an objective with a name
    ///
    /// # Errors
    /// Will return an error if an objective already exists for `name` and it does not match `objective`
    pub fn add_objective_with_name(
        &mut self,
        objective: Objective<'a>,
        name: String,
    ) -> Result<(), Error> {
        // If objective is already known under this name, return.
        // Else if objective is not known, add it under this name and return.
        // Else if objective is known under another name, add this alias and return.
        if self.objective_map.get(&objective) == Some(&name)
            || self.name_map.get(&name) == Some(&objective)
        {
            Ok(())
        } else if let Entry::Vacant(entry) = self.objective_map.entry(objective.clone()) {
            self.name_map.insert(name.clone(), objective);
            entry.insert(name);
            Ok(())
        } else if let Entry::Vacant(entry) = self.name_map.entry(name.clone()) {
            entry.insert(objective);
            Ok(())
        } else {
            Err(anyhow!(
                "{name} already exists in Objectives and has a different objective!"
            ))
        }
    }
}

impl Hash for Objective<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.objective_type.hash(state);
        self.is_static.hash(state);
    }
}

impl Objective<'_> {
    /// Generate a name for this objective
    pub fn generate_name(&self) -> String {
        // TODO: Generate a nicer name, preferably matching existing naming convention
        let sd = if self.is_static { "Static" } else { "Dynamic" };
        let mut hasher = DefaultHasher::new();
        self.objective_type.hash(&mut hasher);
        self.description.hash(&mut hasher);
        self.exclude_from_upload.hash(&mut hasher);
        format!("{sd}_{:x}", hasher.finish())
    }
}

impl<'a> Objective<'a> {
    /// Convert from the UbiArt representation
    pub fn from_descriptor(
        descriptor: &json_types::ObjectiveDescriptor<'a>,
        locale_id_map: &LocaleIdMap,
    ) -> Result<Self, Error> {
        Ok(Self {
            objective_type: ObjectiveType::from_descriptor(descriptor, locale_id_map)?,
            description: locale_id_map
                .get(descriptor.description())
                .unwrap_or_default(),
            description_raw: descriptor.description_raw(),
            is_static: descriptor.is_static(),
            exclude_from_upload: descriptor.exclude_from_upload(),
        })
    }

    /// Convert from the old UbiArt represntation
    pub fn from_old_descriptor(
        descriptor: &json_types::ObjectiveDesc1819<'a>,
        unlimited_only: bool,
        locale_id_map: &LocaleIdMap,
    ) -> Self {
        let (o_type, o_static) = ObjectiveType::from_old_descriptor(descriptor, unlimited_only);
        Self {
            objective_type: o_type,
            description: locale_id_map
                .get(descriptor.description())
                .unwrap_or_default(),
            description_raw: Cow::Borrowed(""),
            is_static: o_static,
            exclude_from_upload: true,
        }
    }
}

impl<'a> From<Objective<'a>> for json_types::ObjectiveDescriptor<'a> {
    fn from(value: Objective<'a>) -> Self {
        match value.objective_type {
            ObjectiveType::AccumulateXCal(data) => json_types::ObjectiveDescriptor::AccumulateXCal(
                json_types::ObjectiveDescriptorAccumulateXCal {
                    description: value.description,
                    description_raw: value.description_raw,
                    components: data.components.into_iter().map(Component::into).collect(),
                    is_static: value.is_static,
                    exclude_from_upload: value.exclude_from_upload,
                    calories_amount: data.calories_amount,
                    in_one_session: data.in_one_session,
                    ..Default::default()
                },
            ),
            ObjectiveType::AccumulateXMoves(data) => {
                json_types::ObjectiveDescriptor::AccumulateXMoves(
                    json_types::ObjectiveDescriptorAccumulateXMoves {
                        description: value.description,
                        description_raw: value.description_raw,
                        components: data.components.into_iter().map(Component::into).collect(),
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        moves_count: data.moves_count,
                        categories_to_count: data
                            .categories_to_count
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        ..Default::default()
                    },
                )
            }
            ObjectiveType::AddXSongsToAPlaylist(data) => {
                json_types::ObjectiveDescriptor::AddXSongsToAPlaylist(
                    json_types::ObjectiveDescriptorAddXSongsToAPlaylist {
                        description: value.description,
                        description_raw: value.description_raw,
                        components: data.components.into_iter().map(Component::into).collect(),
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        songs_added_count: data.songs_added_count,
                        ..Default::default()
                    },
                )
            }
            ObjectiveType::ChangeCustoItemXTimes(data) => {
                json_types::ObjectiveDescriptor::ChangeCustoItemXTimes(
                    json_types::ObjectiveDescriptorChangeCustoItemXTimes {
                        description: value.description,
                        description_raw: value.description_raw,
                        components: data.components.into_iter().map(Component::into).collect(),
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        custo_item_changes_count: data.custo_item_changes_count,
                        ..Default::default()
                    },
                )
            }
            ObjectiveType::CompleteXQuests(data) => {
                json_types::ObjectiveDescriptor::CompleteXQuests(
                    json_types::ObjectiveDescriptorCompleteXQuests {
                        description: value.description,
                        description_raw: value.description_raw,
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        quests_count: data.quests_count,
                        ..Default::default()
                    },
                )
            }
            ObjectiveType::DanceXSeconds(data) => json_types::ObjectiveDescriptor::DanceXSeconds(
                json_types::ObjectiveDescriptorDanceXSeconds {
                    description: value.description,
                    description_raw: value.description_raw,
                    is_static: value.is_static,
                    exclude_from_upload: value.exclude_from_upload,
                    dance_time: data.dance_time,
                    ..Default::default()
                },
            ),
            ObjectiveType::FinishXPlaylist(data) => {
                json_types::ObjectiveDescriptor::FinishXPlaylist(
                    json_types::ObjectiveDescriptorFinishXPlaylist {
                        description: value.description,
                        description_raw: value.description_raw,
                        components: data.components.into_iter().map(Component::into).collect(),
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        playlists_play_count: data.playlists_play_count,
                        ..Default::default()
                    },
                )
            }
            ObjectiveType::GatherXStars(data) => json_types::ObjectiveDescriptor::GatherXStars(
                json_types::ObjectiveDescriptorGatherXStars {
                    description: value.description,
                    description_raw: value.description_raw,
                    components: data.components.into_iter().map(Component::into).collect(),
                    is_static: value.is_static,
                    exclude_from_upload: value.exclude_from_upload,
                    stars_count: data.stars_count,
                    ..Default::default()
                },
            ),
            ObjectiveType::PlayDailyQuestsForXDays(data) => {
                json_types::ObjectiveDescriptor::PlayDailyQuestsForXDays(
                    json_types::ObjectiveDescriptorPlayDailyQuestsForXDays {
                        description: value.description,
                        description_raw: value.description_raw,
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        consecutive_days: data.consecutive_days,
                        ..Default::default()
                    },
                )
            }
            ObjectiveType::PlayGachaXTimes(data) => {
                json_types::ObjectiveDescriptor::PlayGachaXTimes(
                    json_types::ObjectiveDescriptorPlayGachaXTimes {
                        description: value.description,
                        description_raw: value.description_raw,
                        components: data.components.into_iter().map(Component::into).collect(),
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        gacha_plays_count: data.gacha_plays_count,
                        unlock_all_acceptable_gacha_items: data.unlock_all_acceptable_gacha_items,
                        ..Default::default()
                    },
                )
            }
            ObjectiveType::PlayXMaps(data) => json_types::ObjectiveDescriptor::PlayXMaps(
                json_types::ObjectiveDescriptorPlayXMaps {
                    description: value.description,
                    description_raw: value.description_raw,
                    components: data.components.into_iter().map(Component::into).collect(),
                    is_static: value.is_static,
                    exclude_from_upload: value.exclude_from_upload,
                    maps_count: data.maps_count,
                    ..Default::default()
                },
            ),
            ObjectiveType::ReachRankX(data) => json_types::ObjectiveDescriptor::ReachRankX(
                json_types::ObjectiveDescriptorReachRankX {
                    description: value.description,
                    description_raw: value.description_raw,
                    is_static: value.is_static,
                    exclude_from_upload: value.exclude_from_upload,
                    rank_to_reach: data.rank_to_reach,
                    ..Default::default()
                },
            ),
            ObjectiveType::UnlockXPortraitBorders(data) => {
                json_types::ObjectiveDescriptor::UnlockXPortraitBorders(
                    json_types::ObjectiveDescriptorUnlockXPortraitBorders {
                        description: value.description,
                        description_raw: value.description_raw,
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        portrait_border_count: data.portrait_border_count,
                        ..Default::default()
                    },
                )
            }
            _ => json_types::ObjectiveDescriptor::SwitchSweatMode(
                // Convert all objectives that are impossible to do with the mod to sweat mode
                json_types::ObjectiveDescriptorBase {
                    description: value.description,
                    description_raw: value.description_raw,
                    is_static: value.is_static,
                    exclude_from_upload: value.exclude_from_upload,
                    ..Default::default()
                },
            ),
        }
    }
}

/// The thing a user needs to do
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum ObjectiveType<'a> {
    /// Burn X calories
    AccumulateXCal(AccumulateXCal<'a>),
    /// Do X moves
    AccumulateXMoves(AccumulateXMoves<'a>),
    /// Active coop mode
    ActivateCoopMode,
    /// Add X song to a playlist
    AddXSongsToAPlaylist(AddXSongsToAPlaylist<'a>),
    /// Beat a World Dance Floor boss
    BeatWDFBoss,
    /// Change a customisation item X times
    ChangeCustoItemXTimes(ChangeCustoItemXTimes<'a>),
    /// Complete X quests
    CompleteXQuests(CompleteXQuests),
    /// Dance for X seconds
    DanceXSeconds(DanceXSeconds),
    /// Finish X playlists
    FinishXPlaylist(FinishXPlaylist<'a>),
    /// Gather X stars
    GatherXStars(GatherXStars<'a>),
    /// Link a Uplay account
    LinkedToUplay,
    /// Open the anthology menu (JD2020)
    OpenAnthologyMode,
    /// Open the postcards menu
    OpenPostcardsGallery,
    /// Open the sticker album
    OpenStickerAlbum,
    /// Open the video gallery
    OpenVideoGallery,
    /// Play daily quests for X days
    PlayDailyQuestsForXDays(PlayDailyQuestsForXDays),
    /// Play the gacha machine X times
    PlayGachaXTimes(PlayGachaXTimes<'a>),
    /// Have a savefile with a previous version of Just Dance
    PlayPreviousJD,
    /// Play a World Dance Floor tournament
    PlayWDFTournament(PlayWDFTournament),
    /// Play X maps
    PlayXMaps(PlayXMaps<'a>),
    /// Play X World Dance Floor tournament rounds
    PlayXWDFTournamentRounds(PlayXWDFTournamentRounds<'a>),
    /// Reach Rank X
    ReachRankX(ReachRankX),
    /// Renew the Just Dance Unlimited Subscription
    RenewJDUSub,
    /// Switch to sweat mode (calorie mode)
    SwitchSweatMode,
    /// Get Alias Pack 1 on Uplay
    UnlockUplayRewardAliasPack1,
    /// Get Alias Pack 2 on Uplay
    UnlockUplayRewardAliasPack2,
    /// Unlock X portrait borders (skins)
    UnlockXPortraitBorders(UnlockXPortraitBorders),
    /// Unlock X stickers
    UnlockXStickers(UnlockXStickers<'a>),
    /// Win a World Dance Floor team battle
    WinWDFTeamBattle,
}

impl<'a> ObjectiveType<'a> {
    /// Convert from the UbiArt representation
    pub fn from_descriptor(
        descriptor: &json_types::ObjectiveDescriptor<'a>,
        locale_id_map: &LocaleIdMap,
    ) -> Result<Self, Error> {
        match descriptor {
            json_types::ObjectiveDescriptor::AccumulateXCal(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::AccumulateXCal(AccumulateXCal {
                    calories_amount: data.calories_amount,
                    in_one_session: data.in_one_session,
                    components,
                }))
            }
            json_types::ObjectiveDescriptor::AccumulateXMoves(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                let mut categories_to_count = Vec::with_capacity(data.categories_to_count.len());
                for category in &data.categories_to_count {
                    categories_to_count.push(category.try_into()?);
                }
                Ok(Self::AccumulateXMoves(AccumulateXMoves {
                    moves_count: data.moves_count,
                    categories_to_count,
                    components,
                }))
            }
            json_types::ObjectiveDescriptor::ActivateCoopMode(_) => Ok(Self::ActivateCoopMode),
            json_types::ObjectiveDescriptor::AddXSongsToAPlaylist(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::AddXSongsToAPlaylist(AddXSongsToAPlaylist {
                    songs_added_count: data.songs_added_count,
                    components,
                }))
            }
            json_types::ObjectiveDescriptor::BeatWDFBoss(_) => Ok(Self::BeatWDFBoss),
            json_types::ObjectiveDescriptor::ChangeCustoItemXTimes(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::ChangeCustoItemXTimes(ChangeCustoItemXTimes {
                    custo_item_changes_count: data.custo_item_changes_count,
                    components,
                }))
            }
            json_types::ObjectiveDescriptor::CompleteXQuests(data) => {
                Ok(Self::CompleteXQuests(CompleteXQuests {
                    quests_count: data.quests_count,
                }))
            }
            json_types::ObjectiveDescriptor::DanceXSeconds(data) => {
                Ok(Self::DanceXSeconds(DanceXSeconds {
                    dance_time: data.dance_time,
                }))
            }
            json_types::ObjectiveDescriptor::FinishXPlaylist(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::FinishXPlaylist(FinishXPlaylist {
                    playlists_play_count: data.playlists_play_count,
                    components,
                }))
            }
            json_types::ObjectiveDescriptor::GatherXStars(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::GatherXStars(GatherXStars {
                    stars_count: data.stars_count,
                    components,
                }))
            }
            json_types::ObjectiveDescriptor::LinkedToUplay(_) => Ok(Self::LinkedToUplay),
            json_types::ObjectiveDescriptor::OpenAnthologyMode(_) => Ok(Self::OpenAnthologyMode),
            json_types::ObjectiveDescriptor::OpenPostcardsGallery(_) => {
                Ok(Self::OpenPostcardsGallery)
            }
            json_types::ObjectiveDescriptor::OpenStickerAlbum(_) => Ok(Self::OpenStickerAlbum),
            json_types::ObjectiveDescriptor::OpenVideoGallery(_) => Ok(Self::OpenVideoGallery),
            json_types::ObjectiveDescriptor::PlayDailyQuestsForXDays(data) => {
                Ok(Self::PlayDailyQuestsForXDays(PlayDailyQuestsForXDays {
                    consecutive_days: data.consecutive_days,
                }))
            }
            json_types::ObjectiveDescriptor::PlayGachaXTimes(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::PlayGachaXTimes(PlayGachaXTimes {
                    gacha_plays_count: data.gacha_plays_count,
                    unlock_all_acceptable_gacha_items: data.unlock_all_acceptable_gacha_items,
                    components,
                }))
            }
            json_types::ObjectiveDescriptor::PlayPreviousJD(_) => Ok(Self::PlayPreviousJD),
            json_types::ObjectiveDescriptor::PlayWDFTournament(data) => {
                Ok(Self::PlayWDFTournament(PlayWDFTournament {
                    tournament_count: data.tournament_count.unwrap_or(1),
                }))
            }
            json_types::ObjectiveDescriptor::PlayXMaps(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::PlayXMaps(PlayXMaps {
                    maps_count: data.maps_count,
                    components,
                }))
            }
            json_types::ObjectiveDescriptor::PlayXWDFTournamentRounds(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::PlayXWDFTournamentRounds(PlayXWDFTournamentRounds {
                    rounds_count: data.rounds_count,
                    components,
                }))
            }
            json_types::ObjectiveDescriptor::ReachRankX(data) => Ok(Self::ReachRankX(ReachRankX {
                rank_to_reach: data.rank_to_reach,
            })),
            json_types::ObjectiveDescriptor::RenewJDUSub(_) => Ok(Self::RenewJDUSub),
            json_types::ObjectiveDescriptor::SwitchSweatMode(_) => Ok(Self::SwitchSweatMode),
            json_types::ObjectiveDescriptor::UnlockUplayRewardAliasPack1(_) => {
                Ok(Self::UnlockUplayRewardAliasPack1)
            }
            json_types::ObjectiveDescriptor::UnlockUplayRewardAliasPack2(_) => {
                Ok(Self::UnlockUplayRewardAliasPack2)
            }
            json_types::ObjectiveDescriptor::UnlockXPortraitBorders(data) => {
                Ok(Self::UnlockXPortraitBorders(UnlockXPortraitBorders {
                    portrait_border_count: data.portrait_border_count,
                }))
            }
            json_types::ObjectiveDescriptor::UnlockXStickers(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::UnlockXStickers(UnlockXStickers {
                    all_stickers: data.all_stickers.unwrap_or_default(),
                    stickers_count: data.stickers_count,
                    components,
                }))
            }
            json_types::ObjectiveDescriptor::WinWDFTeamBattle(_) => Ok(Self::WinWDFTeamBattle),
        }
    }

    /// Convert from the old UbiArt representation
    pub fn from_old_descriptor(
        descriptor: &json_types::ObjectiveDesc1819<'a>,
        unlimited_only: bool,
    ) -> (Self, bool) {
        let mut components = if unlimited_only {
            vec![Component {
                c_type: ComponentType::OnlyOnUnlimitedSongs,
                only_diff_values: false,
            }]
        } else {
            Vec::new()
        };
        match descriptor {
            // The comments matter!
            #[allow(clippy::match_same_arms)]
            json_types::ObjectiveDesc1819::Base(data) => match data.objective_type {
                0 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![
                                    LaunchContext::Family,
                                    LaunchContext::Quickplay,
                                    LaunchContext::WorldDanceFloor,
                                ],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    });
                    (
                        Self::GatherXStars(GatherXStars {
                            stars_count: data.minimum_value,
                            components,
                        }),
                        false,
                    )
                }
                3 => (
                    Self::AccumulateXCal(AccumulateXCal {
                        calories_amount: data.minimum_value,
                        in_one_session: false,
                        components,
                    }),
                    false,
                ),
                5 => {
                    // This supposed to be 'dance to x dance lab maps' but that doesn't exist anymore
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            ..Default::default()
                        }),
                        true,
                    )
                }
                6 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![
                                    LaunchContext::Family,
                                    LaunchContext::Quickplay,
                                    LaunchContext::WorldDanceFloor,
                                ],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    });
                    (
                        Self::AccumulateXMoves(AccumulateXMoves {
                            components,
                            moves_count: data.minimum_value,
                            categories_to_count: vec![MoveCategories::Gold],
                        }),
                        false,
                    )
                }
                7 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![
                                    LaunchContext::Family,
                                    LaunchContext::Quickplay,
                                    LaunchContext::WorldDanceFloor,
                                ],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    });
                    (
                        Self::AccumulateXMoves(AccumulateXMoves {
                            moves_count: data.minimum_value,
                            categories_to_count: vec![
                                MoveCategories::Super,
                                MoveCategories::Perfect,
                                MoveCategories::Gold,
                            ],
                            components,
                        }),
                        false,
                    )
                }
                12 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![
                                    LaunchContext::Family,
                                    LaunchContext::Quickplay,
                                    LaunchContext::WorldDanceFloor,
                                ],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapNameRequirement(MapNameRequirement::default()),
                        only_diff_values: true,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapScoreRequirement(MapScoreRequirement::default()),
                        only_diff_values: false,
                    });
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            components,
                        }),
                        false,
                    )
                }
                13 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![
                                    LaunchContext::Family,
                                    LaunchContext::Quickplay,
                                    LaunchContext::WorldDanceFloor,
                                ],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapScoreRequirement(MapScoreRequirement::default()),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapCoachCountRequirement(MapCoachCountRequirement {
                            acceptable_coach_counts: vec![NumberOfCoaches::Solo],
                        }),
                        only_diff_values: false,
                    });
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            components,
                        }),
                        false,
                    )
                }
                14 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![
                                    LaunchContext::Family,
                                    LaunchContext::Quickplay,
                                    LaunchContext::WorldDanceFloor,
                                ],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapScoreRequirement(MapScoreRequirement::default()),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapCoachCountRequirement(MapCoachCountRequirement {
                            acceptable_coach_counts: vec![NumberOfCoaches::Duo],
                        }),
                        only_diff_values: false,
                    });
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            components,
                        }),
                        false,
                    )
                }
                15 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![
                                    LaunchContext::Family,
                                    LaunchContext::Quickplay,
                                    LaunchContext::WorldDanceFloor,
                                ],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapScoreRequirement(MapScoreRequirement::default()),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapCoachCountRequirement(MapCoachCountRequirement {
                            acceptable_coach_counts: vec![NumberOfCoaches::Trio],
                        }),
                        only_diff_values: false,
                    });
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            components,
                        }),
                        false,
                    )
                }
                16 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![
                                    LaunchContext::Family,
                                    LaunchContext::Quickplay,
                                    LaunchContext::WorldDanceFloor,
                                ],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapScoreRequirement(MapScoreRequirement::default()),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapCoachCountRequirement(MapCoachCountRequirement {
                            acceptable_coach_counts: vec![NumberOfCoaches::Quarto],
                        }),
                        only_diff_values: false,
                    });
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            components,
                        }),
                        false,
                    )
                }
                17 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![
                                    LaunchContext::Family,
                                    LaunchContext::Quickplay,
                                    LaunchContext::WorldDanceFloor,
                                ],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapScoreRequirement(MapScoreRequirement::default()),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapRequireAllMoves(MapRequireAllMoves {
                            acceptable_categories: vec![MoveCategories::Gold],
                        }),
                        only_diff_values: false,
                    });
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            components,
                        }),
                        false,
                    )
                }
                30 => {
                    // This supposed to be 'create a playlist' but that doesn't exist anymore
                    (
                        Self::AddXSongsToAPlaylist(AddXSongsToAPlaylist {
                            songs_added_count: data.minimum_value,
                            ..Default::default()
                        }),
                        true,
                    )
                }
                31 => (
                    Self::FinishXPlaylist(FinishXPlaylist {
                        playlists_play_count: data.minimum_value,
                        ..Default::default()
                    }),
                    false,
                ),
                32 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_subcontexts: vec![LaunchSubcontext::Search],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapScoreRequirement(MapScoreRequirement::default()),
                        only_diff_values: false,
                    });
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            components,
                        }),
                        true,
                    )
                }
                34 => (
                    Self::AddXSongsToAPlaylist(AddXSongsToAPlaylist {
                        songs_added_count: data.minimum_value,
                        ..Default::default()
                    }),
                    true,
                ),
                35 => {
                    // This supposed to be 'open your dancer card' but that doesn't exist anymore
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            ..Default::default()
                        }),
                        true,
                    )
                }
                36 => {
                    let components = vec![Component {
                        c_type: ComponentType::CustoItemTypeRequirement(CustoItemTypeRequirement {
                            acceptable_custo_item_types: vec![CustomisableItemType::Avatar],
                        }),
                        only_diff_values: false,
                    }];
                    (
                        Self::ChangeCustoItemXTimes(ChangeCustoItemXTimes {
                            custo_item_changes_count: data.minimum_value,
                            components,
                        }),
                        true,
                    )
                }
                37 => (
                    Self::PlayGachaXTimes(PlayGachaXTimes {
                        gacha_plays_count: data.minimum_value,
                        ..Default::default()
                    }),
                    true,
                ),
                39 => (Self::LinkedToUplay, true),
                41 => {
                    let components = vec![
                        Component {
                            c_type: ComponentType::PlaylistIdRequirement(PlaylistIdRequirement {
                                acceptable_playlist_ids: vec![Cow::Borrowed(
                                    "Favorite_Playlist_ID",
                                )],
                            }),
                            only_diff_values: false,
                        },
                        Component {
                            c_type: ComponentType::MapScoreRequirement(
                                MapScoreRequirement::default(),
                            ),
                            only_diff_values: false,
                        },
                    ];
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            components,
                        }),
                        true,
                    )
                }
                42 => (Self::SwitchSweatMode, true),
                43 => {
                    // This supposed to be 'use shuffle' but that doesn't exist anymore
                    let components = vec![Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![LaunchContext::Quickplay],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    }];
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            components,
                        }),
                        true,
                    )
                }
                44 => (Self::RenewJDUSub, false),
                45 => {
                    components.push(Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: Vec::new(),
                                acceptable_launch_subcontexts: vec![LaunchSubcontext::Home],
                            },
                        ),
                        only_diff_values: false,
                    });
                    components.push(Component {
                        c_type: ComponentType::MapScoreRequirement(MapScoreRequirement::default()),
                        only_diff_values: false,
                    });
                    (
                        Self::PlayXMaps(PlayXMaps {
                            maps_count: data.minimum_value,
                            components,
                        }),
                        true,
                    )
                }
                46 => {
                    components.push(Component {
                        c_type: ComponentType::CustoItemTypeRequirement(CustoItemTypeRequirement {
                            acceptable_custo_item_types: vec![CustomisableItemType::Alias],
                        }),
                        only_diff_values: false,
                    });
                    (
                        Self::ChangeCustoItemXTimes(ChangeCustoItemXTimes {
                            custo_item_changes_count: data.minimum_value,
                            components,
                        }),
                        true,
                    )
                }
                47 => {
                    // 47 is actually for the video gallery, but that doesn't exist anymore
                    (Self::OpenVideoGallery, true)
                }
                48 => (Self::UnlockUplayRewardAliasPack1, true),
                49 => (Self::UnlockUplayRewardAliasPack2, true),
                50 => (Self::PlayPreviousJD, true),
                51 => (
                    Self::PlayXMaps(PlayXMaps {
                        maps_count: 1,
                        ..Default::default()
                    }),
                    true,
                ),
                53 => (
                    Self::PlayWDFTournament(PlayWDFTournament {
                        tournament_count: data.minimum_value,
                    }),
                    true,
                ),
                55 => {
                    let components = vec![Component {
                        c_type: ComponentType::MapLaunchLocationRequirement(
                            MapLaunchLocationRequirement {
                                acceptable_launch_contexts: vec![
                                    LaunchContext::Family,
                                    LaunchContext::Quickplay,
                                    LaunchContext::WorldDanceFloor,
                                ],
                                ..Default::default()
                            },
                        ),
                        only_diff_values: false,
                    }];
                    (
                        Self::AccumulateXMoves(AccumulateXMoves {
                            moves_count: data.minimum_value,
                            categories_to_count: vec![
                                MoveCategories::Perfect,
                                MoveCategories::Gold,
                            ],
                            components,
                        }),
                        true,
                    )
                }
                56 => {
                    // 56 is actually for the artwork gallery, but that doesn't exist anymore
                    (Self::OpenStickerAlbum, true)
                }
                57 => {
                    let components = vec![Component {
                        c_type: ComponentType::GachaItemTypeRequirement(GachaItemTypeRequirement {
                            acceptable_gacha_item_types: vec![GachaItemType::Postcard],
                        }),
                        only_diff_values: false,
                    }];
                    (
                        Self::PlayGachaXTimes(PlayGachaXTimes {
                            gacha_plays_count: u32::MAX,
                            unlock_all_acceptable_gacha_items: true,
                            components,
                        }),
                        true,
                    )
                }
                x => {
                    println!("Unknown old objective!:");
                    println!("ObjectiveType: {x}");
                    (Self::SwitchSweatMode, true)
                }
            },
            json_types::ObjectiveDesc1819::MinStarsReachedSongCount(data) => {
                assert!(data.objective_type == 18, "Objective type is not 18");
                components.push(Component {
                    c_type: ComponentType::MapLaunchLocationRequirement(
                        MapLaunchLocationRequirement {
                            acceptable_launch_contexts: vec![
                                LaunchContext::Family,
                                LaunchContext::Quickplay,
                                LaunchContext::WorldDanceFloor,
                            ],
                            ..Default::default()
                        },
                    ),
                    only_diff_values: false,
                });
                components.push(Component {
                    c_type: ComponentType::MapScoreRequirement(MapScoreRequirement {
                        score: 6000,
                        ..Default::default()
                    }),
                    only_diff_values: false,
                });
                components.push(Component {
                    c_type: ComponentType::MapNameRequirement(MapNameRequirement::default()),
                    only_diff_values: true,
                });
                (
                    Self::PlayXMaps(PlayXMaps {
                        maps_count: data.minimum_value,
                        components,
                    }),
                    true,
                )
            }
            json_types::ObjectiveDesc1819::PlaySpecificMap(data) => {
                assert!(data.objective_type == 10, "Objective type is not 10!");
                let acceptable_map_names = if data.map_name.is_empty() {
                    Vec::new()
                } else {
                    vec![data.map_name.clone()]
                };
                components.push(Component {
                    c_type: ComponentType::MapLaunchLocationRequirement(
                        MapLaunchLocationRequirement {
                            acceptable_launch_contexts: vec![
                                LaunchContext::Family,
                                LaunchContext::Quickplay,
                                LaunchContext::WorldDanceFloor,
                            ],
                            ..Default::default()
                        },
                    ),
                    only_diff_values: false,
                });
                components.push(Component {
                    c_type: ComponentType::MapScoreRequirement(MapScoreRequirement::default()),
                    only_diff_values: false,
                });
                components.push(Component {
                    c_type: ComponentType::MapNameRequirement(MapNameRequirement {
                        acceptable_map_names,
                    }),
                    only_diff_values: false,
                });
                (
                    Self::PlayXMaps(PlayXMaps {
                        maps_count: data.minimum_value,
                        components,
                    }),
                    false,
                )
            }
            json_types::ObjectiveDesc1819::GatherStarsWDF(data) => {
                assert!(data.objective_type == 0, "Objective type is not 0!");
                components.push(Component {
                    c_type: ComponentType::MapLaunchLocationRequirement(
                        MapLaunchLocationRequirement {
                            acceptable_launch_contexts: vec![
                                LaunchContext::Family,
                                LaunchContext::Quickplay,
                                LaunchContext::WorldDanceFloor,
                            ],
                            ..Default::default()
                        },
                    ),
                    only_diff_values: false,
                });
                components.push(Component {
                    c_type: ComponentType::MapPlaymodeRequirement(MapPlaymodeRequirement {
                        wdf: true,
                        ..Default::default()
                    }),
                    only_diff_values: false,
                });
                (
                    Self::GatherXStars(GatherXStars {
                        stars_count: data.minimum_value,
                        components,
                    }),
                    false,
                )
            }
            json_types::ObjectiveDesc1819::SweatSongCount(data) => {
                assert!(data.objective_type == 11, "Objective type is not 11");
                components.push(Component {
                    c_type: ComponentType::MapLaunchLocationRequirement(
                        MapLaunchLocationRequirement {
                            acceptable_launch_contexts: vec![
                                LaunchContext::Family,
                                LaunchContext::Quickplay,
                                LaunchContext::WorldDanceFloor,
                            ],
                            ..Default::default()
                        },
                    ),
                    only_diff_values: false,
                });
                components.push(Component {
                    c_type: ComponentType::MapPlaymodeRequirement(MapPlaymodeRequirement {
                        sweat: true,
                        ..Default::default()
                    }),
                    only_diff_values: false,
                });
                (
                    Self::PlayXMaps(PlayXMaps {
                        maps_count: data.minimum_value,
                        components,
                    }),
                    false,
                )
            }
            json_types::ObjectiveDesc1819::WDFSongCount(data) => {
                assert!(data.objective_type == 11, "Objective type is not 11");
                components.push(Component {
                    c_type: ComponentType::MapLaunchLocationRequirement(
                        MapLaunchLocationRequirement {
                            acceptable_launch_contexts: vec![
                                LaunchContext::Family,
                                LaunchContext::Quickplay,
                                LaunchContext::WorldDanceFloor,
                            ],
                            ..Default::default()
                        },
                    ),
                    only_diff_values: false,
                });
                components.push(Component {
                    c_type: ComponentType::MapPlaymodeRequirement(MapPlaymodeRequirement {
                        wdf: true,
                        ..Default::default()
                    }),
                    only_diff_values: false,
                });
                (
                    Self::PlayXMaps(PlayXMaps {
                        maps_count: data.minimum_value,
                        components,
                    }),
                    false,
                )
            }
            json_types::ObjectiveDesc1819::RecommendSongCount(data) => {
                // Recommend songs don't exist anymore as objective type, so changed to quickplay songs
                assert!(data.objective_type == 19, "Objective type is not 19");
                let components = vec![Component {
                    c_type: ComponentType::MapLaunchLocationRequirement(
                        MapLaunchLocationRequirement {
                            acceptable_launch_contexts: vec![LaunchContext::Quickplay],
                            acceptable_launch_subcontexts: Vec::new(),
                        },
                    ),
                    only_diff_values: false,
                }];
                (
                    Self::PlayXMaps(PlayXMaps {
                        maps_count: data.minimum_value,
                        components,
                    }),
                    true,
                )
            }
            json_types::ObjectiveDesc1819::ClassicTournamentRank(data) => {
                // Classic Tournament doesn't exist anymore so map to regular rank
                assert!(data.objective_type == 54, "Objective type is not 54");
                (
                    Self::ReachRankX(ReachRankX {
                        rank_to_reach: data.minimum_value,
                    }),
                    true,
                )
            }
        }
    }
}

/// Grading of a move
#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MoveCategories {
    /// Ok
    Ok = 1,
    /// Good
    Good = 2,
    /// Super
    Super = 3,
    /// Perfect
    Perfect = 4,
    /// Gold move
    Gold = 5,
}

impl From<MoveCategories> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(value: MoveCategories) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for MoveCategories {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Ok),
            2 => Ok(Self::Good),
            3 => Ok(Self::Super),
            4 => Ok(Self::Perfect),
            5 => Ok(Self::Gold),
            _ => Err(anyhow!("Unknown value for MoveCategory: {}", value)),
        }
    }
}

impl TryFrom<&u8> for MoveCategories {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Ok),
            2 => Ok(Self::Good),
            3 => Ok(Self::Super),
            4 => Ok(Self::Perfect),
            5 => Ok(Self::Gold),
            _ => Err(anyhow!("Unknown value for MoveCategory: {}", value)),
        }
    }
}

/// Requirements for [`ObjectiveType::AccumulateXCal`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct AccumulateXCal<'a> {
    /// How many calories to burn
    pub calories_amount: u32,
    /// Should it be done in one session
    pub in_one_session: bool,
    /// Additional generic requirements
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::AccumulateXMoves`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct AccumulateXMoves<'a> {
    /// How many moves
    pub moves_count: u32,
    /// What grade should the moves be
    pub categories_to_count: Vec<MoveCategories>,
    /// Additional generic requirements
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::AddXSongsToAPlaylist`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct AddXSongsToAPlaylist<'a> {
    /// How many songs to add
    pub songs_added_count: u32,
    /// Additional generic requirements
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::ChangeCustoItemXTimes`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct ChangeCustoItemXTimes<'a> {
    /// How many items to change
    pub custo_item_changes_count: u32,
    /// Additional generic requirements
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::CompleteXQuests`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct CompleteXQuests {
    /// How many quests to complete
    pub quests_count: u32,
}

/// Requirements for [`ObjectiveType::DanceXSeconds`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct DanceXSeconds {
    /// How many seconds to dance
    pub dance_time: u32,
}

/// Requirements for [`ObjectiveType::FinishXPlaylist`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct FinishXPlaylist<'a> {
    /// How many playlists to finish
    pub playlists_play_count: u32,
    /// Additional generic requirements
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::GatherXStars`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct GatherXStars<'a> {
    /// How many stars to get
    pub stars_count: u32,
    /// Additional generic requirements
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::PlayDailyQuestsForXDays`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct PlayDailyQuestsForXDays {
    /// How many consecutive days
    pub consecutive_days: u32,
}

/// Requirements for [`ObjectiveType::PlayGachaXTimes`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct PlayGachaXTimes<'a> {
    /// How many times to play
    pub gacha_plays_count: u32,
    /// Unlock all gacha items (can be narrowed via [`Component`])
    pub unlock_all_acceptable_gacha_items: bool,
    /// Additional generic requirements
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::PlayWDFTournament`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct PlayWDFTournament {
    /// How many tournaments to play
    pub tournament_count: u32,
}

/// Requirements for [`ObjectiveType::PlayXMaps`]
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct PlayXMaps<'a> {
    /// How many maps to play
    pub maps_count: u32,
    /// Additional generic requirements
    pub components: Vec<Component<'a>>,
}

impl Default for PlayXMaps<'_> {
    fn default() -> Self {
        Self {
            maps_count: 1,
            components: Vec::default(),
        }
    }
}

/// Requirements for [`ObjectiveType::PlayXWDFTournamentRounds`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct PlayXWDFTournamentRounds<'a> {
    /// How many rounds to play
    pub rounds_count: u32,
    /// Additional generic requirements
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::ReachRankX`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct ReachRankX {
    /// Which rank to reach
    pub rank_to_reach: u32,
}

/// Requirements for [`ObjectiveType::UnlockXPortraitBorders`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct UnlockXPortraitBorders {
    /// How many portrait borders to unlock
    pub portrait_border_count: u32,
}

/// Requirements for [`ObjectiveType::UnlockXStickers`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct UnlockXStickers<'a> {
    /// Unlock all sticker
    pub all_stickers: bool,
    /// How many stickers to unlock
    pub stickers_count: u32,
    /// Additional generic requirements
    pub components: Vec<Component<'a>>,
}

/// A requirements that can be shared between various objective types
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Component<'a> {
    /// The actual requirements
    pub c_type: ComponentType<'a>,
    /// Should only different values be accepted
    pub only_diff_values: bool,
}

impl<'a> Component<'a> {
    /// Convert from UbiArt representation
    pub fn from_component(
        component: &json_types::ObjectiveDescriptorComponent<'a>,
        locale_id_map: &LocaleIdMap,
    ) -> Result<Self, Error> {
        let c_type = match component {
            json_types::ObjectiveDescriptorComponent::CustoItemTypeRequirement(data) => {
                let mut acceptable_custo_item_types =
                    Vec::with_capacity(data.acceptable_custo_item_types.len());
                for item in &data.acceptable_custo_item_types {
                    acceptable_custo_item_types.push(CustomisableItemType::try_from(*item)?);
                }
                ComponentType::CustoItemTypeRequirement(CustoItemTypeRequirement {
                    acceptable_custo_item_types,
                })
            }
            json_types::ObjectiveDescriptorComponent::GachaItemTypeRequirement(data) => {
                let mut acceptable_gacha_item_types =
                    Vec::with_capacity(data.acceptable_gacha_item_types.len());
                for item in &data.acceptable_gacha_item_types {
                    acceptable_gacha_item_types.push(GachaItemType::try_from(*item)?);
                }
                ComponentType::GachaItemTypeRequirement(GachaItemTypeRequirement {
                    acceptable_gacha_item_types,
                })
            }
            json_types::ObjectiveDescriptorComponent::MapCoachCountRequirement(data) => {
                let mut acceptable_coach_counts =
                    Vec::with_capacity(data.acceptable_coach_counts.len());
                for item in &data.acceptable_coach_counts {
                    acceptable_coach_counts.push(NumberOfCoaches::try_from(*item)?);
                }
                ComponentType::MapCoachCountRequirement(MapCoachCountRequirement {
                    acceptable_coach_counts,
                })
            }
            json_types::ObjectiveDescriptorComponent::MapLaunchLocationRequirement(data) => {
                let mut acceptable_launch_contexts =
                    Vec::with_capacity(data.acceptable_launch_contexts.len());
                for item in &data.acceptable_launch_contexts {
                    acceptable_launch_contexts.push(LaunchContext::try_from(item.as_ref())?);
                }
                let mut acceptable_launch_subcontexts =
                    Vec::with_capacity(data.acceptable_launch_subcontexts.len());
                for item in &data.acceptable_launch_subcontexts {
                    acceptable_launch_subcontexts.push(LaunchSubcontext::try_from(item.as_ref())?);
                }
                ComponentType::MapLaunchLocationRequirement(MapLaunchLocationRequirement {
                    acceptable_launch_contexts,
                    acceptable_launch_subcontexts,
                })
            }
            json_types::ObjectiveDescriptorComponent::MapMovesRequirement(data) => {
                let mut acceptable_categories =
                    Vec::with_capacity(data.acceptable_categories.len());
                for cat in &data.acceptable_categories {
                    acceptable_categories.push(cat.try_into()?);
                }
                if data.all_map_moves_count {
                    test(&data.exact_moves_count, &u32::MAX)?;
                    test(&data.min_moves_count, &u32::MAX)?;
                    test(&data.max_moves_count, &u32::MAX)?;
                    test(&data.only_map_last_move, &false)?;
                    test(&data.moves_in_a_row, &false)?;
                    ComponentType::MapRequireAllMoves(MapRequireAllMoves {
                        acceptable_categories,
                    })
                } else if data.only_map_last_move {
                    assert!(
                        (data.exact_moves_count == 1 && data.min_moves_count == u32::MAX)
                            || (data.exact_moves_count == u32::MAX && data.min_moves_count == 1),
                        "Exact/Min moves count have unexpected value! {} {}",
                        data.exact_moves_count,
                        data.min_moves_count
                    );
                    test(&data.max_moves_count, &u32::MAX)?;
                    test(&data.all_map_moves_count, &false)?;
                    test(&data.moves_in_a_row, &false)?;
                    ComponentType::MapRequireLastMove(MapRequireLastMove {
                        acceptable_categories,
                    })
                } else if data.moves_in_a_row {
                    test(&data.exact_moves_count, &u32::MAX)?;
                    test(&data.max_moves_count, &u32::MAX)?;
                    test(&data.only_map_last_move, &false)?;
                    test(&data.all_map_moves_count, &false)?;
                    ComponentType::MapRequireXMovesInARow(MapRequireXMovesInARow {
                        min_moves_count: data.min_moves_count,
                        acceptable_categories,
                    })
                } else {
                    return Err(anyhow!("MapMovesRequirement options are all false!"));
                }
            }
            json_types::ObjectiveDescriptorComponent::MapNameRequirement(data) => {
                ComponentType::MapNameRequirement(MapNameRequirement {
                    acceptable_map_names: data.acceptable_map_names.clone(),
                })
            }
            json_types::ObjectiveDescriptorComponent::MapPlaymodeRequirement(data) => {
                ComponentType::MapPlaymodeRequirement(MapPlaymodeRequirement {
                    classic: data.classic,
                    coop: data.coop,
                    sweat: data.sweat,
                    playlist: data.playlist,
                    wdf: data.wdf,
                    kids: data.kids,
                    anthology: data.anthology.unwrap_or_default(),
                })
            }
            json_types::ObjectiveDescriptorComponent::MapScoreRequirement(data) => {
                ComponentType::MapScoreRequirement(MapScoreRequirement {
                    score: data.score,
                    better_than_dancer_of_the_week: data.better_than_dancer_of_the_week,
                })
            }
            json_types::ObjectiveDescriptorComponent::MapTagsRequirement(data) => {
                let mut acceptable_map_tags = Vec::with_capacity(data.acceptable_map_tags.len());
                for item in &data.acceptable_map_tags {
                    acceptable_map_tags.push(Tag::try_from(item.as_ref())?);
                }
                let mut unacceptable_map_tags =
                    Vec::with_capacity(data.unacceptable_map_tags.len());
                for item in &data.unacceptable_map_tags {
                    unacceptable_map_tags.push(Tag::try_from(item.as_ref())?);
                }
                ComponentType::MapTagsRequirement(MapTagsRequirement {
                    acceptable_map_tags,
                    unacceptable_map_tags,
                })
            }
            json_types::ObjectiveDescriptorComponent::OnlyOnline(_) => ComponentType::OnlyOnline,
            json_types::ObjectiveDescriptorComponent::OnlyOnUnlimitedSongs(_) => {
                ComponentType::OnlyOnUnlimitedSongs
            }
            json_types::ObjectiveDescriptorComponent::PlaylistIdRequirement(data) => {
                ComponentType::PlaylistIdRequirement(PlaylistIdRequirement {
                    acceptable_playlist_ids: data.acceptable_playlist_ids.clone(),
                })
            }
            json_types::ObjectiveDescriptorComponent::ScoringModeRequirement(data) => {
                let mut acceptable_scoring_modes =
                    Vec::with_capacity(data.acceptable_scoring_modes.len());
                for item in &data.acceptable_scoring_modes {
                    acceptable_scoring_modes.push(ScoringMode::try_from(*item)?);
                }
                ComponentType::ScoringModeRequirement(ScoringModeRequirement {
                    acceptable_scoring_modes,
                })
            }
            json_types::ObjectiveDescriptorComponent::SearchLabelsRequirement(data) => {
                ComponentType::SearchLabelsRequirement(SearchLabelsRequirement {
                    acceptable_labels: data
                        .acceptable_label_loc_ids
                        .iter()
                        .map(|id| locale_id_map.get(*id).unwrap_or_default())
                        .collect(),
                    forbidden_labels: data
                        .forbidden_label_loc_ids
                        .iter()
                        .map(|id| locale_id_map.get(*id).unwrap_or_default())
                        .collect(),
                })
            }
            json_types::ObjectiveDescriptorComponent::StickerIdRequirement(data) => {
                ComponentType::StickerIdRequirement(StickerIdRequirement {
                    acceptable_sticker_ids: data.acceptable_sticker_ids.clone(),
                })
            }
        };
        Ok(Self {
            c_type,
            only_diff_values: component.only_diff_values(),
        })
    }
}

impl<'a> From<Component<'a>> for json_types::ObjectiveDescriptorComponent<'a> {
    fn from(value: Component<'a>) -> Self {
        match value.c_type {
            ComponentType::CustoItemTypeRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::CustoItemTypeRequirement(
                    json_types::ObjectiveDescriptorComponentCustoItemTypeRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_custo_item_types: data
                            .acceptable_custo_item_types
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect(),
                    },
                )
            }
            ComponentType::GachaItemTypeRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::GachaItemTypeRequirement(
                    json_types::ObjectiveDescriptorComponentGachaItemTypeRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_gacha_item_types: data
                            .acceptable_gacha_item_types
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect(),
                    },
                )
            }
            ComponentType::MapCoachCountRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::MapCoachCountRequirement(
                    json_types::ObjectiveDescriptorComponentMapCoachCountRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_coach_counts: data
                            .acceptable_coach_counts
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect(),
                    },
                )
            }
            ComponentType::MapLaunchLocationRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::MapLaunchLocationRequirement(
                    json_types::ObjectiveDescriptorComponentMapLaunchLocationRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_launch_contexts: data
                            .acceptable_launch_contexts
                            .iter()
                            .copied()
                            .map(LaunchContext::into)
                            .collect(),
                        acceptable_launch_subcontexts: data
                            .acceptable_launch_subcontexts
                            .iter()
                            .copied()
                            .map(LaunchSubcontext::into)
                            .collect(),
                    },
                )
            }
            ComponentType::MapNameRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::MapNameRequirement(
                    json_types::ObjectiveDescriptorComponentMapNameRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_map_names: data.acceptable_map_names,
                    },
                )
            }
            ComponentType::MapPlaymodeRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::MapPlaymodeRequirement(
                    json_types::ObjectiveDescriptorComponentMapPlaymodeRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        classic: data.classic,
                        coop: data.coop,
                        sweat: data.sweat,
                        playlist: data.playlist,
                        wdf: data.wdf,
                        kids: data.kids,
                        anthology: Some(data.anthology),
                    },
                )
            }
            ComponentType::MapScoreRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::MapScoreRequirement(
                    json_types::ObjectiveDescriptorComponentMapScoreRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        score: data.score,
                        better_than_dancer_of_the_week: data.better_than_dancer_of_the_week,
                    },
                )
            }
            ComponentType::MapTagsRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::MapTagsRequirement(
                    json_types::ObjectiveDescriptorComponentMapTagsRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_map_tags: data
                            .acceptable_map_tags
                            .iter()
                            .copied()
                            .map(Tag::to_cow)
                            .collect(),
                        unacceptable_map_tags: data
                            .unacceptable_map_tags
                            .iter()
                            .copied()
                            .map(Tag::to_cow)
                            .collect(),
                    },
                )
            }
            ComponentType::OnlyOnline => json_types::ObjectiveDescriptorComponent::OnlyOnline(
                json_types::ObjectiveDescriptorComponentBase {
                    class: None,
                    only_diff_values: value.only_diff_values,
                },
            ),
            ComponentType::OnlyOnUnlimitedSongs => {
                json_types::ObjectiveDescriptorComponent::OnlyOnUnlimitedSongs(
                    json_types::ObjectiveDescriptorComponentBase {
                        class: None,
                        only_diff_values: value.only_diff_values,
                    },
                )
            }
            ComponentType::PlaylistIdRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::PlaylistIdRequirement(
                    json_types::ObjectiveDescriptorComponentPlaylistIdRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_playlist_ids: data.acceptable_playlist_ids,
                    },
                )
            }
            ComponentType::ScoringModeRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::ScoringModeRequirement(
                    json_types::ObjectiveDescriptorComponentScoringModeRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_scoring_modes: data
                            .acceptable_scoring_modes
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect(),
                    },
                )
            }
            ComponentType::SearchLabelsRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::SearchLabelsRequirement(
                    json_types::ObjectiveDescriptorComponentSearchLabelsRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_label_loc_ids: data.acceptable_labels,
                        forbidden_label_loc_ids: data.forbidden_labels,
                    },
                )
            }
            ComponentType::StickerIdRequirement(data) => {
                json_types::ObjectiveDescriptorComponent::StickerIdRequirement(
                    json_types::ObjectiveDescriptorComponentStickerIdRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_sticker_ids: data.acceptable_sticker_ids,
                    },
                )
            }
            ComponentType::MapRequireAllMoves(data) => {
                json_types::ObjectiveDescriptorComponent::MapMovesRequirement(
                    json_types::ObjectiveDescriptorComponentMapMovesRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        exact_moves_count: u32::MAX,
                        min_moves_count: u32::MAX,
                        max_moves_count: u32::MAX,
                        all_map_moves_count: true,
                        only_map_last_move: false,
                        moves_in_a_row: false,
                        acceptable_categories: data
                            .acceptable_categories
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect(),
                    },
                )
            }
            ComponentType::MapRequireLastMove(data) => {
                json_types::ObjectiveDescriptorComponent::MapMovesRequirement(
                    json_types::ObjectiveDescriptorComponentMapMovesRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        exact_moves_count: 1,
                        min_moves_count: u32::MAX,
                        max_moves_count: u32::MAX,
                        all_map_moves_count: false,
                        only_map_last_move: true,
                        moves_in_a_row: false,
                        acceptable_categories: data
                            .acceptable_categories
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect(),
                    },
                )
            }
            ComponentType::MapRequireXMovesInARow(data) => {
                json_types::ObjectiveDescriptorComponent::MapMovesRequirement(
                    json_types::ObjectiveDescriptorComponentMapMovesRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        exact_moves_count: u32::MAX,
                        min_moves_count: data.min_moves_count,
                        max_moves_count: u32::MAX,
                        all_map_moves_count: false,
                        only_map_last_move: false,
                        moves_in_a_row: true,
                        acceptable_categories: data
                            .acceptable_categories
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect(),
                    },
                )
            }
        }
    }
}

/// Requirement for a objective
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum ComponentType<'a> {
    /// Require a certain customisable item type for customisation objectives
    CustoItemTypeRequirement(CustoItemTypeRequirement),
    /// Require a certain item type for gacha machine objectives
    GachaItemTypeRequirement(GachaItemTypeRequirement),
    /// Require the map to have X coaches
    MapCoachCountRequirement(MapCoachCountRequirement),
    /// Require the map to be launched from a certain location
    MapLaunchLocationRequirement(MapLaunchLocationRequirement),
    /// Require all moves to be a certain grade
    MapRequireAllMoves(MapRequireAllMoves),
    /// Require the last move to be a certain grade
    MapRequireLastMove(MapRequireLastMove),
    /// Require X moves in a row of a certain grade
    MapRequireXMovesInARow(MapRequireXMovesInARow),
    /// Require certain maps
    MapNameRequirement(MapNameRequirement<'a>),
    /// Require a playmode
    MapPlaymodeRequirement(MapPlaymodeRequirement),
    /// Require a minium score
    MapScoreRequirement(MapScoreRequirement),
    /// Require certain tags
    MapTagsRequirement(MapTagsRequirement),
    /// Require to be online
    OnlyOnline,
    /// Require Unlimited songs
    OnlyOnUnlimitedSongs,
    /// Require certain playlists
    PlaylistIdRequirement(PlaylistIdRequirement<'a>),
    /// Require certain scoring modes
    ScoringModeRequirement(ScoringModeRequirement),
    /// Require certain search labels
    SearchLabelsRequirement(SearchLabelsRequirement),
    /// Require certain sticker ids
    StickerIdRequirement(StickerIdRequirement),
}

/// Customisable items
#[repr(u8)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CustomisableItemType {
    /// Avatars
    Avatar = 0,
    /// Aliases
    Alias = 1,
    /// Portrait borders/Skin
    Skin = 2,
}

impl From<CustomisableItemType> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(value: CustomisableItemType) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for CustomisableItemType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Avatar),
            1 => Ok(Self::Alias),
            2 => Ok(Self::Skin),
            _ => Err(anyhow!("Unknown ItemType: {value}")),
        }
    }
}

/// Require certain customisation item types
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct CustoItemTypeRequirement {
    /// Accepatble customisation item types
    pub acceptable_custo_item_types: Vec<CustomisableItemType>,
}

/// Item types that can be won in the gacha machine
#[repr(u8)]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GachaItemType {
    /// Postcard
    #[default]
    Postcard = 0,
}

impl From<GachaItemType> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(value: GachaItemType) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for GachaItemType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Postcard),
            _ => Err(anyhow!("Unknown GachaItemType: {value}")),
        }
    }
}

/// Require certain item types
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct GachaItemTypeRequirement {
    /// Acceptable item types
    pub acceptable_gacha_item_types: Vec<GachaItemType>,
}

/// Require X coaches
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MapCoachCountRequirement {
    /// Acceptable coach amounts
    pub acceptable_coach_counts: Vec<NumberOfCoaches>,
}

/// Where is the map launched from
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LaunchContext {
    /// Normal?
    Family,
    /// Quickplay
    Quickplay,
    /// World Dance Floor
    WorldDanceFloor,
    /// Kids mode
    Kids,
    /// Anthology
    Anthology,
}

impl TryFrom<&str> for LaunchContext {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "context_family" => Ok(Self::Family),
            "context_quickplay" => Ok(Self::Quickplay),
            "context_wdf" => Ok(Self::WorldDanceFloor),
            "context_kids" => Ok(Self::Kids),
            "context_anthology" => Ok(Self::Anthology),
            _ => Err(anyhow!("Unknown launch context: {value}")),
        }
    }
}

impl From<LaunchContext> for Cow<'static, str> {
    fn from(value: LaunchContext) -> Self {
        match value {
            LaunchContext::Family => Cow::Borrowed("context_family"),
            LaunchContext::Quickplay => Cow::Borrowed("context_quickplay"),
            LaunchContext::WorldDanceFloor => Cow::Borrowed("context_wdf"),
            LaunchContext::Kids => Cow::Borrowed("context_kids"),
            LaunchContext::Anthology => Cow::Borrowed("context_anthology"),
        }
    }
}

/// Where is the map launched from
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LaunchSubcontext {
    /// Search
    Search,
    /// Recommendations
    Home,
}

impl TryFrom<&str> for LaunchSubcontext {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "search" => Ok(Self::Search),
            "home" => Ok(Self::Home),
            _ => Err(anyhow!("Unknown launch subcontext: {value}")),
        }
    }
}

impl From<LaunchSubcontext> for Cow<'static, str> {
    fn from(value: LaunchSubcontext) -> Self {
        match value {
            LaunchSubcontext::Search => Cow::Borrowed("search"),
            LaunchSubcontext::Home => Cow::Borrowed("home"),
        }
    }
}

/// Require the map to be launched in a certain way
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MapLaunchLocationRequirement {
    /// Normal ways
    pub acceptable_launch_contexts: Vec<LaunchContext>,
    /// Search and recommendations
    pub acceptable_launch_subcontexts: Vec<LaunchSubcontext>,
}

/// Require all moves to be a specific grade
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MapRequireAllMoves {
    /// The grade
    pub acceptable_categories: Vec<MoveCategories>,
}

/// Require last move to be a specific grade
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MapRequireLastMove {
    /// The grades
    pub acceptable_categories: Vec<MoveCategories>,
}

/// Require X moves of certain grade
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MapRequireXMovesInARow {
    /// Minimum amount of moves in a row
    pub min_moves_count: u32,
    /// All these grades
    pub acceptable_categories: Vec<MoveCategories>,
}

/// Require map codename
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MapNameRequirement<'a> {
    /// Map codenames
    pub acceptable_map_names: Vec<Cow<'a, str>>,
}

/// Require specific playmodes
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MapPlaymodeRequirement {
    /// Normal mode
    pub classic: bool,
    /// Coop mode
    pub coop: bool,
    /// Sweat mode
    pub sweat: bool,
    /// Playlist
    pub playlist: bool,
    /// World Dance Floor
    pub wdf: bool,
    /// Kids mode
    pub kids: bool,
    /// Anthology mode
    pub anthology: bool,
}

/// Require a certain score
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MapScoreRequirement {
    /// Minimum score
    pub score: u32,
    /// Better dan dancer of the week
    pub better_than_dancer_of_the_week: bool,
}

impl Default for MapScoreRequirement {
    fn default() -> Self {
        Self {
            score: 1000,
            better_than_dancer_of_the_week: false,
        }
    }
}

/// Require specific tags
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MapTagsRequirement {
    /// Acceptable tags
    pub acceptable_map_tags: Vec<Tag>,
    /// Forbidden tags
    pub unacceptable_map_tags: Vec<Tag>,
}

/// Require a specifc playlist
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct PlaylistIdRequirement<'a> {
    /// Acceptable playlists
    pub acceptable_playlist_ids: Vec<Cow<'a, str>>,
}

/// A scoring mode like controller or Kinect
#[repr(u8)]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ScoringMode {
    /// Using a phone or controller
    #[default]
    PhoneDevice = 2,
}

impl From<ScoringMode> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(value: ScoringMode) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for ScoringMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::PhoneDevice),
            _ => Err(anyhow!("Unknown ScoringMode {value}")),
        }
    }
}

/// Require a specific scoring mode (Kinect/Controller)
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct ScoringModeRequirement {
    /// The scoring modes
    pub acceptable_scoring_modes: Vec<ScoringMode>,
}

/// Require specific search labels
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct SearchLabelsRequirement {
    /// Required labels
    pub acceptable_labels: Vec<LocaleId>,
    /// Forbidden labels
    pub forbidden_labels: Vec<LocaleId>,
}

/// Require specific sticker ids
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct StickerIdRequirement {
    /// The sticker ids
    pub acceptable_sticker_ids: Vec<u32>,
}
