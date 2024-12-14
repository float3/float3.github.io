export const DEFAULT_SHADER = "https://www.shadertoy.com/view/4XK3Wt"
export const DEFAULT_SHADER_SOURCE = `void mainImage( out vec4 O, vec2 U ){

    vec2 r = iResolution.xy;
    vec2 uv = U/r;
    vec2 cuv = (2.*U-r)/r.y;
    vec2 muv = (2.*iMouse.xy-r)/r.y;
    
    O = vec4(0);
    float a = aValue;
    for (int i = -int(a); i < int(a); i++){
        O += texture(iChannel0, uv + vec2(0,float(i))/r)*blur(float(i), floor(a));

    }

    O = pow(O, vec4(1./2.2));
}`
