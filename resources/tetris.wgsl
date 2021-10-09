[[block]]
struct Index {
    row: u32;
    column: u32;
};

[[group(0), binding(0)]]
var<uniform> index: Index;

[[stage(vertex)]]
fn vs_main([[location(0)]]aPos: vec2<u32>) -> [[builtin(position)]] vec4<f32> {
    let ROW_UNIT: f32 = 2.0 / 10.0;
    let COLUMN_UNIT: f32 = 2.0 / 20.0;

    let x = f32(aPos.x + index.row) * ROW_UNIT - 1.0;
    let y = - f32(aPos.y + index.column) * COLUMN_UNIT + 1.0;
    return vec4<f32>(x, y, 0.0, 1.0);
}

[[stage(fragment)]]
fn fs_main([[builtin(position)]] pos: vec4<f32>) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}