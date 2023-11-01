#ifndef MUSICMOTION__FX
#define MUSICMOTION__FX

#define CB_MUSICMOTION

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

REGISTER_SAMPLER(mumo_samp, 0);
REGISTER_SAMPLER(mumo_samp, 1);
REGISTER_SAMPLER(mumo_samp, 2);
REGISTER_SAMPLER(mumo_samp, 3);
REGISTER_SAMPLER(mumo_samp, 4);

struct VS_IN
{
	float4 pos : POSITION;
    float2 uv0 : TEXCOORD0;
};

struct VS_OUT
{
	float4 pos : VS_OUT_POS;
    float2 uv0 : TEXCOORD0;
};

struct PS_OUT
{
	float4 clr : PS_OUT_COLOR;
};

#ifdef VERTEX_PROFILE

VS_OUT vs_mumo( VS_IN input )
{
	VS_OUT output;
	output.pos = input.pos;
	output.uv0 = input.uv0;
	return output;
}
#endif

#ifdef PIXEL_PROFILE

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Utility functions

float4 tex2D_blurG3( float2 uv, float2 texelOffset )
{
	float2 off = texelOffset*ps_mumo_reg0.x;
	 
	float4 res = TEXTURE_READ_2D( mumo_samp, 0, uv + float2( -1*off.x,  -1*off.y ) ) *0.07511;
	res += TEXTURE_READ_2D( mumo_samp, 0, uv + float2( 0*off.x,  -1*off.y ) ) *0.12384;
	res += TEXTURE_READ_2D( mumo_samp, 0, uv + float2( 1*off.x,  -1*off.y ) ) *0.07511;

	res += TEXTURE_READ_2D( mumo_samp, 0, uv + float2( -1*off.x,   0*off.y ) ) *0.12384;
	res += TEXTURE_READ_2D( mumo_samp, 0, uv + float2( 0*off.x,   0*off.y ) ) *0.20418;
	res += TEXTURE_READ_2D( mumo_samp, 0, uv + float2( 1*off.x,   0*off.y ) ) *0.12384;

	res += TEXTURE_READ_2D( mumo_samp, 0, uv + float2( -1*off.x,   1*off.y ) ) *0.07511;
	res += TEXTURE_READ_2D( mumo_samp, 0, uv + float2( 0*off.x,   1*off.y ) ) *0.12384;
	res += TEXTURE_READ_2D( mumo_samp, 0, uv + float2( 1*off.x,   1*off.y ) ) *0.07511;

	return res;
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Pixel Shaders( filters )

PS_OUT ps_mumo_blur_g3( VS_OUT input )
{
	PS_OUT output;
	output.clr = tex2D_blurG3( input.uv0, ps_mumo_samp0Size.zw );
	return output;
}

PS_OUT ps_mumo_copy_as_is( VS_OUT input )
{
	PS_OUT output;
	output.clr = TEXTURE_READ_2D( mumo_samp, 0, input.uv0 ).xyzw;
	return output;
}

PS_OUT ps_mumo_silhouette( VS_OUT input )
{
	float4 InputColour = TEXTURE_READ_2D( mumo_samp, 0, input.uv0 ).rgba;
	float3 ColourFactors = { 0.3333, 0.3333, 0.3333 };
	float fLum = dot( ColourFactors, InputColour.xyz );
	float Middle = ps_mumo_reg0.x;
	float StepSize = ps_mumo_reg0.y;
	float Scaler = ps_mumo_reg0.z;
	float TargetLum = ps_mumo_reg0.w;
	float Modulator = smoothstep( 0.0, 1.0, saturate( ( fLum - Middle )*Scaler ) )/(fLum + 0.001 )*TargetLum;

    PS_OUT output;
    output.clr = InputColour;
    output.clr = float4( InputColour.xyz*Modulator, InputColour.a );
    return output;
}

PS_OUT ps_mumo_mul_color( VS_OUT input )
{
	PS_OUT output;

	output.clr = TEXTURE_READ_2D( mumo_samp, 0, input.uv0 ).xyzw * ps_mumo_reg0;
	return output;
}

float4 ps_mumo_sobel( VS_OUT In ) : PS_OUT_COLOR
{
	float edgeWidth = ps_mumo_reg0.x;
	float2 offset = ps_mumo_samp0Size.zw*edgeWidth;
  	float OffsetX = offset.x;
	float OffsetY = offset.y;
	
	float4 s00 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2( -OffsetX, -OffsetY ) ) );
	float4 s01 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2(      0.0, -OffsetY ) ) );
	float4 s02 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2(  OffsetX, -OffsetY ) ) );

	float4 s10 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2( -OffsetX,      0.0 ) ) );
	float4 s12 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2(  OffsetX,      0.0 ) ) );

	float4 s20 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2( -OffsetX,  OffsetY ) ) );
	float4 s21 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2(      0.0,  OffsetY ) ) );
	float4 s22 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2(  OffsetX,  OffsetY ) ) );
	
	// Calc X gradient
	float4 GradX = s00 + 2.0*s10 + s20 - ( s02 + 2.0*s12 + s22 );
	float4 GradY = s00 + 2.0*s01 + s02 - ( s20 + 2.0*s21 + s22 );
    float asum = max(max(max(s00.a,s01.a),max(s02.a,s10.a)),max(max(s12.a,s20.a),max(s21.a,s22.a)));
	
	float edgeColourMultiplier = ps_mumo_reg0.y;
	float4 SquareGrad = GradX*GradX + GradY*GradY;
	float4 FragCol = sqrt( SquareGrad )*edgeColourMultiplier;
    FragCol.rgb *= asum;

    return FragCol;
}

