use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::cooked::isc::types::impl_deserialize_for_internally_tagged_enum;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "@NAME", deny_unknown_fields)]
pub enum Value<'a> {
    #[serde(rename = "PropertyPatchValue_Color")]
    Color(WrappedColor),
    #[serde(rename = "PropertyPatchValue_ColorSet")]
    ColorSet(WrappedColorSet<'a>),
    #[serde(rename = "PropertyPatchValue_Float")]
    Float(WrappedFloat),
    #[serde(rename = "PropertyPatchValue_Int")]
    Int(WrappedInt),
    #[serde(rename = "PropertyPatchValue_Path")]
    Path(WrappedPath<'a>),
    #[serde(rename = "PropertyPatchValue_String")]
    String(WrappedString<'a>),
    #[serde(rename = "PropertyPatchValue_Time")]
    Time(WrappedBase),
}

impl_deserialize_for_internally_tagged_enum! {
    Value<'a>, "@NAME",
    ("PropertyPatchValue_Color" => Color(WrappedColor)),
    ("PropertyPatchValue_ColorSet" => ColorSet(WrappedColorSet<'a>)),
    ("PropertyPatchValue_Float" => Float(WrappedFloat)),
    ("PropertyPatchValue_Int" => Int(WrappedInt)),
    ("PropertyPatchValue_Path" => Path(WrappedPath<'a>)),
    ("PropertyPatchValue_String" => String(WrappedString<'a>)),
    ("PropertyPatchValue_Time" => Time(WrappedBase)),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedBase {
    #[serde(rename = "PropertyPatchValue_Base")]
    base: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedColor {
    #[serde(rename = "PropertyPatchValue_Color")]
    color: Color,
}

impl AsRef<Color> for WrappedColor {
    fn as_ref(&self) -> &Color {
        &self.color
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Color {
    #[serde(rename = "@VALUE")]
    pub value: crate::cooked::isc::types::Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedColorSet<'a> {
    #[serde(borrow, rename = "PropertyPatchValue_ColorSet")]
    color_set: ColorSets<'a>,
}

impl<'a> AsRef<ColorSets<'a>> for WrappedColorSet<'a> {
    fn as_ref(&self) -> &ColorSets<'a> {
        &self.color_set
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ColorSets<'a> {
    #[serde(borrow, rename = "colorSet")]
    pub color_set: Vec<ColorSet<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ColorSet<'a> {
    #[serde(rename = "@KEY")]
    pub key: Cow<'a, str>,
    #[serde(rename = "@VAL")]
    pub value: crate::cooked::isc::types::Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedFloat {
    #[serde(rename = "PropertyPatchValue_Float")]
    float: Float,
}

impl AsRef<Float> for WrappedFloat {
    fn as_ref(&self) -> &Float {
        &self.float
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Float {
    #[serde(rename = "@VALUE")]
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedInt {
    #[serde(rename = "PropertyPatchValue_Int")]
    int: Int,
}

impl AsRef<Int> for WrappedInt {
    fn as_ref(&self) -> &Int {
        &self.int
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Int {
    #[serde(rename = "@VALUE")]
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPath<'a> {
    #[serde(borrow, rename = "PropertyPatchValue_Path")]
    path: Path<'a>,
}

impl<'a> AsRef<Path<'a>> for WrappedPath<'a> {
    fn as_ref(&self) -> &Path<'a> {
        &self.path
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Path<'a> {
    #[serde(rename = "@VALUE")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedString<'a> {
    #[serde(borrow, rename = "PropertyPatchValue_String")]
    string: String<'a>,
}

impl<'a> AsRef<String<'a>> for WrappedString<'a> {
    fn as_ref(&self) -> &String<'a> {
        &self.string
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct String<'a> {
    #[serde(rename = "@VALUE")]
    pub value: Cow<'a, str>,
}
