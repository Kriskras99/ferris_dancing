use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[allow(clippy::wildcard_imports)]
use super::*;

pub struct Template17Owned<C: StableDeref> {
    yoke: Yoke<Template17<'static>, C>,
}

impl<C: StableDeref> From<Yoke<Template17<'static>, C>> for Template17Owned<C> {
    fn from(yoke: Yoke<Template17<'static>, C>) -> Self {
        Self { yoke }
    }
}

impl<'a, C: StableDeref> Template17Owned<C> {
    pub fn template(&'a self) -> &'a Template17<'a> {
        self.yoke.get()
    }
}

#[derive(Debug, Serialize, Deserialize, Yokeable)]
#[serde(tag = "__class")]
pub enum Template17<'a> {
    #[serde(borrow, rename = "Actor_Template")]
    Actor(Actor17<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "AFXPostProcessComponent_Template")]
    AFXPostProcessComponent(AFXPostProcessComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "BezierTreeComponent_Template")]
    BezierTreeComponent(BezierTreeComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "BoxInterpolatorComponent_Template")]
    BoxInterpolatorComponent(BoxInterpolatorComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ConvertedTmlTape_Template")]
    ConvertedTmlTape(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "DynamicMusicTrackComponent_Template")]
    DynamicMusicTrackComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FxBankComponent_Template")]
    FxBankComponent(FxBankComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FXControllerComponent_Template")]
    FxControllerComponent(FxControllerComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_AgingBot_BehaviourAllTrees")]
    AgingBotBehaviourAllTrees(AgingBotBehaviourAllTrees<'a>),
    #[serde(borrow, rename = "JD_AutodanceComponent_Template")]
    AutodanceComponent(AutodanceComponent<'a>),
    #[serde(borrow, rename = "JD_AvatarDescTemplate")]
    AvatarDescription(AvatarDescription<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_Carousel_Template")]
    Carousel(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CreditsComponent_Template")]
    CreditsComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_FixedCameraComponent_Template")]
    FixedCameraComponent(FixedCameraComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_GachaComponent_Template")]
    GachaComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_GoldMoveComponent_Template")]
    GoldMoveComponent(Empty<'a>),
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
    #[serde(borrow, rename = "JD_NotificationBubblesPile_Template")]
    NotificationBubblesPile(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_NotificationBubble_Template")]
    NotificationBubble(Empty<'a>),
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
    #[serde(borrow, rename = "JD_SkinDescTemplate")]
    SkinDescription(SkinDescription<'a>),
    #[serde(borrow, rename = "JD_SongDatabaseTemplate")]
    SongDatabase(SongDatabase<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_SongDescTemplate")]
    SongDescription(SongDescription<'a>),
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
    #[serde(borrow, rename = "JD_WDFTeamBattlePresentationComponent_Template")]
    WDFTeamBattlePresentationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFThemePresentationComponent_Template")]
    WDFThemePresentationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFUnlimitedFeedbackComponent_Template")]
    WDFUnlimitedFeedbackComponent(Empty<'a>),
    #[serde(borrow, rename = "MasterTape_Template")]
    MasterTape(MasterTape<'a>),
    #[serde(borrow, rename = "MaterialGraphicComponent_Template")]
    MaterialGraphicComponent(MaterialGraphicComponent<'a>),
    #[serde(borrow, rename = "MusicTrackComponent_Template")]
    MusicTrackComponent(MusicTrackComponent<'a>),
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
    #[serde(borrow, rename = "SingleInstanceMesh3DComponent_Template")]
    SingleInstanceMesh3DComponent(SingleInstanceMesh3DComponent<'a>),
    #[serde(borrow, rename = "SoundComponent_Template")]
    SoundComponent(SoundComponent<'a>),
    #[serde(borrow, rename = "TapeCase_Template")]
    TapeCase(MasterTape<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TextureGraphicComponent_Template")]
    TextureGraphicComponent(TextureGraphicComponent<'a>),
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
    #[serde(borrow, rename = "UINineSliceComponent_Template")]
    UiNineSliceComponent(MaterialGraphicComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UINineSliceMaskComponent_Template")]
    UINineSliceMaskComponent(UINineSliceMaskComponent<'a>),
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
    #[serde(borrow, rename = "JD_AchievementsDatabase_Template")]
    AchievementsDatabase(AchievementsDatabase<'a>),
    #[serde(borrow, rename = "JD_LocalAliases")]
    LocalAliases(LocalAliases1719<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "CameraShakeConfig_Template")]
    CameraShakeConfig(CameraShakeConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "CarouselManager_Template")]
    CarouselManager(CarouselManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_GachaContentDatabase_Template")]
    GachaContentDatabase(GachaContentDatabase<'a>),
    #[serde(borrow, rename = "JD_GameManagerConfig_Template")]
    GameManagerConfig(Box<GameManagerConfig17<'a>>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FontEffectList_Template")]
    FontEffectList(FontEffectList<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_FTUESteps_Template")]
    FTUESteps(FTUESteps<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CarouselRules")]
    CarouselRules(CarouselRules<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TRCLocalisation_Template")]
    TRCLocalisation(TRCLocalisation<'a>),
    #[serde(borrow, rename = "JD_ObjectivesDatabase_Template")]
    ObjectivesDatabase(ObjectivesDatabase<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PadRumbleManager_Template")]
    PadRumbleManager(PadRumbleManager<'a>),
    #[serde(borrow, rename = "JD_PlaylistDatabase_Template")]
    PlaylistDatabase(PlaylistDatabase<'a>),
    #[serde(borrow, rename = "JD_PortraitBordersDatabase_Template")]
    PortraitBordersDatabase(PortraitBordersDatabase<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_QuickplayRules_Template")]
    QuickplayRules(QuickplayRules<'a>),
    #[serde(borrow, rename = "JD_ScheduledQuestDatabase_Template")]
    ScheduledQuestDatabase(ScheduledQuestDatabase<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "SoundConfig_Template")]
    SoundConfig(SoundConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UITextManager_Template")]
    UITextManager(UITextManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "VibrationManager_Template")]
    VibrationManager(VibrationManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFLinearRewards")]
    WDFLinearRewards(WDFLinearRewards<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ZInputConfig_Template")]
    ZInputConfig(ZInputConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ZInputManager_Template")]
    ZInputManager(ZInputManager<'a>),
    #[serde(borrow, rename = "Tape")]
    Tape(Tape<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_AnthologyConfig")]
    AnthologyConfig(AnthologyConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FeedbackFXManager_Template")]
    FeedbackFXManager(FeedbackFXManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "GFXMaterialShader_Template")]
    GFXMaterialShader(GFXMaterialShader1718<'a>),
}

impl<'a> Template17<'a> {
    /// Convert this template to a `GameManagerConfig17`.
    ///
    /// # Errors
    /// Will error if this template is not a `GameManagerConfig17`.
    pub fn game_manager_config(self) -> Result<GameManagerConfig17<'a>, Error> {
        if let Template17::GameManagerConfig(gmc) = self {
            Ok(*gmc)
        } else {
            Err(anyhow!("GameManagerConfig not found in template: {self:?}"))
        }
    }

    /// Convert this template to a `ObjectivesDatabase`.
    ///
    /// # Errors
    /// Will error if this template is not a `ObjectivesDatabase`.
    pub fn objectives_database(&'a self) -> Result<&'a ObjectivesDatabase<'a>, Error> {
        if let Template17::ObjectivesDatabase(objs_db) = self {
            Ok(objs_db)
        } else {
            Err(anyhow!(
                "ObjectivesDatabase not found in template: {self:?}"
            ))
        }
    }

    /// Convert this template to a `ScheduledQuestDatabase`.
    ///
    /// # Errors
    /// Will error if this template is not a `ScheduledQuestDatabase`.
    pub fn scheduled_quests_database(&'a self) -> Result<&'a ScheduledQuestDatabase<'a>, Error> {
        if let Template17::ScheduledQuestDatabase(sqst_db) = self {
            Ok(sqst_db)
        } else {
            Err(anyhow!(
                "ScheduledQuestDatabase not found in template: {self:?}"
            ))
        }
    }

    /// Convert this template to a `PlaylistDatabase`.
    ///
    /// # Errors
    /// Will error if this template is not a `PlaylistDatabase`.
    pub fn playlists_database(&'a self) -> Result<&'a PlaylistDatabase<'a>, Error> {
        if let Template17::PlaylistDatabase(playlist_db) = self {
            Ok(playlist_db)
        } else {
            Err(anyhow!("PlaylistDatabase not found in template: {self:?}"))
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GameManagerConfig17<'a> {
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
    #[serde(rename = "skindb_scene")]
    pub skindb_scene: Cow<'a, str>,
    #[serde(rename = "flagdb_scene")]
    pub flagdb_scene: Cow<'a, str>,
    pub avatar_folder: Cow<'a, str>,
    pub pin_unplayed_song: Cow<'a, str>,
    pub wdf_player_name_prefix_on_xbox_one: Cow<'a, str>,
    #[serde(rename = "wdfPlayerNamePrefixNonPS4")]
    pub wdf_player_name_prefix_non_ps4: Cow<'a, str>,
    pub short_cut_configs: HashMap<Cow<'a, str>, ShortcutSetup1719<'a>>,
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
    pub actors_to_bundle: Vec<Cow<'a, str>>,
    pub genericstages: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub popupconfigs: PopupConfigList<'a>,
    pub clubrewardconfigs: HashMap<Cow<'a, str>, ClubRewardConfig<'a>>,
    pub scoringparams: ScoringParams<'a>,
    pub kinect_scoringparams: ScoringCameraParams<'a>,
    pub menuassetsparams: Vec<MenuAssetsCacheParams<'a>>,
    pub menumusicsparams: HashMap<Cow<'a, str>, MenuMusicParams<'a>>,
    pub remotesoundparams: HashMap<Cow<'a, str>, RemoteSoundParams<'a>>,
    pub menu_music_multi_tracks: HashMap<Cow<'a, str>, MenuMultiTrackItem<'a>>,
    pub menumusicconfig: MenuMusicConfig<'a>,
    pub sweat_programs: Vec<u32>,
    pub mashupdates: HashMap<Cow<'a, str>, u32>,
    pub mashupavatars: HashMap<Cow<'a, str>, u32>,
    pub mojoprices: HashMap<Cow<'a, str>, u32>,
    pub rankdescriptor: RankDescriptor<'a>,
    pub slave_phone_loc_ids: HashMap<Cow<'a, str>, Vec<u32>>,
    pub questdataentries: Vec<QuestEntry17<'a>>,
    pub questplayercamslot: HashMap<Cow<'a, str>, Vec<u32>>,
    pub unlimitedupsellsonglist: Vec<UnlimitedUpsellSongList<'a>>,
    pub questconfig: QuestConfig1718<'a>,
    pub questchallengerentries: Vec<QuestChallengerEntry1718<'a>>,
    pub customizableitemconfig: CustomizableItemConfig<'a>,
    pub dancemachinerandomizeconfig: DanceMachineRandomSetup17<'a>,
    pub dancemachineglobalconfig: DanceMachineGlobalConfig1719<'a>,
    pub sweatrandomizeconfig: SweatRandomizeConfig1719<'a>,
    pub searchconfig: SearchConfig1719<'a>,
    pub challenger_evolution_template_list: Vec<ChallengerScoreEvolutionTemplate1719<'a>>,
    pub countryentries: Vec<CountryEntry<'a>>,
    pub credits_textbox_path: Cow<'a, str>,
    pub avatar_min_anim_hud_duration: u32,
    pub b2b_maps: Vec<Cow<'a, str>>,
    pub chatmessagesparams: ChatMessagesParams1718<'a>,
    pub chat_messages: HashMap<Cow<'a, str>, Vec<u32>>,
    pub coop_score_diamonds_values: Vec<f32>,
    pub coop_jauge_anim_time: Vec<u32>,
    pub rival_recap_incr_score_speed: f32,
    pub countdown_delays: HashMap<Cow<'a, str>, u32>,
    pub autodance_effects_list: Vec<AutoDanceEffectData<'a>>,
    pub autodance_transition_sound_path: Cow<'a, str>,
    pub autodance_transition_sound_synchronise_sample: u32,
    pub autodance_transition_sound_synchronise_time: u32,
    pub coop_tweaked_texts: Vec<CoopTweakedText17<'a>>,
    pub messages_slides: HashMap<Cow<'a, str>, TutorialContent<'a>>,
    pub tutorials: Vec<TutorialDesc<'a>>,
    pub redeem_maps: HashMap<Cow<'a, str>, Vec<Cow<'a, str>>>,
    pub uplay_unlockable_maps: HashMap<Cow<'a, str>, u32>,
    pub stars_6th_step_song_score: u32,
    pub stars_6th_step_incoming_effect_start_relative_score: i32,
    pub perfect_plus_feedback_min_score: u32,
    pub min_song_nb_for_shuffle: u32,
    pub wdf_boss_entries: Vec<WDFBossEntry<'a>>,
    pub itemcolorlookup: ItemColorLookUp<'a>,
    pub looped_video_config: HashMap<Cow<'a, str>, VideoLoopSetup<'a>>,
    #[serde(rename = "defaultJDUVideoPreview")]
    pub default_jdu_video_preview: Cow<'a, str>,
    pub diamond_points: Vec<u32>,
    pub jd_points_per_star: Vec<u32>,
    pub banned_maps_in_chinese: Vec<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Actor17<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub wip: u32,
    pub lowupdate: u32,
    pub update_layer: u32,
    pub procedural: u32,
    pub startpaused: u32,
    pub forceisenvironment: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Template17<'a>>,
}