float4 ps_mumo_threshold( VS_OUT In ) : PS_OUT_COLOR
{
	float3 ColourFactors = { 0.3333, 0.3333, 0.3333 };
	float fThreshold = ps_mumo_reg0.x;
	float fMultiplier = ps_mumo_reg0.y;
	
	float4 InColour = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).xyzw;
	float fLum = dot( ColourFactors, InColour.xyz ) + 0.001;
	float fInvLum = 1.0/fLum;
	float3 NormColour = InColour.xyz*fInvLum;
	
	float4 OutColour = float4( saturate( ( InColour.xyz - fThreshold*NormColour )*fMultiplier ), InColour.a );
		
	return OutColour;
}

float4 ps_mumo_fakealpha( VS_OUT In ) : PS_OUT_COLOR
{
	float3 ColourFactors = { 0.3333, 0.3333, 0.3333 };
	float fThreshold = 0.3f;
	float fMultiplier = 1.0f;
	
	float4 InColour = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).xyzw;
	float fLum = dot( ColourFactors, InColour.xyz );

	float4 OutColour = InColour;//float4( InColour.xyz, fLum );
		
	return OutColour;
}

float4 ps_mumo_gaussian_9tap_x( VS_OUT In ) : PS_OUT_COLOR
{
	float offsets[3] = { 0.0, 1.384615385, 3.230769231 };
	float weights[3] = { 0.227027027, 0.316216216, 0.07027027 };

	float4 OutColour = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).xyzw * weights[0];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( offsets[1]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( offsets[1]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( offsets[2]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[2];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( offsets[2]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[2];
		
	return OutColour;
}

float4 ps_mumo_gaussian_9tap_y( VS_OUT In ) : PS_OUT_COLOR
{
	float offsets[3] = { 0.0, 1.384615385, 3.230769231 };
	float weights[3] = { 0.227027027, 0.316216216, 0.07027027 };

	float4 OutColour = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).xyzw * weights[0];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( 0.0, offsets[1]*ps_mumo_samp0Size.w ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( 0.0, offsets[1]*ps_mumo_samp0Size.w ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( 0.0, offsets[2]*ps_mumo_samp0Size.w ) ).xyzw * weights[2];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( 0.0, offsets[2]*ps_mumo_samp0Size.w ) ).xyzw * weights[2];
		
	return OutColour;
}

float4 ps_mumo_gaussian_13tap_x( VS_OUT In ) : PS_OUT_COLOR
{
	float offsets[4] = { 0.0, 1.454545455, 3.393939394, 5.33333333 };
	float weights[4] = { 0.140245, 0.241991001, 0.133732, 0.045156};
	
	float4 OutColour = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).xyzw * weights[0];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( offsets[1]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( offsets[1]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( offsets[2]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[2];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( offsets[2]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[2];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( offsets[3]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[3];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( offsets[3]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[3];
		
	return OutColour;
}

float4 ps_mumo_gaussian_13tap_y( VS_OUT In ) : PS_OUT_COLOR
{
	float offsets[4] = { 0.0, 1.454545455, 3.393939394, 5.33333333 };
	float weights[4] = { 0.140245, 0.241991001, 0.133732, 0.045156};

	float4 OutColour = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).xyzw * weights[0];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( 0.0, offsets[1]*ps_mumo_samp0Size.w ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( 0.0, offsets[1]*ps_mumo_samp0Size.w ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( 0.0, offsets[2]*ps_mumo_samp0Size.w ) ).xyzw * weights[2];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( 0.0, offsets[2]*ps_mumo_samp0Size.w ) ).xyzw * weights[2];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( 0.0, offsets[3]*ps_mumo_samp0Size.w ) ).xyzw * weights[3];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( 0.0, offsets[3]*ps_mumo_samp0Size.w ) ).xyzw * weights[3];
		
	return OutColour;
}

float4 ps_mumo_gaussian_17tap_x( VS_OUT In ) : PS_OUT_COLOR
{
	float offsets[5] = { 0.0, 1.47826087, 3.449275, 5.42029, 7.391304 };
	float weights[5] = { 0.09741, 0.181367477, 0.136219, 0.081208, 0.038293 };
	
	float4 OutColour = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).xyzw * weights[0];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( offsets[1]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( offsets[1]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( offsets[2]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[2];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( offsets[2]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[2];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( offsets[3]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[3];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( offsets[3]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[3];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( offsets[4]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[4];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( offsets[4]*ps_mumo_samp0Size.z, 0.0 ) ).xyzw * weights[4];
		
	return OutColour;
}

float4 ps_mumo_gaussian_17tap_y( VS_OUT In ) : PS_OUT_COLOR
{
	float offsets[5] = { 0.0, 1.47826087, 3.449275, 5.42029, 7.391304 };
	float weights[5] = { 0.09741, 0.181367477, 0.136219, 0.081208, 0.038293 };

	float4 OutColour = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).xyzw * weights[0];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( 0.0, offsets[1]*ps_mumo_samp0Size.w ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( 0.0, offsets[1]*ps_mumo_samp0Size.w ) ).xyzw * weights[1];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( 0.0, offsets[2]*ps_mumo_samp0Size.w ) ).xyzw * weights[2];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( 0.0, offsets[2]*ps_mumo_samp0Size.w ) ).xyzw * weights[2];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( 0.0, offsets[3]*ps_mumo_samp0Size.w ) ).xyzw * weights[3];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( 0.0, offsets[3]*ps_mumo_samp0Size.w ) ).xyzw * weights[3];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( 0.0, offsets[4]*ps_mumo_samp0Size.w ) ).xyzw * weights[4];
	OutColour += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - float2( 0.0, offsets[4]*ps_mumo_samp0Size.w ) ).xyzw * weights[4];
		
	return OutColour;
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

float4 ps_mumo_median3x3( VS_OUT In ) : PS_OUT_COLOR
{
	float4 v[9], temp;

	v[0] = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( -ps_mumo_samp0Size.z, -ps_mumo_samp0Size.w ) ).xyzw;
	v[1] = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2(             0.0, -ps_mumo_samp0Size.w ) ).xyzw;
	v[2] = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2(  ps_mumo_samp0Size.z, -ps_mumo_samp0Size.w ) ).xyzw;
	v[3] = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( -ps_mumo_samp0Size.z,             0.0 ) ).xyzw;
	v[4] = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2(             0.0,             0.0 ) ).xyzw;
	v[5] = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2(  ps_mumo_samp0Size.z,             0.0 ) ).xyzw;
	v[6] = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2( -ps_mumo_samp0Size.z,  ps_mumo_samp0Size.w ) ).xyzw;
	v[7] = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2(             0.0,  ps_mumo_samp0Size.w ) ).xyzw;
	v[8] = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + float2(  ps_mumo_samp0Size.z,  ps_mumo_samp0Size.w ) ).xyzw;
	
	// Starting with a subset of size 6, remove the min and max each time
	mnmx6(v[0], v[1], v[2], v[3], v[4], v[5]);
	mnmx5(v[1], v[2], v[3], v[4], v[6]);
	mnmx4(v[2], v[3], v[4], v[7]);
	mnmx3(v[3], v[4], v[8]);
  
	return v[4];
}

