#ifndef AUTODANCE__FX
#define AUTODANCE__FX

#define CB_AUTODANCE

#ifdef ITF_X360 
#define ALT_TOONSHADER
#endif

#ifdef DX11_SHADERS 
#define ALT_TOONSHADER
#endif

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

REGISTER_SAMPLER(samp, 0);
REGISTER_SAMPLER(samp, 1);
REGISTER_SAMPLER(samp, 2);
REGISTER_SAMPLER(samp, 3);

#define PI		3.14159265359f
#define EPSILON 0.0001f

#define DEFINE_CONSTANTS \
    const float3 rgb_to_y = float3( 0.212671f, 0.715160f, 0.072169f ); \
	const float2 vec2_zero = float2(0.0f,0.0f); \
	const float2 vec2_one  = float2(1.0f,1.0f); \
	const float3 vec3_zero = float3(0.0f,0.0f,0.0f); \
	const float3 vec3_one  = float3(1.0f,1.0f,1.0f); \
	const float4 vec4_zero = float4(0.0f,0.0f,0.0f,0.0f); \
	const float4 vec4_one  = float4(1.0f,1.0f,1.0f,1.0f);

struct VS_IN
{
	float4 Position     : POSITION;
    float2 uv0          : TEXCOORD0;
};

struct VS_OUT
{
	float4 Position     : VS_OUT_POS;
    float2 uv0          : TEXCOORD0;
};

struct VS_OUT_T2
{
	float4 Position     : VS_OUT_POS;
    float4 uv0          : TEXCOORD0;
};

struct VS_PLANE_OUT
{
	float4 Position     : VS_OUT_POS;
    float2 uv0          : TEXCOORD0;
	float4 pospostvs    : TEXCOORD1;
	
};

struct PS_OUT
{
	float4 clr : PS_OUT_COLOR;
};

/////////////////////////////
// Perspective correction
/////////////////////////////

struct VS_PC_IN
{
	float4 Position : POSITION;
	float4 col : COLOR0;
	float2 uv0 : TEXCOORD0;
	float4 uv1 : TEXCOORD1;
	float4 uv2 : TEXCOORD2;
	float2 uv3 : TEXCOORD3;
};

struct VS_PC_OUT
{
	float4 Position : VS_OUT_POS;
	float3 uv  : TEXCOORD0;
	float4 uv2 : TEXCOORD1;
};


struct VS_Particle_OUT
{
	float4 Position : VS_OUT_POS;
	float4 uv  : TEXCOORD0;
	float4 uv2 : TEXCOORD1;
};

float3 XYZ2RGB(float3 input)
{
    const float3 rFactors   = float3(3.2404542f, -1.5371385f, -0.4985314f);
    const float3 gFactors   = float3(-0.9692660f, 1.8760108f,  0.0415560f);
    const float3 bFactors   = float3(0.0556434f, -0.2040259f,  1.0572252f);

    float r = dot(input,rFactors);
    float g = dot(input,gFactors);
    float b = dot(input,bFactors);

    return float3(r,g,b);
}

float3 XYY2XYZ(float3 input)
{
    float x = 0.0f;
    float y = 0.0f;
    float z = 0.0f;

    if (input.y > 0.001f)
    {
        x = (input.x * input.z) / input.y;
        y = input.z;
        z = ((1.0f - input.x - input.y) * input.z) / input.y;
    }

    return float3(x,y,z);
}

float3 RGB2XYZ(float3 input)
{
    const float3 xFactors   = float3(0.4124f, 0.3576f,  0.1805f);
    const float3 yFactors   = float3(0.2126f, 0.7152f,  0.0722f);
    const float3 zFactors   = float3(0.0193f, 0.1192f,  0.9505f);

    float x = dot(input,xFactors);
    float y = dot(input,yFactors);
    float z = dot(input,zFactors);

    return float3(x,y,z);
}
    
float3 XYZ2XYY(float3 input)
{
    float sum = max(1e-6f, input.x + input.y + input.z);
 
    float rx = input.x / sum;
    float ry = input.y / sum;
    float rY = input.y;
    
    return float3(rx,ry,rY);
}

float3 XYY2RGB(float3 input)
{
    float3 valXYZ = XYY2XYZ(input);
    return saturate(XYZ2RGB(valXYZ));
}

float3 RGB2XYY(float3 input)
{
    float3 valXYZ = RGB2XYZ(input);
    return XYZ2XYY(valXYZ);
}

#ifdef VERTEX_PROFILE

VS_OUT vs_copy_as_is( VS_IN input )
{
	VS_OUT output;
	output.Position = input.Position;
	output.uv0 = input.uv0;
	return output;
}

VS_OUT_T2 vs_copy_as_is_embedded( VS_IN input )
{
	VS_OUT_T2 output;
	output.Position = input.Position;
	output.uv0.zw = frac(input.uv0);
    output.uv0.xy = (input.uv0 - output.uv0.zw) / 256.0f;
	return output;
}

VS_PLANE_OUT vs_worldviewproj(VS_IN input)
{
	VS_PLANE_OUT output;
	output.Position = mul(input.Position, vs_mWorldViewProjection);
	output.Position.z = clamp(output.Position.z, 0.0f, output.Position.w);
	output.uv0 = input.uv0;
	output.pospostvs = output.Position;
	
	return output;
}

// Perspective correction
VS_PC_OUT vs_copy_as_is_PC( VS_PC_IN input )
{
	VS_PC_OUT output;
	output.Position = input.Position;
	output.uv = input.uv1.xyz;
	output.uv2 = input.uv2;
	return output;
}
#if defined( _CAFE_ ) || defined ( _NX_ )
VS_Particle_OUT vs_particles( VS_PC_IN input )
{
	VS_Particle_OUT output;
	output.Position = input.Position;
	output.uv = float4(input.uv1.xyz,1);
	output.uv2 = input.uv2;
	return output;
}
#else

// particles stuff
VS_Particle_OUT vs_particles( VS_PC_IN input )
{
	VS_Particle_OUT output;

    const float startRadius     = vs_reg0.x;
    const float endRadius       = vs_reg0.y;
    const float minSpin         = vs_reg0.z;
    const float maxSpin         = vs_reg0.w;

    const float minWanderAmp    = vs_reg1.x * 0.01f;
    const float maxWanderAmp    = vs_reg1.y * 0.01f;
    const float radiusVar       = vs_reg1.z;
    const float time            = vs_reg1.w;

    const float minSpeed        = vs_reg2.x;
    const float maxSpeed        = vs_reg2.y;
    const float dirX            = vs_reg2.z;
    const float dirY            = vs_reg2.w;

    const float minWanderRate   = vs_reg3.x;
    const float maxWanderRate   = vs_reg3.y;
    const float radiusNoiseAmp  = vs_reg3.z;
    const float radiusNoiseRate = vs_reg3.w;

    const float3 stColxyY       = vs_reg4.xyz;
    const float stAlpha         = vs_reg4.w;
    const float3 edColxyY       = vs_reg5.xyz;
    const float edAlpha         = vs_reg5.w;

    const float imageU          = vs_reg6.x;
    const float motionPower     = vs_reg6.y;
    const float aspect          = vs_reg6.z;

    const float4 randVals       = input.uv1;
    const float4 sinRateCoeffs  = float4(1.0f,2.0f,4.0f,8.0f) * radiusNoiseRate;
    const float4 sinAmpCoeffs   = float4(0.53333f,0.26667f,0.13333f,0.06667f);

    float2  currPos             = input.Position.xy;
    float2  cornerOffset        = (input.uv0 * 2.0f) - float2(1.0f,1.0f);
    float2  dirVector           = vs_reg2.zw;
    float2  edgeVector          = float2(-dirVector.y,dirVector.x);

    float   posdot              = dot(currPos,dirVector);
    float   edgedot             = dot(currPos,edgeVector);
    float   currAngle           = time * lerp(minSpin,maxSpin,randVals.w);
    currAngle                   += atan2(dirVector.y,dirVector.x) - (1.57f);

    float   sinAng              = sin(currAngle);
    float   cosAng              = cos(currAngle);

    float2  cornerPos           = float2((cornerOffset.x * cosAng) + (cornerOffset.y * -sinAng),(cornerOffset.x * sinAng) + (cornerOffset.y * cosAng));

    float lifeRatio             = frac((time * lerp(minSpeed,maxSpeed,randVals.x)) + posdot);
    lifeRatio                   = pow(lifeRatio,motionPower);
    float linePos               = ((lifeRatio * 2.0f) - 1.0f) * 1.414f;

    float4 radNoiseLoops        = sin((lifeRatio.xxxx + randVals) * sinRateCoeffs) * radiusNoiseAmp;
    float  radiusNoise          = dot(radNoiseLoops,sinAmpCoeffs);

    float radius                = (1.0f / 40.0f) * max(0.0f,lerp(startRadius,endRadius,lifeRatio) * (1.0f + radiusNoise + (radiusVar * ((randVals.y * 2.0f) - 1.0f))));
    float lineWander            = sin(time * lerp(minWanderRate,maxWanderRate,randVals.z)) * lerp(minWanderAmp,maxWanderAmp,randVals.x);
    
    float3 colRGB               = XYY2RGB(lerp(stColxyY,edColxyY,lifeRatio * (0.5f + (randVals.x * 1.0f))));
    float  colAlpha             = lerp(stAlpha,edAlpha,lifeRatio);

    edgedot                     += lineWander;

    currPos                     = (dirVector * linePos) + (edgeVector * edgedot);
    cornerPos                   *= radius;
    cornerPos.x                 *= 1.0f / aspect;

    float2  finalPos = currPos + cornerPos;

	output.Position      = float4(finalPos,0,input.Position.w);
	output.uv.xy    = input.uv0;
    output.uv.x     *= (1.0f / 12.0f);  
    output.uv.x     += imageU;
    output.uv.z     = 0.0f;
    output.uv.w     = 0.0f;
	output.uv2      = float4(colRGB.xyz,colAlpha);
	return output;
}
#endif // not _CAFE_ and not _NX_
#endif // VERTEX_PROFILE

#ifdef PIXEL_PROFILE

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Utility functions

inline float linStep( float xmin, float xmax, float x )
{
	float a = 1.0/( xmax - xmin );
	float b = -a * xmin;
	return saturate( ( a*x )+b );
}

inline float rectFunc( float xmin, float xmax, float x )
{
	return step(xmin, x) * ( 1.0f - step(xmax, x) );
}

inline float2 rectFunc( float xmin, float xmax, float2 x )
{
	return step(xmin, x) * ( 1.0f - step(xmax, x) );
}

inline float3 rectFunc( float xmin, float xmax, float3 x )
{
	return step(xmin, x) * ( 1.0f - step(xmax, x) );
}

inline float4 rectFunc( float4 xmin, float4 xmax, float4 x )
{
	return step(xmin, x) * ( 1.0f - step(xmax, x) );
}

inline float colorDistance_max( float3 a, float3 b )
{
    return max
	( 
        max
		( 
			abs( a.x-b.x ), abs( a.y-b.y )
		 ),
        abs( a.z-b.z )
	 );
}

inline float colorDistance_avg( float3 a, float3 b )
{
    float3 dist = abs( a - b );
	return ( dist.x + dist.y + dist.z ) / 3.0f;
}

inline float colorDistance_euclidean( float3 a, float3 b )
{
	const float sqrt3 = 1.7321f;
    float3 dist = abs( a - b );
	return (float) length( dist ) / sqrt3;
}

inline float Max( float f1, float4 v )
{
	float2 v1 = float2( v.x, v.y );
	float2 v2 = float2( v.z, v.w );
	v1 = max( v1, v2 );
	v1 = max( v1.x, v1.y );
	return max( f1, v1.x );
}

inline float Max( float4 v )
{
	float2 v1 = float2( v.x, v.y );
	float2 v2 = float2( v.z, v.w );
	v1 = max( v1, v2 );
	return max( v1.x, v1.y );
}

inline float Min( float f1, float4 v )
{
	float2 v1 = float2( v.x, v.y );
	float2 v2 = float2( v.z, v.w );
	v1 = min( v1, v2 );
	v1 = min( v1.x, v1.y );
	return min( f1, v1.x );
}

inline float Min( float4 v )
{
	float2 v1 = float2( v.x, v.y );
	float2 v2 = float2( v.z, v.w );
	v1 = min( v1, v2 );
	return min( v1.x, v1.y );
}

inline float Sum( float4 v )
{
	const float4 vec4_one = float4(1.0f,1.0f,1.0f,1.0f);
	return dot( v, vec4_one );
}

#ifdef DX11_SHADERS
float4 tex2D_blurFast( Texture2D text_smp0, SamplerState samp_smp0, float2 uv, float2 texelOffset )
#else
float4 tex2D_blurFast( sampler2D smp0, float2 uv, float2 texelOffset )
#endif
{
	float2 off   = texelOffset;
	float2 off_h = off * 0.5;

	float4 tCenter = TEXTURE_READ_2D( smp, 0, uv );
	float4 t0      = TEXTURE_READ_2D( smp, 0, uv + float2( -off_h.x,  off.y  ) );
	float4 t1      = TEXTURE_READ_2D( smp, 0, uv + float2( off.x,    off_h.y ) );
	float4 t2      = TEXTURE_READ_2D( smp, 0, uv + float2( off_h.x, -off.y  ) );
	float4 t3      = TEXTURE_READ_2D( smp, 0, uv + float2( -off.x,   -off_h.y ) );

	return( ( t0+t1+t2+t3 )*0.25*0.4 ) +( tCenter*0.6 );
}

#ifdef DX11_SHADERS
float4 tex2D_blurG3( Texture2D text_smp0, SamplerState samp_smp0, float2 uv, float2 texelOffset )
#else
float4 tex2D_blurG3( sampler smp0, float2 uv, float2 texelOffset )
#endif
{
	 float2 off = texelOffset;

	float4 res = TEXTURE_READ_2D( smp, 0, uv + float2( -1*off.x,  -1*off.y ) ) *0.07511;
	res += TEXTURE_READ_2D( smp, 0, uv + float2( 0*off.x,  -1*off.y ) ) *0.12384;
	res += TEXTURE_READ_2D( smp, 0, uv + float2( 1*off.x,  -1*off.y ) ) *0.07511;

	res += TEXTURE_READ_2D( smp, 0, uv + float2( -1*off.x,   0*off.y ) ) *0.12384;
	res += TEXTURE_READ_2D( smp, 0, uv + float2( 0*off.x,   0*off.y ) ) *0.20418;
	res += TEXTURE_READ_2D( smp, 0, uv + float2( 1*off.x,   0*off.y ) ) *0.12384;

	res += TEXTURE_READ_2D( smp, 0, uv + float2( -1*off.x,   1*off.y ) ) *0.07511;
	res += TEXTURE_READ_2D( smp, 0, uv + float2( 0*off.x,   1*off.y ) ) *0.12384;
	res += TEXTURE_READ_2D( smp, 0, uv + float2( 1*off.x,   1*off.y ) ) *0.07511;

	return res;
}

#ifdef DX11_SHADERS
float4 tex2D_blurH( Texture2D text_smp0, SamplerState samp_smp0, float2 uv, float2 texelOffset )
#else
float4 tex2D_blurH( sampler smp0, float2 uv, float2 texelOffset )
#endif
{
	float2 off = texelOffset;
		
	float4 res = 	TEXTURE_READ_2D( smp, 0, float2(uv.x - 4.0*off.x,	uv.y)) * 0.05;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x - 3.0*off.x, 	uv.y)) * 0.09;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x - 2.0*off.x, 	uv.y)) * 0.12;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x - off.x, 		uv.y)) * 0.15;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x, 				uv.y)) * 0.16;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x + off.x, 		uv.y)) * 0.15;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x + 2.0*off.x, 	uv.y)) * 0.12;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x + 3.0*off.x, 	uv.y)) * 0.09;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x + 4.0*off.x, 	uv.y)) * 0.05;
	
	return res;
}

