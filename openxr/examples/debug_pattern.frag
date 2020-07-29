#version 450
#extension GL_EXT_multiview : require

layout(location = 0) in vec2 screen_coords;
layout(location = 0) out vec4 color;

void main() {
    color = vec4(screen_coords, gl_ViewIndex, 1);
}
