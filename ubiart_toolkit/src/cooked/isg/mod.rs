#![allow(clippy::struct_excessive_bools, reason = "Format is dictated by the engine")]
use std::collections::HashMap;

use hipstr::HipStr;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use superstruct::superstruct;
use ubiart_toolkit_shared_types::{Color, LocaleId};

use crate::shared_json_types::{AutodanceVideoStructure, Empty, ObjectiveDesc};

pub type AliasesObjectives<'a> = HashMap<u32, HipStr<'a>>;
pub type MapsGoals<'a> = HashMap<HipStr<'a>, Vec<HipStr<'a>>>;
pub type MapsObjectives<'a> = HashMap<HipStr<'a>, HipStr<'a>>;
pub type OfflineRecommendation<'a> = Vec<HipStr<'a>>;
pub type AvatarsObjectives<'a> = HashMap<u32, HipStr<'a>>;

pub use crate::utils::json::parse;

/// For serde to set a value to default to `u32::MAX`
const fn u32_max() -> u32 {
    u32::MAX
}

#[superstruct(
    variants(V22, V21, V20, V20C, V19, V18, V17, V16),
    variant_attributes(
        serde_as,
        derive(Debug, Serialize, Deserialize, Clone),
        serde(deny_unknown_fields, rename_all = "camelCase")
    )
)]
pub struct GameManagerConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub game_text_file_path: HipStr<'a>,
    #[serde(borrow)]
    pub loading: HipStr<'a>,
    #[serde(borrow)]
    pub game_flow_scene_path: HipStr<'a>,
    #[serde(borrow)]
    pub camera_shake_config: HipStr<'a>,
    pub cut_scene_default_unskippable_duration_first_time: u32,
    pub max_local_players: u32,
    pub max_online_players: u32,
    pub max_bonus_teensy: u32,
    pub jdversion: u32,
    pub attract_waiting_time: u32,
    pub sweat_calories_per_second: f32,
    pub sweat_met_value: u32,
    pub other_met_value: u32,
    pub sweat_magic_mult: u32,
    pub sweat_magic_add: u32,
    #[serde(borrow, rename = "carousel_rules")]
    pub carousel_rules: HipStr<'a>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow, rename = "alias_db_path")]
    pub alias_db_path: HipStr<'a>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub jdpaths: HashMap<u32, HipStr<'a>>,
    #[serde(borrow)]
    pub jdblockspath: HipStr<'a>,
    #[serde(borrow)]
    pub jdcommontapepath: HipStr<'a>,
    #[serde(borrow)]
    pub picto_component_tpl_paths: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow)]
    pub dynamic_music_track_component_tpl_path: HipStr<'a>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub anchor_tpl_path: HipStr<'a>,
    #[serde(borrow, rename = "songdb_scene")]
    pub songdb_scene: HipStr<'a>,
    #[serde(borrow, rename = "agingbot_behavioursTpl")]
    pub agingbot_behaviours_tpl: HipStr<'a>,
    #[serde(borrow, rename = "avatardb_scene")]
    pub avatardb_scene: HipStr<'a>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow, rename = "skindb_scene")]
    pub skindb_scene: HipStr<'a>,
    #[serde(borrow, rename = "flagdb_scene")]
    pub flagdb_scene: HipStr<'a>,
    #[superstruct(only(V16, V17, V18, V19))]
    #[serde(borrow)]
    pub avatar_folder: HipStr<'a>,
    #[superstruct(only(V17, V18, V19))]
    #[serde(borrow)]
    pub pin_unplayed_song: HipStr<'a>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub wdf_player_name_prefix_on_xbox_one: HipStr<'a>,
    #[superstruct(only(V21, V22))]
    #[serde(borrow)]
    /// Introduced in an update for V21
    pub wdf_player_name_prefix_on_stadia: Option<HipStr<'a>>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow, rename = "wdfPlayerNamePrefixNonPS4")]
    pub wdf_player_name_prefix_non_ps4: HipStr<'a>,
    #[superstruct(only(V16, V17))]
    /// Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub song_tags: Option<Vec<HipStr<'a>>>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub shortcut_descriptors: HashMap<HipStr<'a>, ShortcutDesc1719<'a>>,
    #[superstruct(only(V16, V17, V18, V19))]
    #[serde(borrow)]
    pub short_cut_configs: HashMap<HipStr<'a>, ShortcutSetup1619<'a>>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub space_between_shortcuts: HipStr<'a>,
    #[superstruct(only(V16, V17, V18, V19))]
    #[serde(borrow)]
    pub default_phone_images: HashMap<HipStr<'a>, HipStr<'a>>,
    pub max_controller_sleep_time: f32,
    #[superstruct(only(V16, V17, V18, V19))]
    #[serde(borrow)]
    pub audio_package_name: HipStr<'a>,
    #[serde(borrow)]
    pub package_scene_paths: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow, rename = "ed_songdb_scene")]
    pub ed_songdb_scene: HipStr<'a>,
    #[serde(borrow)]
    pub cameras: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow)]
    pub uiscenes: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow)]
    pub banner_scenes: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub transition_scenes: Vec<HipStr<'a>>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub actors_to_bundle: Vec<HipStr<'a>>,
    #[superstruct(only(V16, V17, V18))]
    #[serde(borrow)]
    pub genericstages: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow)]
    pub popupconfigs: PopupConfigList<'a>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub clubrewardconfigs: HashMap<HipStr<'a>, ClubRewardConfig<'a>>,
    #[serde(borrow)]
    pub scoringparams: ScoringParams<'a>,
    #[superstruct(only(V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub scoringcameraparams: ScoringCameraParams<'a>,
    #[superstruct(only(V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub scoringmovespaceparams: ScoringMovespaceParams<'a>,
    #[superstruct(only(V16, V17))]
    #[serde(borrow)]
    pub kinect_scoringparams: ScoringCameraParams<'a>,
    #[serde(borrow)]
    pub menuassetsparams: Vec<MenuAssetsCacheParams<'a>>,
    #[serde(borrow)]
    pub menumusicsparams: HashMap<HipStr<'a>, MenuMusicParams<'a>>,
    #[serde(borrow)]
    pub remotesoundparams: HashMap<HipStr<'a>, RemoteSoundParams<'a>>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub menu_music_multi_tracks: HashMap<HipStr<'a>, MenuMultiTrackItem<'a>>,
    #[serde(borrow)]
    pub menumusicconfig: MenuMusicConfig<'a>,
    #[superstruct(only(V16, V17, V18))]
    pub sweat_programs: Vec<u32>,
    #[superstruct(only(V16, V17, V18))]
    #[serde(borrow)]
    pub mashupdates: HashMap<HipStr<'a>, u32>,
    #[superstruct(only(V16, V17, V18))]
    #[serde(borrow)]
    pub mashupavatars: HashMap<HipStr<'a>, u32>,
    #[superstruct(only(V16, V17, V18, V19))]
    #[serde(borrow)]
    pub mojoprices: HashMap<HipStr<'a>, u32>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub rankdescriptor: RankDescriptor<'a>,
    #[superstruct(only(V16, V17))]
    #[serde(borrow)]
    pub slave_phone_loc_ids: HashMap<HipStr<'a>, Vec<u32>>,
    #[superstruct(only(V16, V17))]
    #[serde(borrow)]
    pub questdataentries: Vec<QuestEntry1617<'a>>,
    #[superstruct(only(V17))]
    #[serde(borrow)]
    pub questplayercamslot: HashMap<HipStr<'a>, Vec<u32>>,
    #[serde(borrow)]
    pub unlimitedupsellsonglist: Vec<UnlimitedUpsellSongList<'a>>,
    #[superstruct(only(V20, V21, V22))]
    /// Not in the Japanese version of JD20
    #[serde(borrow, rename = "defaultJDUVideoPreviewSubtitles")]
    pub default_jdu_video_preview_subtitles: Option<UnlimitedUpsellSubtitles<'a>>,
    #[superstruct(only(V18, V19))]
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub galaxyconfig: Vec<SystemDescriptor18<'a>>,
    #[superstruct(only(V18, V19))]
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub adventure_bosses: Vec<AdventureBossDesc18<'a>>,
    #[superstruct(only(V18, V19))]
    #[serde(
        borrow,
        rename = "adventuremode_setup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub adventuremode_setup: Option<AdventureModeSetup18<'a>>,
    #[superstruct(only(V16, V17, V18, V19))]
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub questconfig: Option<QuestConfig1618<'a>>,
    #[superstruct(only(V16, V17, V18, V19))]
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub questchallengerentries: Vec<QuestChallengerEntry1618<'a>>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    /// Only on NX
    pub customizableitemconfig: Option<CustomizableItemConfig<'a>>,
    #[superstruct(only(V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow, rename = "scheduled_questSetup")]
    pub scheduled_quest_setup: ScheduledQuestSetup<'a>,
    #[superstruct(only(V17))]
    #[serde(borrow)]
    pub dancemachinerandomizeconfig: DanceMachineRandomSetup17<'a>,
    #[superstruct(only(V17, V18, V19))]
    #[serde(borrow)]
    pub dancemachineglobalconfig: DanceMachineGlobalConfig1719<'a>,
    #[superstruct(only(V17))]
    /// Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub dancemachinedebugconfig: Option<DanceMachineDebugConfig<'a>>,
    #[superstruct(only(V16, V17, V18, V19))]
    #[serde(borrow)]
    pub sweatrandomizeconfig: SweatRandomizeConfig1619<'a>,
    #[superstruct(only(V17, V18, V19))]
    #[serde(borrow)]
    pub searchconfig: SearchConfig1719<'a>,
    #[superstruct(only(V16, V17, V18, V19))]
    #[serde(borrow)]
    pub challenger_evolution_template_list: Vec<ChallengerScoreEvolutionTemplate1619<'a>>,
    #[serde(borrow)]
    pub countryentries: Vec<CountryEntry<'a>>,
    #[superstruct(only(V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub default_country_code: HipStr<'a>,
    #[serde(borrow)]
    pub credits_textbox_path: HipStr<'a>,
    #[superstruct(only(V16, V17, V18))]
    pub avatar_min_anim_hud_duration: u32,
    #[superstruct(only(V16, V17, V18))]
    #[serde(borrow)]
    pub b2b_maps: Vec<HipStr<'a>>,
    #[superstruct(only(V16, V17, V18))]
    #[serde(borrow)]
    pub chatmessagesparams: ChatMessagesParams1618<'a>,
    #[superstruct(only(V16, V17, V18))]
    #[serde(borrow)]
    pub chat_messages: HashMap<HipStr<'a>, Vec<u32>>,
    #[superstruct(only(V16, V17))]
    /// Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub challenge_algo_order: Option<Vec<HipStr<'a>>>,
    #[superstruct(only(V16, V17))]
    /// Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub challenge_friend_score_offset: Option<f32>,
    #[superstruct(only(V16, V17))]
    /// Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub challenge_signature_score_offset: Option<f32>,
    #[superstruct(only(V16, V17))]
    /// Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub challenger_connection_time_delay: Option<f32>,
    #[superstruct(only(V16, V17, V18, V19, V20, V20C))]
    pub coop_score_diamonds_values: Vec<f32>,
    #[superstruct(only(V16, V17))]
    pub coop_jauge_anim_time: Vec<u32>,
    #[superstruct(only(V16, V17, V18, V19))]
    pub rival_recap_incr_score_speed: f32,
    #[superstruct(only(V16, V17))]
    /// Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retained_most_played_among_all_played_songs_ratio: Option<f32>,
    #[superstruct(only(V16, V17))]
    /// Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked_songs_push_occurence_value: Option<u32>,
    #[superstruct(only(V16, V17))]
    /// Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unlimited_songs_push_occurence_value: Option<u32>,
    #[superstruct(only(V16, V17))]
    /// Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_recap_min_played_song_count_before_unlimited: Option<u32>,
    #[serde(borrow)]
    pub countdown_delays: HashMap<HipStr<'a>, u32>,
    #[serde(borrow)]
    pub autodance_effects_list: Vec<AutoDanceEffectData<'a>>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub autodance_transition_sound_path: HipStr<'a>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    pub autodance_transition_sound_synchronise_sample: u32,
    #[superstruct(only(V17, V18, V19))]
    pub autodance_transition_sound_synchronise_time: u32,
    #[superstruct(only(V16, V17))]
    #[serde(borrow)]
    pub coop_tweaked_texts: Vec<CoopTweakedText17<'a>>,
    #[superstruct(only(V16, V17, V18))]
    #[serde(borrow)]
    pub messages_slides: HashMap<HipStr<'a>, TutorialContent<'a>>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub tutorials_contents: HashMap<HipStr<'a>, TutorialContent<'a>>,
    #[serde(borrow)]
    pub tutorials: Vec<TutorialDesc<'a>>,
    #[serde(borrow)]
    pub redeem_maps: HashMap<HipStr<'a>, Vec<HipStr<'a>>>,
    #[superstruct(only(V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow, rename = "UplayRewards")]
    pub uplay_rewards: Vec<UplayReward<'a>>,
    #[superstruct(only(V16, V17))]
    #[serde(borrow)]
    pub uplay_unlockable_maps: HashMap<HipStr<'a>, u32>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    pub stars_6th_step_song_score: u32,
    #[superstruct(only(V17, V18, V19, V20, V20C))]
    pub stars_6th_step_incoming_effect_start_relative_score: i32,
    #[superstruct(only(V18, V19, V20, V20C, V21, V22))]
    pub stars_7th_step_song_score: u32,
    #[superstruct(only(V18, V19, V20, V20C, V21, V22))]
    pub perfect_feedback_min_score: u32,
    #[superstruct(only(V17))]
    pub perfect_plus_feedback_min_score: u32,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    pub min_song_nb_for_shuffle: u32,
    #[superstruct(only(V19))]
    pub stars_needed_to_unlock_extreme_alt_map: u32,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub wdf_boss_entries: Vec<WDFBossEntry<'a>>,
    #[superstruct(only(V18, V19))]
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub adventure_objectives: Vec<AdventureObjective18<'a>>,
    #[superstruct(only(V18, V19))]
    #[serde(borrow, rename = "scheduled_quests")]
    pub scheduled_quests: Vec<ScheduledQuestDesc1819<'a>>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub itemcolorlookup: ItemColorLookUp<'a>,
    #[superstruct(only(V17, V18, V19))]
    #[serde(borrow)]
    pub looped_video_config: HashMap<HipStr<'a>, VideoLoopSetup<'a>>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow, rename = "defaultJDUVideoPreview")]
    pub default_jdu_video_preview: HipStr<'a>,
    #[superstruct(only(V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow, rename = "defaultJDUVideoPreview_kids")]
    pub default_jdu_video_preview_kids: HipStr<'a>,
    #[superstruct(only(V17))]
    pub diamond_points: Vec<u32>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    pub jd_points_per_star: Vec<u32>,
    #[superstruct(only(V17, V18, V19, V20, V20C, V21, V22))]
    /// Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub banned_maps_in_chinese: Option<Vec<HipStr<'a>>>,
    #[superstruct(only(V18, V19))]
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub hueconfig: Option<HueConfig<'a>>,
    #[superstruct(only(V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub collectiblealbum: CollectibleAlbum<'a>,
    #[superstruct(only(V18, V19, V20, V20C, V21))]
    #[serde(borrow)]
    pub stickerdatabase: Vec<StickerEntry<'a>>,
    #[superstruct(only(V18, V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub gachaconfig: GachaConfig<'a>,
    #[superstruct(only(V18, V19))]
    #[serde(borrow)]
    pub ftueconfig: FTUEConfig<'a>,
    #[superstruct(only(V18, V19))]
    #[serde(borrow)]
    pub rumbleconfig: RumbleConfig<'a>,
    #[superstruct(only(V18, V19))]
    pub profile_landing_stats_thresholds: Vec<(u32, u32, u32)>,
    #[superstruct(only(V21, V22), no_getter)]
    #[serde(borrow)]
    pub config_files_path: ConfigFilesPathV2122<'a>,
    #[superstruct(only(V20, V20C), no_getter)]
    #[serde(borrow)]
    pub config_files_path: ConfigFilesPathV20<'a>,
    #[superstruct(only(V19), no_getter)]
    #[serde(borrow)]
    pub config_files_path: ConfigFilesPathV19<'a>,
    #[superstruct(only(V18), no_getter)]
    #[serde(borrow)]
    pub config_files_path: ConfigFilesPathV18<'a>,
    #[superstruct(only(V18, V19))]
    pub news_update_interval: u32,
    #[superstruct(only(V18, V19))]
    pub new_update_pause_time: u32,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub watermark: HipStr<'a>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub grid_actors_to_preload: HashMap<HipStr<'a>, GridActorsToPreload<'a>>,
    #[superstruct(only(V19))]
    #[serde(borrow)]
    pub grid_descriptors: HashMap<HipStr<'a>, GridDesc<'a>>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub grid_item_descriptors: HashMap<HipStr<'a>, CarouselElementDesc<'a>>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub tabbed_grids_layout_descriptors: HashMap<HipStr<'a>, LayoutTabbedGrids<'a>>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub home_data_config: HomeDataConfig<'a>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub home_data_tips_config: Vec<HomeDataTipEntry<'a>>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub home_data_default_article_thumbnail: HipStr<'a>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub home_data_online_incentive_article_thumbnail: HipStr<'a>,
    #[superstruct(only(V19))]
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub home_videos_descs: HashMap<HipStr<'a>, HomeVideoDesc<'a>>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow, rename = "special_characters")]
    pub special_characters: Vec<HipStr<'a>>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow, rename = "derived_letters")]
    pub derived_letters: HashMap<HipStr<'a>, HipStr<'a>>,
    #[superstruct(only(V19, V20, V20C, V21, V22))]
    #[serde(borrow, rename = "search_labels")]
    pub search_labels: SongsSearchTags<'a>,
    #[superstruct(only(V20, V20C))]
    #[serde(borrow)]
    pub groups_sound_notification_config: GroupsSoundNotificationConfig<'a>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub mapsobjectives: MapsObjectives<'a>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub maps_goals: MapsGoals<'a>,
    #[superstruct(only(V20, V20C, V21, V22))]
    pub legacy_alias_id: u32,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(rename = "JDUAliasId")]
    pub jdu_alias_id: u32,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub avatarsobjectives: AvatarsObjectives<'a>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub aliasesobjectives: AliasesObjectives<'a>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub specific_cases_check_order: Vec<HipStr<'a>>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub on_fly_notification_types: HashMap<HipStr<'a>, OnFlyNotificationTypeParams<'a>>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub bubbles_prioritized_notif_types_groups: Vec<Vec<HipStr<'a>>>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub reward_screen_prioritized_notif_types: Vec<HipStr<'a>>,
    #[superstruct(only(V20, V20C, V21, V22))]
    pub bubbles_pile_delay_before_exit: Vec<f32>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub new_notification_tree: HashMap<HipStr<'a>, Vec<HipStr<'a>>>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub recap_config: RecapConfig<'a>,
    #[superstruct(only(V20, V20C, V21, V22))]
    #[serde(borrow)]
    pub offline_recommendation: OfflineRecommendation<'a>,
    #[superstruct(only(V20, V21, V22))]
    #[serde(borrow, rename = "whats_new_configs")]
    pub whats_new_configs: Option<WhatsNewConfigs<'a>>,
    #[superstruct(only(V21, V22))]
    #[serde(borrow, rename = "wdf_linear_rewards_path")]
    pub wdf_linear_rewards_path: HipStr<'a>,
}

