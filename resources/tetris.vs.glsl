#version 450 core

const float ROW_UNIT = 2 / 10.0;
const float COLUMN_UNIT = 2 / 20.0;

layout(location=0) in vec2 aPos;

uniform int row;
uniform int column;

void main() {
    float x = (row + aPos.x) * ROW_UNIT - 1;
    float y = (column + aPos.y) * COLUMN_UNIT - 1;
    gl_Position = vec4(x, y, 0, 1.0);
}