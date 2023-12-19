use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::cooked::isc::types::{impl_deserialize_for_internally_tagged_enum, Color};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "@NAME", deny_unknown_fields)]
pub enum WrappedPropertyPatchValue<'a> {
    #[serde(rename = "PropertyPatchValue_Color")]
    Color(WrappedPropertyPatchValueColor),
    #[serde(rename = "PropertyPatchValue_ColorSet")]
    ColorSet(WrappedPropertyPatchValueColorSet<'a>),
    #[serde(rename = "PropertyPatchValue_Float")]
    Float(WrappedPropertyPatchValueFloat),
    #[serde(rename = "PropertyPatchValue_Int")]
    Int(WrappedPropertyPatchValueInt),
    #[serde(rename = "PropertyPatchValue_Path")]
    Path(WrappedPropertyPatchValuePath<'a>),
    #[serde(rename = "PropertyPatchValue_String")]
    String(WrappedPropertyPatchValueString<'a>),
    #[serde(rename = "PropertyPatchValue_Time")]
    Time(WrappedPropertyPatchValueBase),
}

impl_deserialize_for_internally_tagged_enum! {
    WrappedPropertyPatchValue<'a>, "@NAME",
    ("PropertyPatchValue_Color" => Color(WrappedPropertyPatchValueColor)),
    ("PropertyPatchValue_ColorSet" => ColorSet(WrappedPropertyPatchValueColorSet<'a>)),
    ("PropertyPatchValue_Float" => Float(WrappedPropertyPatchValueFloat)),
    ("PropertyPatchValue_Int" => Int(WrappedPropertyPatchValueInt)),
    ("PropertyPatchValue_Path" => Path(WrappedPropertyPatchValuePath<'a>)),
    ("PropertyPatchValue_String" => String(WrappedPropertyPatchValueString<'a>)),
    ("PropertyPatchValue_Time" => Time(WrappedPropertyPatchValueBase)),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchValueBase {
    #[serde(rename = "PropertyPatchValue_Base")]
    property_patch_value_base: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchValueColor {
    #[serde(rename = "PropertyPatchValue_Color")]
    property_patch_value_color: PropertyPatchValueColor,
}

impl AsRef<PropertyPatchValueColor> for WrappedPropertyPatchValueColor {
    fn as_ref(&self) -> &PropertyPatchValueColor {
        &self.property_patch_value_color
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchValueColor {
    #[serde(rename = "@VALUE")]
    pub value: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchValueColorSet<'a> {
    #[serde(borrow, rename = "PropertyPatchValue_ColorSet")]
    property_patch_value_color_set: PropertyPatchValueColorSet<'a>,
}

impl<'a> AsRef<PropertyPatchValueColorSet<'a>> for WrappedPropertyPatchValueColorSet<'a> {
    fn as_ref(&self) -> &PropertyPatchValueColorSet<'a> {
        &self.property_patch_value_color_set
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchValueColorSet<'a> {
    #[serde(borrow, rename = "colorSet")]
    pub color_set: Vec<ColorSet<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ColorSet<'a> {
    #[serde(rename = "@KEY")]
    pub key: Cow<'a, str>,
    #[serde(rename = "@VAL")]
    pub value: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchValueFloat {
    #[serde(rename = "PropertyPatchValue_Float")]
    property_patch_value_float: PropertyPatchValueFloat,
}

impl AsRef<PropertyPatchValueFloat> for WrappedPropertyPatchValueFloat {
    fn as_ref(&self) -> &PropertyPatchValueFloat {
        &self.property_patch_value_float
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchValueFloat {
    #[serde(rename = "@VALUE")]
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchValueInt {
    #[serde(rename = "PropertyPatchValue_Int")]
    property_patch_value_int: PropertyPatchValueInt,
}

impl AsRef<PropertyPatchValueInt> for WrappedPropertyPatchValueInt {
    fn as_ref(&self) -> &PropertyPatchValueInt {
        &self.property_patch_value_int
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchValueInt {
    #[serde(rename = "@VALUE")]
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchValuePath<'a> {
    #[serde(borrow, rename = "PropertyPatchValue_Path")]
    property_patch_value_path: PropertyPatchValuePath<'a>,
}

impl<'a> AsRef<PropertyPatchValuePath<'a>> for WrappedPropertyPatchValuePath<'a> {
    fn as_ref(&self) -> &PropertyPatchValuePath<'a> {
        &self.property_patch_value_path
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchValuePath<'a> {
    #[serde(rename = "@VALUE")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[repr(transparent)]
pub struct WrappedPropertyPatchValueString<'a> {
    #[serde(borrow, rename = "PropertyPatchValue_String")]
    property_patch_value_string: PropertyPatchValueString<'a>,
}

impl<'a> AsRef<PropertyPatchValueString<'a>> for WrappedPropertyPatchValueString<'a> {
    fn as_ref(&self) -> &PropertyPatchValueString<'a> {
        &self.property_patch_value_string
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatchValueString<'a> {
    #[serde(rename = "@VALUE")]
    pub value: Cow<'a, str>,
}