#ifdef DX11_SHADERS
float4 tex2D_blurV( Texture2D text_smp0, SamplerState samp_smp0, float2 uv, float2 texelOffset )
#else
float4 tex2D_blurV( sampler smp0, float2 uv, float2 texelOffset )
#endif
{
	float2 off = texelOffset;
		
	float4 res = 	TEXTURE_READ_2D( smp, 0, float2(uv.x, uv.y - 4.0*off.y	)) * 0.05;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x, uv.y - 3.0*off.y 	)) * 0.09;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x, uv.y - 2.0*off.y 	)) * 0.12;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x, uv.y - off.y 		)) * 0.15;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x, uv.y 				)) * 0.16;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x, uv.y + off.y		)) * 0.15;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x, uv.y + 2.0*off.y	)) * 0.12;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x, uv.y + 3.0*off.y	)) * 0.09;
	res += 			TEXTURE_READ_2D( smp, 0, float2(uv.x, uv.y + 4.0*off.y	)) * 0.05;
	
	return res;
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Pixel Shaders( filters )

PS_OUT ps_add_color_then_blend( VS_OUT input )
{
	float4 oldImage = TEXTURE_READ_2D( samp, 0, input.uv0 ) + ps_reg0;

	float4 newImage = TEXTURE_READ_2D( samp, 1, input.uv0 );

	float  finalAlpha = newImage.w +( oldImage.w*( 1.0-newImage.w ) );
	float3 finalClr   =( newImage.xyz * newImage.w ) +( oldImage.xyz*oldImage.w*( 1.0-newImage.w ) );
	finalClr /= finalAlpha + 0.001f;
	
	PS_OUT output;
	output.clr = float4( finalClr, finalAlpha );
	return output;
}

PS_OUT ps_alpha_blend( VS_OUT input )
{
	float4 oldImage = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float4 newImage = TEXTURE_READ_2D( samp, 1, input.uv0 );

	float  finalAlpha = newImage.w +( oldImage.w*( 1.0-newImage.w ) );
	float3 finalClr   =( newImage.xyz * newImage.w ) +( oldImage.xyz*oldImage.w*( 1.0-newImage.w ) );
	finalClr /= finalAlpha + 0.001f;

	PS_OUT output;
	output.clr = float4( finalClr, finalAlpha );
	return output;
}

PS_OUT ps_blur_g3( VS_OUT input )
{
	PS_OUT output;
	output.clr = tex2D_blurG3( SAMPLER_PARAM(samp, 0), input.uv0, ps_samp0Size.zw ).xyzw;
	return output;
}

PS_OUT ps_blur_h( VS_OUT input )
{
	PS_OUT output;
	output.clr = tex2D_blurH( SAMPLER_PARAM(samp, 0), input.uv0, ps_samp0Size.zw ).xyzw;
	return output;
}

PS_OUT ps_blur_v( VS_OUT input )
{
	PS_OUT output;
	output.clr = tex2D_blurV( SAMPLER_PARAM(samp, 0), input.uv0, ps_samp0Size.zw ).xyzw;
	return output;
}

PS_OUT ps_color_grading( VS_OUT input )
{
	float3 img_c = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyz;

	float3 recolored;
	float  recoloredOpacity;
	{
		float3 clr3 = img_c * 0.333333;
		float clr = clr3.x + clr3.y + clr3.z;

		float lowToMid       = ps_reg3.x;
		float lowToMidWidth  = ps_reg3.y;
		float midToHigh      = ps_reg3.z;
		float midToHighWidth = ps_reg3.w;

		float d0 = linStep( lowToMid+( 0.5*lowToMidWidth ), lowToMid-( 0.5*lowToMidWidth ), clr );
		float d2 = linStep( midToHigh-( 0.5*midToHighWidth ), midToHigh+( 0.5*midToHighWidth ), clr );
		float d1 = 1.0-( d0+d2 );

		recoloredOpacity =
			 ( d0 * ps_reg0.w )
			+( d1 * ps_reg1.w )
			+( d2 * ps_reg2.w );

		recolored =  ( ps_reg0.xyz * d0 * clr/lowToMid )
		            +( ps_reg1.xyz * d1 )
		            +( ps_reg2.xyz * d2 *( clr/( 1.0-midToHigh )+midToHigh ) );
	}

#ifdef DX11_SHADERS
	if( ps_alphaTest.x )
	{
		clip( recoloredOpacity - ps_alphaTest.y );
	}
#endif

	PS_OUT output;
	output.clr = float4( recolored, recoloredOpacity );
	return output;
}

PS_OUT ps_copy_as_is( VS_OUT input )
{
	PS_OUT output;
	output.clr = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw;
	return output;
}

PS_OUT ps_copy_fix_edge( VS_OUT input )
{
#if 1
	PS_OUT output;
	output.clr = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw;
	return output;
#else

	PS_OUT output;
	const float2 pixelOffset = ps_samp0Size.zw * 2.0f;
	
	const float2 UV0c = input.uv0 + float2(-pixelOffset.x,0);
	const float2 UV1c = input.uv0 + float2(pixelOffset.x,0);
	const float2 UVcc = input.uv0;
	const float2 UVc0 = input.uv0 + float2(0,-pixelOffset.y);
	const float2 UVc1 = input.uv0 + float2(0,pixelOffset.y);

	float4 col  = TEXTURE_READ_2D( samp, 0, UVcc);
	float col0c = TEXTURE_READ_2D( samp, 0, UV0c).r;
	float col1c = TEXTURE_READ_2D( samp, 0, UV1c).r;
	float colcc = col.r;
	float colc0 = TEXTURE_READ_2D( samp, 0, UVc0).r;
	float colc1 = TEXTURE_READ_2D( samp, 0, UVc1).r;

	float xdist = col1c - col0c;
	float ydist = colc1 - colc0;

	float dist  = sqrt((xdist * xdist) + (ydist * ydist));
	float scale = saturate(colcc * dist);

	output.clr  = scale * col;
	output.clr	= float4(dist.xxx,1.0f);

	return output;
#endif
}

PS_OUT ps_copy_as_is_alpha_test(VS_OUT input)
{
	float4 color = TEXTURE_READ_2D( samp, 0, input.uv0 );

#ifdef DX11_SHADERS
	if( ps_alphaTest.x )
	{
		clip( color.a - ps_alphaTest.y );
	}
#endif

	PS_OUT output;
	output.clr = color;
	return output;
}

PS_OUT ps_blend_with_mask( VS_OUT input )
{
	PS_OUT output;
	float4 clr0 = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float4 clr1 = TEXTURE_READ_2D( samp, 1, input.uv0 );
	float mask  = TEXTURE_READ_2D( samp, 2, input.uv0 ).r;
	
	float4 clr = lerp( clr0, clr1, mask );

	float alpha = lerp( saturate(clr0.a * ps_reg0.x + clr1.a * ps_reg0.y),
						clr.a,
						ps_reg0.x * ps_reg0.y );

	output.clr = float4( clr.rgb, alpha );

	return output;
}

PS_OUT ps_blend_with_mask_color( VS_OUT input )
{
	PS_OUT output;
	float4 clr0 = ps_reg1;
	float4 clr1 = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float mask  = TEXTURE_READ_2D( samp, 1, input.uv0 ).r;
	
	float4 clr = lerp( clr0, clr1, mask );

	float alpha = lerp( saturate(clr0.a * ps_reg0.x + clr1.a * ps_reg0.y),
						clr.a,
						ps_reg0.x * ps_reg0.y );

	output.clr = float4( clr.rgb, alpha );

	return output;
}

PS_OUT ps_blend_premul( VS_OUT input )
{
	PS_OUT output;
  	float4 clr0 = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float4 clr1 = TEXTURE_READ_2D( samp, 1, input.uv0 );
	
    clr1 *= (1.0f - clr0.a);

	output.clr = float4(clr1.rgb + clr0.rgb,1.0f);

	return output;
}

PS_OUT ps_mask_smoothstepped_img( VS_OUT input )
{
	float playerMask = TEXTURE_READ_2D( samp, 1, input.uv0 /*- float2( ps_samp1Size.z*2, -ps_samp1Size.w*2 )*/ ).x;
	playerMask = linStep( ps_reg2.x, ps_reg2.y, playerMask );
	float4 color = float4( TEXTURE_READ_2D( samp, 0, input.uv0 ).xyz, playerMask );

	PS_OUT output;
	output.clr = color;
	return output;
}

PS_OUT ps_mask_spike_clr( VS_OUT input )
{
	float tinput    = TEXTURE_READ_2D( samp, 1, input.uv0 - float2( ps_samp1Size.z*2, -ps_samp1Size.w*2 ) ).x;

	float middle    = ps_reg2.x;
	float thickness = ps_reg2.y;
	float smoothFactor    = ps_reg2.z;

	float lowStart  = middle -( thickness*0.5 ) - smoothFactor;
	float lowEnd    = middle -( thickness*0.5 );
	float highStart = middle +( thickness*0.5 ) + smoothFactor;
	float highEnd   = middle +( thickness*0.5 );

	float contourMask = min( 
        smoothstep( lowStart,  lowEnd,  tinput ),
        smoothstep( highStart, highEnd, tinput ) );

#ifdef DX11_SHADERS
	if( ps_alphaTest.x )
	{
		clip( (ps_reg3.w*contourMask) - ps_alphaTest.y );
	}
#endif

    PS_OUT output;
    output.clr = float4( ps_reg3.xyz, ps_reg3.w*contourMask );
    return output;
}

PS_OUT ps_mul_color( VS_OUT input )
{
	PS_OUT output;

	output.clr = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw;
	output.clr *= ps_reg0;
	return output;
}

#ifdef DX11_SHADERS
float GetIRBulbCorrection(float2 screenUV)
{
    float2 posVal,posExp;
 
    float  val = 0.0f; 
    float2 pos = screenUV;
    pos      *= 2.0f;
    pos      -= 1.0f;
    pos.y    *= -1.0f;

    pos      = (pos * float2(2.307619f,1.216094f)) + float2(-0.430332f,-0.036529f);
    pos      = pos * pos;
 
    posExp   = pos * float2(-0.921264f,-0.683528f);
    posVal   = float2(0.474407f,0.779357f) / (1.0f - (posExp * 0.93138945f) + (posExp * posExp * 0.79807341f));
 
    val      += posVal.x * posVal.y;
    pos      = (pos * float2(4.335279f,2.030378f)) + float2(0.109023f,-0.406690f);
    pos      = pos * pos;
 
    posExp   = pos * float2(-0.004911f,-0.419739f);
    posVal   = float2(0.945161f,0.999998f) / (1.0f - (posExp * 0.93138945f) + (posExp * posExp * 0.79807341f));
 
    val      += posVal.x * posVal.y;
    pos      = (pos * float2(0.064690f,6.608789f)) + float2(0.521754f,-0.082028f);
    pos      = pos * pos;
 
    posExp   = pos * float2(-0.134696f,-0.393378f);
    posVal   = float2(0.213287f,-0.909062f) / (1.0f - (posExp * 0.93138945f) + (posExp * posExp * 0.79807341f));
 
    val      += posVal.x * posVal.y;
    pos      = (pos * float2(4.686698f,1.812515f)) + float2(-0.245583f,-0.135284f);
    pos      = pos * pos;
 
    posExp   = pos * float2(-0.422476f,-0.000599f);
    posVal   = float2(-0.482204f,0.487757f) / (1.0f - (posExp * 0.93138945f) + (posExp * posExp * 0.79807341f));
 
    val      += posVal.x * posVal.y;
    //< subtle balance bodge to avoid any zero divides.
    val      = 0.01f + (val * 0.99f);
    return 1.0f / saturate(val);
}

float ApplyImprovementMask( float player, VS_OUT input, float discrete )
{
    //< get centre pixel values for IR, depth and player ID
    const float offx            = 4.0f / (512.0f * 1.99f);
    const float offy            = 4.0f / (424.0f * 1.99f);
    const float2 uv0c          = float2(-offx,0) + input.uv0;
    const float2 uv1c          = float2(offx,0) + input.uv0;
    const float2 uvc0          = float2(0,-offy) + input.uv0;
    const float2 uvc1          = float2(0,offy) + input.uv0;

    float lumcc = TEXTURE_READ_2D( samp , 2, input.uv0).r;
    float lumIR = TEXTURE_READ_2D( samp , 1, input.uv0).r;

    float bulbCorrection   = GetIRBulbCorrection(input.uv0);
    lumIR   = lumIR * bulbCorrection;

    float originalIR = lumIR;
    
    float IRDist = lumIR * (lumcc * lumcc);

    float lum0c = TEXTURE_READ_2D( samp, 2, uv0c ).r;
    float lum1c = TEXTURE_READ_2D( samp, 2, uv1c ).r;
    float lumc0 = TEXTURE_READ_2D( samp, 2, uvc0 ).r;
    float lumc1 = TEXTURE_READ_2D( samp, 2, uvc1 ).r;

    float4 plPixels = float4(TEXTURE_READ_2D( samp, 0, uv0c ).r,TEXTURE_READ_2D( samp, 0, uv1c ).r,TEXTURE_READ_2D( samp, 0, uvc0 ).r,TEXTURE_READ_2D( samp, 0, uvc1 ).r);

    float avgs  = dot(plPixels,float4(0.25f,0.25f,0.25f,0.25f));//< calculate the average depth 
    float errx  = abs(lumcc - ((lum0c + lum1c) * 0.5f));        //< estimate the error in depth across the pixel on the X axis (the general pixel error without light atenuation bais) 
    float erry  = abs(lumcc - ((lumc0 + lumc1) * 0.5f));        //< estimate the error in depth across the pixel on the Y axis (the general pixel error without light atenuation bais)

    float errScale = exp(-50.0f * (errx + erry));

    float lum = saturate(errScale * (IRDist * 300000.0f));
    
    lum *= step( 0.3f, lum );

    lum = lerp( lum, ceil(lum), discrete );

    return player * lum;
}
#endif

PS_OUT ps_player_ir_correction( VS_OUT input )
{
    float player = TEXTURE_READ_2D( samp, 0, input.uv0 ).x;

#ifdef DX11_SHADERS
    player = ApplyImprovementMask( player, input, ps_reg0.x );
#endif

	PS_OUT output;
	output.clr = float4( player, player, player, player );
	return output;
}

PS_OUT ps_player_to_rgba( VS_OUT input )
{
	float player = saturate( TEXTURE_READ_2D( samp, 0, input.uv0 ).x * 255.0 );

	PS_OUT output;
	output.clr = float4( player, player, player, player );
	return output;
}

PS_OUT ps_player_to_rgba_extract( VS_OUT input )
{
	float playerVal = TEXTURE_READ_2D( samp, 0, input.uv0 ).x;
	float player = rectFunc( ps_reg0.x, ps_reg0.y, playerVal );	
	player = saturate( player * 255.0 );

	PS_OUT output;
	output.clr = float4( player, player, player, player );
	return output;
}

