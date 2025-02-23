#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GroundShader {
    camera_x: f32,
    camera_y: f32,
    random: f32,
    time: f32,
};

@group(2) @binding(100)
var<uniform> input: GroundShader;

const ZOOM: f32 = 180.0;
const USE_BEAT_INPUT: bool = true;
const ANIMATE_SHADER: bool = true;
const GRID_LINE_COLOR: vec4f = vec4f(0.0);
const SPACE_COLOR_ALPHA: f32 = 0.3;
const CAMERA_OFFSET: f32 = 0.001953 / 2.0; // This number works well for tiling but I haven't figured out why yet, DON'T CHANGE IT
const GRID_RATIO: f32 = 10.0;

const CONTRAST: f32 = 0.25;
const SATURATION: f32 = 0.5;
const BRIGHTNESS: f32 = 0.5;
const BACKGROUND_COLOR: vec3f = vec3f(0.2, 0.0, 0.4);
const COLOR_1: vec3f = vec3f(0.0, 0.5, 0.0); // GREEN
const COLOR_2: vec3f = vec3f(0.0, 0.5, 1.0); // BLUE
const COLOR_3: vec3f = vec3f(1.0, 1.0, 0.0); // YELLOW
const COLOR_4: vec3f = vec3f(1.0, 0.0, 0.0); // RED

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4f {
    let rand = select(1.0, max(input.random, 0.0001), USE_BEAT_INPUT);
    let camera_x = input.camera_x * CAMERA_OFFSET;
    let camera_y = input.camera_y * -CAMERA_OFFSET;

    let uv = (in.uv.xy + vec2(camera_x, camera_y)) * ZOOM;

    if on_grid_line(uv) {
        return GRID_LINE_COLOR;
    }

    let tile = floor(uv * GRID_RATIO * 0.1);
    var color = tile_color(tile / rand);

    color = mix(color, BACKGROUND_COLOR, 1.0 - CONTRAST);
    color = mix(color, grayscale(color), 1.0 - SATURATION);
    color = mix(color, vec3f(0.0), 1.0 - BRIGHTNESS);
    color *= select(1.0, cos(input.time * 0.5), ANIMATE_SHADER);

    return vec4f(color, SPACE_COLOR_ALPHA);
}

fn grayscale(color: vec3f) -> vec3f {
    let luma = dot(color, vec3f(0.299, 0.587, 0.114));
    return vec3f(luma);
}

fn tile_color(v: vec2f) -> vec3f {
    switch (hash2(v)) {
        case 1u: { return COLOR_1; }
        case 2u: { return COLOR_2; }
        case 3u: { return COLOR_3; }
        case 4u: { return COLOR_4; }
        default: { return BACKGROUND_COLOR; }
    }
}

fn hash2(v: vec2f) -> u32 {
    let pattern_size = 1000.0;
    return u32(sin(cos(v.x) * tan(v.y)) * pattern_size) % 10;
}

fn on_grid_line(uv: vec2f)-> bool {
    let i = step(fract(uv), vec2(1.0 / GRID_RATIO));
    return (1.0 - i.x) * (1.0 - i.y) < 0.5;
}

