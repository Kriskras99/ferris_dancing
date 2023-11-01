//--------------------------------------------------------------------------------------
//
// File: RenderPCT.fx
//
//--------------------------------------------------------------------------------------

#define CB_RENDERPCT

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

//--------------------------------------------------------------------------------------
// Define
//--------------------------------------------------------------------------------------

#ifdef LIGHT3D
	#define VIEWPOS 1
#endif

//--------------------------------------------------------------------------------------
// Texture samplers
//--------------------------------------------------------------------------------------

#ifdef PIXEL_PROFILE
REGISTER_SAMPLER(TextureSampler, 0) 
REGISTER_SAMPLER(TextureSampler, 1) // scene
REGISTER_SAMPLER(TextureSampler, 2) // normal / diffuse_2
REGISTER_SAMPLER(TextureSampler, 3) // backlight
REGISTER_SAMPLER(TextureSampler, 4) // separate alpha
REGISTER_SAMPLER(TextureSampler, 5) // backlight_2

REGISTER_SAMPLER(FrontLightMaskSampler, 6)
REGISTER_SAMPLER(BackLightMaskSampler, 7) 
#endif

//--------------------------------------------------------------------------------------
// Vertex shader input structure
//--------------------------------------------------------------------------------------
struct VS_DEFAULT_INPUT
{
	float4 vPos			: POSITION;
  #ifdef COLOR
	float4 fColor		: COLOR0;
  #endif
  #ifdef SKINNING
	float4 SknBlendWeight : BLENDWEIGHT;  
    #if defined(_CAFE_) || defined(_NX_) ||defined(DX11_SHADERS)
	int4 SknBlendIndices : BLENDINDICES;
    #else
	float4 SknBlendIndices : BLENDINDICES;
    #endif
  #endif  
  #ifdef TEXTURE
    float2 TextureUV	: TEXCOORD0;
	#ifdef TEXTUREUV2
		float2 TextureUV2	: TEXCOORD3;
	#endif
  #endif
};

//--------------------------------------------------------------------------------------
// Vertex shader output structure
//--------------------------------------------------------------------------------------
struct VS_DEFAULT_OUTPUT
{
    float4 Position   : VS_OUT_POS;   // vertex position 
    
  #ifdef COLOR
    float4 Diffuse    : COLOR0_C;    // vertex diffuse color (note that COLOR0 is clamped from 0..1)
  #endif
  #ifdef TEXTURE
    float4 TextureUV  : TEXCOORD0;  // vertex texture coords 
  #endif
  #ifdef FOGBOX1
    float4 fog1	  : TEXCOORD1;  //  xy : position in view space, z: alphaAttenuation
  #endif
  #ifdef FOGBOX2
    float4 fog2	  : TEXCOORD2;  //  xy : position in view space, z: alphaAttenuation
  #endif
  #ifdef VIEWPOS
    float4 viewPos	  : TEXCOORD4;
  #endif
  #if defined(LIGHT) || defined(REFLECTION)
	#ifndef DX11_SHADERS
		float4 screenPos  : VPOS;
	#endif
  #endif
  #ifdef TANGENT
    float4 Tangent	  : TEXCOORD5;  //xy : 2D tangent from bezier
  #endif
};

struct VS_PCT_OUTPUT
{
    float4 Position   : VS_OUT_POS;   // vertex position 
    float4 Diffuse    : COLOR0_C;    // vertex diffuse color (note that COLOR0 is clamped from 0..1)
    float4 TextureUV  : TEXCOORD0;  // vertex texture coords : xy->UV1, zw->UV2
  #ifdef FOGBOX1
    float4 fog1	  : TEXCOORD1;  //  xy : position in view space, z: alphaAttenuation
  #endif
  #ifdef FOGBOX2
    float4 fog2	  : TEXCOORD2;  //  xy : position in view space, z: alphaAttenuation
  #endif
  #ifdef VIEWPOS
    float4 viewPos	  : TEXCOORD4;
  #endif
  #ifdef TANGENT
    float4 Tangent	  : TEXCOORD5;  //xy : 2D tangent from bezier
  #endif
};

struct VS_PC2T_OUTPUT
{
    float4 Position   : VS_OUT_POS;   // vertex position 
    float4 Diffuse    : COLOR0_C;     // vertex diffuse color (note that COLOR0 is clamped from 0..1)
    float2 TextureUV  : TEXCOORD0;  // vertex texture coords 
  #ifdef FOGBOX1
    float4 fog1	  : TEXCOORD1;  //  xy : position in view space, z: alphaAttenuation
  #endif
  #ifdef FOGBOX2
    float4 fog2	  : TEXCOORD2;  //  xy : position in view space, z: alphaAttenuation
  #endif
    float4 ScreenUV  : TEXCOORD3;  // vertex texture coords 
  #ifdef VIEWPOS
    float4 viewPos	  : TEXCOORD4;
  #endif
};

#ifdef VERTEX_PROFILE

//--------------------------------------------------------------------------------------
// These shaders computes standard transform and lighting
//--------------------------------------------------------------------------------------

float4 zinject(in float4 vpos)
{
#ifdef ZINJECT
	vpos.z = vs_zInject.x * vpos.w;
#endif
    return vpos;
}

float4 computeDynFogColor_VS(in float3 _position, in VS_DynFogParam _fogParam )
{
	float2 boxDir = _fogParam.f4_BoxCenter.xy - _position.xy;

	float f_camDist = _fogParam.f4_CamFarNearDist.x - _position.z;
	float f_camFactor = _fogParam.f4_CamFarNearDist.y - f_camDist;
	f_camFactor = saturate(f_camFactor * _fogParam.f4_CamFarNearDist.z);
	
	f_camFactor = lerp(_fogParam.f4_AlphaAtt.y, _fogParam.f4_AlphaAtt.x, f_camFactor);
	
	return float4(boxDir,f_camFactor, 1.0f);
}