PS_OUT ps_rgba_to_yuv( VS_OUT input )
{
	float3 rgb      = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( ps_samp0Size.z*0.25, 0.0 ) );
	float3 rgbRight = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( ps_samp0Size.z*1.25, 0.0 ) ).xyz;

    PS_OUT output;

    output.clr.a = -0.148*rgb.r      - 0.291*rgb.g      + 0.439*rgb.b      + (128.0/255.0); // U
    output.clr.b =  0.257*rgb.r      + 0.504*rgb.g      + 0.098*rgb.b      + ( 16.0/255.0); // Y
    output.clr.g =  0.439*rgbRight.r - 0.368*rgbRight.g - 0.071*rgbRight.b + (128.0/255.0); // V
    output.clr.r =  0.257*rgbRight.r + 0.504*rgbRight.g + 0.098*rgbRight.b + ( 16.0/255.0); // Y

#if defined(ITF_WIN32)
	// Windows DX9 & DX11
	output.clr.rgba = output.clr.gbar;
#elif defined(DX11_SHADERS)
	// Durango & Orbis
	output.clr.rgba = output.clr.gbar;
#elif defined(_CAFE_) || defined(_NX_)
	// Cafe
	output.clr.rgba = output.clr.abgr;	// Output actual UYVY
#endif

    return output;
}

float4 calcSobel_samp0(VS_OUT In,float edgeWidth,float colScale)
{
	float2 offset = ps_samp0Size.zw*edgeWidth;
  	float OffsetX = offset.x;
	float OffsetY = offset.y;
	
	float4 s00 = TEXTURE_READ_2D( samp, 0, In.uv0 + ( float2( -OffsetX, -OffsetY ) ) );
	float4 s01 = TEXTURE_READ_2D( samp, 0, In.uv0 + ( float2(      0.0, -OffsetY ) ) );
	float4 s02 = TEXTURE_READ_2D( samp, 0, In.uv0 + ( float2(  OffsetX, -OffsetY ) ) );

	float4 s10 = TEXTURE_READ_2D( samp, 0, In.uv0 + ( float2( -OffsetX,      0.0 ) ) );
	float4 s12 = TEXTURE_READ_2D( samp, 0, In.uv0 + ( float2(  OffsetX,      0.0 ) ) );

	float4 s20 = TEXTURE_READ_2D( samp, 0, In.uv0 + ( float2( -OffsetX,  OffsetY ) ) );
	float4 s21 = TEXTURE_READ_2D( samp, 0, In.uv0 + ( float2(      0.0,  OffsetY ) ) );
	float4 s22 = TEXTURE_READ_2D( samp, 0, In.uv0 + ( float2(  OffsetX,  OffsetY ) ) );
	
	// Calc X gradient
	float4 GradX = s00 + 2.0*s10 + s20 - ( s02 + 2.0*s12 + s22 );
	float4 GradY = s00 + 2.0*s01 + s02 - ( s20 + 2.0*s21 + s22 );
    float asum = max(max(max(s00.a,s01.a),max(s02.a,s10.a)),max(max(s12.a,s20.a),max(s21.a,s22.a)));
	
#ifdef DX11_SHADERS
	if( ps_alphaTest.x )
	{
		clip( asum - ps_alphaTest.y );
	}
#endif

	float4 SquareGrad = GradX*GradX + GradY*GradY;
	float4 final = sqrt( SquareGrad )*colScale;
	final.rgb *= asum;

	return final;
}

float4 calcSobel_samp1(VS_OUT In,float edgeWidth,float colScale)
{
	float2 offset = ps_samp0Size.zw*edgeWidth;
  	float OffsetX = offset.x;
	float OffsetY = offset.y;
	
	float4 s00 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2( -OffsetX, -OffsetY ) ) );
	float4 s01 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2(      0.0, -OffsetY ) ) );
	float4 s02 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2(  OffsetX, -OffsetY ) ) );
										
	float4 s10 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2( -OffsetX,      0.0 ) ) );
	float4 s12 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2(  OffsetX,      0.0 ) ) );
										
	float4 s20 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2( -OffsetX,  OffsetY ) ) );
	float4 s21 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2(      0.0,  OffsetY ) ) );
	float4 s22 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2(  OffsetX,  OffsetY ) ) );
	
	// Calc X gradient
	float4 GradX = s00 + 2.0*s10 + s20 - ( s02 + 2.0*s12 + s22 );
	float4 GradY = s00 + 2.0*s01 + s02 - ( s20 + 2.0*s21 + s22 );
    float asum = max(max(max(s00.a,s01.a),max(s02.a,s10.a)),max(max(s12.a,s20.a),max(s21.a,s22.a)));
	
#ifdef DX11_SHADERS
	if( ps_alphaTest.x )
	{
		clip( asum - ps_alphaTest.y );
	}
#endif

	float4 SquareGrad = GradX*GradX + GradY*GradY;
	float4 final = sqrt( SquareGrad )*colScale;
	final.rgb *= asum;

	return final;
}

float4 calcSobelGradiant_samp1Red(VS_OUT In,float edgeWidth,float scale)
{
	float2 offset = ps_samp0Size.zw*edgeWidth;
  	float OffsetX = offset.x;
	float OffsetY = offset.y;
	
	float s00 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2( -OffsetX, -OffsetY ) ) ).x;
	float s01 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2(      0.0, -OffsetY ) ) ).x;
	float s02 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2(  OffsetX, -OffsetY ) ) ).x;
									
	float s10 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2( -OffsetX,      0.0 ) ) ).x;
	float s11 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2( 0.0,           0.0 ) ) ).x;
	float s12 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2(  OffsetX,      0.0 ) ) ).x;
									
	float s20 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2( -OffsetX,  OffsetY ) ) ).x;
	float s21 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2(      0.0,  OffsetY ) ) ).x;
	float s22 = TEXTURE_READ_2D( samp, 1, In.uv0 + ( float2(  OffsetX,  OffsetY ) ) ).x;
	
	float2 Grad = float2(s00 + 2.0*s10 + s20 - ( s02 + 2.0*s12 + s22 ),s00 + 2.0*s01 + s02 - ( s20 + 2.0*s21 + s22 ));

    float4 bright0     = float4(s00,s01,s02,s10);
    float4 bright1     = float4(s12,s20,s21,s22);

    //< 9-tap gaussian on red channel only
    float bright0Avg   = dot(bright0,float4(0.07511f,0.12384f,0.07511f,0.12384f));
    float bright1Avg   = dot(bright1,float4(0.12384,0.07511,0.12384f,0.07511f));
    float brightAvg    = bright0Avg + bright1Avg + (s11.x * 0.20418);

	float sobel        = sqrt(dot(Grad,Grad)) * scale;

    float4 final        = float4(Grad.x * scale,Grad.y * scale,brightAvg,sobel);

	return final;
}

//#ifdef ALT_TOONSHADER
//PS_OUT ps_sobel_colored( VS_OUT In )
//{
//	float4 FragCol	= calcSobel_samp0(In,1.0f,1.0f);
//
//    PS_OUT output;
//    output.clr = FragCol;
//    return output;
//}
//#else
PS_OUT ps_sobel_colored( VS_OUT input )
{
    float3 img_c = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyz;
    float3 img_l = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z, 0.0 ) ).xyz;
    float3 img_r = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( ps_samp0Size.z, 0.0 ) ).xyz;
    float3 img_u = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0,  ps_samp0Size.w ) ).xyz;
    float3 img_d = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0, -ps_samp0Size.w ) ).xyz;

    float4 sobel;
    {
        float l_dot = colorDistance_euclidean( img_c, img_l );
        float r_dot = colorDistance_euclidean( img_c, img_r );
        float max_lr = max( l_dot, r_dot );

        float u_dot = colorDistance_euclidean( img_c, img_u );
        float d_dot = colorDistance_euclidean( img_c, img_d );
        float max_ud = max( u_dot, d_dot );

        float delta = max( max_lr, max_ud );
        sobel = ps_reg2;
        sobel.w *= saturate( pow( delta, ps_reg0.x ) * ps_reg0.y );
	}

    PS_OUT output;
    output.clr = sobel;
    return output;
}
//#endif

PS_OUT ps_yuv_pack_to_rgba( VS_OUT input )
{
    float2 clr      = TEXTURE_READ_2D( samp, 0, input.uv0                              ).xy;
    float2 clrRight = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( ps_samp0Size.z, 0.0 ) ).xy;
    float2 clrLeft  = TEXTURE_READ_2D( samp, 0, input.uv0 - float2( ps_samp0Size.z, 0.0 ) ).xy;
    
    float Fact = 1.164123535;
    float3 Crc = float3( 1.595794678, -0.813476563,          0.0 );
    float3 Crb = float3(         0.0, -0.391448975,  2.017822266 );
    float3 Adj = float3( -0.87065506,  0.529705048, -1.081668854 );
    float3x3 matToRgb = float3x3( 
                    float3(Crc.x, Crc.y, Crc.z),
                    float3(Crb.x, Crb.y, Crb.z),
                    float3(Adj.x, Adj.y, Adj.z) );
    
    float y, u, v;

    int isOdd = floor( input.uv0.x * ps_samp0Size.x );
    isOdd = isOdd -( ( isOdd / 2 ) * 2 );

    y = clr.x;
    if( isOdd )
    {
        u = clrLeft.y;
        v = clr.y;
    }
    else
    {
        u = clr.y;
        v = clrRight.y;
    }

    float3 res = mul( float3( v, u, 1.0 ), matToRgb );
    res += float3( y, y, y ) * Fact;
       
    PS_OUT output;
    output.clr = float4( res, 1.0 );
    
    return output;
}

PS_OUT ps_yuv_to_yuv_pack( VS_OUT input )
{
	// unpack YUV :
	//  x <=> Y
	//  y <=> U
	//  z <=> Y

	// packed YUV :
	//  x <=> U or V( half rez )
	//  y <=> Y

	float3 yuv       = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float3 yuv_right = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( ps_samp0Size.z, 0.0 ) ).xyz;
	float3 yuv_left  = TEXTURE_READ_2D( samp, 0, input.uv0 - float2( ps_samp0Size.z, 0.0 ) ).xyz;
	
	float3 yuv_pack;

	int isOdd = floor( input.uv0.x * ps_samp0Size.x );
	isOdd = isOdd -( ( isOdd / 2 ) * 2 );
	if( isOdd )
	{
		yuv_pack.y = yuv.x; // Y
		yuv_pack.x =( yuv.z + yuv_left.z ) * 0.5; //V
		yuv_pack.z = 0.0;
	}
	else
	{
		yuv_pack.y = yuv.x; // Y
		yuv_pack.x =( yuv.y + yuv_right.y ) * 0.5; //U
		yuv_pack.z = 0.0;
	}

	PS_OUT output;
	output.clr = float4( yuv_pack, 1.0 );
	return output;
}

PS_OUT ps_copy_depth( VS_OUT input )
{
	PS_OUT output;
	float depth = TEXTURE_READ_2D( samp, 0, input.uv0 ).x;

	float band_size  = ps_reg0.x;
	float banded     = fmod(depth.x, band_size);
	float contour    = abs((banded / (band_size / 2)) - 1);

	output.clr.rgba = float4(depth.x, contour, depth.x, 1);
	
	return output;
}
 
// RGB / HSL conversions
float3 rgb_to_hsl(float3 rgb)
{
    float3 hsl = 0;
    float2 MinMax;
    float  delta;

    MinMax.x = min( rgb.r, min(rgb.g, rgb.b) );
    MinMax.y = max( rgb.r, max(rgb.g, rgb.b) );

    hsl.z = 0.5 * (MinMax.x + MinMax.y);

    if (MinMax.x != MinMax.y)
    {
        delta = (MinMax.y - MinMax.x);

        if (hsl.z > 0.5)
            hsl.y = delta / (2 - MinMax.x - MinMax.y);
        else
            hsl.y = delta / (MinMax.x + MinMax.y);

        if (rgb.r == MinMax.y)
            hsl.x = ( (rgb.g - rgb.b) / delta );
        else if (rgb.g == MinMax.y)
            hsl.x = 2 + (rgb.b - rgb.r) / delta;
        else
            hsl.x = 4 + (rgb.r - rgb.g) / delta;

        hsl.x /= 6.0;
    
        if (hsl.x < 0.0)
            hsl.x += 1.0;
    }

    return hsl;
}

float hsl_value(float n1, float n2, float hue)
{
    float val;

    if (hue > 6.0)
        hue -= 6.0;
    else if (hue < 0.0)
        hue += 6.0;

    if (hue < 1.0)
        val = n1 + (n2 - n1) * hue;
    else if (hue < 3.0)
        val = n2;
    else if (hue < 4.0)
        val = n1 + (n2 - n1) * (4.0 - hue);
    else
        val = n1;

    return val;
}

float3 hsl_to_rgb(float3 hsl)
{
    float3 rgb = 0;

    if (hsl.y == 0)
    {
        rgb.r = hsl.z;
        rgb.g = hsl.z;
        rgb.b = hsl.z;
    }
    else
    {
        float m1, m2;

        if (hsl.z <= 0.5)
            m2 = hsl.z * (1.0 + hsl.y);
        else
            m2 = hsl.z + hsl.y - hsl.z * hsl.y;

        m1 = 2.0 * hsl.z - m2;

        rgb.r = hsl_value (m1, m2, hsl.x * 6.0 + 2.0);
        rgb.g = hsl_value (m1, m2, hsl.x * 6.0);
        rgb.b = hsl_value (m1, m2, hsl.x * 6.0 - 2.0);
    }

    return rgb;
}

// Contrast
float EnhanceContrast(float lighting)
{
	float weight = step(0.5f, lighting);
	float lNew = smoothstep(0.0f, 1.0f, lighting);
	lNew = lNew * 2.0f - 1.0f;
	lNew = lerp( saturate(pow(lNew + 1, ps_reg0.z)), saturate(pow(1 - lNew, ps_reg0.z)), weight );
	lNew = lerp( lNew - 1, 1 - lNew, weight );
	lNew = lNew * 0.5f + 0.5f;
	return lNew;
}

// UV Blackout
PS_OUT ps_uv_blackout( VS_OUT input )
{
	PS_OUT output;

	output.clr = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw;
	float lightness = TEXTURE_READ_2D( samp, 1, input.uv0 ).a;

	// Convert to HSL
	float3 hsl = rgb_to_hsl( output.clr.rgb );

	// Desaturate
	hsl.y *= ps_reg0.y;

	// Enhance lighting contrast
	hsl.z = EnhanceContrast(lightness);

	// Convert to RGB
	float3 clr = hsl_to_rgb( hsl );

	// Negative
	clr = 1 - clr;

	// Tint
	clr *= ps_reg1.rgb;

	// Brightness
	clr += ps_reg0.w;

	// Factor
	output.clr.rgb = lerp( output.clr.rgb, clr, ps_reg0.x );

	return output;
}

// --------------------------------------------------------------------
// 3x3 Median
// Based on Morgan McGuire and Kyle Whitson implementation in Shader X6

#define s2(a, b)				temp = a; a = min(a, b); b = max(temp, b);
#define mn3(a, b, c)			s2(a, b); s2(a, c);
#define mx3(a, b, c)			s2(b, c); s2(a, c);

#define mnmx3(a, b, c)			mx3(a, b, c); s2(a, b);                                   // 3 exchanges
#define mnmx4(a, b, c, d)		s2(a, b); s2(c, d); s2(a, c); s2(b, d);                   // 4 exchanges
#define mnmx5(a, b, c, d, e)	s2(a, b); s2(c, d); mn3(a, c, e); mx3(b, d, e);           // 6 exchanges
#define mnmx6(a, b, c, d, e, f) s2(a, d); s2(b, e); s2(c, f); mn3(a, b, c); mx3(d, e, f); // 7 exchanges

