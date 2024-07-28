#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals

@group(2) @binding(0) var<uniform> inserted_color: vec4<f32>;
@group(2) @binding(1) var<uniform> bevel_color: vec4<f32>;
@group(2) @binding(2) var<uniform> highlight_color: vec4<f32>;
@group(2) @binding(3) var<uniform> data: vec4<f32>; // trigger_time_seconds, trigger_duration_seconds, umodified_trigger_time_seconds, shape_selection
@group(2) @binding(4) var<uniform> data2: vec4<f32>; // time, padding x3

const BLACK = vec4<f32>(0., 0., 0., 1.);
const WHITE =  vec4<f32>(1., 1., 1., 1.);
const TRANSPARENT = vec4<f32>(0., 0., 0., 0.);
const EDGE_SIZE = 0.02;
const THICKNESS = 0.1;
const TRIGGER_INDICATOR_DURATION_SECONDS = 0.25;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let trigger_time = data[0];
    let now = data2[0];
    let trigger_duration = data[1];
    let unmod_trigger_time = data[2]; 
    let shape_selection = data[3]; // integer between 0 - 5

    var elapsed = now - trigger_time;
    elapsed -= trigger_duration * 0.075;  // keep them lit a little longer to prevent hits that LOOK like theyll trigger that dont

    let umod_elapsed = now - unmod_trigger_time;    // used for the trigger indicator, which colors the bevel 

    let progress = clamp(elapsed / trigger_duration, 0.0, 1.0);

    let uv: vec2<f32> = 2.0 * mesh.uv - vec2<f32>(1.0, 1.0);
    
    let bevel_radius = 1.0 - EDGE_SIZE;
    let socket_radius = bevel_radius - THICKNESS;

    // i draw these two circles inside each other
    let bevel_dist = circle(uv, bevel_radius);
    let socket_dist = circle(uv, socket_radius);


    let smooth_bevel = 1.0 - smoothstep(-EDGE_SIZE, EDGE_SIZE, bevel_dist);
    let smooth_socket = smoothstep(-EDGE_SIZE, EDGE_SIZE, socket_dist);

    
    // first mixes, are we in the socket at all?
    var trigger_indicator_color = mix(WHITE, bevel_color, umod_elapsed / TRIGGER_INDICATOR_DURATION_SECONDS);
    var final_color = mix(TRANSPARENT, trigger_indicator_color, smooth_bevel);

    // second mix, are we in the inner socket?
    if (uv.y < 2.0 * progress - 1.0) {
        final_color = mix(inserted_color, final_color, smooth_socket);
    } else {
        final_color = mix(highlight_color, final_color, smooth_socket);
    }

    let darkened_insert_color = vec4<f32>(inserted_color.rgb * 0.25, inserted_color.a);

    var shape_dist: f32 = 0.0;
    switch (i32(round(shape_selection))) {
        case 0: {
            shape_dist = cross(uv, vec2<f32>(0.1, 0.1), socket_radius / 2.);
        }
        case 1: {
            shape_dist = separatedTriangles(uv, socket_radius / 2., 0.1);
        }
        case 2: {
            shape_dist = poundSymbol(uv, socket_radius / 2., 0.15);
        }
        case 3: {
            shape_dist = minusSymbol(uv, socket_radius / 2., 0.1);
        }
        case 4: {
            shape_dist = xSymbol(uv, vec2<f32>(0.1, 0.1), socket_radius / 2.);
        }
        default: {
            shape_dist = 0.;
        }
    }

    let smooth_shape = smoothstep(-EDGE_SIZE, EDGE_SIZE, shape_dist);
    final_color = mix(darkened_insert_color, final_color, smooth_shape);
    
    return final_color;
    
}

fn circle(location: vec2<f32>, radius: f32) -> f32 {
    return length(location) - radius;
}