// --------------------------------------------------------------------
// Procedural Half tone

float4 ps_mumo_halftone( VS_OUT In ) : PS_OUT_COLOR
{
	float4 playerMask = TEXTURE_READ_2D( mumo_samp, 1, In.uv0 ).rgba;
	float alphaval = dot( playerMask.rgb, float3( 1, 1, 1 ) );
	
#if defined( ITF_DURANGO )
	// extract the shader scalar parameters
	float cellsize 			= ps_mumo_reg1.x;	// 12.0;
	float cellscale 		= ps_mumo_reg1.y;	// 0.3;
	float circlefalloff 	= ps_mumo_reg1.z;	// 15.0f;
	float farIntensity 		= ps_mumo_reg1.w;	// 0.02f;
	float minIntensity 		= ps_mumo_reg2.x;	// 0.3f;
	float mindepth 			= ps_mumo_reg2.y;	// 100.0;
	float maxdepth 			= ps_mumo_reg2.z;	// 4000.0f;
	float overbloomFactor	= ps_mumo_reg2.w;
	
	// Calculate the sub-cell UV coordinate in the rotated half-tone grid ( UV => [-cellsize/2, -cellsize/2] -> [cellsize/2, cellsize/2] )
	float2 screenCoord = In.pos.xy;
	float2x2 matRot = ps_mumo_reg0;
	float2 rotUV = mul(screenCoord, matRot ) + ps_viewportDimensions.xy;
	float2 subUV = ( rotUV % cellsize ) - float2( cellsize/2, cellsize/2 );
	
	// The IR texture feed is subject to inverse square intensity falloff due to the point IR light source
	// Correct the per-pixel intensity using the depth value
	// To prevent very small half-tone dots we clamp the minimum intensity 
	float depth = TEXTURE_READ_2D( mumo_samp, 2, In.uv0 ).x*65535.0;
	float lerpval = saturate( ( depth - mindepth )/( maxdepth - mindepth ) );
	lerpval = ( lerpval*lerpval*( 1.0 - farIntensity ) + farIntensity )*( 1.0/farIntensity );
	float inputIntensity = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).r;
	inputIntensity *= lerpval;
	inputIntensity = max( minIntensity, inputIntensity );

	// Calculate the squared distance to the centre of the half-tone cell from the current position to determine if the current pixel lies within the halftone dot
	// At the boundary edge of the half-tone dot some falloff is applied to avoid aliasing
	float distancetocellcentre = dot( subUV, subUV ) * ( 1.0/( cellsize*cellsize*cellscale ) );
	float val = saturate(( inputIntensity - distancetocellcentre )*circlefalloff/overbloomFactor)*overbloomFactor;

	// Output colour is modulated with the player mask to get the final coloured output

	float4 shivaColour = TEXTURE_READ_2D( mumo_samp, 3, In.uv0 );
	float shivaAlpha = dot( shivaColour.rgb, float3( 1.0, 1.0, 1.0 ) );
	float4 blendColour = lerp( float4( shivaColour.rgb, shivaAlpha ), float4( playerMask.rgb*val, alphaval ), alphaval );

	float4 outputColour = blendColour;
	
