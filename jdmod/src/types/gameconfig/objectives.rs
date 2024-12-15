//! # Objectives
//! Types for objectives (as used by scheduled quests)
use std::{
    collections::{
        hash_map::{DefaultHasher, Entry},
        HashMap,
    },
    hash::{Hash, Hasher},
};

use anyhow::{anyhow, Error};
use hipstr::HipStr;
use ownable::IntoOwned;
use serde::{Deserialize, Serialize};
use test_eq::{test_eq, test_or};
use ubiart_toolkit::{cooked, shared_json_types};

use crate::types::{
    localisation::{LocaleId, LocaleIdMap},
    song::NumberOfCoaches,
};

// TODO: Replace with BiMap which allows mapping multiple strings to one objective
/// Represents all the objectives in the mod
///
/// Note: an objective can have multiple names but a name cannot refer to multiple objectives
#[derive(Debug, Clone, Default)]
pub struct Objectives<'a> {
    /// Mapping from a objective to a objective name
    pub objective_map: HashMap<Objective<'a>, HipStr<'a>>,
    /// Mapping from objective name to the objective
    pub name_map: HashMap<HipStr<'a>, Objective<'a>>,
}

/// Describes an objective
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, IntoOwned)]
pub struct Objective<'a> {
    /// What does a user need to do
    #[serde(borrow)]
    pub objective_type: ObjectiveType<'a>,
    /// Description of the objective
    pub description: LocaleId,
    /// Description of the objective as a string (empty most of the time)
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
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
    pub fn add_objective(&mut self, objective: Objective<'a>) -> HipStr<'a> {
        if let Some(name) = self.objective_map.get(&objective) {
            name.clone()
        } else {
            let name = objective.generate_name();
            self.name_map.insert(name.clone(), objective.clone());
            self.objective_map.insert(objective, name.clone());
            name
        }
    }

    /// Add an objective with a name
    ///
    /// # Errors
    /// Will return an error if an objective already exists for `name` and it does not match `objective`
    pub fn add_objective_with_name(
        &mut self,
        objective: Objective<'a>,
        name: HipStr<'a>,
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
    #[must_use]
    pub fn generate_name(&self) -> HipStr<'static> {
        // TODO: Generate a nicer name, preferably matching existing naming convention
        let sd = if self.is_static { "Static" } else { "Dynamic" };
        let mut hasher = DefaultHasher::new();
        self.objective_type.hash(&mut hasher);
        self.description.hash(&mut hasher);
        self.exclude_from_upload.hash(&mut hasher);
        HipStr::from(format!("{sd}_{:x}", hasher.finish()))
    }
}

