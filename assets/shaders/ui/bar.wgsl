#import bevy_ui::ui_vertex_output::UiVertexOutput

struct BarMaterial {
    foreground_color: vec4<f32>,
    background_color: vec4<f32>,
    percent: f32,
    border_x: f32,
    border_y: f32,
};

@group(1) @binding(0) var<uniform> bar: BarMaterial;

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
    if (mesh.uv.x <= bar.border_x || mesh.uv.x >= (1 - bar.border_x)) {
        return bar.background_color;
    }
    if (mesh.uv.y <= bar.border_y || mesh.uv.y >= (1 - bar.border_y)) {
        return bar.background_color;
    }

    if (mesh.uv.x <= bar.percent) {
        return bar.foreground_color;
    } else {
        return bar.background_color;
    }
}
