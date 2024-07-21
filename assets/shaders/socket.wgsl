#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0) var<uniform> inserted_color: vec4<f32>;
@group(2) @binding(1) var<uniform> bevel_color: vec4<f32>;

const BLACK = vec4<f32>(0., 0., 0., 1.);
const WHITE =  vec4<f32>(1., 1., 1., 1.);
const TRANSPARENT = vec4<f32>(0., 0., 0., 0.);
const EDGE_SIZE = 0.005;
const THICKNESS = 0.1;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv: vec2<f32> = 2.0 * mesh.uv - vec2<f32>(1.0, 1.0);
    
    let bevel_radius = 1.0 - EDGE_SIZE;
    let socket_radius = bevel_radius - THICKNESS;

    let bevel_dist = circle(uv, bevel_radius);
    let socket_dist = circle(uv, socket_radius);

    let smooth_bevel = 1.0 - smoothstep(-EDGE_SIZE, EDGE_SIZE, bevel_dist);
    let smooth_socket = smoothstep(-EDGE_SIZE, EDGE_SIZE, socket_dist);
    
    // first mix, are we in the socket at all?

    var final_color = mix(TRANSPARENT, bevel_color, smooth_bevel);

    // second mix, are we in the inner socket?

    final_color = mix(inserted_color, final_color, smooth_socket);

    return final_color;
    
}

fn circle(location: vec2<f32>, radius: f32) -> f32 {
    return length(location) - radius;
}