#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0) var<uniform> base_color: vec4<f32>;
@group(2) @binding(1) var<uniform> blend_color: vec4<f32>;


fn hash(n: vec2<f32>) -> f32 {
    let x = sin(dot(n, vec2<f32>(12.9898, 78.233))) * 43758.5453;
    return fract(x);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i + vec2<f32>(0.0, 0.0)), hash(i + vec2<f32>(1.0, 0.0)), u.x),
               mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x), u.y);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let center: vec2<f32> = vec2<f32>(0.0, 0.0);
    let radius: f32 = 600.0;

    let dist: f32 = distance(mesh.world_position.xy, center);

    let gray_scale: f32 = smoothstep(0.0, radius, dist);

    let noise_factor: f32 = 0.05 * noise(mesh.world_position.xy * 10.0);
    let final_gray_scale: f32 = clamp(gray_scale + noise_factor, 0.0, 1.0);

    let color_blend: vec3<f32> = base_color.rgb * (1.0 - final_gray_scale) + blend_color.rgb * final_gray_scale;

    return vec4<f32>(color_blend, 1.0);
}