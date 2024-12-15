use bevy::prelude::*;
use std::collections::HashMap;

use hyperion::{grammar::Token, LSystem, Module};

use super::{RenderConfig, RenderState};

pub struct Renderer<'a> {
    data: super::MeshData,
    lsys: &'a LSystem<Token>,
    cfg: RenderConfig,
    state: RenderState,
    last_state: (u32, RenderState),
}

impl<'a> Renderer<'a> {
    pub fn new(lsys: &'a LSystem<Token>, cfg: RenderConfig) -> Renderer<'a> {
        Renderer {
            lsys,
            data: Default::default(),
            last_state: (0, (&cfg).into()),
            state: (&cfg).into(),
            cfg,
        }
    }

    pub fn build(mut self, generation: usize) -> Mesh {
        let mut locations = HashMap::new();

        let mut meshes = vec![];

        //meshes.push(self.draw_segment());

        let mut stack = vec![];
        self.last_state = (0, (&self.cfg).into());
        self.state = (&self.cfg).into();
        let tokens = self.lsys.sample(generation);
        for token in tokens {
            match &token.token {
                Token::F => {
                    self.forward(&token, &mut locations, &mut meshes)
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
                Token::External(o) => self.external(o),
                _ => {}
            }
        }

        let mut data: Mesh = meshes.remove(0);
        for m in meshes {
            data.merge(&m);
        }
        data
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

    fn external(&mut self, o: &char) {
        if o.is_uppercase() {
            self.state.up();
        }
    }

    fn forward(
        &mut self,
        token: &Module<Token>,
        locations: &mut HashMap<u32, Vec3>,
        meshes: &mut Vec<Mesh>,
    ) {
        self.state.length(token.params.first());
        self.state.width(token.params.get(1));
        self.state.up();
        let contains_value = locations
            .iter()
            .find(|(_, x)| *x == &self.state.cursor.translation);
        if !contains_value.is_some() {
            let cursor = meshes.iter().map(|x| x.count_vertices()).sum::<usize>() as u32
                + self.data.positions.len() as u32;
            locations.insert(cursor, self.state.cursor.translation);
            meshes.push(self.draw_segment());
            self.last_state = (cursor, self.state.clone());
        } else if let Some((dex, _)) = contains_value {
            self.last_state = (*dex, self.state.clone());
        }
    }

    fn draw_segment(&mut self) -> Mesh {
        let mut data = super::MeshData::default();
        let last_state = self.last_state.1.clone();
        draw_circle(&self.cfg, &mut data, &last_state, &last_state);
        draw_circle(&self.cfg, &mut data, &self.state, &last_state);
        let ring = self.cfg.resolution;
        let last_ring = 0;

        for j in 0..self.cfg.resolution as u32 + 1 {
            data.indices.extend_from_slice(&[
                last_ring + j,
                ring + j,
                ring + j + 1,
                last_ring + j,
                ring + j + 1,
                last_ring + j + 1,
            ]);
        }
        data.into()
    }
}

fn draw_circle(
    cfg: &RenderConfig,
    data: &mut super::MeshData,
    state: &RenderState,
    other: &RenderState,
) {
    let step_theta = std::f32::consts::TAU / cfg.resolution as f32;
    for segment in 0..=cfg.resolution {
        let theta = segment as f32 * step_theta;
        let (sin, cos) = theta.sin_cos();
        let radius = state.width / 2.0;
        let p = Vec3::new(radius * cos, 0.0, radius * sin);
        let mut transformer = state.cursor.clone();
        if state.cursor != other.cursor {       
            let angle = state.cursor.rotation.angle_between(other.cursor.rotation);
            transformer.rotate_axis(state.cursor.local_y().into(), -angle);
        }
        data.positions
            .push(transformer.transform_point(p).to_array());
    }
}