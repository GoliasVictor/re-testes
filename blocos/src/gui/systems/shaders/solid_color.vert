#version 140

in vec2 position;
in vec4 color;      
out vec4 vertex_color;

uniform mat4 matrix;

void main() {
	vertex_color = color; 
	gl_Position = matrix * vec4(position, 0.0, 1.0);
}