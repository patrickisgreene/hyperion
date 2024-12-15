use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*, utils::HashMap,
};
use hyperion::{grammar::Token, LSystem, LSystemBuilder, Value};
use rand::Rng;

mod mesh;

fn monopodial() -> Result<LSystem<Token>, nom::Err<nom::error::Error<&'static str>>> {
    Ok(LSystemBuilder::new_str("A( 1 , 0.25)")?
        .rule_str("A(l,w) -> F(l,w)[W&(c)B(l*b,w*h)]//(180)[&(d)B(l*e,w*h)")?
        .rule_str("B(l,w) -> F(l,w)[-( d )$C(l*e,w*h)]C(l*b,w*h)")?
        .rule_str("C(l,w) -> WF(l,w)[+(d)$B(l*e , w*h)]B(l*b,w*h)")?
        .variable('b', Value::Num(0.9))
        .variable('e', Value::Num(0.8))
        .variable('c', Value::Num(45.0))
        .variable('d', Value::Num(45.0))
        .variable('h', Value::Num(0.707))
        .build())
}

fn sympodial() -> Result<LSystem<Token>, nom::Err<nom::error::Error<&'static str>>> {
    Ok(LSystemBuilder::new_str("A(1,0.25)")?
        .rule_str("A(l,w) -> F(l,w)[W&(c)B(l*b,w*h)]//(180)[&(d)B(l*e,w*h)")?
        .rule_str("B(l,w)->F(l,w)[+(c)$B(l*b,w*h)][-(d)$B( l * e , w * h )]")?
        .variable('b', Value::Num(0.9))
        .variable('e', Value::Num(0.7))
        .variable('c', Value::Num(5.0))
        .variable('d', Value::Num(65.0))
        .variable('h', Value::Num(0.707))
        .build())
}

fn phyllotaxis() -> Result<LSystem<Token>, nom::Err<nom::error::Error<&'static str>>> {
    Ok(LSystemBuilder::new_str("A(0)")?
        .rule_str("A(n) -> +(a)[f(n^0.5)D]A(n+1)")?
        .variable('a', Value::Num(137.5))
        .build())
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Species {
    Mono,
    Sym
}

fn setup_forest(
    mut mesh_cache: Local<HashMap<(usize, Species), Handle<Mesh>>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let monopodial = monopodial().unwrap();
    let mono_material = materials.add(Color::from(Srgba::GREEN));
    let sympodial = sympodial().unwrap();
    let sym_material = materials.add(Color::from(Srgba::BLUE));

    let mut change = false;
    let layout_sys = phyllotaxis().unwrap();
    let point_cfg = mesh::RenderConfig {
        length: 10.0,
        angle: 135.7,
        width: 0.0,
        resolution: 0
    };
    let point_renderer = mesh::PointRenderer::new(&layout_sys, point_cfg);
    for point in point_renderer.build(11).iter() {
        let (mesh, material) = if change {
            change = false;
            let generation = rand::thread_rng().gen_range(1..12);
            if mesh_cache.contains_key(&(generation, Species::Mono)) {
                (mesh_cache.get(&(generation, Species::Mono)).unwrap().clone(), mono_material.clone())
            } else {
                let _mono_mesh = mesh::Renderer::new(&monopodial, default()).build(generation);
                let mono_mesh = meshes.add(_mono_mesh);
                mesh_cache.insert((generation, Species::Mono), mono_mesh.clone());
                (mono_mesh, mono_material.clone())
            }
        } else {
            change = true;
            let generation = rand::thread_rng().gen_range(1..12);
            if mesh_cache.contains_key(&(generation, Species::Sym)) {
                (mesh_cache.get(&(generation, Species::Sym)).unwrap().clone(), sym_material.clone())
            } else {
                let _sym_mesh = mesh::Renderer::new(&sympodial, default()).build(generation);
                let sym_mesh = meshes.add(_sym_mesh);
                mesh_cache.insert((generation, Species::Sym), sym_mesh.clone());
                (sym_mesh, sym_material.clone())
            }
        };
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::new(point.x, point.z, point.y) * rand::thread_rng().gen_range(0.3..1.0)),
            Wireframe,
        ));
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WireframePlugin))
        .add_plugins(bevy_panorbit_camera::PanOrbitCameraPlugin)
        .add_systems(Startup, (setup_environment, setup_forest))
        .run();
}

fn setup_environment(
    mut commands: Commands,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        bevy_panorbit_camera::PanOrbitCamera::default()
    ));
}
