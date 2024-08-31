use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use ubiart_toolkit_shared_types::errors::ParserError;

#[cfg(feature = "full_json_types")]
use super::{
    frt::FeedbackFXManager,
    isg::{
        CameraShakeConfig, CarouselManager, CarouselRules, FontEffectList, PadRumbleManager,
        RewardContainer, SoundConfig, StatsContainer, TRCLocalisation, UITextManager, ZInputConfig,
        ZInputManager,
    },
    just_dance::{AgingBotBehaviourAllTrees, FixedCameraComponent, SongDescription},
    msh::GFXMaterialShader1618,
    tfn::FontTemplate,
    tpl::{
        BezierTreeComponent, FxBankComponent, FxControllerComponent, ModeType, PleoComponent,
        PleoTextureGraphicComponent, SingleInstanceMesh3DComponent, TextureGraphicComponent,
        UITextBox,
    },
    Empty,
};
use super::{
    isg::{
        AutoDanceEffectData, ChallengerScoreEvolutionTemplate1619, ChatMessagesParams1618,
        CoopTweakedText17, CountryEntry, MenuAssetsCacheParams, MenuMusicConfig, MenuMusicParams,
        PopupConfigList, QuestChallengerEntry1618, QuestConfig1618, QuestEntry1617,
        RemoteSoundParams, ScoringCameraParams, ScoringParams, ShortcutSetup1619,
        SweatRandomizeConfig1619, TutorialContent, TutorialDesc, UnlimitedUpsellSongList,
    },
    just_dance::{AutodanceComponent, SongDatabase},
    tpl::{MasterTape, MaterialGraphicComponent, MusicTrackComponent, SoundComponent},
};
use crate::tpl::{AsyncPlayerDescTemplate, BlockFlowTemplate};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "__class")]
pub enum Template16<'a> {
    #[serde(borrow, rename = "Actor_Template")]
    Actor(Actor16<'a>),
    #[serde(borrow, rename = "JD_AutodanceComponent_Template")]
    AutodanceComponent(AutodanceComponent<'a>),
    #[serde(borrow, rename = "JD_AsyncPlayerDesc_Template")]
    AsyncPlayerDescTemplate(AsyncPlayerDescTemplate<'a>),
    #[serde(borrow, rename = "JD_AvatarDescTemplate")]
    AvatarDescription(AvatarDescription16<'a>),
    #[serde(borrow, rename = "JD_BlockFlowTemplate")]
    BlockFlowTemplate(BlockFlowTemplate<'a>),
    #[serde(borrow, rename = "JD_GameManagerConfig_Template")]
    GameManagerConfig(Box<GameManagerConfig16<'a>>),
    #[serde(borrow, rename = "JD_SongDatabaseTemplate")]
    SongDatabase(SongDatabase<'a>),
    #[serde(borrow, rename = "MasterTape_Template")]
    MasterTape(MasterTape<'a>),
    #[serde(borrow, rename = "MaterialGraphicComponent_Template")]
    MaterialGraphicComponent(MaterialGraphicComponent<'a>),
    #[serde(borrow, rename = "MusicTrackComponent_Template")]
    MusicTrackComponent(MusicTrackComponent<'a>),
    #[serde(borrow, rename = "SoundComponent_Template")]
    SoundComponent(SoundComponent<'a>),
    #[serde(borrow, rename = "TapeCase_Template")]
    TapeCase(MasterTape<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "BezierTreeComponent_Template")]
    BezierTreeComponent(BezierTreeComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "CameraGraphicComponent_Template")]
    CameraGraphicComponent(MaterialGraphicComponent<'a>),
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
    #[serde(borrow, rename = "FxBankComponent_Template")]
    FxBankComponent(FxBankComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FXControllerComponent_Template")]
    FxControllerComponent(FxControllerComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "GFXMaterialShader_Template")]
    GFXMaterialShader(GFXMaterialShader1618<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_AgingBot_BehaviourAllTrees")]
    AgingBotBehaviourAllTrees(AgingBotBehaviourAllTrees<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_BeatPulseComponent_Template")]
    BeatPulseComponent(ModeType<'a>),
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
    #[serde(borrow, rename = "JD_FixedCameraComponent_Template")]
    FixedCameraComponent(FixedCameraComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_GoldMoveComponent_Template")]
    GoldMoveComponent(ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_PictoComponent_Template")]
    PictoComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_PictoTimeline_Template")]
    PictoTimeline(ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_PleoInfoComponent_Template")]
    PleoInfoComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_RegistrationComponent_Template")]
    RegistrationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_SongDescTemplate")]
    SongDescription(SongDescription<'a>),
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
    #[serde(borrow, rename = "JD_UIWidgetGroupHUD_AutodanceRecorder_Template")]
    UIWidgetGroupHUDAutodanceRecorder(ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIWidgetGroupHUD_Lyrics_Template")]
    UIWidgetGroupHUDLyrics(ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIWidgetGroupHUD_PauseIcon_Template")]
    UIWidgetGroupHUDPauseIcon(ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIWidgetGroupHUD_Template")]
    UIWidgetGroupHUD(ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "Mesh3DComponent_Template")]
    Mesh3DComponent(SingleInstanceMesh3DComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PadRumbleManager_Template")]
    PadRumbleManager(PadRumbleManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PleoComponent_Template")]
    PleoComponent(PleoComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PleoTextureGraphicComponent_Template")]
    PleoTextureGraphicComponent(PleoTextureGraphicComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PropertyPatcher_Template")]
    PropertyPatcher(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "RewardContainer_Template")]
    RewardContainer(RewardContainer<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "SingleInstanceMesh3DComponent_Template")]
    SingleInstanceMesh3DComponent(SingleInstanceMesh3DComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "SoundConfig_Template")]
    SoundConfig(SoundConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "StatsContainer_Template")]
    StatsContainer(StatsContainer<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TextureGraphicComponent_Template")]
    TextureGraphicComponent(TextureGraphicComponent<'a>),
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
    #[serde(borrow, rename = "UITextBox_Template")]
    UITextBox(UITextBox<'a>),
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
    /// Convert this template to a `Actor17`.
    pub fn into_actor(self) -> Result<Actor16<'a>, ParserError> {
        if let Template16::Actor(actor) = self {
            Ok(actor)
        } else {
            Err(ParserError::custom(format!(
                "Actor not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a `AvatarDescription17`.
    pub fn into_avatar_description(self) -> Result<AvatarDescription16<'a>, ParserError> {
        if let Template16::AvatarDescription(avatar_description) = self {
            Ok(avatar_description)
        } else {
            Err(ParserError::custom(format!(
                "AvatarDescription not found in template: {self:?}"
            )))
        }
    }

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
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub game_text_file_path: Cow<'a, str>,
    pub loading: Cow<'a, str>,
    pub game_flow_scene_path: Cow<'a, str>,
    pub camera_shake_config: Cow<'a, str>,
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
    #[serde(rename = "carousel_rules")]
    pub carousel_rules: Cow<'a, str>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub jdpaths: HashMap<u32, Cow<'a, str>>,
    pub jdblockspath: Cow<'a, str>,
    pub jdcommontapepath: Cow<'a, str>,
    pub picto_component_tpl_paths: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub dynamic_music_track_component_tpl_path: Cow<'a, str>,
    #[serde(rename = "songdb_scene")]
    pub songdb_scene: Cow<'a, str>,
    #[serde(rename = "agingbot_behavioursTpl")]
    pub agingbot_behaviours_tpl: Cow<'a, str>,
    #[serde(rename = "avatardb_scene")]
    pub avatardb_scene: Cow<'a, str>,
    #[serde(rename = "flagdb_scene")]
    pub flagdb_scene: Cow<'a, str>,
    pub avatar_folder: Cow<'a, str>,
    pub song_tags: Vec<Cow<'a, str>>,
    pub short_cut_configs: HashMap<Cow<'a, str>, ShortcutSetup1619<'a>>,
    pub default_phone_images: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub max_controller_sleep_time: f32,
    pub audio_package_name: Cow<'a, str>,
    pub package_scene_paths: HashMap<Cow<'a, str>, Cow<'a, str>>,
    #[serde(rename = "ed_songdb_scene")]
    pub ed_songdb_scene: Cow<'a, str>,
    pub cameras: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub uiscenes: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub banner_scenes: Vec<Cow<'a, str>>,
    pub transition_scenes: Vec<Cow<'a, str>>,
    pub genericstages: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub popupconfigs: PopupConfigList<'a>,
    pub scoringparams: ScoringParams<'a>,
    pub kinect_scoringparams: ScoringCameraParams<'a>,
    pub menuassetsparams: Vec<MenuAssetsCacheParams<'a>>,
    pub menumusicsparams: HashMap<Cow<'a, str>, MenuMusicParams<'a>>,
    pub remotesoundparams: HashMap<Cow<'a, str>, RemoteSoundParams<'a>>,
    pub menumusicconfig: MenuMusicConfig<'a>,
    pub sweat_programs: Vec<u32>,
    pub mashupdates: HashMap<Cow<'a, str>, u32>,
    pub mashupavatars: HashMap<Cow<'a, str>, u32>,
    pub mojoprices: HashMap<Cow<'a, str>, u32>,
    pub slave_phone_loc_ids: HashMap<Cow<'a, str>, Vec<u32>>,
    pub questdataentries: Vec<QuestEntry1617<'a>>,
    pub unlimitedupsellsonglist: Vec<UnlimitedUpsellSongList<'a>>,
    pub questconfig: QuestConfig1618<'a>,
    pub questchallengerentries: Vec<QuestChallengerEntry1618<'a>>,
    pub sweatrandomizeconfig: SweatRandomizeConfig1619<'a>,
    pub challenger_evolution_template_list: Vec<ChallengerScoreEvolutionTemplate1619<'a>>,
    pub countryentries: Vec<CountryEntry<'a>>,
    pub credits_textbox_path: Cow<'a, str>,
    pub avatar_min_anim_hud_duration: u32,
    pub b2b_maps: Vec<Cow<'a, str>>,
    pub chatmessagesparams: ChatMessagesParams1618<'a>,
    pub chat_messages: HashMap<Cow<'a, str>, Vec<u32>>,
    pub challenge_algo_order: Vec<Cow<'a, str>>,
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
    pub countdown_delays: HashMap<Cow<'a, str>, u32>,
    pub autodance_effects_list: Vec<AutoDanceEffectData<'a>>,
    pub coop_tweaked_texts: Vec<CoopTweakedText17<'a>>,
    pub messages_slides: HashMap<Cow<'a, str>, TutorialContent<'a>>,
    pub tutorials: Vec<TutorialDesc<'a>>,
    pub redeem_maps: HashMap<Cow<'a, str>, Vec<Cow<'a, str>>>,
    pub uplay_unlockable_maps: HashMap<Cow<'a, str>, u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Actor16<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub wip: u32,
    pub lowupdate: u32,
    pub update_layer: u32,
    pub procedural: u32,
    pub startpaused: u32,
    pub forceisenvironment: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Template16<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AvatarDescription16<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub jd_version: u16,
    pub relative_song_name: Cow<'a, str>,
    #[serde(rename = "RelativeQuestID")]
    pub relative_quest_id: Cow<'a, str>,
    pub relative_game_mode_name: Cow<'a, str>,
    pub actor_path: Cow<'a, str>,
    pub avatar_id: u16,
    pub phone_image: Cow<'a, str>,
    pub status: u8,
    pub unlock_type: u8,
    pub mojo_price: u16,
    pub wdf_level: u8,
    pub count_in_progression: u8,
}