void fillCommonPCTOutput(inout VS_PCT_OUTPUT _output, float4 _position)
{
    _output.Position = mul(_position, vs_mWorldViewProjection);
	_output.Diffuse = float4(0.0f, 0.0f, 0.0f, 0.0f);
	_output.TextureUV = float4(0.0f, 0.0f, 0.0f, 0.0f);

  #if defined(FOGBOX1) || defined(VIEWPOS)
    float3 viewPos = mul(_position, vs_mWorld).xyz;
  #endif
  #ifdef FOGBOX1
    _output.fog1 = computeDynFogColor_VS(viewPos, vs_fog1Param);
   #ifdef FOGBOX2
    _output.fog2 = computeDynFogColor_VS(viewPos, vs_fog2Param);
   #endif
  #endif
  #ifdef VIEWPOS
    _output.viewPos = float4(viewPos, 0.0f);
  #endif

    _output.Position = zinject(_output.Position);
}

VS_DEFAULT_OUTPUT default_VS( VS_DEFAULT_INPUT In )
{
    VS_DEFAULT_OUTPUT Output;
	
	#ifdef SKINNING

	float3 v;
	v = mul( In.vPos, vs_mBoneMatrix00[In.SknBlendIndices.x]) * In.SknBlendWeight.x;
	v += mul( In.vPos, vs_mBoneMatrix00[In.SknBlendIndices.y]) * In.SknBlendWeight.y;
	v += mul( In.vPos, vs_mBoneMatrix00[In.SknBlendIndices.z]) * In.SknBlendWeight.z;
	v += mul( In.vPos, vs_mBoneMatrix00[In.SknBlendIndices.w]) * In.SknBlendWeight.w;
	In.vPos = float4( v, 1);

    #endif // SKINNING
        
    Output.Position = mul(In.vPos, vs_mWorldViewProjection);
    
  #if defined(FOGBOX1) || defined(VIEWPOS)
    float3 viewPos = mul(In.vPos, vs_mWorld).xyz;
  #endif
  #if defined(VIEWPOS)
    Output.viewPos = float4(viewPos, 0.0f);
  #endif
  
  #ifdef FOGBOX1
    Output.fog1 = computeDynFogColor_VS(viewPos, vs_fog1Param);
   #ifdef FOGBOX2
    Output.fog2 = computeDynFogColor_VS(viewPos, vs_fog2Param);
   #endif
  #endif
  

  #ifdef COLOR
    Output.Diffuse = In.fColor * vs_globalColor; 
  #endif  
    
  #ifdef TEXTURE
  float2 vTexCoord0 = In.TextureUV;
  #ifdef UV1
    float2x2 uvm = (float2x2)vs_mUVmat;
    vTexCoord0 = vTexCoord0 * 2.f - 1.f;
    vTexCoord0 += float2(vs_mUVmat[0][3], vs_mUVmat[1][3]);
    vTexCoord0 = mul(vTexCoord0, uvm);
    vTexCoord0 = ( vTexCoord0 + 1.f ) * 0.5f;
    vTexCoord0 += float2(vs_mUVmat[3][0], vs_mUVmat[3][1]);	
  #endif
	Output.TextureUV = vTexCoord0.xyxy;
    #ifdef UV2
	  float2x2 uvm2 = (float2x2)vs_mUVmat2;
      #ifdef TEXTUREUV2
	  vTexCoord0 = In.TextureUV2 * 2.f - 1.f;
	  #else
      vTexCoord0 = In.TextureUV * 2.f - 1.f;
	  #endif
      vTexCoord0 += float2(vs_mUVmat2[0][3], vs_mUVmat2[1][3]);
      vTexCoord0 = mul(vTexCoord0, uvm2);
      vTexCoord0 = ( vTexCoord0 + 1.f ) * 0.5f;
      vTexCoord0 += float2(vs_mUVmat2[3][0], vs_mUVmat2[3][1]);	  
	  Output.TextureUV.zw = vTexCoord0;
    #endif

  #endif
  
    Output.Position = zinject(Output.Position);
	return Output;    
}

VS_PCT_OUTPUT default_PTambiant_VS( float4 vPos : POSITION,
			  float2 vTexCoord0 : TEXCOORD0 )
{
    VS_PCT_OUTPUT Output;
    
	fillCommonPCTOutput(Output, vPos); 
    Output.TextureUV = vTexCoord0.xyxy;
	Output.Diffuse = vs_globalColor;

    return Output;    
}

//--------------------------------------------------------------------------------------
// This shader computes Frize vertex anim.
/*
vs_vconst1.x = TimeCur
vs_vconst1.y = global sync
vs_vconst1.z = global speed
vs_vconst1.w = global rotation

uv2.x = speed X vertex
uv2.y = speed Y vertex
uv2.z = synchro X vertex
uv2.w = synchro Y vertex

uv3.x = amplitude X vertex
uv3.y = amplitude Y vertex
uv3.z = synchro Vertex
uv3.w = angle Vertex
*/
//--------------------------------------------------------------------------------------
VS_PCT_OUTPUT frize_PNC3T_VS( float4 vPos : POSITION, 
                         float4 fColor : COLOR0,
                          float2 vTexCoord0 : TEXCOORD0,
                          float4 uv2 : TEXCOORD1,
                          float4 uv3 : TEXCOORD2,
                          float2 uv4 : TEXCOORD3 )
{
    VS_PCT_OUTPUT Output;  

    float time = ( vs_frizeAnim.x * vs_frizeAnim.z ) + vs_frizeAnim.y + uv3.z;
    float angle = uv3.w + vs_frizeAnim.w;
    
    float x1 = cos( time * uv2.x + uv2.z) * uv3.x;
    float y1 = sin( time * uv2.y + uv2.w) * uv3.y;

    float cosAngle = cos(angle);
    float sinAngle = sin(angle); 

    float x2 = x1 * cosAngle - y1 * sinAngle;
    float y2 = x1 * sinAngle + y1 * cosAngle;
    
    float4 posw = float4(vPos.x + x2, vPos.y + y2, vPos.z, 1.f);
                
	fillCommonPCTOutput(Output, posw); 

    Output.Diffuse = fColor * vs_globalColor;
    
    float2x2 uvm = (float2x2)vs_mUVmat;
    Output.TextureUV = mul(vTexCoord0, uvm).xyxy;
    Output.TextureUV.xy += float2(vs_mUVmat[3][0],  vs_mUVmat[3][1]);
    
  #ifdef UV2
    float2x2 uvm2 = (float2x2)vs_mUVmat2;
    Output.TextureUV.zw = mul(vTexCoord0, uvm2); //vs_mUVmat
	Output.TextureUV.zw += float2(vs_mUVmat2[3][0], vs_mUVmat2[3][1]);	
  #endif

    return Output;    
}

