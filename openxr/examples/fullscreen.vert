#version 450

layout(location = 0) out vec2 screen_coords;

void main()  {
    screen_coords = vec2((gl_VertexIndex << 1) & 2, gl_VertexIndex & 2);
    gl_Position = vec4(screen_coords * 2.0f + -1.0f, 0.0, 1.0f);
}
