#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0) var<uniform> data: vec4<f32>; // radius, thickness, progress, padding

const BLACK = vec4<f32>(0., 0., 0., 1.);
const WHITE =  vec4<f32>(1., 1., 1., 1.);
const TRANSPARENT = vec4<f32>(0., 0., 0., 0.);
const RING_COLOR = vec4<f32>(0., 0., 0., 1.);
const EDGE_SIZE = 0.005;
const INDICATOR_THICKNESS = 0.005;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let radius = data[0];
    let thickness = data[1];
    let progress = 1. - data[2]; // between 0 and 1 representin gthe progress around the ring

    let uv: vec2<f32> = 2.0 * mesh.uv - vec2<f32>(1.0, 1.0);

    let ring_visible = circle(uv, radius);
    let ring_mask = circle(uv, radius - thickness);
    
    let smoothed_visible_portion = smoothstep(EDGE_SIZE, -EDGE_SIZE, ring_visible);
    let smoothed_mask = smoothstep(EDGE_SIZE, -EDGE_SIZE, ring_mask);
    
    let ring_dist = smoothed_visible_portion - smoothed_mask;

    // +0.5 to normalize the angle into the [0, 1] range
    // -0.25 to rotate starting position by 90 degrees (because 1 = 360 in this model)
    // fract to re-normalize the angle back into the [0, 1] range
    let start_angle = fract(0.5 + atan2(uv.y, uv.x) / (2.0 * 3.1415) - 0.25);

    let progress_indicator_start = fract(start_angle + progress + INDICATOR_THICKNESS);
    let progress_indicator_end = fract(start_angle + progress - INDICATOR_THICKNESS);

    
    let is_in_progress_segment = step(progress_indicator_start, progress_indicator_end);

    let color = mix(RING_COLOR, WHITE, is_in_progress_segment * ring_dist);

    return mix(TRANSPARENT, color, ring_dist);
    
}

fn circle(location: vec2<f32>, radius: f32) -> f32 {
    return length(location) - radius;
}