fn equilateralTriangle(p: vec2<f32>, r: f32) -> f32 {
    let k: f32 = sqrt(3.0);
    var p_mod: vec2<f32> = p;
    p_mod.x = abs(p_mod.x) - r;
    p_mod.y = p_mod.y + r / k;
    
    if (p_mod.x + k * p_mod.y > 0.0) {
        p_mod = vec2<f32>(p_mod.x - k * p_mod.y, -k * p_mod.x - p_mod.y) / 2.0;
    }
    
    p_mod.x -= clamp(p_mod.x, -2.0 * r, 0.0);
    
    return -length(p_mod) * sign(p_mod.y);
}

fn cross(p: vec2<f32>, size: vec2<f32>, thickness: f32) -> f32 {
    // Vertical rectangle
    let vertical = vec2<f32>(thickness, size.y);
    let dv = sdBox(p, vertical);
    
    // Horizontal rectangle
    let horizontal = vec2<f32>(size.x, thickness);
    let dh = sdBox(p, horizontal);
    
    // Combine distances using smooth min function
    return smin(dv, dh, 0.01);
}

// Helper function for rectangle SDF
fn sdBox(p: vec2<f32>, b: vec2<f32>) -> f32 {
    let d = abs(p) - b;
    return length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0);
}

// Smooth min function for smoother blending
fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = max(k - abs(a - b), 0.0) / k;
    return min(a, b) - h * h * k * 0.25;
}

fn separatedTriangles(p: vec2<f32>, size: f32, gap: f32) -> f32 {
    let halfSize: f32 = size * 0.6;
    let halfGap: f32 = gap * 0.25;

    // Adjusting for triangle pointing left (rotate 90 degrees clockwise)
    let leftP = p + vec2<f32>(-halfSize - halfGap, 0.0);
    let leftTriangle = equilateralTriangle(vec2<f32>(-leftP.y, leftP.x), halfSize);

    // Adjusting for triangle pointing right (rotate 90 degrees counterclockwise)
    let rightP = p + vec2<f32>(halfSize + halfGap, 0.0);
    let rightTriangle = equilateralTriangle(vec2<f32>(rightP.y, -rightP.x), halfSize);

    // Return the minimum distance to either triangle
    return min(leftTriangle, rightTriangle);
}

fn poundSymbol(p: vec2<f32>, size: f32, thickness: f32) -> f32 {
    let half_thickness = thickness * 0.5;

    // Define positions for horizontal and vertical bars
    let h_offset = size * 0.4;
    let v_offset = size * 0.4;

    // Horizontal bars
    let h1_pos = vec2<f32>(0.0, -h_offset);
    let h2_pos = vec2<f32>(0.0, h_offset);
    let h_dim = vec2<f32>(size, half_thickness);

    // Vertical bars
    let v1_pos = vec2<f32>(-v_offset, 0.0);
    let v2_pos = vec2<f32>(v_offset, 0.0);
    let v_dim = vec2<f32>(half_thickness, size);

    // Compute SDFs for the four bars
    let h1_dist = sdBox(p - h1_pos, h_dim);
    let h2_dist = sdBox(p - h2_pos, h_dim);
    let v1_dist = sdBox(p - v1_pos, v_dim);
    let v2_dist = sdBox(p - v2_pos, v_dim);

    // Combine distances
    return min(min(h1_dist, h2_dist), min(v1_dist, v2_dist));
}


fn minusSymbol(p: vec2<f32>, length: f32, thickness: f32) -> f32 {
    let horizontal = vec2<f32>(length, thickness);
    return sdBox(p, horizontal);
}

fn xSymbol(p: vec2<f32>, size: vec2<f32>, thickness: f32) -> f32 {
    let p_rotated = vec2<f32>(p.x + p.y, p.x - p.y) * 0.7071; // Rotate 45 degrees
    let diagonal1 = sdBox(p_rotated, vec2<f32>(thickness, size.y));
    let diagonal2 = sdBox(p_rotated.yx, vec2<f32>(thickness, size.x));
    
    return smin(diagonal1, diagonal2, 0.01);
}