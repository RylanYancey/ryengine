
#version 140

in vec2 position;

uniform vec2 translation;
uniform vec2 perspective;

void main() {
    gl_Position = vec4((translation + position) - perspective, 0.0, 1.0);
}