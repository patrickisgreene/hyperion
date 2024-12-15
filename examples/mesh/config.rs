#[derive(Clone)]
pub struct RenderConfig {
    pub angle: f32,
    pub width: f32,
    pub length: f32,
    pub resolution: u32,
}

impl Default for RenderConfig {
    fn default() -> RenderConfig {
        RenderConfig {
            angle: 16.0,
            width: 0.5,
            length: 0.5,
            resolution: 6
        }
    }
}

impl<'a> Into<super::RenderState> for &'a RenderConfig {
    fn into(self) -> super::RenderState {
        let RenderConfig {
            length,
            width,
            angle,
            ..
        } = self;
        super::RenderState {
            length: *length,
            width: *width,
            angle: *angle,
            cursor: Default::default(),
        }
    }
}