#else // defined( ITF_DURANGO )
	
	float4 outputColour = float4( playerMask.rgb, alphaval );

#endif // defined( ITF_DURANGO )

	return outputColour;
}

// --------------------------------------------------------------------
// ps_mumo_pip

float4 ps_mumo_pip( VS_OUT In ) : PS_OUT_COLOR
{
	float4 playerMask = TEXTURE_READ_2D( mumo_samp, 1, In.uv0 ).rgba;
	float alphaval = dot( playerMask.rgb, float3( 1, 1, 1 ) );
	
#if defined( ITF_DURANGO )
	// extract the shader scalar parameters
	float farIntensity 		= ps_mumo_reg1.w;	// 0.02f;
	float minIntensity 		= ps_mumo_reg2.x;	// 0.3f;
	float mindepth 			= ps_mumo_reg2.y;	// 100.0;
	float maxdepth 			= ps_mumo_reg2.z;	// 4000.0f;
		
	// The IR texture feed is subject to inverse square intensity falloff due to the point IR light source
	// Correct the per-pixel intensity using the depth value
	// To prevent very small half-tone dots we clamp the minimum intensity 
	float depth = TEXTURE_READ_2D( mumo_samp, 2, In.uv0 ).x*65535.0;
	float depthcomp = saturate( ( depth - mindepth )/( maxdepth - mindepth ) );
	depthcomp = ( depthcomp*depthcomp*( 1.0 - farIntensity ) + farIntensity )*( 1.0/farIntensity );
	float IRIntensity = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).r;
	
	if(IRIntensity.r < 0.5)
	{
		if(depth < 0.45)
		{
			//alphaval = 1.0;
		}
	}
	
	//IRIntensity += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + 0.0075F ).r;
	//IRIntensity += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + 0.005F ).r;
	//IRIntensity += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - 0.0075F ).r;
	//IRIntensity += TEXTURE_READ_2D( mumo_samp, 0, In.uv0 - 0.005F ).r;	
	//IRIntensity /= 5.0;
	//diffuse +=tex2D(DiffuseSampler, data.UV0 + 0.0075F); 
	//diffuse +=tex2D(DiffuseSampler, data.UV0 + 0.005F); 
    //diffuse +=tex2D(DiffuseSampler, data.UV0 - 0.0075F); 
	//diffuse +=tex2D(DiffuseSampler, data.UV0 - 0.005F); 	
	
	IRIntensity *= depthcomp;
	IRIntensity = max( minIntensity, IRIntensity );
	
	// Modulate the player mask with the gradient colour texture in sampler 4
	playerMask.rgb *= TEXTURE_READ_2D( mumo_samp, 4, In.uv0 );

	float IRValue = IRIntensity*0.12;
	float4 Color = float4( IRValue, IRValue, IRValue, alphaval );
	
	// EDGE DETECTION
	float edgeWidth = 2.4;//5.0;//ps_mumo_reg0.x;
	float2 offset = ps_mumo_samp0Size.zw*edgeWidth;
  	float OffsetX = offset.x;
	float OffsetY = offset.y;
	
	float4 s00 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2( -OffsetX, -OffsetY ) ) );
	float4 s01 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2(      0.0, -OffsetY ) ) );
	float4 s02 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2(  OffsetX, -OffsetY ) ) );

	float4 s10 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2( -OffsetX,      0.0 ) ) );
	float4 s12 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2(  OffsetX,      0.0 ) ) );

	float4 s20 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2( -OffsetX,  OffsetY ) ) );
	float4 s21 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2(      0.0,  OffsetY ) ) );
	float4 s22 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 + ( float2(  OffsetX,  OffsetY ) ) );
	
	// Calc X gradient
	float4 GradX = s00 + 2.0*s10 + s20 - ( s02 + 2.0*s12 + s22 );
	float4 GradY = s00 + 2.0*s01 + s02 - ( s20 + 2.0*s21 + s22 );
	
	float edgeColourMultiplier = 0.15*depthcomp;//ps_mumo_reg0.y;
	float4 SquareGrad = GradX*GradX + GradY*GradY;
	float4 FragCol = sqrt( SquareGrad )*edgeColourMultiplier;

	FragCol.g = FragCol.r;
	FragCol.b = FragCol.r * 2.8;
	if(( FragCol.r + FragCol.g + FragCol.b ) > 1.9)
	{
		FragCol *= 1.8; 
	}
	else
	{
		FragCol /= 2.8;
	}
	FragCol.a = alphaval;

	FragCol.r = FragCol.r * 0.5 + playerMask.r * 2.8;
	FragCol.g = FragCol.g * 0.5 + playerMask.g * 2.8;	
	FragCol.b = FragCol.b + playerMask.b * 1.8;		

	
