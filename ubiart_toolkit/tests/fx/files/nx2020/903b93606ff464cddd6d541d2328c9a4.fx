//--------------------------------------------------------------------------------------
//
// File: Multitexture.fx
//
//--------------------------------------------------------------------------------------

#define CB_MULTITEXTURE

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

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
#ifdef _USE_MULTICHANNEL_
	float2 UV2		: TEXCOORD3;
#endif //_USE_MULTICHANNEL_
};

struct VS_InputSinus
{
	float4 Position   : POSITION;
	float4 Color	  : COLOR0;
	float2 UV		  : TEXCOORD0;
	float2 SinusParam : TEXCOORD3;
};

struct VS_Output
{
	float4 Position : VS_OUT_POS;
	float4 Color	: COLOR0;
#ifdef _USE_LAYER1_
	float2 UV		: TEXCOORD0;
#endif //_USE_LAYER1_
#ifdef _USE_LAYER2_
	float2 UV2		: TEXCOORD1;
#endif //_USE_LAYER2_
#ifdef _USE_LAYER3_
	float2 UV3		: TEXCOORD2;
#endif //_USE_LAYER3_
#ifdef _USE_LAYER4_
	float2 UV4		: TEXCOORD3;
#endif //_USE_LAYER4_

};

#ifdef VERTEX_PROFILE

VS_Output defaultMat_VS(VS_Input input)
{
	VS_Output output;

	output.Position = mul(input.Position, vs_mWorldViewProjection);
	output.Color = input.Color * vs_globalColor;

	float2 diffuseUV = input.UV;
#ifdef _USE_MULTICHANNEL_
	float2 sideChannelUV = input.UV2;
#else
	float2 sideChannelUV = input.UV;
#endif //_USE_MULTICHANNEL_

#ifdef _USE_LAYER1_	
	output.UV = mul(vs_multiTexTransformUV[0], float4(sideChannelUV, 1, 1));
#endif //_USE_LAYER1_
#ifdef _USE_LAYER2_	
	output.UV2 = mul(vs_multiTexTransformUV[1], float4(sideChannelUV, 1, 1));
#endif //_USE_LAYER2_
#ifdef _USE_LAYER3_	
	output.UV3 = mul(vs_multiTexTransformUV[2], float4(sideChannelUV, 1, 1));
#endif //_USE_LAYER3_
#ifdef _USE_LAYER4_	
	output.UV4 = mul(vs_multiTexTransformUV[3], float4(diffuseUV, 1, 1));
#endif //_USE_LAYER4_
	
	return output;
}

VS_Output defaultMatWithSinus_VS(VS_InputSinus input)
{
	VS_Output output;

	float4 pos = input.Position;
	float weight = input.SinusParam.x;
	float offsetTime = input.SinusParam.y;
	pos.xyz += vs_MultiTexSinusAmplitude.xyz * weight * sin((vs_MultiTexSinusParam.x * vs_MultiTexSinusParam.y) + offsetTime);
	
	output.Position = mul(pos, vs_mWorldViewProjection);
	output.Color = input.Color * vs_globalColor;

#ifdef _USE_LAYER1_
	output.UV = mul(vs_multiTexTransformUV[0], float4(input.UV, 1, 1));
#endif //_USE_LAYER1_
#ifdef _USE_LAYER2_	
	output.UV2 = mul(vs_multiTexTransformUV[1], float4(input.UV, 1, 1));
#endif //_USE_LAYER2_
#ifdef _USE_LAYER3_	
	output.UV3 = mul(vs_multiTexTransformUV[2], float4(input.UV, 1, 1));
#endif //_USE_LAYER3_
#ifdef _USE_LAYER4_	
	output.UV4 = mul(vs_multiTexTransformUV[3], float4(input.UV, 1, 1));
#endif //_USE_LAYER4_
	
	return output;
}

#endif //VERTEX_PROFILE

#ifdef PIXEL_PROFILE

