use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{impl_deserialize_for_internally_tagged_enum, wrap};

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
    Time(Base),
}

impl_deserialize_for_internally_tagged_enum! {
    Value<'a>, "@NAME",
    ("PropertyPatchValue_Color" => Color(WrappedColor)),
    ("PropertyPatchValue_ColorSet" => ColorSet(WrappedColorSet<'a>)),
    ("PropertyPatchValue_Float" => Float(WrappedFloat)),
    ("PropertyPatchValue_Int" => Int(WrappedInt)),
    ("PropertyPatchValue_Path" => Path(WrappedPath<'a>)),
    ("PropertyPatchValue_String" => String(WrappedString<'a>)),
    ("PropertyPatchValue_Time" => Time(Base)),
}

wrap!(WrappedColor, Color, "PropertyPatchValue_Color");
wrap!(WrappedColorSet, ColorSets, "PropertyPatchValue_ColorSet", 'a);
wrap!(WrappedFloat, Float, "PropertyPatchValue_Float");
wrap!(WrappedInt, Int, "PropertyPatchValue_Int");
wrap!(WrappedPath, Path, "PropertyPatchValue_Path", 'a);
wrap!(WrappedString, String, "PropertyPatchValue_String", 'a);
wrap!(Base, "PropertyPatchValue_Base");

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Color {
    #[serde(rename = "@VALUE")]
    pub value: ubiart_toolkit_shared_types::Color,
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
    pub value: ubiart_toolkit_shared_types::Color,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Float {
    #[serde(rename = "@VALUE")]
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Int {
    #[serde(rename = "@VALUE")]
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Path<'a> {
    #[serde(rename = "@VALUE")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct String<'a> {
    #[serde(rename = "@VALUE")]
    pub value: Cow<'a, str>,
}
