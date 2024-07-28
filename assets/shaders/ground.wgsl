#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GroundShader {
    camera_x: f32,
    camera_y: f32,
    random: f32,
    time: f32,
};

@group(2) @binding(100)
var<uniform> input: GroundShader;

const ZOOM:f32 = 180.0;
const USE_BEAT_INPUT: bool = true;
const ANIMATE_SHADER: bool = true;
const GRID_LINE_COLOR: vec4f = vec4f(0.0);
const SPACE_COLOR_ALPHA: f32 = 0.3;
const CAMERA_OFFSET: f32 = 0.001953 / 2.0; // This number works well for tiling but I haven't figured out why yet, DON'T CHANGE IT
const GRID_RATIO:f32 = 10.;

const CONTRAST_FACTOR: f32 = 1.5;
const SATURATION_FACTOR: f32 = 0.5;
const BRIGHTNESS: f32 = 0.5;
const BACKGROUND_COLOR: vec3f = vec3<f32>(0.2, 0.0, 0.4);
const COLOR_1: vec3f = vec3<f32>(0.0, 0.5, 0.0); // GREEN
const COLOR_2: vec3f = vec3<f32>(0.0, 0.5, 1.0); // BLUE
const COLOR_3: vec3f = vec3<f32>(1.0, 1.0, 0.0); // YELLOW
const COLOR_4: vec3f = vec3<f32>(1.0, 0.0, 0.0); // RED

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let rand = max(select(1.0, input.random, USE_BEAT_INPUT), 0.0001);
    var camera_x = input.camera_x * CAMERA_OFFSET;
    var camera_y = input.camera_y * -CAMERA_OFFSET;
    
    let uv = (in.uv.xy + vec2(camera_x, camera_y)) * ZOOM;

    if is_line(uv) {
        return GRID_LINE_COLOR;
    } 

    let square = floor(uv * GRID_RATIO * 0.1);
    var tile_color = get_color_from_vec2f(square / rand);

    let average_color = average_rgb(tile_color, BACKGROUND_COLOR);
    let contrasted_color = blend_towards(tile_color, average_color, CONTRAST_FACTOR);
    let saturated_color = blend_towards(contrasted_color, rgb_to_grayscale(contrasted_color), SATURATION_FACTOR); 
    let brightened_color = blend_towards(saturated_color, vec3f(0.0), BRIGHTNESS); 

    let final_color = brightened_color * select(1.0, cos(input.time), ANIMATE_SHADER);

    return vec4<f32>(final_color, SPACE_COLOR_ALPHA);
}

fn average_rgb(color1: vec3<f32>, color2: vec3<f32>) -> vec3<f32> {
    return (color1 + color2) / 2.0;
}

fn blend_towards(color: vec3<f32>, toward_color: vec3<f32>, factor: f32) -> vec3<f32> {
    return mix(color, toward_color, factor);
}

fn rgb_to_grayscale(color: vec3<f32>) -> vec3<f32> {
    let grayscale_value = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    return vec3<f32>(grayscale_value, grayscale_value, grayscale_value);
}

fn get_color_from_vec2f(v: vec2<f32>) -> vec3<f32> {
    let index = hash_vec2f(v);
    return get_color(index);
}

fn get_color(index: u32) -> vec3<f32> {
    switch (index) {
        case 1u: { return COLOR_1; }
        case 2u: { return COLOR_2; }
        case 3u: { return COLOR_3; }
        case 4u: { return COLOR_4; }
        default: { return BACKGROUND_COLOR; }
    }
}

fn hash_vec2f(v: vec2<f32>) -> u32 {
    let pattern_size = 1000.0;
    return u32(sin(cos(v.x) * tan(v.y)) * pattern_size) % 10;
}

fn is_line(uv: vec2<f32>)-> bool {
    let i = step(fract(uv), vec2(1.0/GRID_RATIO));
    return ((1.0-i.x) * (1.0-i.y)) < 0.5;
}