////////////////////////////////////////////////////////////////////////////////////////
	float2 newUV = In.uv0;
	float4 outputColour = FragCol;
	for(int x = -4; x < 0; x++)
	{
		newUV.x = newUV.x + x * 0.0005F;//11F;
		outputColour += float4(TEXTURE_READ_2D( mumo_samp, 1, newUV ).rgba);
	}

	newUV = In.uv0;
	for(int x = 0; x < 4; x++)
	{
		newUV.x = newUV.x + x * 0.0005F;//11F;
		outputColour += float4(TEXTURE_READ_2D( mumo_samp, 1, newUV ).rgba);
	}

	newUV = In.uv0;
	for(int y = 0; y < 4; y++)
	{
		newUV.y = newUV.y + y * 0.0005F;//11F;
		outputColour += float4(TEXTURE_READ_2D( mumo_samp, 1, newUV ).rgba);
	}

	newUV = In.uv0;
	for(int y = -4; y < 0; y++)
	{
		newUV.y = newUV.y + y * 0.0005F;//11F;
		outputColour += float4(TEXTURE_READ_2D( mumo_samp, 1, newUV ).rgba);
	}


	if(FragCol.a > 0.1f)
		outputColour = FragCol;
	else
	{
		if(outputColour.a != 0)
		{
			outputColour.r = 0.7f;
			outputColour.g = 0.7f;
			outputColour.b = 0.7f;
			// Stencil Version
			outputColour.a = outputColour.a;
			// Gradient Version
			//outputColour.a = outputColour.a / 4;

		}
	}
		//clip( ret.Color.a < 0.001f ? -1:1 );
		
	/////////////////////////////////////////				