impl<'a> GameManagerConfig<'a> {
    pub const fn config_files_path<'b>(&'b self) -> Result<ConfigFilesPathRef<'b, 'a>, ()> {
        match self {
            GameManagerConfig::V22(gmc) => Ok(ConfigFilesPathRef::V2122(&gmc.config_files_path)),
            GameManagerConfig::V21(gmc) => Ok(ConfigFilesPathRef::V2122(&gmc.config_files_path)),
            GameManagerConfig::V20(gmc) => Ok(ConfigFilesPathRef::V20(&gmc.config_files_path)),
            GameManagerConfig::V20C(gmc) => Ok(ConfigFilesPathRef::V20(&gmc.config_files_path)),
            GameManagerConfig::V19(gmc) => Ok(ConfigFilesPathRef::V19(&gmc.config_files_path)),
            GameManagerConfig::V18(gmc) => Ok(ConfigFilesPathRef::V18(&gmc.config_files_path)),
            GameManagerConfig::V16(_) | GameManagerConfig::V17(_) => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct DanceMachineDebugConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub debug_list: HashMap<HipStr<'a>, Vec<HipStr<'a>>>,
}

#[superstruct(
    variants(V2122, V20, V19, V18),
    variant_attributes(derive(Debug, Serialize, Deserialize, Clone)),
    enum_variant_attributes(serde(borrow))
)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub struct ConfigFilesPath<'a> {
    #[superstruct(only(V20, V2122))]
    #[serde(borrow)]
    pub gachacontent: HipStr<'a>,
    #[superstruct(only(V20, V2122))]
    #[serde(borrow)]
    pub ftuesteps: HipStr<'a>,
    #[superstruct(only(V20))]
    #[serde(borrow)]
    pub anthology: HipStr<'a>,
    #[superstruct(only(V20, V2122))]
    #[serde(borrow)]
    pub objectives: HipStr<'a>,
    #[superstruct(only(V19, V20, V2122))]
    #[serde(borrow)]
    pub playlist: HipStr<'a>,
    #[superstruct(only(V20, V2122))]
    #[serde(borrow)]
    pub portraitborders: HipStr<'a>,
    #[superstruct(only(V2122))]
    #[serde(borrow)]
    pub quickplayrules: HipStr<'a>,
    #[superstruct(only(V20, V2122))]
    #[serde(borrow)]
    pub scheduledquests: HipStr<'a>,
    #[superstruct(only(V18, V19))]
    #[serde(borrow)]
    pub dmconfig: HipStr<'a>,
    #[superstruct(only(V19))]
    #[serde(borrow)]
    pub postcards: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScheduledQuestDatabase1819<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub scheduled_quests: Vec<ScheduledQuestDesc1819<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ScheduledQuestDesc1819<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Type")]
    pub type_it: u8,
    pub unlimited_only: bool,
    pub mojo_reward: u32,
    #[serde(borrow)]
    pub objective: ObjectiveDesc<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub preconditions_objectives_id: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<HipStr<'a>>,
}

