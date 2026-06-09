#pragma name "Decoration/Default"
#pragma vertex vertSkin
#pragma fragment fragSkin
#pragma cull back
#pragma property texture2D _MainTex
#pragma property vector _MainTex_ST (1,1,0,0)
#pragma property color _Color (1)
#pragma property color _CustomCol (1)

#pragma property range(0, 1) _Metallic 0
#pragma property range(0, 1) _Smoothness 0.5

#pragma shader_feature USE_ALPHA_CLIP
#pragma property range(0,1) _AlphaThreadHold 0.5

#pragma shader_feature USE_NORMAL_TEX
#pragma property texture2D _NormalTex
#pragma property vector _NormalTex_ST (1,1,0,0)
#pragma property range(-5, 5) _NormalStrength 1

#pragma shader_feature USE_EMISSION_TEX
#pragma property texture2D _EmissiveTex
#pragma property vector _EmissiveTex_ST (1,1,0,0)
#pragma property color _EmissiveColor (0,0,0,1)
#pragma property range(0,5) _EmissiveStrength 1

#pragma using MatrixM,MatrixVP, MatrixMVP, DLightDirection, DLightColor, AmbientColor, ViewPosition
#pragma shader_feature_base
#pragma shader_feature_shadow
#pragma shader_feature_fog
#pragma shader_feature_additional_light
#pragma shader_feature_batch_instance
#pragma shader_feature_lightmap
#pragma shader_feature_shader_level

#include "pbr_data.h"
#include "pbr_static_batch_input.h"
#include "lightmap.h"
#include "gamma_correction.h"

#ifdef USE_ADDITIONAL_LIGHT
#include "additional_light_data.h"
#include "additional_light.h"
void FillLightDataFromAdditionalLight(AdditionalLightData adLightData,float3 wpos,float3 wnormal, inout LightData data)
{
    float3 lightVector = adLightData.wpos - wpos;
    data.light_dir 		= normalize(lightVector);
    float sqrDistance = dot(lightVector,lightVector);
    half atten = DistanceAttenuation(sqrDistance,adLightData.args.y);
    atten *= SpotLightAngleAttenuation(adLightData ,data.light_dir);
    atten *= GetAdditionalRealtimeShadow(adLightData,wpos,wnormal,data.light_dir);
	data.light_color 	= adLightData.color * atten;
} 

half3 GetAddictionalLightColor(float3 wpos,float3 wnormal,inout LightData data)
{
    half3 c = vec3(0.0f,0.0f,0.0f);

    for(int i = 0 ; i < ALIGHT_COUNT ; ++i)
    {
        AdditionalLightData adLightData = GetAdditionalLightData(i);
        FillLightDataFromAdditionalLight(adLightData,wpos,wnormal,data);
        PostFillLightData(data);
        half3 color = BRDF_Diffuse(data) + saturate(BRDF_Specular(data));
        c += color;
    }
    return c;
}
#endif

half4 GetCustomCol()
{
	#ifdef _STATIC_BATCH_INSTANCE_
	uint instanceIndex = BuildinBatchInstanceIndex;
	return _CustomCol[instanceIndex];
	#else
	return _CustomCol;
	#endif
}

v2f vertSkin(vrt i) {
    v2f v;
    v.pos = ObjectToClipPos(i.pos);
    v.worldNormal = normalize(mat3(MatrixM) * i.normal);
    v.worldPos = (MatrixM * i.pos).xyz;


    v.mainTexUV = TransfromMainTexUV(i.uv);

#if defined(USE_DIRECTIONAL_LIGHT) || defined(USE_ADDITIONAL_LIGHT) || defined(_DIRECTIONAL_LIGHTMAP_)
    #ifdef USE_NORMAL_TEX
    v.normalTexUV.xy = TransfromNormalTexUV(i.uv);

    float3 worldTangent = mat3(MatrixM) * i.tangent.xyz;
    float3 worldBinormal = normalize(cross(worldTangent,v.worldNormal ) * i.tangent.w);
    v.T = normalize(worldTangent);
    v.B = normalize(worldBinormal);
    #endif
#endif


#ifdef USE_EMISSION_TEX
    v.emissiveTexUV = TransfromEmissiveTexUV(i.uv);
#endif

    return v;
}