float4 BlendLayers(float4 L1, float4 L2, float4 L3, float4 L4)
{
	float4 finalColor = float4(0.0, 0.0, 0.0, 0.0);
#ifdef _USE_LAYER1_
	finalColor = L1;
#endif //_USE_LAYER1_
#ifdef _USE_LAYER2_
	#ifdef _BLEND_LAYER2_ALPHA_
		finalColor = float4(L2.rgb * L2.a, L2.a) + finalColor*(1.0-L2.a);
	#endif //_BLEND_LAYER2_ALPHA_
	
	#ifdef _BLEND_LAYER2_ADD_
		finalColor = L2 + finalColor;
	#endif //_BLEND_LAYER2_ADD_
	
	#ifdef _BLEND_LAYER2_ADDALPHA_
		finalColor = float4(L2.rgb * L2.a, L2.a) + finalColor;  
	#endif //_BLEND_LAYER2_ADDALPHA_
	
	#ifdef _BLEND_LAYER2_MUL_
		finalColor = L2 * finalColor;
	#endif //_BLEND_LAYER2_MUL_
	
	#ifdef _BLEND_LAYER2_MULALPHA_
		finalColor = L2.a * finalColor;
	#endif //_BLEND_LAYER2_MULALPHA_
#endif //_USE_LAYER2_

#ifdef _USE_LAYER3_
	#ifdef _BLEND_LAYER3_ALPHA_
		finalColor = float4(L3.rgb * L3.a, L3.a) + finalColor*(1.0-L3.a);
	#endif //_BLEND_LAYER3_ALPHA_
	
	#ifdef _BLEND_LAYER3_ADD_
		finalColor = L3 + finalColor;
	#endif //_BLEND_LAYER3_ADD_
	
	#ifdef _BLEND_LAYER3_ADDALPHA_
		finalColor = float4(L3.rgb * L3.a, L3.a) + finalColor;  
	#endif //_BLEND_LAYER3_ADDALPHA_
	
	#ifdef _BLEND_LAYER3_MUL_
		finalColor = L3 * finalColor;
	#endif //_BLEND_LAYER3_MUL_
	
	#ifdef _BLEND_LAYER3_MULALPHA_
		finalColor = L3.a * finalColor;
	#endif //_BLEND_LAYER3_MULALPHA_
#endif //_USE_LAYER3_

#ifdef _USE_LAYER4_
	#ifdef _BLEND_LAYER4_ALPHA_
		finalColor = float4(L4.rgb * L4.a, L4.a) + finalColor*(1.0-L4.a);
	#endif //_BLEND_LAYER4_ALPHA_
	
	#ifdef _BLEND_LAYER4_ADD_
		finalColor = L4 + finalColor;
	#endif //_BLEND_LAYER4_ADD_
	
	#ifdef _BLEND_LAYER4_ADDALPHA_
		finalColor = float4(L4.rgb * L4.a, L4.a) + finalColor;  
	#endif //_BLEND_LAYER4_ADDALPHA_
	
	#ifdef _BLEND_LAYER4_MUL_
		finalColor = L4 * finalColor;
	#endif //_BLEND_LAYER4_MUL_
	
	#ifdef _BLEND_LAYER4_MULALPHA_
		finalColor = L4.a * finalColor;
	#endif //_BLEND_LAYER4_MULALPHA_
#endif //_USE_LAYER4_

	return finalColor;
}

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

float4 RemapAlphaChannel(float4 inColor, float2 componentSaturator, float4 alphaSel)
{	
	float4 outColor;
	outColor = saturate(inColor + float4(componentSaturator.x, componentSaturator.x, componentSaturator.x, componentSaturator.x));
	outColor.a = dot(inColor, alphaSel) + componentSaturator.y;
	return outColor;
}

float4 AlphaThreshold(float4 inColor, float threshold)
{
    float4 outColor = inColor;
#ifdef _ALPHA_THRESHOLD_    
    if (threshold >= 0.f)
    {
        if (inColor.a > threshold)
        {
            outColor.a = 1.f;
        }
        else
        {
            outColor.a = 0.f;
        }
    }
#endif // _ALPHA_THRESHOLD_   
    return outColor;
}

struct PS_Output
{
	float4 Color : PS_OUT_COLOR;
};

PS_Output defaultMat_PS(VS_Output input)
{
	PS_Output output;
	
	float4 Color1 = ps_multiTexColors[0];
	float4 Color2 = ps_multiTexColors[1];
	float4 Color3 = ps_multiTexColors[2];
	float4 Color4 = ps_multiTexColors[3];
	
	output.Color = float4( 0.0, 0.0, 0.0, 0.0 );

#ifdef _USE_LAYER1_	
	Color1 *= AlphaThreshold(RemapAlphaChannel(SampleTex2D(SAMPLER_PARAM(TextureSampler, 0), input.UV), float2(ps_multiTexTextureUsageColorSaturator[0], ps_multiTexTextureUsageAlphaSaturator[0]), ps_multiTexTextureUsageAlphaSelector[0]), ps_multiTexTextureAlphaThreshold[0]);
#endif //_USE_LAYER1_
#ifdef _USE_LAYER2_
	Color2 *= AlphaThreshold(RemapAlphaChannel(SampleTex2D(SAMPLER_PARAM(TextureSampler, 1), input.UV2), float2(ps_multiTexTextureUsageColorSaturator[1], ps_multiTexTextureUsageAlphaSaturator[1]), ps_multiTexTextureUsageAlphaSelector[1]), ps_multiTexTextureAlphaThreshold[1]);
#endif //_USE_LAYER2_
#ifdef _USE_LAYER3_
	Color3 *= AlphaThreshold(RemapAlphaChannel(SampleTex2D(SAMPLER_PARAM(TextureSampler, 2), input.UV3), float2(ps_multiTexTextureUsageColorSaturator[2], ps_multiTexTextureUsageAlphaSaturator[2]), ps_multiTexTextureUsageAlphaSelector[2]), ps_multiTexTextureAlphaThreshold[2]);
#endif //_USE_LAYER3_
#ifdef _USE_LAYER4_
	Color4 *= AlphaThreshold(RemapAlphaChannel(SampleTex2D(SAMPLER_PARAM(TextureSampler, 3), input.UV4), float2(ps_multiTexTextureUsageColorSaturator[3], ps_multiTexTextureUsageAlphaSaturator[3]), ps_multiTexTextureUsageAlphaSelector[3]), ps_multiTexTextureAlphaThreshold[3]);
#endif //_USE_LAYER4_

	output.Color = input.Color * BlendLayers(Color1, Color2, Color3, Color4);

	if(output.Color.a < 0.003921577) // 1/255
	{
		discard;
	}
	
	return output;
}

#endif //PIXEL_PROFILE