float4 ps_autodance_median3x3( VS_OUT In, float4 clrCenter )
{
    const float offx = 1.0f / 900.0f;
    const float offy = 1.0f / 600.0f;

    float4 v[9], temp;

	v[0] = TEXTURE_READ_2D( samp, 0, In.uv0 + float2( -offx, -offy ) ).xyzw;
	v[1] = TEXTURE_READ_2D( samp, 0, In.uv0 + float2(  0.0, -offy ) ).xyzw;
	v[2] = TEXTURE_READ_2D( samp, 0, In.uv0 + float2(  offx, -offy ) ).xyzw;
	v[3] = TEXTURE_READ_2D( samp, 0, In.uv0 + float2( -offx,  0.0 ) ).xyzw;
	v[4] = clrCenter;
	v[5] = TEXTURE_READ_2D( samp, 0, In.uv0 + float2(  offx,  0.0 ) ).xyzw;
	v[6] = TEXTURE_READ_2D( samp, 0, In.uv0 + float2( -offx,  offy ) ).xyzw;
	v[7] = TEXTURE_READ_2D( samp, 0, In.uv0 + float2(  0.0,   offy ) ).xyzw;
	v[8] = TEXTURE_READ_2D( samp, 0, In.uv0 + float2(  offx,  offy ) ).xyzw;
	
	// Starting with a subset of size 6, remove the min and max each time
	mnmx6(v[0], v[1], v[2], v[3], v[4], v[5]);
	mnmx5(v[1], v[2], v[3], v[4], v[6]);
	mnmx4(v[2], v[3], v[4], v[7]);
	mnmx3(v[3], v[4], v[8]);
  
	return v[4];
}
#ifdef ALT_TOONSHADER
// Toon shader
PS_OUT ps_toon( VS_OUT input )
{
	DEFINE_CONSTANTS;

	PS_OUT output;

	float4 clrCenter = TEXTURE_READ_2D( samp, 0, input.uv0 ).rgba;

    float4 col = ps_autodance_median3x3(input, clrCenter);

	// Convert to HSL
	float3 hsl = rgb_to_hsl( col.rgb );

	// apply toon banding.
    hsl.z = (ceil((hsl.z * ps_reg0.y) + 0.5f) - 0.5f) / ps_reg0.y;
    hsl.y = (ceil((hsl.y * ps_reg0.y) + 0.5f) - 0.5f) / ps_reg0.y;

    // Convert back to RGB
	float3 clr		= hsl_to_rgb( hsl );
    float4 refCol	= float4(clr,1);

	// Factor
	output.clr.rgb  = lerp( clrCenter.rgb, refCol.rgb, ps_reg0.x );
    output.clr.a    = col.a;

	return output;
}

float SmoothRect(float x,float loStart,float loRange,float hiStart,float hiRange)
{
    float lo  = ((x - loStart) / loRange);
    float hi  = 1.0 - ((x - hiStart) / hiRange);
    float res = min(lo,hi);
    return saturate(res);
}

PS_OUT ps_toon_outline( VS_OUT input )
{
	DEFINE_CONSTANTS;

	PS_OUT output;

	const float  lowThreshold = 0.01f;
	const float  highThreshold = 1.0f - lowThreshold;

	float4 playerMaskData = calcSobelGradiant_samp1Red(input,4.0f,0.25f);
    float playerMask = max(0,playerMaskData.z - (0.5f * (saturate(((1.0f - playerMaskData.z) - 0.5f) * 2.0f) * (1.0f - playerMaskData.w))));

    output.clr = TEXTURE_READ_2D( samp, 0, input.uv0 ).rgba;

    float finalAlpha    = SmoothRect(playerMask,0.0f,0.1f,0.6f,0.2f) * ps_reg1.x;
    float colBW         = (1.0f - saturate((playerMask - 0.35f) * 10.0f)) * finalAlpha;

    float4 outline      = float4(colBW.xxx,finalAlpha);

	output.clr = (output.clr * (1.0f - outline.a)) + outline;

    return output;
}
#else
// Toon shader
PS_OUT ps_toon( VS_OUT input )
{
	DEFINE_CONSTANTS;

	PS_OUT output;

	output.clr = TEXTURE_READ_2D( samp, 0, input.uv0 ).rgba;
	float sobel = TEXTURE_READ_2D( samp, 1, input.uv0 ).w;

	// Convert to HSL
	float3 hsl = rgb_to_hsl( output.clr );

	float lightness = hsl.z;

	// Cut out
	float bandSize = floor( 256.0f / ps_reg0.y );
	float bandIndex = 1.0f + floor(hsl.z * 255.0f / bandSize);
	hsl.z = saturate( bandIndex * bandSize / 255.0f );

	// Convert to RGB
	float3 clr = hsl_to_rgb( hsl );

	// Multiply blend mode
	clr *= clr;

	// Overlay blend mode
	float weight = step(0.5f, lightness);
	clr = lerp(
		clamp( pow(clr * output.clr.rgb, ps_reg0.z), vec3_zero, output.clr.rgb ),
		vec3_one - (vec3_one - clr) * (vec3_one - output.clr.rgb),
		weight);

	// Photocopy filter
	clr *= 1.0f - sobel;

	// Factor
	output.clr.rgb = lerp( output.clr.rgb, clr, ps_reg0.x );

	return output;
}

PS_OUT ps_toon_outline( VS_OUT input )
{
	DEFINE_CONSTANTS;

	PS_OUT output;

	const float  lowThreshold = 0.01f;
	const float  highThreshold = 1.0f - lowThreshold;

	output.clr = TEXTURE_READ_2D( samp, 0, input.uv0 ).rgba;

	float playerMask = TEXTURE_READ_2D( samp, 1, input.uv0 ).x;

	// Black contour

	float middle    = ps_reg0.x;
	float thickness = ps_reg0.y;
	float smoothFactor    = ps_reg0.z;

	float lowStart  = middle -( thickness*0.5 ) - smoothFactor;
	float lowEnd    = middle -( thickness*0.5 );
	float highStart = middle +( thickness*0.5 ) + smoothFactor;
	float highEnd   = middle +( thickness*0.5 );

	lowStart	= max(lowStart, lowThreshold);
	lowEnd		= max(lowEnd, lowThreshold);
	highStart	= min(highStart, highThreshold);
	highEnd		= min(highEnd, highThreshold);

	float contourMaskBlack = min( 
        smoothstep( lowStart,  lowEnd,  playerMask ),
        smoothstep( highStart, highEnd, playerMask ) );

	// White contour

	middle     = middle - (0.5 * ps_reg0.w * thickness + 0.5 * thickness + smoothFactor);
	thickness *= ps_reg0.w;

	lowStart  = middle -( thickness*0.5 ) - smoothFactor;
	lowEnd    = middle -( thickness*0.5 );
	highStart = middle +( thickness*0.5 ) + smoothFactor;
	highEnd   = middle +( thickness*0.5 );

	lowStart	= max(lowStart, lowThreshold);
	lowEnd		= max(lowEnd, lowThreshold);
	highStart	= min(highStart, highThreshold);
	highEnd		= min(highEnd, highThreshold);

	float contourMaskWhite = min( 
        smoothstep( lowStart,  lowEnd,  playerMask ),
        smoothstep( highStart, highEnd, playerMask ) );

	// Blend

	float4 clr = output.clr;
	clr.rgb = lerp( clr.rgb, vec3_one, contourMaskWhite );
	clr.rgb = lerp( clr.rgb, vec3_zero, contourMaskBlack );

	// Set Alpha at the outline
	clr.a = lerp( output.clr.a, 1.0f, max( contourMaskWhite, contourMaskBlack ) );

	// Factor
	output.clr = lerp( output.clr, clr, ps_reg1.x );

    return output;
}
#endif

// Half tone
PS_OUT ps_half_tone( VS_OUT input )
{
	const float factor = 0.35f;

	PS_OUT output;

	output.clr = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw;
	float lightness = TEXTURE_READ_2D( samp, 2, input.uv0 ).r;

	// Convert to HSL
	float3 hsl = rgb_to_hsl( output.clr );

	// Cut out
	float bandSize = floor( 256.0f / ps_reg0.y );
	float bandIndex = clamp( floor(hsl.z * 255.0f / bandSize), 1.0f, ps_reg0.y );
	hsl.z = bandIndex * bandSize / 255.0f;

	// Convert to RGB
	float3 clr = hsl_to_rgb( hsl );

	// Apply halftone pattern
	const float4 xmin = float4(0.0f, 0.15f, 0.3f, 0.45f);
	const float4 xmax = float4(0.15f, 0.3f, 0.45f, 0.6f);
	const float tiling = 40.0;
	float4 halftoneTx = TEXTURE_READ_2D( samp, 1, input.uv0 * tiling ).xyzw;
	float4 halftone = halftoneTx * rectFunc(xmin, xmax, lightness);
	halftone = factor * dot(halftone, float4(1,1,1,1));
	halftone = 1.0f - halftone;

	// Blend layers
	output.clr.rgb = lerp( output.clr.rgb, clr * halftone.x, ps_reg0.x );

	return output;
}
PS_OUT ps_mask_body_part(VS_OUT_T2 input)
{
    float2 linkage = float2(input.uv0.zw * 2.0f) - float2(1.0f,0.5f);
    linkage        *= float2(1.7f,-0.9f);
    linkage         = float2(0,0);//normalize(linkage) * 0.005f;

    float playerMask = TEXTURE_READ_2D( samp, 1, input.uv0.xy - linkage).x;
	
    playerMask = linStep( ps_reg2.x, ps_reg2.y, max(0,playerMask ));
	float4 color = float4( TEXTURE_READ_2D( samp, 0, input.uv0 ).xyz, playerMask );

	PS_OUT output;
	output.clr = color;
	return output;
}

PS_OUT ps_mask_body_part_hide(VS_OUT input)
{
	float3 color0 = TEXTURE_READ_2D( samp, 0, ps_reg0.xy ).xyz;
	float3 color1 = TEXTURE_READ_2D( samp, 0, ps_reg0.zw ).xyz;
	float3 color2 = TEXTURE_READ_2D( samp, 0, ps_reg1.xy ).xyz;
	float3 color3 = TEXTURE_READ_2D( samp, 0, ps_reg1.zw ).xyz;
	
	float playerMask = TEXTURE_READ_2D( samp, 1, input.uv0 ).x;
	playerMask = linStep( ps_reg2.x, ps_reg2.y, playerMask );
	float4 color = float4( TEXTURE_READ_2D( samp, 0, input.uv0.xy ).xyz, playerMask );

	float4 colorF = float4( ( ( color0 + color1 + color2 + color3 ) / 4.0f), playerMask );

	PS_OUT output;
	output.clr = lerp( color, colorF, playerMask);
    //output.clr = float4(playerMask.xxx,1.0f);

	return output;	
}


// Perspective correction
PS_OUT ps_mask_body_part_PC(VS_PC_OUT input)
{
    float2 uv = input.uv.xy / input.uv.z;
	float playerMask = TEXTURE_READ_2D( samp, 1, uv ).x;
	playerMask = linStep( ps_reg2.x, ps_reg2.y, playerMask );
	float4 color = float4( TEXTURE_READ_2D( samp, 0, uv ).xyz, playerMask );

    float2 uv2 = input.uv2.xy / input.uv2.z;
	float playerMask2 = TEXTURE_READ_2D( samp, 2, uv2 ).x;
	playerMask2 = linStep( ps_reg2.x, ps_reg2.y, playerMask2 );	
	float4 color2 = float4( TEXTURE_READ_2D( samp, 0, uv2 ).xyz, playerMask2 );
	
	float val = 0.0f;
	val += ( 1.0f - rectFunc( 0.0f, ps_reg2.z, input.uv2.w ) ) * ( 1.0f - rectFunc( ps_reg2.z, 1.0f, input.uv2.w ) ) ;
	val += rectFunc( ps_reg2.z, ps_reg2.w, input.uv2.w ) * ( ( input.uv2.w - ps_reg2.z ) / ( ps_reg2.w - ps_reg2.z ) );
	val += rectFunc( ps_reg2.w, 1.0f, input.uv2.w );
	val = saturate(val);

	PS_OUT output;
	output.clr = lerp( color, color2, val );
	return output;
}

PS_OUT ps_mask_body_part_depth(VS_OUT input)
{
	float playerMask = TEXTURE_READ_2D( samp, 1, input.uv0 ).x;
	playerMask = linStep( ps_reg2.x, ps_reg2.y, playerMask );
	float4 color = float4( TEXTURE_READ_2D( samp, 0, input.uv0 ).xyz, playerMask );

	clip( playerMask - 1.0/255.0f );

	PS_OUT output;
	output.clr = color;
	return output;
}

// Perspective correction
PS_OUT ps_mask_body_part_depth_PC(VS_PC_OUT input)
{
    float2 uv = input.uv.xy / input.uv.z;
	float playerMask = TEXTURE_READ_2D( samp, 1, uv ).x;
	playerMask = linStep( ps_reg2.x, ps_reg2.y, playerMask );
	float4 color = float4( TEXTURE_READ_2D( samp, 0, uv ).xyz, playerMask );

    float2 uv2 = input.uv2.xy / input.uv2.z;
	float playerMask2 = TEXTURE_READ_2D( samp, 2, uv2 ).x;
	playerMask2 = linStep( ps_reg2.x, ps_reg2.y, playerMask2 );	
	float4 color2 = float4( TEXTURE_READ_2D( samp, 0, uv2 ).xyz, playerMask2 );
	
	float val = 0.0f;
	val += ( 1.0f - rectFunc( 0.0f, ps_reg2.z, input.uv2.w ) ) * ( 1.0f - rectFunc( ps_reg2.z, 1.0f, input.uv2.w ) ) ;
	val += rectFunc( ps_reg2.z, ps_reg2.w, input.uv2.w ) * ( ( input.uv2.w - ps_reg2.z ) / ( ps_reg2.w - ps_reg2.z ) );
	val += rectFunc( ps_reg2.w, 1.0f, input.uv2.w );
	val = saturate(val);

	float4 colorRes = lerp( color, color2, val );
	clip( colorRes.w - 1.0/255.0f );

	PS_OUT output;
	output.clr = colorRes;
	return output;
}

#define BODY_PART_HEAD			1.0f
#define BODY_PART_HAND_LEFT		2.0f
#define BODY_PART_HAND_RIGHT	3.0f
#define BODY_PART_FOOT_LEFT		4.0f
#define BODY_PART_FOOT_RIGHT	5.0f
#define BODY_PART_MAX			BODY_PART_FOOT_RIGHT
#define BODY_PART_MARGIN		0.5f / BODY_PART_MAX

float isPart( float partIndex, float bodyPart )
{
	return rectFunc( bodyPart - BODY_PART_MARGIN, bodyPart + BODY_PART_MARGIN, partIndex );
}

PS_OUT ps_mask_body_part_index(VS_OUT input)
{
	float playerMask = TEXTURE_READ_2D( samp, 0, input.uv0 ).x;

#ifdef DX11_SHADERS
	if( ps_alphaTest.x )
	{
		clip( playerMask - ps_alphaTest.y );
	}
#endif

	PS_OUT output;
	output.clr = ps_reg2.x / BODY_PART_MAX;
	output.clr.a = playerMask;
	return output;
}

PS_OUT ps_mask_body_part_index_PC(VS_PC_OUT input)
{
    float2 uv = input.uv.xy / input.uv.z;
	float playerMask = TEXTURE_READ_2D( samp, 0, uv ).x;

#ifdef DX11_SHADERS
	if( ps_alphaTest.x )
	{
		clip( playerMask - ps_alphaTest.y );
	}
#endif

	PS_OUT output;
	output.clr = ps_reg2.x / BODY_PART_MAX;
	output.clr.a = playerMask;
	return output;
}