//--------------------------------------------------------------------------------------
// This shader computes bezier patch for anim.
//--------------------------------------------------------------------------------------
VS_PCT_OUTPUT default_PCT_Patch_VS( float4 vPos : POSITION,
			  float2 vTexCoord0 : TEXCOORD0)
{
    VS_PCT_OUTPUT Output;
    
    int pos = vPos.z * 8;
    int colorpos = vPos.z;
    
    /// Vertex -> vector const xy.
    float2 PF1 = vs_va0[pos].xy * (1 - vTexCoord0.y) + vs_va0[pos + 4].xy * vTexCoord0.y;    
    float2 PF1X = vs_va0[pos + 1].xy * (1 - vTexCoord0.y) + vs_va0[pos + 5].xy * vTexCoord0.y;    
    float2 PFX2 = vs_va0[pos + 2].xy * (1 - vTexCoord0.y) + vs_va0[pos + 6].xy * vTexCoord0.y;    
    float2 PF2 = vs_va0[pos + 3].xy * (1 - vTexCoord0.y) + vs_va0[pos + 7].xy * vTexCoord0.y;    

    float UvInv = (1-vTexCoord0.x);
    float2 carreinv = UvInv*UvInv;
    float2 carre = vTexCoord0.x*vTexCoord0.x;
    float2 cube = vTexCoord0.x*vTexCoord0.x*vTexCoord0.x;
    float2 cubeinv = UvInv*UvInv*UvInv;
    
    float2 Position = PF1*cubeinv + 3*PF1X*vTexCoord0.x*carreinv+3*PFX2*carre*UvInv + PF2*cube;

	/// Uv -> vector const zw.
    PF1 = vs_va0[pos].zw * (1 - vTexCoord0.y) + vs_va0[pos + 4].zw * vTexCoord0.y;    
    PF1X = vs_va0[pos + 1].zw * (1 - vTexCoord0.y) + vs_va0[pos + 5].zw * vTexCoord0.y;    
    PFX2 = vs_va0[pos + 2].zw * (1 - vTexCoord0.y) + vs_va0[pos + 6].zw * vTexCoord0.y;    
    PF2 = vs_va0[pos + 3].zw * (1 - vTexCoord0.y) + vs_va0[pos + 7].zw * vTexCoord0.y;    

	float4 position = float4(Position.x,Position.y, vs_patchZ.x, 1.f);
	fillCommonPCTOutput(Output, position); 

	float2 texUV = PF1*cubeinv + 3*PF1X*vTexCoord0.x*carreinv+3*PFX2*carre*UvInv + PF2*cube;
    Output.TextureUV = texUV.xyxy;
	
	// alpha.
	float a0 = vs_va1[colorpos].x;
	float a1 = vs_va1[colorpos].y;
	float a2 = vs_va1[colorpos].z;
	float a3 = vs_va1[colorpos].w;
	
	float l1 = lerp( a0, a1, vTexCoord0.x);
	float l2 = lerp( a2, a3, vTexCoord0.x);
	
	Output.Diffuse.a = lerp(l1, l2, vTexCoord0.y) * vs_globalColor.a;
    Output.Diffuse.rgb = vs_globalColor.rgb;
	    
    return Output;    
}

//--------------------------------------------------------------------------------------
// This shader computes bezier patch with fixed witdh (procedural).
//--------------------------------------------------------------------------------------
VS_PCT_OUTPUT default_PCT_BezierPatch_VS( float4 vPos : POSITION,
			  float2 vTexCoord0 : TEXCOORD0)
{
    VS_PCT_OUTPUT Output;
    
    int pos 		= (vs_patchParam.z + vPos.z) * 5;
    int colorPos 	= (vs_patchParam.z + vPos.z) * 2;
    
    // Voluntary inversion !
	float x = vTexCoord0.y * vs_patchParam.x;
	float y = vTexCoord0.x;
    
	float curYUV 		=  vs_va0[pos + 4].x;
	float curWidthUV 	=  vs_va0[pos + 4].y;
    
	float UvInv 		= (1-x);
    float carreinv 		= UvInv*UvInv;
    float carre 		= x*x;
    float cube 			= x*x*x;
    float cubeinv 		= UvInv*UvInv*UvInv;
    	
	/// POINTS	
	/// Get position on Bezier curve
	float4 dataInter = vs_va0[pos]*cubeinv + 3*vs_va0[pos+1]*x*carreinv + 3*vs_va0[pos+2]*carre*UvInv + vs_va0[pos+3]*cube;

	/// Get Tangeant
	float4 Tangeant = 	3 * ( vs_va0[pos] * (-UvInv * UvInv) + 
						vs_va0[pos+1] * (carreinv - 2 * x * UvInv) +
						vs_va0[pos+2] * (2 * x * UvInv - carre) +
						vs_va0[pos+3] * carre );

	
	/// Get Point Perpendicular
	float2 TangeantPt = normalize(Tangeant.xy);
	float2 Perpendicular;
	Perpendicular.x = -TangeantPt.y;
	Perpendicular.y = TangeantPt.x;
	
	#ifdef TANGENT
    Output.Tangent = normalize(Perpendicular.xy).xyxy * (y - 0.5f) * 2.f;
    #endif
	
	/// Extrude Position
	float2 Position = dataInter.xy + (Perpendicular * dataInter.w * (y - 0.5f));

	/// Get position on Bezier curve
	float2 TextureUV = float2(dataInter.z, curYUV - curWidthUV * (y - 0.5f));

	/// Output
	float4 position = float4(Position.x,Position.y, vs_patchZ.x, 1.f);
	fillCommonPCTOutput(Output, position); 

    if (vs_patchParam.y)
		Output.TextureUV = TextureUV.yxyx;
	else
		Output.TextureUV = TextureUV.xyxy;
	
	/// color.
	Output.Diffuse = lerp( vs_va1[colorPos], vs_va1[colorPos+1], x) * vs_globalColor;
	
    return Output;     
}


