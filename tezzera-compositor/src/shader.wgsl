// Fullscreen-quad compositor shader (D075, D080-D081).
//
// Vertex stage generates two triangles covering the entire clip space from
// vertex_index — no vertex buffer needed. UV origin (0,0) is top-left so it
// matches tiny-skia's pixel layout directly.
//
// The optional `u_offset` uniform shifts the UV sample position for GPU-side
// scroll transforms (D081). With offset=(0,0) this is identical to Phase 15/16.
// Out-of-range UV returns transparent (alpha=0), revealing the layer below.

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0)       uv:            vec2<f32>,
};

struct LayerUniforms {
    // UV-space offset: offset_pixels / texture_size.
    // Positive offset_x scrolls the texture left; positive offset_y scrolls up.
    offset: vec2<f32>,
    // Pad to 16 bytes (wgpu uniform alignment requirement).
    _pad:   vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOutput {
    // Two triangles (CCW): 6 vertices for a full-screen quad.
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0, -1.0),
    );
    // UV: clip-space Y is flipped relative to texture Y (origin top-left).
    var uv = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 1.0),
    );
    var out: VertexOutput;
    out.clip_position = vec4<f32>(pos[idx], 0.0, 1.0);
    out.uv             = uv[idx];
    return out;
}

@group(0) @binding(0) var t_frame:  texture_2d<f32>;
@group(0) @binding(1) var s_frame:  sampler;
@group(0) @binding(2) var<uniform> u_layer: LayerUniforms;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv + u_layer.offset;
    // Return transparent for out-of-range UV (D081 — content boundary).
    if uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    return textureSample(t_frame, s_frame, uv);
}
