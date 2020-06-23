use super::{MeshContainer, Model, Msg, ShouldRender};
use crate::io::*;
use crate::render_buffers::{MeshSurface, VertexData};
use image::GenericImageView;
use log::*;
use rust_3d::{io::load_stl_mesh_unique, *};
use std::collections::HashMap;
use std::path::Path;
use three_d::core::types::InnerSpace;
use three_d::vec3;
use yew::services::reader::File;
use yew::{html, ChangeData, Html};

type Rust3DMesh = rust_3d::Mesh3D<Point3D, PointCloud3D<Point3D>, Vec<usize>>;

fn string_to_format(format_string: &String) -> Option<image::ImageFormat> {
    let test_string = format_string.to_lowercase();
    if test_string == "png" {
        Some(image::ImageFormat::Png)
    } else if test_string == "jpg" || test_string == ".jpeg" {
        Some(image::ImageFormat::Jpeg)
    } else if test_string == "gif" {
        Some(image::ImageFormat::Gif)
    } else if test_string == "webp" {
        Some(image::ImageFormat::WebP)
    } else if test_string == "pnm" {
        Some(image::ImageFormat::Pnm)
    } else if test_string == "tiff" || test_string == "tiff" {
        Some(image::ImageFormat::Tiff)
    } else if test_string == "tga" {
        Some(image::ImageFormat::Tga)
    } else if test_string == "dds" {
        Some(image::ImageFormat::Dds)
    } else if test_string == "bmp" {
        Some(image::ImageFormat::Bmp)
    } else if test_string == "ico" {
        Some(image::ImageFormat::Ico)
    } else if test_string == "hdr" {
        Some(image::ImageFormat::Hdr)
    } else {
        None
    }
}

fn compute_normals(indices: &[u32], positions: &[f32]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len() * 3];
    for face in 0..indices.len() / 3 {
        let index0 = indices[face * 3] as usize;
        let p0 = vec3(
            positions[index0 * 3],
            positions[index0 * 3 + 1],
            positions[index0 * 3 + 2],
        );
        let index1 = indices[face * 3 + 1] as usize;
        let p1 = vec3(
            positions[index1 * 3],
            positions[index1 * 3 + 1],
            positions[index1 * 3 + 2],
        );
        let index2 = indices[face * 3 + 2] as usize;
        let p2 = vec3(
            positions[index2 * 3],
            positions[index2 * 3 + 1],
            positions[index2 * 3 + 2],
        );

        let normal = (p1 - p0).cross(p2 - p0);
        normals[index0 * 3] += normal.x;
        normals[index0 * 3 + 1] += normal.y;
        normals[index0 * 3 + 2] += normal.z;
        normals[index1 * 3] += normal.x;
        normals[index1 * 3 + 1] += normal.y;
        normals[index1 * 3 + 2] += normal.z;
        normals[index2 * 3] += normal.x;
        normals[index2 * 3 + 1] += normal.y;
        normals[index2 * 3 + 2] += normal.z;
    }

    for i in 0..normals.len() / 3 {
        let normal = vec3(normals[3 * i], normals[3 * i + 1], normals[3 * i + 2]).normalize();
        normals[3 * i] = normal.x;
        normals[3 * i + 1] = normal.y;
        normals[3 * i + 2] = normal.z;
    }
    normals
}

fn create_mesh_surface(
    model: &mut Model,
    mesh: &Rust3DMesh,
    uvs: &rust_3d::PointCloud3D<Point3D>,
    material_name: Option<String>,
    surface: &MaterialSurface,
) -> MeshSurface {
    let gl_ref = model.gl.as_ref().expect("GL Context not initialized!");
    // Only add faces that belong to this surface.
    let mut indices: Vec<u32> = vec![];
    for fid in 0..mesh.num_faces() {
        let vids = mesh.face_vertex_ids(FId { val: fid }).unwrap();
        if !surface.faces.contains(&vids.clone()) {
            continue;
        }
        indices.push(vids.a.val as u32);
        indices.push(vids.b.val as u32);
        indices.push(vids.c.val as u32);
    }
    let mut maybe_uvs: Option<&[f32]> = None;
    let mut uv_vec: Vec<f32> = vec![];
    if uvs.len() > 0 {
        for point in uvs.data.iter() {
            uv_vec.push(point.x as f32);
            uv_vec.push(point.y as f32);
            uv_vec.push(point.z as f32);
        }
        info!("Loaded {} uvs", uv_vec.len() / 3);
        maybe_uvs = Some(&uv_vec[..]);
    }
    MeshSurface::new(gl_ref, &indices[..], maybe_uvs, material_name).unwrap()
}

