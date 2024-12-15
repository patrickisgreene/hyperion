use bevy::prelude::*;

use hyperion::{grammar::Token, LSystem, Module, Value};

use super::{RenderConfig, RenderState};

pub struct PointRenderer<'a> {
    lsys: &'a LSystem<Token>,
    cfg: RenderConfig,
    state: RenderState,
    last_state: (u32, RenderState),
}

impl<'a> PointRenderer<'a> {
    pub fn new(lsys: &'a LSystem<Token>, cfg: RenderConfig) -> PointRenderer<'a> {
        PointRenderer {
            lsys,
            last_state: (0, (&cfg).into()),
            state: (&cfg).into(),
            cfg,
        }
    }

    pub fn build(mut self, generation: usize) -> Vec<Vec3> {

        let mut  points = vec![];

        points.push(self.state.cursor.translation);

        let mut stack = vec![];
        self.last_state = (0, (&self.cfg).into());
        self.state = (&self.cfg).into();
        let tokens = self.lsys.sample(generation);
        for token in tokens {
            match &token.token {
                Token::F => {
                    self.forward(&token, &mut points)
                }
                Token::Push => stack.push((self.last_state.clone(), self.state.clone())),
                Token::Pop => {
                    if let Some((lstate, s)) = stack.pop() {
                        self.last_state = lstate;
                        self.state = s;
                    }
                }
                Token::Rotate => self.rotate(),
                Token::Left => self.math(&token, |t, arg| t.rotate_local_z(arg)),
                Token::Right => self.math(&token, |t, arg| t.rotate_local_z(-arg)),
                Token::Down => self.math(&token, |t, arg| t.rotate_local_x(-arg)),
                Token::Up => self.math(&token, |t, arg| t.rotate_local_x(arg)),
                Token::Roll => self.math(&token, |t, arg| t.rotate_local_y(arg)),
                Token::CounterRoll => self.math(&token, |t, arg| t.rotate_local_y(-arg)),
                Token::External(o) => self.external(o, &token.params, &mut points),
                _ => {}
            }
        }

        points
    }

    fn rotate(&mut self) {
        self.state
            .cursor
            .rotate_local(Quat::from_euler(EulerRot::XYZ, 0.0, 1.0, 0.0))
    }

    fn math<F: Fn(&mut Transform, f32)>(&mut self, token: &Module<Token>, f: F) {
        self.state.angle(token.params.first());
        f(&mut self.state.cursor, self.state.angle.to_radians());
    }

    fn external(&mut self, o: &char, params: &Vec<Value>, points: &mut Vec<Vec3>) {
        if o.is_uppercase() {
            self.state.length(params.first());
            self.state.width(params.get(1));
            self.state.up();
            points.push(self.state.cursor.translation);
        }
    }

    fn forward(
        &mut self,
        token: &Module<Token>,
        points: &mut Vec<Vec3>,
    ) {
        self.state.length(token.params.first());
        self.state.width(token.params.get(1));
        self.state.up();
        println!("{:?}", self.state.cursor.translation);
        points.push(self.state.cursor.translation);
    }

}