impl ScheduledQuestDesc1819<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_ScheduledQuestDesc");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AchievementsDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub achievements: Vec<AchievementDescriptor<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AchievementDescriptor<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub platform_id: u32,
    pub uplay_id: u32,
    #[serde(
        rename = "uplayLocID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub uplay_loc_id: Option<u32>,
    #[serde(borrow)]
    pub unlock_objective_desc_id: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CameraShakeConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub shakes: Vec<CameraShake<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CameraShake<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub intensity: f32,
    pub duration: f32,
    pub ease_in_duration: f32,
    pub ease_out_duration: f32,
    #[serde(borrow)]
    pub shake_x: CameraShakeCurveParams<'a>,
    #[serde(borrow)]
    pub shake_y: CameraShakeCurveParams<'a>,
    #[serde(borrow)]
    pub shake_z: CameraShakeCurveParams<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CameraShakeCurveParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub xofs: u32,
    pub yofs: u32,
    pub x_scale: u32,
    pub y_scale: u32,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    #[serde(rename = "loop")]
    pub loop_it: u32,
    pub frequency: u32,
    pub amplitude: f32,
    pub offset: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GachaContentDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub collectibles: Vec<CollectibleGachaItem<'a>>,
}

impl GachaContentDatabase<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_GachaContentDatabase_Template");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CollectibleGachaItem<'a> {
    #[serde(borrow, rename = "JD_CollectibleGachaItemAlias")]
    Alias(CollectibleGachaItemAlias<'a>),
    #[serde(borrow, rename = "JD_CollectibleGachaItemAvatar")]
    Avatar(CollectibleGachaItemAvatar<'a>),
    #[serde(borrow, rename = "JD_CollectibleGachaItemPortraitBorder")]
    PortraitBorder(CollectibleGachaItemPortraitBorder<'a>),
    #[serde(borrow, rename = "JD_CollectibleGachaItemSticker")]
    Sticker(CollectibleGachaItemSticker<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleGachaItemAlias<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub alias_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleGachaItemAvatar<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub avatar_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleGachaItemPortraitBorder<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub portrait_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleGachaItemSticker<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sticker_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShortcutSetup1619<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub contexts: HashMap<HipStr<'a>, ContextSetup1719<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContextSetup1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub platforms: HashMap<HipStr<'a>, ShortcutDescriptorList1719<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShortcutDescriptorList1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub descriptor_list: Vec<ShortcutDescriptor1719<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum ShortcutDescriptor1719<'a> {
    #[serde(borrow, rename = "JD_DancerProfileShortcutDescriptor")]
    DancerProfile(DancerProfileShortcutDescriptor1719<'a>),
    #[serde(borrow, rename = "JD_ShortcutDescriptor")]
    Base(ShortcutDesc1719<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DancerProfileShortcutDescriptor1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    // Not in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_on_phone: Option<bool>,
    #[serde(borrow)]
    pub behaviour_name: HipStr<'a>,
    pub show_button: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShortcutDesc1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    // Not in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_on_phone: Option<bool>,
    #[serde(borrow)]
    pub behaviour_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct PopupConfigList<'a> {
    // In nx2017 this is not a class, but a regular hashmap
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<HipStr<'a>, PopupContentConfig<'a>>>,
    // Not used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub navigation: Option<HashMap<HipStr<'a>, PopupNavigationConfig<'a>>>,
    // Not used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub popup_description: Option<Vec<PopupParams<'a>>>,
    /// Retired after NX2019
    #[serde(rename = "menuDebugErrorList", default)]
    pub menu_debug_error_list: Option<Vec<u32>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub club_cross: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub renew_cross: Option<PopupConfig<'a>>,
    // Only used in nx2017, all caps on WiiU
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Option::is_none",
        alias = "DEFAULT"
    )]
    pub default: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub check_cross: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub cross_check: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub none: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub overwrite_nosave: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub retry_nosave: Option<PopupConfig<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupContentConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub content_scene_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupNavigationConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub left_item: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub middle_item: HipStr<'a>,
    #[serde(borrow)]
    pub right_item: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub up_item: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub bottom_item: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(default)]
    pub start_button_index: u32,
    #[serde(borrow)]
    pub phone_button_image: HipStr<'a>,
    pub phone_button_loc_id: u32,
    pub button_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub popup_id: HipStr<'a>,
    #[serde(borrow)]
    pub content_key: HipStr<'a>,
    #[serde(borrow)]
    pub navigation_key: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(default)]
    pub full_screen_display: bool,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub popup_overriding_sound_context: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub grid_overriding_sound_context: HipStr<'a>,
    pub display_wait_screen_on_phone_during_enter: u32,
    /// Introduced in NX2020
    #[serde(default)]
    pub message_left_alignment: bool,
    /// Introduced in NX2020
    #[serde(default)]
    pub message_area_width: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub associate_error_list: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub left_item: HipStr<'a>,
    #[serde(borrow)]
    pub right_item: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ClubRewardConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "locIdCR")]
    pub loc_id_cr: u32,
    #[serde(borrow, rename = "imgUrlCR")]
    pub img_url_cr: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScoringParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub on_fire_default: f32,
    #[serde(rename = "on_fire_X360")]
    pub on_fire_x360: f32,
    #[serde(rename = "on_fire_Durango")]
    pub on_fire_durango: f32,
    // Not in nx2017
    #[serde(
        rename = "KidsMode_charity_OK_no_move_energy_amount_factor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kids_mode_charity_ok_no_move_energy_amount_factor: Option<u32>,
    // Not in nx2017
    #[serde(
        rename = "KidsMode_charity_OK_score_ratio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kids_mode_charity_ok_score_ratio: Option<f32>,
    // Not in nx2017
    #[serde(
        rename = "KidsMode_decreasingScoreRatioBoost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kids_mode_decreasing_score_ratio_boost: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_distance_low_threshold: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_distance_high_threshold: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_auto_correlation_theshold: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_direction_impact_factor: Option<u32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_move_penalty_if_energy_amount_under: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charity_bonus_if_energy_factor_above: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perfect_malus_if_energy_factor_under: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_no_move_penalty_if_energy_amount_under: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_shake_detected_max_score_ratio: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_direction_malus_multiplier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "force_use_WiiU_classifiers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_use_wiiu_classifiers: Option<u32>,
    // Only in nx2017
    #[serde(
        rename = "PSMove_energy_modifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub psmove_energy_modifier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "PSMove_low_distance_threshold_modifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub psmove_low_distance_threshold_modifier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "PSMove_high_distance_threshold_modifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub psmove_high_distance_threshold_modifier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "PSMove_shake_sensitivity_modifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub psmove_shake_sensitivity_modifier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "PSMove_direction_sensibility_modifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub psmove_direction_sensibility_modifier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "SwitchJoyCon_direction_malus_multiplier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub switch_joycon_direction_malus_multiplier: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScoringCameraParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "maxPlayerCountScoreBoostPower")]
    pub max_player_count_score_boost_power: f32,
    // Not in nx2018 or earlier
    #[serde(
        rename = "no_move_penalty_if_energy_amount_under_XOne",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_move_penalty_if_energy_amount_under_xone: Option<u32>,
    // Not in nx2018 or earlier
    #[serde(
        rename = "no_move_penalty_if_energy_amount_under_PS4",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_move_penalty_if_energy_amount_under_ps4: Option<u32>,
    // Not in nx2018 or earlier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charity_bonus_if_energy_factor_above: Option<f32>,
    // Not in nx2018 or earlier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perfect_malus_if_energy_factor_under: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScoringMovespaceParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub default_distance_low_threshold: f32,
    pub default_distance_high_threshold: f32,
    pub default_auto_correlation_theshold: f32,
    pub default_direction_impact_factor: u32,
    pub no_move_penalty_if_energy_amount_under: f32,
    pub charity_bonus_if_energy_factor_above: f32,
    pub perfect_malus_if_energy_factor_under: f32,
    pub phone_no_move_penalty_if_energy_amount_under: f32,
    pub phone_shake_detected_max_score_ratio: f32,
    pub phone_direction_malus_multiplier: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_move_score_boost: Option<u32>,
    #[serde(
        rename = "force_use_WiiU_classifiers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_use_wiiu_classifiers: Option<u32>,
    #[serde(rename = "PSMove_energy_modifier")]
    pub psmove_energy_modifier: u32,
    #[serde(rename = "PSMove_low_distance_threshold_modifier")]
    pub psmove_low_distance_threshold_modifier: f32,
    #[serde(rename = "PSMove_high_distance_threshold_modifier")]
    pub psmove_high_distance_threshold_modifier: f32,
    #[serde(rename = "PSMove_shake_sensitivity_modifier")]
    pub psmove_shake_sensitivity_modifier: u32,
    #[serde(rename = "PSMove_direction_sensibility_modifier")]
    pub psmove_direction_sensibility_modifier: i32,
    #[serde(rename = "SwitchJoyCon_direction_malus_multiplier")]
    pub switch_joycon_direction_malus_multiplier: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MenuAssetsCacheParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub asset_type: HipStr<'a>,
    pub max_assets: u32,
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub default_assets: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub asset_path_fmts: HashMap<HipStr<'a>, HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MenuMusicParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub scene_path: HipStr<'a>,
    pub prefetch: u32,
    pub fadein: u32,
    /// Not in 2016
    #[serde(borrow, default)]
    pub stinger: HipStr<'a>,
    /// Not in 2016
    #[serde(borrow, default)]
    pub jingle: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RemoteSoundParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sound_id_for_phone: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MenuMultiTrackItem<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub menu_music_path: HipStr<'a>,
    /// Not in 2016
    #[serde(borrow, default)]
    pub stinger: HipStr<'a>,
    /// Not in 2016
    #[serde(borrow, default)]
    pub jingle: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MenuMusicConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub loop_cross_fade_duration: f32,
    pub start_fade_duration: f32,
    pub multi_track_transition_beat_count: u32,
    pub end_of_loop_soundwich_notif_time_offset: u32,
    /// Only in 2016
    #[serde(borrow, default)]
    pub menu_music_multi_tracks: HashMap<HipStr<'a>, HipStr<'a>>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct RankDescriptor<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "maxRank")]
    pub max_rank: u32,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub color_look_up: HashMap<u32, u32>,
    pub rank_limits: Vec<u32>,
    // Not in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub gain_types: Option<HashMap<HipStr<'a>, u32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QuestEntry1617<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub quest_id: HipStr<'a>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    pub locked: u32,
    pub trigger_end: u32,
    #[serde(borrow)]
    pub phone_image: HipStr<'a>,
    #[serde(borrow)]
    pub playlist: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub cover_path: HipStr<'a>,
    #[serde(borrow)]
    pub logo_path: HipStr<'a>,
    #[serde(borrow)]
    pub logo_shaded_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnlimitedUpsellSongList<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(borrow)]
    pub artist: HipStr<'a>,
    #[serde(borrow, default)]
    pub map_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct SystemDescriptor18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(borrow)]
    pub button_scene_id: HipStr<'a>,
    #[serde(borrow)]
    pub visual_scene_id: HipStr<'a>,
    pub boss_id: u32,
    #[serde(borrow)]
    pub planets: Vec<PlanetDescriptor18<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PlanetDescriptor18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub map_list: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub play_mode: HipStr<'a>,
    pub is_boss_planet: bool,
    pub is_surprise: bool,
    #[serde(borrow)]
    pub planet_objectives: Vec<PlanetObjectiveDesc18<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PlanetObjectiveDesc18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub objective_id: u32,
    pub mandatory: bool,
    pub rewards: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AdventureBossDesc18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub avatar_id: u32,
    pub skin_id: u32,
    pub final_score: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AdventureModeSetup18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub video_paths: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QuestConfig1618<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "rankingScenePathID")]
    pub ranking_scene_path_id: HipStr<'a>,
    #[serde(borrow, rename = "rankingActorPathID")]
    pub ranking_actor_path_id: HipStr<'a>,
    pub difficulty_final_scores: Vec<(u32, u32)>,
    pub threshold_rank: u32,
    pub nb_challengers: u32,
    pub ranking_points_gain: Vec<u32>,
    pub mojo_per_star: u32,
    pub mojo_per_rank: Vec<(u32, u32, u32)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QuestChallengerEntry1618<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub avatar_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnlimitedUpsellSubtitles<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub subtitles: HipStr<'a>,
    pub subtitles_loc_ids: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CustomizableItemConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub default_avatar_id: u32,
    pub default_skin_id: u32,
    /// Introduced in NX2020
    #[serde(default)]
    pub default_portrait_border_id: u32,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ScheduledQuestSetup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub minimum_score: u32,
    pub session_count_until_discovery_kill: u32,
    pub session_count_until_quest_kill: u32,
    pub session_count_until_first_discovery_kill: u32,
    pub session_count_until_normal_quest_setting: u32,
    pub first_discovery_quest_id: u32,
    /// Not used before NX2020
    #[serde(borrow, rename = "MapProbabilities", default)]
    pub map_probabilities: MapChoosingProbabilities<'a>,
    /// Superseded by `map_probabilities` in NX2020 and later
    #[serde(
        borrow,
        rename = "MapProbabilitiesNX",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_probabilities_nx: Option<MapChoosingProbabilities<'a>>,
    /// Superseded by `map_probabilities` in NX2020 and later
    #[serde(
        borrow,
        rename = "MapProbabilitiesOther",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_probabilities_other: Option<MapChoosingProbabilities<'a>>,
    #[serde(rename = "PushSongProbability")]
    pub push_song_probability: u32,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub selection_pre_conditions: Option<HashMap<HipStr<'a>, Vec<u32>>>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub update_timings: HashMap<u32, f32>,
    pub time_cap_in_hours_to_renew: u32,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub exclude_from_algorithm_quest_tags: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct MapChoosingProbabilities<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub ubisoft_club: u32,
    pub normal_sku: u32,
    /// Not used in NX2020 and later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double_scoring: Option<u32>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct DanceMachineRandomSetup17<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub increase_priority_tag: HashMap<HipStr<'a>, u32>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub special_priority: HashMap<u32, DanceMachinePriority17<'a>>,
    pub bonus_stage_min_value: u32,
    pub bonus_stage_max_value: u32,
    pub default_increase_priority_value: u32,
    pub bonus_priority_for_lowest_played_blocks: u32,
    pub delta_for_unlock_unlimited: u32,
    #[serde(borrow)]
    pub reward_tag_order: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct DanceMachinePriority17<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub priority_map: HashMap<HipStr<'a>, i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct DanceMachineGlobalConfig1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub value_for_win_move: f32,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_for_unlock_rewards: Option<Vec<f32>>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nb_blocks: Option<u32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_score_for_global_battery: Option<u32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_animation_recap_global: Option<u32>,
    // Only in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub video_path: Option<HashMap<HipStr<'a>, Vec<HipStr<'a>>>>,
    // Only in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub anim_syncrho: Option<HashMap<HipStr<'a>, HipStr<'a>>>,
    // Only in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub first_experience: Option<Vec<HipStr<'a>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SweatRandomizeConfig1619<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub weights: HashMap<HipStr<'a>, Vec<f32>>,
    #[serde(borrow)]
    pub excluded_tags: Vec<HipStr<'a>>,
    pub seed_range: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SearchConfig1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub num_of_search_history: u32,
    // Not in nx2019?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_rows: Option<u32>,
    // Not in nx2019?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tries_artist_match: Option<u32>,
    // Not in nx2019?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_num_artist_matches: Option<u32>,
    // Not in nx2019?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tries_tag_match: Option<u32>,
    // Not in nx2019?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tag_matches: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ChallengerScoreEvolutionTemplate1619<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub template_name: HipStr<'a>,
    #[serde(borrow)]
    pub template_descriptor: HashMap<HipStr<'a>, f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CountryEntry<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub loc_id: u32,
    #[serde(borrow)]
    pub code: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub region: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ChatMessagesParams1618<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "AllMessages_MinDelayBetweenMessages")]
    pub all_messages_min_delay_between_messages: f32,
    #[serde(rename = "AutoMessages_MinDelayBeforeFirstMessage")]
    pub auto_messages_min_delay_before_first_message: f32,
    #[serde(rename = "AutoMessages_MaxDelayBeforeFirstMessage")]
    pub auto_messages_max_delay_before_first_message: f32,
    #[serde(rename = "AutoMessages_MessagesAverageDelay")]
    pub auto_messages_messages_average_delay: f32,
    #[serde(rename = "AutoMessages_MessagesAverageDelayRandomVariation")]
    pub auto_messages_messages_average_delay_random_variation: f32,
    #[serde(rename = "AutoMessages_DelaysBoostForEachAdditionalAvatar")]
    pub auto_messages_delays_boost_for_each_additional_avatar: f32,
    #[serde(rename = "AutoMessages_GenericMessagesOccurrenceFactor")]
    pub auto_messages_generic_messages_occurrence_factor: f32,
    #[serde(rename = "AutoMessages_LevelBasedMessagesOccurrenceFactor")]
    pub auto_messages_level_based_messages_occurrence_factor: f32,
    #[serde(rename = "AutoMessages_CountryBasedMessagesOccurrenceFactor")]
    pub auto_messages_country_based_messages_occurrence_factor: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_ReadyToPlay_MinDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_ready_to_play_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_ReadyToPlay_MaxDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_ready_to_play_max_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_NewPosition_MinDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_new_position_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_NewPosition_MaxDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_new_position_max_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_NewPositionUpOrDown_TriggerProbability")]
    pub instant_messages_raceline_in_game_new_position_up_or_down_trigger_probability: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_GoldMove_MinDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_gold_move_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_GoldMove_MaxDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_gold_move_max_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_Emote_MinDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_emote_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_Emote_MaxDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_emote_max_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_Vote_MinDelayBeforeMessage")]
    pub instant_messages_raceline_vote_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_Vote_MaxDelayBeforeMessage")]
    pub instant_messages_raceline_vote_max_delay_before_message: f32,
    #[serde(rename = "InstantMessages_RecapResult_MinDelayBeforeMessage")]
    pub instant_messages_recap_result_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Recapresult_MaxDelayBeforeMessage")]
    pub instant_messages_recapresult_max_delay_before_message: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, untagged)]
