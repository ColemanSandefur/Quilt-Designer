#version 400

uniform int samples;
uniform vec2 resolution;
uniform sampler2DMS tex;

varying vec2 v_tex_coords;

vec4 textureMultisample(sampler2DMS sampler, ivec2 coord) {
    vec4 color = vec4(0.0);

    for (int i = 0; i < samples; i++) {
        color += texelFetch(sampler, coord, i);
    }

    color /= float(samples);

    return color;
}

void main() {
    ivec2 texture_size = textureSize(tex);
    ivec2 coord = ivec2(v_tex_coords * texture_size);
    vec4 color = textureMultisample(tex, coord);
    gl_FragColor = color;
}