VS_PCT_OUTPUT fluid_PCT_VS( float4 vPos : POSITION,
			  float2 vTexCoord0 : TEXCOORD0 )
{
    VS_PCT_OUTPUT Output;
    

    float idx = vTexCoord0.y*vs_fluidVDiv;
    int   idxInt1 = idx/2.f;
    int   idxInt2 = idx/2.f + 0.5f;    
    int   factor  = idxInt2 - idxInt1;
    
    float steps = vs_fluidParam.y;
    float delta1 = vs_fluidParam.z;
    float delta2 = vs_fluidParam.w;
    
    float interpolatedXOffset = delta1*(steps-idx)+delta2*idx;
    float colorInMap = vs_va0[idxInt1].x*(1-factor) + vs_va0[idxInt1].z*factor;
    int   secondColorIndex = floor(colorInMap)*4;
    
    float4 position = vPos;
    position.y += (interpolatedXOffset)*(1.f-vTexCoord0.x);
    //position.y += (vs_va0[idxInt1].x*(1-factor) + vs_va0[idxInt1].z*factor)*(1.f-vTexCoord0.x);
    position.x -= (vs_va0[idxInt1].y*(1-factor) + vs_va0[idxInt1].w*factor)*(1.f-vTexCoord0.x);
    
	fillCommonPCTOutput(Output, position); 

	float ratio = vTexCoord0.y*vs_fluidVDiv/vs_fluidParam.x;
	float2 texUV = (vs_fluidUV1.xy*(1 - ratio) + vs_fluidUV2.xy*ratio)*vTexCoord0.x + 
		(vs_fluidUV1.zw*(1 - ratio) + vs_fluidUV2.zw*ratio)*(1-vTexCoord0.x);
	Output.TextureUV = texUV.xyxy;
		
	
    float4 color0 = vs_va1[secondColorIndex];
    float4 color1 = vs_va1[secondColorIndex+1];
    float4 color2 = vs_va1[secondColorIndex+2];
    float4 color3 = vs_va1[secondColorIndex+3];
    
    
	Output.Diffuse =
		(color0*(1 - ratio) + color2*ratio)*vTexCoord0.x + 
		(color1*(1 - ratio) + color3*ratio)*(1-vTexCoord0.x);
	
	Output.Diffuse *= vs_globalColor;

    return Output;    
}

VS_PCT_OUTPUT fluid2_PCT_VS( float4 vPos : POSITION,
			  float2 vTexCoord0 : TEXCOORD0 )
{
    VS_PCT_OUTPUT Output;
    
    
    float idx = vTexCoord0.y*vs_fluidVDiv;
    int   idxInt1 = idx/2.f;
    int   idxInt2 = idx/2.f + 0.5f;
    int   factor  = idxInt2 - idxInt1;
    
    float steps = vs_fluidParam.y;
    float delta1 = vs_fluidParam.z;
    float delta2 = vs_fluidParam.w;
    
    float interpolatedXOffset = delta1*(steps-idx)+delta2*idx;
    float colorInMap = vs_va0[idxInt1].x*(1-factor) + vs_va0[idxInt1].z*factor;
    int   secondColorIndex = floor(colorInMap)*4;
    
    float4 position = vPos;
	position.y += interpolatedXOffset;
    //position.y += (vs_va0[idxInt1].x*(1-factor) + vs_va0[idxInt1].z*factor);
    position.z -= (vs_va0[idxInt1].y*(1-factor) + vs_va0[idxInt1].w*factor)*(1.f-vTexCoord0.x);
   
    fillCommonPCTOutput(Output, position); 


	float ratio = vTexCoord0.y*vs_fluidVDiv/vs_fluidParam.x;
	float2 texUV = (vs_fluidUV1.xy*(1 - ratio) + vs_fluidUV2.xy*ratio)*vTexCoord0.x + 
		(vs_fluidUV1.zw*(1 - ratio) + vs_fluidUV2.zw*ratio)*(1-vTexCoord0.x);
	Output.TextureUV = texUV.xyxy;
		

    float4 color0 = vs_va1[secondColorIndex];
    float4 color1 = vs_va1[secondColorIndex+1];
    float4 color2 = vs_va1[secondColorIndex+2];
    float4 color3 = vs_va1[secondColorIndex+3];
        
	Output.Diffuse =
		(color0*(1 - ratio) + color2*ratio)*vTexCoord0.x + 
		(color1*(1 - ratio) + color3*ratio)*(1-vTexCoord0.x);
	
	Output.Diffuse *= vs_globalColor;

    return Output;    
}

// --------------------------------------------------------------------------------------
// Trail 3D shader
// --------------------------------------------------------------------------------------
VS_PCT_OUTPUT trail_PCT_VS( float4 vPos : POSITION,
							float2 vTexCoord0 : TEXCOORD0 )
{
    VS_PCT_OUTPUT Output;

	int		idxInt		= vTexCoord0.y*vs_trailParam.x;
	
	float4 position		= vPos;
	Output.TextureUV	= vTexCoord0.xyxy;
	Output.Diffuse		= float4(0,0,0,0);

	position.xy	= vs_va0[idxInt].xy*(1-vTexCoord0.x) + vs_va0[idxInt].zw*vTexCoord0.x;
	int	  idxDec  = idxInt % 4;
	int   idxIn   = idxInt / 4;

	if (idxDec == 0)
		position.z = vs_va1[idxIn].x;
	else if (idxDec == 1)
		position.z = vs_va1[idxIn].y;
	else if (idxDec == 2)
		position.z = vs_va1[idxIn].z;
	else
		position.z = vs_va1[idxIn].w;
						
    fillCommonPCTOutput(Output, position); 

	float alpha			= lerp(vs_trailAlpha.x, vs_trailAlpha.y, Output.TextureUV.x);
	Output.TextureUV.x	= vTexCoord0.y*vs_trailParam.x/vs_trailParam.y;
	Output.TextureUV.y	= vTexCoord0.x;
	Output.Diffuse		= vs_globalColor * alpha;
	
	if (vs_trailParam.z > 0.f && idxInt > 0)
	{
		float alphaFactor 	= length(vs_va0[idxInt].xy + vs_va0[idxInt].zw - 
								   vs_va0[idxInt-1].xy - vs_va0[idxInt-1].zw)*0.5f/vs_trailParam.z;
		alphaFactor			= clamp(alphaFactor, 0, 1);
		Output.Diffuse.w   *= alphaFactor;
	}

    return Output;    
}

// --------------------------------------------------------------------------------------
// Spline shader
// --------------------------------------------------------------------------------------

