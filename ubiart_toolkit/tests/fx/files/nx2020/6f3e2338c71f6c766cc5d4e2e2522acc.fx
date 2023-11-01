//--------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------
// File: AfterFx.fx
//--------------------------------------------------------------------------------------

#define CB_AFTERFX

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

//--------------------------------------------------------------------------------------
// Vertex shader output structure
//--------------------------------------------------------------------------------------

// NX compiler optimises PS shaders which do not use the Diffuse value by removing it
// which causes the shader to interpolate values from Diffuse, thinking it's getting TextureUV0 data instead
// simplest solution was to invert TextureUV0 & Diffuse in this struct
struct VS_PCT1_OUTPUT
{
    float4 Position   : VS_OUT_POS;   // vertex position 
    float2 TextureUV0  : TEXCOORD0;  // vertex texture coords
    float4 Diffuse    : COLOR0_C;     // vertex diffuse color (note that COLOR0 is clamped from 0..1)
};

struct VS_PCT5_OUTPUT
{
    float4 Position   : VS_OUT_POS;   // vertex position 
    float4 Diffuse    : COLOR0_C;     // vertex diffuse color (note that COLOR0 is clamped from 0..1)
    float4 TextureUV0_1: TEXCOORD0;  // vertex texture coords 
    float4 TextureUV2_3: TEXCOORD1;  // vertex texture coords 
    float4 TextureUV  : TEXCOORD2;  // vertex texture coords 
};

struct PS_OUTPUT
{
    float4 RGBColor : PS_OUT_COLOR;  // Pixel color    
};

#ifdef VERTEX_PROFILE

//--------------------------------------------------------------------------------------
// This shader computes standard transform and lighting
//--------------------------------------------------------------------------------------

VS_PCT5_OUTPUT blur_VS( float4 vPos : POSITION, 
                         float4 fColor : COLOR0,
                         float2 vTexCoord0 : TEXCOORD0 )
{
    VS_PCT5_OUTPUT Output;
        
    Output.Position = mul(vPos, vs_mWorldViewProjection);
    Output.Diffuse = fColor; 
  
    Output.TextureUV0_1 = float4(vTexCoord0.x - vs_afxBlur.x, vTexCoord0.y - vs_afxBlur.y, vTexCoord0.x - vs_afxBlur.x, vTexCoord0.y + vs_afxBlur.y);
    Output.TextureUV2_3 = float4(vTexCoord0.x + vs_afxBlur.x, vTexCoord0.y + vs_afxBlur.y, vTexCoord0.x + vs_afxBlur.x, vTexCoord0.y - vs_afxBlur.y);
    Output.TextureUV    = vTexCoord0.xyxy;

    return Output;    
}

VS_PCT1_OUTPUT PCT1_VS( float4 vPos : POSITION, 
                         float4 fColor : COLOR0,
                         float2 vTexCoord0 : TEXCOORD0 )
{
    VS_PCT1_OUTPUT Output;
        
    Output.Position = mul(vPos, vs_mWorldViewProjection);
    Output.Diffuse = fColor; 
    
    Output.TextureUV0 = vTexCoord0;
    return Output;    
}
#endif // VERTEX_PROFILE

#ifdef PIXEL_PROFILE

//--------------------------------------------------------------------------------------
// Pixels Shaders.
//--------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------
// Texture samplers
//--------------------------------------------------------------------------------------

REGISTER_SAMPLER(TextureSampler, 0)


PS_OUTPUT BigBlur_PS( VS_PCT5_OUTPUT In ) 
{ 
	PS_OUTPUT Output;
	
	float4 color = 0;
	
	color  = TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0_1.xy) * 0.5;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0_1.zw) * 0.5;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV2_3.xy) * 0.5;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV2_3.zw) * 0.5;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV.xy) * 1.0f;
	color /= 3.0f;

    Output.RGBColor = color * In.Diffuse;
    return Output;
}

//	-----------------------------------------------------------------------------------------
//		MERGED POST PROCESS
//	-----------------------------------------------------------------------------------------

PS_OUTPUT mergedEffect_PS( VS_PCT1_OUTPUT In )
{
    PS_OUTPUT Output;
	
	float4 f4_color = In.Diffuse;
	float2 newUV = In.TextureUV0;

//Get Blur and compute glow if needed
#ifdef BLUR
	f4_color.rgb = TEXTURE_READ_2D(TextureSampler, 0, newUV).rgb;
	f4_color.a = ps_AFXParam.z;
#endif

	//Color effect
	Output.RGBColor = f4_color;

    return Output;
}

#endif // PIXEL_PROFILE
