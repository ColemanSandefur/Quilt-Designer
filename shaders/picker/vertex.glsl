#version 140

in vec2 position;
in vec4 color;
in mat4 model;
in mat4 rotation;
in uint id;

flat out uint v_id;

uniform mat4 view;
uniform mat4 projection;
uniform mat4 rotation_point;

void main() {
    v_id = id;

    mat4 inverse_rotation = rotation_point;

    inverse_rotation[3][0] = -rotation_point[3][0];
    inverse_rotation[3][1] = -rotation_point[3][1];

    gl_Position = projection * view * model * rotation_point * rotation * inverse_rotation * vec4(position, 1.0, 1.0);
}