//	float4 shivaColour = TEXTURE_READ_2D( mumo_samp, 3, In.uv0 );
//	float shivaAlpha = dot( shivaColour.rgb, float3( 1.0, 1.0, 1.0 ) );
//	if(shivaColour.r > 0 && shivaColour.r > shivaColour.g  && shivaColour.r > shivaColour.b)
//	{
//		shivaColour = float4(1.0,0,0,shivaAlpha);
//	}
//	else if(shivaColour.g > 0 && shivaColour.g > shivaColour.r  && shivaColour.g > shivaColour.b)
//	{
//		shivaColour = float4(0,1.0,0,shivaAlpha);
//	}
//	else if(shivaColour.b > 0 && shivaColour.b > shivaColour.r  && shivaColour.b > shivaColour.g)
//	{
//		shivaColour = float4(0,0,1.0,shivaAlpha);
//	}
//	shivaColour = float4(shivaColour.r * 0.7 + playerMask.r * 0.3, shivaColour.g * 0.7 + playerMask.g * 0.3, shivaColour.b * 0.7 + playerMask.b * 0.3, shivaColour.a );

//	float4 blendColour = lerp( float4( shivaColour.rgb, shivaAlpha ), FragCol, alphaval );

	////////////////////////////////////////////
	float4 blurColour = TEXTURE_READ_2D( mumo_samp, 3, In.uv0 ) * ps_mumo_reg0.a;
	float4 pipColour = float4( (outputColour.rgb * outputColour.a), outputColour.a );
	outputColour = float4( pipColour.rgb + (blurColour.rgb * (1-pipColour.a)), 1 - ( (1-blurColour.a) * (1-pipColour.a) ) );

#else // defined( ITF_DURANGO )
	
	float4 outputColour = float4( playerMask.rgb, alphaval );

#endif // defined( ITF_DURANGO )

	return outputColour;
}

// --------------------------------------------------------------------
// Combine lobby pips

float4 ps_mumo_lobby_composite( VS_OUT In ) : PS_OUT_COLOR
{
	float4 colour1 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).xyzw;
	float4 colour2 = TEXTURE_READ_2D( mumo_samp, 1, In.uv0 ).xyzw;
	float3 ColourFactors = { 0.5, 0.5, 0.5 };

	colour1.w = saturate( dot( colour1.rgb, ColourFactors ) );
	colour2.w = saturate( dot( colour2.rgb, ColourFactors ) )*0.9f;
	colour2.rgb = float3( 0.0, 0.0, 0.0 );

	float4 outColour = saturate( colour1 + colour2 );
	return outColour;
}

// --------------------------------------------------------------------

#if defined( ITF_DURANGO ) 
float randNum( float2 p )
{
  const float2 r = float2(23.1406926327792690,2.6651441426902251);

  return frac( cos( 123456789.0 % (1e-7 + 256.0 * dot(p,r)) ) );  
}
#endif

