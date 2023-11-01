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

struct VS_PCT1_OUTPUT
{
    float4 Position   : VS_OUT_POS;   // vertex position 
    float4 Diffuse    : COLOR0_C;     // vertex diffuse color (note that COLOR0 is clamped from 0..1)
    float2 TextureUV0  : TEXCOORD0;  // vertex texture coords 
};

struct VS_PCT4_OUTPUT
{
    float4 Position   : VS_OUT_POS;   // vertex position 
    float4 Diffuse    : COLOR0_C;     // vertex diffuse color (note that COLOR0 is clamped from 0..1)
    float2 TextureUV0  : TEXCOORD0;  // vertex texture coords 
    float2 TextureUV1  : TEXCOORD1;  // vertex texture coords 
    float2 TextureUV2  : TEXCOORD2;  // vertex texture coords 
    float2 TextureUV3  : TEXCOORD3;  // vertex texture coords 
};

struct VS_PCT5_OUTPUT
{
    float4 Position   : VS_OUT_POS;   // vertex position 
    float4 Diffuse    : COLOR0_C;     // vertex diffuse color (note that COLOR0 is clamped from 0..1)
    float4 TextureUV0_1: TEXCOORD0;  // vertex texture coords 
    float4 TextureUV2_3: TEXCOORD1;  // vertex texture coords 
    float4 TextureUV  : TEXCOORD2;  // vertex texture coords 
};

struct VS_PCT2_OUTPUT
{
    float4 Position   : VS_OUT_POS;   // vertex position 
    float4 Diffuse    : COLOR0_C;     // vertex diffuse color (note that COLOR0 is clamped from 0..1)
    float2 TextureUV0  : TEXCOORD0;  // vertex texture coords 
    float2 TextureUV1  : TEXCOORD1;  // vertex texture coords 
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
REGISTER_SAMPLER(TextureSampler, 1) //scene
REGISTER_SAMPLER(TextureSampler, 2) //normal
REGISTER_SAMPLER(TextureSampler, 3) //custom


PS_OUTPUT BigBlur_PS( VS_PCT5_OUTPUT In ) 
{ 
	PS_OUTPUT Output;
	
	float4 color = 0;
	
  #ifdef GAUSS
	float2 dlt = In.TextureUV2_3.xy - In.TextureUV.xy;

	color  = TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0_1.xy) * 1.0f;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0_1.zw) * 1.0f;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV2_3.xy) * 1.0f;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV2_3.zw) * 1.0f;

	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV.xy + float2(dlt.x, 0.0f) ) * 2.0f;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV.zw + float2(-dlt.x, 0.0f)) * 2.0f;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV.xy + float2(0.0f, dlt.y)) * 2.0f;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV.zw + float2( 0.0f, -dlt.y)) * 2.0f;

	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV.xy) * 4.0f;
	color /= 16.0f;
  #else
	color  = TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0_1.xy) * 0.5;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0_1.zw) * 0.5;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV2_3.xy) * 0.5;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV2_3.zw) * 0.5;
	color += TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV.xy) * 1.0f;
	color /= 3.0f;
  #endif

    Output.RGBColor = color * In.Diffuse;
    return Output;
}

//--------------------------------------------------------------------------------------
// ColorLevels.
// ps_addMulFactor.x = glowfactor
// ps_addMulFactor.y = addalpha
//--------------------------------------------------------------------------------------
PS_OUTPUT AddMul_PS( VS_PCT1_OUTPUT In ) 
{ 
    PS_OUTPUT Output;

    float4 colorSamp = TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0) * ps_addMulFactor.x;
    float4 colorScene = TEXTURE_READ_2D(TextureSampler, 1, In.TextureUV0);
	
    Output.RGBColor = ( colorScene + colorSamp * (colorSamp.a * ps_addMulFactor.y) );
    return Output;
}