fn create_vertex_data(
    model: &mut Model,
    mesh: &Rust3DMesh,
    maybe_normals: Option<Vec<f32>>,
) -> VertexData {
    let gl_ref = model.gl.as_ref().expect("GL Context not initialized!");
    let mut vertices: Vec<f32> = vec![];
    let mut indices: Vec<u32> = vec![];
    for fid in 0..mesh.num_faces() {
        let vids = mesh.face_vertex_ids(FId { val: fid }).unwrap();
        indices.push(vids.a.val as u32);
        indices.push(vids.b.val as u32);
        indices.push(vids.c.val as u32);
    }
    for vid in 0..mesh.num_vertices() {
        let vertex = mesh.vertex(VId { val: vid }).unwrap();
        vertices.push(vertex.x as f32);
        vertices.push(vertex.y as f32);
        vertices.push(vertex.z as f32);
    }
    match maybe_normals {
        Some(normals) => {
            info!("Using {} provided normals.", normals.len());
            VertexData::new(gl_ref, &vertices[..], &normals[..]).unwrap()
        }
        None => {
            info!(
                "computing normals with {} indices and {} indices.",
                vertices.len(),
                indices.len()
            );
            VertexData::new(
                gl_ref,
                &vertices[..],
                &compute_normals(&indices[..], &vertices[..]),
            )
            .unwrap()
        }
    }
}

fn extend_by_vertex(p: &Point3D, array: &mut Vec<f32>) {
    array.push(p.x() as f32);
    array.push(p.y() as f32);
    array.push(p.z() as f32);
}

/// Per-wedge UVs require that we duplicate any
/// vertices that are referenced multiple times
/// in the index vector so that the vertex and
/// UV buffer that we send to the GPU are the
/// same length. The resulting vectors are the
/// same length and arrangement of the index
/// vector, and the index vector is therefore
/// sequential.
fn divide_mesh_by_materials_per_wedge(
    model: &mut Model,
    mesh: &Rust3DMesh,
    material_info: &MaterialInfo,
) -> MeshContainer {
    info!("Rebuilding vertex/UV vectors so they agree.");
    let gl_ref = model.gl.as_ref().expect("GL Context not initialized!");
    let mut vertices = vec![];
    let mut uvs = vec![];
    let uv_in = &material_info.uv;
    if 3 * mesh.num_faces() != uv_in.len() {
        // This function is only meant to be used with a mesh that has
        // per-face UVs.
        panic!(
            "Expected 1 UV per wedge ({} != {})!",
            mesh.num_faces() * 3,
            uv_in.len()
        );
    }
    let mut surface_indices: HashMap<String, Vec<u32>> = HashMap::new();

    // Initialize index vector for each surface.
    for surface in &material_info.surfaces {
        surface_indices.insert(surface.0.clone(), vec![]);
    }
    let mut all_indices = vec![];
    for fid in 0..mesh.num_faces() {
        let vids = mesh.face_vertex_ids(FId { val: fid }).unwrap();
        all_indices.push((fid * 3 + 0) as u32);
        all_indices.push((fid * 3 + 1) as u32);
        all_indices.push((fid * 3 + 2) as u32);
        // Insert this index into surfaces to which this face belongs.
        for surface in &material_info.surfaces {
            if surface.1.faces.contains(&vids) {
                let index_vector = surface_indices.get_mut(surface.0).unwrap();
                index_vector.push((fid * 3 + 0) as u32);
                index_vector.push((fid * 3 + 1) as u32);
                index_vector.push((fid * 3 + 2) as u32);
            }
        }
        let v0 = mesh.vertex(vids.a).unwrap();
        extend_by_vertex(&v0, &mut vertices);
        let v1 = mesh.vertex(vids.b).unwrap();
        extend_by_vertex(&v1, &mut vertices);
        let v2 = mesh.vertex(vids.c).unwrap();
        extend_by_vertex(&v2, &mut vertices);
        // 1 UV per face, each a point3D, is stored for each face,
        // for a total of 3 points per face.
        // UVs.
        let uv_base_idx = fid * 3;
        let uv0 = uv_in.get_d(uv_base_idx + 0);
        extend_by_vertex(&uv0, &mut uvs);
        let uv1 = uv_in.get_d(uv_base_idx + 1);
        extend_by_vertex(&uv1, &mut uvs);
        let uv2 = uv_in.get_d(uv_base_idx + 2);
        extend_by_vertex(&uv2, &mut uvs);
    }
    // we always have to compute normals here since the pre-computed
    // normals will not match, and it's better to have per-face
    // normals anyway.
    let vertex_data = VertexData::new(
        gl_ref,
        &vertices[..],
        &compute_normals(&all_indices[..], &vertices[..]),
    )
    .unwrap();
    let mut surfaces: Vec<MeshSurface> = vec![];
    for name_and_indices in surface_indices {
        info!("Material name {}", name_and_indices.0);
        let surface = MeshSurface::new(
            gl_ref,
            &name_and_indices.1[..],
            Some(&uvs[..]),
            Some(name_and_indices.0),
        )
        .unwrap();
        surfaces.push(surface);
    }
    MeshContainer {
        vertices: vertex_data,
        surfaces: surfaces,
        visible: true,
    }
}

