use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use self::action::Action;
use self::value::Value;

pub mod action;
pub mod value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[allow(clippy::module_name_repetitions, reason = "Otherwise it's `Wrapped` or conflicts with `PropertyPatcher`")]
#[repr(transparent)]
pub struct WrappedPropertyPatcher<'a> {
    #[serde(borrow)]
    property_patcher: PropertyPatcher<'a>,
}

impl<'a> AsRef<PropertyPatcher<'a>> for WrappedPropertyPatcher<'a> {
    fn as_ref(&self) -> &PropertyPatcher<'a> {
        &self.property_patcher
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatcher<'a> {
    #[serde(rename = "@applyOnActivation", serialize_with = "super::ser_bool")]
    pub apply_on_activation: bool,
    #[serde(
        rename = "@ignoreActorsInSubScenes",
        serialize_with = "super::ser_option_bool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_actors_in_sub_scenes: Option<bool>,
    #[serde(
        borrow,
        rename = "propertyPatches",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_patches: Vec<PropertyPatches<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatches<'a> {
    #[serde(borrow, rename = "PropertyPatch")]
    pub property_patch: PropertyPatch<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PropertyPatch<'a> {
    #[serde(rename = "@marker")]
    pub marker: Cow<'a, str>,
    #[serde(rename = "@invertActivationApply", serialize_with = "super::ser_bool")]
    pub invert_activation_apply: bool,
    #[serde(
        rename = "@patchedOnDataStatusChanged",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub patched_on_data_status_changed: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub action: Action<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<Value<'a>>,
}