// transform black to white overdraw image to 
PS_OUTPUT ColorOverDraw_PS( VS_PCT1_OUTPUT In)
{ 
    PS_OUTPUT Output;
    
    float4 fcolortex = TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0);
 
    #if 0
	Output.RGBColor.r = tex1D(TextureSampler3, fcolortex.r).r;
	Output.RGBColor.g = tex1D(TextureSampler3, fcolortex.g).g;
	Output.RGBColor.b = tex1D(TextureSampler3, fcolortex.b).b;
	#else
        // Pure shader version -> texture sampled version has an unknown edge issue
	    float marker1 = 50./256.;
		float marker2 = 100./256.;
		float marker3 = 150./256.;
	
		float3 red = float3(1., 0., 0.);
		float3 blue = float3(0., 0., 1.);
		float3 yellow = float3(1., 1., 0.);
		float3 green = float3(0., 1., 0.);
	
        float refColor = fcolortex.r;
    
		if (refColor == 0)
		{
            Output.RGBColor.rgb = float3(0.f, 0.f, 0.f);
		}
		else if (refColor<marker1)  // blue/green.
		{
			float dist = marker1 - 1./256.;
            float vlerp = (marker1 - refColor)/dist;
			Output.RGBColor.rgb = lerp(green, blue, vlerp);
		}
		else if (refColor<marker2) // green/Yellow.
        {
            float dist = marker2 - marker1;
            float vlerp = (marker2 - refColor)/dist;
			Output.RGBColor.rgb = lerp(yellow, green, vlerp);
        }
		else if (refColor<marker3) // Yellow/Red.
        {
            float dist = marker3 - marker2;
            float vlerp = (marker3 - refColor)/dist;
			Output.RGBColor.rgb = lerp(red, yellow, vlerp);
        }
		else
        {
			Output.RGBColor.rgb = red;
        }
	#endif
	
    Output.RGBColor.a = 1.f;
    return Output;
}

//	-----------------------------------------------------------------------------------------
//		MERGED POST PROCESS
//	-----------------------------------------------------------------------------------------

#if defined(SATURATION) || defined(BRIGHT) || defined(CONTRAST) || defined(COLOR_CORRECTION)
	#define COLOR_SETTING 1
#endif

float2 blendResult(in float2 _val1, in float2 _val2, float _lerpVal)
{
	#ifdef BLEND_VALUE
		return lerp(_val1, _val2, _lerpVal);
	#else
		return _val2;
	#endif
}

float3 blendResult(in float3 _val1, in float3 _val2, float _lerpVal)
{
	#ifdef BLEND_VALUE
		return lerp(_val1, _val2, _lerpVal);
	#else
		return _val2;
	#endif
}

//Visual Displacement Screen Effect
void computeUVDisplacement(inout float2 _UV, out float _colorMask)
{
	float2 UVDisplaced = _UV;
	_colorMask = float3(1.0f, 1.0f, 1.0f);

	#ifdef MIRROR
		float2 offset = ps_mirror.xy;
		float2 sign = ps_mirror.zw;
		UVDisplaced = sign * (abs( sign * UVDisplaced - offset) + offset) ;
	#endif

	#ifdef EYEFISH
		float2 uv = (UVDisplaced * 2.0f - 1.0f) * ps_eyeFish.z;
		float z =  sqrt(1.0f - uv.x * uv.x - uv.y * uv.y);
		float a = 1.0f / (z * tan(ps_eyeFish.x * 0.5f));
		uv = (uv * a ) * 0.5f + 0.5f;
		UVDisplaced = blendResult(_UV, uv, ps_eyeFish.y);
		_colorMask = z;
	#endif

	#ifdef MOSAIC
		UVDisplaced = floor((UVDisplaced + ps_mosaic.z) * ps_mosaic.y) * ps_mosaic.x;
	#endif

	#ifdef TILE
		//UVDisplaced = ((UVDisplaced - float2(0.5f, 0.5f)) * ps_tile.x) + float2(0.5f, 0.5f);
		UVDisplaced = fmod(UVDisplaced, ps_tile.x);
	#endif

	_UV = UVDisplaced;
}