/// Divide a mesh by materials, in instances where the only
/// buffer we're passing to the shader is the vertex buffer. In
/// this case we can use the values and existing order
/// directly.
fn divide_mesh_by_materials(
    model: &mut Model,
    mesh: &Rust3DMesh,
    material_info: &MaterialInfo,
    maybe_normals: Option<Vec<f32>>,
) -> MeshContainer {
    let vertices = create_vertex_data(model, mesh, maybe_normals);
    info!("Adding model with {} vertices", mesh.num_vertices());
    let mut surfaces: Vec<MeshSurface> = vec![];
    if material_info.surfaces.len() > 0 {
        for surface in material_info.surfaces.iter() {
            info!(
                "Adding surface {} with {} faces",
                surface.0,
                surface.1.faces.len()
            );
            let mesh_surface = create_mesh_surface(
                model,
                mesh,
                &material_info.uv,
                Some(surface.0.clone()),
                surface.1,
            );
            surfaces.push(mesh_surface);
        }
    } else {
        let gl_ref = model.gl.as_ref().expect("GL Context not initialized!");
        let mut indices: Vec<u32> = vec![];
        for fid in 0..mesh.num_faces() {
            let vids = mesh.face_vertex_ids(FId { val: fid }).unwrap();
            indices.push(vids.a.val as u32);
            indices.push(vids.b.val as u32);
            indices.push(vids.c.val as u32);
        }
        let surface = MeshSurface::new(gl_ref, &indices[..], None, None).unwrap();
        surfaces.push(surface);
    }
    MeshContainer {
        vertices: vertices,
        surfaces: surfaces,
        visible: true,
    }
}

