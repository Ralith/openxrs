#version 450
#extension GL_EXT_multiview : require

layout(location = 0) in vec3 vPosition;

layout(location = 0) out vec2 screen_coords;
layout(location = 1) out vec3 vColor;

layout(push_constant) uniform ModelTransform {
    mat4 model;
};

struct Transform {
    mat4 view_project;
};

layout(binding = 0) uniform Transforms {
    Transform transform[2];
};

void main()  {
    vec4 world_pos = model * vec4(vPosition, 1.0);
    vec4 pos = transform[gl_ViewIndex].view_project * world_pos;
    gl_Position = pos;
    vColor = vec3(gl_VertexIndex/10.0, world_pos.gb);
}
