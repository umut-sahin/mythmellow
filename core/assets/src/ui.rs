//! User interface assets.

use mythmallow_core_dependencies::*;


/// Material for the bars of the head-up display.
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct BarMaterial {
    /// Foreground color of the bar.
    #[uniform(0)]
    pub foreground_color: Vec4,

    /// Background color of the bar.
    #[uniform(0)]
    pub background_color: Vec4,

    /// Percentage of the bar that is filled.
    #[uniform(0)]
    pub percent: f32,

    /// Percentage of the border of the bar in the x-axis.
    #[uniform(0)]
    pub border_x: f32,

    /// Percentage of the border of the bar in the y-axis.
    #[uniform(0)]
    pub border_y: f32,
}

impl BarMaterial {
    /// Creates the material of a health bar.
    pub fn for_health() -> BarMaterial {
        BarMaterial {
            foreground_color: Vec4::new(1.00, 0.00, 0.00, 1.00),
            background_color: Vec4::new(0.00, 0.00, 0.00, 1.00),
            percent: 1.00,
            border_x: 0.025,
            border_y: 0.125,
        }
    }

    /// Creates the material of an experience bar.
    pub fn for_experience() -> BarMaterial {
        BarMaterial {
            foreground_color: Vec4::new(0.00, 1.00, 0.00, 1.00),
            background_color: Vec4::new(0.00, 0.00, 0.00, 1.00),
            percent: 0.00,
            border_x: 0.025,
            border_y: 0.125,
        }
    }
}

impl UiMaterial for BarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ui/bar.wgsl".into()
    }
}