PS_OUT ps_update_lightness( VS_OUT input )
{
	PS_OUT output;

	float3 clr			= TEXTURE_READ_2D( samp, 0, input.uv0 ).rgb;
	float4 lightPrev	= TEXTURE_READ_2D( samp, 1, input.uv0 );

	float3 hsl = rgb_to_hsl( clr );
	
	float4 lightNew = hsl.z;

	output.clr = lerp( lightPrev, lightNew, ps_reg0 );

	return output;
}

// NuiToWorld (see DepthVisualizer example)
float3 NuiToWorld( float3 vNuiPosition )
{
    float3 vWorldPosition;
    
    vWorldPosition.xy = vNuiPosition.z * ps_reg0.xy * ( vNuiPosition.xy - 0.5f );
    vWorldPosition.z = vNuiPosition.z;
    
    return vWorldPosition;
}

// Normal Map
PS_OUT ps_normal_map(VS_OUT input)
{
	float4 depths  = float4(
		TEXTURE_READ_2D( samp, 0, input.uv0 ).r,
		TEXTURE_READ_2D( samp, 0, input.uv0 + float2( ps_samp0Size.z, 0.0 ) ).r,
		TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0, ps_samp0Size.w ) ).r,
		1.0f );

    // Recover the 3 worldspace sample coordinates
    float3 vWorld00 = NuiToWorld( float3( input.uv0, depths.x ) );
    float3 vWorld10 = NuiToWorld( float3( input.uv0 + float2( ps_samp0Size.z, 0.0 ), depths.y ) );
    float3 vWorld01 = NuiToWorld( float3( input.uv0 + float2( 0.0, ps_samp0Size.w ), depths.z ) );
    
    // From the change in depth in the x and the y direction, compute the viewspace normal vector
    float3 vTangent = vWorld10 - vWorld00;
    float3 vBinormal = vWorld01 - vWorld00;
	float3 vNormal = normalize( cross( vTangent, vBinormal ) );

	PS_OUT output;
	output.clr = float4( vNormal * 0.5f + 0.5f, 1.0f );
	return output;
}

// Bilateral depth filter

#define KER_HALFSIZE		3			// Fixed kernel size
#define KER_DIRECTION		ps_reg0.xy
#define BILATERAL_ATT		ps_reg0.z
#define KER_WEIGHTS			ps_reg1

PS_OUT ps_filter_depth(VS_OUT input)
{
	// Construct weights
	float vWeights[ KER_HALFSIZE * 2 + 1 ] = { KER_WEIGHTS.w, KER_WEIGHTS.z, KER_WEIGHTS.y, KER_WEIGHTS.x, KER_WEIGHTS.y, KER_WEIGHTS.z, KER_WEIGHTS.w };

	// Take samples

	float vSamples[ KER_HALFSIZE * 2 + 1 ];
	for (int i = -KER_HALFSIZE; i <= KER_HALFSIZE; ++i)
	{
		float2 uvOffset = KER_DIRECTION * ps_samp0Size.zw * i;
		vSamples[ KER_HALFSIZE + i ] = TEXTURE_READ_2D( samp, 0, input.uv0 + uvOffset ).r;
	}

	// Average them taking edges into account
	
	float fCenterDepth = vSamples[KER_HALFSIZE];

	float fDepthWeighted = 0.0f;
	float fTotalWeight	 = 0.0f;

	float denormalize = 1.0f;
#ifdef ITF_DURANGO
	denormalize = 3500.0f;
#endif

	for (int i = -KER_HALFSIZE; i <= KER_HALFSIZE; ++i)
	{
		float fDepth = vSamples[ KER_HALFSIZE + i ];
		float fWeight = vWeights[ KER_HALFSIZE + i ];
		float fDepthDist = fCenterDepth - fDepth;
		float fFalloff = exp2( -BILATERAL_ATT * fDepthDist * denormalize * fDepthDist * denormalize );
		fWeight *= fFalloff;
		fDepthWeighted += fWeight * fDepth;
		fTotalWeight   += fWeight;
	}

	PS_OUT output;
	output.clr = fDepthWeighted / fTotalWeight;
	return output;
}

#undef KER_DIRECTION
#undef KER_HALFSIZE
#undef KER_WEIGHTS
#undef BILATERAL_ATT

#define KER_HALFSIZE		3			// Fixed kernel size
#define KER_DIRECTION		ps_reg0.xy
#define KER_WEIGHTS			ps_reg1

PS_OUT ps_filter_mask(VS_OUT input)
{
	// Construct weights
	float vWeights[ KER_HALFSIZE * 2 + 1 ] = { KER_WEIGHTS.w, KER_WEIGHTS.z, KER_WEIGHTS.y, KER_WEIGHTS.x, KER_WEIGHTS.y, KER_WEIGHTS.z, KER_WEIGHTS.w };

	// Take samples

	float vSamples[ KER_HALFSIZE * 2 + 1 ];
	for (int i = -KER_HALFSIZE; i <= KER_HALFSIZE; ++i)
	{
		float2 uvOffset = KER_DIRECTION * ps_samp0Size.zw * i;
		vSamples[ KER_HALFSIZE + i ] = TEXTURE_READ_2D( samp, 0, input.uv0 + uvOffset ).r;
	}

	// Average them taking edges into account
	
	float fMaskWeighted = 0.0f;
	float fTotalWeight	 = 0.0f;

	for (int i = -KER_HALFSIZE; i <= KER_HALFSIZE; ++i)
	{
		float3 fClr = vSamples[ KER_HALFSIZE + i ];
		float fWeight = vWeights[ KER_HALFSIZE + i ];
		fMaskWeighted += fWeight * fClr;
		fTotalWeight   += fWeight;
	}

	float fRes = fMaskWeighted / fTotalWeight;

	PS_OUT output;
	output.clr = fRes;
	return output;
}

#undef KER_DIRECTION
#undef KER_HALFSIZE
#undef KER_WEIGHTS

PS_OUT ps_refraction(VS_OUT input)
{ 
	float2 normal = TEXTURE_READ_2D( samp, 2, input.uv0 ).xy;
	normal = ( normal * 2.0f ) - 1.0f;

	float2 refractTexCoord = input.uv0 + ( normal.xy * ps_reg0.xy );
		
	float4 iceColor			= float4( TEXTURE_READ_2D( samp, 1, input.uv0 ).xyz * ps_reg1.xyz, 1.0f );
	float4 refractedColor	= TEXTURE_READ_2D( samp, 0, refractTexCoord );
	
	float4 cleanColor 		= TEXTURE_READ_2D( samp, 0, input.uv0 );
	float4 refractionResult = float4( lerp(refractedColor, iceColor, ps_reg0.z).xyz, cleanColor.w );
	
	PS_OUT output;
	output.clr = lerp( cleanColor, refractionResult, ps_reg0.w);
	return output;	
}

PS_OUT ps_alpha_gradient(VS_OUT input)
{
	float min = ps_reg0.x;
	float max =  ps_reg0.y;
	
	float4 vOrigColor = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float fAlpha = clamp(((1.0f-input.uv0.y) - min) / (max-min), 0.0f, 1.0f);
	
	PS_OUT output;
	output.clr = float4(vOrigColor.rgb, fAlpha * vOrigColor.a);
	return output;
}

PS_OUT ps_colored_shiva_alpha_blend( VS_OUT input )
{
	float4 oldImage = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float4 newImage = TEXTURE_READ_2D( samp, 1, input.uv0 );

	float  finalAlpha = newImage.w +( oldImage.w*( 1.0-newImage.w ) );
	float3 finalClr   =( newImage.xyz * newImage.w ) +( oldImage.xyz*oldImage.w*( 1.0-newImage.w ) );
	finalClr /= finalAlpha + 0.001f;

	if( newImage.w >= ps_reg0.x && newImage.w < ps_reg0.y )
	{
		finalClr = lerp( finalClr, ps_reg1.xyz, ps_reg1.w );
	}
	else if( newImage.w >= ps_reg0.y && newImage.w < ps_reg0.z )
	{
		finalClr = lerp( finalClr, ps_reg2.xyz, ps_reg2.w );
	}
	else if( newImage.w >= ps_reg0.z && newImage.w < ps_reg0.w )
	{
		finalClr = lerp( finalClr, ps_reg3.xyz, ps_reg3.w );
	} 
	
	PS_OUT output;
	output.clr = float4( finalClr, finalAlpha );
	return output;
}

float GetShivaAlpha(float2 uv)
{
	return TEXTURE_READ_2D( samp, 0, uv ).a;
}

float4 GetShivaColourFromAlpha(float alpha,float alphaDelta,float4 defaultCol)
{
	float  finalAlpha = alpha;//max(alpha,alphaDelta);
	float3 finalClr   = defaultCol * alpha;
	finalClr = lerp(ps_reg1.xyz,finalClr,finalAlpha);

	if( alpha >= ps_reg0.x && alpha < ps_reg0.y )
	{
		finalClr = lerp( finalClr, ps_reg1.xyz, ps_reg1.w );
	}
	else if( alpha >= ps_reg0.y && alpha < ps_reg0.z )
	{
		finalClr = lerp( finalClr, ps_reg2.xyz, ps_reg2.w );
	}
	else if( alpha >= ps_reg0.z && alpha < ps_reg0.w )
	{
		finalClr = lerp( finalClr, ps_reg3.xyz, ps_reg3.w );
	} 

    return float4(RGB2XYY(finalClr),alpha);
}

float4 GetShivaColour(float2 uv)
{
	float4 newImage = TEXTURE_READ_2D( samp, 0, uv );

	float  finalAlpha = newImage.w;
	float3 finalClr   = newImage.xyz * newImage.w;
	finalClr = lerp(ps_reg1.xyz,finalClr,finalAlpha);

	if( newImage.w >= ps_reg0.x && newImage.w < ps_reg0.y )
	{
		finalClr = lerp( finalClr, ps_reg1.xyz, ps_reg1.w );
		//finalAlpha = ps_reg1.w;
	}
	else if( newImage.w >= ps_reg0.y && newImage.w < ps_reg0.z )
	{
		finalClr = lerp( finalClr, ps_reg2.xyz, ps_reg2.w );
		//finalAlpha = ps_reg2.w;
	}
	else if( newImage.w >= ps_reg0.z && newImage.w < ps_reg0.w )
	{
		finalClr = lerp( finalClr, ps_reg3.xyz, ps_reg3.w );
		//finalAlpha = ps_reg3.w;
	} 

    return float4(RGB2XYY(finalClr),finalAlpha);
}

PS_OUT ps_colored_shiva( VS_OUT input )
{
    float ou = ddx(input.uv0.x) * 1.5f;
    float ov = ddy(input.uv0.y) * 1.5f;

	float3 baseColour = RGB2XYY(TEXTURE_READ_2D( samp, 0, input.uv0 ).rgb);
	float4 colcc = GetShivaColour(input.uv0);

	float alpha00 = GetShivaAlpha(input.uv0 + float2(-ou,0));
	float alpha10 = GetShivaAlpha(input.uv0 + float2(ou,0));
	float alpha11 = GetShivaAlpha(input.uv0 + float2(0,ov));
	float alpha01 = GetShivaAlpha(input.uv0 + float2(0,-ov));

    float alphaDelta    = max(abs(alpha01 - alpha00),abs(alpha11 - alpha10));
    float alphaSum      = alpha00 + alpha10 + alpha11 + alpha01 + colcc.a;
    float alphaMax      = max(max(max(max(alpha00,alpha10),alpha11),alpha01),colcc.a);

    float4 col00 = GetShivaColourFromAlpha(alpha00,alphaDelta,colcc);
	float4 col10 = GetShivaColourFromAlpha(alpha10,alphaDelta,colcc);
	float4 col11 = GetShivaColourFromAlpha(alpha11,alphaDelta,colcc);
	float4 col01 = GetShivaColourFromAlpha(alpha01,alphaDelta,colcc);
    
    float4 finalClr     = (col01 + col11 + col10 + col00 + colcc) * 0.2f;

    float blend         = saturate((finalClr.a - 0.85f) * 10.0f);

    finalClr.rgb        = XYY2RGB(lerp(finalClr.rgb,baseColour.rgb,blend));
    finalClr.a          = saturate((finalClr.a - alphaDelta) * 10.0f);

	PS_OUT output;
	output.clr = finalClr;

	return output;
}

// Saturation
PS_OUT ps_saturation( VS_OUT input )
{
	float4 sourceColor = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float3 sourceColorHSL = rgb_to_hsl( sourceColor );
	sourceColorHSL.y = clamp( sourceColorHSL.y + ps_reg0.x, 0.0f, 1.0f );	
	float3 modifiedColor = hsl_to_rgb( sourceColorHSL );
	
	PS_OUT output;
	output.clr = float4( modifiedColor, sourceColor.w );
	return output;
}

// Ghostbusters Slime Effect

// .. Get downward player edges
PS_OUT ps_slime_mask_p0(VS_OUT input)
{
    float clr_0 = saturate(TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0, -ps_samp0Size.w *ps_reg0.x) ).r);
    float clr_1 = saturate(TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0, -ps_samp0Size.w *ps_reg0.x*2 ) ).r);
    float clr_2 = saturate(TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0,  ps_samp0Size.w *ps_reg0.x ) ).r);
    float clr_3 = saturate(TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0,  ps_samp0Size.w *ps_reg0.x*2 ) ).r);

	float fRes = step( 1.5f, ( clr_2 + clr_3 - clr_0 - clr_1 ) );

	PS_OUT output;
	output.clr = float4( fRes, fRes, fRes, 1.0f );
	return output;
}

// .. Get upward normals
PS_OUT ps_slime_mask_p1(VS_OUT input)
{
	float3 normal = TEXTURE_READ_2D( samp, 0, input.uv0 ).rgb * 2.0f - 1.0f;

	float fRes = step( ps_reg0.y, normal.y );

	PS_OUT output;

	output.clr = float4( fRes, fRes, fRes, 1.0f );

	return output;
}

// .. Drip
PS_OUT ps_slime_mask_drip(VS_OUT input)
{
    float clr_c = saturate(TEXTURE_READ_2D( samp, 0, input.uv0 ).r);
    float clr_0 = saturate(TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0, -ps_samp0Size.w ) ).r);
    float clr_1 = saturate(TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0, -ps_samp0Size.w * 2 ) ).r);
    float clr_2 = saturate(TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0, -ps_samp0Size.w * 3 ) ).r);
    float clr_3 = saturate(TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0, -ps_samp0Size.w * 4 ) ).r);

	float2 uv = ( input.uv0 - float2( 0.0f, 0.75f ) ) / float2( 0.25f, 0.25f );
	float partIndex = TEXTURE_READ_2D( samp, 1, uv ).r * BODY_PART_MAX;

	float fRes = ceil( ( clr_c.x + clr_0 + clr_1 + clr_2 + clr_3 ) / 5.0f );

	// Do not drip if head
	fRes = lerp( fRes, clr_c, isPart(partIndex, BODY_PART_HEAD) );

	PS_OUT output;
	output.clr = float4( fRes, fRes, fRes, 1.0f );
	return output;
}

// .. Slime

#define SLIME_FACTOR		ps_reg0.x
#define SLIME_COLOR			ps_reg0.gba
#define NORMAL_TILING		ps_reg1.xy
#define LIGHT_ANGLE_X		ps_reg1.z
#define LIGHT_ANGLE_Z		ps_reg1.w
#define REFRACTION_AMOUNT	ps_reg2.x
#define REFRACTION_INDEX	ps_reg2.y
#define SPECULAR_AMOUNT		ps_reg2.z
#define SPECULAR_POWER		ps_reg2.w
#define SLIME_AMBIENT		ps_reg3.x
#define SLIME_OPACITY		ps_reg3.y

