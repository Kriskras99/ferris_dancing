/*

This effect file is designed for rending bitmap fonts output by
AngelCode Bitmap Font Generator. It is capable of rendering
from 32bit textures that pack colored icons together with outlined
characters into one texture, where the icons use all 32bits and the
characters only use 8bits each.

*/

#define CB_FONT

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

#ifdef VERTEX_PROFILE

float4 zinject(in float4 vpos)
{
#ifdef ZINJECT
	vpos.z = vs_zInject.x * vpos.w;
#endif
    return vpos;
}

struct VS_VERTSCENE_INPUT
{
	float4 Position         : POSITION;
	float4 vColor       : COLOR0;
	int4   vChnl        : BLENDINDICES;
	float2 vTex0        : TEXCOORD0;
	float2 vTex1        : TEXCOORD3;
	float2 vTex2		: TEXCOORD1;
};

struct VS_VERTSCENE_OUTPUT
{
	 float4 oDiffuse		: COLOR0_C;
	 float4 oChnl			: TEXCOORD0;
	 float4 oTex0_Gradient	: TEXCOORD3;
	 float4 Position		: VS_OUT_POS;
};

VS_VERTSCENE_OUTPUT VertScene( VS_VERTSCENE_INPUT _in )
{
	VS_VERTSCENE_OUTPUT output;
    output.Position = zinject( mul(_in.Position, vs_mWorldViewProjection) );

    // Just copy the texture coordinate and color 
    output.oDiffuse = _in.vColor * vs_globalColor;
    output.oChnl = _in.vChnl;
    output.oTex0_Gradient.xy = _in.vTex0;
	
#ifdef PROGRESSIVE
	float2 scale = float2(_in.vTex2.x, 1.0);
	output.oTex0_Gradient.zw = scale * _in.vTex1;
#else
	output.oTex0_Gradient.zw = _in.vTex1;
#endif //PROGRESSIVE

	
	//output.vChnl = _in.vChnl;
	return output;
}

#endif // VERTEX_PROFILE

#ifdef PIXEL_PROFILE

struct PS_PIXSCENE_INPUT
{
	float4 color : COLOR0;
	float4 chnl : TEXCOORD0;
	float4 tex0 : TEXCOORD3;
};

REGISTER_SAMPLER(TextureSampler, 0) //g_diffuseTexture
REGISTER_SAMPLER(TextureSampler, 1) 

float4 PixWithoutOutline( PS_PIXSCENE_INPUT _in ) : PS_OUT_COLOR
{
    float4 pixel = TEXTURE_READ_2D(TextureSampler, 0, _in.tex0.xy);
    
    // Are we rendering a colored image, or 
    // a character from only one of the channels
    if( dot(float4(1,1,1,1), _in.chnl) )
    {
        // Get the pixel value
		float val = dot(pixel, _in.chnl);
		
        pixel.rgb = 1;
        pixel.a   = val;
    }
    
	return pixel * _in.color;
}

float4 PixWithOutline( PS_PIXSCENE_INPUT _in ) : PS_OUT_COLOR
{
    float4 pixel = TEXTURE_READ_2D(TextureSampler, 0, _in.tex0.xy);
    
    // Are we rendering a colored image, or 
    // a character from only one of the channels

    if( dot(float4(1,1,1,1), _in.chnl) )
    {
        // Get the pixel value
		float val = dot(pixel, _in.chnl);

		
        // A value above .5 is part of the actual character glyph
        // A value below .5 is part of the glyph outline
		pixel.rgb = val > 0.5 ? 2*val-1 : 0;
		pixel.a   = val > 0.5 ? 1 : 2*val;
    }
	
	//pixel.rgba =  float4(1.0,0.0,0.0,1.0);
	
    return pixel * _in.color;
}

float4 PixFont( PS_PIXSCENE_INPUT _in ) : PS_OUT_COLOR
{
    float4 pixel = TEXTURE_READ_2D(TextureSampler, 0, _in.tex0.xy);
    
    // Are we rendering a colored image, or 
    // a character from only one of the channels

    if( dot(float4(1,1,1,1), _in.chnl) )
    {
        // Get the pixel value
		float val = dot(pixel, _in.chnl);

	#ifdef OUTLINE
        // A value above .5 is part of the actual character glyph
        // A value below .5 is part of the glyph outline
		pixel.rgb = val > 0.5 ? 2*val-1 : 0;
		pixel.a   = val > 0.5 ? 1 : 2*val;
	#else
	
        pixel.rgb = 1;
        pixel.a   = val;
	#endif
    }
		
	float4 newColor = _in.color;
#ifdef GRADIENT
	float colorGradient = saturate((1.0f - abs(_in.tex0.w * 2.0f - 1.0f)) * ps_FontParam.x);
	newColor = lerp(ps_FontColorGradient, newColor, colorGradient);
#endif

#ifdef PROGRESSIVE
	newColor = ps_FontProgressiveColor;	
	float3 mask = TEXTURE_READ_2D(TextureSampler, 1, _in.tex0.zw).rgb;
	newColor.rgb *= mask;	
	pixel.rgb -= mask - newColor.rgb;
	pixel.rgba *= _in.color.rgba;
#else
	pixel = pixel * newColor;
#endif //PROGRESSIVE

#ifdef DX11_SHADERS
	   if (ps_alphaTest.r > 0.0f)
	   {
		  if (pixel.a < ps_alphaTest.g)
		  {
			  discard;
		  }
	   }
#endif

    return pixel;
}

#endif // PIXEL_PROFILE

