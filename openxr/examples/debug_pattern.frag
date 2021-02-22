#version 450
#extension GL_EXT_multiview : require

layout(location = 1) in vec3 vColor;
layout(location = 0) out vec4 color;

void main() {
    color = vec4(vColor.r, (cos(vColor.gb * 4) + 1)/2.0, 1);
}