VS_PCT_OUTPUT spline_PCT_VS(  float4 vPos : POSITION,
							float2 vTexCoord0 : TEXCOORD0 )
{
    VS_PCT_OUTPUT Output;
	
	float4 position		= vPos;
	Output.TextureUV	= vTexCoord0.xyxy;

	float _t = vTexCoord0.x * (vs_splineParam.x - 3) + 1; //vs_splineParam.x = bufferPoint
	int pos1 = (int)(vTexCoord0.x * (vs_splineParam.x - 3) + 1); 
	int pos0 = pos1-1;
	int pos2 = pos1+1;
	int pos3 = pos1+2;

	
	float time1 = vs_va0[pos1].w;
	float time2 = vs_va0[pos2].w;
	
	
	if (time2 < 0)
		time2 = -time2 - 1;

	float3 tan;

	if (time1 < 0)
	{
		time1 = -time1 - 1;
		_t = time1*(1-(_t-pos1)) + time2*(_t-pos1);
		float deltaT	= _t - time1;
		float deltaTime = time2-time1;
		
		float t  = deltaT/deltaTime;
		float t2 = t * t;
	    float t3 = t2 * t;

	    float b0 = .5f * (  -t3 + 2*t2 - t);
	    float b1 = .5f * ( 3*t3 - 5*t2 + 2);
	    float b2 = .5f * (-3*t3 + 4*t2 + t);
	    float b3 = .5f * (   t3 -   t2    );

	    position.xyz = vs_va0[pos0].xyz*b0 + vs_va0[pos1].xyz*b1 + vs_va0[pos2].xyz*b2 + vs_va0[pos3].xyz*b3;

		float b0P = .5f * (  -3*t2 + 4*t - 1);
		float b1P = .5f * ( 9*t2 - 10*t );
		float b2P = .5f * (-9*t2 + 8*t + 1);
		float b3P = .5f * (   3*t2 - 2*t  );
		tan = vs_va0[pos0].xyz*b0P + vs_va0[pos1].xyz*b1P + vs_va0[pos2].xyz*b2P + vs_va0[pos3].xyz*b3P;	
		tan = tan*(1.f/sqrt(tan.x*tan.x + tan.y*tan.y + tan.z*tan.z));
		
	} else
	{
		 position.xyz = vs_va0[pos1].xyz*(1-(_t-pos1)) + vs_va0[pos2].xyz*(_t-pos1);
		_t = time1*(1-(_t-pos1)) + time2*(_t-pos1);

		 tan = vs_va0[pos2].xyz - vs_va0[pos1].xyz;
		 tan = tan*(1.f/sqrt(tan.x*tan.x + tan.y*tan.y + tan.z*tan.z));
	 }
	
	float3 normal = float3(tan.y, -tan.x, tan.z);
    position.xyz += normal*((Output.TextureUV.y - .5f)*vs_splineParam.y); //vs_splineParam.y = _height

    fillCommonPCTOutput(Output, position); 

	Output.TextureUV.x	= _t*.5f;
	Output.Diffuse 		= vs_globalColor;
	
	return Output;
}

// --------------------------------------------------------------------------------------
// Refraction shader.
// --------------------------------------------------------------------------------------

void ComputePosUV2D( out float4 f4_OutPos, out float2 f2_OutUV, in float4 f4_InPos, in float2 f2_InUV )
{
	f4_OutPos.xy = f4_InPos.xy * float2(2.0f,-2.0f) + float2(-1.0f, 1.0f);
	f4_OutPos.zw = f4_InPos.zw;
	f2_OutUV.x = f2_InUV.x;
	f2_OutUV.y = /*1.0f - */f2_InUV.y;
}

VS_PC2T_OUTPUT refraction_PCT_VS( float4 vPos : POSITION, 
                         float4 fColor : COLOR0,
                         float2 vTexCoord0 : TEXCOORD0 )
{
    VS_PC2T_OUTPUT Output;
       
    Output.Position = mul(vPos, vs_mWorldViewProjection);

  #if defined(FOGBOX1) || defined(VIEWPOS) || defined(LIGHT3D)
    float3 viewPos = mul(Output.Position, vs_mWorld).xyz;
  #endif
  #if defined(LIGHT3D) || defined(VIEWPOS)
    Output.viewPos = float4(viewPos, 0.0f);
  #endif
  #ifdef FOGBOX1
    Output.fog1 = computeDynFogColor_VS(viewPos, vs_fog1Param);
   #ifdef FOGBOX2
    Output.fog2 = computeDynFogColor_VS(viewPos, vs_fog2Param);
   #endif
  #endif

    
    Output.Diffuse = fColor * vs_globalColor; 

    Output.TextureUV = vTexCoord0.xy;
    Output.TextureUV.xy += float2(vs_mUVmat[3][0],  vs_mUVmat[3][1]);
    
    /// Screen pos go from space [-1 1] to [0 1]
    Output.ScreenUV = Output.Position;
    Output.ScreenUV.y = -Output.ScreenUV.y;
    Output.ScreenUV.xy = (Output.ScreenUV.xy + Output.ScreenUV.w)*0.5f;
    Output.ScreenUV.zw = float2(1,Output.ScreenUV.w );
     
    Output.Position = zinject(Output.Position);
    return Output;    
}

struct VS_PCT_INPUT
{
	float4 vPos			: POSITION;
	float4 fColor		: COLOR0;
	int4   iIndex       : BLENDINDICES;
    float2 vTexCoord0	: TEXCOORD0;
};

VS_PCT_OUTPUT OVERLAY_PCBT_VS( VS_PCT_INPUT _in )
{
    VS_PCT_OUTPUT Output;
	
    float4 npos = _in.vPos;

    float4 vOffset;
    if (_in.iIndex.x == 0)
        vOffset = float4(0, 0, 0, 0);
    else
        vOffset = vs_va0[(_in.iIndex.x-1)/2];
    
    if (((_in.iIndex.x-1)%2) == 0)
        npos.xy += vOffset.xy;
    else
        npos.xy += vOffset.zw;
	
	fillCommonPCTOutput(Output, npos); 

    Output.Diffuse = _in.fColor * vs_globalColor;
    Output.TextureUV = _in.vTexCoord0.xyxy;
    return Output;    
}

