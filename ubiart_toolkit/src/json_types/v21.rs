use std::collections::HashMap;

use hipstr::HipStr;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use ubiart_toolkit_shared_types::errors::ParserError;

#[cfg(feature = "full_json_types")]
use super::{
    frt::FeedbackFXManager,
    isg::{
        AchievementsDatabase, CameraShakeConfig, CarouselManager, CarouselRules, FTUESteps,
        FontEffectList, GachaContentDatabase, PadRumbleManager, QuickplayRules, SoundConfig,
        TRCLocalisation, UITextManager, VibrationManager, WDFLinearRewards, ZInputConfig,
        ZInputManager,
    },
    msh::GFXMaterialShader,
    tfn::FontTemplate,
    Empty,
};
use super::{
    isg::{
        AutoDanceEffectData, CarouselElementDesc, ClubRewardConfig, CollectibleAlbum, CountryEntry,
        CustomizableItemConfig, GachaConfig, GridActorsToPreload, HomeDataConfig, HomeDataTipEntry,
        ItemColorLookUp, LayoutTabbedGrids, LocalAliases, MenuAssetsCacheParams,
        MenuMultiTrackItem, MenuMusicConfig, MenuMusicParams, ObjectivesDatabase,
        OnFlyNotificationTypeParams, PlaylistDatabase, PopupConfigList, PortraitBordersDatabase,
        RankDescriptor, RecapConfig, RemoteSoundParams, ScheduledQuestDatabase,
        ScheduledQuestSetup, ScoringCameraParams, ScoringMovespaceParams, ScoringParams,
        ShortcutDesc1719, SongsSearchTags, StickerEntry, TutorialContent, TutorialDesc,
        UnlimitedUpsellSongList, UnlimitedUpsellSubtitles, UplayReward, WDFBossEntry,
        WhatsNewConfigs,
    },
    AliasesObjectives, AvatarsObjectives, MapsGoals, MapsObjectives, OfflineRecommendation,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "__class")]
