use std::{collections::HashMap, fmt::Debug};
use std::fmt::Formatter;
use hipstr::HipStr;
use ownable::IntoOwned;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error, Visitor};
use serde_with::{serde_as, DeserializeAs};
use tracing::{error, trace};
use ubiart_toolkit_shared_types::{errors::ParserError, Color, LocaleId};

use crate::shared_json_types::AutodanceVideoStructure;
#[cfg(feature = "full_json_types")]
use crate::shared_json_types::Empty;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct Actor<'a> {
    #[serde(borrow, rename = "__class")]
    pub class: HipStr<'a>,
    pub wip: u32,
    pub lowupdate: u32,
    pub update_layer: u32,
    pub procedural: u32,
    pub startpaused: u32,
    pub forceisenvironment: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Template<'a>>,
}

impl Default for Actor<'static> {
    fn default() -> Self {
        Self {
            class: Self::CLASS,
            wip: 0,
            lowupdate: 0,
            update_layer: 0,
            procedural: 0,
            startpaused: 0,
            forceisenvironment: 0,
            components: Vec::new(),
        }
    }
}

impl Actor<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("Actor_Template");
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "__class")]
pub enum Template<'a> {
    #[serde(borrow, rename = "JD_AutodanceComponent_Template")]
    AutodanceComponent(AutodanceComponent<'a>),
    #[serde(borrow, rename = "JD_AsyncPlayerDesc_Template")]
    AsyncPlayerDescTemplate(AsyncPlayerDescTemplate<'a>),
    #[serde(borrow, rename = "JD_AvatarDescTemplate")]
    AvatarDescription(AvatarDescription<'a>),
    #[serde(borrow, rename = "JD_BlockFlowTemplate")]
    BlockFlowTemplate(BlockFlowTemplate<'a>),
    #[serde(borrow, rename = "JD_SongDatabaseTemplate")]
    SongDatabase(SongDatabase<'a>),
    #[serde(borrow, rename = "JD_SongDescTemplate")]
    SongDescription(SongDescription<'a>),
    #[serde(borrow, rename = "MasterTape_Template")]
    MasterTape(MasterTape<'a>),
    #[serde(borrow, rename = "MaterialGraphicComponent_Template")]
    MaterialGraphicComponent(MaterialGraphicComponent<'a>),
    #[serde(borrow, rename = "MusicTrackComponent_Template")]
    MusicTrackComponent(MusicTrackComponent<'a>),
    #[serde(borrow, rename = "PleoComponent_Template")]
    PleoComponent(PleoComponent<'a>),
    #[serde(borrow, rename = "PleoTextureGraphicComponent_Template")]
    PleoTextureGraphicComponent(PleoTextureGraphicComponent<'a>),
    #[serde(borrow, rename = "SoundComponent_Template")]
    SoundComponent(SoundComponent<'a>),
    #[serde(borrow, rename = "TapeCase_Template")]
    TapeCase(MasterTape<'a>),
    // From here types are not required for JDMod
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "AFXPostProcessComponent_Template")]
    AFXPostProcessComponent(super::extra_types::AFXPostProcessComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "BezierTreeComponent_Template")]
    BezierTreeComponent(super::extra_types::BezierTreeComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "BoxInterpolatorComponent_Template")]
    BoxInterpolatorComponent(super::extra_types::BoxInterpolatorComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "CameraGraphicComponent_Template")]
    CameraGraphicComponent(MaterialGraphicComponent<'a>),
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
    #[serde(borrow, rename = "FxBankComponent_Template")]
    FxBankComponent(super::extra_types::FxBankComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "FXControllerComponent_Template")]
    FxControllerComponent(super::extra_types::FxControllerComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_AgingBot_BehaviourAllTrees")]
    AgingBotBehaviourAllTrees(super::extra_types::AgingBotBehaviourAllTrees<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_AliasUnlockNotification_Template")]
    AliasUnlockNotification(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_AnthologyGrid_Template")]
    AnthologyGrid(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_BeatPulseComponent_Template")]
    BeatPulseComponent(super::extra_types::ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_CameraFeedComponent_Template")]
    CameraFeedComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_Carousel_Template")]
    Carousel(Empty<'a>),
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
    FixedCameraComponent(super::extra_types::FixedCameraComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_GachaComponent_Template")]
    GachaComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_GoldMoveComponent_Template")]
    GoldMoveComponent(super::extra_types::ModeType<'a>),
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
    #[serde(borrow, rename = "JD_PictoComponent_Template")]
    PictoComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_PictoTimeline_Template")]
    PictoTimeline(super::extra_types::ModeType<'a>),
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
    #[serde(borrow, rename = "JD_ScrollBarComponent_Template")]
    ScrollBarComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_ScrollingPopupComponent_Template")]
    ScrollingPopupComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_ScrollingTextComponent_Template")]
    ScrollingTextComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_SkinDescTemplate")]
    SkinDescription(super::extra_types::SkinDescription<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_StickerGrid_Template")]
    StickerGrid(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_SubtitleComponent_Template")]
    SubtitleComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIGrid_Template")]
    UIGrid(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIAvatarUnlockWidget_Template")]
    UIAvatarUnlockWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudAutodanceRecorderComponent_Template")]
    UIHudAutodanceRecorderComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudDoubleScoringPlayerComponent_Template")]
    UIHudDoubleScoringPlayerComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudCamerafeedComponent_Template")]
    UIHudCamerafeedComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudCommunityDancerCardComponent_Template")]
    UIHudCommunityDancerCardComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudCoopFeedbackComponent_Template")]
    UIHudCoopFeedbackComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudLyricsComponent_Template")]
    UIHudLyricsComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudLyricsFeedbackComponent_Template")]
    UIHudLyricsFeedbackComponent(Empty<'a>),
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
    #[serde(borrow, rename = "JD_UIHudRacelineCoopComponent_Template")]
    UIHudRacelineCoopComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineDM_Template")]
    UIHudRacelineDM(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineGaugeBarComponent_Template")]
    UIHudRacelineGaugeBarComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineGaugeComponent_Template")]
    UIHudRacelineGaugeComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineRivalBarComponent_Template")]
    UIHudRacelineRivalBarComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudRacelineRivalComponent_Template")]
    UIHudRacelineRivalComponent(Empty<'a>),
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
    #[serde(borrow, rename = "JD_UIHudRacelineWDFSpotlightComponent_Template")]
    UIHudRacelineWDFSpotlightComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudShowtimePhotoFeedbackComponent_Template")]
    UIHudShowtimePhotoFeedbackComponent(Empty<'a>),
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
    #[serde(borrow, rename = "JD_UIHudVumeterComponent_Template")]
    UIHudVumeterComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIHudWDFIngameNotificationComponent_Template")]
    UIHudWDFIngameNotificationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIJDRankWidget_Template")]
    UIJDRankWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIJoyconWidget_Template")]
    UIJoyconWidget(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UILineGrid_Template")]
    UILineGrid(Empty<'a>),
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
    #[serde(borrow, rename = "JD_UIWidgetElement_Template")]
    UIWidgetElement(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIWidgetGroupHUD_AutodanceRecorder_Template")]
    UIWidgetGroupHUDAutodanceRecorder(super::extra_types::ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIWidgetGroupHUD_Lyrics_Template")]
    UIWidgetGroupHUDLyrics(super::extra_types::ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIWidgetGroupHUD_PauseIcon_Template")]
    UIWidgetGroupHUDPauseIcon(super::extra_types::ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_UIWidgetGroupHUD_Template")]
    UIWidgetGroupHUD(super::extra_types::ModeType<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFBossSpawnerComponent_Template")]
    WDFBossSpawnerComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFOnlineRankTransitionComponent_Template")]
    WDFOnlineRankTransitionComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFTeamBattlePresentationComponent_Template")]
    WDFTeamBattlePresentationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFTeamBattleTransitionComponent_Template")]
    WDFTeamBattleTransitionComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFThemePresentationComponent_Template")]
    WDFThemePresentationComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFTransitionComponent_Template")]
    WDFTransitionComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "JD_WDFUnlimitedFeedbackComponent_Template")]
    WDFUnlimitedFeedbackComponent(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "Mesh3DComponent_Template")]
    Mesh3DComponent(super::extra_types::SingleInstanceMesh3DComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "PropertyPatcher_Template")]
    PropertyPatcher(Empty<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "SingleInstanceMesh3DComponent_Template")]
    SingleInstanceMesh3DComponent(super::extra_types::SingleInstanceMesh3DComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TextureGraphicComponent_Template")]
    TextureGraphicComponent(super::extra_types::TextureGraphicComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "TexturePatcherComponent_Template")]
    TexturePatcherComponent(Empty<'a>),
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
    UIComponent(Empty<'a>),
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
    #[serde(borrow, rename = "UINineSliceComponent_Template")]
    UiNineSliceComponent(MaterialGraphicComponent<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "UINineSliceMaskComponent_Template")]
    UINineSliceMaskComponent(super::extra_types::UINineSliceMaskComponent<'a>),
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
    UITextBox(super::extra_types::UITextBox<'a>),
    #[cfg(feature = "full_json_types")]
    #[serde(borrow, rename = "ViewportUIComponent_Template")]
    ViewportUIComponent(Empty<'a>),
}

impl<'a> Template<'a> {
    /// Convert this template to a [`SongDescription`].
    pub fn into_song_description(self) -> Result<SongDescription<'a>, ParserError> {
        if let Template::SongDescription(template) = self {
            Ok(template)
        } else {
            Err(ParserError::custom(format!(
                "SongDescription not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a [`MusicTrackComponent`].
    pub fn into_musictrack_component(self) -> Result<MusicTrackComponent<'a>, ParserError> {
        if let Template::MusicTrackComponent(template) = self {
            Ok(template)
        } else {
            Err(ParserError::custom(format!(
                "MusicTrackComponent not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a [`MasterTape`].
    pub fn into_master_tape(self) -> Result<MasterTape<'a>, ParserError> {
        if let Template::MasterTape(template) = self {
            Ok(template)
        } else {
            Err(ParserError::custom(format!(
                "MasterTape not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a [`SoundComponent`].
    pub fn into_sound_component(self) -> Result<SoundComponent<'a>, ParserError> {
        if let Template::SoundComponent(template) = self {
            Ok(template)
        } else {
            Err(ParserError::custom(format!(
                "SoundComponent not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a [`MasterTape`].
    pub fn into_tape_case_component(self) -> Result<MasterTape<'a>, ParserError> {
        if let Template::TapeCase(template) = self {
            Ok(template)
        } else {
            Err(ParserError::custom(format!(
                "TapeCase not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a [`AutodanceComponent`].
    pub fn into_autodance_component(self) -> Result<AutodanceComponent<'a>, ParserError> {
        if let Template::AutodanceComponent(template) = self {
            Ok(template)
        } else {
            Err(ParserError::custom(format!(
                "AutodanceComponent not found in template: {self:?}"
            )))
        }
    }

    /// Convert this template to a [`AvatarDescription`].
    pub fn into_avatar_description(self) -> Result<AvatarDescription<'a>, ParserError> {
        if let Template::AvatarDescription(template) = self {
            Ok(template)
        } else {
            Err(ParserError::custom(format!(
                "AvatarDescription not found in template: {self:?}"
            )))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AsyncPlayerDescTemplate<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub player_name: HipStr<'a>,
    pub player_country: Country<'a>,
    pub player_age_bracket: u32,
    pub player_gender: u32,
    #[serde(rename = "avatarID")]
    pub avatar_id: u32,
    #[serde(borrow)]
    pub thumbnails_path: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AutodanceComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub song: HipStr<'a>,
    #[serde(borrow)]
    pub autodance_data: AutodanceData<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct AutodanceData<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub recording_structure: AutodanceRecordingStructure<'a>,
    #[serde(borrow)]
    pub video_structure: AutodanceVideoStructure<'a>,
    #[serde(rename = "autodanceSoundPath")]
    pub autodance_sound_path: HipStr<'a>,
}

impl AutodanceData<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_AutodanceData");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AutodanceRecordingStructure<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub records: Vec<Record<'a>>,
}

impl AutodanceRecordingStructure<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_AutodanceRecordingStructure");
}

#[derive(Debug, Serialize, Deserialize, Clone, IntoOwned)]
#[serde(untagged)]
pub enum AvatarDescription<'a> {
    #[serde(borrow)]
    V16(AvatarDescription16<'a>),
    #[serde(borrow)]
    V17(AvatarDescription17<'a>),
    #[serde(borrow)]
    V1819(AvatarDescription1819<'a>),
    #[serde(borrow)]
    V2022(AvatarDescription2022<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone, IntoOwned)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AvatarDescription16<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub jd_version: u32,
    pub relative_song_name: HipStr<'a>,
    #[serde(rename = "RelativeQuestID")]
    pub relative_quest_id: HipStr<'a>,
    pub relative_game_mode_name: HipStr<'a>,
    pub actor_path: HipStr<'a>,
    pub avatar_id: u32,
    pub phone_image: HipStr<'a>,
    pub status: u32,
    pub unlock_type: u32,
    pub mojo_price: u32,
    pub wdf_level: u32,
    pub count_in_progression: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, IntoOwned)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AvatarDescription17<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub jd_version: u16,
    pub relative_song_name: HipStr<'a>,
    #[serde(rename = "RelativeQuestID")]
    pub relative_quest_id: HipStr<'a>,
    #[serde(rename = "RelativeWDFBossName")]
    pub relative_wdf_boss_name: HipStr<'a>,
    #[serde(rename = "RelativeWDFTournamentName")]
    pub relative_wdf_tournament_name: HipStr<'a>,
    #[serde(rename = "RelativeJDRank")]
    pub relative_jd_rank: HipStr<'a>,
    pub relative_game_mode_name: HipStr<'a>,
    pub sound_family: HipStr<'a>,
    pub status: u32,
    pub unlock_type: u32,
    pub mojo_price: u16,
    pub wdf_level: u8,
    pub count_in_progression: u8,
    pub actor_path: HipStr<'a>,
    pub phone_image: HipStr<'a>,
    pub avatar_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, IntoOwned)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AvatarDescription1819<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub jd_version: u16,
    pub relative_song_name: HipStr<'a>,
    #[serde(rename = "RelativeQuestID")]
    pub relative_quest_id: HipStr<'a>,
    #[serde(rename = "RelativeWDFBossName")]
    pub relative_wdf_boss_name: HipStr<'a>,
    #[serde(rename = "RelativeWDFTournamentName")]
    pub relative_wdf_tournament_name: HipStr<'a>,
    #[serde(rename = "RelativeJDRank")]
    pub relative_jd_rank: HipStr<'a>,
    pub relative_game_mode_name: HipStr<'a>,
    pub sound_family: HipStr<'a>,
    pub status: u32,
    pub unlock_type: u32,
    pub mojo_price: u16,
    pub wdf_level: u8,
    pub count_in_progression: u8,
    pub actor_path: HipStr<'a>,
    pub phone_image: HipStr<'a>,
    pub avatar_id: u32,
    #[serde(rename = "UsedAsCoach_MapName")]
    pub used_as_coach_map_name: HipStr<'a>,
    #[serde(rename = "UsedAsCoach_CoachId")]
    pub used_as_coach_coach_id: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, IntoOwned)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AvatarDescription2022<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub jd_version: u32,
    pub relative_song_name: HipStr<'a>,
    #[serde(rename = "RelativeQuestID")]
    pub relative_quest_id: HipStr<'a>,
    #[serde(rename = "RelativeWDFBossName")]
    pub relative_wdf_boss_name: HipStr<'a>,
    #[serde(rename = "RelativeWDFTournamentName")]
    pub relative_wdf_tournament_name: HipStr<'a>,
    #[serde(rename = "RelativeJDRank")]
    pub relative_jd_rank: HipStr<'a>,
    pub relative_game_mode_name: HipStr<'a>,
    pub sound_family: HipStr<'a>,
    pub status: u32,
    pub unlock_type: u32,
    pub mojo_price: u32,
    pub wdf_level: u32,
    pub count_in_progression: u32,
    pub actor_path: HipStr<'a>,
    pub phone_image: HipStr<'a>,
    pub avatar_id: u32,
    #[serde(rename = "UsedAsCoach_MapName")]
    pub used_as_coach_map_name: HipStr<'a>,
    #[serde(rename = "UsedAsCoach_CoachId")]
    pub used_as_coach_coach_id: u32,
    #[serde(rename = "specialEffect")]
    pub special_effect: u32,
    #[serde(rename = "mainAvatarId")]
    pub main_avatar_id: u32,
}

impl Default for AvatarDescription2022<'static> {
    fn default() -> Self {
        Self {
            class: Option::default(),
            jd_version: 2022,
            relative_song_name: HipStr::default(),
            relative_quest_id: HipStr::default(),
            relative_wdf_boss_name: HipStr::default(),
            relative_wdf_tournament_name: HipStr::default(),
            relative_jd_rank: HipStr::default(),
            relative_game_mode_name: HipStr::default(),
            sound_family: HipStr::default(),
            status: Default::default(),
            unlock_type: Default::default(),
            mojo_price: 0,
            wdf_level: 1,
            count_in_progression: 1,
            actor_path: HipStr::default(),
            phone_image: HipStr::default(),
            avatar_id: Default::default(),
            used_as_coach_map_name: HipStr::default(),
            used_as_coach_coach_id: Default::default(),
            special_effect: 0,
            main_avatar_id: u32::from(u16::MAX),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BlockFlowTemplate<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub is_mash_up: u32,
    pub is_party_master: u32,
    pub block_descriptor_vector: Vec<BlockReplacements<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BlockReplacements<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub base_block: BlockDescriptor<'a>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternative_blocks: Vec<BlockDescriptor<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BlockDescriptor<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub song_name: HipStr<'a>,
    pub frst_beat: u32,
    pub last_beat: u32,
    pub song_switch: u32,
    pub video_coach_offset: (f32, f32),
    pub video_coach_scale: f32,
    pub dance_step_name: HipStr<'a>,
    pub playing_speed: f32,
    pub is_entry_point: u32,
    pub is_empty_block: u32,
    pub is_no_score_block: u32,
    pub guid: HipStr<'a>,
    pub force_display_last_pictos: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Country<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "countryID")]
    pub country_id: u32,
    pub country_code: HipStr<'a>,
    pub country_name: HipStr<'a>,
}

const fn theme_default() -> Color {
    Color {
        color: (1.0, 1.0, 1.0, 1.0),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DefaultColors {
    #[serde(default = "theme_default")]
    pub theme: Color,
    pub lyrics: Color,
    #[serde(alias = "songColor_1A", skip_serializing_if = "Option::is_none")]
    pub songcolor_1a: Option<Color>,
    #[serde(alias = "songColor_1B", skip_serializing_if = "Option::is_none")]
    pub songcolor_1b: Option<Color>,
    #[serde(alias = "songColor_2A", skip_serializing_if = "Option::is_none")]
    pub songcolor_2a: Option<Color>,
    #[serde(alias = "songColor_2B", skip_serializing_if = "Option::is_none")]
    pub songcolor_2b: Option<Color>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SongDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub path_creation_formats: PathCreationFormat<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PathCreationFormat<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub timeline: HipStr<'a>,
    // In WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cine_mashup: Option<HipStr<'a>>,
    // Only in 2018
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub videowebm: Option<HipStr<'a>>,
    pub cine: HipStr<'a>,
    pub graph: HipStr<'a>,
    // In WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mainscene: Option<HipStr<'a>>,
    pub video: HipStr<'a>,
    pub videoalpha: HipStr<'a>,
    // Not in WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub videompd: Option<HipStr<'a>>,
    // Not in WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub videomappreview: Option<HipStr<'a>>,
    // In WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cine_partymaster: Option<HipStr<'a>>,
    pub audio: HipStr<'a>,
    // Can be all caps on WiiU
    #[serde(alias = "FX")]
    pub fx: HipStr<'a>,
    // Can be all caps or capitalized on WiiU
    #[serde(alias = "AUTODANCE", alias = "Autodance")]
    pub autodance: HipStr<'a>,
}

const fn default_sweat_difficulty() -> u32 {
    1
}
const fn default_dancer_name() -> HipStr<'static> {
    HipStr::borrowed("Unknown Dancer")
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SongDescription<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub map_name: HipStr<'a>,
    #[serde(rename = "JDVersion")]
    pub jd_version: u32,
    #[serde(rename = "OriginalJDVersion")]
    pub original_jd_version: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_albums: Vec<HipStr<'a>>,
    pub artist: HipStr<'a>,
    /// Only in Chinese version
    #[serde(rename = "CN_Lyrics", default, skip_serializing_if = "Option::is_none")]
    pub cn_lyrics: Option<HipStr<'a>>,
    #[serde(default = "default_dancer_name")]
    pub dancer_name: HipStr<'a>,
    pub title: HipStr<'a>,
    /// Missing in some mods
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credits: Option<HipStr<'a>>,
    /// Only in Chinese version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_title: Option<HipStr<'a>>,
    /// Only in Chinese version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_credits: Option<HipStr<'a>>,
    /// Only in Chinese version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_artist: Option<HipStr<'a>>,
    pub phone_images: PhoneImages<'a>,
    pub num_coach: u32,
    pub main_coach: i32,
    /// Only in versions before nx2020
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_scoring_type: Option<i8>,
    pub difficulty: u32,
    #[serde(default = "default_sweat_difficulty")]
    pub sweat_difficulty: u32,
    #[serde(rename = "backgroundType")]
    pub background_type: u32,
    pub lyrics_type: i32,
    /// Only in nx2017, always 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy: Option<u32>,
    /// Not always there in 2016
    #[serde(default)]
    pub tags: Vec<HipStr<'a>>,
    /// Only in nx2017
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jdm_attributes: Option<Vec<HipStr<'a>>>,
    pub status: u32,
    #[serde(rename = "LocaleID")]
    pub locale_id: LocaleId,
    pub mojo_value: u32,
    pub count_in_progression: u32,
    pub default_colors: DefaultColors,
    /// Not present in nx2017
    #[serde(default)]
    pub video_preview_path: HipStr<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_with_both_controllers: Option<usize>,
    /// Only in versions before nx2020
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub paths: Option<Paths<'a>>,
}

impl SongDescription<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_SongDescTemplate");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Paths<'a> {
    #[serde(borrow, alias = "AsyncPlayers")]
    pub asyncplayers: Option<Vec<HipStr<'a>>>,
    #[serde(borrow, alias = "Avatars")]
    pub avatars: Option<Vec<HipStr<'a>>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MusicTrackComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub track_data: MusicTrackData<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MusicTrackData<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub structure: MusicTrackStructure<'a>,
    pub path: HipStr<'a>,
    pub url: HipStr<'a>,
}

impl MusicTrackData<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("MusicTrackData");
}

#[allow(dead_code)]
struct FloatOrU32 {}

impl<'de> DeserializeAs<'de, u32> for FloatOrU32 {
    fn deserialize_as<D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>
    {
        struct FloatIntVisitor {}
        impl <'de> Visitor<'de> for FloatIntVisitor {
            type Value = u32;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                error!("Called expecting");
                formatter.write_str("a float or integer between 0 and 2^32-1")
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                error!("Called visit_u32: {v}");
                Ok(v)
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                error!("Called visit_f32: {v}");
                trace!("Got f32 {v} instead of u32");
                let v = v.round();
                if v < 0.0 || v > 4_294_967_295.0 {
                    Err(E::custom(format!("float value {} is out of range", v)))
                } else {
                    Ok(v as u32)
                }
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                error!("Called visit_f64: {v}");
                trace!("Got f64 {v} instead of u32");
                let v = v.round();
                if v < 0.0 || v > 4_294_967_295.0 {
                    Err(E::custom(format!("float value {} is out of range", v)))
                } else {
                    Ok(v as u32)
                }
            }
        }
        deserializer.deserialize_any(FloatIntVisitor {})
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde_as]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MusicTrackStructure<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde_as(as = "Vec<FloatOrU32>")]
    pub markers: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signatures: Vec<MusicSignature<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<MusicSection<'a>>,
    pub start_beat: i32,
    pub end_beat: u32,
    #[serde(default)]
    pub fade_start_beat: u32,
    #[serde(default)]
    pub use_fade_start_beat: bool,
    #[serde(default)]
    pub fade_end_beat: u32,
    #[serde(default)]
    pub use_fade_end_beat: bool,
    pub video_start_time: f32,
    pub preview_entry: u32,
    pub preview_loop_start: u32,
    pub preview_loop_end: u32,
    pub volume: f32,
    #[serde(default)]
    pub fade_in_duration: u32,
    #[serde(default)]
    pub fade_in_type: u32,
    #[serde(default)]
    pub fade_out_duration: u32,
    #[serde(default)]
    pub fade_out_type: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_points: Vec<u32>,
}

impl MusicTrackStructure<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("MusicTrackStructure");
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MusicSignature<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub marker: i32,
    pub beats: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<HipStr<'a>>,
}

impl MusicSignature<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("MusicSignature");
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MusicSection<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub marker: i32,
    pub section_type: u32,
    pub comment: HipStr<'a>,
}

impl MusicSection<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("MusicSection");
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MasterTape<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tapes_rack: Vec<TapeGroup<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct TapeGroup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub entries: Vec<TapeEntry<'a>>,
}

impl TapeGroup<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("TapeGroup");
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct TapeEntry<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub label: HipStr<'a>,
    pub path: HipStr<'a>,
}

impl TapeEntry<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("TapeEntry");
}

pub type PhoneImages<'a> = HashMap<HipStr<'a>, HipStr<'a>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PleoComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub video: HipStr<'a>,
    #[serde(rename = "videoURL")]
    pub video_url: HipStr<'a>,
    pub auto_play: u32,
    pub play_from_memory: u32,
    pub play_to_texture: u32,
    #[serde(rename = "loop")]
    pub loop_it: u32,
    /// Not in 2016
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clean_loop: Option<u32>,
    pub alpha: u32,
    pub sound: u32,
    #[serde(rename = "channelID")]
    pub channel_id: HipStr<'a>,
    pub adaptive: u32,
    pub auto_stop_at_the_end: u32,
    // Not in WiiU version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_discard_after_stop: Option<u32>,
    #[serde(rename = "dashMPD")]
    pub dash_mpd: HipStr<'a>,
    pub audio_bus: HipStr<'a>,
    /// Not in 2016
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loop_frame: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Record<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub start: f32,
    pub duration: f32,
}

impl Record<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("Record");
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MaterialGraphicComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub patch_level: u32,
    pub patch_h_level: u32,
    pub patch_v_level: u32,
    #[serde(rename = "visualAABB")]
    pub visual_aabb: AaBb<'a>,
    pub renderintarget: u32,
    pub pos_offset: (u32, u32),
    pub angle_offset: f32,
    pub blendmode: u32,
    pub materialtype: u32,
    pub self_illum_color: Color,
    pub disable_light: u32,
    pub force_disable_light: u32,
    pub use_shadow: u32,
    pub use_root_bone: u32,
    pub shadow_size: (f32, f32),
    pub shadow_material: Box<GFXMaterialSerializable<'a>>,
    pub shadow_attenuation: f32,
    pub shadow_dist: f32,
    pub shadow_offset_pos: (u32, u32, u32),
    pub angle_limit: u32,
    pub material: Box<GFXMaterialSerializable<'a>>,
    pub default_color: Color,
    pub z_offset: u32,
}

impl Default for MaterialGraphicComponent<'static> {
    fn default() -> Self {
        Self {
            class: Option::default(),
            patch_level: Default::default(),
            patch_h_level: 2,
            patch_v_level: 2,
            visual_aabb: AaBb::default(),
            renderintarget: Default::default(),
            pos_offset: Default::default(),
            angle_offset: Default::default(),
            blendmode: 2,
            materialtype: Default::default(),
            self_illum_color: Color::default(),
            disable_light: Default::default(),
            force_disable_light: Default::default(),
            use_shadow: Default::default(),
            use_root_bone: Default::default(),
            shadow_size: Default::default(),
            shadow_material: Box::<GFXMaterialSerializable<'_>>::default(),
            shadow_attenuation: 1.0,
            shadow_dist: Default::default(),
            shadow_offset_pos: Default::default(),
            angle_limit: Default::default(),
            material: Box::<GFXMaterialSerializable<'_>>::default(),
            default_color: Color {
                color: (1.0, 1.0, 1.0, 1.0),
            },
            z_offset: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GFXMaterialSerializable<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub texture_set: GFXMaterialTexturePathSet<'a>,
    #[serde(rename = "ATL_Channel")]
    pub atl_channel: u32,
    #[serde(rename = "ATL_Path")]
    pub atl_path: HipStr<'a>,
    pub shader_path: HipStr<'a>,
    pub material_params: GFXMaterialSerializableParam<'a>,
    /// Not in nx2017-nx2019
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outlined_mask_params: Option<OutlinedMaskMaterialParams<'a>>,
    /// Only in nx2017-nx2019 and nx2020_japan
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stencil_test: Option<u32>,
    pub alpha_test: u32,
    pub alpha_ref: u32,
}

impl GFXMaterialSerializable<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("GFXMaterialSerializable");
}

impl Default for GFXMaterialSerializable<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            texture_set: GFXMaterialTexturePathSet::default(),
            atl_channel: 0,
            atl_path: HipStr::default(),
            shader_path: HipStr::default(),
            material_params: GFXMaterialSerializableParam::default(),
            outlined_mask_params: Some(OutlinedMaskMaterialParams::default()),
            stencil_test: None,
            alpha_test: u32::MAX,
            alpha_ref: u32::MAX,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GFXMaterialTexturePathSet<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub diffuse: HipStr<'a>,
    pub back_light: HipStr<'a>,
    pub normal: HipStr<'a>,
    #[serde(rename = "separateAlpha")]
    pub separate_alpha: HipStr<'a>,
    pub diffuse_2: HipStr<'a>,
    pub back_light_2: HipStr<'a>,
    pub anim_impostor: HipStr<'a>,
    pub diffuse_3: HipStr<'a>,
    pub diffuse_4: HipStr<'a>,
}

impl GFXMaterialTexturePathSet<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("GFXMaterialTexturePathSet");
}

impl Default for GFXMaterialTexturePathSet<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            diffuse: HipStr::default(),
            back_light: HipStr::default(),
            normal: HipStr::default(),
            separate_alpha: HipStr::default(),
            diffuse_2: HipStr::default(),
            back_light_2: HipStr::default(),
            anim_impostor: HipStr::default(),
            diffuse_3: HipStr::default(),
            diffuse_4: HipStr::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GFXMaterialSerializableParam<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "Reflector_factor")]
    pub reflector_factor: u32,
}

impl GFXMaterialSerializableParam<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("GFXMaterialSerializableParam");
}

impl Default for GFXMaterialSerializableParam<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            reflector_factor: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OutlinedMaskMaterialParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub mask_color: Color,
    pub outline_color: Color,
    pub thickness: u32,
}

impl OutlinedMaskMaterialParams<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("OutlinedMaskMaterialParams");
}

impl Default for OutlinedMaskMaterialParams<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            mask_color: Color::default(),
            outline_color: Color::default(),
            thickness: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PleoTextureGraphicComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub patch_level: u32,
    pub patch_h_level: u32,
    pub patch_v_level: u32,
    #[serde(rename = "visualAABB")]
    pub visual_aabb: AaBb<'a>,
    pub renderintarget: u32,
    pub pos_offset: (u32, u32),
    pub angle_offset: f32,
    pub blendmode: u32,
    pub materialtype: u32,
    pub self_illum_color: Color,
    pub disable_light: u32,
    pub force_disable_light: u32,
    pub use_shadow: u32,
    pub use_root_bone: u32,
    pub shadow_size: (f32, f32),
    pub shadow_material: Box<GFXMaterialSerializable<'a>>,
    pub shadow_attenuation: f32,
    pub shadow_dist: f32,
    pub shadow_offset_pos: (u32, u32, u32),
    pub angle_limit: u32,
    pub material: Box<GFXMaterialSerializable<'a>>,
    pub default_color: Color,
    pub z_offset: u32,
    #[serde(rename = "channelID")]
    pub channel_id: HipStr<'a>,
    pub auto_activate: u32,
    pub use_conductor: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE", deny_unknown_fields)]
pub struct AaBb<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub min: (f32, f32),
    pub max: (f32, f32),
}

impl AaBb<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("AABB");
}

impl Default for AaBb<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            min: Default::default(),
            max: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoundComponent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sound_list: Vec<SoundDescriptor<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoundDescriptor<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub name: HipStr<'a>,
    pub volume: f32,
    pub category: HipStr<'a>, // TODO: Make enum
    pub limit_category: HipStr<'a>,
    pub limit_mode: u32,
    pub max_instances: u32,
    pub files: Vec<HipStr<'a>>,
    pub serial_playing_mode: u32,
    pub serial_stopping_mode: u32,
    pub params: SoundParams<'a>,
    pub pause_insensitive_flags: u32,
    pub out_devices: u32,
    #[serde(rename = "soundPlayAfterdestroy")]
    pub sound_play_after_destroy: u32,
}

impl SoundDescriptor<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("SoundDescriptor_Template");
}

impl Default for SoundDescriptor<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            name: HipStr::default(),
            volume: 0.0,
            category: HipStr::borrowed("amb"),
            limit_category: HipStr::default(),
            limit_mode: 0,
            max_instances: u32::MAX,
            files: Vec::new(),
            serial_playing_mode: 0,
            serial_stopping_mode: 0,
            params: SoundParams::default(),
            pause_insensitive_flags: 0,
            out_devices: u32::MAX,
            sound_play_after_destroy: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SoundParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "loop")]
    pub loop_it: u32,
    pub play_mode: u32,
    pub play_mode_input: HipStr<'a>,
    pub random_vol_min: f32,
    pub random_vol_max: f32,
    pub delay: u32,
    pub random_delay: u32,
    /// Not present in nx2017 and earlier
    #[serde(default = "default_pitch")]
    pub pitch: f32,
    pub random_pitch_min: f32,
    pub random_pitch_max: f32,
    pub fade_in_time: f32,
    pub fade_out_time: f32,
    pub filter_frequency: u32,
    pub filter_type: u32,
    /// Not present in nx2016
    #[serde(default)]
    pub transition_sample_offset: u32,
}

const fn default_pitch() -> f32 {
    1.0
}

impl SoundParams<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("SoundParams");
}

impl Default for SoundParams<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            loop_it: 0,
            play_mode: 1,
            play_mode_input: HipStr::default(),
            random_vol_min: 0.0,
            random_vol_max: 0.0,
            delay: 0,
            random_delay: 0,
            pitch: 1.0,
            random_pitch_min: 1.0,
            random_pitch_max: 1.0,
            fade_in_time: 0.0,
            fade_out_time: 0.0,
            filter_frequency: 0,
            filter_type: 2,
            transition_sample_offset: 0,
        }
    }
}