PS_OUT ps_slime(VS_OUT input)
{
	const float3 down	= float3(0.0f, -1.0f, 0.0f);
	const float3 toEye	= float3(0,0,-1);

	float3 normal			= TEXTURE_READ_2D( samp, 1, input.uv0 ).rgb * 2.0f - 1.0f;
	float3 normalDistortion	= TEXTURE_READ_2D( samp, 2, input.uv0 * NORMAL_TILING ).rbg * 2.0f - 1.0f;

	// Transform normal

	float3 tg		= cross(normal, down);
	float3 binormal = cross(normal, tg);
	tg				= cross(binormal, normal);

	float3x3 tgToWorld = float3x3( 
					float3(tg.x, tg.y, tg.z),
					float3(normal.x, normal.y, normal.z),
					float3(binormal.x, binormal.y, binormal.z ));

	float3 newNormal = normalize( mul( normalDistortion, tgToWorld ) );

	// Compute the reflection vector.
	float3 gLightVecW = float3( LIGHT_ANGLE_X, 0.0f, LIGHT_ANGLE_Z );
	gLightVecW = normalize(gLightVecW);
	float3 vReflect = reflect(gLightVecW, newNormal);

	// Specular light
	float spec = pow(max(dot(vReflect, toEye), 0.0f), SPECULAR_POWER);

	// Refraction
	float3 vRefract = refract(-toEye, newNormal, REFRACTION_INDEX) * REFRACTION_AMOUNT;
	float2 uvRefract = vRefract.xy;

	// Refracted color
	float4 clrSrc  = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float4 clrRefr = TEXTURE_READ_2D( samp, 0, input.uv0 + uvRefract );
	clrRefr.rgb = lerp( clrSrc.rgb, clrRefr.rgb, clrRefr.a );

	// Final color
	float3 vAmbient		= SLIME_COLOR * SLIME_AMBIENT;
	float3 vDiffuse		= SLIME_COLOR;// * dot(newNormal, -gLightVecW);
	float3 vTransmitted = clrRefr.rgb * SLIME_COLOR;
	float3 vSpecular	= (spec * SLIME_COLOR) * SPECULAR_AMOUNT;
	float3 finalClr		= vAmbient +
						  lerp( vTransmitted, vDiffuse, SLIME_OPACITY ) +
						  vSpecular;

	PS_OUT output;

	output.clr = float4( lerp( clrSrc.rgb, finalClr, SLIME_FACTOR ), clrSrc.a );

	return output;
}

#undef SLIME_FACTOR
#undef SLIME_COLOR
#undef NORMAL_TILING
#undef LIGHT_ANGLE_X
#undef LIGHT_ANGLE_Z
#undef REFRACTION_AMOUNT
#undef REFRACTION_INDEX
#undef SPECULAR_AMOUNT
#undef SPECULAR_POWER

PS_OUT ps_replace_color_pow_alpha( VS_OUT input )
{
	PS_OUT output;
	float alpha = TEXTURE_READ_2D( samp, 0, input.uv0 ).a;
	output.clr = float4(ps_reg0.xyz, pow(alpha, ps_reg0.w));
	
	return output;
}

PS_OUT ps_replace_tex_pow_alpha( VS_OUT input )
{
	PS_OUT output;
	float alpha = TEXTURE_READ_2D( samp, 0, input.uv0 ).a;
	float4 clr = TEXTURE_READ_2D( samp, 1, input.uv0 * ps_reg0.zz ).rgba;
	output.clr = float4(clr.rgb, clr.a * pow(alpha, ps_reg0.x) * ps_reg0.y );
	
	return output;
}

PS_OUT ps_overlay_grayscale_color( VS_OUT input )
{
	DEFINE_CONSTANTS;

	PS_OUT output;
		
	float4 color = float4(0.0f, 0.0f, 0.0f, 1.0f);
	
	float3 bcolor = ps_reg0.xyz;
	float3 acolor = TEXTURE_READ_2D( samp, 1, input.uv0 ).xyz;
	
	float4 sourceColor = TEXTURE_READ_2D( samp, 0, input.uv0 );
	
	float k = saturate(ceil(0.5f - acolor.r));
	
	color.rgb = lerp( (2.0f*acolor*bcolor), (vec3_one - 2.0f*(vec3_one - acolor)*( vec3_one - bcolor)), k);
	
#ifndef DX11_SHADERS
	//Revert gamma correction on 360
	color.rgb = pow(color.rgb, 2.2);
#endif

	output.clr = lerp(sourceColor, color, ps_reg0.w);
	
	return output;
}

PS_OUT ps_overlay_grayscale_color_2( VS_OUT input )
{
	PS_OUT output;
		
	float4 color = float4(0.0f, 0.0f, 0.0f, 1.0f);
	
	const float3 vec3_one = float3(1.0f,1.0f,1.0f);
	const float3 vec3_zero = float3(0.0f,0.0f,0.0f);
	
	float4 sourceColor = TEXTURE_READ_2D( samp, 0, input.uv0 );
	
	float l = 0;
    float2 MinMax;
    
    MinMax.x = min( sourceColor.r, min(sourceColor.g, sourceColor.b) );
    MinMax.y = max( sourceColor.r, max(sourceColor.g, sourceColor.b) );
	
	l = 0.5 * (MinMax.x + MinMax.y);
	
	float3 bcolor = ps_reg1.xyz;
	float3 acolor = float3(l,l,l);
		
	float k = saturate(ceil(0.5f - l));
	
	float3 overlayBelnded = lerp( (2.0f*acolor*bcolor), (vec3_one - 2.0f*(vec3_one - acolor)*( vec3_one - bcolor)), k);
	
#ifndef DX11_SHADERS
	//Revert gamma correction on 360
	// color.rgb = pow(color.rgb, 2.2);
#endif

	output.clr = float4(lerp(sourceColor.rgb, overlayBelnded, ps_reg1.w), sourceColor.a);
	
	return output;
}

PS_OUT ps_lerp_copy_as_is( VS_OUT input )
{
	PS_OUT output;
	output.clr = lerp( TEXTURE_READ_2D( samp, 0, input.uv0 ), TEXTURE_READ_2D( samp, 1, input.uv0 ), ps_reg0.x );
	return output;
}

PS_OUT ps_tint_mul_color( VS_OUT input )
{
	PS_OUT output;
	
	float4 color = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float3 colorTint = ps_reg1.xyz;
	
	float l = 0;
    float2 MinMax;
    
    MinMax.x = min( color.r, min(color.g, color.b) );
    MinMax.y = max( color.r, max(color.g, color.b) );
	
	l = 0.5 * (MinMax.x + MinMax.y);
	
	output.clr = lerp(color, float4( colorTint * l, color.a), ps_reg0.x );
	return output;
}

PS_OUT ps_particles( VS_Particle_OUT input )
{
	PS_OUT output;
    float2 texUV            = input.uv;
	float4 texCol			= TEXTURE_READ_2D( samp, 0, texUV );
    texCol.a                = dot(texCol.rgb,float3(0.299f, 0.587f, 0.114f));

	output.clr = input.uv2 * texCol;
	return output;
}

PS_OUT ps_lobbytoy_explode( VS_OUT input )
{
	PS_OUT output;
	
	output.clr = float4(0,0,0,0);
	
	float fPerc =  ps_reg0.x;

	float2 vDir = float2(0.5, 0.5) - input.uv0;

	float2 vExplode = vDir * fPerc;
	float2 vSrc = input.uv0 + vExplode;
	
	output.clr = TEXTURE_READ_2D(samp, 0, vSrc);;
		
	return output;
}

// ---------------------------------------Player Mask --------------------------------------

// Buffers info
//
//	Body info:		( Mask,				MaxDilation,			IgnoreHoles,	 MaxSize )
//	DilatedMask:	( Mask Dilated,		MaxDilation Dilated,	",				 Mask Original )
//	Shrink:			( Mask Shrinked,	",						Mask Dilated, 	 Mask Original )
//	Fill color:		( Mask Filling,		Color rgb for comparison )
//  Fill Holes:		( Mask Filling,		MaxDilation Dilated,	Mask Dilated,  	 Dilation Count )
//

// Body part info

#ifdef ITF_DURANGO
#define DILATION_MAX		4.0f
#define DILATION_HEAD       4.0f
#define DILATION_HANDS      2.0f
#define DILATION_FEET       2.0f
#else
#define DILATION_MAX		6.0f
#define DILATION_HEAD       6.0f
#define DILATION_HANDS      3.0f
#define DILATION_FEET       3.0f
#endif
#define DILATION_NORM		(1.0f / DILATION_MAX)

// Player Utility functions

#define GET_NEIGHBORHOOD_INFO \
	float4 mask_t  = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0, -ps_samp0Size.w ) ).xyzw; \
    float4 mask_l  = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z, 0.0 ) ).xyzw; \
	float4 mask_c  = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw; \
    float4 mask_r  = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( ps_samp0Size.z, 0.0 ) ).xyzw; \
    float4 mask_b  = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0,  ps_samp0Size.w ) ).xyzw; \
	float  isMask       = ceil(mask_c.x); \
	float4 neighbors_x	= float4( mask_l.x, mask_r.x, mask_t.x, mask_b.x ); \
	float4 neighbors_y	= float4( mask_l.y, mask_r.y, mask_t.y, mask_b.y ); \
	float4 neighbors_z	= float4( mask_l.z, mask_r.z, mask_t.z, mask_b.z ); \
	float4 neighbors_w	= float4( mask_l.w, mask_r.w, mask_t.w, mask_b.w );

#define GET_NEIGHBORHOOD_INFO_EX \
	float4 mask_tt = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0.0, -ps_samp0Size.w * 2 ) ).xyzw; \
	float4 mask_ll = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z * 2, 0.0 ) ).xyzw; \
	float4 mask_rr = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z * 2, 0.0 ) ).xyzw; \
	float4 mask_bb = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  0.0,  ps_samp0Size.w * 2 ) ).xyzw; \
	float4 neighborsEx_x	= float4( mask_ll.x, mask_rr.x, mask_tt.x, mask_bb.x ); \
	float4 neighborsEx_y	= float4( mask_ll.y, mask_rr.y, mask_tt.y, mask_bb.y ); \
	float4 neighborsEx_z	= float4( mask_ll.z, mask_rr.z, mask_tt.z, mask_bb.z ); \
	float4 neighborsEx_w	= float4( mask_ll.w, mask_rr.w, mask_tt.w, mask_bb.w );

inline float Neighborhood_SamePlayers( float mask_c, float4 mask_neighbors )
{
	float fPlayerIdxMax		= Max( mask_c, mask_neighbors );
	float fPlayersCount		= ceil(mask_c) + Sum( ceil(mask_neighbors) );
	float fPlayerIdxAvg		= ( mask_c + Sum( mask_neighbors ) ) / fPlayersCount;
	
	return step( abs( fPlayerIdxMax - fPlayerIdxAvg ), EPSILON );
}

inline float Neighborhood_CommonPlayerIdx( float mask_c, float4 mask_neighbors )
{
	float samePlayers   = Neighborhood_SamePlayers( mask_c, mask_neighbors );
	float fPlayerIdxMax	= Max( mask_c.x, mask_neighbors );
	
	return samePlayers * fPlayerIdxMax;
}

inline float Neighborhood_AllMask( float mask_c, float4 mask_neighbors )
{
    return ITF_ALL2( float2( mask_c, ITF_ALL2( mask_neighbors ) ) );
}

inline float Neighborhood_AnyMask( float mask_c, float4 mask_neighbors )
{
    return ITF_ANY2( float2( mask_c, ITF_ANY2( mask_neighbors ) ) );
}

float Neighborhood_SameRegions( float4 mask_neighbors, float4 mask_neighborsEx )
{
	float4 v1 = float4( mask_neighbors.x, mask_neighborsEx.x, mask_neighbors.y, mask_neighborsEx.y );
	float4 v2 = float4( mask_neighbors.z, mask_neighborsEx.z, mask_neighbors.w, mask_neighborsEx.w );

	float4 maskTexels1 = ceil(v1);
	float4 maskTexels2 = ceil(v2);

	float maskTexelsCount = Sum( maskTexels1 + maskTexels2 );
	float maskTexels      = Sum( v1 + v2 );

	float maskTexelsAvg = maskTexels / maskTexelsCount;

	float4 maskTexelsDiff1 = step( EPSILON, abs( v1 - maskTexels1 * maskTexelsAvg ) );
	float4 maskTexelsDiff2 = step( EPSILON, abs( v2 - maskTexels2 * maskTexelsAvg ) );
    
    return 1.0f - saturate( float( ITF_ANY4( maskTexelsDiff1 ) + ITF_ANY4( maskTexelsDiff2 ) ) );
}

// CMA (Cumulative Movement Average Buffer)

PS_OUT ps_cma_copy( VS_OUT input )
{
	PS_OUT output;
	output.clr.rgb = TEXTURE_READ_2D( samp, 0, input.uv0 ).rgb;
	output.clr.a = 1.0f; // Set pixel invalid
	return output;
}

PS_OUT ps_cma( VS_OUT input )
{
	PS_OUT output;

	output.clr			= TEXTURE_READ_2D( samp, 0, input.uv0 ).rgba;
	float3 clr			= TEXTURE_READ_2D( samp, 1, input.uv0 ).rgb;
#ifdef ITF_DURANGO
	float  playerMask	= ceil( TEXTURE_READ_2D( samp, 2, input.uv0 ).x );
#else
	float  playerMask	= ceil( TEXTURE_READ_2D( samp, 2, input.uv0 ).y );
#endif

	float3 cma = output.clr.rgb;

	// If the pixel has never been written (invalid), we will write the clr completely
	float bgFactor = max( ps_reg0.x, output.clr.a );

	// Accumulate only background
	output.clr.rgb = lerp( lerp( cma, clr, bgFactor ), cma, playerMask );

	// Set valid pixel
	output.clr.a = min( output.clr.a, playerMask );

	return output;
}

// Edge Diff

PS_OUT ps_edge_diff_intensity(VS_OUT input)
{
	DEFINE_CONSTANTS;

    float3 clr = TEXTURE_READ_2D( samp, 0, input.uv0 ).rgb;
	float4 cma = TEXTURE_READ_2D( samp, 1, input.uv0 ).rgba;
                
    float fClrI = dot( rgb_to_y, clr.rgb );
	float fCmaI = dot( rgb_to_y, cma.rgb );

    PS_OUT output;
    output.clr = float4( fClrI, fCmaI, cma.a, 1.0f );
    return output;
}

PS_OUT ps_edge_diff_blur(VS_OUT input)
{
	float2 fRes = 0;

	float3 img_c = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( 0, 0 ) ).xyz;

	fRes += TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z,  -ps_samp0Size.w ) ).xy * 0.07511f;
	fRes += TEXTURE_READ_2D( samp, 0, input.uv0 + float2(				0, -ps_samp0Size.w ) ).xy * 0.12384f;
	fRes += TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z,  -ps_samp0Size.w ) ).xy * 0.07511f;
	fRes += TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z,   0 ) ).xy * 0.12384f;
	fRes += img_c.xy * 0.20418f;
	fRes += TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z,   0 ) ).xy * 0.12384f;
	fRes += TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z,   ps_samp0Size.w ) ).xy * 0.07511f;
	fRes += TEXTURE_READ_2D( samp, 0, input.uv0 + float2(			   0,   ps_samp0Size.w ) ).xy * 0.12384f;
	fRes += TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z,   ps_samp0Size.w ) ).xy * 0.07511f;

    PS_OUT output;
    output.clr = float4(fRes, img_c.z, 1.0f);
    return output;
}