struct VS_QUAD_INSTANCING_INPUT
{
	float2 vInstance	: TEXCOORD0;
    float2 vPos2D		: TEXCOORD3;
    float2 vPrevPos		: TEXCOORD4;
	float4 fColor		: COLOR0;
};

VS_PCT_OUTPUT QUAD_INSTANCING_VS( VS_QUAD_INSTANCING_INPUT _in )
{
    VS_PCT_OUTPUT Output;
	
	float2 currentPos = _in.vPos2D.xy;
	float2 previousPos = _in.vPrevPos.xy;
	float2 diffPos = (currentPos - previousPos);

	// Position
	const float s_factor = 3.0f;
	float2 halfVec = diffPos * 0.5f * s_factor;
	float2 halfPos = previousPos + halfVec;
	float2 halfVecNorm = normalize(halfVec);
	float2 perpendicular = float2(-halfVecNorm.y, halfVecNorm.x) * vs_SpriteInstancing.x;
	halfVec += halfVecNorm * vs_SpriteInstancing.xx;

	float4 position;
	position.xy = halfPos + (halfVec * _in.vInstance.yy) + (perpendicular * _in.vInstance.xx);
	position.z = vs_SpriteInstancing.y;
	position.w = 1.0f;

	fillCommonPCTOutput(Output, position); 

	// Color
	float speedColor = length(diffPos) * vs_SpriteInstancing.z;
    Output.Diffuse = _in.fColor;
	Output.Diffuse.r = speedColor;
	Output.Diffuse *= vs_globalColor;

	// UV
	float2 uv = _in.vInstance.xy * 0.5f + 0.5f;
    Output.TextureUV = uv.xyxy;

    return Output;    
}

#endif // VERTEX_PROFILE


//--------------------------------------------------------------------------------------
// Pixel shader output structure
//--------------------------------------------------------------------------------------
struct PS_OUTPUT
{
    float4 RGBColor : PS_OUT_COLOR;  // Pixel color    
};

//--------------------------------------------------------------------------------------
// Pixels Shaders.
//--------------------------------------------------------------------------------------

#ifdef PIXEL_PROFILE

//--------------------------------------------------------------------------------------
// FOG functions

float3 computeDynFog_PS(float3 _color, float4 _FogInterpolant, PS_DynFogParam _fogParam)
{
	float	f_DistAttenuation;
#ifdef FULLFOGBOX	
	f_DistAttenuation = _FogInterpolant.z;
#else
	//formule : (att-X)/(size-att) + 1
	float2 f2_ramp = (_fogParam.f4_BoxSizeAtt.zw - abs(_FogInterpolant.xy)) * _fogParam.f4_BoxSizeAtt.xy + 1;
	float2 f2_tmp = saturate( f2_ramp );
	
	f_DistAttenuation	= min( f2_tmp.x, f2_tmp .y );
	f_DistAttenuation *= _FogInterpolant.z;
#endif
	_color.rgb = lerp(_color.rgb, _fogParam.f4_Color.rgb, f_DistAttenuation);

	return _color.rgb;
}

float3 computeFOG(in float3 _colorIn, VS_DEFAULT_OUTPUT In)
{
	float3 colorOut = _colorIn;
	#ifdef STATIC_FOG
		colorOut = lerp(_colorIn, ps_staticFog.xyz, ps_staticFog.w);   //factor fog -> ps_staticFog.w
	#else
	  #ifdef FOGBOX1
		colorOut = computeDynFog_PS(_colorIn, In.fog1, ps_fog1Param);
	   #ifdef FOGBOX2
		colorOut = computeDynFog_PS(colorOut, In.fog2, ps_fog2Param);
	   #endif
	  #endif
	#endif
	return colorOut;
}

//--------------------------------------------------------------------------------------
// Exotic Shader

// overdraw pix
PS_OUTPUT overDraw_PS( VS_PCT_OUTPUT In ) 
{ 
    PS_OUTPUT Output;
	Output.RGBColor.xyz = 1.f;
	Output.RGBColor.w = 0.05f;
	
#ifdef DX11_SHADERS
	if (ps_alphaTest.r > 0.0f)
    {
	   if (Output.RGBColor.w < ps_alphaTest.g)
	   {
	 	  discard;
	   }
    }
#endif
	
    return Output;
}

PS_OUTPUT refraction_PCT_PS(	VS_DEFAULT_OUTPUT In ) 
{ 
    PS_OUTPUT Output;
    
	float4 color = float4(1.0f, 1.0f, 1.0f, 1.0f);
  	float4 uvOrigin = float4(1.0f, 1.0f, 1.0f, 1.0f);

  #ifdef COLOR
	color = In.Diffuse;
  #endif  
  #ifdef TEXTURE  
	uvOrigin = In.TextureUV;
  #endif

	float2 uv = uvOrigin * ps_refractionParam.x;
	float4 normal = TEXTURE_READ_2D(TextureSampler, 2, uv);
	float2 decal = ps_refractionParam.yy * (normal.xy * 2 - 1.0f);
	decal = (decal + 1.0f) * 0.5f;

    Output.RGBColor = decal.xyxy;
	Output.RGBColor.a = normal.a * color.a;
	
#ifdef DX11_SHADERS
	if (ps_alphaTest.r > 0.0f)
    {
	   if (Output.RGBColor.a < ps_alphaTest.g)
	   {
	 	  discard;
	   }
    }
#endif
	
	return Output;
}