half4 fragSkin(v2f f) {

    vec2 uv     = f.mainTexUV;

    half4 customCol = GetCustomCol();
    
    half4 color  = GetColor() * SampleMainTex(uv);
    half temp = step(0.3,color.a);
    half3 texCol = lerp(lerp(customCol.rgb,color.rgb,float3(customCol.a)),color.rgb,float3(temp));
    color = float4(texCol, color.a);


#ifdef SHADER_LEVEL_LOW
    
    float difLight = dot(f.worldNormal, DLightDirection.xyz);
    float halfLambert = difLight * 0.5 + 0.5;

    vec3 diffuse = color.rgb * DLightColor.rgb * halfLambert;
    vec4 result = vec4(diffuse + AmbientColor.rgb, 1.0);
    return result;
#else

    // vec2 uv     = f.mainTexUV;
    // half4 color  = GetColor() * SampleMainTex(uv);
#ifdef USE_ALPHA_CLIP
    clip(color.a - GetAlphaThreadHold() - 0.000001);
#endif

    float3 worldNormal = f.worldNormal;
#if defined(USE_DIRECTIONAL_LIGHT) || defined(USE_ADDITIONAL_LIGHT) || defined(_DIRECTIONAL_LIGHTMAP_)
    #ifdef USE_NORMAL_TEX
    float3 texNormal = SampleNormalTex(f.normalTexUV).xyz;
    float2 tangentNormalXY = (texNormal.xy * 2.0 - float2(1.0)) * GetNormalStrength();
    float3 tangentNormal = normalize(float3(tangentNormalXY,1.));

    float3x3 TBN = float3x3(f.T, f.B, worldNormal);
    worldNormal = TBN * tangentNormal;
    #endif
#endif

    half4 emission = GetEmissiveColor();
#ifdef USE_EMISSION_TEX    
    emission *= SampleEmissiveTex(f.emissiveTexUV);
#endif
    emission = float4(emission.rgb * emission.a * GetEmissiveStrength(),1.);

    half3 c = emission.rgb;
#ifdef _USE_LIGHTMAP_

    c+= color.rgb * pow(SampleLightmap(worldNormal), float3(0.4166667));
#else
    c += AmbientColor.xyz * color.xyz;
#endif


#if defined (USE_DIRECTIONAL_LIGHT) || defined(USE_ADDITIONAL_LIGHT)
    LightData data;
    float perceptualRoughness = (1.0 - GetSmoothness());
    data.color 			= color.xyz;
    data.metallic 		= GetMetallic();
    data.roughness 		= max(perceptualRoughness * perceptualRoughness, 0.002);
    data.view_dir 		= normalize(ViewPosition.xyz - f.worldPos);
    data.normal 		= worldNormal;
    data.NdotV 			= saturate(dot(data.normal, data.view_dir));
#endif

#ifdef USE_DIRECTIONAL_LIGHT
    half shadow         = SHADOW_ATTENUATION;
    data.light_color 	= DLightColor.rgb * shadow;
    data.light_dir 		= DLightDirection.xyz;
    
    data.half_way 		= normalize(data.view_dir + data.light_dir);
    data.NdotL 			= saturate(dot(data.normal, data.light_dir));
    
    data.VdotH 			= saturate(dot(data.view_dir, data.half_way));
    data.NdotH 			= saturate(dot(data.normal, data.half_way));
    data.LdotH 			= saturate(dot(data.light_dir, data.half_way));

    c += BRDF_Diffuse(data) + saturate(BRDF_Specular(data)) ;

#endif

#ifdef USE_ADDITIONAL_LIGHT
    c += GetAddictionalLightColor(f.worldPos,worldNormal,data);
#endif

    ApplyFog(c);

    return vec4(c,1.0);
#endif
}

