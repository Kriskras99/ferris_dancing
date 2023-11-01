//--------------------------------------------------------------------------------------
//
// File: Mumomat.fx
//
//--------------------------------------------------------------------------------------

#define CB_MUMOMAT

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

#ifdef _DEBUG_
#include "Debug.fxh"
#endif

#ifdef PIXEL_PROFILE
REGISTER_SAMPLER(TextureSampler, 0);
REGISTER_SAMPLER(TextureSampler, 1);
REGISTER_SAMPLER(TextureSampler, 2);
REGISTER_SAMPLER(TextureSampler, 3);
#endif //PIXEL_PROFILE

struct VS_Input
{
	float4 Position : POSITION;
	float4 Color	: COLOR0;
	float2 UV		: TEXCOORD0;
};

struct VS_Output
{
	float4 Position : VS_OUT_POS;
	float4 Color	: COLOR0;
	float2 UV		: TEXCOORD0;
};

#ifdef VERTEX_PROFILE

VS_Output defaultMat_VS(VS_Input input)
{
	VS_Output output;

	output.Position = mul(input.Position, vs_mWorldViewProjection);
	output.Color = float4( input.Color.rgb,  input.Color.a * vs_globalColor.a );
	output.UV = float4(input.UV, 1, 1);
	
	return output;
}

#endif //VERTEX_PROFILE

#ifdef PIXEL_PROFILE

#ifdef DX11_SHADERS
float4 SampleTex2D(Texture2D tex, SamplerState samp, float2 uv)
#else
float4 SampleTex2D(sampler tex, float2 uv)
#endif
{
#ifdef DX11_SHADERS
    return tex.Sample(samp, uv);
#else
	return tex2D(tex, uv);
#endif
}

struct PS_Output
{
	float4 Color : PS_OUT_COLOR;
};

PS_Output defaultMat_PS(VS_Output input)
{
	// Texture 0
	// R - Colour coordinate
	// G - Bar gradient
	// B - Bar index
	// Texture 1
	// RGB - Interpolated colour
	// Texture 2
	// A - Mask
	
	PS_Output output;
	float4 VertColor = input.Color;

	float4 sample0 = SampleTex2D(SAMPLER_PARAM(TextureSampler, 0), input.UV);
	float4 sample1 = SampleTex2D(SAMPLER_PARAM(TextureSampler, 2), input.UV).w;
	float time = saturate( VertColor.r - ps_mumomat_reg0.x );
	float alpha = VertColor.w;

	float ColourIndex = sample0.r;
	float Mask = sample1.w;

	float4 sample2 = SampleTex2D(SAMPLER_PARAM(TextureSampler, 1), float2( ColourIndex, time ) );
	output.Color = sample2*Mask*alpha;

#if defined(_EQUALISER_) && ( !defined(ITF_PS3) && !defined(ITF_CAFE) && !defined(ITF_NX) )

	float bar_index = (sample0.b/2.0)*255.0f + 0.01f;
	float array_index = ( bar_index/4 );
	float comp_index = bar_index % 4;
	float bar_height;
	
	if ( comp_index > 1.5 )
	{
		if ( comp_index > 2.5 )
		{
			bar_height = ps_mumomat_bars[array_index].w;
		}
		else
		{
			bar_height = ps_mumomat_bars[array_index].z;
		}
	}
	else
	{
		if ( comp_index > 0.5 )
		{
			bar_height = ps_mumomat_bars[array_index].y;
		}
		else
		{
			bar_height = ps_mumomat_bars[array_index].x;
		}
	}
	
	float bar_gradient = saturate( ( ( bar_height - 1 )*ps_mumomat_reg0.y + sample0.g )*ps_mumomat_reg0.z );
	output.Color *= bar_gradient;
	
#endif // _defined(_EQUALISER_) && ( !defined(ITF_PS3) && !defined(ITF_CAFE) && !defined(ITF_NX) )
	
#ifdef DX11_SHADERS
	if (ps_alphaTest.r > 0.0f)
    {
	   if (output.Color.a < ps_alphaTest.g)
	   {
	 	  discard;
	   }
    }
#endif
    
	return output;
}

#endif //PIXEL_PROFILE
