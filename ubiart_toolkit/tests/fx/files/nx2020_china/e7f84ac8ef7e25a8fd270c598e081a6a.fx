// Andrew Davies Ubisoft Newcastle 2013

#define CB_PIP

#ifndef PIP_FX
#define PIP_FX

// A pixel shader to extract multiple player silhouettes from the kinect depth stream.
// The three least sig bits of the kinect depth stream contain skeleton identity values.
// These are extracted and used to select a colour from an array in the input constants.
// If the val is 0, this represents either the background or an object. If the depth (in 
// the top 13 bits) is non-zero then we assume the pixel represents part of an object 
// otherwise it's background. The user can set a colour for background and one for object 
// pixels. The general idea is to use this shader early in the rendering to produce a 
// texture to be used as input to more complex effects.
//
// The vertex shader is used but doesn't do anything critical to the process.

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

REGISTER_SAMPLER( TextureSampler, 0 )

#ifdef DX11_SHADERS
REGISTER_SAMPLER( DepthSampler, 1 )
REGISTER_SAMPLER( IRSampler, 2 )
#endif
// ---------------------------
// VERTEX SHADER
// ---------------------------

struct VS_OUT                                
{                                            
     float4 Position    : VS_OUT_POS;
     float2 UV          : TEXCOORD0;
};

#ifdef VERTEX_PROFILE

VS_OUT VS_PiP(
	float4 Position		: POSITION,
	float2 vTexture		: TEXCOORD0 )
{                                            
	VS_OUT Out;                              
	Out.Position = Position;
	Out.UV = vTexture;
	return Out;                              
}

#endif // VERTEX_PROFILE

// ---------------------------
// PIXEL SHADER
// ---------------------------

#ifdef PIXEL_PROFILE
// function body generated from 90000 generation genetic algorithm to compensate for brightness variance across IR capture plane.
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


