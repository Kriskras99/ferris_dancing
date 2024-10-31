use std::collections::HashMap;

use hipstr::HipStr;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use ubiart_toolkit_shared_types::errors::ParserError;

use super::isg::{
    AutoDanceEffectData, ChallengerScoreEvolutionTemplate1619, ChatMessagesParams1618,
    CoopTweakedText17, CountryEntry, MenuAssetsCacheParams, MenuMusicConfig, MenuMusicParams,
    PopupConfigList, QuestChallengerEntry1618, QuestConfig1618, QuestEntry1617, RemoteSoundParams,
    ScoringCameraParams, ScoringParams, ShortcutSetup1619, SweatRandomizeConfig1619,
    TutorialContent, TutorialDesc, UnlimitedUpsellSongList,
};
#[cfg(feature = "full_json_types")]
use super::{
    frt::FeedbackFXManager,
    isg::{
        CameraShakeConfig, CarouselManager, CarouselRules, FontEffectList, PadRumbleManager,
        RewardContainer, SoundConfig, StatsContainer, TRCLocalisation, UITextManager, ZInputConfig,
        ZInputManager,
    },
    msh::GFXMaterialShader1618,
    tfn::FontTemplate,
    Empty,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "__class")]
pub enum Template16<'a> {
    #[serde(borrow, rename = "JD_GameManagerConfig_Template")]
    GameManagerConfig(Box<GameManagerConfig16<'a>>),
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
    #[serde(borrow, rename = "JD_CameraFeedComponent_Template")]
    CameraFeedComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CarouselRules")]
    CarouselRules(CarouselRules<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_ChannelZappingComponent_Template")]
    ChannelZappingComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CMU_GenericStage_Component_Template")]
    CMUGenericStageComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CreditsComponent_Template")]
    CreditsComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_PictoComponent_Template")]
    PictoComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_PleoInfoComponent_Template")]
    PleoInfoComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_RegistrationComponent_Template")]
    RegistrationComponent(Empty<'a>),
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
    #[serde(borrow, rename = "JD_UIHudLyricsFeedbackComponent_Template")]
    UIHudLyricsFeedbackComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudPlayerComponent_Template")]
    UIHudPlayerComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineCoopComponent_Template")]
    UIHudRacelineCoopComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineRivalBarComponent_Template")]
    UIHudRacelineRivalBarComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineRivalComponent_Template")]
    UIHudRacelineRivalComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudShowtimePhotoFeedbackComponent_Template")]
    UIHudShowtimePhotoFeedbackComponent(Empty<'a>),
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
    #[serde(borrow, rename = "JD_UIHudVumeterComponent_Template")]
    UIHudVumeterComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIMojoWidget_Template")]
    UIMojoWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UISaveWidget_Template")]
    UISaveWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIUplayNotification_Template")]
    UIUplayNotification(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIUploadIcon_Template")]
    UIUploadIcon(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIWidgetElement_Template")]
    UIWidgetElement(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PadRumbleManager_Template")]
    PadRumbleManager(PadRumbleManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PropertyPatcher_Template")]
    PropertyPatcher(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "RewardContainer_Template")]
    RewardContainer(RewardContainer<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "SoundConfig_Template")]
    SoundConfig(SoundConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "StatsContainer_Template")]
    StatsContainer(StatsContainer<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TexturePatcherComponent_Template")]
    TexturePatcherComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TRCLocalisation_Template")]
    TRCLocalisation(TRCLocalisation<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UIAnchor_Template")]
    UIAnchor(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UICarousel_Template")]
    UICarousel(Empty<'a>),
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
    #[serde(borrow, rename = "UIItemSlot_Template")]
    UIItemSlot(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UIItemTextField_Template")]
    UIItemTextField(Empty<'a>),
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
    #[serde(borrow, rename = "ViewportUIComponent_Template")]
    ViewportUIComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ZInputConfig_Template")]
    ZInputConfig(ZInputConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ZInputManager_Template")]
    ZInputManager(ZInputManager<'a>),
}

impl<'a> Template16<'a> {
    /// Convert this template to a `GameManagerConfig17`.
    pub fn into_game_manager_config(self) -> Result<GameManagerConfig16<'a>, ParserError> {
        if let Template16::GameManagerConfig(gmc) = self {
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
pub struct GameManagerConfig16<'a> {
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
    #[serde(borrow, rename = "flagdb_scene")]
    pub flagdb_scene: HipStr<'a>,
    #[serde(borrow)]
    pub avatar_folder: HipStr<'a>,
    #[serde(borrow)]
    pub song_tags: Vec<HipStr<'a>>,
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
    pub genericstages: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow)]
    pub popupconfigs: PopupConfigList<'a>,
    #[serde(borrow)]
    pub scoringparams: ScoringParams<'a>,
    #[serde(borrow)]
    pub kinect_scoringparams: ScoringCameraParams<'a>,
    #[serde(borrow)]
    pub menuassetsparams: Vec<MenuAssetsCacheParams<'a>>,
    #[serde(borrow)]
    pub menumusicsparams: HashMap<HipStr<'a>, MenuMusicParams<'a>>,
    #[serde(borrow)]
    pub remotesoundparams: HashMap<HipStr<'a>, RemoteSoundParams<'a>>,
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
    pub slave_phone_loc_ids: HashMap<HipStr<'a>, Vec<u32>>,
    #[serde(borrow)]
    pub questdataentries: Vec<QuestEntry1617<'a>>,
    #[serde(borrow)]
    pub unlimitedupsellsonglist: Vec<UnlimitedUpsellSongList<'a>>,
    #[serde(borrow)]
    pub questconfig: QuestConfig1618<'a>,
    #[serde(borrow)]
    pub questchallengerentries: Vec<QuestChallengerEntry1618<'a>>,
    #[serde(borrow)]
    pub sweatrandomizeconfig: SweatRandomizeConfig1619<'a>,
    #[serde(borrow)]
    pub challenger_evolution_template_list: Vec<ChallengerScoreEvolutionTemplate1619<'a>>,
    #[serde(borrow)]
    pub countryentries: Vec<CountryEntry<'a>>,
    #[serde(borrow)]
    pub credits_textbox_path: HipStr<'a>,
    pub avatar_min_anim_hud_duration: u32,
    #[serde(borrow)]
    pub b2b_maps: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub chatmessagesparams: ChatMessagesParams1618<'a>,
    #[serde(borrow)]
    pub chat_messages: HashMap<HipStr<'a>, Vec<u32>>,
    #[serde(borrow)]
    pub challenge_algo_order: Vec<HipStr<'a>>,
    pub challenge_friend_score_offset: f32,
    pub challenge_signature_score_offset: f32,
    pub challenger_connection_time_delay: f32,
    pub coop_score_diamonds_values: Vec<f32>,
    pub coop_jauge_anim_time: Vec<u32>,
    pub rival_recap_incr_score_speed: f32,
    pub retained_most_played_among_all_played_songs_ratio: f32,
    pub locked_songs_push_occurence_value: u32,
    pub unlimited_songs_push_occurence_value: u32,
    pub push_recap_min_played_song_count_before_unlimited: u32,
    #[serde(borrow)]
    pub countdown_delays: HashMap<HipStr<'a>, u32>,
    #[serde(borrow)]
    pub autodance_effects_list: Vec<AutoDanceEffectData<'a>>,
    #[serde(borrow)]
    pub coop_tweaked_texts: Vec<CoopTweakedText17<'a>>,
    #[serde(borrow)]
    pub messages_slides: HashMap<HipStr<'a>, TutorialContent<'a>>,
    #[serde(borrow)]
    pub tutorials: Vec<TutorialDesc<'a>>,
    #[serde(borrow)]
    pub redeem_maps: HashMap<HipStr<'a>, Vec<HipStr<'a>>>,
    #[serde(borrow)]
    pub uplay_unlockable_maps: HashMap<HipStr<'a>, u32>,
}
