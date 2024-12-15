use bevy::render::{
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

#[derive(Default)]
pub struct MeshData {
    pub positions: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl Into<Mesh> for MeshData {
    fn into(self) -> Mesh {
        Mesh::new(PrimitiveTopology::TriangleList, Default::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
            .with_inserted_indices(Indices::U32(self.indices))
            .with_duplicated_vertices()
            .with_computed_flat_normals()
    }
}