float4 PS_PiP( VS_OUT In ) : PS_OUT_COLOR
{
#if defined(__PSSL__)    
    // During this process, each pixel is assigned a 16-bit value that has the following meaning:
    //  � 0 - invalid pixels
    //  � 1 to 10 - pixels falling in this range are considered as part of users. All the pixels with the same value
    //      are therefore considered to belong to the same user.
    //  � 11 - pixels having this value refer to the 7oor. The 7oor information is not available until a calibration
    //      has been run successfully.
    //  * 12 to 255 - pixels within this range refer to components that have been identi6ed as objects in the
    //  scene. All the pixels with the same value are therefore considered to belong to the same object
    
	float fLabel = TEXTURE_READ_2D( TextureSampler, 0, In.UV ).r * 65535.0f;
	int label = (int)fLabel;

    if( ( label == 0 ) || ( label >= 11 ) )
    {
        return float4( 0.0f, 0.0f, 0.0f, 0.0f );
    }

    return ps_pipParam.vSkelColors[ label ];
#endif

//#if defined(__PSSL__)
//	float depth = TEXTURE_READ_2D( TextureSampler, 0, In.UV ).r;
//	int label = (int)( depth * 65535.f);
//	
//	float4 diff = ps_pipParam.vObjectColor - label.xxxx;
//	
//	//select the index of the color to paint the player
//	//if the index is 0 there is no player
//	int index = dot(((int4)step(abs(diff) , float(0.f).xxxx)),int4(1,2,3,4));
//	
//	return index > 0 ? ps_pipParam.vSkelColors[index - 1] : ps_pipParam.vBackgroundColor;
//#endif

#if defined(ITF_WIN32) || defined(ITF_DURANGO) || defined(ITF_ORBIS)
	// Pretty sure this isn't the most efficient shader implementation but it is a working implementation =)

#ifdef DX11_SHADERS

// on durango, both ITF_DURANGO & ITF_WIN32 are defined. So test everything except ITF_WIN32 first
#if defined (ITF_DURANGO) || defined(ITF_ORBIS)

	// Durango : Player stream is L8.
    //< initial constants for selecting 4 offsets around a centre pixel
    const float offx            = 1.0f / (512.0f * 1.99f);
    const float offy            = 1.0f / (424.0f * 1.99f);
    const float2 uv0c          = float2(-offx,0) + In.UV;
    const float2 uv1c          = float2(offx,0) + In.UV;
    const float2 uvc0          = float2(0,-offy) + In.UV;
    const float2 uvc1          = float2(0,offy) + In.UV;

    //< get centre pixel values for player ID .. abort if this is a non-player pixel
    int playerIDVal = TEXTURE_READ_2D( TextureSampler, 0, In.UV).r * 255;
    
    if( playerIDVal == 255 ) // If 255, the pixel is not part of any skel/player and is either background or an object.
	{
		// If the original depth value is 0 the pixel is background.
		return float4(ps_pipParam.vBackgroundColor.rgb,0);
	}

    //< get centre pixel values for IR, depth and player ID
    float lumcc = TEXTURE_READ_2D( DepthSampler , 1, In.UV).r;
    float lumIR = TEXTURE_READ_2D( IRSampler    , 2, In.UV).r;
    float bulbCorrection   = GetIRBulbCorrection(In.UV);

    lumIR   = lumIR * bulbCorrection;

    float originalIR = lumIR;

    float4 playerCol = ps_pipParam.vSkelColors[ playerIDVal & 7 ];
    float IRDist = lumIR * (lumcc * lumcc);

    float lum0c = TEXTURE_READ_2D( DepthSampler, 1, uv0c ).r;
    float lum1c = TEXTURE_READ_2D( DepthSampler, 1, uv1c ).r;
    float lumc0 = TEXTURE_READ_2D( DepthSampler, 1, uvc0 ).r;
    float lumc1 = TEXTURE_READ_2D( DepthSampler, 1, uvc1 ).r;

    float4 plPixels = float4(TEXTURE_READ_2D( TextureSampler, 0, uv0c ).r,TEXTURE_READ_2D( TextureSampler, 0, uv1c ).r,TEXTURE_READ_2D( TextureSampler, 0, uvc0 ).r,TEXTURE_READ_2D( TextureSampler, 0, uvc1 ).r);

    float minPlr = 1.0f - ((plPixels.r * plPixels.g * plPixels.b * plPixels.a));        //< if all four pixels are player pixels.. allow the result to be non-zero
    float avgs  = dot(plPixels,float4(0.25f,0.25f,0.25f,0.25f));//< calculate the average depth 
    float errx  = abs(lumcc - ((lum0c + lum1c) * 0.5f));        //< estimate the error in depth across the pixel on the X axis (the general pixel error without light atenuation bais) 
    float erry  = abs(lumcc - ((lumc0 + lumc1) * 0.5f));        //< estimate the error in depth across the pixel on the Y axis (the general pixel error without light atenuation bais)

    float errScale = exp(-50.0f * (errx + erry));

    float lum = saturate(minPlr * errScale * (IRDist * 200000.0f));
    float4 finalCol = playerCol * lum;

    if (lum < 0.3f) 
    {
        playerCol.a = 0.0f;
    }
    
    finalCol.a = playerCol.a;
    //finalCol.rgb = originalIR.xxx;
    return finalCol;

#else
	
	int depth = TEXTURE_READ_2D( TextureSampler, 0, In.UV).r * 65535;
	
	int playerIDVal = depth & 7;
    
    if( playerIDVal == 0 ) // If 0, the pixel is not part of any skel/player and is either background or an object.
	{
		return ( depth != 0 ) ? ps_pipParam.vObjectColor : ps_pipParam.vBackgroundColor;
	}

	// If the val isn't 0 then we need to subtract 1 from it to get the skel index.
	// Skel index is 0-5 (as there's a max of 6 skels on kinect)
	// playerIDVal is 0-7 -> 0-6 after subtracting 1
	// So it's theoretically possible to index a seventh element in the sklelColour array. 
	// This should never happen but I've extended the array because it's the best compromise 
	// to safety and shader speed (no more 'if's).
	int skelIdx = playerIDVal - 1;

	return ps_pipParam.vSkelColors[skelIdx];

#endif
#else

    // Depth stream is DL16. Sampler is set to point. We can grab r,g,b or a.
#if defined(ITF_WIN32)
    //On dx9, the depth is accessible in the z/b component. 
    //Since depth stream is L16 which has integer unorm, we multiply by (2^n - 1). 
    int depth = TEXTURE_READ_2D( TextureSampler, 0, In.UV ).z  * 65535;
#else
    //Make sure this is unsigned int and not UNORM.
    int depth = TEXTURE_READ_2D( TextureSampler, 0, In.UV ).r;
#endif

	int playerIDVal = ( depth % 8 ); // 'Mask' out to 13 bits, leaving just the bottom 3 identity bits.
	if( playerIDVal == 0 ) // If 0, the pixel is not part of any skel/player and is either background or an object.
	{
		// If the original depth value is 0 the pixel is background.
		return ( depth != 0 ) ? ps_pipParam.vObjectColor : ps_pipParam.vBackgroundColor;
	}

	// If the val isn't 0 then we need to subtract 1 from it to get the skel index.
	// Skel index is 0-5 (as there's a max of 6 skels on kinect)
	// playerIDVal is 0-7 -> 0-6 after subtracting 1
	// So it's theoretically possible to index a seventh element in the sklelColour array. 
	// This should never happen but I've extended the array because it's the best compromise 
	// to safety and shader speed (no more 'if's).
	int skelIdx = playerIDVal - 1;

	return ps_pipParam.vSkelColors[skelIdx];
#endif

#else //defined(ITF_WIN32) || defined(ITF_DURANGO) || defined(ITF_ORBIS)
	return float4( 1.f, 1.f, 1.f, 1.f );
#endif // defined(ITF_WIN32) || defined(ITF_DURANGO) || defined(ITF_ORBIS)
}

#endif // PIXEL_PROFILE

#endif  // PIP_FX