//Color effect
float3 colorEffect(VS_PCT1_OUTPUT In, in float3 _colorIn)
{
#ifdef NEGATIF
	_colorIn.rgb = blendResult(_colorIn.rgb, float3(1.0f, 1.0f, 1.0f) - saturate(_colorIn.rgb), ps_AFXParam2.y);
#endif

//Compute colorSetting
#ifdef COLOR_SETTING
	float3 f3_NewColor = _colorIn.rgb;

	#ifdef SATURATION
	    f3_NewColor = lerp( dot(f3_NewColor, float3(0.3f, 0.59f, 0.11f)).xxx, f3_NewColor, ps_colorSetting.xxx );//Saturation
	#endif
	#ifdef BRIGHT
	  f3_NewColor = (f3_NewColor + ps_colorSetting.w);								  // Brightness
	#endif
	#ifdef CONTRAST
	  f3_NewColor = ( f3_NewColor * ps_colorSetting.z + ps_colorSetting.y );		  // Contraste
	#endif
	#ifdef COLOR_CORRECTION
	  f3_NewColor = ( f3_NewColor * ps_colorCorrection.rgb ) * ps_colorCorrection.a;  // Color Correction
	#endif

	f3_NewColor = saturate(f3_NewColor);
	_colorIn.rgb = blendResult(_colorIn.rgb, f3_NewColor, ps_AFXParam.w);
#endif

#ifdef NOISE
	float2 p = In.TextureUV0.xy * 100.0f + ps_noise.xx;
    const float2 r = float2(
        23.1406926327792690,  // e^pi (Gelfond's constant)
        2.6651441426902251); // 2^sqrt(2) (Gelfond–Schneider constant)
		float random = frac( cos( fmod( 123456789., 1e-7 + 256. * dot(p,r) ) ) );
    _colorIn.rgb = blendResult(_colorIn.rgb, lerp(_colorIn.rgb, random.xxx, ps_noise.y), ps_AFXParam2.w);
#endif
	return _colorIn;
}

PS_OUTPUT mergedEffect_PS( VS_PCT1_OUTPUT In )
{
    PS_OUTPUT Output;
	
	float4 f4_color = In.Diffuse;

#if defined(COLOR_SETTING) || defined(REFRACTION) || defined(NEGATIF) || defined(OLD_TV) || defined(NOISE)
    float4 f4_sceneColor = TEXTURE_READ_2D(TextureSampler, 1, In.TextureUV0);
	f4_color = f4_sceneColor;
	float4 newSceneColor = f4_sceneColor;
#endif

	float2 newUV = In.TextureUV0;

//Compute refracted Scene
#ifdef REFRACTION
	float2 decal = (TEXTURE_READ_2D(TextureSampler, 2,In.TextureUV0).xy * 2.0f - 1.0f);
	newUV = In.TextureUV0 + ( decal.xy * f4_sceneColor.a * ps_AFXParam2.x);
	newSceneColor =  TEXTURE_READ_2D(TextureSampler, 1, newUV);
	if(newSceneColor.a == 1.0f)
	{
		f4_color.rgb = newSceneColor.rgb;
	}
	else
	{
		f4_color.rgb = f4_sceneColor.rgb;
	}	
#endif

//Get Blur and compute glow if needed
#ifdef BLUR
	f4_color.rgb = TEXTURE_READ_2D(TextureSampler, 0, newUV).rgb;
	f4_color.a = ps_AFXParam.z;
	#ifdef GLOW
	  #ifdef TONEMAP
		float3 f3_tonemap = saturate(f4_color.rgb + ps_glowParam.x);
		float3 f3_tonemap2 = f3_tonemap * f3_tonemap;
		float3 f3_tonemap4 = f3_tonemap2 * f3_tonemap2;
		f4_color.rgb = lerp(f4_color.rgb, f3_tonemap4, ps_glowParam.y);
	  #endif
		f4_color.rgb *= ps_AFXParam.x;
		f4_color.a = ps_AFXParam.y;
	#endif
#endif

//Blend effect
#if defined(COLOR_SETTING) || defined(REFRACTION)
  #ifdef BLUR
	#ifdef GLOW
	  f4_color.rgb = (f4_color.rgb * ps_AFXParam.y) + newSceneColor.rgb;
	#else
      f4_color.rgb = blendResult(newSceneColor.rgb, f4_color.rgb, ps_AFXParam.z);
	#endif
  #endif
#endif

//Prepare effect
#ifdef OLD_TV
	float scanLine = saturate(1.0f - abs( (1.0f - (In.TextureUV0.y + ps_oldTV.w)) / ps_oldTV.z));
	scanLine *= ps_oldTV.y;
#endif


	//Visual Displacement Screen Effect
	float2 UVDisplaced = In.TextureUV0;
	float colorMask = 1.0f;

	computeUVDisplacement(UVDisplaced, colorMask);

#ifdef OLD_TV
	UVDisplaced += scanLine.xx * 0.05;
#endif

#if defined(TILE) || defined(MOSAIC) || defined(MIRROR) || defined(EYEFISH) || defined(OLD_TV)
  #ifdef BLUR
    f4_color.rgb = TEXTURE_READ_2D(TextureSampler, 0, UVDisplaced).rgb;
  #else
    f4_color.rgb = TEXTURE_READ_2D(TextureSampler, 1, UVDisplaced).rgb;
  #endif
	f4_color.rgb *= colorMask;
#endif

	//Color effect
	Output.RGBColor.rgb = colorEffect(In, f4_color.rgb);
	Output.RGBColor.a = f4_color.a;

#ifdef OLD_TV
	Output.RGBColor.rgb += TEXTURE_READ_2D(TextureSampler, 3, In.TextureUV0).rgb * ps_oldTV.x;
	Output.RGBColor.rgb += scanLine.xxx;
#endif


    return Output;
}