float4 ps_mumo_particle( VS_OUT In ) : PS_OUT_COLOR
{
    const float TWO_PI = 6.283185f;
    const float particleLifeSpan = 4.0f;

	float3 ColourFactors = { 0.5, 0.5, 0.5 };
   	float time = ps_mumo_reg0.x;
    float xoffs = sin(time * 0.3f) * 0.1f;
#if defined( ITF_CAFE ) || defined( ITF_NX )
    const float flat   = 0.0f;
#else 
    float flat   = ps_mumo_reg0.w;
#endif

    float2 src1          = float2(0.5f+xoffs,1.0f);
    float2 src2          = float2(0.5f-xoffs,1.0f);
    float2 sline1        = In.uv0 - src1;
    float2 sline2        = In.uv0 - src2;
    float2 tdist         = float2(dot(sline1,sline1),dot(sline2,sline2));

    
    tdist   *= tdist;
    tdist   *= float2(25.0f * TWO_PI,21.0f * TWO_PI);
    tdist   -= float2(time * 6.0f,time * 4.0f);

    float2 tscale = lerp(sin(tdist),float2(1.0f,1.0f),(flat * 0.7f));

    float yscale = lerp(In.uv0.y,1.0f,flat * 0.6f);
    float scale  = dot(tscale,float2(1,0.5f));
    scale        += 1.0f;
    scale        *= yscale * 0.2f;

  	float4 srcColour1 = TEXTURE_READ_2D( mumo_samp, 0, In.uv0 ).xyzw;
	float4 srcColour2 = TEXTURE_READ_2D( mumo_samp, 1, In.uv0 ).xyzw;
  	float4 colour1 = srcColour1;
	float4 colour2 = srcColour2;
    colour2.a *= colour2.a;
    float4 playerColour = colour2;

    float edgeShelf = saturate((playerColour.a - 0.25f) * 100.0f);
    float minAlpha  = (1.0f - edgeShelf) * 0.25f;

    colour2.a = min(colour2.a * 8.0f,1.0f);
    colour2.a = max(colour2.a,minAlpha);
    playerColour.a = max(playerColour.a,minAlpha);
//    colour1.a = max(colour1.a,minAlpha);
    //colour1.rgb *= edgeShelf;
//    playerColour.rgb *= edgeShelf;

    float testTime = time;
    float particle = 0.0f;

    float    hdist   = 0.4f * playerColour.a;
    hdist   = min(1.0f,hdist);

    float upper = 0.1f;
    upper += (hdist * 0.4f) + (flat * 0.5f);
    upper += (In.uv0.y * In.uv0.y * In.uv0.y * In.uv0.y * 4.0f);

    upper *= (hdist + 0.5f);
    float waveY = (In.uv0.y - (scale * 0.92f));

#if defined( ITF_DURANGO ) //< pixel shader particle system .. only on Durango
	for(int y = 0; y < 10; y++)
	{
        float timeslot = testTime - (testTime % particleLifeSpan);

        float particlex      = randNum(timeslot.xx);
        float ratio          = ((testTime - timeslot) / particleLifeSpan);
        float particley      = ratio;

        particley *= particley;
        particley = 2.0f - (3.0f * particley);

        float2 ppos         = float2(particlex,particley);
                
        float2 pline;
        pline.x             = abs((frac(((In.uv0.x - ppos.x) * (4.0f - (flat * 3.0f)))) * 2) - 1);
        pline.y             = (waveY - ppos.y);
        
        float radRatio      = 1.0 - min(1.0f,1.0f * (dot(pline,pline) / (upper)));
   
        particle            += radRatio * (1.0f - ratio);
            
        testTime += 18.234f;
	}
#endif

    particle *= particle;
    particle += (waveY * waveY) * (playerColour.a * 2.0f);
    particle *= (1.9f * hdist) + 0.2f;

    particle = ((particle * 3.8f) + 0.1f);
    particle *= playerColour.a;

    particle = lerp(particle,1.0f,flat * 0.5f);


	colour1.w = ( dot( colour1.rgb, ColourFactors ) );
    colour1.w *= playerColour.a;
	colour2.w = ( dot( colour2.rgb, ColourFactors ) )*0.8f;
    colour2.rgb *= (scale * yscale) + (yscale * 0.75f);

    colour2.rgb *= particle;
    colour2.rgb = lerp(playerColour.rgb,colour2.rgb,playerColour.a) * playerColour.a;

    float4 outColour = saturate( (colour1 * (1.0f - flat)) + colour2);
    float cpos       = saturate(1.0f - min(1.0f,((2.0f - In.uv0.y) - (ps_mumo_reg0.z * 5.0f))));

    outColour       = (outColour * cpos.xxxx) / ps_mumo_reg0.y;

    
    float colDot    = dot(outColour.rgb,outColour.rgb);
    colDot          = saturate((colDot-0.01f) * 10.0f);

    outColour.a     = min(1.0f,outColour.a * 4.0f);
    outColour       *= colDot;

    return outColour;
}

