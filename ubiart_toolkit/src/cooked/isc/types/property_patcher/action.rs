use hipstr::HipStr;
use serde::{Deserialize, Serialize};

use crate::cooked::isc::{impl_deserialize_for_internally_tagged_enum, wrap};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "@NAME", deny_unknown_fields)]
pub enum Action<'a> {
    #[serde(rename = "PropertyPatchAction_ColorActor")]
    ColorActor(ColorActor),
    #[serde(borrow, rename = "PropertyPatchAction_ColorDiffuse")]
    ColorDiffuse(WrappedColorDiffuse<'a>),
    #[serde(borrow, rename = "PropertyPatchAction_FormattedDate")]
    FormattedDate(WrappedFormattedDate<'a>),
    #[serde(borrow, rename = "PropertyPatchAction_FormattedText")]
    FormattedText(WrappedFormattedText<'a>),
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "PropertyPatchAction_PleoTextureChannel")]
    PleoTextureChannel(PleoTextureChannel),
    #[serde(borrow, rename = "PropertyPatchAction_Redirection")]
    Redirection(WrappedRedirection<'a>),
    #[serde(borrow, rename = "PropertyPatchAction_Spinner")]
    Spinner(WrappedSpinner<'a>),
    #[serde(borrow, rename = "PropertyPatchAction_TapeSlider")]
    TapeSlider(WrappedTapeSlider<'a>),
    #[serde(borrow, rename = "PropertyPatchAction_TapeSwitch")]
    TapeSwitch(WrappedTapeSwitch<'a>),
    #[serde(borrow, rename = "PropertyPatchAction_Text")]
    Text(WrappedText<'a>),
    #[serde(borrow, rename = "PropertyPatchAction_Texture")]
    Texture(WrappedTexture<'a>),
}

impl_deserialize_for_internally_tagged_enum! {
    Action<'a>, "@NAME",
    ("PropertyPatchAction_ColorActor" => ColorActor(ColorActor)),
    ("PropertyPatchAction_ColorDiffuse" => ColorDiffuse(WrappedColorDiffuse)),
    ("PropertyPatchAction_FormattedDate" => FormattedDate(WrappedFormattedDate)),
    ("PropertyPatchAction_FormattedText" => FormattedText(WrappedFormattedText)),
    ("" => Empty),
    ("PropertyPatchAction_PleoTextureChannel" => PleoTextureChannel(PleoTextureChannel)),
    ("PropertyPatchAction_Redirection" => Redirection(WrappedRedirection)),
    ("PropertyPatchAction_Spinner" => Spinner(WrappedSpinner)),
    ("PropertyPatchAction_TapeSlider" => TapeSlider(WrappedTapeSlider)),
    ("PropertyPatchAction_TapeSwitch" => TapeSwitch(WrappedTapeSwitch)),
    ("PropertyPatchAction_Text" => Text(WrappedText)),
    ("PropertyPatchAction_Texture" => Texture(WrappedTexture)),
}

wrap!(ColorActor, "PropertyPatchAction_ColorActor");
wrap!(WrappedColorDiffuse, ColorDiffuse, "PropertyPatchAction_ColorDiffuse", 'a);
wrap!(WrappedFormattedDate, FormattedString, "PropertyPatchAction_FormattedDate", 'a);
wrap!(WrappedFormattedText, FormattedString, "PropertyPatchAction_FormattedText", 'a);
wrap!(PleoTextureChannel, "PropertyPatchAction_PleoTextureChannel");
wrap!(WrappedRedirection, Redirection, "PropertyPatchAction_Redirection", 'a);
wrap!(WrappedSpinner, Spinner, "PropertyPatchAction_Spinner", 'a);
wrap!(WrappedTapeSlider, TapeSlider, "PropertyPatchAction_TapeSlider", 'a);
wrap!(WrappedTapeSwitch, TapeSwitches, "PropertyPatchAction_TapeSwitch", 'a);
wrap!(WrappedText, Text, "PropertyPatchAction_Text", 'a);
wrap!(WrappedTexture, Texture, "PropertyPatchAction_Texture", 'a);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ColorDiffuse<'a> {
    #[serde(borrow, rename = "ColorPatches")]
    pub color_patches: Vec<ColorPatch<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ColorPatch<'a> {
    #[serde(borrow, rename = "@KEY")]
    pub key: HipStr<'a>,
    #[serde(borrow, rename = "@VAL")]
    pub value: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct FormattedString<'a> {
    #[serde(rename = "@formatLocId")]
    pub format_loc_id: u32,
    #[serde(borrow, rename = "@formatRawText")]
    pub format_raw_text: HipStr<'a>,
    #[serde(borrow, rename = "@varOpeningBracket")]
    pub var_opening_bracket: HipStr<'a>,
    #[serde(borrow, rename = "@varClosingBracket")]
    pub var_closing_bracket: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Redirection<'a> {
    #[serde(borrow, rename = "@subMarker")]
    pub sub_marker: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Spinner<'a> {
    #[serde(borrow, rename = "@loadingAnim")]
    pub loading_anim: HipStr<'a>,
    #[serde(borrow, rename = "@loadedAnim")]
    pub loaded_anim: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TapeSlider<'a> {
    #[serde(borrow, rename = "@TapeLabel")]
    pub tape_label: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TapeSwitches<'a> {
    #[serde(borrow, rename = "TapeSwitch")]
    pub tape_switches: Vec<TapeSwitch<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TapeSwitch<'a> {
    #[serde(rename = "@KEY")]
    pub key: i32,
    #[serde(borrow, rename = "@VAL")]
    pub value: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Text<'a> {
    #[serde(borrow, rename = "@duplicationCount")]
    pub duplication_count: HipStr<'a>,
    #[serde(borrow, rename = "@duplicationSeperator")]
    pub duplication_seperator: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Texture<'a> {
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
    #[serde(borrow, rename = "@VAL")]
    pub value: HipStr<'a>,
}
