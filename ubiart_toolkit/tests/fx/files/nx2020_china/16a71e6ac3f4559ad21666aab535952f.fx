
#define CB_OUTLINEDMASK

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

struct VS_VERTSCENE_OUTPUT
{
	float4 Position     : VS_OUT_POS;
	float4 Color        : COLOR0;
	float2 UV           : TEXCOORD0;
};

#ifdef VERTEX_PROFILE

struct VS_VERTSCENE_INPUT
{
	float4 Position    : POSITION;
	float4 Color       : COLOR0;
	float2 Tex0        : TEXCOORD0;
};
VS_VERTSCENE_OUTPUT outlinedMask_vertex( VS_VERTSCENE_INPUT _In )
{
	VS_VERTSCENE_OUTPUT output;
    output.Position =  mul(_In.Position, vs_mWorldViewProjection);

	output.UV = _In.Tex0;
	output.Color = _In.Color * vs_globalColor;

	return output;
}

#endif // VERTEX_PROFILE

#ifdef PIXEL_PROFILE

REGISTER_SAMPLER(TextureSampler, 0) //g_diffuseTexture

float2 pickPoint(in float2 _center, in int _ID)
{
	// constant picking kernel (it's a circle)
#ifdef DOUBLESAMPLE
	const float2 kernel[32] = // this describes a circle, for optimal results
	{
		float2(1.0, 0.0),
		float2(0.98078528, 0.195090322),
		float2(0.923879533, 0.382683432),
		float2(0.831469612, 0.555570233),
		float2(0.707106781, 0.707106781),
		float2(0.555570233, 0.831469612),
		float2(0.382683432, 0.923879533),
		float2(0.195090322, 0.98078528),
		float2(0.0, 1.0),
		float2(-0.195090322, 0.98078528),
		float2(-0.382683432, 0.923879533),
		float2(-0.555570233, 0.831469612),
		float2(-0.707106781, 0.707106781),
		float2(-0.831469612, 0.555570233),
		float2(-0.923879533, 0.382683432),
		float2(-0.98078528, 0.195090322),
		float2(-1.0, 0.0),
		float2(-0.98078528, -0.195090322),
		float2(-0.923879533, -0.382683432),
		float2(-0.831469612, -0.555570233),
		float2(-0.707106781, -0.707106781),
		float2(-0.555570233, -0.831469612),
		float2(-0.382683432, -0.923879533),
		float2(-0.195090322, -0.98078528),
		float2(0.0, -1.0),
		float2(0.195090322, -0.98078528),
		float2(0.382683432, -0.923879533),
		float2(0.555570233, -0.831469612),
		float2(0.707106781, -0.707106781),
		float2(0.831469612, -0.555570233),
		float2(0.923879533, -0.382683432),
		float2(0.98078528, -0.195090322)
	};
#else
	const float2 kernel[16] = // this describes a circle, for optimal results
	{
		float2(1.0, 0.0),
		float2(0.923879, 0.382683),
		float2(0.707106, 0.707106),
		float2(0.382683, 0.923879),
		float2(0.0, 1.0),
		float2(-0.382682, 0.923879),
		float2(-0.707106, 0.707106),
		float2(-0.923879, 0.382683),
		float2(-1.0, 0.0),
		float2(-0.923879, -0.382683),
		float2(-0.707106, -0.707106),
		float2(-0.382683, -0.923879),
		float2(0.0, -1.0),
		float2(0.382683, -0.923879),
		float2(0.707106, -0.707106),
		float2(0.923879, -0.382683)
	};
#endif

    return (_center + float2(ps_om_params.x * kernel[_ID].x, ps_om_params.y * kernel[_ID].y));
}

float4 outlinedMask_pixel (VS_VERTSCENE_OUTPUT _In) : PS_OUT_COLOR
{
#ifdef DOUBLESAMPLE
	const int nbSamples = 32;
#else
	const int nbSamples = 16;
#endif

	// sample at given UV (alpha only)
	float vign = TEXTURE_READ_2D(TextureSampler, 0, _In.UV).a;
	
	// if inside image, just otuput mask
	float4 pixel;
	if (vign >= 0.8f)
	{
		// avoid aliasing by interpolating beween mask and outline color
		float t = saturate((1.0 - vign) * 5.0f); // [1.0 - 0.8] -> [0.0 - 1.0]
		pixel = ps_om_maskColor * (1 - t) + ps_om_outlineColor * t;
	}
	// if outside, check if we are near the image, and outline if necessary
	else
	{
		float around = 0.0f;
		for (int i = 0; i < nbSamples; ++i)
		{
			// take only values that are in [0.8 - 1.0] i.e. considered as being part of the mask
			around = max(TEXTURE_READ_2D(TextureSampler, 0, pickPoint(_In.UV, i)).a, around);
			around *= step(0.8, around);
		}
		// also avoid aliasing by interpolating, again (but only the alpha here)
		around = (around * 5.0) - 4.0; // [0.8 - 1.0] -> [0.0 - 1.0]
		pixel = float4(ps_om_outlineColor.xyz, ps_om_outlineColor.a * around);
	}

	return pixel * _In.Color;
}

#endif // PIXEL_PROFILE