float4 ps_GetHUDColour(float2 uv,float flash,float fade,float alphaCutoff)
{
    const float TWO_PI = 6.283185f;

	float3 ColourFactors = { 0.5, 0.5, 0.5 };
   	float time = ps_mumo_reg0.x;

    float2 src1          = float2(0.5f,1.0f);
    float2 sline1        = uv - src1;
    float tdist          = dot(sline1,sline1);
    
    tdist   *= tdist;
    tdist   *= float(15.0f * TWO_PI);
    tdist   -= time * 6.0f;

    float scale  = 0.0f;//sin(tdist);
    float yscale = uv.y;
    scale        += 1.0f;
    scale        *= yscale * 0.1f;

	float4 srcColour2 = TEXTURE_READ_2D( mumo_samp, 0, uv ).xyzw;
    float4 outColour  = float4(0,0,0,0);

    float4 colour2 = srcColour2;
    colour2.a *= colour2.a;
    float4 playerColour = colour2;

	colour2.w = ( dot( colour2.rgb, ColourFactors ) )*0.8f;
    colour2.rgb *= (scale * yscale) + (yscale * 0.75f);

    colour2.rgb = lerp(playerColour.rgb,colour2.rgb,playerColour.a) * playerColour.a;
    colour2.rgb += (srcColour2.rgb * 0.25f);

    outColour       = saturate( colour2 );

    outColour       = (outColour * fade) / (1.01f - flash);
    
    float colDot    = dot(outColour.rgb,outColour.rgb);
    colDot          = saturate((colDot-0.01f) * 10.0f);

    outColour.a     = min(1.0f,outColour.a * 4.0f);
    outColour       *= colDot * ((srcColour2.a - (flash * 0.25f)) * (1.0f / (1.0f - (flash * 0.25f))));

    return outColour;
}

float inUnitRect(float2 uv)
{
    float2 low   = saturate(uv * 100.0f);
    float2 high  = saturate((float2(1.0f,1.0f) - uv) * 100.0f);

    return min(min(low.x,high.x),min(low.y,high.y));
}

float4 ps_mumo_ingameHUD( VS_OUT In ) : PS_OUT_COLOR
{
    float2 uv            = In.uv0;
    uv  *= 3.0f;
    uv  -= float2(1.0f,1.0f);

    float  fade          = ps_mumo_reg0.y;
    float2 src1          = float2(0.5f,2.0f - (fade * 3.0f));
    float2 sline1        = uv - src1;
    float  tdist         = sqrt(dot(sline1,sline1));
    float  flash         = sin(ps_mumo_reg0.y * 3.14159f);
    sline1               = normalize(sline1) * (flash * -0.04f) * tdist;
	float4 playerTest    = TEXTURE_READ_2D( mumo_samp, 0, uv ).xyzw;
    float4 colsum = float4(0,0,0,0); 

//    if (playerTest.a < 0.25f) 
//    { 
//        colsum = playerTest;
//        flash = 0.0f;
//    }

    float2 uvPos  = uv;
    float  power  = 1.0f;
    float  sum    = 0.0f;
    float cutoff  = 0.0f;

    if (flash > 0.0f)
    {
        for (int i = 0;i < 30;i++)
        {
            sum     += power;
            if (inUnitRect(uvPos) > 0.0f)
            {
                colsum  += ps_GetHUDColour(uvPos,flash,fade,cutoff) * power;
            }
            cutoff  = 0.15f;
            uvPos   += sline1;
            power   *= 0.75f;
        }
        colsum  = colsum / sum;
    }
    else
    {
        colsum = ps_GetHUDColour(uvPos,0.0f,1.0f,0.0f) * inUnitRect(uvPos);
    }
    
    float4 outColour = saturate( colsum );
    outColour.a = dot(outColour.rgb,float3(0.299f,0.587f,0.114f)) * max((1.0f - flash),fade);
    //outColour.a = 1.0f;

    return outColour;
    
    //return float4(playerTest.aaa,1.0f);
}


#endif // PIXEL_PROFILE

#endif //MUSICMOTION__FX