// Canny Edge detection

PS_OUT ps_edge_diff_canny_gradient( VS_OUT input )
{
	float2 img_00 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z, -ps_samp0Size.w ) ).xy;
	float2 img_01 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(				0, -ps_samp0Size.w ) ).xy;
	float2 img_02 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z, -ps_samp0Size.w ) ).xy;
	float2 img_10 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z,				 0 ) ).xy;
	float2 img_12 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z,				 0 ) ).xy;
	float2 img_20 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z,  ps_samp0Size.w ) ).xy;
	float2 img_21 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(				0,  ps_samp0Size.w ) ).xy;
	float2 img_22 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z,  ps_samp0Size.w ) ).xy;

    float2 fGradIntensityX = img_02 + 2.0f * img_12 + img_22 - img_00 - 2.0f * img_10 - img_20;
	float2 fGradIntensityY = img_00 + 2.0f * img_01 + img_02 - img_20 - 2.0f * img_21 - img_22;
	float2 fGradIntensity = abs(fGradIntensityX) + abs(fGradIntensityY);
	float2 fGradDir = atan2( fGradIntensityY, fGradIntensityX );

	// Invalid cma
	float valid = 1.0f - TEXTURE_READ_2D( samp, 0, input.uv0 ).z;
	fGradIntensity.y *= valid;
	fGradDir.y *= valid;

    PS_OUT output;
    output.clr = float4( fGradIntensity / 4.0f, (fGradDir / PI) * 0.5f + 0.5f );
    return output;
}

float2 IsDirection(float2 angle1, float2 angle2, float direction)
{
	const float DIRECTION_GROUP_SIZE   = 1.0f / 4.0f;
	const float DIRECTION_GROUP_OFFSET = 1.0f / 16.0f;
	float2 g1 = floor( frac(2.0 * (angle1 + DIRECTION_GROUP_OFFSET)) / DIRECTION_GROUP_SIZE );
	float2 g2 = floor( frac(2.0 * (angle2 + DIRECTION_GROUP_OFFSET)) / DIRECTION_GROUP_SIZE );
	return step( abs(g1 - g2), EPSILON ) * step( abs(g1 - direction), EPSILON );
}

float2 IsSameDirection(float2 angles)
{
	const float DIRECTION_GROUP_SIZE   = 1.0f / 4.0f;
	const float DIRECTION_GROUP_OFFSET = 1.0f / 16.0f;
	float2 g = floor( frac(2.0 * (angles + DIRECTION_GROUP_OFFSET)) / DIRECTION_GROUP_SIZE );
	return step( abs(g.x - g.y), EPSILON );
}

PS_OUT ps_edge_diff_canny_non_max_suppresion_and_diff( VS_OUT input )
{
	DEFINE_CONSTANTS;
	
	float4 img_00 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z, -ps_samp0Size.w ) ).xyzw;
	float4 img_01 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(				0, -ps_samp0Size.w ) ).xyzw;
	float4 img_02 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z, -ps_samp0Size.w ) ).xyzw;
	float4 img_10 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z,				 0 ) ).xyzw;
	float4 img_c  = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw;
	float4 img_12 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z,				 0 ) ).xyzw;
	float4 img_20 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z,  ps_samp0Size.w ) ).xyzw;
	float4 img_21 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(				0,  ps_samp0Size.w ) ).xyzw;
	float4 img_22 = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z,  ps_samp0Size.w ) ).xyzw;
	
	//float4 maskDilated = TEXTURE_READ_2D( samp, 1, input.uv0 ).xyzw;
	
	// Classify direction groups
	float2 g_00 = IsDirection(img_c.zw, img_00.zw, 3.0f);
	float2 g_01 = IsDirection(img_c.zw, img_01.zw, 2.0f);
	float2 g_02 = IsDirection(img_c.zw, img_02.zw, 1.0f);
	float2 g_10 = IsDirection(img_c.zw, img_10.zw, 0.0f);
	float2 g_12 = IsDirection(img_c.zw, img_12.zw, 0.0f);
	float2 g_20 = IsDirection(img_c.zw, img_20.zw, 1.0f);
	float2 g_21 = IsDirection(img_c.zw, img_21.zw, 2.0f);
	float2 g_22 = IsDirection(img_c.zw, img_22.zw, 3.0f);

	// Compare gradient direction with that of its neighbors
	float2 fIsEdge = 
		lerp( vec2_one, step( img_00.xy, img_c.xy ), g_00 ) *
		lerp( vec2_one, step( img_01.xy, img_c.xy ), g_01 ) *
		lerp( vec2_one, step( img_02.xy, img_c.xy ), g_02 ) *
		lerp( vec2_one, step( img_10.xy, img_c.xy ), g_10 ) *
		lerp( vec2_one, step( img_12.xy, img_c.xy ), g_12 ) *
		lerp( vec2_one, step( img_20.xy, img_c.xy ), g_20 ) *
		lerp( vec2_one, step( img_21.xy, img_c.xy ), g_21 ) *
		lerp( vec2_one, step( img_22.xy, img_c.xy ), g_22 );

	float2 fGradIntensity = fIsEdge * img_c.xy;

	// Apply threshold
	fGradIntensity = step( ps_reg0.x, fGradIntensity );

	// Output edges that are in foreground but not in background
	float fRes = saturate( fGradIntensity.x - fGradIntensity.y * IsSameDirection(img_c.zw) );

    PS_OUT output;
	output.clr = float4( fRes, fRes, fRes, 1.0f );
    return output;
}

// Short Edges Suppresion

PS_OUT ps_edge_diff_length(VS_OUT input)
{
	float4 img_tl = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z, -ps_samp0Size.w ) ) * ps_reg0.x;
	float4 img_t  = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(             0.0, -ps_samp0Size.w ) ) * ps_reg0.x;
    float4 img_tr = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z, -ps_samp0Size.w ) ) * ps_reg0.x;
	float4 img_l  = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z,             0.0 ) ) * ps_reg0.x;
	float4 img_c  = TEXTURE_READ_2D( samp, 0, input.uv0 ) * ps_reg0.x;
    float4 img_r  = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z,             0.0 ) ) * ps_reg0.x;
    float4 img_bl = TEXTURE_READ_2D( samp, 0, input.uv0 + float2( -ps_samp0Size.z,  ps_samp0Size.w ) ) * ps_reg0.x;
	float4 img_b  = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(             0.0,  ps_samp0Size.w ) ) * ps_reg0.x;
	float4 img_br = TEXTURE_READ_2D( samp, 0, input.uv0 + float2(  ps_samp0Size.z,  ps_samp0Size.w ) ) * ps_reg0.x;

	float fConnectTl		= (1.0f - saturate( img_t.x + img_l.x ));
	float fConnectTr		= (1.0f - saturate( img_t.x + img_r.x ));
	float fConnectBl		= (1.0f - saturate( img_b.x + img_l.x ));
	float fConnectBr		= (1.0f - saturate( img_b.x + img_r.x ));

	float fToReceive		= img_tl.y * fConnectTl;
	fToReceive			   += img_tr.y * fConnectTr;
	fToReceive			   += img_bl.y * fConnectBl;
	fToReceive			   += img_br.y * fConnectBr;
	fToReceive			   += img_t.y + img_l.y + img_r.y + img_b.y;
	fToReceive			   *= saturate(img_c.x);

	fToReceive			   -= img_c.w * ps_reg0.w; // Subtract what was given before and has come back

	float fConnectionsAcc   = img_c.x * ps_reg0.w + max(fToReceive, 0);

	fToReceive			   -= ps_reg0.z; // Subtract 1 only at initialization

	float fGiveConnections	= saturate(img_t.x);
	fGiveConnections	   += saturate(img_l.x);
	fGiveConnections	   += saturate(img_r.x);
	fGiveConnections	   += saturate(img_b.x);
	fGiveConnections	   += saturate(img_tl.x) * fConnectTl;
	fGiveConnections	   += saturate(img_tr.x) * fConnectTr;
	fGiveConnections	   += saturate(img_bl.x) * fConnectBl;
	fGiveConnections	   += saturate(img_br.x) * fConnectBr;
	fGiveConnections	   *= saturate(img_c.x);

	float fToGive			= (fGiveConnections - 1.0f * ps_reg0.w) * fToReceive; // Do not keep connection that will not come back

	float fToGivePrev		= img_c.z * ps_reg0.w;

    PS_OUT output;
    output.clr = float4(fConnectionsAcc * ps_reg0.y, fToReceive * ps_reg0.y, fToGive * ps_reg0.y, fToGivePrev * ps_reg0.y);
    return output;
}

PS_OUT ps_edge_diff_length_suppresion(VS_OUT input)
{
    float edge = TEXTURE_READ_2D( samp, 0, input.uv0 ).x * ps_reg0.x;
	
	float fRes = step( ps_reg0.y, edge );

    PS_OUT output;
    output.clr = float4(fRes, fRes, fRes, 1.0f);
    return output;
}

// Remove isolated pixels

PS_OUT ps_mask_remove_isolated_erode(VS_OUT input)
{
	// Get info
	GET_NEIGHBORHOOD_INFO;

	// Check if one of the neighbors is background
	float wantErode = 1.0f - Neighborhood_AllMask( 1, neighbors_x );

	// Check if can erode
	float canErode = isMask * wantErode;

	// Output
	float fMask = mask_c.x * (1.0f - canErode);
	
    PS_OUT output;
    output.clr = fMask;
    return output;
}

PS_OUT ps_mask_remove_isolated_dilate(VS_OUT input)
{
	// Get info
	GET_NEIGHBORHOOD_INFO;
	GET_NEIGHBORHOOD_INFO_EX;

	float fPlayerIdx = Neighborhood_CommonPlayerIdx( mask_c.x, neighbors_x );

	// Check if one of the neighbors is mask
    float wantDilate = Neighborhood_AnyMask( mask_c.x, neighbors_x );

	// Check if can dilate
	float canDilate = (1.0f - isMask) * wantDilate;

	// Output
	float fMask = lerp( mask_c.x, fPlayerIdx, canDilate );

    PS_OUT output;
    output.clr = fMask;
    return output;
}

// Body Part info

PS_OUT ps_mask_part_info_init(VS_OUT input)
{
	float mask = TEXTURE_READ_2D( samp, 0, input.uv0 ).r; // Player Input

    PS_OUT output;
    output.clr = float4( mask, 0.0f, 0.0f, 0.0f );
    return output;
}

PS_OUT ps_mask_part_info_mark(VS_OUT input)
{
	DEFINE_CONSTANTS;

	ITF_CONST float  kDilationHead          = DILATION_HEAD;
    ITF_CONST float  kDilationHands         = DILATION_HANDS;
    ITF_CONST float  kDilationFeet          = DILATION_FEET;
	ITF_CONST float4 kDilationLimb          = float4( kDilationHands.xx, kDilationFeet.xx );
    ITF_CONST float4 sizeLimbs              = ps_reg2.xyzw;
    ITF_CONST float  sizeHead               = ps_reg3.z;
	ITF_CONST float4 sizeFactorMarkLimbs    = float4( ps_reg3.ww, ps_reg4.xx );
	ITF_CONST float  sizeFactorMarkHead     = ps_reg4.yy;

	// Get info
	float mask = TEXTURE_READ_2D( samp, 0, input.uv0 ).r;

	// Calculate distances

    float4 fDirToLimbsX = float4( input.uv0.x - ps_reg0 );
    float4 fDirToLimbsY = float4( input.uv0.y - ps_reg1 );
    float4 fDistLimbs   = sqrt(fDirToLimbsX*fDirToLimbsX + fDirToLimbsY*fDirToLimbsY) - sizeLimbs * sizeFactorMarkLimbs;

	float  fDistHead	= length(input.uv0.xy - ps_reg3.xy) - sizeHead * sizeFactorMarkHead;

	fDistLimbs	= vec4_one - step( EPSILON, fDistLimbs );
	fDistHead	= 1.0f - step( EPSILON, fDistHead );

	// Calculate dilations
	float4 fDilationLimbs = kDilationLimb * fDistLimbs;
	float  fDilationHead  = kDilationHead * fDistHead;
	float  fDilation	  = Max( fDilationHead, fDilationLimbs );

	// Mark the size that the region needs to grow
	float4 fSizeLimbs = sizeLimbs * fDistLimbs;
	float  fSizeHead  = sizeHead * fDistHead;
	float  fSize	  = Max( fSizeHead, fSizeLimbs );

	// Mark if region needs to keep holes
	float fIgnoreHoles = step( EPSILON, fDistHead );

	// Check if texel belongs to current player
	float isPlayer = rectFunc( ps_reg4.z, ps_reg4.w, mask );

	// Output
	float3 outputValues = float3( fDilation * DILATION_NORM, fIgnoreHoles, fSize );
	outputValues *= isPlayer;

    PS_OUT output;
    output.clr = float4( 0.0f, outputValues );
    return output;
}

PS_OUT ps_mask_part_info_dilate(VS_OUT input)
{
	// Get info
	GET_NEIGHBORHOOD_INFO;

	// Do not dilate if different players
	float samePlayers = Neighborhood_SamePlayers( mask_c.x, neighbors_x );

	// Dilation
	float fDilation = Max( neighbors_y );
	float wantDilate = ceil(fDilation);

	// Ignore holes
    float fIgnoreHoles = ITF_ANY4( neighbors_z );

	// Size
	float fSize = Max( neighbors_w );
	fSize -= ps_samp0Size.z;
	float maxSizeNotReached = step( 0, fSize );

	// Check if can dilate
	float noDilationInfo = 1.0f - ceil(mask_c.y);
	float canDilate = (isMask * wantDilate * noDilationInfo * samePlayers * maxSizeNotReached);

	// Output
	float3 outputValues = float3( fDilation, fIgnoreHoles, fSize );
	outputValues = lerp( mask_c.yzw, outputValues, canDilate );

    PS_OUT output;
    output.clr = float4( mask_c.x, outputValues );
    return output;
}

// Dilate Background

PS_OUT ps_mask_dilated_init(VS_OUT input)
{
	float4 mask = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw;

    float fMaxDilation = ceil(mask.x) * max( mask.y, DILATION_NORM );

    PS_OUT output;
    output.clr = float4( mask.x, fMaxDilation, mask.zx ); // Mask, MaxDilation, IgnoreHoles, Mask Original
    return output;
}

PS_OUT ps_mask_dilated_dilate(VS_OUT input)
{
	DEFINE_CONSTANTS;

	// Get info

	GET_NEIGHBORHOOD_INFO;
	GET_NEIGHBORHOOD_INFO_EX;

	// Do not dilate if different players
	float samePlayers = Neighborhood_SamePlayers( mask_c.x, neighbors_x );

	// Check if we should keep the existent holes to the minimum width
	float fIgnoreHolesMarked = ITF_ANY4( neighbors_z );	

	float fIgnoreHoles = Neighborhood_SameRegions( neighbors_y, neighborsEx_y );

	if ( fIgnoreHolesMarked < 0.5f )
	{
		float4 v1 = float4( neighbors_x.xz, neighborsEx_x.xz );
		float4 v2 = float4( neighbors_x.yw, neighborsEx_x.yw );
		v1 = vec4_one - min( v1, v2 );
		float nonConnect = floor( v1.x * v1.y * v1.z * v1.w );
		fIgnoreHoles = min( nonConnect, fIgnoreHoles );
	}

	// Dilate if possible
	float fPlayerIdx = Max( mask_c.x, neighbors_x );
	float wantDilate = ceil( Sum( neighbors_x ) / 4.0f );
	float fRes		 = fPlayerIdx;

	// Ensure maximum dilation for that pixel is not reached
	float fMaxDilation = Max( neighbors_y );
	float fMaxDilationNotReached = step( ps_reg0.x, fMaxDilation * DILATION_MAX - 0.5f );

	// Check if can dilate
	float canDilate = (1.0f - isMask) * wantDilate * fIgnoreHoles * samePlayers * fMaxDilationNotReached;

	// Output
	float3 outputValues = float3( fRes, fMaxDilation, fIgnoreHolesMarked );
	outputValues = lerp( mask_c.xyz, outputValues, canDilate );

    PS_OUT output;
    output.clr = float4(outputValues, mask_c.w);
    return output;
}

