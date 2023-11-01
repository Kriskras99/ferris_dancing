#define CB_MOVIE

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

struct VS_OUT
{
	float4 Pos   : VS_OUT_POS;
	float4 Color : COLOR0;
	float2 UV    : TEXCOORD0;
	float2 UV2   : TEXCOORD3;
};

#ifdef VERTEX_PROFILE

VS_OUT VS_Pleo( float4 vPos      : POSITION,
				float4 vColor    : COLOR0,
				float2 vTexture  : TEXCOORD0,
				float2 vTexture2 : TEXCOORD3)
{
	VS_OUT Out;
	Out.Pos = mul(vPos, vs_mWorldViewProjection);
	Out.UV = vTexture;
	Out.UV2 = vTexture2;
	Out.Color = vColor;

	return Out;
}

#endif

#ifdef PIXEL_PROFILE

REGISTER_SAMPLER(TextureSampler, 0) //g_diffuseTexture
REGISTER_SAMPLER(TextureSampler, 1) //g_normalTexture
REGISTER_SAMPLER(TextureSampler, 2) //g_sceneTexture
REGISTER_SAMPLER(TextureSampler, 3) 

// for YCrCb to RGB conversion see these links
// http://www.fourcc.org/fccyvrgb.php
// http://en.wikipedia.org/wiki/YCbCr
// http://en.wikipedia.org/wiki/YUV#BT.709_and_BT.601
// the color conversion used is BT.601 check that video encode uses the same conversion matrix
// 255/219 = 1.164
// 16/256 = 0.0625

static const float YToAlphaOffset = 16.0/255.0;
static const float YToAlphaScale = 255.0/(235.0-16.0);

struct PS_IN
{
	float4 Pos   : VS_OUT_POS;
	float4 Color : COLOR0;
	float2 UV    : TEXCOORD0;
	float2 UV2   : TEXCOORD3;
};

float4 PS_Pleo(PS_IN In   ) : PS_OUT_COLOR
{
	float  Y = TEXTURE_READ_2D( TextureSampler, 0, In.UV ).r;
	float cR = TEXTURE_READ_2D( TextureSampler, 1, In.UV ).r;
	float cB = TEXTURE_READ_2D( TextureSampler, 2, In.UV ).r;

	float R = 1.164 * ( Y - 0.0625 ) + 1.596 * ( cR - 0.5 );
	float G = 1.164 * ( Y - 0.0625 ) - 0.391 * ( cB - 0.5 ) - 0.813 * ( cR - 0.5 );
	float B = 1.164 * ( Y - 0.0625 ) + 2.018 * ( cB - 0.5 );

	float4 ARGB;
	ARGB.a = ps_movieAlpha.x;
	ARGB.r = R;
	ARGB.g = G;
	ARGB.b = B;

	if(ps_movieAlpha.w == 1.f)
	{
		float A = TEXTURE_READ_2D( TextureSampler, 3, In.UV2 ).r;
		ARGB.a = clamp((A-YToAlphaOffset)*YToAlphaScale, 0.0, 1.0); // Map the [16, 235] value in Y to [0, 255].
		//ARGB.a = 1.0;
	}

#ifdef DX11_SHADERS
	if (ps_alphaTest.r > 0.0f)
	{
		if (ARGB.a < ps_alphaTest.g)
		{
			discard;
		}
	}
#endif

	return ARGB;
}

#endif // PIXEL_PROFILE
