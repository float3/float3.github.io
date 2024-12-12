---
title: unity shaders
updated: 2024-11-26
date: 2024-11-26
tags:
  - graphics
  - programming
  - unity
  - shaders
---

unity shader things i've learned over the years

- [dark side of the moon fresnel, orthographic camera issue](#dark-side-of-the-moon-fresnel-orthographic-camera-issue)
- [depth parallax](#depth-parallax)
- [detect reflection probe rendering](#detect-reflection-probe-rendering)
- [warnings](#warnings)
- [whole bunch of helper functions](#whole-bunch-of-helper-functions)
- [better attenuation macro](#better-attenuation-macro)
- [screen space shadow anti aliasing](#screen-space-shadow-anti-aliasing)
- [shadows from world position](#shadows-from-world-position)
- [get camera fov](#get-camera-fov)
- [various vr headset resolutions](#various-vr-headset-resolutions)
- [map uv to screen](#map-uv-to-screen)
- [the forbidden nanmarching technique](#the-forbidden-nanmarching-technique)
- [raymarching with depth](#raymarching-with-depth)
- [camera calculations](#camera-calculations)
- [pbr functions](#pbr-functions)
- [mesh data calculations](#mesh-data-calculations)
- [light calculations](#light-calculations)
- [unity lighting macro usage example:](#unity-lighting-macro-usage-example)

# dark side of the moon fresnel, orthographic camera issue

![dark side of the moon](/misc/media/darkside-of-the-moon.png)

left spheres are my pbr shader, right spheres are unity's standard shader. \
the fresnel effect is not working correctly on the right spheres
because unity doesn't calculate the view direction correctly when using an orthographic camera

this is how to do it correctly:

```hlsl
bool IsOrtho()
{
    return unity_OrthoParams.w == 1 || UNITY_MATRIX_P[3][3] == 1;
}

float3 viewDir = !IsOrtho() ? normalize(_WorldSpaceCameraPos - i.worldPos.xyz) : normalize(UNITY_MATRIX_I_V._m02_m12_m22);
```

# depth parallax

the right one is mine, the left one is a common parallax shader

![depth parallax](/misc/media/depth_parallax.mp4)

if you want to output correct fragment depth when doing parallax follow my example shader here:
https://gist.github.com/float3/dbba6ab34b2605f47cfc56e748bf4ba5

# detect reflection probe rendering

```hlsl
bool isReflectionProbe()
{
    return unity_CameraProjection._m11 == 1 && UNITY_MATRIX_P[0][0] == 1;
}
```

# warnings

```hlsl
#pragma warning (default : 3206) // implicit truncation
```

# whole bunch of helper functions

```hlsl
bool isVR()
{
	// USING_STEREO_MATRICES
	#if UNITY_SINGLE_PASS_STEREO
	return true;
	#else
	return false;
	#endif
}

//UNITY_MATRIX_P._13 < 0 left eye, UNITY_MATRIX_P._13 > 0 right eye & UNITY_MATRIX_P._13 == 0 not vr
bool isLeftEye()
{
	return UNITY_MATRIX_P._13 < 0;
}

bool isRightEye()
{
	return UNITY_MATRIX_P._13 > 0;
}

bool isNotVr()
{
	return UNITY_MATRIX_P._13 == 0;
}

bool isOrtho()
{
	return unity_OrthoParams.w == 1 || UNITY_MATRIX_P[3][3] == 1;
}

float verticalFOV()
{
	return 2.0 * atan(1.0 / unity_CameraProjection._m11) * 180.0 / UNITY_PI;
}

bool isReflectionProbe()
{
	return UNITY_MATRIX_P[0][0] == 1 && unity_CameraProjection._m11 == 1;
}

bool IsNan_float(float In)
{
	return In < 0.0 || In > 0.0 || In == 0.0 ? 0 : 1;
}

bool IsNan_float(float2 In)
{
	return any(In < 0.0) || any(In > 0.0) || any(In == 0.0) ? 0 : 1;
}

bool IsNan_float(float3 In)
{
	return any(In < 0.0) || any(In > 0.0) || any(In == 0.0) ? 0 : 1;
}

bool IsNan_float(float4 In)
{
	return any(In < 0.0) || any(In > 0.0) || any(In == 0.0) ? 0 : 1;
}

#ifdef USING_STEREO_MATRICES
#define _WorldSpaceStereoCameraCenterPos lerp(unity_StereoWorldSpaceCameraPos[0], unity_StereoWorldSpaceCameraPos[1], 0.5)
#else
#define _WorldSpaceStereoCameraCenterPos _WorldSpaceCameraPos
#endif

//invert matrix
float4x4 inverse(float4x4 input)
{
	#define minor(a,b,c) determinant(float3x3(input.a, input.b, input.c))
	//determinant(float3x3(input._22_23_23, input._32_33_34, input._42_43_44))

	const float4x4 cofactors = float4x4(
		minor(_22_23_24, _32_33_34, _42_43_44),
		-minor(_21_23_24, _31_33_34, _41_43_44),
		minor(_21_22_24, _31_32_34, _41_42_44),
		-minor(_21_22_23, _31_32_33, _41_42_43),

		-minor(_12_13_14, _32_33_34, _42_43_44),
		minor(_11_13_14, _31_33_34, _41_43_44),
		-minor(_11_12_14, _31_32_34, _41_42_44),
		minor(_11_12_13, _31_32_33, _41_42_43),

		minor(_12_13_14, _22_23_24, _42_43_44),
		-minor(_11_13_14, _21_23_24, _41_43_44),
		minor(_11_12_14, _21_22_24, _41_42_44),
		-minor(_11_12_13, _21_22_23, _41_42_43),

		-minor(_12_13_14, _22_23_24, _32_33_34),
		minor(_11_13_14, _21_23_24, _31_33_34),
		-minor(_11_12_14, _21_22_24, _31_32_34),
		minor(_11_12_13, _21_22_23, _31_32_33)
	);
	#undef minor
	return transpose(cofactors) / determinant(input);
}

float3 clampLength(float3 v, float l)
{
	return v * min(rsqrt(dot(v, v)) * l, 1);
}

float3 setLength(float3 v, float l)
{
	return v * (rsqrt(dot(v, v)) * l);
}

float3 setLength(float3 v)
{
	return v * (rsqrt(dot(v, v)) * 1);
}

float4 setLength(float4 v)
{
	return v * (rsqrt(dot(v, v)) * 1);
}

float3 setLengthFastSafe(float3 v, float l)
{
	return v * min(1e30, rsqrt(dot(v, v)) * l);
}

float3 fastPosMatMul(float4x4 m, float3 pos)
{
	return m._14_24_34 + m._11_12_13 * pos.x + m._21_22_23 * pos.y + m._31_32_33 * pos.z;
}

//from me
float4x4 worldToViewMatrix()
{
	return UNITY_MATRIX_V;
}

float4x4 viewToWorldMatrix()
{
	return UNITY_MATRIX_I_V;
}

float4x4 viewToClipMatrix()
{
	return UNITY_MATRIX_P;
}

float4x4 clipToViewMatrix()
{
	return inverse(UNITY_MATRIX_P);
}

float4x4 worldToClipMatrix()
{
	return UNITY_MATRIX_VP;
}

float4x4 clipToWorldMatrix()
{
	return inverse(UNITY_MATRIX_VP);
}

float4x4 lookAt(float3 Eye, float3 Center, float3 Up)
{
	float4x4 Matrix;

	float3 X, Y, Z;

	Z = Eye - Center;
	Z = normalize(Z);
	Y = Up;
	X = cross(Y, Z);
	Y = cross(Z, X);

	X = normalize(X);
	Y = normalize(Y);

	Matrix[0][0] = X.x;
	Matrix[1][0] = X.y;
	Matrix[2][0] = X.z;
	Matrix[3][0] = dot(-X, Eye);
	Matrix[0][1] = Y.x;
	Matrix[1][1] = Y.y;
	Matrix[2][1] = Y.z;
	Matrix[3][1] = dot(-Y, Eye);
	Matrix[0][2] = Z.x;
	Matrix[1][2] = Z.y;
	Matrix[2][2] = Z.z;
	Matrix[3][2] = dot(-Z, Eye);
	Matrix[0][3] = 0;
	Matrix[1][3] = 0;
	Matrix[2][3] = 0;
	Matrix[3][3] = 1.0f;

	return Matrix;
}

// clips vec so that it can't be at a angle greater than 90 facing away from the view plane
float3 clipVec(float3 v, float3 r )
{
	float k = dot(v,r);
	return (k>0.0) ? v : (v-r*k)* rsqrt(1.0-k*k/dot(v,v));
}

float4 GetWorldPositionFromDepthValue(float2 uv, float linearDepth)
//Getting the World Coordinate Position by Depth
{
	float camPosZ = _ProjectionParams.y + (_ProjectionParams.z - _ProjectionParams.y) * linearDepth;

	float height = 2 * camPosZ / unity_CameraProjection._m11;
	float width = _ScreenParams.x / _ScreenParams.y * height;

	float camPosX = width * uv.x - width / 2;
	float camPosY = height * uv.y - height / 2;
	float4 camPos = float4(camPosX, camPosY, camPosZ, 1.0);
	return mul(unity_CameraToWorld, camPos);
}

float3 point_quat_rotate( float3 v, float4 quaternion)
{
	return v + 2.0 * cross(quaternion.xyz, cross(quaternion.xyz, v) + quaternion.w * v);
}

float2x2 rot(float angle)
{
	return float2x2(cos(angle), -sin(angle), sin(angle), cos(angle));
}

float GetClipDepthFromDepthValue(float2 uv, float linearDepth)
//Getting the World Coordinate Position by Depth
{
	float4 world = GetWorldPositionFromDepthValue(uv,linearDepth);
	return UnityWorldToClipPos(world.xyz).z;
}

//I don't know how to reverse the density if Fog is linear
float getFogDensity()
{
	#ifdef FOG_EXP2
	return unity_FogParams.x * sqrt(log(2));
	#endif

	#ifdef FOG_EXP
	return unity_FogParams.y * log(2);
	#endif
	return 0;
};


// blend between two directions by %
// https://www.shadertoy.com/view/4sV3zt
// https://keithmaggio.wordpress.com/2011/02/15/math-magician-lerp-slerp-and-nlerp/
float3 slerp(float3 start, float3 end, float percent)
{
	float d     = dot(start, end);
	d           = clamp(d, -1.0, 1.0);
	float theta = acos(d)*percent;
	float3 RelativeVec  = normalize(end - start*d);
	return      ((start*cos(theta)) + (RelativeVec*sin(theta)));
}

/*
float3 LightOrCameraRayToObject(float3 objectPos)
{
	if (UNITY_MATRIX_P[3][3] == 1.0)
	{
		return WorldToObjectNormal(-UNITY_MATRIX_V[2].xyz);
	}
	else
	{
		return objectPos - WorldToObjectPos(UNITY_MATRIX_I_V._m03_m13_m23);
	}
}
*/


// https://github.com/Xiexe/Xiexes-Unity-Shaders/blob/2bade4beb87e96d73811ac2509588f27ae2e989f/Main/CGIncludes/XSHelperFunctions.cginc#L120
half2 calcScreenUVs(float4 screenPos)
{
	half2 uv = screenPos.xy / (screenPos.w + 0.0000000001);
	#if UNITY_SINGLE_PASS_STEREO
	uv.xy *= half2(_ScreenParams.x * 2, _ScreenParams.y);
	#else
	uv.xy *= _ScreenParams.xy;
	#endif

	return uv;
}

inline half Dither8x8Bayer(int x, int y)
{
	const half dither[64] = {
		1, 49, 13, 61, 4, 52, 16, 64,
		33, 17, 45, 29, 36, 20, 48, 32,
		9, 57, 5, 53, 12, 60, 8, 56,
		41, 25, 37, 21, 44, 28, 40, 24,
		3, 51, 15, 63, 2, 50, 14, 62,
		35, 19, 47, 31, 34, 18, 46, 30,
		11, 59, 7, 55, 10, 58, 6, 54,
		43, 27, 39, 23, 42, 26, 38, 22
	};
	int r = y * 8 + x;
	return dither[r] / 65; // Use 65 instead of 64 to get better centering
}

half applyDithering(half alpha, float4 screenPos, half spacing)
{
	half2 screenuv = calcScreenUVs(screenPos).xy;
	half dither = Dither8x8Bayer(fmod(screenuv.x, 8), fmod(screenuv.y, 8));
	return alpha + (0.5 - dither)/spacing;
}

```

# better attenuation macro

```hlsl
// UNITY_LIGHT_ATTENUATION macros without the shadow multiplied in, versions with/without shadow coord interpolator
// Probably should be changed somehow to not rely on EXCLUDE_SHADOW_COORDS
#ifdef POINT
	#define LIGHT_ATTENUATION_NO_SHADOW_MUL(destName, input, worldPos) \
	unityShadowCoord3 lightCoord = mul(unity_WorldToLight, unityShadowCoord4(worldPos, 1)).xyz; \
	float shadow = UNITY_SHADOW_ATTENUATION(input, worldPos); \
	float destName = tex2D(_LightTexture0, dot(lightCoord, lightCoord).rr).r;
#endif
#ifdef SPOT
	#define LIGHT_ATTENUATION_NO_SHADOW_MUL(destName, input, worldPos) \
	DECLARE_LIGHT_COORD(input, worldPos); \
	float shadow = UNITY_SHADOW_ATTENUATION(input, worldPos); \
	float destName = (lightCoord.z > 0) * UnitySpotCookie(lightCoord) * UnitySpotAttenuate(lightCoord.xyz);
#endif
#ifdef DIRECTIONAL
#define LIGHT_ATTENUATION_NO_SHADOW_MUL(destName, input, worldPos) \
	float shadow = UNITY_SHADOW_ATTENUATION(input, worldPos); \
	float destName = 1;
#endif
#ifdef POINT_COOKIE
	#define LIGHT_ATTENUATION_NO_SHADOW_MUL(destName, input, worldPos) \
	DECLARE_LIGHT_COORD(input, worldPos); \
	float shadow = UNITY_SHADOW_ATTENUATION(input, worldPos); \
	float destName = tex2D(_LightTextureB0, dot(lightCoord, lightCoord).rr).r * texCUBE(_LightTexture0, lightCoord).w;
#endif
#ifdef DIRECTIONAL_COOKIE
	#define LIGHT_ATTENUATION_NO_SHADOW_MUL(destName, input, worldPos) \
	DECLARE_LIGHT_COORD(input, worldPos); \
	float shadow = UNITY_SHADOW_ATTENUATION(input, worldPos); \
	float destName = tex2D(_LightTexture0, lightCoord).w;
#endif

```

# screen space shadow anti aliasing

```hlsl
#if defined(SHADOWS_SCREEN) && defined(UNITY_PASS_FORWARDBASE) // fix screen space shadow arficats from msaa

#ifndef HAS_DEPTH_TEXTURE
#define HAS_DEPTH_TEXTURE
sampler2D_float _CameraDepthTexture;
float4 _CameraDepthTexture_TexelSize;
#endif

float SSDirectionalShadowAA(float4 _ShadowCoord, float atten)
{
	float a = atten;
	float2 screenUV = _ShadowCoord.xy / _ShadowCoord.w;
	float shadow = tex2D(_ShadowMapTexture, screenUV).r;

	if (frac(_Time.x) > 0.5)
		a = shadow;

	float fragDepth = _ShadowCoord.z / _ShadowCoord.w;
	float depth_raw = tex2D(_CameraDepthTexture, screenUV).r;

	float depthDiff = abs(fragDepth - depth_raw);
	float diffTest = 1.0 / 100000.0;

	if (depthDiff > diffTest)
	{
		float2 texelSize = _CameraDepthTexture_TexelSize.xy;
		float4 offsetDepths = 0;

		float2 uvOffsets[5] = {
			float2(1.0, 0.0) * texelSize,
			float2(-1.0, 0.0) * texelSize,
			float2(0.0, 1.0) * texelSize,
			float2(0.0, -1.0) * texelSize,
			float2(0.0, 0.0)
		};

		offsetDepths.x = tex2D(_CameraDepthTexture, screenUV + uvOffsets[0]).r;
		offsetDepths.y = tex2D(_CameraDepthTexture, screenUV + uvOffsets[1]).r;
		offsetDepths.z = tex2D(_CameraDepthTexture, screenUV + uvOffsets[2]).r;
		offsetDepths.w = tex2D(_CameraDepthTexture, screenUV + uvOffsets[3]).r;

		float4 offsetDiffs = abs(fragDepth - offsetDepths);

		float diffs[4] = {offsetDiffs.x, offsetDiffs.y, offsetDiffs.z, offsetDiffs.w};

		int lowest = 4;
		float tempDiff = depthDiff;
		for (int i = 0; i < 4; i++)
		{
			if (diffs[i] < tempDiff)
			{
				tempDiff = diffs[i];
				lowest = i;
			}
		}

		a = tex2D(_ShadowMapTexture, screenUV + uvOffsets[lowest]).r;
	}
	return a;
}
#endif
```

# shadows from world position

```hlsl
float sampleShadowMap(float3 worldPos, float distToCam) {
float4 near = float4 (distToCam >= _LightSplitsNear);
float4 far = float4 (distToCam < _LightSplitsFar);
float4 weights = near * far;

// Our world pos MUST be float4 with W coordinate of 1.
float3 shadowCoord0 = mul(unity_WorldToShadow[0], float4(worldPos, 1)).xyz;
float3 shadowCoord1 = mul(unity_WorldToShadow[1], float4(worldPos, 1)).xyz;
 float3 shadowCoord2 = mul(unity_WorldToShadow[2], float4(worldPos, 1)).xyz;
 float3 shadowCoord3 = mul(unity_WorldToShadow[3], float4(worldPos, 1)).xyz;

float3 coord =
    shadowCoord0 * weights.x +     // case: Cascaded one
    shadowCoord1 * weights.y +     // case: Cascaded two
    shadowCoord2 * weights.z +     // case: Cascaded three
    shadowCoord3 * weights.w;     // case: Cascaded four

return UNITY_SAMPLE_SHADOW(_ShadowMapTexture, coord);
}
```

# get camera fov

```hlsl
float getCameraFOV()
{
    float t = unity_CameraProjection._m11;
    const float Rad2Deg = 180 / UNITY_PI;
    float fov = atan(1.0f / t) * 2.0 * Rad2De
    return fov;
}
```

# various vr headset resolutions

```hlsl
#define FORTE_VFX1 253/230

            #define OCULUS_RIFT_DK1 640/800

            #define OCULUS_RIFT_DK2 960/1080
            #define PLAYSTATION_VR 960/1080

            #define HTC_VIVE 1080/1200
            #define OCULUS_RIFT 1080/1200

            #define OCULUS_GO 1280/1440

            #define VALVE_INDEX 1440/1600
            #define OCULUS_QUEST 1440/1600

            #define OCULUS_RIFT_S 1648/1774 //roughly
            #define OCULUS_RIFT_S_VFOV 94.2 //roughly

            #define PIMAX_ARTISAN 1700/1440

            #define OCULUS_QUEST_2 1832/1920
            #define PICO_NEO_3 1832/1920

            #define HP_REVERB_G2 2160/2160

            #define PIMAX_5K_SUPER 2560/1440
            #define VRGINEERS_XTAL 2560/1440

            #define HTC_VIVE_PRO_2 2448/2448
            #define HTC_VIVE_FOCUS_3 2448/2448

            #define ARPARA_VR 2560/2560

            #define VARJO_AERO 2880/2720

            #define PIMAX_VISION_8K_PLUS 3840/2160
            #define PIMAX_VISION_8KX 3840/2160
            #define VRGINEERS_XTAL_8k 3840/2160
            #define PIMAX_REALITY_12K_QLED 5670/3240
```

# map uv to screen

```hlsl
o.pos = float4(float2(1, -1) * (v.uv * 2 - 1), 0, 1);
```

# the forbidden nanmarching technique

this technique allows you to write in screenspace only to pixels that are affected by a light that has a specific layer mask, allowing you to render over a specific layer with your effect

![2022-01-26_23-08-20-reencoded.mp4](/misc/media/2022-01-26_23-08-20-reencoded.mp4)

first you need to create a color swatch to get a nan light, or write a script to set a light to nan, then choose your lights layer mask correctly

%APPDATA%\Unity\Editor-5.x\Preferences\Presets

```
m_Presets:
  - m_Name:
    m_Color: {r: 1, g: 1, b: 1, a: 1}
  - m_Name:
    m_Color: {r: NaN, g: NaN, b: NaN, a: NaN}
  - m_Name:
    m_Color: {r: -0.5, g: -0.5, b: -0.5, a: -0.5}
  - m_Name:
    m_Color: {r: -5, g: -5, b: -5, a: -5}
```

(btw you can also do infinite or negative values here)

then you grabpass and check if the color of the pixel is nan

```hlsl
float4 pixelValue = _MyGrabTex.Load(int3(i.pos.x, i.pos.y, 0));
	if (!any(isnan(asfloat(_Zero ^ asuint(pixelValue)))))
{
	clip(-1);
	discard;
	return (fragOut)0;
}
```

we have to do it in this roundabout way because the compiler assumes that texture samples can't be NaN so if you run isNan on the result of the grabpass that check will be optimized out, therefore we multiply with a uniform (\_Zero)

full example: https://github.com/float3/ShaderArchive/blob/master/Misc/AudioLinkScreenSpaceNaNMarching.shader

# raymarching with depth

https://github.com/float3/ShaderArchive/blob/master/Misc/AudioLinkScreenSpaceNaNMarching.shader

# camera calculations

```hlsl
struct Camera
{
    float NoV;
    float3 viewDir;
};

bool isOrtho()
{
    return unity_OrthoParams.w == 1 || UNITY_MATRIX_P[3][3] == 1;
}

float3 CalculateViewDirection(float3 worldPos)
{
    return !isOrtho()
               ? normalize(_WorldSpaceCameraPos - worldPos)
               : normalize(UNITY_MATRIX_I_V._m02_m12_m22);
}

Camera GetCamera(float3 worldPos, float3 normal)
{
    Camera cameraData;
    cameraData.viewDir = CalculateViewDirection(worldPos);
    cameraData.NoV = dot(cameraData.viewDir, normal);
    return cameraData;
}
```

# pbr functions

```hlsl
float sq(float x)
{
    return x * x;
}


//------------------------------------------------------------------------------
// BRDF configuration
//------------------------------------------------------------------------------

// Diffuse BRDFs
#define DIFFUSE_LAMBERT             0
#define DIFFUSE_BURLEY              1

// Specular BRDF
// Normal distribution functions
#define SPECULAR_D_GGX              0

// Anisotropic NDFs
#define SPECULAR_D_GGX_ANISOTROUNITY_PIC  0

// Cloth NDFs
#define SPECULAR_D_CHARLIE          0

// Visibility functions
#define SPECULAR_V_SMITH_GGX        0
#define SPECULAR_V_SMITH_GGX_FAST   1
#define SPECULAR_V_GGX_ANISOTROUNITY_PIC  2
#define SPECULAR_V_KELEMEN          3
#define SPECULAR_V_NEUBELT          4

// Fresnel functions
#define SPECULAR_F_SCHLICK          0

#define BRDF_DIFFUSE                DIFFUSE_LAMBERT

#define BRDF_SPECULAR_D             SPECULAR_D_GGX
#define BRDF_SPECULAR_V             SPECULAR_V_SMITH_GGX
#define BRDF_SPECULAR_F             SPECULAR_F_SCHLICK

#define BRDF_CLEAR_COAT_D           SPECULAR_D_GGX
#define BRDF_CLEAR_COAT_V           SPECULAR_V_KELEMEN

#define BRDF_ANISOTROUNITY_PIC_D          SPECULAR_D_GGX_ANISOTROUNITY_PIC
#define BRDF_ANISOTROUNITY_PIC_V          SPECULAR_V_GGX_ANISOTROUNITY_PIC

#define BRDF_CLOTH_D                SPECULAR_D_CHARLIE
#define BRDF_CLOTH_V                SPECULAR_V_NEUBELT

//------------------------------------------------------------------------------
// Specular BRDF implementations
//------------------------------------------------------------------------------

float D_GGX(float roughness, float NoH, const float3 h)
{
    // Walter et al. 2007, "Microfacet Models for Refraction through Rough Surfaces"

    // In mediump, there are two problems computing 1.0 - NoH^2
    // 1) 1.0 - NoH^2 suffers floating point cancellation when NoH^2 is close to 1 (highlights)
    // 2) NoH doesn't have enough precision around 1.0
    // Both problem can be fixed by computing 1-NoH^2 in highp and providing NoH in highp as well

    // However, we can do better using Lagrange's identity:
    //      ||a x b||^2 = ||a||^2 ||b||^2 - (a . b)^2
    // since N and H are unit vectors: ||N x H||^2 = 1.0 - NoH^2
    // This computes 1.0 - NoH^2 directly (which is close to zero in the highlights and has
    // enough precision).
    // Overall this yields better performance, keeping all computations in mediump

    float oneMinusNoHSquared = 1.0 - NoH * NoH;

    float a = NoH * roughness;
    float k = roughness / (oneMinusNoHSquared + a * a);
    float d = k * k * (1.0 / UNITY_PI);
    return d;
}

float D_GGX_Anisotropic(float at, float ab, float ToH, float BoH, float NoH)
{
    // Burley 2012, "Physically-Based Shading at Disney"

    // The values at and ab are perceptualRoughness^2, a2 is therefore perceptualRoughness^4
    // The dot product below computes perceptualRoughness^8. We cannot fit in fp16 without clamping
    // the roughness to too high values so we perform the dot product and the division in fp32
    float a2 = at * ab;
    float3 d = float3(ab * ToH, at * BoH, a2 * NoH);
    float d2 = dot(d, d);
    float b2 = a2 / d2;
    return a2 * b2 * b2 * (1.0 / UNITY_PI);
}

float D_Charlie(float roughness, float NoH)
{
    // Estevez and Kulla 2017, "Production Friendly Microfacet Sheen BRDF"
    float invAlpha = 1.0 / roughness;
    float cos2h = NoH * NoH;
    float sin2h = max(1.0 - cos2h, 0.0078125); // 2^(-14/2), so sin2h^2 > 0 in fp16
    return (2.0 + invAlpha) * pow(sin2h, invAlpha * 0.5) / (2.0 * UNITY_PI);
}

float V_SmithGGXCorrelated(float roughness, float NoV, float NoL)
{
    // Heitz 2014, "Understanding the Masking-Shadowing Function in Microfacet-Based BRDFs"
    float a2 = roughness * roughness;
    // TODO: lambdaV can be pre-computed for all the lights, it should be moved out of this function
    float lambdaV = NoL * sqrt((NoV - a2 * NoV) * NoV + a2);
    float lambdaL = NoV * sqrt((NoL - a2 * NoL) * NoL + a2);
    float v = 0.5 / (lambdaV + lambdaL);
    // a2=0 => v = 1 / 4*NoL*NoV   => min=1/4, max=+inf
    // a2=1 => v = 1 / 2*(NoL+NoV) => min=1/4, max=+inf
    // clamp to the maximum value representable in mediump
    return (v);
}

float V_SmithGGXCorrelated_Fast(float roughness, float NoV, float NoL)
{
    // Hammon 2017, "PBR Diffuse Lighting for GGX+Smith Microsurfaces"
    float v = 0.5 / lerp(2.0 * NoL * NoV, NoL + NoV, roughness);
    return (v);
}

float V_SmithGGXCorrelated_Anisotropic(float at, float ab, float ToV, float BoV,
                                       float ToL, float BoL, float NoV, float NoL)
{
    // Heitz 2014, "Understanding the Masking-Shadowing Function in Microfacet-Based BRDFs"
    // TODO: lambdaV can be pre-computed for all the lights, it should be moved out of this function
    float lambdaV = NoL * length(float3(at * ToV, ab * BoV, NoV));
    float lambdaL = NoV * length(float3(at * ToL, ab * BoL, NoL));
    float v = 0.5 / (lambdaV + lambdaL);
    return (v);
}

float V_Kelemen(float LoH)
{
    // Kelemen 2001, "A Microfacet Based Coupled Specular-Matte BRDF Model with Importance Sampling"
    return (0.25 / (LoH * LoH));
}

float V_Neubelt(float NoV, float NoL)
{
    // Neubelt and Pettineo 2013, "Crafting a Next-gen Material Pipeline for The Order: 1886"
    return (1.0 / (4.0 * (NoL + NoV - NoL * NoV)));
}

float3 F_Schlick(const float3 f0, float f90, float VoH)
{
    // Schlick 1994, "An Inexpensive BRDF Model for Physically-Based Rendering"
    return f0 + (f90 - f0) * Pow5(1.0 - VoH);
}

float3 F_Schlick(const float3 f0, float VoH)
{
    float f = pow(1.0 - VoH, 5.0);
    return f + f0 * (1.0 - f);
}

float F_Schlick(float f0, float f90, float VoH)
{
    return f0 + (f90 - f0) * Pow5(1.0 - VoH);
}

//------------------------------------------------------------------------------
// Specular BRDF dispatch
//------------------------------------------------------------------------------

float distribution(float roughness, float NoH, const float3 h)
{
    #if BRDF_SPECULAR_D == SPECULAR_D_GGX
    return D_GGX(roughness, NoH, h);
    #endif
}

float visibility(float roughness, float NoV, float NoL)
{
    #if BRDF_SPECULAR_V == SPECULAR_V_SMITH_GGX
    return V_SmithGGXCorrelated(roughness, NoV, NoL);
    #elif BRDF_SPECULAR_V == SPECULAR_V_SMITH_GGX_FAST
    return V_SmithGGXCorrelated_Fast(roughness, NoV, NoL);
    #endif
}

float3 fresnel(const float3 f0, float LoH)
{
    #if BRDF_SPECULAR_F == SPECULAR_F_SCHLICK
    float f90 = saturate(dot(f0, float3(50.0 * 33.0, 50.0 * 33.0, 50.0 * 33.0)));
    return F_Schlick(f0, f90, LoH);
    #endif
}

float distributionAnisotropic(float at, float ab, float ToH, float BoH, float NoH)
{
    #if BRDF_ANISOTROUNITY_PIC_D == SPECULAR_D_GGX_ANISOTROUNITY_PIC
    return D_GGX_Anisotropic(at, ab, ToH, BoH, NoH);
    #endif
}

float visibilityAnisotropic(float roughness, float at, float ab,
                            float ToV, float BoV, float ToL, float BoL, float NoV, float NoL)
{
    #if BRDF_ANISOTROUNITY_PIC_V == SPECULAR_V_SMITH_GGX
    return V_SmithGGXCorrelated(roughness, NoV, NoL);
    #elif BRDF_ANISOTROUNITY_PIC_V == SPECULAR_V_GGX_ANISOTROUNITY_PIC
    return V_SmithGGXCorrelated_Anisotropic(at, ab, ToV, BoV, ToL, BoL, NoV, NoL);
    #endif
}

float distributionClearCoat(float roughness, float NoH, const float3 h)
{
    #if BRDF_CLEAR_COAT_D == SPECULAR_D_GGX
    return D_GGX(roughness, NoH, h);
    #endif
}

float visibilityClearCoat(float LoH)
{
    #if BRDF_CLEAR_COAT_V == SPECULAR_V_KELEMEN
    return V_Kelemen(LoH);
    #endif
}

float distributionCloth(float roughness, float NoH)
{
    #if BRDF_CLOTH_D == SPECULAR_D_CHARLIE
    return D_Charlie(roughness, NoH);
    #endif
}

float visibilityCloth(float NoV, float NoL)
{
    #if BRDF_CLOTH_V == SPECULAR_V_NEUBELT
    return V_Neubelt(NoV, NoL);
    #endif
}

//------------------------------------------------------------------------------
// Diffuse BRDF implementations
//------------------------------------------------------------------------------

float Fd_Lambert()
{
    return 1.0 / UNITY_PI;
}

float Fd_Burley(float roughness, float NoV, float NoL, float LoH)
{
    // Burley 2012, "Physically-Based Shading at Disney"
    float f90 = 0.5 + 2.0 * roughness * LoH * LoH;
    float lightScatter = F_Schlick(1.0, f90, NoL);
    float viewScatter = F_Schlick(1.0, f90, NoV);
    return lightScatter * viewScatter * (1.0 / UNITY_PI);
}

// Energy conserving wrap diffuse term, does *not* include the divide by pi
float Fd_Wrap(float NoL, float w)
{
    return saturate((NoL + w) / sq(1.0 + w));
}

//------------------------------------------------------------------------------
// Diffuse BRDF dispatch
//------------------------------------------------------------------------------

float diffuse(float roughness, float NoV, float NoL, float LoH)
{
    #if BRDF_DIFFUSE == DIFFUSE_LAMBERT
    return Fd_Lambert();
    #elif BRDF_DIFFUSE == DIFFUSE_BURLEY
    return Fd_Burley(roughness, NoV, NoL, LoH);
    #endif
}
```

# mesh data calculations

```hlsl
struct Mesh
{
    float3 normal;
    float4 tangent;
    float3 bitangent;
    float3x3 tangentToWorld;
};


Mesh CalculateMeshData(float3 normal, float4 tangent, bool facing)
{
    Mesh meshData;
    #ifdef DOUBLESIDED
    UNITY_BRANCH
    if (!facing)
    {
        normal *= -1;
        tangent *= -1;
    }
    #endif
    meshData.normal = normal; // clipIfNeg(normal, CalculateViewDirection(worldPos)); // TODO:
    meshData.tangent = tangent;
    meshData.bitangent = cross(tangent.xyz, normal) * (tangent.w * unity_WorldTransformParams.w);
    meshData.tangentToWorld = float3x3(tangent.xyz, meshData.bitangent, normal);
    return meshData;
}
```

# light calculations

```hlsl
float computeMicroShadowing(float NoL, float visibility)
{
    // Chan 2018, "Material Advances in Call of Duty: WWII"
    float aperture = rsqrt(1.0 - visibility);
    float microShadow = saturate(NoL * aperture);
    return microShadow * microShadow;
}


Light CalcMainLight(Surface surf, v2f input, Camera cameraData, Mesh meshData)
{
    Light MainLight;
    MainLight.direction = normalize(UnityWorldSpaceLightDir(input.worldPos.xyz));
    MainLight.halfVector = normalize(MainLight.direction + cameraData.viewDir);
    MainLight.NoL = saturate(dot(meshData.normal, MainLight.direction));
    MainLight.LoH = saturate(dot(MainLight.direction, MainLight.halfVector));
    MainLight.NoH = saturate(dot(meshData.normal, MainLight.halfVector));
    MainLight.color = _LightColor0.rgb;
    MainLight.intensity = dot(MainLight.color, unity_ColorSpaceLuminance.rgb);
	UNITY_LIGHT_ATTENUATION(lightAttenuation, input, input.worldPos.xyz);
	MainLight.attenuation = lightAttenuation;
    #if defined(SHADOWS_SCREEN) && defined(UNITY_PASS_FORWARDBASE) // fix screen space shadow arficats from msaa
		MainLight.attenuation = SSDirectionalShadowAA(input._ShadowCoord, MainLight.attenuation);
    #endif
    #if defined(SHADOWS_SCREEN) || defined(SHADOWS_DEPTH) || defined(SHADOWS_CUBE)
        if (MainLight.NoL > 0.0) MainLight.attenuation *= computeMicroShadowing(MainLight.NoL, surf.occlusion);
    #endif

    MainLight.colorXatten = MainLight.color * MainLight.attenuation;
    MainLight.irradiance = MainLight.colorXatten * MainLight.NoL;
    MainLight.diffuse = diffuse(surf.roughness, cameraData.NoV, MainLight.NoL, MainLight.LoH);
    MainLight.finalLight = MainLight.NoL * MainLight.attenuation * MainLight.color * MainLight.diffuse;


    MainLight.specular = 0;

    return MainLight;
}
```

# unity lighting macro usage example:

https://github.com/float3/ShaderArchive/blob/master/Misc/AudioLinkScreenSpaceNaNMarching.shader