pub fn update(model: &mut Model, msg: &Msg) -> ShouldRender {
    match msg {
        Msg::Files(files) => {
            for file in files.into_iter() {
                let task = {
                    let callback = model.link.callback(Msg::Loaded);
                    model.reader.read_file(file.clone(), callback).unwrap()
                };
                model.tasks.push(task);
            }
        }
        Msg::Loaded(file) => {
            let path = Path::new(&file.name);
            info!("Loading file {}", file.name);
            if let Some(ext) = path.extension() {
                let maybe_image_format =
                    string_to_format(&ext.to_os_string().into_string().unwrap().to_lowercase());
                if let Some(image_format) = maybe_image_format {
                    info!("Loading image {}", file.name);
                    match image::load_from_memory_with_format(&file.content[..], image_format) {
                        Ok(image) => {
                            let gl_ref = model.gl.as_ref().expect("GL Context not initialized!");
                            let (width, height) = image.dimensions();
                            info!("Loaded {}x{} image {}", file.name, width, height);
                            match three_d::texture::Texture2D::new_with_u8(
                                gl_ref,
                                three_d::Interpolation::Linear,
                                three_d::Interpolation::Linear,
                                Some(three_d::Interpolation::Linear),
                                three_d::Wrapping::ClampToEdge,
                                three_d::Wrapping::ClampToEdge,
                                width,
                                height,
                                &image.to_bytes()[..],
                            ) {
                                Ok(texture) => {
                                    info!("Created texture {}", file.name);
                                    model.images.insert(file.name.clone(), texture);
                                }
                                Err(e) => {
                                    info!(
                                        "Could not load {} as a texture due to error {:?}",
                                        file.name, e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            info!(
                                "Could not load {} as an image due to error {}",
                                file.name, e
                            );
                        }
                    }
                } else if ext == "ply" || ext == "obj" {
                    let mut m = Rust3DMesh::default();
                    let mut material_info = MaterialInfo::new();
                    let mut maybe_normals = None;
                    if ext == "ply" {
                        let mut material = Material::new();
                        match load_ply(
                            &mut &file.content[..],
                            &mut m,
                            &mut material,
                            &mut material_info,
                            &file.name,
                        ) {
                            Ok(_) => {
                                model.materials.insert(file.name.clone(), material);
                            }
                            Err(e) => {
                                warn!("Could not load {} as a PLY due to {:?}", file.name, e);
                            }
                        }
                    } else if ext == "obj" {
                        match load_obj_mesh(&mut &file.content[..], &mut m, &mut material_info) {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("Could not load {} as an OBJ due to {:?}", file.name, e);
                            }
                        }
                    } else if ext == "off" {
                        match load_off_mesh(&mut &file.content[..], &mut m) {
                            Ok(_) => {}
                            Err(e) => {
                                warn!("Could not load {} as an off due to {:?}", file.name, e);
                            }
                        }
                    } else if ext == "stl" {
                        let mut r3d_normals = vec![];
                        match load_stl_mesh_unique(
                            &mut &file.content[..],
                            rust_3d::io::StlFormat::Auto,
                            &mut m,
                            &mut r3d_normals,
                        ) {
                            Ok(_) => {
                                let mut normals = vec![];
                                for normal in r3d_normals {
                                    normals.push(normal.x as f32);
                                    normals.push(normal.x as f32);
                                    normals.push(normal.z as f32);
                                }
                                maybe_normals = Some(normals);
                                let meshes = divide_mesh_by_materials(
                                    model,
                                    &m,
                                    &material_info,
                                    maybe_normals,
                                );
                                model.mesh.insert(file.name.clone(), meshes);
                            }
                            Err(e) => {
                                warn!("Could not load {} as an off due to {:?}", file.name, e);
                            }
                        }
                        return true;
                    }
                    if m.num_vertices() == 0 {
                        return false;
                    }
                    info!(
                        "{} has {} vertices and {} indices",
                        file.name,
                        m.num_vertices(),
                        m.num_faces()
                    );
                    if material_info.uv.len() == 3 * m.num_faces() {
                        // Per-wedge UVs. If a model has both per-wedge and per-vertex
                        // UVs we should prefer per-wedge.
                        let meshes = divide_mesh_by_materials_per_wedge(model, &m, &material_info);
                        model.mesh.insert(file.name.clone(), meshes);
                    } else {
                        // Per-vetex UVs
                        let meshes =
                            divide_mesh_by_materials(model, &m, &material_info, maybe_normals);
                        model.mesh.insert(file.name.clone(), meshes);
                    }
                    return true;
                } else if ext == "mtl" {
                    info!("Loading an MTL file.");
                    match load_mtl(&mut &file.content[..]) {
                        Ok(materials) => {
                            for material in materials {
                                info!("Loading material {}: {:?}.", material.0, material.1);
                                model.materials.insert(material.0, material.1);
                            }
                        }
                        Err(e) => {
                            warn!(
                                "Could not load {} as an mtl file due to error: {:?}",
                                file.name, e
                            );
                        }
                    }
                }
            }
        }
        _ => {}
    };
    false
}
pub fn view(model: &Model) -> Html {
    html! {
        <form>
        <input id="load_mesh" type="file" multiple=true accept=".ply, .obj, .off, .stl, .mtl, .png, .jpeg, .jpg, .gif, .webp, .pnm, .tif, .tiff, .tga, .dds, .bmp, .ico, .hdr" onchange=model.link.callback(move |value| {
            let mut result = Vec::new();
            if let ChangeData::Files(files) = value {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .into_iter()
                    .map(|v| File::from(v.unwrap()));
                result.extend(files);
            }
            Msg::Files(result)
        })/>
        <label for="load_mesh">{"Select files to view."}</label>
        </form>
    }
}
