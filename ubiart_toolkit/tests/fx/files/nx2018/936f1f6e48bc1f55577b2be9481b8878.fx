//--------------------------------------------------------------------------------------
//
// File: Debug.fx
//
//--------------------------------------------------------------------------------------

#define CB_DEBUG

#include "PlatformAdapter.fxh"
#include "ShaderParameters.fxh"

#ifdef VERTEX_PROFILE

struct VS_Output
{
    float4 vPos : VS_OUT_POS;
};

VS_Output Debug_main_VS(float4 vPosition : POSITION)
{
	VS_Output output;
    output.vPos = mul(vPosition, vs_mWorldViewProjection);
	return output;
}

#endif // VERTEX_PROFILE

#ifdef PIXEL_PROFILE

float4 Debug_main_PS() : PS_OUT_COLOR
{
    return float4(1, 1 , 1, ps_debugParam.x);
}

#endif // PIXEL_PROFILE