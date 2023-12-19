use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::cooked::isc::types::impl_deserialize_for_internally_tagged_enum;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "@NAME", deny_unknown_fields)]
pub enum WrappedPropertyPatchAction<'a> {
    #[serde(rename = "PropertyPatchAction_ColorActor")]
    ColorActor,
    #[serde(rename = "PropertyPatchAction_ColorDiffuse")]
    ColorDiffuse(WrappedPropertyPatchActionColorDiffuse<'a>),
    #[serde(rename = "PropertyPatchAction_FormattedDate")]
    FormattedDate(WrappedPropertyPatchActionFormattedDate<'a>),
    #[serde(rename = "PropertyPatchAction_FormattedText")]
    FormattedText(WrappedPropertyPatchActionFormattedText<'a>),
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "PropertyPatchAction_PleoTextureChannel")]
    PleoTextureChannel,
    #[serde(rename = "PropertyPatchAction_Redirection")]
    Redirection(WrappedPropertyPatchActionRedirection<'a>),
    #[serde(rename = "PropertyPatchAction_Spinner")]
    Spinner(WrappedPropertyPatchActionSpinner<'a>),
    #[serde(rename = "PropertyPatchAction_TapeSlider")]
    TapeSlider(WrappedPropertyPatchActionTapeSlider<'a>),
    #[serde(rename = "PropertyPatchAction_TapeSwitch")]
    TapeSwitch(WrappedPropertyPatchActionTapeSwitch<'a>),
    #[serde(rename = "PropertyPatchAction_Text")]
    Text(WrappedPropertyPatchActionText<'a>),
    #[serde(rename = "PropertyPatchAction_Texture")]
    Texture(WrappedPropertyPatchActionTexture<'a>),
}

impl_deserialize_for_internally_tagged_enum! {
    WrappedPropertyPatchAction<'a>, "@NAME",
    ("PropertyPatchAction_ColorActor" => ColorActor),
    ("PropertyPatchAction_ColorDiffuse" => ColorDiffuse(WrappedPropertyPatchActionColorDiffuse)),
    ("PropertyPatchAction_FormattedDate" => FormattedDate(WrappedPropertyPatchActionFormattedDate)),
    ("PropertyPatchAction_FormattedText" => FormattedText(WrappedPropertyPatchActionFormattedText)),
    ("" => Empty),
    ("PropertyPatchAction_PleoTextureChannel" => PleoTextureChannel),
    ("PropertyPatchAction_Redirection" => Redirection(WrappedPropertyPatchActionRedirection)),
    ("PropertyPatchAction_Spinner" => Spinner(WrappedPropertyPatchActionSpinner)),
    ("PropertyPatchAction_TapeSlider" => TapeSlider(WrappedPropertyPatchActionTapeSlider)),
    ("PropertyPatchAction_TapeSwitch" => TapeSwitch(WrappedPropertyPatchActionTapeSwitch)),
    ("PropertyPatchAction_Text" => Text(WrappedPropertyPatchActionText)),
    ("PropertyPatchAction_Texture" => Texture(WrappedPropertyPatchActionTexture)),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchActionColorDiffuse<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_ColorDiffuse")]
    property_patch_action_color_diffuse: PropertyPatchActionColorDiffuse<'a>,
}

impl<'a> AsRef<PropertyPatchActionColorDiffuse<'a>> for WrappedPropertyPatchActionColorDiffuse<'a> {
    fn as_ref(&self) -> &PropertyPatchActionColorDiffuse<'a> {
        &self.property_patch_action_color_diffuse
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchActionColorDiffuse<'a> {
    #[serde(borrow, rename = "ColorPatches")]
    pub color_patches: Vec<ColorPatch<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ColorPatch<'a> {
    #[serde(rename = "@KEY")]
    pub key: Cow<'a, str>,
    #[serde(rename = "@VAL")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchActionFormattedDate<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_FormattedDate")]
    property_patch_action_formatted_string: PropertyPatchActionFormattedString<'a>,
}

impl<'a> AsRef<PropertyPatchActionFormattedString<'a>>
    for WrappedPropertyPatchActionFormattedDate<'a>
{
    fn as_ref(&self) -> &PropertyPatchActionFormattedString<'a> {
        &self.property_patch_action_formatted_string
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchActionFormattedText<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_FormattedText")]
    property_patch_action_formatted_string: PropertyPatchActionFormattedString<'a>,
}

impl<'a> AsRef<PropertyPatchActionFormattedString<'a>>
    for WrappedPropertyPatchActionFormattedText<'a>
{
    fn as_ref(&self) -> &PropertyPatchActionFormattedString<'a> {
        &self.property_patch_action_formatted_string
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchActionFormattedString<'a> {
    #[serde(rename = "@formatLocId")]
    pub format_loc_id: u32,
    #[serde(rename = "@formatRawText")]
    pub format_raw_text: Cow<'a, str>,
    #[serde(rename = "@varOpeningBracket")]
    pub var_opening_bracket: Cow<'a, str>,
    #[serde(rename = "@varClosingBracket")]
    pub var_closing_bracket: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchActionRedirection<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_Redirection")]
    property_patch_action_redirection: PropertyPatchActionRedirection<'a>,
}

impl<'a> AsRef<PropertyPatchActionRedirection<'a>> for WrappedPropertyPatchActionRedirection<'a> {
    fn as_ref(&self) -> &PropertyPatchActionRedirection<'a> {
        &self.property_patch_action_redirection
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchActionRedirection<'a> {
    #[serde(rename = "@subMarker")]
    pub sub_marker: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchActionSpinner<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_Spinner")]
    property_patch_action_spinner: PropertyPatchActionSpinner<'a>,
}

impl<'a> AsRef<PropertyPatchActionSpinner<'a>> for WrappedPropertyPatchActionSpinner<'a> {
    fn as_ref(&self) -> &PropertyPatchActionSpinner<'a> {
        &self.property_patch_action_spinner
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchActionSpinner<'a> {
    #[serde(rename = "@loadingAnim")]
    pub loading_anim: Cow<'a, str>,
    #[serde(rename = "@loadedAnim")]
    pub loaded_anim: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchActionTapeSlider<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_TapeSlider")]
    property_patch_action_tape_slider: PropertyPatchActionTapeSlider<'a>,
}

impl<'a> AsRef<PropertyPatchActionTapeSlider<'a>> for WrappedPropertyPatchActionTapeSlider<'a> {
    fn as_ref(&self) -> &PropertyPatchActionTapeSlider<'a> {
        &self.property_patch_action_tape_slider
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchActionTapeSlider<'a> {
    #[serde(rename = "@TapeLabel")]
    pub tape_label: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchActionTapeSwitch<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_TapeSwitch")]
    property_patch_action_tape_switch: PropertyPatchActionTapeSwitch<'a>,
}

impl<'a> AsRef<PropertyPatchActionTapeSwitch<'a>> for WrappedPropertyPatchActionTapeSwitch<'a> {
    fn as_ref(&self) -> &PropertyPatchActionTapeSwitch<'a> {
        &self.property_patch_action_tape_switch
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchActionTapeSwitch<'a> {
    #[serde(borrow, rename = "TapeSwitch")]
    pub tape_switches: Vec<TapeSwitch<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TapeSwitch<'a> {
    #[serde(rename = "@KEY")]
    pub key: i32,
    #[serde(rename = "@VAL")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchActionText<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_Text")]
    property_patch_action_text: PropertyPatchActionText<'a>,
}

impl<'a> AsRef<PropertyPatchActionText<'a>> for WrappedPropertyPatchActionText<'a> {
    fn as_ref(&self) -> &PropertyPatchActionText<'a> {
        &self.property_patch_action_text
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchActionText<'a> {
    #[serde(rename = "@duplicationCount")]
    pub duplication_count: Cow<'a, str>,
    #[serde(rename = "@duplicationSeperator")]
    pub duplication_seperator: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchActionTexture<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_Texture")]
    property_patch_action_texture: PropertyPatchActionTexture<'a>,
}

impl<'a> AsRef<PropertyPatchActionTexture<'a>> for WrappedPropertyPatchActionTexture<'a> {
    fn as_ref(&self) -> &PropertyPatchActionTexture<'a> {
        &self.property_patch_action_texture
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchActionTexture<'a> {
    #[serde(
        rename = "@MaterialIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub material_index: Option<u32>,
    #[serde(borrow, rename = "TexturePatches")]
    pub texture_patches: Vec<TexturePatch<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TexturePatch<'a> {
    #[serde(rename = "@VAL")]
    pub value: Cow<'a, str>,
}