pub enum Template21<'a> {
    #[serde(borrow, rename = "JD_GameManagerConfig_Template")]
    GameManagerConfig(Box<GameManagerConfig21<'a>>),
    #[serde(borrow, rename = "JD_LocalAliases")]
    LocalAliases(LocalAliases<'a>),
    #[serde(borrow, rename = "JD_ObjectivesDatabase_Template")]
    ObjectivesDatabase(ObjectivesDatabase<'a>),
    #[serde(borrow, rename = "JD_PlaylistDatabase_Template")]
    PlaylistDatabase(PlaylistDatabase<'a>),
    #[serde(borrow, rename = "JD_PortraitBordersDatabase_Template")]
    PortraitBordersDatabase(PortraitBordersDatabase<'a>),
    #[serde(borrow, rename = "JD_ScheduledQuestDatabase_Template")]
    ScheduledQuestDatabase(ScheduledQuestDatabase<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "CameraShakeConfig_Template")]
    CameraShakeConfig(CameraShakeConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "CarouselManager_Template")]
    CarouselManager(CarouselManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ConvertedTmlTape_Template")]
    ConvertedTmlTape(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "DynamicMusicTrackComponent_Template")]
    DynamicMusicTrackComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FeedbackFXManager_Template")]
    FeedbackFXManager(FeedbackFXManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FontEffectList_Template")]
    FontEffectList(FontEffectList<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FontTemplate")]
    FontTemplate(FontTemplate<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "GFXMaterialShader_Template")]
    GFXMaterialShader(GFXMaterialShader<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_AchievementsDatabase_Template")]
    AchievementsDatabase(AchievementsDatabase<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_Carousel_Template")]
    Carousel(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CarouselRules")]
    CarouselRules(CarouselRules<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CreditsComponent_Template")]
    CreditsComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_FTUESteps_Template")]
    FTUESteps(FTUESteps<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_GachaComponent_Template")]
    GachaComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_GachaContentDatabase_Template")]
    GachaContentDatabase(GachaContentDatabase<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_Grid_CustomPatterned_Template")]
    GridCustomPatterned(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_Grid_RegularPatterned_Template")]
    GridRegularPatterned(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_LineGrid_Template")]
    LineGrid(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_NotificationBubble_Template")]
    NotificationBubble(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_NotificationBubblesPile_Template")]
    NotificationBubblesPile(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_QuickplayRules_Template")]
    QuickplayRules(QuickplayRules<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_SceneSpawnerComponent_Template")]
    SceneSpawnerComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_ScrollBarComponent_Template")]
    ScrollBarComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_ScrollingTextComponent_Template")]
    ScrollingTextComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_StickerGrid_Template")]
    StickerGrid(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_SubtitleComponent_Template")]
    SubtitleComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIAvatarUnlockWidget_Template")]
    UIAvatarUnlockWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudAutodanceRecorderComponent_Template")]
    UIHudAutodanceRecorderComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudCoopFeedbackComponent_Template")]
    UIHudCoopFeedbackComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudLyricsComponent_Template")]
    UIHudLyricsComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudPictoComponent_Template")]
    UIHudPictoComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudPictolineComponent_Template")]
    UIHudPictolineComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudPlayerComponent_Template")]
    UIHudPlayerComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineGaugeBarComponent_Template")]
    UIHudRacelineGaugeBarComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineGaugeComponent_Template")]
    UIHudRacelineGaugeComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineWDFBossComponent_Template")]
    UIHudRacelineWDFBossComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineWDFRankComponent_Template")]
    UIHudRacelineWDFRankComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineWDFTeamBattleComponent_Template")]
    UIHudRacelineWDFTeamBattleComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudStarvingComponent_Template")]
    UIHudStarvingComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudSweatCounter_Template")]
    UIHudSweatCounter(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudSweatTimer_Template")]
    UIHudSweatTimer(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudWDFIngameNotificationComponent_Template")]
    UIHudWDFIngameNotificationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIJoyconWidget_Template")]
    UIJoyconWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIMojoWidget_Template")]
    UIMojoWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UISaveWidget_Template")]
    UISaveWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIScheduledQuestComponent_Template")]
    UIScheduledQuestComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIUploadIcon_Template")]
    UIUploadIcon(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFBossSpawnerComponent_Template")]
    WDFBossSpawnerComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFLinearRewards")]
    WDFLinearRewards(WDFLinearRewards<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFTeamBattlePresentationComponent_Template")]
    WDFTeamBattlePresentationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFThemePresentationComponent_Template")]
    WDFThemePresentationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFUnlimitedFeedbackComponent_Template")]
    WDFUnlimitedFeedbackComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PadRumbleManager_Template")]
    PadRumbleManager(PadRumbleManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PropertyPatcher_Template")]
    PropertyPatcher(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "SoundConfig_Template")]
    SoundConfig(SoundConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TRCLocalisation_Template")]
    TRCLocalisation(TRCLocalisation<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UIAnchor_Template")]
    UIAnchor(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UIChangePage_Template")]
    UIChangePage(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UIComponent_Template")]
    UiComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UIControl_Template")]
    UIControl(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UICountdown_Template")]
    UICountdown(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UIPhoneData_Template")]
    UIPhoneData(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UIRootComponent_Template")]
    UIRootComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UIScreenComponent_Template")]
    UIScreenComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UITextManager_Template")]
    UITextManager(UITextManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "VibrationManager_Template")]
    VibrationManager(VibrationManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ZInputConfig_Template")]
    ZInputConfig(ZInputConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ZInputManager_Template")]
    ZInputManager(ZInputManager<'a>),
}

impl<'a> Template21<'a> {
    /// Convert this template to a `GameManagerConfig21`.
    pub fn into_game_manager_config(self) -> Result<GameManagerConfig21<'a>, ParserError> {
        if let Template21::GameManagerConfig(gmc) = self {
            Ok(*gmc)
        } else {
            Err(ParserError::custom(format!(
                "GameManagerConfig not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a `ObjectivesDatabase`.
    pub fn into_objectives_database(&'a self) -> Result<&'a ObjectivesDatabase<'a>, ParserError> {
        if let Template21::ObjectivesDatabase(objs_db) = self {
            Ok(objs_db)
        } else {
            Err(ParserError::custom(format!(
                "ObjectivesDatabase not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a `ScheduledQuestDatabase`.
    pub fn into_scheduled_quests_database(
        &'a self,
    ) -> Result<&'a ScheduledQuestDatabase<'a>, ParserError> {
        if let Template21::ScheduledQuestDatabase(sqst_db) = self {
            Ok(sqst_db)
        } else {
            Err(ParserError::custom(format!(
                "ScheduledQuestDatabase not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a `PlaylistDatabase`.
    pub fn into_playlists_database(&'a self) -> Result<&'a PlaylistDatabase<'a>, ParserError> {
        if let Template21::PlaylistDatabase(playlist_db) = self {
            Ok(playlist_db)
        } else {
            Err(ParserError::custom(format!(
                "PlaylistDatabase not found in template: {self:?}"
            )))
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GameManagerConfig21<'a> {
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
    #[serde(borrow)]
    pub anchor_tpl_path: HipStr<'a>,
    #[serde(borrow, rename = "songdb_scene")]
    pub songdb_scene: HipStr<'a>,
    #[serde(borrow, rename = "agingbot_behavioursTpl")]
    pub agingbot_behaviours_tpl: HipStr<'a>,
    #[serde(borrow, rename = "avatardb_scene")]
    pub avatardb_scene: HipStr<'a>,
    #[serde(borrow, rename = "skindb_scene")]
    pub skindb_scene: HipStr<'a>,
    #[serde(borrow, rename = "flagdb_scene")]
    pub flagdb_scene: HipStr<'a>,
    #[serde(borrow)]
    pub wdf_player_name_prefix_on_xbox_one: HipStr<'a>,
    #[serde(borrow)]
    pub wdf_player_name_prefix_on_stadia: Option<HipStr<'a>>,
    #[serde(borrow, rename = "wdfPlayerNamePrefixNonPS4")]
    pub wdf_player_name_prefix_non_ps4: HipStr<'a>,
    #[serde(borrow)]
    pub shortcut_descriptors: HashMap<HipStr<'a>, ShortcutDesc1719<'a>>,
    #[serde(borrow)]
    pub space_between_shortcuts: HipStr<'a>,
    pub max_controller_sleep_time: f32,
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
    #[serde(borrow)]
    pub actors_to_bundle: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub popupconfigs: PopupConfigList<'a>,
    #[serde(borrow)]
    pub clubrewardconfigs: HashMap<HipStr<'a>, ClubRewardConfig<'a>>,
    #[serde(borrow)]
    pub scoringparams: ScoringParams<'a>,
    #[serde(borrow)]
    pub scoringcameraparams: ScoringCameraParams<'a>,
    #[serde(borrow)]
    pub scoringmovespaceparams: ScoringMovespaceParams<'a>,
    #[serde(borrow)]
    pub menuassetsparams: Vec<MenuAssetsCacheParams<'a>>,
    #[serde(borrow)]
    pub menumusicsparams: HashMap<HipStr<'a>, MenuMusicParams<'a>>,
    #[serde(borrow)]
    pub remotesoundparams: HashMap<HipStr<'a>, RemoteSoundParams<'a>>,
    #[serde(borrow)]
    pub menu_music_multi_tracks: HashMap<HipStr<'a>, MenuMultiTrackItem<'a>>,
    #[serde(borrow)]
    pub menumusicconfig: MenuMusicConfig<'a>,
    #[serde(borrow)]
    pub rankdescriptor: RankDescriptor<'a>,
    #[serde(borrow)]
    pub unlimitedupsellsonglist: Vec<UnlimitedUpsellSongList<'a>>,
    #[serde(borrow, rename = "defaultJDUVideoPreviewSubtitles")]
    pub default_jdu_video_preview_subtitles: UnlimitedUpsellSubtitles<'a>,
    #[serde(borrow)]
    pub customizableitemconfig: CustomizableItemConfig<'a>,
    #[serde(borrow, rename = "scheduled_questSetup")]
    pub scheduled_quest_setup: ScheduledQuestSetup<'a>,
    #[serde(borrow)]
    pub countryentries: Vec<CountryEntry<'a>>,
    #[serde(borrow)]
    pub default_country_code: HipStr<'a>,
    #[serde(borrow)]
    pub credits_textbox_path: HipStr<'a>,
    #[serde(borrow)]
    pub countdown_delays: HashMap<HipStr<'a>, u32>,
    #[serde(borrow)]
    pub autodance_effects_list: Vec<AutoDanceEffectData<'a>>,
    #[serde(borrow)]
    pub autodance_transition_sound_path: HipStr<'a>,
    pub autodance_transition_sound_synchronise_sample: u32,
    #[serde(borrow)]
    pub tutorials_contents: HashMap<HipStr<'a>, TutorialContent<'a>>,
    #[serde(borrow)]
    pub tutorials: Vec<TutorialDesc<'a>>,
    #[serde(borrow)]
    pub redeem_maps: HashMap<HipStr<'a>, Vec<HipStr<'a>>>,
    #[serde(borrow, rename = "UplayRewards")]
    pub uplay_rewards: Vec<UplayReward<'a>>,
    pub stars_6th_step_song_score: u32,
    pub stars_7th_step_song_score: u32,
    pub perfect_feedback_min_score: u32,
    pub min_song_nb_for_shuffle: u32,
    #[serde(borrow)]
    pub wdf_boss_entries: Vec<WDFBossEntry<'a>>,
    #[serde(borrow)]
    pub itemcolorlookup: ItemColorLookUp<'a>,
    #[serde(borrow, rename = "defaultJDUVideoPreview")]
    pub default_jdu_video_preview: HipStr<'a>,
    #[serde(borrow, rename = "defaultJDUVideoPreview_kids")]
    pub default_jdu_video_preview_kids: HipStr<'a>,
    pub jd_points_per_star: Vec<u32>,
    #[serde(borrow)]
    pub banned_maps_in_chinese: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub collectiblealbum: CollectibleAlbum<'a>,
    #[serde(borrow)]
    pub stickerdatabase: Vec<StickerEntry<'a>>,
    #[serde(borrow)]
    pub gachaconfig: GachaConfig<'a>,
    #[serde(borrow)]
    pub config_files_path: ConfigFilesPath21<'a>,
    #[serde(borrow)]
    pub watermark: HipStr<'a>,
    #[serde(borrow)]
    pub grid_actors_to_preload: HashMap<HipStr<'a>, GridActorsToPreload<'a>>,
    #[serde(borrow)]
    pub grid_item_descriptors: HashMap<HipStr<'a>, CarouselElementDesc<'a>>,
    #[serde(borrow)]
    pub tabbed_grids_layout_descriptors: HashMap<HipStr<'a>, LayoutTabbedGrids<'a>>,
    #[serde(borrow)]
    pub home_data_config: HomeDataConfig<'a>,
    #[serde(borrow)]
    pub home_data_tips_config: Vec<HomeDataTipEntry<'a>>,
    #[serde(borrow)]
    pub home_data_default_article_thumbnail: HipStr<'a>,
    #[serde(borrow)]
    pub home_data_online_incentive_article_thumbnail: HipStr<'a>,
    #[serde(borrow, rename = "special_characters")]
    pub special_characters: Vec<HipStr<'a>>,
    #[serde(borrow, rename = "derived_letters")]
    pub derived_letters: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow, rename = "search_labels")]
    pub search_labels: SongsSearchTags<'a>,
    #[serde(borrow)]
    pub mapsobjectives: MapsObjectives<'a>,
    #[serde(borrow)]
    pub maps_goals: MapsGoals<'a>,
    pub legacy_alias_id: u32,
    #[serde(rename = "JDUAliasId")]
    pub jdu_alias_id: u32,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub avatarsobjectives: AvatarsObjectives<'a>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub aliasesobjectives: AliasesObjectives<'a>,
    #[serde(borrow)]
    pub specific_cases_check_order: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub on_fly_notification_types: HashMap<HipStr<'a>, OnFlyNotificationTypeParams<'a>>,
    #[serde(borrow)]
    pub bubbles_prioritized_notif_types_groups: Vec<Vec<HipStr<'a>>>,
    #[serde(borrow)]
    pub reward_screen_prioritized_notif_types: Vec<HipStr<'a>>,
    pub bubbles_pile_delay_before_exit: Vec<f32>,
    #[serde(borrow)]
    pub new_notification_tree: HashMap<HipStr<'a>, Vec<HipStr<'a>>>,
    #[serde(borrow)]
    pub recap_config: RecapConfig<'a>,
    #[serde(borrow)]
    pub offline_recommendation: OfflineRecommendation<'a>,
    #[serde(borrow, rename = "whats_new_configs")]
    pub whats_new_configs: WhatsNewConfigs<'a>,
    #[serde(borrow, rename = "wdf_linear_rewards_path")]
    pub wdf_linear_rewards_path: HipStr<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFilesPath21<'a> {
    #[serde(borrow)]
    pub gachacontent: HipStr<'a>,
    #[serde(borrow)]
    pub ftuesteps: HipStr<'a>,
    #[serde(borrow)]
    pub objectives: HipStr<'a>,
    #[serde(borrow)]
    pub playlist: HipStr<'a>,
    #[serde(borrow)]
    pub portraitborders: HipStr<'a>,
    #[serde(borrow)]
    pub quickplayrules: HipStr<'a>,
    #[serde(borrow)]
    pub scheduledquests: HipStr<'a>,
}