PS_OUTPUT fluid_PCT_PS(	VS_DEFAULT_OUTPUT In ) 
{ 
    PS_OUTPUT Output;
	float2 screenPos = float2(1,1);
   #ifdef REFLECTION
		#ifdef DX11_SHADERS
			screenPos = In.Position.xy;
		#else
			screenPos = In.screenPos.xy;
		#endif
   #endif

  #ifdef _XBOX
	screenPos += ps_tillingVposOffset.xy;
  #endif
	float2 screenUV = screenPos * ps_viewportDimensions.zw;

	float4 colorUp = ps_fluidColor2;
	float4 colorDown = ps_fluidColor1;
	float4 colorOff = float4(0.0f, 0.0f, 0.0f, 0.0f);
  #ifdef FLUIDGLOW
	float4 colorGlow = ps_fluidColor5;
	float glowValue = TEXTURE_READ_2D(TextureSampler, 2, screenUV).a * 2.0f * colorGlow.a;
	colorUp.a = glowValue;
	if(glowValue < 0.01f)
		discard;
  #endif

	Output.RGBColor = colorOff;

  #ifdef COLOR
	Output.RGBColor = In.Diffuse;
  #endif  
  #ifdef TEXTURE  

	//Blur texture
	float4 BlurColor = TEXTURE_READ_2D(TextureSampler, 1, screenUV);

	float  waterValue = BlurColor.a;
	float  speedValue = BlurColor.r;
	float2 flowValue  = BlurColor.gb;

	Output.RGBColor = colorDown;

	//emboss/pseudo reflection/foam
	//kernel :
	//  0   1   1
    // -1   0   1
    // -1  -1   0

	float2 deltaUVEmboss = ps_viewportDimensions.zw * 8.0f;
	float embossValue = 0.0f;
	
	embossValue += TEXTURE_READ_2D(TextureSampler, 1, screenUV + (deltaUVEmboss * float2(1,1))).a;
	embossValue += TEXTURE_READ_2D(TextureSampler, 1, screenUV + (deltaUVEmboss * float2(0,1))).a;
	embossValue += TEXTURE_READ_2D(TextureSampler, 1, screenUV + (deltaUVEmboss * float2(-1,1))).a;
	embossValue -= TEXTURE_READ_2D(TextureSampler, 1, screenUV + (deltaUVEmboss * float2(1,-1))).a;
	embossValue -= TEXTURE_READ_2D(TextureSampler, 1, screenUV + (deltaUVEmboss * float2(0,-1))).a;
	embossValue -= TEXTURE_READ_2D(TextureSampler, 1, screenUV + (deltaUVEmboss * float2(-1,-1))).a;
	embossValue = saturate(embossValue);
	Output.RGBColor = lerp(Output.RGBColor, ps_fluidColor4, embossValue * embossValue); 

	//Color
  	float seuil = ps_fluidParam.x;
	float seuil2 = ps_fluidParam.y;
	float rampA = 1.0f / (seuil-seuil2);
	float rampB = seuil2 / (seuil-seuil2);
	if(waterValue < seuil)
	{
		if(waterValue < seuil2)
		{
			//Glow
		  #ifdef FLUIDGLOW
			Output.RGBColor.rgb = colorGlow.rgb;
			Output.RGBColor.a = glowValue;
			if(Output.RGBColor.a == 0.0f)
				discard;
		  #else
			discard; 
		  #endif
		}
		else
		{
			Output.RGBColor = lerp(colorUp, Output.RGBColor,  waterValue * rampA - rampB); 
		}
	}

	//texture
	//Output.RGBColor.rgb += tex2D(TextureSampler2, (screenUV*8) + BlurColor.gb).rgb;
    //Output.RGBColor.rgb += speedValue.xxx; 

	//Flow part
	float2 s = float2(0.5f, 0.5f);
	float2 marge = ps_fluidParam.zz;
	float2 motif = flowValue;
	motif = (1.0f - abs(motif - s)) - (1.0f - marge);
	motif = saturate(motif * (1.0f / marge) * 2.0f);
	Output.RGBColor.rgba = lerp(Output.RGBColor.rgba, ps_fluidColor3.rgba, motif.x * motif.y * ps_fluidParam.w * Output.RGBColor.a);

	//speed effect
	Output.RGBColor.rgb = lerp(Output.RGBColor.rgb, colorUp.rgb,  speedValue);

	//Output.RGBColor = BlurColor;
	//Output.RGBColor.a = 1.0f;
  #endif

  	Output.RGBColor.xyz = computeFOG(Output.RGBColor.xyz, In);

#ifdef DX11_SHADERS	
	if (ps_alphaTest.r > 0.0f)
    {
	   if (Output.RGBColor.a < ps_alphaTest.g)
	   {
	 	  discard;
	   }
    }
#endif
	
	return Output;
}

//--------------------------------------------------------------------------------------
// Lighting functions

float3 addOmniLight3D(float3 _colorIn, float3 _viewPos, float3 _backTextureColor, PS_OmniLightParam _lightParam)
{
	float3 light = float3(0,0,0);
	if(_lightParam.f4_Radius_useLight.z > 0.0f)
	{
		float3 f3_LDir  = _lightParam.f4_Position.xyz - _viewPos;
		float f_DistSqr = dot( f3_LDir, f3_LDir );
		float f_Atten   = saturate( f_DistSqr * _lightParam.f4_Radius_useLight.x + _lightParam.f4_Radius_useLight.y );

		light = _lightParam.f4_Color.rgb * f_Atten;
	}
	return _colorIn + light * _backTextureColor;	
}

#ifdef TEXTURE
void getDiffuseTexture(out float4 _albedo, in float _vertexAlpha, in float4 _uv, out float4 _backAlbedo)
{
    _albedo = TEXTURE_READ_2D(TextureSampler, 0, _uv.xy);
  #ifdef BLEND_TEXTURE
    float3 albedo2 = TEXTURE_READ_2D(TextureSampler, 2, _uv.xy).rgb;
	_albedo.rgb = lerp(albedo2, _albedo.rgb, _vertexAlpha);
  #endif

  #ifdef SEPARATE_ALPHA
	_albedo.a = TEXTURE_READ_2D(TextureSampler, 4, _uv.zw).a;
  #endif

	_backAlbedo = float4(0.0f, 0.0f, 0.0f, 0.0f);
  #ifdef USE_BACKLIGHT
	_backAlbedo = TEXTURE_READ_2D(TextureSampler, 3, _uv.xy);
	#ifdef BLEND_TEXTURE
	  float3 _backAlbedo2 = TEXTURE_READ_2D(TextureSampler, 5, _uv.xy).rgb;
	  _backAlbedo.rgb = lerp(_backAlbedo.rgb, _backAlbedo2, _vertexAlpha);
	#endif
  #endif
}
#endif