// Mask Shrink

PS_OUT ps_mask_refine_region_shrink_init(VS_OUT input)
{
	float4 mask = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw;

    PS_OUT output;
    output.clr = mask.xyxw; // Mask Dilated, MaxDilation, Mask Dilated, Mask Original 
    return output;
}

PS_OUT ps_mask_refine_region_shrink(VS_OUT input)
{
	// Get info

	GET_NEIGHBORHOOD_INFO;

	float edge		 = TEXTURE_READ_2D( samp, 1, input.uv0 ).r;
	float playerMask = ceil(mask_c.w);
                
	// Check neigbhors
	float fAllMask = Neighborhood_AllMask( 1, neighbors_x );
    float fAnyMask = Neighborhood_AnyMask( 0, neighbors_x );

	// Stop if foreground
	float isBackground = 1.0f - max( edge, playerMask );

	// Check if needs shrinking
	float wantShrink = fAnyMask * (1.0f - fAllMask);

	// Max Dilation
	float fMaxShrinkNotReached = step( ps_reg0.x, mask_c.y * DILATION_MAX - 0.5f );

	// Check if can shrink
	float canShrink = isMask * wantShrink * isBackground * fMaxShrinkNotReached;

	// Output
	float fMask = mask_c.x * (1.0f - canShrink);

    PS_OUT output;
    output.clr = float4(fMask, mask_c.yzw);
    return output;
}

// Fill color

PS_OUT ps_mask_fill_color_init(VS_OUT input)
{
	float  mask = TEXTURE_READ_2D( samp, 0, input.uv0 ).r;
	float3 clr  = TEXTURE_READ_2D( samp, 1, input.uv0 ).rgb;

    PS_OUT output;
	output.clr = float4( mask, ceil(mask) * clr.rgb );
    return output;
}

PS_OUT ps_mask_fill_color_dilate(VS_OUT input)
{
	DEFINE_CONSTANTS;

	// Get info

	GET_NEIGHBORHOOD_INFO;

	float3 clr	   = TEXTURE_READ_2D( samp, 1, input.uv0 ).rgb;
	float4 cma	   = TEXTURE_READ_2D( samp, 2, input.uv0 ).rgba;

	// Check if dilating the pixel would connect two different players
	float fPlayerIdx = Neighborhood_CommonPlayerIdx( mask_c.x, neighbors_x );
	float fSamePlayers = ceil(fPlayerIdx);
	
	// Check if one of the neighbors is mask and its color is continuous
	
	float4 masks = ceil( neighbors_x );

	float4 colorDistance = float4(	colorDistance_avg( clr, mask_l.gba ),
									colorDistance_avg( clr, mask_r.gba ),
									colorDistance_avg( clr, mask_t.gba ),
									colorDistance_avg( clr, mask_b.gba ) );
	colorDistance += vec4_one - masks;
	colorDistance += step( ps_reg0.yyyy, colorDistance );

	float colorDistanceMin = Min( 0.95f, colorDistance );

	float4 weights = step( colorDistance, colorDistanceMin );
	float wantDilate = ITF_ANY4(weights);	

	float3 clrNeighbor = lerp(	max( max( mask_l.gba * weights.x, mask_r.gba * weights.y ),
									 max( mask_t.gba * weights.z, mask_b.gba * weights.w ) ),
								mask_c.gba,
								isMask );

	// Check if it's foreground, so that it can continue
	float isForeground = step( ps_reg0.z, colorDistance_avg( clr, cma.rgb ) + cma.a );

	// Check if can dilate
	float canDilate = (1.0f - isMask) * fSamePlayers * wantDilate * isForeground;

	// Output
	float fMask = lerp( mask_c.x, fPlayerIdx, canDilate );

	PS_OUT output;
	output.clr = float4( fMask, clrNeighbor );
    return output;
}

PS_OUT ps_mask_fill_color_combine(VS_OUT input)
{
	float4 clr0 = TEXTURE_READ_2D( samp, 0, input.uv0 ).rgba; // Shrink
	float4 clr1 = TEXTURE_READ_2D( samp, 1, input.uv0 ).rgba; // Fill color
	float  bg   = ceil(TEXTURE_READ_2D( samp, 2, input.uv0 ).r);

    PS_OUT output;

	output.clr = float4( max(clr0.x, clr1.x * bg), clr0.yzw );

    return output;
}

// Fill holes

PS_OUT ps_mask_fill_holes_init(VS_OUT input)
{
	float4 mask = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw;

    PS_OUT output;
    output.clr = float4( mask.xyz, 0.0f ); // Mask Shrinked, MaxDilation Dilated, Mask Dilated, Dilation Count
    return output;

}

PS_OUT ps_mask_fill_holes_dilate(VS_OUT input)
{
	// Get info
	GET_NEIGHBORHOOD_INFO;
#ifdef ITF_DURANGO
	GET_NEIGHBORHOOD_INFO_EX;
#endif

	// Check if dilating the pixel would connect two different players
	float fPlayerIdx = Neighborhood_CommonPlayerIdx( mask_c.x, neighbors_x );
	float fSamePlayers = ceil(fPlayerIdx);

	// Check collision with background mask of other player
	fSamePlayers *= Neighborhood_SamePlayers( mask_c.x, neighbors_z );
	
#ifdef ITF_DURANGO
	// Check collision with other regions of same player
	fSamePlayers *= Neighborhood_SameRegions( neighbors_y, neighborsEx_y );
#endif
	
	// Check if one of the neighbors is mask
    float wantDilate = Neighborhood_AnyMask( mask_c.x, neighbors_x );

	// Check if can dilate
	float canDilate = (1.0f - isMask) * fSamePlayers * wantDilate;

	// Output
	float2 outputValues = float2( fPlayerIdx, ps_reg0.x );
	outputValues = lerp( mask_c.xw, outputValues, canDilate );

    PS_OUT output;
    output.clr = float4( outputValues.x, mask_c.yz, outputValues.y );
    return output;
}

PS_OUT ps_mask_fill_holes_erode(VS_OUT input)
{
	// Get info
	GET_NEIGHBORHOOD_INFO;

	// Check if one of the neighbors is background
	float wantErode = 1.0f - Neighborhood_AllMask( 1, neighbors_x );

	// Check if is in same dilate iteration
	float fSameDilateIteration = step( ps_reg0.x, mask_c.w + ps_reg0.y );

	// Check if can erode
	float canErode = isMask * wantErode * fSameDilateIteration;

	// Output
	float fMask = mask_c.x * (1.0f - canErode);
	
    PS_OUT output;
    output.clr = float4( fMask, mask_c.yzw );
    return output;
}

PS_OUT ps_mask_fill_holes_final(VS_OUT input)
{
	// Get info
	float4 mask_c = TEXTURE_READ_2D( samp, 0, input.uv0 ).xyzw;

	// Output
	float fMask = mask_c.x * ceil(mask_c.z);
	
    PS_OUT output;
    output.clr = float4(fMask, mask_c.yzw);
    return output;
}

PS_OUT ps_mask_final_erode(VS_OUT input)
{
	// Get info
	GET_NEIGHBORHOOD_INFO;
	
	float originalMask = ceil( TEXTURE_READ_2D( samp, 1, input.uv0 ).w );

	// Check if one of the neighbors is background
	float wantErode = 1.0f - Neighborhood_AllMask( 1, neighbors_x );

    // Ensure we don't go beyond the original mask
    float keepOriginalMask = 1.0f - originalMask;

	// Check if can erode
	float canErode = isMask * wantErode * keepOriginalMask;

	// Output
	float fMask = mask_c.x * (1.0f - canErode);
	
    PS_OUT output;
    output.clr = float4( fMask, mask_c.yzw );
    return output;
}

PS_OUT ps_temp(VS_OUT input)
{
	float4 clr = TEXTURE_READ_2D( samp, 0, input.uv0 ).rgba;

    PS_OUT output;

    output.clr = ps_reg0 * lerp( clr.rgba, ceil(clr.rgba), ps_reg1 );
    output.clr.rgb += output.clr.a;
	output.clr.a = 1.0f;

	return output;
}

PS_OUT ps_plane_mask( VS_OUT input )
{
	PS_OUT output;
	float depth = TEXTURE_READ_2D( samp, 0, input.uv0 ).r;
	float hasDepth = ceil(saturate(depth)); //0 no depth 1 depth
	
	float3 rayOrig = NuiToWorld( float3(input.uv0, depth))/1000.0f; 	
	float3 normal = normalize(ps_reg1.xyz);
	float d =  ps_reg1.w;
	float t = clamp( dot( rayOrig, normal) + d - ps_reg0.z, 0.0f, 999.0f);
	
	float finalValue = saturate(1.0f - saturate(ceil(t)) + ( 1.0f - hasDepth));

	output.clr = float4( finalValue, finalValue, finalValue, finalValue);
	return output;
}
	
PS_OUT ps_blend_plane( VS_PLANE_OUT input )
{
    PS_OUT output;
	
	float3 ndc = (input.pospostvs/input.pospostvs.w+1.0f)/2.0f;
	float2 uvPlaneMask = float2(ndc.x, 1.0f - ndc.y);
	
	//Animate 
	//ps_reg1 -> x,y :water speed z:"wave" speed w: time	
	float2 offsetWave = ps_reg1.z * float2( sin( ps_reg1.w + ndc.x*10.0f), sin( ps_reg1.w + ndc.y*10.0f)); 
    float2 offsetLinear = ps_reg1.xy * ps_reg1.w; 
	float4 planeColor = TEXTURE_READ_2D(samp, 0, input.uv0 + offsetWave + offsetLinear);
	
	float planeMaskAlpha = TEXTURE_READ_2D( samp, 1, uvPlaneMask ).x;
	
	float planeTopPointScreen = lerp( ps_reg0.x, ps_reg0.y, ndc.x);
	float alpha = planeColor.w * (1.0f - ndc.y/planeTopPointScreen) * planeMaskAlpha * ps_reg0.z;
	
	output.clr = float4( planeColor.xyz, pow( alpha, 0.5f )); 
    return output;
}

PS_OUT ps_lighten_blend( VS_OUT input )
{
	PS_OUT output;
	
	float4 colorA = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float4 colorB = TEXTURE_READ_2D( samp, 1, input.uv0 );
	
	output.clr = lerp( colorA, float4(max(colorA.r, colorB.r), max(colorA.g, colorB.g), max(colorA.b, colorB.b), colorA.a ), pow(colorB.a, 0.5f));
	
	return output;
}

PS_OUT ps_screen_blend_inverse_alpha( VS_OUT input )
{
    PS_OUT output;

	const float3 vec3_one = float3(1.0f,1.0f,1.0f);
		
	float4 colorA = TEXTURE_READ_2D( samp, 0, input.uv0 );
	float4 colorB = TEXTURE_READ_2D( samp, 1, ((input.uv0 -0.5f) * ps_reg1.xy) + 0.5f + ps_reg1.zw );
	
	float3 screenBlendColor = (vec3_one - ( vec3_one - colorA.rgb ) * ( vec3_one - colorB.rgb ));
	output.clr = lerp( colorA, float4( lerp(screenBlendColor, colorA.rgb, ceil(colorB.a)), colorA.a), ps_reg0.x );

	return output;
}

PS_OUT ps_triple_layer_background ( VS_OUT input )
{
	PS_OUT output;
	
	ITF_CONST float factor	    = ps_reg0.x;
	ITF_CONST float speedX	    = ps_reg0.y;
	ITF_CONST float speedY	    = ps_reg0.z;
	ITF_CONST float time   	    = ps_reg0.w;
	ITF_CONST float3 tintColor  = ps_reg1.xyz;
	
	// Sample sources
	float3 firstLayerColor	= TEXTURE_READ_2D( samp, 0, input.uv0 );
	float3 secondLayerColor	= TEXTURE_READ_2D( samp, 1, input.uv0 );	
	float3 thirdLayerColor	= TEXTURE_READ_2D( samp, 2, input.uv0 + float2( speedX, speedY )*time );
	float3 videoColor		= TEXTURE_READ_2D( samp, 3, input.uv0 );  

	// Tint third layer color
	float3 tintedLayeredColor = thirdLayerColor * tintColor;
	
	// Compute final color from layers
	float3 finalLayeredColor = lerp( firstLayerColor, tintedLayeredColor, 1.0f - secondLayerColor.r );
	
	// Final color
	float3 finalColor = lerp( videoColor, finalLayeredColor, factor );
	
	output.clr = float4( finalColor, 1.0f );
	
	return output;
}

//note: uniformly distributed, normalized rand, [0;1[
float nrand( float2 n )
{
	return frac(sin(dot(n.xy, float2(12.9898, 78.233)))* 43758.5453);
}

PS_OUT ps_transition ( VS_OUT input )
{
	PS_OUT output;
	
	float3 oldImage = TEXTURE_READ_2D( samp, 0, input.uv0 );	
	float nrnd = nrand( input.uv0 + 0.07*ps_reg0.y );	    
   	
	output.clr = float4( ps_reg0.x*float3(nrnd,nrnd,nrnd) + (1. - ps_reg0.x)*oldImage, 1. );
	return output;
}

PS_OUT ps_transition2 ( VS_OUT input )
{
	PS_OUT output;
	
	float progress = ps_reg0.x;
	float time = ps_reg0.y;
	float maxColorModificationRatio = ps_reg0.z;
	float maxNoiseRatio = ps_reg0.w;
	
    float timeOffset = frac((time+pow(1.f+frac(time),5.f))/10.f);
			
	float offsetU = sin((input.uv0.y+timeOffset)*2.0f*PI)*0.05f;
	offsetU += sin((input.uv0.y*10.f+frac(timeOffset))*2.f*PI)*0.05f;
	offsetU += sin((input.uv0.y*5.f+frac(timeOffset*2.f))*2.f*PI)*0.05f;
	offsetU *= progress;
	
	float2 newUVCoord = input.uv0;
	newUVCoord.x = fmod(input.uv0.x + offsetU, 1.f);
	newUVCoord.y = fmod(newUVCoord.y + abs(sin(time/10.f))*sin(progress*3.f+time)*0.3f, 1.f);
	float3 oldImage = TEXTURE_READ_2D( samp, 0, newUVCoord );
	float colorCoef1 = abs(cos((newUVCoord.y+time)*4.f*PI));
	float colorCoef2 = abs(cos((newUVCoord.y+time*7.f)*8.f*PI));
	output.clr = float4(oldImage*(1.f-progress*maxColorModificationRatio) + progress*maxColorModificationRatio*float3(0.f,colorCoef1*colorCoef2*0.6f,1.f),1.f);
		
	float nrnd = nrand( input.uv0 + 0.07*frac(time) );	 
	output.clr = output.clr*(1.f-progress*maxNoiseRatio) + output.clr*progress*maxNoiseRatio*float4(nrnd,nrnd,nrnd,1.f);
	
	return output;
}

#endif // PIXEL_PROFILE

#endif //AUTODANCE__FX
