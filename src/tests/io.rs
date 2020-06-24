#[cfg(test)]
mod test {
    use crate::io::*;
    use rust_3d::*;
    use std::{fs::File, io::BufReader};
    #[test]
    fn load_untextured_obj() {
        let box_path = "src/tests/data/box.obj".to_string();
        let mut m = rust_3d::Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
        let mut uv = MaterialInfo::new();
        load_obj_mesh(
            &mut BufReader::new(File::open(box_path).unwrap()),
            &mut m,
            &mut uv,
        )
        .unwrap();
        assert_eq!(m.num_vertices(), 8);
        assert_eq!(m.num_faces(), 12);
        assert_eq!(uv.uv.len(), 0);
        assert_eq!(uv.surfaces.contains_key("Default"), true);
        assert_eq!(uv.surfaces.get("Default").unwrap().faces.len(), 12);
    }
    #[test]
    fn load_textured_obj() {
        let box_path = "src/tests/data/capsule.obj".to_string();
        let mut m = rust_3d::Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
        let mut uv = MaterialInfo::new();
        load_obj_mesh(
            &mut BufReader::new(File::open(box_path).unwrap()),
            &mut m,
            &mut uv,
        )
        .unwrap();
        assert_eq!(m.num_vertices(), 5252);
        assert_eq!(m.num_faces(), 10200);
        assert_eq!(uv.uv.len(), 5252);
        assert_eq!(uv.surfaces.contains_key("material0"), true);
        assert_eq!(uv.surfaces.get("material0").unwrap().faces.len(), 10200);
        assert_eq!(uv.material_libs.len(), 1);
        assert_eq!(uv.material_libs.contains("capsule.mtl"), true);
    }
    #[test]
    fn quad_mesh_works() {
        let box_path = "src/tests/data/box_quads.obj".to_string();
        let mut m = rust_3d::Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
        let mut uv = MaterialInfo::new();
        load_obj_mesh(
            &mut BufReader::new(File::open(box_path).unwrap()),
            &mut m,
            &mut uv,
        )
        .unwrap();
        assert_eq!(m.num_vertices(), 8);
        assert_eq!(m.num_faces(), 12);
        assert_eq!(uv.uv.len(), 0);
        assert_eq!(uv.surfaces.contains_key("Default"), true);
        assert_eq!(uv.surfaces.get("Default").unwrap().faces.len(), 0);
    }

    #[test]
    fn load_mtl_works() {
        let mtl_path = "src/tests/data/capsule.mtl".to_string();
        let maybe_materials = load_mtl(&mut BufReader::new(File::open(mtl_path).unwrap()));
        match maybe_materials {
            Ok(materials) => {
                assert_eq!(materials.len(), 1);
                assert_eq!(materials.contains_key("material0"), true);
                if let Some(material) = materials.get("material0") {
                    assert!((material.diffuse_color.x - 0.8).abs() < 1e-6);
                    assert!((material.diffuse_color.y - 0.75).abs() < 1e-6);
                    assert!((material.diffuse_color.z - 0.70).abs() < 1e-6);
                    assert!((material.specular_intensity - 30.0).abs() < 1e-6);
                    match material.texture_name.clone() {
                        Some(texture_name) => {
                            assert_eq!(texture_name, "capsule0.jpg");
                        }
                        None => {
                            assert!(false);
                        }
                    }
                }
            }
            Err(_e) => {
                assert!(false);
            }
        }
    }
}
