use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::impl_deserialize_for_internally_tagged_enum;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "@NAME", deny_unknown_fields)]
pub enum Action<'a> {
    #[serde(rename = "PropertyPatchAction_ColorActor")]
    ColorActor,
    #[serde(rename = "PropertyPatchAction_ColorDiffuse")]
    ColorDiffuse(WrappedColorDiffuse<'a>),
    #[serde(rename = "PropertyPatchAction_FormattedDate")]
    FormattedDate(WrappedFormattedDate<'a>),
    #[serde(rename = "PropertyPatchAction_FormattedText")]
    FormattedText(WrappedFormattedText<'a>),
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "PropertyPatchAction_PleoTextureChannel")]
    PleoTextureChannel,
    #[serde(rename = "PropertyPatchAction_Redirection")]
    Redirection(WrappedRedirection<'a>),
    #[serde(rename = "PropertyPatchAction_Spinner")]
    Spinner(WrappedSpinner<'a>),
    #[serde(rename = "PropertyPatchAction_TapeSlider")]
    TapeSlider(WrappedTapeSlider<'a>),
    #[serde(rename = "PropertyPatchAction_TapeSwitch")]
    TapeSwitch(WrappedTapeSwitch<'a>),
    #[serde(rename = "PropertyPatchAction_Text")]
    Text(WrappedText<'a>),
    #[serde(rename = "PropertyPatchAction_Texture")]
    Texture(WrappedTexture<'a>),
}

impl_deserialize_for_internally_tagged_enum! {
    Action<'a>, "@NAME",
    ("PropertyPatchAction_ColorActor" => ColorActor),
    ("PropertyPatchAction_ColorDiffuse" => ColorDiffuse(WrappedColorDiffuse)),
    ("PropertyPatchAction_FormattedDate" => FormattedDate(WrappedFormattedDate)),
    ("PropertyPatchAction_FormattedText" => FormattedText(WrappedFormattedText)),
    ("" => Empty),
    ("PropertyPatchAction_PleoTextureChannel" => PleoTextureChannel),
    ("PropertyPatchAction_Redirection" => Redirection(WrappedRedirection)),
    ("PropertyPatchAction_Spinner" => Spinner(WrappedSpinner)),
    ("PropertyPatchAction_TapeSlider" => TapeSlider(WrappedTapeSlider)),
    ("PropertyPatchAction_TapeSwitch" => TapeSwitch(WrappedTapeSwitch)),
    ("PropertyPatchAction_Text" => Text(WrappedText)),
    ("PropertyPatchAction_Texture" => Texture(WrappedTexture)),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedColorDiffuse<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_ColorDiffuse")]
    color_diffuse: ColorDiffuse<'a>,
}

impl<'a> AsRef<ColorDiffuse<'a>> for WrappedColorDiffuse<'a> {
    fn as_ref(&self) -> &ColorDiffuse<'a> {
        &self.color_diffuse
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ColorDiffuse<'a> {
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
pub struct WrappedFormattedDate<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_FormattedDate")]
    formatted_string: FormattedString<'a>,
}

impl<'a> AsRef<FormattedString<'a>> for WrappedFormattedDate<'a> {
    fn as_ref(&self) -> &FormattedString<'a> {
        &self.formatted_string
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedFormattedText<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_FormattedText")]
    formatted_string: FormattedString<'a>,
}

impl<'a> AsRef<FormattedString<'a>> for WrappedFormattedText<'a> {
    fn as_ref(&self) -> &FormattedString<'a> {
        &self.formatted_string
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct FormattedString<'a> {
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
pub struct WrappedRedirection<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_Redirection")]
    redirection: Redirection<'a>,
}

impl<'a> AsRef<Redirection<'a>> for WrappedRedirection<'a> {
    fn as_ref(&self) -> &Redirection<'a> {
        &self.redirection
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Redirection<'a> {
    #[serde(rename = "@subMarker")]
    pub sub_marker: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedSpinner<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_Spinner")]
    spinner: Spinner<'a>,
}

impl<'a> AsRef<Spinner<'a>> for WrappedSpinner<'a> {
    fn as_ref(&self) -> &Spinner<'a> {
        &self.spinner
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Spinner<'a> {
    #[serde(rename = "@loadingAnim")]
    pub loading_anim: Cow<'a, str>,
    #[serde(rename = "@loadedAnim")]
    pub loaded_anim: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedTapeSlider<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_TapeSlider")]
    tape_slider: TapeSlider<'a>,
}

impl<'a> AsRef<TapeSlider<'a>> for WrappedTapeSlider<'a> {
    fn as_ref(&self) -> &TapeSlider<'a> {
        &self.tape_slider
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TapeSlider<'a> {
    #[serde(rename = "@TapeLabel")]
    pub tape_label: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedTapeSwitch<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_TapeSwitch")]
    tape_switch: TapeSwitches<'a>,
}

impl<'a> AsRef<TapeSwitches<'a>> for WrappedTapeSwitch<'a> {
    fn as_ref(&self) -> &TapeSwitches<'a> {
        &self.tape_switch
    }
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
    #[serde(rename = "@VAL")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedText<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_Text")]
    text: Text<'a>,
}

impl<'a> AsRef<Text<'a>> for WrappedText<'a> {
    fn as_ref(&self) -> &Text<'a> {
        &self.text
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Text<'a> {
    #[serde(rename = "@duplicationCount")]
    pub duplication_count: Cow<'a, str>,
    #[serde(rename = "@duplicationSeperator")]
    pub duplication_seperator: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedTexture<'a> {
    #[serde(borrow, rename = "PropertyPatchAction_Texture")]
    texture: Texture<'a>,
}

impl<'a> AsRef<Texture<'a>> for WrappedTexture<'a> {
    fn as_ref(&self) -> &Texture<'a> {
        &self.texture
    }
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
    #[serde(rename = "@VAL")]
    pub value: Cow<'a, str>,
}
