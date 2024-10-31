use std::collections::HashMap;

use hipstr::HipStr;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use ubiart_toolkit_shared_types::errors::ParserError;

#[cfg(feature = "full_json_types")]
use super::{
    frt::FeedbackFXManager,
    isg::{
        CameraShakeConfig, CarouselManager, CarouselRules, FontEffectList, PadRumbleManager,
        SoundConfig, TRCLocalisation, UITextManager, ZInputConfig, ZInputManager,
    },
    msh::GFXMaterialShader1618,
    tfn::FontTemplate,
    Empty,
};
use super::{
    isg::{
        AdventureBossDesc18, AdventureModeSetup18, AdventureObjective18, AutoDanceEffectData,
        ChallengerScoreEvolutionTemplate1619, ChatMessagesParams1618, ClubRewardConfig,
        CollectibleAlbum, CountryEntry, CustomizableItemConfig, DanceMachineGlobalConfig1719,
        FTUEConfig, GachaConfig, HueConfig, ItemColorLookUp, MenuAssetsCacheParams,
        MenuMultiTrackItem, MenuMusicConfig, MenuMusicParams, PopupConfigList,
        QuestChallengerEntry1618, QuestConfig1618, RankDescriptor, RemoteSoundParams, RumbleConfig,
        ScheduledQuestSetup, ScoringCameraParams, ScoringMovespaceParams, ScoringParams,
        SearchConfig1719, ShortcutSetup1619, StickerEntry, SweatRandomizeConfig1619,
        SystemDescriptor18, TutorialContent, TutorialDesc, UnlimitedUpsellSongList, UplayReward,
        VideoLoopSetup, WDFBossEntry,
    },
    v1819::ScheduledQuestDesc1819,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "__class")]