pub enum AutoDanceEffectData<'a> {
    #[serde(borrow)]
    JD1722(AutoDanceEffectData1722<'a>),
    #[serde(borrow)]
    JD16(AutoDanceEffectData16<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AutoDanceEffectData1722<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "video_structure")]
    pub video_structure: Box<AutodanceVideoStructure<'a>>,
    pub effect_type: u32,
    #[serde(borrow)]
    pub effect_id: HipStr<'a>,
    /// Only in 2016
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub effect_map_name: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AutoDanceEffectData16<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub effect_type: u32,
    #[serde(borrow)]
    pub effect_map_name: HipStr<'a>,
    #[serde(borrow)]
    pub effect_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CoopTweakedText17<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub min_score: f32,
    pub title: u32,
    #[serde(default = "u32_max")]
    pub title_one_player: u32,
    pub desc: u32,
    #[serde(default = "u32_max")]
    pub desc_one_player: u32,
    #[serde(borrow)]
    pub sound_notification: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct TutorialContent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub popup: u32,
    pub browsable: u32,
    pub slide_delay: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub platforms: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub message_descs: Vec<MessageDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum MessageDesc<'a> {
    #[serde(borrow, rename = "MessageSlideDesc")]
    Slide(MessageSlideDesc<'a>),
    #[serde(borrow, rename = "MessageFocusDesc")]
    Focus(MessageFocusDesc<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct MessageSlideDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub message_id: u32,
    #[serde(borrow)]
    pub image_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MessageFocusDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub focus_id: u32,
    pub loc_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct TutorialDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub tutorial_context: HipStr<'a>,
    #[serde(borrow)]
    pub game_context: HipStr<'a>,
    // Named Messages before nx2019
    #[serde(borrow, alias = "Messages")]
    pub contents: Vec<HipStr<'a>>,
    pub priority: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub mandatory_song_tags: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub optional_song_tags: Vec<HipStr<'a>>,
    pub max_display: i32,
    pub max_display_per_session: i32,
    /// Not used after NX2019
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_ftue_step: Option<i32>,
    /// Introduced in NX2019
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_dependant: Option<bool>,
    /// Introduced in NX2020
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub tracking_string: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UplayReward<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub reward_id: u32,
    #[serde(borrow)]
    pub reward_name: HipStr<'a>,
    pub reward_type: u32,
    pub amount_to_unlock: u32,
    #[serde(borrow)]
    pub reward_string_on_server: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WDFBossEntry<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub boss_id: HipStr<'a>,
    #[serde(borrow)]
    pub scene_path: HipStr<'a>,
    #[serde(borrow)]
    pub logo: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AdventureObjective18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(borrow)]
    pub objective: ObjectiveDesc<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemColorLookUp<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub descriptor: HashMap<HipStr<'a>, ItemColorMap<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemColorMap<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub colors: HashMap<HipStr<'a>, Color>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoLoopSetup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "videoFPS")]
    pub video_fps: u32,
    #[serde(borrow)]
    pub descriptors: HashMap<HipStr<'a>, VideoBrickDescriptor<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoBrickDescriptor<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub local_start_frame: u32,
    pub local_end_frame: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HueConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub menu_color: Color,
    pub gold_effect_color: Color,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbum<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_page_delay: Option<f32>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub bonus_page_unlock_objective_id: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub pages: Vec<CollectibleAlbumPage<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumPage<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CollectibleAlbumItem<'a>>,
    #[serde(borrow)]
    pub scene_path: HipStr<'a>,
    // Not present in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub carousel_item_scene_id: HipStr<'a>,
    #[serde(default)]
    pub is_bonus_page: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CollectibleAlbumItem<'a> {
    #[serde(borrow, rename = "JD_CollectibleAlbumItemSticker")]
    Sticker(CollectibleAlbumItemSticker<'a>),
    #[serde(borrow, rename = "JD_CollectibleAlbumItemCustomizable")]
    Customizable(CollectibleAlbumItemCustomizable<'a>),
    #[serde(borrow, rename = "JD_CollectibleAlbumItemPostcard")]
    Postcard(CollectibleAlbumItemPostcard<'a>),
    #[serde(borrow, rename = "JD_CollectibleAlbumItemMap")]
    Map(CollectibleAlbumItemMap<'a>),
    #[serde(borrow, rename = "JD_CollectibleAlbumItemJDM")]
    JDM(CollectibleAlbumItemJDM<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemSticker<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub rarity: u32,
    pub sticker_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemCustomizable<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub rarity: u32,
    pub customizable_item_id: u32,
    pub customizable_item_type: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemPostcard<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub postcard_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemMap<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub rarity: u32,
    #[serde(borrow)]
    pub map_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemJDM<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub rarity: u32,
    #[serde(borrow)]
    pub episode_id: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StickerEntry<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sticker_id: u32,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub objective_id: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<HipStr<'a>>,
    // Not used in nx2020 and after
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub scene_path: Option<HipStr<'a>>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_emblem: Option<bool>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_sound: Option<bool>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GachaConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub price: u32,
    pub nb_max_history_pickup_reward: u32,
    #[serde(borrow)]
    pub reward_unlock_scenes: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub rarity_pickup_percentage: HashMap<u32, u32>,
    pub force_high_rarity_reward_count: u32,
    pub force_mojo_reward_count: u32,
    // Not used in nx2019 or later
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub mojo_reward_list: Option<Vec<GachaMojoRewardConfig<'a>>>,
    #[serde(borrow)]
    pub puzzle_map_reward: HipStr<'a>,
    pub nb_maps_threshold_before_push_gacha_screen: (u32, u32),
    // Not used in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_play_count_between_map_rewards: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GachaMojoRewardConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub mojo_amount: u32,
    pub number_of_packs: u32,
    pub rarity: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FTUEConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not in nx2018 or before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ordered_steps: Option<Vec<StepInfo<'a>>>,
    #[serde(borrow)]
    pub songs_to_be_kept_unlocked: Vec<HipStr<'a>>,
    // Not in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps_helper_version_mismatch_indicator: Option<u32>,
    // Not in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub songs_available_on_main_carousel: Option<i32>,
    // Not in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub songs_to_unlock_dance_card: Option<i32>,
    // Not in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub songs_to_push_gatcha: Option<i32>,
    // Not in nx2019 or later
    #[serde(
        rename = "songsToUnlockJDMainCarousel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub songs_to_unlock_jd_main_carousel: Option<i32>,
    // Not in nx2019 or later
    #[serde(
        rename = "songsToUnlockJDU",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub songs_to_unlock_jdu: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StepInfo<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub step: u32,
    pub map_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RumbleConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sleep_time: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GridDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub parent_marker: HipStr<'a>,
    pub delta_x: u32,
    pub delta_y: u32,
    pub rows_count: u32,
    pub columns_count: u32,
    pub start_line: u32,
    pub start_line_offset: u32,
    pub start_column: u32,
    pub line_scroll_condition: u32,
    pub no_scroll_height: u32,
    pub element_count_per_line: u32,
    pub visible_element_count: i32,
    pub banner_trigger_time: f32,
    pub audio_preview_time: f32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub actors_to_preload: Vec<ActorsToPreload<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GridActorsToPreload<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub actors_to_preload: Vec<ActorsToPreload<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActorsToPreload<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub scene_path: HipStr<'a>,
    #[serde(borrow)]
    pub actor_path: HipStr<'a>,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LayoutTabbedGrids<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub tabbed_grid_descs: Vec<TabbedGridDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TabbedGridDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub tab_name: HipStr<'a>,
    #[serde(borrow)]
    pub tab_hover_tutorial_context: HipStr<'a>,
    #[serde(borrow)]
    pub tab_content_tutorial_context: HipStr<'a>,
    pub start_index: u32,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub grid_actors_to_preload_id: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_trigger_time: Option<f32>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_preview_time: Option<f32>,
    // Not used in nx2020 and later
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub grid_desc_id: Option<HipStr<'a>>,
    // Not used in nx2020 and later
    #[serde(
        rename = "RequiredFtueSteps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_ftue_steps: Option<u32>,
    // Not used in nx2020 and later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires_full_install: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HomeDataConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub elements_list_by_visual_type: HashMap<HipStr<'a>, [HipStr<'a>; 5]>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HomeDataTipEntry<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub tip_id: HipStr<'a>,
    pub catch_phrase: u32,
    pub content: u32,
    #[serde(borrow)]
    pub thumbnail: HipStr<'a>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<HipStr<'a>>,
    // Not used in nx2020 and later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by_first_time_layout: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HomeVideoDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub video_thumbnail_path: HipStr<'a>,
    #[serde(borrow)]
    pub video_path: HipStr<'a>,
    pub video_tile_title_loc_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongsSearchTags<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub maps: HashMap<HipStr<'a>, SongSearchTags<'a>>,
}

impl SongsSearchTags<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_SongsSearchTags");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongSearchTags<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub tags: Vec<SongSearchTag<'a>>,
}

impl SongSearchTags<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_SongSearchTags");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongSearchTag<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub tag_loc_id: LocaleId,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
}

impl SongSearchTag<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_SongSearchTag");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GroupsSoundNotificationConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "delay_before_sendingEvent_enterNewGroupStartTimer")]
    pub delay_before_sending_event_enter_new_group_start_timer: f32,
    #[serde(rename = "delay_before_sendingEvent_changeGroup")]
    pub delay_before_sending_event_change_group: f32,
    #[serde(borrow)]
    pub items_group: Vec<ItemsGroup<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemsGroup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub group_name: HipStr<'a>,
    pub items_indexes: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OnFlyNotificationTypeParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub default_form_and_timing: FormAndTiming<'a>,
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub specific_cases_form_and_timing: HashMap<HipStr<'a>, FormAndTiming<'a>>,
    pub bubble_title_loc_id: u32,
    pub reward_screen_title_loc_id: u32,
    pub specific_content_loc_id: u32,
    pub forbid_reward_screen_flow_jump_button: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FormAndTiming<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub form: u32,
    pub timing: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RecapConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub gauge_number_of_stars_per_beat: u32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WhatsNewConfigs<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub configs: Vec<WhatsNewConfig<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WhatsNewConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub config_name: HipStr<'a>,
    #[serde(borrow)]
    pub ui_display: HipStr<'a>,
    pub max_views: u32,
    pub session_interval: u32,
    #[serde(borrow)]
    pub related_song_tags: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub subscribed_grid_desc: HipStr<'a>,
    #[serde(borrow)]
    pub unsubscribed_grid_desc: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselManager<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    /// Not in 2016
    #[serde(borrow, default)]
    pub anim_setups: HashMap<HipStr<'a>, CarouselAnimSetup<'a>>,
    #[serde(borrow)]
    pub carousel_descs: HashMap<HipStr<'a>, CarouselDesc<'a>>,
    #[serde(borrow)]
    pub item_object: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow)]
    pub item_logic: HashMap<HipStr<'a>, HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselAnimSetup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub acceleration: u32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub min_deceleration_start_ratio: f32,
    pub max_deceleration_start_ratio: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    pub start_index: u32,
    pub is_loop: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<CarouselElementDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CarouselElementDesc<'a> {
    #[serde(borrow, rename = "CarouselElementDesc_Base")]
    CarouselElementDescBase(CarouselElementDescBase<'a>),
    #[serde(borrow, rename = "CarouselElementDesc_Carousel")]
    CarouselElementDescCarousel(CarouselElementDescCarousel<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc")]
    Base(JdCarouselElementDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action")]
    Action(JdCarouselElementDescAction<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ActivateConnection")]
    ActionActivateConnection(JdCarouselElementDescActionActivateConnection<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Age")]
    ActionAge(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Amiibo")]
    ActionAmiibo(JdCarouselElementDescActionBase<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_ChangeCluster_InstallCheck"
    )]
    ActionChangeClusterInstallCheck(JdCarouselElementDescActionChangeCluster<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_ChangeCluster_VideoChallenge"
    )]
    ActionChangeClusterVideoChallenge(JdCarouselElementDescActionChangeCluster<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ChangeCluster_WDF")]
    ActionChangeClusterWDF(JdCarouselElementDescActionChangeCluster<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ChangeCluster")]
    ActionChangeCluster(JdCarouselElementDescActionChangeCluster<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ChangePage")]
    ActionChangePage(JdCarouselElementDescActionChangePage<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_ChangePageFromHomeTile"
    )]
    ActionChangePageFromHomeTile(JdCarouselElementDescActionChangePageFromHomeTile<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ChangePageWithContext")]
    ActionChangePageWithContext(JdCarouselElementDescActionChangePageWithContext<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_CheckBox")]
    ActionCheckBox(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_DeviceSelection")]
    ActionDeviceSelection(JdCarouselElementDescActionBase<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_EditFocusedDancerCardNickname"
    )]
    ActionEditFocusedDancerCardNickname(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_EnterGameMode")]
    ActionEnterGameMode(JdCarouselElementDescActionEnterGameMode<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_EquipAlias")]
    ActionEquipAlias(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_EquipAvatar")]
    ActionEquipAvatar(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_EquipPortraitBorder")]
    ActionEquipPortraitBorder(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Gacha_Cancel")]
    ActionGachaCancel(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Gacha_Play")]
    ActionGachaPlay(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Gender")]
    ActionGender(JdCarouselElementDescActionBase<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_Goto_DancerCardCreation"
    )]
    ActionGotoDancerCardCreation(JdCarouselElementDescActionGoto<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Goto_Gacha")]
    ActionGotoGacha(JdCarouselElementDescActionGotoGacha<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Goto_JDU")]
    ActionGotoJDU(JdCarouselElementDescActionGoto<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Goto_Postcards")]
    ActionGotoPostcards(JdCarouselElementDescActionGoto<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_LaunchJDTV")]
    ActionLaunchJDTV(JdCarouselElementDescActionLaunchJDTV<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_LaunchWDF")]
    ActionLaunchWDF(JdCarouselElementDescActionGoto<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_OpenHomeArticle")]
    ActionOpenHomeArticle(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_OpenKeyboard")]
    ActionOpenKeyboard(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_OpenUplay")]
    ActionOpenUplay(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_PlaySong")]
    ActionPlaySong(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_PlaySongBanner")]
    ActionPlaySongBanner(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_PostcardFullscreen")]
    ActionPostcardFullscreen(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_QuestDifficulty")]
    ActionQuestDifficulty(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_RematchChallenge")]
    ActionRematchChallenge(JdCarouselElementDescActionRematchChallenge<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ReportChallengeVideo")]
    ActionReportChallengeVideo(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_SavePlaylist")]
    ActionSavePlaylist(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_SelectChallengeMode")]
    ActionSelectChallengeMode(JdCarouselElementDescActionSelectChallengeMode<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_SendTauntMessage")]
    ActionSendTauntMessage(JdCarouselElementDescActionBase<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_SetFocusedDancerCardAsMain"
    )]
    ActionSetFocusedDancerCardAsMain(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_StartPlaylist")]
    ActionStartPlaylist(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_StartQuickplay")]
    ActionStartQuickplay(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_SweatHome")]
    ActionSweatHome(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_WDF_VoteChoice")]
    ActionWDFVoteChoice(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_TabItem")]
    TabItem(JdCarouselElementDescTabItem<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionBase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescAction<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionActivateConnection<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub connection: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangePage<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionSelectChallengeMode<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionRematchChallenge<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangePageFromHomeTile<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    // Not present in nx2019 and before
    #[serde(
        rename = "needToSetJDUDestination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub need_to_set_jdu_destination: Option<bool>,
    // Not present in nx2020 and after
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub tracking_tile_type: Option<HipStr<'a>>,
    // Not present in nx2020 and after
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub tracking_tile_sub_type: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangePageWithContext<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow)]
    pub context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangeCluster<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionEnterGameMode<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionGotoGacha<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    pub gacha_mode: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionGoto<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionLaunchJDTV<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow)]
    pub context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct BannerSetup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "Type")]
    pub type_it: HipStr<'a>,
    #[serde(borrow)]
    pub theme: HipStr<'a>,
    #[serde(borrow)]
    pub context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselElementDescBase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub half_size_x: Option<u32>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub half_size_y: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<CarouselElementDesc<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescTabItem<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselElementDescCarousel<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow, rename = "carouselDescID")]
    pub carousel_desc_id: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CarouselElementDescComponent<'a> {
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_MojoDisplay")]
    MojoDisplay(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Device")]
    Device(JdCarouselElementDescComponentDevice<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_GameMode")]
    GameMode(JdCarouselElementDescComponentGameMode<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_NewMarker_Item")]
    NewMarkerItem(JdCarouselElementDescComponentNewMarkerItem<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_NewMarker_Tab")]
    NewMarkerTab(JdCarouselElementDescComponentNewMarkerTab<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDescComponent_PostcardsCompletionDisplay"
    )]
    PostcardsCompletionDisplay(JdCarouselElementDescComponentCompletionDisplay<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_SoundNotification")]
    SoundNotification(JdCarouselElementDescComponentSoundNotification<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDescComponent_SoundNotification_ForHomeTileMap"
    )]
    SoundNotificationForHomeTileMap(JdCarouselElementDescComponentSoundNotification<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDescComponent_StickerAlbumCompletionDisplay"
    )]
    StickerAlbumCompletionDisplay(JdCarouselElementDescComponentCompletionDisplay<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_WDF_VoteChoice")]
    WDFVoteChoice(JdCarouselElementDescComponentWDFVoteChoice<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Age")]
    Age(JdCarouselElementDescComponentAge<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Amiibo")]
    Amiibo(JdCarouselElementDescComponentAmiibo<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Cluster")]
    Cluster(JdCarouselElementDescComponentCluster<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Gender")]
    Gender(JdCarouselElementDescComponentGender<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Rival_Mode")]
    RivalMode(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Coop_Mode")]
    CoopMode(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_SweatMode")]
    SweatMode(JdCarouselElementDescComponentSweatMode<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_QuestDifficulty")]
    QuestDifficulty(JdCarouselElementDescComponentQuestDifficulty<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_TauntCategory")]
    TauntCategory(JdCarouselElementDescComponentTauntCategory<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentAge<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub catage: u32,
    #[serde(borrow)]
    pub phone_image_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentAmiibo<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub avatar_id: u32,
    pub character_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentCluster<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub context: HipStr<'a>,
    #[serde(borrow)]
    pub transition: HipStr<'a>,
    #[serde(borrow)]
    /// Not in 2019
    pub news_placement: Option<HipStr<'a>>,
    pub availability_check: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentGender<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub gender: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentSweatMode<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sweat_mode: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentQuestDifficulty<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub difficulty: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentTauntCategory<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub messages: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentNewMarkerItem<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub patch_marker: HipStr<'a>,
    #[serde(borrow)]
    pub indicator_id: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentNewMarkerTab<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub patch_marker: HipStr<'a>,
    #[serde(borrow)]
    pub tab_name: HipStr<'a>,
    pub clean_rule: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentGameMode<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub context: HipStr<'a>,
    #[serde(borrow)]
    pub transition: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentDevice<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub scoring_type: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentWDFVoteChoice<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub vote: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentSoundNotification<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_notification_prefix: Option<HipStr<'a>>,
    // Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub module_name: Option<HipStr<'a>>,
    // Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub item_name: Option<HipStr<'a>>,
    // Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub focus_notification: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentCompletionDisplay<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub marker: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FontEffectList<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub effects: Vec<FontEffect<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FontEffect<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(rename = "type")]
    pub type_it: u32,
    pub fadein_start: u32,
    pub fadein_end: u32,
    pub fadeout_start: i32,
    pub fadeout_end: i32,
    pub speed_min: u32,
    pub speed_max: u32,
    #[serde(rename = "static")]
    pub static_it: u32,
    pub static_seed: u32,
    pub min: u32,
    pub max: u32,
    pub limit: u32,
    pub value: u32,
    pub alpha_left: u32,
    pub alpha_mid_left: u32,
    pub length_left: u32,
    pub alpha_right: u32,
    pub alpha_mid_right: u32,
    pub length_right: u32,
    pub alpha_top: f32,
    pub alpha_mid_top: u32,
    pub length_top: u32,
    pub alpha_bottom: f32,
    pub alpha_mid_bottom: u32,
    pub length_bottom: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FTUESteps<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub ftue_steps_descs: Vec<FTUEStepDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FTUEStepDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub step_name: HipStr<'a>,
    #[serde(borrow)]
    pub step_done_objective_id: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct TRCLocalisation<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "TRCLocalisationList")]
    pub trc_localisation_list: Vec<TRCLocalisationDetail<'a>>,
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub popups_illustrations: HashMap<HipStr<'a>, HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TRCLocalisationDetail<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "TRCError")]
    pub trc_error: u32,
    /// Not in 2017 or earlier
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub illustration_id: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub title: SmartLocId<'a>,
    #[serde(borrow)]
    pub message: SmartLocId<'a>,
    #[serde(borrow)]
    pub button_left: SmartLocId<'a>,
    /// Not in 2017 or earlier
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub button_middle: Option<SmartLocId<'a>>,
    #[serde(borrow)]
    pub button_right: SmartLocId<'a>,
    /// Not in 2018 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_button: Option<u32>,
    /// Not in 2018 or later
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub message_text: Option<HipStr<'a>>,
    /// Not in 2018 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<u32>,
    /// Not in 2018 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub button_left_id: Option<u32>,
    /// Not in 2018 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub button_right_id: Option<u32>,
    /// Not in 2018 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SmartLocId<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub loc_id: u32,
    #[serde(borrow)]
    pub default_text: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ObjectivesDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub objective_descs: HashMap<HipStr<'a>, ObjectiveDescriptor<'a>>,
}

impl ObjectivesDatabase<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_ObjectivesDatabase_Template");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum ObjectiveDescriptor<'a> {
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_AccumulateXCal")]
    AccumulateXCal(ObjectiveDescriptorAccumulateXCal<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_AccumulateXMoves")]
    AccumulateXMoves(ObjectiveDescriptorAccumulateXMoves<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_ActivateCoopMode")]
    ActivateCoopMode(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_AddXSongsToAPlaylist")]
    AddXSongsToAPlaylist(ObjectiveDescriptorAddXSongsToAPlaylist<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_BeatWDFBoss")]
    BeatWDFBoss(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_ChangeCustoItemXTimes")]
    ChangeCustoItemXTimes(ObjectiveDescriptorChangeCustoItemXTimes<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_CompleteXQuests")]
    CompleteXQuests(ObjectiveDescriptorCompleteXQuests<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_DanceXSeconds")]
    DanceXSeconds(ObjectiveDescriptorDanceXSeconds<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_FinishXPlaylist")]
    FinishXPlaylist(ObjectiveDescriptorFinishXPlaylist<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_GatherXStars")]
    GatherXStars(ObjectiveDescriptorGatherXStars<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_LinkedToUplay")]
    LinkedToUplay(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_OpenAnthologyMode")]
    OpenAnthologyMode(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_OpenPostcardsGallery")]
    OpenPostcardsGallery(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_OpenStickerAlbum")]
    OpenStickerAlbum(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_OpenVideoGallery")]
    OpenVideoGallery(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayDailyQuestsForXDays")]
    PlayDailyQuestsForXDays(ObjectiveDescriptorPlayDailyQuestsForXDays<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayGachaXTimes")]
    PlayGachaXTimes(ObjectiveDescriptorPlayGachaXTimes<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayPreviousJD")]
    PlayPreviousJD(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayWDFTournament")]
    PlayWDFTournament(ObjectiveDescriptorPlayWDFTournament<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayXMaps")]
    PlayXMaps(ObjectiveDescriptorPlayXMaps<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayXWDFTournamentRounds")]
    PlayXWDFTournamentRounds(ObjectiveDescriptorPlayXWDFTournamentRounds<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_ReachRankX")]
    ReachRankX(ObjectiveDescriptorReachRankX<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_RenewJDUSub")]
    RenewJDUSub(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_SwitchSweatMode")]
    SwitchSweatMode(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_UnlockUplayRewardAliasPack1")]
    UnlockUplayRewardAliasPack1(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_UnlockUplayRewardAliasPack2")]
    UnlockUplayRewardAliasPack2(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_UnlockXPortraitBorders")]
    UnlockXPortraitBorders(ObjectiveDescriptorUnlockXPortraitBorders<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_UnlockXStickers")]
    UnlockXStickers(ObjectiveDescriptorUnlockXStickers<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_WinWDFTeamBattle")]
    WinWDFTeamBattle(ObjectiveDescriptorBase<'a>),
}

impl ObjectiveDescriptor<'_> {
    #[must_use]
    pub const fn description(&self) -> LocaleId {
        match self {
            Self::ActivateCoopMode(data)
            | Self::BeatWDFBoss(data)
            | Self::LinkedToUplay(data)
            | Self::OpenPostcardsGallery(data)
            | Self::OpenStickerAlbum(data)
            | Self::OpenVideoGallery(data)
            | Self::OpenAnthologyMode(data)
            | Self::PlayPreviousJD(data)
            | Self::RenewJDUSub(data)
            | Self::SwitchSweatMode(data)
            | Self::UnlockUplayRewardAliasPack2(data)
            | Self::WinWDFTeamBattle(data)
            | Self::UnlockUplayRewardAliasPack1(data) => data.description,
            Self::AccumulateXCal(data) => data.description,
            Self::AccumulateXMoves(data) => data.description,
            Self::AddXSongsToAPlaylist(data) => data.description,
            Self::ChangeCustoItemXTimes(data) => data.description,
            Self::CompleteXQuests(data) => data.description,
            Self::DanceXSeconds(data) => data.description,
            Self::FinishXPlaylist(data) => data.description,
            Self::GatherXStars(data) => data.description,
            Self::PlayDailyQuestsForXDays(data) => data.description,
            Self::PlayGachaXTimes(data) => data.description,
            Self::PlayWDFTournament(data) => data.description,
            Self::PlayXMaps(data) => data.description,
            Self::PlayXWDFTournamentRounds(data) => data.description,
            Self::ReachRankX(data) => data.description,
            Self::UnlockXPortraitBorders(data) => data.description,
            Self::UnlockXStickers(data) => data.description,
        }
    }

    #[must_use]
    pub const fn is_static(&self) -> bool {
        match self {
            Self::ActivateCoopMode(data)
            | Self::BeatWDFBoss(data)
            | Self::LinkedToUplay(data)
            | Self::OpenAnthologyMode(data)
            | Self::OpenPostcardsGallery(data)
            | Self::OpenStickerAlbum(data)
            | Self::OpenVideoGallery(data)
            | Self::PlayPreviousJD(data)
            | Self::RenewJDUSub(data)
            | Self::SwitchSweatMode(data)
            | Self::UnlockUplayRewardAliasPack2(data)
            | Self::WinWDFTeamBattle(data)
            | Self::UnlockUplayRewardAliasPack1(data) => data.is_static,
            Self::AccumulateXCal(data) => data.is_static,
            Self::AccumulateXMoves(data) => data.is_static,
            Self::AddXSongsToAPlaylist(data) => data.is_static,
            Self::ChangeCustoItemXTimes(data) => data.is_static,
            Self::CompleteXQuests(data) => data.is_static,
            Self::DanceXSeconds(data) => data.is_static,
            Self::FinishXPlaylist(data) => data.is_static,
            Self::GatherXStars(data) => data.is_static,
            Self::PlayDailyQuestsForXDays(data) => data.is_static,
            Self::PlayGachaXTimes(data) => data.is_static,
            Self::PlayWDFTournament(data) => data.is_static,
            Self::PlayXMaps(data) => data.is_static,
            Self::PlayXWDFTournamentRounds(data) => data.is_static,
            Self::ReachRankX(data) => data.is_static,
            Self::UnlockXPortraitBorders(data) => data.is_static,
            Self::UnlockXStickers(data) => data.is_static,
        }
    }

    #[must_use]
    pub const fn exclude_from_upload(&self) -> bool {
        match self {
            Self::ActivateCoopMode(data)
            | Self::BeatWDFBoss(data)
            | Self::LinkedToUplay(data)
            | Self::OpenAnthologyMode(data)
            | Self::OpenPostcardsGallery(data)
            | Self::OpenStickerAlbum(data)
            | Self::OpenVideoGallery(data)
            | Self::PlayPreviousJD(data)
            | Self::RenewJDUSub(data)
            | Self::SwitchSweatMode(data)
            | Self::UnlockUplayRewardAliasPack2(data)
            | Self::WinWDFTeamBattle(data)
            | Self::UnlockUplayRewardAliasPack1(data) => data.exclude_from_upload,
            Self::AccumulateXCal(data) => data.exclude_from_upload,
            Self::AccumulateXMoves(data) => data.exclude_from_upload,
            Self::AddXSongsToAPlaylist(data) => data.exclude_from_upload,
            Self::ChangeCustoItemXTimes(data) => data.exclude_from_upload,
            Self::CompleteXQuests(data) => data.exclude_from_upload,
            Self::DanceXSeconds(data) => data.exclude_from_upload,
            Self::FinishXPlaylist(data) => data.exclude_from_upload,
            Self::GatherXStars(data) => data.exclude_from_upload,
            Self::PlayDailyQuestsForXDays(data) => data.exclude_from_upload,
            Self::PlayGachaXTimes(data) => data.exclude_from_upload,
            Self::PlayWDFTournament(data) => data.exclude_from_upload,
            Self::PlayXMaps(data) => data.exclude_from_upload,
            Self::PlayXWDFTournamentRounds(data) => data.exclude_from_upload,
            Self::ReachRankX(data) => data.exclude_from_upload,
            Self::UnlockXPortraitBorders(data) => data.exclude_from_upload,
            Self::UnlockXStickers(data) => data.exclude_from_upload,
        }
    }
}

impl<'a> ObjectiveDescriptor<'a> {
    #[must_use]
    pub fn description_raw(&self) -> HipStr<'a> {
        match self {
            Self::ActivateCoopMode(data)
            | Self::BeatWDFBoss(data)
            | Self::LinkedToUplay(data)
            | Self::OpenPostcardsGallery(data)
            | Self::OpenStickerAlbum(data)
            | Self::OpenVideoGallery(data)
            | Self::OpenAnthologyMode(data)
            | Self::PlayPreviousJD(data)
            | Self::RenewJDUSub(data)
            | Self::SwitchSweatMode(data)
            | Self::UnlockUplayRewardAliasPack2(data)
            | Self::WinWDFTeamBattle(data)
            | Self::UnlockUplayRewardAliasPack1(data) => data.description_raw.clone(),
            Self::AccumulateXCal(data) => data.description_raw.clone(),
            Self::AccumulateXMoves(data) => data.description_raw.clone(),
            Self::AddXSongsToAPlaylist(data) => data.description_raw.clone(),
            Self::ChangeCustoItemXTimes(data) => data.description_raw.clone(),
            Self::CompleteXQuests(data) => data.description_raw.clone(),
            Self::DanceXSeconds(data) => data.description_raw.clone(),
            Self::FinishXPlaylist(data) => data.description_raw.clone(),
            Self::GatherXStars(data) => data.description_raw.clone(),
            Self::PlayDailyQuestsForXDays(data) => data.description_raw.clone(),
            Self::PlayGachaXTimes(data) => data.description_raw.clone(),
            Self::PlayWDFTournament(data) => data.description_raw.clone(),
            Self::PlayXMaps(data) => data.description_raw.clone(),
            Self::PlayXWDFTournamentRounds(data) => data.description_raw.clone(),
            Self::ReachRankX(data) => data.description_raw.clone(),
            Self::UnlockXPortraitBorders(data) => data.description_raw.clone(),
            Self::UnlockXStickers(data) => data.description_raw.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorBase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorAccumulateXCal<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub calories_amount: u32,
    pub in_one_session: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorAccumulateXMoves<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub moves_count: u32,
    pub categories_to_count: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorAddXSongsToAPlaylist<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub songs_added_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorChangeCustoItemXTimes<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub custo_item_changes_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorCompleteXQuests<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub quests_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorDanceXSeconds<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub dance_time: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorFinishXPlaylist<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub playlists_play_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorGatherXStars<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub stars_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayDailyQuestsForXDays<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    #[serde(rename = "consecutiveDays")]
    pub consecutive_days: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayGachaXTimes<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub gacha_plays_count: u32,
    pub unlock_all_acceptable_gacha_items: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayWDFTournament<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    // Not in nx2020
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tournament_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayXMaps<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub maps_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayXWDFTournamentRounds<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub rounds_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorReachRankX<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    #[serde(rename = "rankToReach")]
    pub rank_to_reach: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorUnlockXPortraitBorders<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub portrait_border_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorUnlockXStickers<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub stickers_count: u32,
    // Not in nx2020
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_stickers: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum ObjectiveDescriptorComponent<'a> {
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_CustoItemTypeRequirement"
    )]
    CustoItemTypeRequirement(ObjectiveDescriptorComponentCustoItemTypeRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_GachaItemTypeRequirement"
    )]
    GachaItemTypeRequirement(ObjectiveDescriptorComponentGachaItemTypeRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_MapCoachCountRequirement"
    )]
    MapCoachCountRequirement(ObjectiveDescriptorComponentMapCoachCountRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_MapLaunchLocationRequirement"
    )]
    MapLaunchLocationRequirement(ObjectiveDescriptorComponentMapLaunchLocationRequirement<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptorComponent_MapMovesRequirement")]
    MapMovesRequirement(ObjectiveDescriptorComponentMapMovesRequirement<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptorComponent_MapNameRequirement")]
    MapNameRequirement(ObjectiveDescriptorComponentMapNameRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_MapPlaymodeRequirement"
    )]
    MapPlaymodeRequirement(ObjectiveDescriptorComponentMapPlaymodeRequirement<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptorComponent_MapScoreRequirement")]
    MapScoreRequirement(ObjectiveDescriptorComponentMapScoreRequirement<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptorComponent_MapTagsRequirement")]
    MapTagsRequirement(ObjectiveDescriptorComponentMapTagsRequirement<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptorComponent_OnlyOnline")]
    OnlyOnline(ObjectiveDescriptorComponentBase<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_OnlyOnUnlimitedSongs"
    )]
    OnlyOnUnlimitedSongs(ObjectiveDescriptorComponentBase<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_PlaylistIdRequirement"
    )]
    PlaylistIdRequirement(ObjectiveDescriptorComponentPlaylistIdRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_ScoringModeRequirement"
    )]
    ScoringModeRequirement(ObjectiveDescriptorComponentScoringModeRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_SearchLabelsRequirement"
    )]
    SearchLabelsRequirement(ObjectiveDescriptorComponentSearchLabelsRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_StickerIdRequirement"
    )]
    StickerIdRequirement(ObjectiveDescriptorComponentStickerIdRequirement<'a>),
}

impl ObjectiveDescriptorComponent<'_> {
    #[must_use]
    pub const fn only_diff_values(&self) -> bool {
        match self {
            Self::CustoItemTypeRequirement(data) => data.only_diff_values,
            Self::GachaItemTypeRequirement(data) => data.only_diff_values,
            Self::MapCoachCountRequirement(data) => data.only_diff_values,
            Self::MapLaunchLocationRequirement(data) => data.only_diff_values,
            Self::MapMovesRequirement(data) => data.only_diff_values,
            Self::MapNameRequirement(data) => data.only_diff_values,
            Self::MapPlaymodeRequirement(data) => data.only_diff_values,
            Self::MapScoreRequirement(data) => data.only_diff_values,
            Self::MapTagsRequirement(data) => data.only_diff_values,
            Self::OnlyOnline(data) | Self::OnlyOnUnlimitedSongs(data) => data.only_diff_values,
            Self::PlaylistIdRequirement(data) => data.only_diff_values,
            Self::ScoringModeRequirement(data) => data.only_diff_values,
            Self::SearchLabelsRequirement(data) => data.only_diff_values,
            Self::StickerIdRequirement(data) => data.only_diff_values,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentBase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentCustoItemTypeRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_custo_item_types: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentGachaItemTypeRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_gacha_item_types: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapCoachCountRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_coach_counts: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapLaunchLocationRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_launch_contexts: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_launch_subcontexts: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapMovesRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub exact_moves_count: u32,
    pub min_moves_count: u32,
    pub max_moves_count: u32,
    pub all_map_moves_count: bool,
    pub only_map_last_move: bool,
    pub moves_in_a_row: bool,
    pub acceptable_categories: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapNameRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_map_names: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapPlaymodeRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub classic: bool,
    pub coop: bool,
    pub sweat: bool,
    pub playlist: bool,
    #[serde(rename = "WDF")]
    pub wdf: bool,
    pub kids: bool,
    // Not present in nx2021 or nx2022
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anthology: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapScoreRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub score: u32,
    pub better_than_dancer_of_the_week: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapTagsRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_map_tags: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub unacceptable_map_tags: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentPlaylistIdRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    #[serde(borrow)]
    pub acceptable_playlist_ids: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentScoringModeRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_scoring_modes: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentSearchLabelsRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_label_loc_ids: Vec<LocaleId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forbidden_label_loc_ids: Vec<LocaleId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentStickerIdRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_sticker_ids: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PadRumbleManager<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub rumbles: Vec<PadRumble<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PadRumble<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub intensity: f32,
    pub duration: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PortraitBordersDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub portrait_borders: Vec<PortraitBorderDesc<'a>>,
}

impl PortraitBordersDatabase<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_PortraitBordersDatabase_Template");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PortraitBorderDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub portrait_border_id: u32,
    #[serde(borrow)]
    pub background_texture_path: HipStr<'a>,
    #[serde(borrow)]
    pub foreground_texture_path: HipStr<'a>,
    #[serde(borrow)]
    pub background_phone_path: HipStr<'a>,
    #[serde(borrow)]
    pub foreground_phone_path: HipStr<'a>,
    pub original_lock_status: u32,
    pub visibility: u32,
}

impl PortraitBorderDesc<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_PortraitBorderDesc");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct QuickplayRules<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub rules_by_priority_order: Vec<QuickplayRule<'a>>,
    #[serde(
        borrow,
        rename = "DEBUG_TEST_LIST",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub debug_test_list: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum QuickplayRule<'a> {
    #[serde(
        borrow,
        rename = "JD_QuickplayRule_EnforceEasyMapAfterModerateMapsSequence"
    )]
    EnforceEasyMapAfterModerateMapsSequence(QuickplayRuleBase<'a>),
    #[serde(borrow, rename = "JD_QuickplayRule_EnforceEasyMapAtStart")]
    EnforceEasyMapAtStart(QuickplayRuleEnforceEasyMapAtStart<'a>),
    #[serde(borrow, rename = "JD_QuickplayRule_ForbidExtremeMapsSequence")]
    ForbidExtremeMapsSequence(QuickplayRuleBase<'a>),
    #[serde(borrow, rename = "JD_QuickplayRule_ForbidIntenseMapsSequence")]
    ForbidIntenseMapsSequence(QuickplayRuleBase<'a>),
    #[serde(borrow, rename = "JD_QuickplayRule_PromoteNeverPlayedMaps")]
    PromoteNeverPlayedMaps(QuickplayRuleBase<'a>),
    #[serde(borrow, rename = "JD_QuickplayRule_PromoteNeverPlayedTaggedMaps")]
    PromoteNeverPlayedTaggedMaps(QuickplayRulePromoteNeverPlayedTaggedMaps<'a>),
    #[serde(
        borrow,
        rename = "JD_QuickplayRule_PromoteSameNumberOfCoachesAsNumberOfPlayers"
    )]
    PromoteSameNumberOfCoachesAsNumberOfPlayers(QuickplayRuleBase<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct QuickplayRuleBase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub max_number_of_previous_maps_to_consider: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct QuickplayRuleEnforceEasyMapAtStart<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub max_number_of_previous_maps_to_consider: u32,
    #[serde(rename = "acceptableSweatDifficulties")]
    pub acceptable_sweat_difficulties: Vec<u32>,
    #[serde(rename = "acceptableSongDifficulties")]
    pub acceptable_song_difficulties: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct QuickplayRulePromoteNeverPlayedTaggedMaps<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub max_number_of_previous_maps_to_consider: u32,
    #[serde(borrow, rename = "acceptableTags")]
    pub acceptable_tags: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScheduledQuestDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub scheduled_quests: Vec<ScheduledQuestDesc<'a>>,
}

impl ScheduledQuestDatabase<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_ScheduledQuestDatabase_Template");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ScheduledQuestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Type")]
    pub type_it: u8,
    pub unlimited_only: bool,
    pub mojo_reward: u32,
    pub probability_weight: u32,
    pub objective_id: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub preconditions_objectives_id: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<HipStr<'a>>,
}

impl ScheduledQuestDesc<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_ScheduledQuestDesc");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SoundConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub busses: Vec<BusDef<'a>>,
    #[serde(borrow)]
    pub limiters: Vec<LimiterDef<'a>>,
    pub pause_fade_in: f32,
    pub pause_fade_out: f32,
    #[serde(borrow)]
    pub headphone_bus_mix: BusMix<'a>,
    #[serde(borrow)]
    pub music_tracks: Vec<MusicTrackDef<'a>>,
    #[serde(borrow, rename = "PCMSources")]
    pub pcm_sources: Vec<PCMSourceDef<'a>>,
    #[serde(borrow)]
    pub metronome_debug_sound_bar: HipStr<'a>,
    #[serde(borrow)]
    pub metronome_debug_sound_beat: HipStr<'a>,
    #[serde(borrow, rename = "TestSounds")]
    pub test_sounds: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub bus_mix_list: Vec<EventBusMix<'a>>,
    #[serde(borrow)]
    pub bus_fade_list: Vec<EventBusFade<'a>>,
    #[serde(borrow)]
    pub soundwich_synth: HipStr<'a>,
    #[serde(borrow)]
    pub soundwich_modules: Vec<HipStr<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub project_fade_curves: Option<Vec<ProjectFadeCurve<'a>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BusDef<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<HipStr<'a>>,
    pub volume: i32,
    pub filter_frequency: u32,
    pub filter_type: u32,
    pub out_devices: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LimiterDef<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub max_instances: u32,
    pub mode: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BusMix<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub priority: u32,
    pub duration: f32,
    pub fade_in: f32,
    pub fade_out: f32,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_mix: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub bus_defs: Vec<BusDef<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MusicTrackDef<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub track_name: HipStr<'a>,
    #[serde(borrow)]
    pub bus_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PCMSourceDef<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub nb_channels: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EventBusMix<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sender: u32,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub activate: u32,
    #[serde(borrow)]
    pub bus_mix: BusMix<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EventBusFade<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sender: u32,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(borrow)]
    pub bus: HipStr<'a>,
    pub time: f32,
    pub fade_in: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProjectFadeCurve<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub fade_type: u32,
    pub decibel_volume_for_curve_at_start: i32,
    pub decibel_volume_for_curve_at_middle: i32,
    pub decibel_volume_for_curve_at_end: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UITextManager<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub text_icons: HashMap<HipStr<'a>, TextIcon<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub text_icons_phone: HashMap<HipStr<'a>, HipStr<'a>>,
    // Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub actor_icons: Option<Vec<ActorIcon<'a>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum TextIcon<'a> {
    #[serde(borrow, rename = "TextIcon_Default")]
    TextIconDefault(TextIconDefault<'a>),
    #[serde(borrow, rename = "TextIcon_Shortcut")]
    TextIconShortcut(TextIconShortcut<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TextIconDefault<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub data: IconActorData<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TextIconShortcut<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub datas: HashMap<HipStr<'a>, IconActorData<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct IconActorData<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub path: HipStr<'a>,
    pub font_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActorIcon<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub icon_name: HipStr<'a>,
    #[serde(borrow)]
    pub icon_path: HipStr<'a>,
    pub font_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VibrationManager<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub vib_files_paths: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ZInputConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub inputs: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ZInputManager<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(borrow)]
    pub config: HipStr<'a>,
    pub category: u32,
    #[serde(borrow)]
    pub actions: Vec<ZAction<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ZAction<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub inverted: u32,
    pub scale: u32,
    #[serde(borrow)]
    pub input: Vec<ZInput<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ZInput<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub control: HipStr<'a>,
    #[serde(borrow)]
    pub query: HipStr<'a>,
    pub axis_range: (f32, f32),
    pub threshold: u32,
    pub delay: f32,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct AnthologyConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub sku_map_name: HipStr<'a>,
    pub planets_and_rocket_translation_speed: f32,
    pub ribbon_discovering_speed: f32,
    pub background_squares_translation_speed: f32,
    #[serde(borrow)]
    pub intro_video_path: HipStr<'a>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub progression_videos_paths: HashMap<u32, HipStr<'a>>,
    #[serde(borrow)]
    pub outro_video_path: HipStr<'a>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub opus_textures_paths: HashMap<u32, HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StatsContainer<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub stats_events: Vec<StatsEvent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StatsEvent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub stats_event_name: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub stats_event_parameters: Vec<StatParameter<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub stats_event_user_stats: Vec<UserStat<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StatParameter<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub parameter_name: HipStr<'a>,
    pub parameter_type: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_value: Vec<VarType<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UserStat<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub user_stat_name: HipStr<'a>,
    pub user_stat_behaviour: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_stat_used_on_xbox_one: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub user_stat_parameters: Vec<StatParameter<'a>>,
    #[serde(borrow)]
    pub parameter_used_to_update_value: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RewardContainer<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub rewards: Vec<RewardDetail<'a>>,
    pub is_silent: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RewardDetail<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub id: HipStr<'a>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<HipStr<'a>>,
    pub platform_id: u32,
    #[serde(borrow)]
    pub event_trigger: HipStr<'a>,
    pub has_to_be_checked_in_game: u32,
    pub uplay_id: i32,
    /// Not in 2016
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplay_tag: Option<u32>,
    #[serde(rename = "uplayXP", default, skip_serializing_if = "Option::is_none")]
    pub uplay_xp: Option<u32>,
    pub uplay_points_value: u32,
    #[serde(rename = "uplayLocID")]
    pub uplay_loc_id: u32,
    pub has_no_reward: u32,
    #[serde(
        borrow,
        rename = "REWARD_TRIGGER",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reward_trigger: Vec<RewardTriggerSum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RewardTriggerSum<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub user_stat_name: HipStr<'a>,
    pub amount_to_get: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub values_to_check: Vec<StatParameter<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct VarType<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "type")]
    pub type_it: u32,
    pub var: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PostcardsDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub postcards: Vec<PostcardDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PostcardDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub postcard_id: u32,
    pub texture_thumbnail: HipStr<'a>,
    pub texture_fullscreen: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DanceMachineConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub episodes: Vec<DMEpisodeDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DMEpisodeDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "episodeID")]
    pub episode_id: HipStr<'a>,
    pub default_status: HipStr<'a>,
    pub title_loc: u32,
    pub blocks: Vec<HipStr<'a>>,
}
