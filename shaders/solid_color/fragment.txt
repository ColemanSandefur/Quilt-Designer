#version 140

in vec4 v_color;
flat in uint v_tex_id;
in vec2 v_tex_coords;

out vec4 f_color;

uniform sampler2DArray tex;

void main() {
    f_color = v_color;

    if (v_tex_id > uint(0)) {
        f_color = texture(tex, vec3(v_tex_coords, float(v_tex_id - uint(1))));
    }
}