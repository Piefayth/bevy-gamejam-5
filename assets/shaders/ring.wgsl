#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0) var<uniform> radius: f32;
@group(2) @binding(1) var<uniform> thickness: f32;

const BLACK = vec4<f32>(0., 0., 0., 1.);
const WHITE =  vec4<f32>(1., 1., 1., 1.);
const TRANSPARENT = vec4<f32>(0., 0., 0., 0.);
const RING_COLOR = vec4<f32>(0., 0., 0., 1.);
const EDGE_SIZE = 0.005;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv: vec2<f32> = 2.0 * mesh.uv - vec2<f32>(1.0, 1.0);

    let ring_visible = circle(uv, radius);
    let ring_mask = circle(uv, radius - thickness);
    
    let smoothed_visible_portion = smoothstep(EDGE_SIZE, -EDGE_SIZE, ring_visible);
    let smoothed_mask = smoothstep(EDGE_SIZE, -EDGE_SIZE, ring_mask);
    
    let ring_dist = smoothed_visible_portion - smoothed_mask;

    let ring = mix(TRANSPARENT, RING_COLOR, ring_dist);

    return ring;
    
}

fn circle(location: vec2<f32>, radius: f32) -> f32 {
    return length(location) - radius;
}