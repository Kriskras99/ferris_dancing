use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use yoke::Yokeable;

#[cfg(feature = "full_json_types")]
use super::{
    frt::FeedbackFXManager,
    isg::{
        CameraShakeConfig, CarouselManager, CarouselRules, FontEffectList, PadRumbleManager,
        SoundConfig, TRCLocalisation, UITextManager, ZInputConfig, ZInputManager,
    },
    just_dance::{
        AgingBotBehaviourAllTrees, FixedCameraComponent, SkinDescription, SongDescription,
    },
    msh::GFXMaterialShader1718,
    tpl::{
        BezierTreeComponent, FxBankComponent, FxControllerComponent, PleoComponent,
        PleoTextureGraphicComponent, SingleInstanceMesh3DComponent, TextureGraphicComponent,
        UITextBox,
    },
    Empty,
};
use super::{
    isg::{
        AutoDanceEffectData, ChallengerScoreEvolutionTemplate1719, ChatMessagesParams1718,
        ClubRewardConfig, CoopTweakedText17, CountryEntry, CustomizableItemConfig,
        DanceMachineGlobalConfig1719, DanceMachineRandomSetup17, ItemColorLookUp,
        MenuAssetsCacheParams, MenuMultiTrackItem, MenuMusicConfig, MenuMusicParams,
        PopupConfigList, QuestChallengerEntry1718, QuestConfig1718, QuestEntry17, RankDescriptor,
        RemoteSoundParams, ScoringCameraParams, ScoringParams, SearchConfig1719, ShortcutSetup1719,
        SweatRandomizeConfig1719, TutorialContent, TutorialDesc, UnlimitedUpsellSongList,
        VideoLoopSetup, WDFBossEntry,
    },
    just_dance::{AutodanceComponent, SongDatabase},
    tape::Tape,
    tpl::{MasterTape, MaterialGraphicComponent, MusicTrackComponent, SoundComponent},
};
use crate::utils::errors::ParserError;

#[derive(Debug, Serialize, Deserialize, Yokeable)]
#[serde(tag = "__class")]
pub enum Template17<'a> {
    #[serde(borrow, rename = "Actor_Template")]
    Actor(Actor17<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "BezierTreeComponent_Template")]
    BezierTreeComponent(BezierTreeComponent<'a>),
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
    AvatarDescription(AvatarDescription17<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CreditsComponent_Template")]
    CreditsComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_FixedCameraComponent_Template")]
    FixedCameraComponent(FixedCameraComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_GoldMoveComponent_Template")]
    GoldMoveComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_SkinDescTemplate")]
    SkinDescription(SkinDescription<'a>),
    #[serde(borrow, rename = "JD_SongDatabaseTemplate")]
    SongDatabase(SongDatabase<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_SongDescTemplate")]
    SongDescription(SongDescription<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_SubtitleComponent_Template")]
    SubtitleComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIAvatarUnlockWidget_Template")]
    UIAvatarUnlockWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudPlayerComponent_Template")]
    UIHudPlayerComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineWDFBossComponent_Template")]
    UIHudRacelineWDFBossComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineWDFRankComponent_Template")]
    UIHudRacelineWDFRankComponent(Empty<'a>),
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
    #[serde(borrow, rename = "JD_UIMojoWidget_Template")]
    UIMojoWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UISaveWidget_Template")]
    UISaveWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIUploadIcon_Template")]
    UIUploadIcon(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFBossSpawnerComponent_Template")]
    WDFBossSpawnerComponent(Empty<'a>),
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
    #[serde(borrow, rename = "CameraShakeConfig_Template")]
    CameraShakeConfig(CameraShakeConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "CarouselManager_Template")]
    CarouselManager(CarouselManager<'a>),
    #[serde(borrow, rename = "JD_GameManagerConfig_Template")]
    GameManagerConfig(Box<GameManagerConfig17<'a>>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FontEffectList_Template")]
    FontEffectList(FontEffectList<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CarouselRules")]
    CarouselRules(CarouselRules<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TRCLocalisation_Template")]
    TRCLocalisation(TRCLocalisation<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PadRumbleManager_Template")]
    PadRumbleManager(PadRumbleManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "SoundConfig_Template")]
    SoundConfig(SoundConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UITextManager_Template")]
    UITextManager(UITextManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ZInputConfig_Template")]
    ZInputConfig(ZInputConfig<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ZInputManager_Template")]
    ZInputManager(ZInputManager<'a>),
    #[serde(borrow, rename = "Tape")]
    Tape(Tape<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FeedbackFXManager_Template")]
    FeedbackFXManager(FeedbackFXManager<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "GFXMaterialShader_Template")]
    GFXMaterialShader(GFXMaterialShader1718<'a>),
}

impl<'a> Template17<'a> {
    /// Convert this template to a `Actor17`.
    pub fn into_actor(self) -> Result<Actor17<'a>, ParserError> {
        if let Template17::Actor(actor) = self {
            Ok(actor)
        } else {
            Err(ParserError::custom(format!(
                "Actor not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a `AvatarDescription17`.
    pub fn into_avatar_description(self) -> Result<AvatarDescription17<'a>, ParserError> {
        if let Template17::AvatarDescription(avatar_description) = self {
            Ok(avatar_description)
        } else {
            Err(ParserError::custom(format!(
                "AvatarDescription not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a `GameManagerConfig17`.
    pub fn into_game_manager_config(self) -> Result<GameManagerConfig17<'a>, ParserError> {
        if let Template17::GameManagerConfig(gmc) = self {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AvatarDescription17<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub jd_version: u16,
    pub relative_song_name: Cow<'a, str>,
    #[serde(rename = "RelativeQuestID")]
    pub relative_quest_id: Cow<'a, str>,
    #[serde(rename = "RelativeWDFBossName")]
    pub relative_wdf_boss_name: Cow<'a, str>,
    #[serde(rename = "RelativeWDFTournamentName")]
    pub relative_wdf_tournament_name: Cow<'a, str>,
    #[serde(rename = "RelativeJDRank")]
    pub relative_jd_rank: Cow<'a, str>,
    pub relative_game_mode_name: Cow<'a, str>,
    pub sound_family: Cow<'a, str>,
    pub status: u8,
    pub unlock_type: u8,
    pub mojo_price: u16,
    pub wdf_level: u8,
    pub count_in_progression: u8,
    pub actor_path: Cow<'a, str>,
    pub phone_image: Cow<'a, str>,
    pub avatar_id: u16,
}
