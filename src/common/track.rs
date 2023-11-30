use std::cmp::Ordering;

use anyhow::{anyhow, Result};
use bevy::{math::Vec3, render::mesh::Mesh};

#[derive(Debug)]
pub struct Track(Vec<Vec3>);

impl Track {
    pub fn from_mesh(mesh: &Mesh) -> Result<Self> {
        let points: Vec<Vec3> = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .ok_or(anyhow!("Navmesh has no position!"))?
            .as_float3()
            .ok_or(anyhow!("Navmesh has no position!"))?
            .iter()
            .map(|point| Vec3::from_array(*point))
            .collect();
        let min_y = points
            .iter()
            .map(|point| point.y)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
            .unwrap();
        let mut points: Vec<Vec3> = points
            .into_iter()
            .map(|point| Vec3::new(point.x, min_y, point.z))
            .collect();
        points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        points.dedup();
        Ok(Self(points))
    }

    pub fn first(&self) -> Vec3 {
        *self.0.first().expect("empty track error")
    }
}