impl<'a> Objective<'a> {
    /// Convert from the UbiArt representation
    ///
    /// # Errors
    /// Will error if converting the [`ObjectiveType`] fails
    pub fn from_descriptor(
        descriptor: &cooked::isg::ObjectiveDescriptor<'a>,
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
    #[must_use]
    pub fn from_old_descriptor(
        descriptor: &shared_json_types::ObjectiveDesc<'a>,
        unlimited_only: bool,
        locale_id_map: &LocaleIdMap,
    ) -> Self {
        let (o_type, o_static) = ObjectiveType::from_old_descriptor(descriptor, unlimited_only);
        Self {
            objective_type: o_type,
            description: locale_id_map
                .get(descriptor.description())
                .unwrap_or_default(),
            description_raw: HipStr::borrowed(""),
            is_static: o_static,
            exclude_from_upload: true,
        }
    }
}

impl<'a> From<Objective<'a>> for cooked::isg::ObjectiveDescriptor<'a> {
    fn from(value: Objective<'a>) -> Self {
        match value.objective_type {
            ObjectiveType::AccumulateXCal(data) => {
                cooked::isg::ObjectiveDescriptor::AccumulateXCal(
                    cooked::isg::ObjectiveDescriptorAccumulateXCal {
                        description: value.description,
                        description_raw: value.description_raw,
                        components: data.components.into_iter().map(Component::into).collect(),
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        calories_amount: data.calories_amount,
                        in_one_session: data.in_one_session,
                        ..Default::default()
                    },
                )
            }
            ObjectiveType::AccumulateXMoves(data) => {
                cooked::isg::ObjectiveDescriptor::AccumulateXMoves(
                    cooked::isg::ObjectiveDescriptorAccumulateXMoves {
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
                cooked::isg::ObjectiveDescriptor::AddXSongsToAPlaylist(
                    cooked::isg::ObjectiveDescriptorAddXSongsToAPlaylist {
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
                cooked::isg::ObjectiveDescriptor::ChangeCustoItemXTimes(
                    cooked::isg::ObjectiveDescriptorChangeCustoItemXTimes {
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
                cooked::isg::ObjectiveDescriptor::CompleteXQuests(
                    cooked::isg::ObjectiveDescriptorCompleteXQuests {
                        description: value.description,
                        description_raw: value.description_raw,
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        quests_count: data.quests_count,
                        ..Default::default()
                    },
                )
            }
            ObjectiveType::DanceXSeconds(data) => cooked::isg::ObjectiveDescriptor::DanceXSeconds(
                cooked::isg::ObjectiveDescriptorDanceXSeconds {
                    description: value.description,
                    description_raw: value.description_raw,
                    is_static: value.is_static,
                    exclude_from_upload: value.exclude_from_upload,
                    dance_time: data.dance_time,
                    ..Default::default()
                },
            ),
            ObjectiveType::FinishXPlaylist(data) => {
                cooked::isg::ObjectiveDescriptor::FinishXPlaylist(
                    cooked::isg::ObjectiveDescriptorFinishXPlaylist {
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
            ObjectiveType::GatherXStars(data) => cooked::isg::ObjectiveDescriptor::GatherXStars(
                cooked::isg::ObjectiveDescriptorGatherXStars {
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
                cooked::isg::ObjectiveDescriptor::PlayDailyQuestsForXDays(
                    cooked::isg::ObjectiveDescriptorPlayDailyQuestsForXDays {
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
                cooked::isg::ObjectiveDescriptor::PlayGachaXTimes(
                    cooked::isg::ObjectiveDescriptorPlayGachaXTimes {
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
            ObjectiveType::PlayXMaps(data) => cooked::isg::ObjectiveDescriptor::PlayXMaps(
                cooked::isg::ObjectiveDescriptorPlayXMaps {
                    description: value.description,
                    description_raw: value.description_raw,
                    components: data.components.into_iter().map(Component::into).collect(),
                    is_static: value.is_static,
                    exclude_from_upload: value.exclude_from_upload,
                    maps_count: data.maps_count,
                    ..Default::default()
                },
            ),
            ObjectiveType::ReachRankX(data) => cooked::isg::ObjectiveDescriptor::ReachRankX(
                cooked::isg::ObjectiveDescriptorReachRankX {
                    description: value.description,
                    description_raw: value.description_raw,
                    is_static: value.is_static,
                    exclude_from_upload: value.exclude_from_upload,
                    rank_to_reach: data.rank_to_reach,
                    ..Default::default()
                },
            ),
            ObjectiveType::UnlockXPortraitBorders(data) => {
                cooked::isg::ObjectiveDescriptor::UnlockXPortraitBorders(
                    cooked::isg::ObjectiveDescriptorUnlockXPortraitBorders {
                        description: value.description,
                        description_raw: value.description_raw,
                        is_static: value.is_static,
                        exclude_from_upload: value.exclude_from_upload,
                        portrait_border_count: data.portrait_border_count,
                        ..Default::default()
                    },
                )
            }
            _ => cooked::isg::ObjectiveDescriptor::SwitchSweatMode(
                // Convert all objectives that are impossible to do with the mod to sweat mode
                cooked::isg::ObjectiveDescriptorBase {
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
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub enum ObjectiveType<'a> {
    /// Burn X calories
    #[serde(borrow)]
    AccumulateXCal(AccumulateXCal<'a>),
    /// Do X moves
    #[serde(borrow)]
    AccumulateXMoves(AccumulateXMoves<'a>),
    /// Active coop mode
    ActivateCoopMode,
    /// Add X song to a playlist
    #[serde(borrow)]
    AddXSongsToAPlaylist(AddXSongsToAPlaylist<'a>),
    /// Beat a World Dance Floor boss
    BeatWDFBoss,
    /// Change a customisation item X times
    #[serde(borrow)]
    ChangeCustoItemXTimes(ChangeCustoItemXTimes<'a>),
    /// Complete X quests
    CompleteXQuests(CompleteXQuests),
    /// Dance for X seconds
    DanceXSeconds(DanceXSeconds),
    /// Finish X playlists
    #[serde(borrow)]
    FinishXPlaylist(FinishXPlaylist<'a>),
    /// Gather X stars
    #[serde(borrow)]
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
    #[serde(borrow)]
    PlayGachaXTimes(PlayGachaXTimes<'a>),
    /// Have a savefile with a previous version of Just Dance
    PlayPreviousJD,
    /// Play a World Dance Floor tournament
    PlayWDFTournament(PlayWDFTournament),
    /// Play X maps
    #[serde(borrow)]
    PlayXMaps(PlayXMaps<'a>),
    /// Play X World Dance Floor tournament rounds
    #[serde(borrow)]
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
    #[serde(borrow)]
    UnlockXStickers(UnlockXStickers<'a>),
    /// Win a World Dance Floor team battle
    WinWDFTeamBattle,
}

impl<'a> ObjectiveType<'a> {
    /// Convert from the UbiArt representation
    ///
    /// # Errors
    /// Will error of converting the descriptor fails
    pub fn from_descriptor(
        descriptor: &cooked::isg::ObjectiveDescriptor<'a>,
        locale_id_map: &LocaleIdMap,
    ) -> Result<Self, Error> {
        match descriptor {
            cooked::isg::ObjectiveDescriptor::AccumulateXCal(data) => {
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
            cooked::isg::ObjectiveDescriptor::AccumulateXMoves(data) => {
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
            cooked::isg::ObjectiveDescriptor::ActivateCoopMode(_) => Ok(Self::ActivateCoopMode),
            cooked::isg::ObjectiveDescriptor::AddXSongsToAPlaylist(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::AddXSongsToAPlaylist(AddXSongsToAPlaylist {
                    songs_added_count: data.songs_added_count,
                    components,
                }))
            }
            cooked::isg::ObjectiveDescriptor::BeatWDFBoss(_) => Ok(Self::BeatWDFBoss),
            cooked::isg::ObjectiveDescriptor::ChangeCustoItemXTimes(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::ChangeCustoItemXTimes(ChangeCustoItemXTimes {
                    custo_item_changes_count: data.custo_item_changes_count,
                    components,
                }))
            }
            cooked::isg::ObjectiveDescriptor::CompleteXQuests(data) => {
                Ok(Self::CompleteXQuests(CompleteXQuests {
                    quests_count: data.quests_count,
                }))
            }
            cooked::isg::ObjectiveDescriptor::DanceXSeconds(data) => {
                Ok(Self::DanceXSeconds(DanceXSeconds {
                    dance_time: data.dance_time,
                }))
            }
            cooked::isg::ObjectiveDescriptor::FinishXPlaylist(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::FinishXPlaylist(FinishXPlaylist {
                    playlists_play_count: data.playlists_play_count,
                    components,
                }))
            }
            cooked::isg::ObjectiveDescriptor::GatherXStars(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::GatherXStars(GatherXStars {
                    stars_count: data.stars_count,
                    components,
                }))
            }
            cooked::isg::ObjectiveDescriptor::LinkedToUplay(_) => Ok(Self::LinkedToUplay),
            cooked::isg::ObjectiveDescriptor::OpenAnthologyMode(_) => Ok(Self::OpenAnthologyMode),
            cooked::isg::ObjectiveDescriptor::OpenPostcardsGallery(_) => {
                Ok(Self::OpenPostcardsGallery)
            }
            cooked::isg::ObjectiveDescriptor::OpenStickerAlbum(_) => Ok(Self::OpenStickerAlbum),
            cooked::isg::ObjectiveDescriptor::OpenVideoGallery(_) => Ok(Self::OpenVideoGallery),
            cooked::isg::ObjectiveDescriptor::PlayDailyQuestsForXDays(data) => {
                Ok(Self::PlayDailyQuestsForXDays(PlayDailyQuestsForXDays {
                    consecutive_days: data.consecutive_days,
                }))
            }
            cooked::isg::ObjectiveDescriptor::PlayGachaXTimes(data) => {
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
            cooked::isg::ObjectiveDescriptor::PlayPreviousJD(_) => Ok(Self::PlayPreviousJD),
            cooked::isg::ObjectiveDescriptor::PlayWDFTournament(data) => {
                Ok(Self::PlayWDFTournament(PlayWDFTournament {
                    tournament_count: data.tournament_count.unwrap_or(1),
                }))
            }
            cooked::isg::ObjectiveDescriptor::PlayXMaps(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::PlayXMaps(PlayXMaps {
                    maps_count: data.maps_count,
                    components,
                }))
            }
            cooked::isg::ObjectiveDescriptor::PlayXWDFTournamentRounds(data) => {
                let mut components = Vec::with_capacity(data.components.len());
                for component in &data.components {
                    components.push(Component::from_component(component, locale_id_map)?);
                }
                Ok(Self::PlayXWDFTournamentRounds(PlayXWDFTournamentRounds {
                    rounds_count: data.rounds_count,
                    components,
                }))
            }
            cooked::isg::ObjectiveDescriptor::ReachRankX(data) => {
                Ok(Self::ReachRankX(ReachRankX {
                    rank_to_reach: data.rank_to_reach,
                }))
            }
            cooked::isg::ObjectiveDescriptor::RenewJDUSub(_) => Ok(Self::RenewJDUSub),
            cooked::isg::ObjectiveDescriptor::SwitchSweatMode(_) => Ok(Self::SwitchSweatMode),
            cooked::isg::ObjectiveDescriptor::UnlockUplayRewardAliasPack1(_) => {
                Ok(Self::UnlockUplayRewardAliasPack1)
            }
            cooked::isg::ObjectiveDescriptor::UnlockUplayRewardAliasPack2(_) => {
                Ok(Self::UnlockUplayRewardAliasPack2)
            }
            cooked::isg::ObjectiveDescriptor::UnlockXPortraitBorders(data) => {
                Ok(Self::UnlockXPortraitBorders(UnlockXPortraitBorders {
                    portrait_border_count: data.portrait_border_count,
                }))
            }
            cooked::isg::ObjectiveDescriptor::UnlockXStickers(data) => {
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
            cooked::isg::ObjectiveDescriptor::WinWDFTeamBattle(_) => Ok(Self::WinWDFTeamBattle),
        }
    }

    /// Convert from the old UbiArt representation
    #[must_use]
    pub fn from_old_descriptor(
        descriptor: &shared_json_types::ObjectiveDesc<'a>,
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
            #[allow(clippy::match_same_arms, reason = "The comments matter")]
            shared_json_types::ObjectiveDesc::Base(data) => match data.objective_type {
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
                                acceptable_playlist_ids: vec![HipStr::borrowed(
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
            shared_json_types::ObjectiveDesc::MinStarsReachedSongCount(data) => {
                if data.objective_type != 18 {
                    println!("Warning!: ObjectiveDesc1819::MinStarsReachedSongCount does not have objective type 18");
                    println!("{data:?}");
                }
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
            shared_json_types::ObjectiveDesc::PlaySpecificMap(data) => {
                if data.objective_type != 10 {
                    println!("Warning!: ObjectiveDesc1819::PlaySpecificMap does not have objective type 10");
                    println!("{data:?}");
                }
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
            shared_json_types::ObjectiveDesc::GatherStarsWDF(data) => {
                if data.objective_type != 0 {
                    println!("Warning!: ObjectiveDesc1819::GatherStarsWDF does not have objective type 0");
                    println!("{data:?}");
                }
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
            shared_json_types::ObjectiveDesc::SweatSongCount(data) => {
                if data.objective_type != 11 {
                    println!("Warning!: ObjectiveDesc1819::SweatSongCount does not have objective type 11");
                    println!("{data:?}");
                }
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
            shared_json_types::ObjectiveDesc::WDFSongCount(data) => {
                if data.objective_type != 11 {
                    println!(
                        "Warning!: ObjectiveDesc1819::WDFSongCount does not have objective type 11"
                    );
                    println!("{data:?}");
                }
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
            shared_json_types::ObjectiveDesc::RecommendSongCount(data) => {
                // Recommend songs don't exist anymore as objective type, so changed to quickplay songs
                if data.objective_type != 19 {
                    println!("Warning!: ObjectiveDesc1819::RecommendSongCount does not have objective type 19");
                    println!("{data:?}");
                }
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
            shared_json_types::ObjectiveDesc::ClassicTournamentRank(data) => {
                // Classic Tournament doesn't exist anymore so map to regular rank
                if data.objective_type != 54 {
                    println!("Warning!: ObjectiveDesc1819::ClassicTournamentRank does not have objective type 54");
                    println!("{data:?}");
                }
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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, IntoOwned)]
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

impl From<MoveCategories> for u32 {
    #[allow(clippy::as_conversions, reason = "Is repr(Self)")]
    fn from(value: MoveCategories) -> Self {
        value as Self
    }
}

impl TryFrom<u32> for MoveCategories {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
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

impl TryFrom<&u32> for MoveCategories {
    type Error = Error;

    fn try_from(value: &u32) -> Result<Self, Self::Error> {
        Self::try_from(*value)
    }
}

/// Requirements for [`ObjectiveType::AccumulateXCal`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct AccumulateXCal<'a> {
    /// How many calories to burn
    pub calories_amount: u32,
    /// Should it be done in one session
    pub in_one_session: bool,
    /// Additional generic requirements
    #[serde(borrow)]
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::AccumulateXMoves`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct AccumulateXMoves<'a> {
    /// How many moves
    pub moves_count: u32,
    /// What grade should the moves be
    pub categories_to_count: Vec<MoveCategories>,
    /// Additional generic requirements
    #[serde(borrow)]
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::AddXSongsToAPlaylist`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct AddXSongsToAPlaylist<'a> {
    /// How many songs to add
    pub songs_added_count: u32,
    /// Additional generic requirements
    #[serde(borrow)]
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::ChangeCustoItemXTimes`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct ChangeCustoItemXTimes<'a> {
    /// How many items to change
    pub custo_item_changes_count: u32,
    /// Additional generic requirements
    #[serde(borrow)]
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::CompleteXQuests`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct CompleteXQuests {
    /// How many quests to complete
    pub quests_count: u32,
}

/// Requirements for [`ObjectiveType::DanceXSeconds`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct DanceXSeconds {
    /// How many seconds to dance
    pub dance_time: u32,
}

/// Requirements for [`ObjectiveType::FinishXPlaylist`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct FinishXPlaylist<'a> {
    /// How many playlists to finish
    pub playlists_play_count: u32,
    /// Additional generic requirements
    #[serde(borrow)]
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::GatherXStars`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct GatherXStars<'a> {
    /// How many stars to get
    pub stars_count: u32,
    /// Additional generic requirements
    #[serde(borrow)]
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::PlayDailyQuestsForXDays`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct PlayDailyQuestsForXDays {
    /// How many consecutive days
    pub consecutive_days: u32,
}

/// Requirements for [`ObjectiveType::PlayGachaXTimes`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct PlayGachaXTimes<'a> {
    /// How many times to play
    pub gacha_plays_count: u32,
    /// Unlock all gacha items (can be narrowed via [`Component`])
    pub unlock_all_acceptable_gacha_items: bool,
    /// Additional generic requirements
    #[serde(borrow)]
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::PlayWDFTournament`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct PlayWDFTournament {
    /// How many tournaments to play
    pub tournament_count: u32,
}

/// Requirements for [`ObjectiveType::PlayXMaps`]
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct PlayXMaps<'a> {
    /// How many maps to play
    pub maps_count: u32,
    /// Additional generic requirements
    #[serde(borrow)]
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
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct PlayXWDFTournamentRounds<'a> {
    /// How many rounds to play
    pub rounds_count: u32,
    /// Additional generic requirements
    #[serde(borrow)]
    pub components: Vec<Component<'a>>,
}

/// Requirements for [`ObjectiveType::ReachRankX`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct ReachRankX {
    /// Which rank to reach
    pub rank_to_reach: u32,
}

/// Requirements for [`ObjectiveType::UnlockXPortraitBorders`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct UnlockXPortraitBorders {
    /// How many portrait borders to unlock
    pub portrait_border_count: u32,
}

/// Requirements for [`ObjectiveType::UnlockXStickers`]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct UnlockXStickers<'a> {
    /// Unlock all sticker
    pub all_stickers: bool,
    /// How many stickers to unlock
    pub stickers_count: u32,
    /// Additional generic requirements
    #[serde(borrow)]
    pub components: Vec<Component<'a>>,
}

/// A requirements that can be shared between various objective types
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct Component<'a> {
    /// The actual requirements
    #[serde(borrow)]
    pub c_type: ComponentType<'a>,
    /// Should only different values be accepted
    pub only_diff_values: bool,
}

impl<'a> Component<'a> {
    /// Convert from UbiArt representation
    ///
    /// # Errors
    /// Will error if the component has invalid values
    pub fn from_component(
        component: &cooked::isg::ObjectiveDescriptorComponent<'a>,
        locale_id_map: &LocaleIdMap,
    ) -> Result<Self, Error> {
        let c_type = match component {
            cooked::isg::ObjectiveDescriptorComponent::CustoItemTypeRequirement(data) => {
                let mut acceptable_custo_item_types =
                    Vec::with_capacity(data.acceptable_custo_item_types.len());
                for item in &data.acceptable_custo_item_types {
                    acceptable_custo_item_types.push(CustomisableItemType::try_from(*item)?);
                }
                ComponentType::CustoItemTypeRequirement(CustoItemTypeRequirement {
                    acceptable_custo_item_types,
                })
            }
            cooked::isg::ObjectiveDescriptorComponent::GachaItemTypeRequirement(data) => {
                let mut acceptable_gacha_item_types =
                    Vec::with_capacity(data.acceptable_gacha_item_types.len());
                for item in &data.acceptable_gacha_item_types {
                    acceptable_gacha_item_types.push(GachaItemType::try_from(*item)?);
                }
                ComponentType::GachaItemTypeRequirement(GachaItemTypeRequirement {
                    acceptable_gacha_item_types,
                })
            }
            cooked::isg::ObjectiveDescriptorComponent::MapCoachCountRequirement(data) => {
                let mut acceptable_coach_counts =
                    Vec::with_capacity(data.acceptable_coach_counts.len());
                for item in &data.acceptable_coach_counts {
                    acceptable_coach_counts.push(NumberOfCoaches::try_from(*item)?);
                }
                ComponentType::MapCoachCountRequirement(MapCoachCountRequirement {
                    acceptable_coach_counts,
                })
            }
            cooked::isg::ObjectiveDescriptorComponent::MapLaunchLocationRequirement(data) => {
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
            cooked::isg::ObjectiveDescriptorComponent::MapMovesRequirement(data) => {
                let mut acceptable_categories =
                    Vec::with_capacity(data.acceptable_categories.len());
                for cat in &data.acceptable_categories {
                    acceptable_categories.push(cat.try_into()?);
                }
                if data.all_map_moves_count {
                    test_eq!(data.exact_moves_count, u32::MAX)?;
                    test_eq!(data.min_moves_count, u32::MAX)?;
                    test_eq!(data.max_moves_count, u32::MAX)?;
                    test_eq!(data.only_map_last_move, false)?;
                    test_eq!(data.moves_in_a_row, false)?;
                    ComponentType::MapRequireAllMoves(MapRequireAllMoves {
                        acceptable_categories,
                    })
                } else if data.only_map_last_move {
                    test_or!(
                        test_eq!(data.exact_moves_count, 1)
                            .and(test_eq!(data.min_moves_count, u32::MAX)),
                        test_eq!(data.exact_moves_count, u32::MAX)
                            .and(test_eq!(data.min_moves_count, 1)),
                    )?;
                    test_eq!(data.max_moves_count, u32::MAX)?;
                    test_eq!(data.all_map_moves_count, false)?;
                    test_eq!(data.moves_in_a_row, false)?;
                    ComponentType::MapRequireLastMove(MapRequireLastMove {
                        acceptable_categories,
                    })
                } else if data.moves_in_a_row {
                    test_eq!(data.exact_moves_count, u32::MAX)?;
                    test_eq!(data.max_moves_count, u32::MAX)?;
                    test_eq!(data.only_map_last_move, false)?;
                    test_eq!(data.all_map_moves_count, false)?;
                    ComponentType::MapRequireXMovesInARow(MapRequireXMovesInARow {
                        min_moves_count: data.min_moves_count,
                        acceptable_categories,
                    })
                } else {
                    return Err(anyhow!("MapMovesRequirement options are all false!"));
                }
            }
            cooked::isg::ObjectiveDescriptorComponent::MapNameRequirement(data) => {
                ComponentType::MapNameRequirement(MapNameRequirement {
                    acceptable_map_names: data.acceptable_map_names.clone(),
                })
            }
            cooked::isg::ObjectiveDescriptorComponent::MapPlaymodeRequirement(data) => {
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
            cooked::isg::ObjectiveDescriptorComponent::MapScoreRequirement(data) => {
                ComponentType::MapScoreRequirement(MapScoreRequirement {
                    score: data.score,
                    better_than_dancer_of_the_week: data.better_than_dancer_of_the_week,
                })
            }
            cooked::isg::ObjectiveDescriptorComponent::MapTagsRequirement(data) => {
                ComponentType::MapTagsRequirement(MapTagsRequirement {
                    acceptable_map_tags: data.acceptable_map_tags.clone(),
                    unacceptable_map_tags: data.unacceptable_map_tags.clone(),
                })
            }
            cooked::isg::ObjectiveDescriptorComponent::OnlyOnline(_) => ComponentType::OnlyOnline,
            cooked::isg::ObjectiveDescriptorComponent::OnlyOnUnlimitedSongs(_) => {
                ComponentType::OnlyOnUnlimitedSongs
            }
            cooked::isg::ObjectiveDescriptorComponent::PlaylistIdRequirement(data) => {
                ComponentType::PlaylistIdRequirement(PlaylistIdRequirement {
                    acceptable_playlist_ids: data.acceptable_playlist_ids.clone(),
                })
            }
            cooked::isg::ObjectiveDescriptorComponent::ScoringModeRequirement(data) => {
                let mut acceptable_scoring_modes =
                    Vec::with_capacity(data.acceptable_scoring_modes.len());
                for item in &data.acceptable_scoring_modes {
                    acceptable_scoring_modes.push(ScoringMode::try_from(*item)?);
                }
                ComponentType::ScoringModeRequirement(ScoringModeRequirement {
                    acceptable_scoring_modes,
                })
            }
            cooked::isg::ObjectiveDescriptorComponent::SearchLabelsRequirement(data) => {
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
            cooked::isg::ObjectiveDescriptorComponent::StickerIdRequirement(data) => {
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

impl<'a> From<Component<'a>> for cooked::isg::ObjectiveDescriptorComponent<'a> {
    fn from(value: Component<'a>) -> Self {
        match value.c_type {
            ComponentType::CustoItemTypeRequirement(data) => {
                cooked::isg::ObjectiveDescriptorComponent::CustoItemTypeRequirement(
                    cooked::isg::ObjectiveDescriptorComponentCustoItemTypeRequirement {
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
                cooked::isg::ObjectiveDescriptorComponent::GachaItemTypeRequirement(
                    cooked::isg::ObjectiveDescriptorComponentGachaItemTypeRequirement {
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
                cooked::isg::ObjectiveDescriptorComponent::MapCoachCountRequirement(
                    cooked::isg::ObjectiveDescriptorComponentMapCoachCountRequirement {
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
                cooked::isg::ObjectiveDescriptorComponent::MapLaunchLocationRequirement(
                    cooked::isg::ObjectiveDescriptorComponentMapLaunchLocationRequirement {
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
                cooked::isg::ObjectiveDescriptorComponent::MapNameRequirement(
                    cooked::isg::ObjectiveDescriptorComponentMapNameRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_map_names: data.acceptable_map_names,
                    },
                )
            }
            ComponentType::MapPlaymodeRequirement(data) => {
                cooked::isg::ObjectiveDescriptorComponent::MapPlaymodeRequirement(
                    cooked::isg::ObjectiveDescriptorComponentMapPlaymodeRequirement {
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
                cooked::isg::ObjectiveDescriptorComponent::MapScoreRequirement(
                    cooked::isg::ObjectiveDescriptorComponentMapScoreRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        score: data.score,
                        better_than_dancer_of_the_week: data.better_than_dancer_of_the_week,
                    },
                )
            }
            ComponentType::MapTagsRequirement(data) => {
                cooked::isg::ObjectiveDescriptorComponent::MapTagsRequirement(
                    cooked::isg::ObjectiveDescriptorComponentMapTagsRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_map_tags: data.acceptable_map_tags.clone(),
                        unacceptable_map_tags: data.unacceptable_map_tags.clone(),
                    },
                )
            }
            ComponentType::OnlyOnline => cooked::isg::ObjectiveDescriptorComponent::OnlyOnline(
                cooked::isg::ObjectiveDescriptorComponentBase {
                    class: None,
                    only_diff_values: value.only_diff_values,
                },
            ),
            ComponentType::OnlyOnUnlimitedSongs => {
                cooked::isg::ObjectiveDescriptorComponent::OnlyOnUnlimitedSongs(
                    cooked::isg::ObjectiveDescriptorComponentBase {
                        class: None,
                        only_diff_values: value.only_diff_values,
                    },
                )
            }
            ComponentType::PlaylistIdRequirement(data) => {
                cooked::isg::ObjectiveDescriptorComponent::PlaylistIdRequirement(
                    cooked::isg::ObjectiveDescriptorComponentPlaylistIdRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_playlist_ids: data.acceptable_playlist_ids,
                    },
                )
            }
            ComponentType::ScoringModeRequirement(data) => {
                cooked::isg::ObjectiveDescriptorComponent::ScoringModeRequirement(
                    cooked::isg::ObjectiveDescriptorComponentScoringModeRequirement {
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
                cooked::isg::ObjectiveDescriptorComponent::SearchLabelsRequirement(
                    cooked::isg::ObjectiveDescriptorComponentSearchLabelsRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_label_loc_ids: data.acceptable_labels,
                        forbidden_label_loc_ids: data.forbidden_labels,
                    },
                )
            }
            ComponentType::StickerIdRequirement(data) => {
                cooked::isg::ObjectiveDescriptorComponent::StickerIdRequirement(
                    cooked::isg::ObjectiveDescriptorComponentStickerIdRequirement {
                        class: None,
                        only_diff_values: value.only_diff_values,
                        acceptable_sticker_ids: data.acceptable_sticker_ids,
                    },
                )
            }
            ComponentType::MapRequireAllMoves(data) => {
                cooked::isg::ObjectiveDescriptorComponent::MapMovesRequirement(
                    cooked::isg::ObjectiveDescriptorComponentMapMovesRequirement {
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
                cooked::isg::ObjectiveDescriptorComponent::MapMovesRequirement(
                    cooked::isg::ObjectiveDescriptorComponentMapMovesRequirement {
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
                cooked::isg::ObjectiveDescriptorComponent::MapMovesRequirement(
                    cooked::isg::ObjectiveDescriptorComponentMapMovesRequirement {
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
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
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
    #[serde(borrow)]
    MapNameRequirement(MapNameRequirement<'a>),
    /// Require a playmode
    MapPlaymodeRequirement(MapPlaymodeRequirement),
    /// Require a minium score
    MapScoreRequirement(MapScoreRequirement),
    /// Require certain tags
    #[serde(borrow)]
    MapTagsRequirement(MapTagsRequirement<'a>),
    /// Require to be online
    OnlyOnline,
    /// Require Unlimited songs
    OnlyOnUnlimitedSongs,
    /// Require certain playlists
    #[serde(borrow)]
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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, IntoOwned)]
pub enum CustomisableItemType {
    /// Avatars
    Avatar = 0,
    /// Aliases
    Alias = 1,
    /// Portrait borders/Skin
    Skin = 2,
}

impl From<CustomisableItemType> for u32 {
    #[allow(clippy::as_conversions, reason = "Is repr(Self)")]
    fn from(value: CustomisableItemType) -> Self {
        value as Self
    }
}

impl TryFrom<u32> for CustomisableItemType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Avatar),
            1 => Ok(Self::Alias),
            2 => Ok(Self::Skin),
            _ => Err(anyhow!("Unknown ItemType: {value}")),
        }
    }
}

/// Require certain customisation item types
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct CustoItemTypeRequirement {
    /// Accepatble customisation item types
    pub acceptable_custo_item_types: Vec<CustomisableItemType>,
}

/// Item types that can be won in the gacha machine
#[repr(u8)]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, IntoOwned)]
pub enum GachaItemType {
    /// Postcard
    #[default]
    Postcard = 0,
}

impl From<GachaItemType> for u32 {
    #[allow(clippy::as_conversions, reason = "Is repr(Self)")]
    fn from(value: GachaItemType) -> Self {
        value as Self
    }
}

impl TryFrom<u32> for GachaItemType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Postcard),
            _ => Err(anyhow!("Unknown GachaItemType: {value}")),
        }
    }
}

/// Require certain item types
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct GachaItemTypeRequirement {
    /// Acceptable item types
    pub acceptable_gacha_item_types: Vec<GachaItemType>,
}

/// Require X coaches
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct MapCoachCountRequirement {
    /// Acceptable coach amounts
    pub acceptable_coach_counts: Vec<NumberOfCoaches>,
}

/// Where is the map launched from
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, IntoOwned)]
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

impl From<LaunchContext> for HipStr<'static> {
    fn from(value: LaunchContext) -> Self {
        match value {
            LaunchContext::Family => HipStr::borrowed("context_family"),
            LaunchContext::Quickplay => HipStr::borrowed("context_quickplay"),
            LaunchContext::WorldDanceFloor => HipStr::borrowed("context_wdf"),
            LaunchContext::Kids => HipStr::borrowed("context_kids"),
            LaunchContext::Anthology => HipStr::borrowed("context_anthology"),
        }
    }
}

/// Where is the map launched from
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, IntoOwned)]
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

impl From<LaunchSubcontext> for HipStr<'static> {
    fn from(value: LaunchSubcontext) -> Self {
        match value {
            LaunchSubcontext::Search => HipStr::borrowed("search"),
            LaunchSubcontext::Home => HipStr::borrowed("home"),
        }
    }
}

/// Require the map to be launched in a certain way
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct MapLaunchLocationRequirement {
    /// Normal ways
    pub acceptable_launch_contexts: Vec<LaunchContext>,
    /// Search and recommendations
    pub acceptable_launch_subcontexts: Vec<LaunchSubcontext>,
}

/// Require all moves to be a specific grade
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct MapRequireAllMoves {
    /// The grade
    pub acceptable_categories: Vec<MoveCategories>,
}

/// Require last move to be a specific grade
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct MapRequireLastMove {
    /// The grades
    pub acceptable_categories: Vec<MoveCategories>,
}

/// Require X moves of certain grade
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct MapRequireXMovesInARow {
    /// Minimum amount of moves in a row
    pub min_moves_count: u32,
    /// All these grades
    pub acceptable_categories: Vec<MoveCategories>,
}

/// Require map codename
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct MapNameRequirement<'a> {
    /// Map codenames
    #[serde(borrow)]
    pub acceptable_map_names: Vec<HipStr<'a>>,
}

/// Require specific playmodes
#[allow(clippy::struct_excessive_bools, reason = "Forced by engine")]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
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
#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
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
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct MapTagsRequirement<'a> {
    /// Acceptable tags
    #[serde(borrow)]
    pub acceptable_map_tags: Vec<HipStr<'a>>,
    /// Forbidden tags
    #[serde(borrow)]
    pub unacceptable_map_tags: Vec<HipStr<'a>>,
}

/// Require a specifc playlist
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct PlaylistIdRequirement<'a> {
    /// Acceptable playlists
    #[serde(borrow)]
    pub acceptable_playlist_ids: Vec<HipStr<'a>>,
}

/// A scoring mode like controller or Kinect
#[repr(u8)]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, IntoOwned)]
pub enum ScoringMode {
    /// Using a phone or controller
    #[default]
    PhoneDevice = 2,
}

impl From<ScoringMode> for u32 {
    #[allow(clippy::as_conversions, reason = "Is repr(Self)")]
    fn from(value: ScoringMode) -> Self {
        value as Self
    }
}

impl TryFrom<u32> for ScoringMode {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::PhoneDevice),
            _ => Err(anyhow!("Unknown ScoringMode {value}")),
        }
    }
}

/// Require a specific scoring mode (Kinect/Controller)
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct ScoringModeRequirement {
    /// The scoring modes
    pub acceptable_scoring_modes: Vec<ScoringMode>,
}

/// Require specific search labels
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct SearchLabelsRequirement {
    /// Required labels
    pub acceptable_labels: Vec<LocaleId>,
    /// Forbidden labels
    pub forbidden_labels: Vec<LocaleId>,
}

/// Require specific sticker ids
#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, IntoOwned)]
pub struct StickerIdRequirement {
    /// The sticker ids
    pub acceptable_sticker_ids: Vec<u32>,
}
