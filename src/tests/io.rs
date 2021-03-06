#[cfg(test)]
mod test {
    use crate::io::*;
    use rust_3d::*;
    use std::{fs::File, io::BufReader};
    #[test]
    fn load_untextured_obj() {
        let path = "src/tests/data/box.obj".to_string();
        let mut m = rust_3d::Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
        let mut material_info = MaterialInfo::new();
        load_obj_mesh(
            &mut BufReader::new(File::open(path).unwrap()),
            &mut m,
            &mut material_info,
        )
        .unwrap();
        assert_eq!(m.num_vertices(), 8);
        assert_eq!(m.num_faces(), 12);
        assert_eq!(material_info.uv.len(), 0);
        assert_eq!(material_info.surfaces.contains_key("Default"), true);
        assert_eq!(
            material_info.surfaces.get("Default").unwrap().faces.len(),
            12
        );
    }
    #[test]
    fn load_textured_obj() {
        let path = "src/tests/data/capsule.obj".to_string();
        let mut m = rust_3d::Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
        let mut material_info = MaterialInfo::new();
        load_obj_mesh(
            &mut BufReader::new(File::open(path).unwrap()),
            &mut m,
            &mut material_info,
        )
        .unwrap();
        assert_eq!(m.num_vertices(), 5252);
        assert_eq!(m.num_faces(), 10200);
        assert_eq!(material_info.uv.len(), 5252);
        assert_eq!(material_info.surfaces.contains_key("material0"), true);
        assert_eq!(
            material_info.surfaces.get("material0").unwrap().faces.len(),
            10200
        );
        assert_eq!(material_info.material_libs.len(), 1);
        assert_eq!(material_info.material_libs.contains("capsule.mtl"), true);
    }
    #[test]
    fn quad_mesh_works() {
        let path = "src/tests/data/box_quads.obj".to_string();
        let mut m = rust_3d::Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
        let mut material_info = MaterialInfo::new();
        load_obj_mesh(
            &mut BufReader::new(File::open(path).unwrap()),
            &mut m,
            &mut material_info,
        )
        .unwrap();
        assert_eq!(m.num_vertices(), 8);
        assert_eq!(m.num_faces(), 12);
        assert_eq!(material_info.uv.len(), 0);
        assert_eq!(material_info.surfaces.contains_key("Default"), true);
        assert_eq!(
            material_info.surfaces.get("Default").unwrap().faces.len(),
            12
        );
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

    #[test]
    fn load_textured_binary_ply() {
        let path = "src/tests/data/capsule.ply".to_string();
        let mut m = rust_3d::Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
        let mut material = Material::new();
        let mut material_info = MaterialInfo::new();
        let file_name = "capsule.ply".to_string();
        load_ply(
            &mut BufReader::new(File::open(path).unwrap()),
            &mut m,
            &mut material,
            &mut material_info,
            &file_name,
        )
        .unwrap();
        assert_eq!(m.num_vertices(), 5252);
        assert_eq!(m.num_faces(), 10200);
        assert_eq!(material_info.uv.len(), 3 * 10200);
        assert_eq!(material_info.surfaces.len(), 1);
        assert_eq!(material_info.surfaces.contains_key(&file_name), true);
        assert_eq!(material.texture_name.unwrap(), "capsule0.jpg");
    }

    #[test]
    fn load_textured_ascii_ply() {
        let path = "src/tests/data/capsule-ascii.ply".to_string();
        let mut m = rust_3d::Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
        let mut material = Material::new();
        let mut material_info = MaterialInfo::new();
        let file_name = "capsule.ply".to_string();
        load_ply(
            &mut BufReader::new(File::open(path).unwrap()),
            &mut m,
            &mut material,
            &mut material_info,
            &file_name,
        )
        .unwrap();
        assert_eq!(m.num_vertices(), 5252);
        assert_eq!(m.num_faces(), 10200);
        assert_eq!(material_info.uv.len(), 3 * 10200);
        assert_eq!(material_info.surfaces.len(), 1);
        assert_eq!(material_info.surfaces.contains_key(&file_name), true);
        assert_eq!(material.texture_name.unwrap(), "capsule0.jpg");
    }

    #[test]
    fn load_untextured_binary_ply() {
        let path = "src/tests/data/capsule-notexture-binary.ply".to_string();
        let mut m = rust_3d::Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
        let mut material = Material::new();
        let mut material_info = MaterialInfo::new();
        let file_name = "capsule.ply".to_string();
        load_ply(
            &mut BufReader::new(File::open(path).unwrap()),
            &mut m,
            &mut material,
            &mut material_info,
            &file_name,
        )
        .unwrap();
        assert_eq!(m.num_vertices(), 5252);
        assert_eq!(m.num_faces(), 10200);
        assert_eq!(material_info.uv.len(), 0);
        assert_eq!(material_info.surfaces.len(), 1);
        assert_eq!(material_info.surfaces.contains_key(&file_name), true);
        assert_eq!(material.texture_name, None);
    }

    #[test]
    fn load_untextured_ascii_ply() {
        let path = "src/tests/data/capsule-notexture-ascii.ply".to_string();
        let mut m = rust_3d::Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
        let mut material = Material::new();
        let mut material_info = MaterialInfo::new();
        let file_name = "capsule.ply".to_string();
        load_ply(
            &mut BufReader::new(File::open(path).unwrap()),
            &mut m,
            &mut material,
            &mut material_info,
            &file_name,
        )
        .unwrap();
        assert_eq!(m.num_vertices(), 5252);
        assert_eq!(m.num_faces(), 10200);
        assert_eq!(material_info.uv.len(), 0);
        assert_eq!(material_info.surfaces.len(), 1);
        assert_eq!(material_info.surfaces.contains_key(&file_name), true);
        assert_eq!(material.texture_name, None);
    }
}
