#version 400
attribute vec2 position;
varying vec2 v_tex_coords;

void main() {
    gl_Position = vec4(position, 1.0, 1.0);

    vec2 tex_coords = position;

    tex_coords.x = (tex_coords.x + 1.0) / 2.0;
    tex_coords.y = (tex_coords.y + 1.0) / 2.0;

    v_tex_coords = tex_coords;
}