pub enum Template18<'a> {
    #[serde(borrow, rename = "JD_GameManagerConfig_Template")]
    GameManagerConfig(Box<GameManagerConfig18<'a>>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "CameraShakeConfig_Template")]
    CameraShakeConfig(CameraShakeConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "CarouselManager_Template")]
    CarouselManager(CarouselManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ClearColorComponent_Template")]
    ClearColorComponent(Empty<'a>),
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
    GFXMaterialShader(GFXMaterialShader1618<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CarouselRules")]
    CarouselRules(CarouselRules<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CMU_GenericStage_Component_Template")]
    CMUGenericStageComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CreditsComponent_Template")]
    CreditsComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_GachaComponent_Template")]
    GachaComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_PleoInfoComponent_Template")]
    PleoInfoComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_RegistrationComponent_Template")]
    RegistrationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_SceneSpawnerComponent_Template")]
    SceneSpawnerComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIAvatarUnlockWidget_Template")]
    UIAvatarUnlockWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudCamerafeedComponent_Template")]
    UIHudCamerafeedComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudCommunityDancerCardComponent_Template")]
    UIHudCommunityDancerCardComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudDoubleScoringPlayerComponent_Template")]
    UIHudDoubleScoringPlayerComponent(Empty<'a>),
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
    #[serde(borrow, rename = "JD_UIHudProgressComponent_Template")]
    UIHudProgressComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineDM_Template")]
    UIHudRacelineDM(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineRivalBarComponent_Template")]
    UIHudRacelineRivalBarComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineRivalComponent_Template")]
    UIHudRacelineRivalComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineWDFRankComponent_Template")]
    UIHudRacelineWDFRankComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineWDFSpotlightComponent_Template")]
    UIHudRacelineWDFSpotlightComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineWDFTeamBattleComponent_Template")]
    UIHudRacelineWDFTeamBattleComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudSpotlightPlayerComponent_Template")]
    UIHudSpotlightPlayerComponent(Empty<'a>),
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
    #[serde(borrow, rename = "JD_UIHudVersusPlayerComponent_Template")]
    UIHudVersusPlayerComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudWDFIngameNotificationComponent_Template")]
    UIHudWDFIngameNotificationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIJDRankWidget_Template")]
    UIJDRankWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIMojoWidget_Template")]
    UIMojoWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIProfileStatWidget_Template")]
    UIProfileStatWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UISaveWidget_Template")]
    UISaveWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIScheduledQuestComponent_Template")]
    UIScheduledQuestComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UISkinUnlockWidget_Template")]
    UISkinUnlockWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIUplayNotification_Template")]
    UIUplayNotification(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIUploadIcon_Template")]
    UIUploadIcon(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFBossSpawnerComponent_Template")]
    WDFBossSpawnerComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFOnlineRankTransitionComponent_Template")]
    WDFOnlineRankTransitionComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFTeamBattleTransitionComponent_Template")]
    WDFTeamBattleTransitionComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFUnlimitedFeedbackComponent_Template")]
    WDFUnlimitedFeedbackComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PadRumbleManager_Template")]
    PadRumbleManager(PadRumbleManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "SoundConfig_Template")]
    SoundConfig(SoundConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TRCLocalisation_Template")]
    TRCLocalisation(TRCLocalisation<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UITextManager_Template")]
    UITextManager(UITextManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ZInputConfig_Template")]
    ZInputConfig(ZInputConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ZInputManager_Template")]
    ZInputManager(ZInputManager<'a>),
}

impl<'a> Template18<'a> {
    /// Convert this template to a `GameManagerConfig18`.
    pub fn into_game_manager_config(self) -> Result<GameManagerConfig18<'a>, ParserError> {
        if let Template18::GameManagerConfig(gmc) = self {
            Ok(*gmc)
        } else {
            Err(ParserError::custom(format!(
                "GameManagerConfig not found in template: {self:?}"
            )))
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GameManagerConfig18<'a> {
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
    pub avatar_folder: HipStr<'a>,
    #[serde(borrow)]
    pub pin_unplayed_song: HipStr<'a>,
    #[serde(borrow)]
    pub wdf_player_name_prefix_on_xbox_one: HipStr<'a>,
    #[serde(borrow, rename = "wdfPlayerNamePrefixNonPS4")]
    pub wdf_player_name_prefix_non_ps4: HipStr<'a>,
    #[serde(borrow)]
    pub short_cut_configs: HashMap<HipStr<'a>, ShortcutSetup1619<'a>>,
    #[serde(borrow)]
    pub default_phone_images: HashMap<HipStr<'a>, HipStr<'a>>,
    pub max_controller_sleep_time: f32,
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
    #[serde(borrow)]
    pub actors_to_bundle: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub genericstages: HashMap<HipStr<'a>, HipStr<'a>>,
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
    pub sweat_programs: Vec<u32>,
    #[serde(borrow)]
    pub mashupdates: HashMap<HipStr<'a>, u32>,
    #[serde(borrow)]
    pub mashupavatars: HashMap<HipStr<'a>, u32>,
    #[serde(borrow)]
    pub mojoprices: HashMap<HipStr<'a>, u32>,
    #[serde(borrow)]
    pub rankdescriptor: RankDescriptor<'a>,
    #[serde(borrow)]
    pub unlimitedupsellsonglist: Vec<UnlimitedUpsellSongList<'a>>,
    #[serde(borrow)]
    pub galaxyconfig: Vec<SystemDescriptor18<'a>>,
    #[serde(borrow)]
    pub adventure_bosses: Vec<AdventureBossDesc18<'a>>,
    #[serde(borrow, rename = "adventuremode_setup")]
    pub adventuremode_setup: AdventureModeSetup18<'a>,
    #[serde(borrow)]
    pub questconfig: QuestConfig1618<'a>,
    #[serde(borrow)]
    pub questchallengerentries: Vec<QuestChallengerEntry1618<'a>>,
    #[serde(borrow)]
    pub customizableitemconfig: CustomizableItemConfig<'a>,
    #[serde(borrow, rename = "scheduled_questSetup")]
    pub scheduled_quest_setup: ScheduledQuestSetup<'a>,
    #[serde(borrow)]
    pub dancemachineglobalconfig: DanceMachineGlobalConfig1719<'a>,
    #[serde(borrow)]
    pub sweatrandomizeconfig: SweatRandomizeConfig1619<'a>,
    #[serde(borrow)]
    pub searchconfig: SearchConfig1719<'a>,
    #[serde(borrow)]
    pub challenger_evolution_template_list: Vec<ChallengerScoreEvolutionTemplate1619<'a>>,
    #[serde(borrow)]
    pub countryentries: Vec<CountryEntry<'a>>,
    #[serde(borrow)]
    pub default_country_code: HipStr<'a>,
    #[serde(borrow)]
    pub credits_textbox_path: HipStr<'a>,
    pub avatar_min_anim_hud_duration: u32,
    #[serde(borrow)]
    pub b2b_maps: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub chatmessagesparams: ChatMessagesParams1618<'a>,
    #[serde(borrow)]
    pub chat_messages: HashMap<HipStr<'a>, Vec<u32>>,
    pub coop_score_diamonds_values: Vec<f32>,
    pub rival_recap_incr_score_speed: f32,
    #[serde(borrow)]
    pub countdown_delays: HashMap<HipStr<'a>, u32>,
    #[serde(borrow)]
    pub autodance_effects_list: Vec<AutoDanceEffectData<'a>>,
    #[serde(borrow)]
    pub autodance_transition_sound_path: HipStr<'a>,
    pub autodance_transition_sound_synchronise_sample: u32,
    pub autodance_transition_sound_synchronise_time: u32,
    #[serde(borrow)]
    pub messages_slides: HashMap<HipStr<'a>, TutorialContent<'a>>,
    #[serde(borrow)]
    pub tutorials: Vec<TutorialDesc<'a>>,
    #[serde(borrow)]
    pub redeem_maps: HashMap<HipStr<'a>, Vec<HipStr<'a>>>,
    #[serde(borrow, rename = "UplayRewards")]
    pub uplay_rewards: Vec<UplayReward<'a>>,
    pub stars_6th_step_song_score: u32,
    pub stars_6th_step_incoming_effect_start_relative_score: i32,
    pub stars_7th_step_song_score: u32,
    pub perfect_feedback_min_score: u32,
    pub min_song_nb_for_shuffle: u32,
    #[serde(borrow)]
    pub wdf_boss_entries: Vec<WDFBossEntry<'a>>,
    #[serde(borrow)]
    pub adventure_objectives: Vec<AdventureObjective18<'a>>,
    #[serde(borrow, rename = "scheduled_quests")]
    pub scheduled_quests: Vec<ScheduledQuestDesc1819<'a>>,
    #[serde(borrow)]
    pub itemcolorlookup: ItemColorLookUp<'a>,
    #[serde(borrow)]
    pub looped_video_config: HashMap<HipStr<'a>, VideoLoopSetup<'a>>,
    #[serde(borrow, rename = "defaultJDUVideoPreview")]
    pub default_jdu_video_preview: HipStr<'a>,
    #[serde(borrow, rename = "defaultJDUVideoPreview_kids")]
    pub default_jdu_video_preview_kids: HipStr<'a>,
    pub jd_points_per_star: Vec<u32>,
    #[serde(borrow)]
    pub banned_maps_in_chinese: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub hueconfig: HueConfig<'a>,
    #[serde(borrow)]
    pub collectiblealbum: CollectibleAlbum<'a>,
    #[serde(borrow)]
    pub stickerdatabase: Vec<StickerEntry<'a>>,
    #[serde(borrow)]
    pub gachaconfig: GachaConfig<'a>,
    #[serde(borrow)]
    pub ftueconfig: FTUEConfig<'a>,
    #[serde(borrow)]
    pub rumbleconfig: RumbleConfig<'a>,
    pub profile_landing_stats_thresholds: Vec<(u32, u32, u32)>,
    #[serde(borrow)]
    pub config_files_path: ConfigFilesPath18<'a>,
    pub news_update_interval: u32,
    pub new_update_pause_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFilesPath18<'a> {
    #[serde(borrow)]
    pub dmconfig: HipStr<'a>,
}
