use anyhow::{anyhow, Result};
use bevy::ecs::system::Resource;
use bevy::math::Vec3;
use bevy::render::mesh::Mesh;

#[derive(Debug)]
pub struct Triangle(Vec3, Vec3, Vec3);

impl Triangle {
    /// Calculates the area of the triangle.
    pub fn area(&self) -> f32 {
        let a = self.0 - self.1;
        let b = self.1 - self.2;
        let c = self.2 - self.0;
        let a = a.length();
        let b = b.length();
        let c = c.length();
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }

    /// Returns true if the given point lies within the triangle.
    pub fn contains_point(&self, point: Vec3) -> bool {
        let Triangle(a, b, c) = self;
        let area_main = Self(*a, *b, *c).area();
        let area1 = Self(point, *a, *b).area();
        let area2 = Self(point, *b, *c).area();
        let area3 = Self(point, *c, *a).area();
        (area1 + area2 + area3 - area_main).abs() < 1.0
    }
}

#[test]
fn test_triangle_contains_point() {
    let triangle = Triangle(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    assert!(triangle.contains_point(Vec3::new(0.5, 0.5, 0.0)));

    let triangle = Triangle(
        Vec3::new(-116.33986, 18.517029, 18.589056),
        Vec3::new(155.49776, 18.517029, 18.589056),
        Vec3::new(155.49776, 18.517029, 0.74437195),
    );
    assert!(triangle.contains_point(Vec3::new(151.41608, 18.517029, 8.589251)));
}

#[derive(Resource, Debug)]
pub struct NavMesh {
    pub triangles: Vec<Triangle>,
}

impl NavMesh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        Self { triangles }
    }

    pub fn from_mesh(mesh: &Mesh) -> Result<Self> {
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .ok_or(anyhow!("Navmesh has no position!"))?
            .as_float3()
            .ok_or(anyhow!("Navmesh has no position!"))?;
        let indices: Vec<_> = mesh
            .indices()
            .ok_or(anyhow!("Navmesh has no position!"))?
            .iter()
            .collect();
        let triangles = indices
            .chunks_exact(3)
            .map(|chunk| {
                let a = Vec3::from_array(positions[chunk[0]]);
                let b = Vec3::from_array(positions[chunk[1]]);
                let c = Vec3::from_array(positions[chunk[2]]);
                Triangle(a, b, c)
            })
            .collect();
        Ok(Self::new(triangles))
    }

    pub fn contains_point(&self, point: Vec3) -> bool {
        self.triangles
            .iter()
            .any(|triangle| triangle.contains_point(point))
    }
}
