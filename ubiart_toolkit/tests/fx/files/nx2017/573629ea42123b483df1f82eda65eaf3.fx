//--------------------------------------------------------------------------------------
//
// File: ConversionUtils.fx
//
//--------------------------------------------------------------------------------------

#define CB_CONVERSION

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

struct VS_OUT
{
    float4 Pos   : VS_OUT_POS;
    float2 UV    : TEXCOORD0;
};

#ifdef VERTEX_PROFILE

VS_OUT VS_QuadUV(	float4 vPos      : POSITION,
                    float4 fColor	 : COLOR0,
                    float2 vTexture  : TEXCOORD0	)
{
    VS_OUT Out;
    Out.Pos = mul(vPos, vs_mWorldViewProjection);
    Out.UV = vTexture;
    return Out;
}

#endif

#ifdef PIXEL_PROFILE

REGISTER_SAMPLER(TextureSampler, 0)
REGISTER_SAMPLER(TextureSampler, 1)

float4 PS_Icon(VS_OUT In) : PS_OUT_COLOR
{
    return TEXTURE_READ_2D(TextureSampler, 0, In.UV);
}

float4 PS_YUV2RGB(VS_OUT In) : PS_OUT_COLOR
{
    // YUV to RGB conversion in the DRC Camera case
    // Please refer to the doc on the Camera API in the Cafe specs
    
    float3 YUV;
    YUV.x = TEXTURE_READ_2D(TextureSampler, 0, In.UV).r;
    YUV.yz = TEXTURE_READ_2D(TextureSampler, 1, In.UV).rg - float2(0.5, 0.5);
    
    float3 RGB;
    RGB.r = YUV.x + 1.402 * YUV.z;
    RGB.g = YUV.x - 0.714 * YUV.z - 0.344 * YUV.y;
    RGB.b = YUV.x + 1.772 * YUV.y;
    return float4(RGB, 1);
}

float4 PS_RGB2Y(VS_OUT In) : PS_OUT_COLOR
{
    float3 RGB;
    RGB = TEXTURE_READ_2D(TextureSampler, 0, In.UV).rgb;

    float Y = 0.2990 * RGB.r + 0.5871 * RGB.g + 0.1140 * RGB.b;

    return float4(Y, 0, 0, 0);
}

float4 PS_RGB2UV(VS_OUT In) : PS_OUT_COLOR
{
    float3 RGB;
    RGB = TEXTURE_READ_2D(TextureSampler, 0, In.UV).rgb;

    float U;
    float V;

    U = -0.1687 * RGB.r -0.3313 * RGB.g + 0.5000 * RGB.b;
    V = 0.5000 * RGB.r -0.4187 * RGB.g -0.0813 * RGB.r;

    return float4(0.5 + U, 0.5 + V, 0, 0);
}

#endif // PIXEL_PROFILE
