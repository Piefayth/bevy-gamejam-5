#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0) var<uniform> data: vec4<f32>;

const BLACK = vec4<f32>(0., 0., 0., 1.);
const WHITE =  vec4<f32>(1., 1., 1., 1.);
const TRANSPARENT = vec4<f32>(0., 0., 0., 0.);
const EDGE_SIZE = 0.005;
const OUTLINE_THICKNESS = 0.01;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let width = data[0];
    let height = data[1];
    let rotation_radians = data[2];

    let uv: vec2<f32> = 2.0 * mesh.uv - vec2<f32>(1.0, 1.0);
    let box_translation = vec2<f32>(width, 0.);

    let rotated_uv = rotate_around(vec2<f32>(0.), rotation_radians, uv);
    let translated_box = rotated_uv  + box_translation;

    let box_dist = box(translated_box, vec2<f32>(width, height));
    let outline_dist = box(translated_box, vec2<f32>(width + OUTLINE_THICKNESS, height + OUTLINE_THICKNESS));

    let smooth_box = smoothstep(EDGE_SIZE, -EDGE_SIZE, box_dist);
    let smooth_outline = smoothstep(EDGE_SIZE, -EDGE_SIZE, outline_dist);

    var final_color = mix(TRANSPARENT, WHITE, smooth_outline);

    final_color = mix(BLACK, final_color, smooth_box);

    return mix(TRANSPARENT, final_color, smooth_outline);
}
 
fn box(location: vec2<f32>, box: vec2<f32>) -> f32 {
    let distance = abs(location) - box;
    return length(max(distance, vec2<f32>(0.))) + min(max(distance.x, distance.y), 0.0);
}

fn rotate(p: vec2<f32>, angle: f32) -> vec2<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec2<f32>(
        c * p.x - s * p.y,
        s * p.x + c * p.y
    );
}

fn rotate_around(origin: vec2<f32>, angle: f32, point: vec2<f32>) -> vec2<f32> {
    let m = mat2x2<f32>(cos(angle), -sin(angle), sin(angle), cos(angle));
    return (m * (point - origin)) + origin;
}