#version 140
flat in uint v_id;
out uint f_id;
void main() {
    f_id = v_id;
}