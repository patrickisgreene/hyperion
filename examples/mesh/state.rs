use bevy::prelude::*;
use hyperion::Value;

#[derive(Default, PartialEq, Clone)]
pub struct RenderState {
    pub angle: f32,
    pub length: f32,
    pub width: f32,
    pub cursor: Transform,
}

impl RenderState {
    pub fn angle(&mut self, angle: Option<&Value>) {
        match angle {
            Some(Value::Num(angle)) => self.angle = *angle,
            Some(value) => error!("Attempted to set LSystem angle to non numeric value: {value:?}"),
            None => {}
        }
    }
    pub fn width(&mut self, width: Option<&Value>) {
        match width {
            Some(Value::Num(width)) => self.width = *width,
            Some(value) => error!("Attempted to set LSystem width to non numeric value: {value:?}"),
            None => {}
        }
    }
    pub fn length(&mut self, length: Option<&Value>) {
        match length {
            Some(Value::Num(length)) => self.length = *length,
            Some(value) => {
                error!("Attempted to set LSystem length to non numeric value: {value:?}")
            }
            None => {}
        }
    }
    pub fn up(&mut self) {
        self.cursor.translation += self.cursor.up() * self.length;
    }
}