PS_OUTPUT KaleiPass_PS( VS_PCT1_OUTPUT In )
{
    PS_OUTPUT Output;
	
	float4 f4_color = In.Diffuse;
	f4_color.rgb = TEXTURE_READ_2D(TextureSampler, 1, In.TextureUV0).rgb;

	Output.RGBColor.rgb = f4_color.rgb;
	Output.RGBColor.a = f4_color.a;
    return Output;
}

PS_OUTPUT DebugPass_PS( VS_PCT1_OUTPUT In )
{
    PS_OUTPUT Output;
	
	float4 f4_color = In.Diffuse;
	float3 initialColor = TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0).rgb;
  #ifdef SATURATION
    float contrastScale = 2.0f;
	float contrastAdd = -0.5f;
	f4_color.rgb = dot(initialColor, float3(0.3f, 0.59f, 0.11f)).xxx;
    f4_color.rgb = ( f4_color.rgb * contrastScale + contrastAdd );		  // Contraste
  #endif
  #ifdef EDGEDETECTION
	
	float kernel = dot(initialColor, float3(0.3f, 0.59f, 0.11f)) * 8.0f;
	float2 deltaUV = ps_viewportDimensions.zw * 2.0f;
	kernel -=   dot(TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0 + (deltaUV * float2(-1,-1))).rgb, float3(0.3f, 0.59f, 0.11f));
	kernel -=   dot(TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0 + (deltaUV * float2(0,-1))).rgb, float3(0.3f, 0.59f, 0.11f));
	kernel -=   dot(TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0 + (deltaUV * float2(1,-1))).rgb, float3(0.3f, 0.59f, 0.11f));
	kernel -=   dot(TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0 + (deltaUV * float2(-1,0))).rgb, float3(0.3f, 0.59f, 0.11f));
	kernel -=   dot(TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0 + (deltaUV * float2(1,0))).rgb, float3(0.3f, 0.59f, 0.11f));
	kernel -=   dot(TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0 + (deltaUV * float2(-1,1))).rgb, float3(0.3f, 0.59f, 0.11f));
	kernel -=   dot(TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0 + (deltaUV * float2(0,1))).rgb, float3(0.3f, 0.59f, 0.11f));
	kernel -=   dot(TEXTURE_READ_2D(TextureSampler, 0, In.TextureUV0 + (deltaUV * float2(1,1))).rgb, float3(0.3f, 0.59f, 0.11f));
	float edge = pow(saturate(kernel), 2.0f); //dot(kernel, float3(0.33f, 0.33f, 0.33f));
  #ifdef SATURATION
	f4_color.rgb += edge.xxx;
  #else
	f4_color.rgb = edge.xxx;
  #endif

  #endif
	Output.RGBColor = f4_color;
    return Output;
}

#endif // PIXEL_PROFILE
