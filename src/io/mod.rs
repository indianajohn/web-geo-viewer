use rust_3d::{Face3, Point3D, PointCloud3D};
use std::collections::{HashMap, HashSet};

/// We consider the specular and ambient
/// colors to be properties of the scene
/// lights, not the individual meshes.
#[derive(Clone)]
pub struct Material {
    /// Diffuse color
    pub diffuse_color: three_d::Vec3,

    /// Diffuse intensity
    pub diffuse_intensity: f32,

    /// Specular intensity
    pub specular_intensity: f32,

    /// Name of texture, if there is one.
    pub texture_name: Option<String>,
}

impl Material {
    pub fn new() -> Material {
        Material {
            diffuse_color: three_d::Vec3::new(0.8, 0.8, 0.8),
            diffuse_intensity: 0.5,
            specular_intensity: 0.5,
            texture_name: None,
        }
    }
}

/// Information needed to render a surface.
pub struct MaterialSurface {
    /// Maps a mesh face to a face containing
    /// texture coordinates.
    pub uvs: HashMap<Face3, Face3>,
    /// The faces on the material.
    pub faces: HashSet<Face3>,
    /// Material for this surface.
    pub material: Material,
}

impl MaterialSurface {
    pub fn new() -> MaterialSurface {
        MaterialSurface {
            faces: HashSet::new(),
            uvs: HashMap::new(),
            material: Material::new(),
        }
    }
}

pub struct MaterialInfo {
    /// uvw
    pub uv: PointCloud3D<Point3D>,
    /// mtl:uv
    pub surfaces: HashMap<String, MaterialSurface>,
    /// material files containing information about
    /// surfaces on this mesh.
    pub material_libs: HashSet<String>,
}

impl MaterialInfo {
    pub fn new() -> MaterialInfo {
        MaterialInfo {
            uv: PointCloud3D::<Point3D>::new(),
            surfaces: HashMap::new(),
            material_libs: HashSet::new(),
        }
    }
}

mod ply;
pub use self::ply::*;

mod obj;
pub use self::obj::*;

mod mtl;
pub use self::mtl::*;

mod off;
pub use self::off::*;

mod utils;

mod byte_reader;
