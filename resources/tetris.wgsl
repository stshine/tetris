// [[block]]
// struct Index {
//     row: u32;
//     column: u32;
// };

// [[group(0), binding(0)]]
// var<uniform> index: Index;


struct VertexOutput {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] color: u32;
};

[[stage(vertex)]]
fn vs_main(
    [[builtin(instance_index)]] index: u32,
    [[location(0)]]aPos: vec2<u32>,
    [[location(1)]]color: u32,
) -> VertexOutput {
    let ROW_UNIT: f32 = 2.0 / 10.0;
    let COLUMN_UNIT: f32 = 2.0 / 20.0;

    let row = index % 10u;
    let column = index / 10u;
    // let column: u32 = 1;
    // let row: u32 = 1;

    let x = f32(aPos.x + row) * ROW_UNIT - 1.0;
    let y = - f32(aPos.y + column) * COLUMN_UNIT + 1.0;

    var out: VertexOutput;
    out.pos = vec4<f32>(x, y, 0.0, 1.0);
    out.color = color;
    return out;
}

[[stage(fragment)]]
fn fs_main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    if (input.color == 0u) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    } else {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
}