use hipstr::HipStr;
use serde::{Deserialize, Serialize};
use ubiart_toolkit_shared_types::errors::ParserError;

pub fn parse(data: &[u8]) -> Result<GFXMaterialShader<'_>, ParserError> {
    let res = crate::utils::json::parse(data, false)?;
    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GFXMaterialShader<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub flags: u32,
    pub render_regular: u32,
    pub use_alpha_test: u32,
    pub alpha_ref: u32,
    pub separate_alpha: u32,
    pub texture_blend: u32,
    // Not in 2018 and earlier
    #[serde(rename = "UseMultiChannel")]
    pub use_multi_channel: Option<u32>,
    pub materialtype: u32,
    #[serde(borrow)]
    pub mat_params: GFXMaterialParams<'a>,
    pub blendmode: u32,
    #[serde(borrow, rename = "Layer1")]
    pub layer_1: MaterialLayer<'a>,
    #[serde(rename = "BlendLayer2")]
    pub blend_layer_2: u32,
    #[serde(borrow, rename = "Layer2")]
    pub layer_2: MaterialLayer<'a>,
    #[serde(rename = "BlendLayer3")]
    pub blend_layer_3: u32,
    #[serde(borrow, rename = "Layer3")]
    pub layer_3: MaterialLayer<'a>,
    #[serde(rename = "BlendLayer4")]
    pub blend_layer_4: u32,
    #[serde(borrow, rename = "Layer4")]
    pub layer_4: MaterialLayer<'a>,
}

impl GFXMaterialShader<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("GFXMaterialShader_Template");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GFXMaterialParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "matParams0F")]
    pub mat_params_0f: u32,
    #[serde(rename = "matParams1F")]
    pub mat_params_1f: u32,
    #[serde(rename = "matParams2F")]
    pub mat_params_2f: u32,
    #[serde(rename = "matParams3F")]
    pub mat_params_3f: u32,
    #[serde(rename = "matParams0I")]
    pub mat_params_0i: u32,
    #[serde(rename = "matParams0VX")]
    pub mat_params_0vx: u32,
    #[serde(rename = "matParams0VY")]
    pub mat_params_0vy: u32,
    #[serde(rename = "matParams0VZ")]
    pub mat_params_0vz: u32,
    #[serde(rename = "matParams0VW")]
    pub mat_params_0vw: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialLayer<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub enabled: u32,
    pub alpha_threshold: f32,
    pub tex_adressing_mode_u: u32,
    pub tex_adressing_mode_v: u32,
    pub filtering: u32,
    pub diffuse_color: [f32; 4],
    pub texture_usage: u32,
    #[serde(
        borrow,
        rename = "UVModifiers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub uv_modifiers: Vec<UVModifier<'a>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UVModifier<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub translation_u: f32,
    pub translation_v: f32,
    pub anim_translation_u: u32,
    pub anim_translation_v: u32,
    pub rotation: f64,
    pub rotation_offset_u: f32,
    pub rotation_offset_v: f32,
    pub anim_rotation: u32,
    pub scale_u: f32,
    pub scale_v: f32,
    pub scale_offset_u: f32,
    pub scale_offset_v: f32,
}