#ifdef LIGHT
float4 getLightedDiffuse(float4 uv, float2 screenPos, float3 viewPos, float2 tangent, float vertexAlpha)
{
	float4 color0;
	float4 color1;
    float4 colorBlended;
	getDiffuseTexture(color0, vertexAlpha, uv, color1);
    
    if(color0.a != 0.f)
    {
    #ifdef _XBOX
        screenPos += ps_tillingVposOffset.xy;
    #endif
        float2 screenUV = screenPos * ps_viewportDimensions.zw;

    #ifdef REFLECTION
        float4 colorReflection = TEXTURE_READ_2D(TextureSampler, 1, screenUV);
        color0.rgb = lerp(color0.rgb, colorReflection.rgb, colorReflection.a * ps_reflectionParam.x);
    #endif

        float selfIllum = 0.0f;
    #ifdef SELF_ILLUM
        selfIllum = color1.a;
    #endif
    
    #ifdef LIGHT3D
        colorBlended.rgb = color0.rgb;
        
      #ifdef USE_BACKLIGHT
        colorBlended.rgb = addOmniLight3D(colorBlended.rgb, viewPos, color1.rgb, ps_light3DParam1);
        colorBlended.rgb = addOmniLight3D(colorBlended.rgb, viewPos, color1.rgb, ps_light3DParam2);
        colorBlended.rgb = addOmniLight3D(colorBlended.rgb, viewPos, color1.rgb, ps_light3DParam3);
      #endif
        colorBlended.a = color0.a; // keep alpha from regular texture
    #else

      #ifdef FRONTLIGHT_CONSTANT
        float3 frontLight = ps_frontLightColor.rgb;
      #else
        float3 frontLight = TEXTURE_READ_2D(FrontLightMaskSampler, 6, screenUV).rgb;
      #endif
        float3 light  = lerp(2.0 * frontLight, float3(1, 1, 1), selfIllum);

        //Front Light Color brightness/contrast
        colorBlended.rgb = color0.rgb * (light * ps_BrightContrast.y + ps_BrightContrast.x);

        colorBlended.a = color0 .a; // keep alpha from regular texture

      #ifdef USE_BACKLIGHT
        if(dot(color1.rgb, float3(1.f, 1.f, 1.f))!= 0.f) // any rgb != 0
        {
          #ifdef BACKLIGHT_CONSTANT
            float3 backLight = ps_backLightColor.rgb;
          #else
            float3 backLight = TEXTURE_READ_2D(BackLightMaskSampler, 7, screenUV).rgb;
          #endif
            
            #ifdef TANGENT
            float2 vecDir = float2(2.0f, 3.0f);
            backLight *= saturate(dot(tangent, vecDir));
            #endif

            //Back Light Color brightness/contrast
            colorBlended.rgb +=  saturate(color1.rgb * ( backLight * ps_BrightContrast.w + ps_BrightContrast.z));

        }
      #endif
    #endif
    }
    else
    {
#ifdef FOGBOX
        discard;
#else // FOGBOX
        colorBlended = color0;
#endif // FOGBOX
    }

    return colorBlended;
}
#endif // LIGHT

//--------------------------------------------------------------------------------------
//PS Main Entry

PS_OUTPUT default_PS(	VS_DEFAULT_OUTPUT In ) 
{ 
    PS_OUTPUT Output;
    
	float4 result = float4(1.0f, 1.0f, 1.0f, 1.0f);

  #ifdef COLOR
	result = In.Diffuse;
  #endif
	
  #ifdef LIGHT3D
    float3 f3_viewPos = In.viewPos.xyz;
  #else
    float3 f3_viewPos = float3(0,0,0);
  #endif
  #ifdef TANGENT
    float2 f2_Tangent = In.Tangent.xy;
  #else
    float2 f2_Tangent = float2(0,0);
  #endif
  
	#ifdef TEXTURE

		float4 texColor;
	  #ifdef LIGHT
		#ifdef DX11_SHADERS
			texColor = getLightedDiffuse(In.TextureUV, In.Position.xy, f3_viewPos, f2_Tangent, result.a);
		#else
			texColor = getLightedDiffuse(In.TextureUV, In.screenPos.xy, f3_viewPos, f2_Tangent, result.a);
		#endif
	  #else
		float4 dumpColor;
		getDiffuseTexture(texColor, result.a, In.TextureUV, dumpColor);
	  #endif
        result *= texColor;	
	  
	  #ifdef BLEND_TEXTURE
		result.a = texColor.a;
	  #endif

	  #ifdef REFLECTION
		#ifdef DX11_SHADERS
			float2 screenPos = In.Position.xy;
		#else
			float2 screenPos = In.screenPos.xy;
		#endif
	    #ifdef _XBOX
		  screenPos += ps_tillingVposOffset.xy;
	    #endif
		  float2 screenUV = screenPos * ps_viewportDimensions.zw;
		#if defined(ITF_PS3)
		  screenUV.y = 1 + screenUV.y;
		#endif
		  float4 colorReflection = TEXTURE_READ_2D(TextureSampler, 1, screenUV);
		  result.xyz = lerp(result.rgb, colorReflection.rgb, colorReflection.a * ps_reflectionParam.x);
	  #endif

	#endif
		
	result.xyz = computeFOG(result.xyz, In);

	#if !defined(ITF_X360) && !defined(ITF_DURANGO)
	
	if(result.a < 0.003921577) // rcp(1/255)
	{
		discard;
	}
	
	#endif // !defined(ITF_X360) && !defined(ITF_DURANGO)   

    Output.RGBColor = result;
    return Output;
}

//--------------------------------------------------------------------------------------
//PS Debug Entry

PS_OUTPUT debug_PS(	VS_DEFAULT_OUTPUT In ) 
{ 
    PS_OUTPUT Output;
    
	float4 result = float4(1.0f, 1.0f, 1.0f, 1.0f);
	float4 uv = float4(1.0f, 1.0f, 1.0f, 1.0f);

	//Compute minimal object color (no fog/no light/...)
  #ifdef COLOR
	result = In.Diffuse;
  #endif
	  
  #ifdef TEXTURE  
	uv = In.TextureUV;
	float4 texColor = TEXTURE_READ_2D(TextureSampler, 0, uv.xy);
	#ifdef SEPARATE_ALPHA
	  texColor.a = TEXTURE_READ_2D(TextureSampler, 4, uv.zw).a;
	#endif
	result *= texColor;		
  #endif

	#if !defined(ITF_X360) && !defined(ITF_DURANGO)

	if(result.a < 0.003921577) // rcp(1/255)
	{
		discard;
	}

	#endif // !defined(ITF_X360) && !defined(ITF_DURANGO)
	
    Output.RGBColor = result;
    return Output;
}
#endif // PIXEL_PROFILE
