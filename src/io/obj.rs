/*
Copyright 2020 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! Module for IO of the obj file format

use log::*;
use rust_3d::*;

use std::{
    fmt,
    io::{BufRead, Error as ioError},
};

use super::utils::*;
use super::{MaterialInfo, MaterialSurface};

//------------------------------------------------------------------------------

//@todo offer both point cloud and mesh loading? (applies to .ply as well?)
//@todo many valid files won't be read correctly currently

/// Loads an IsMesh3D from the .obj file format
pub fn load_obj_mesh<EM, P, R>(
    read: &mut R,
    mesh: &mut EM,
    material_info: &mut MaterialInfo,
) -> ObjResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut line_buffer = Vec::new();
    let mut i_line = 0;
    let mut mtl_name = "NotSpecified".to_string();
    material_info
        .surfaces
        .insert(mtl_name.clone(), MaterialSurface::new());

    while let Ok(line) = fetch_line(read, &mut line_buffer) {
        i_line += 1;
        if line.starts_with(b"usemtl ") {
            let mut words = to_words_skip_empty(line);

            // skip "usemtl"
            words.next().ok_or(ObjError::LineParse(i_line))?;

            mtl_name = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::LineParse(i_line))?;
            material_info
                .surfaces
                .insert(mtl_name.clone(), MaterialSurface::new());
        }

        if line.starts_with(b"mtllib ") {
            let mut words = to_words_skip_empty(line);

            // skip "v"
            words.next().ok_or(ObjError::LineParse(i_line))?;

            let lib_name: String = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::LineParse(i_line))?;
            material_info.material_libs.insert(lib_name.clone());
        }

        if line.starts_with(b"v ") {
            let mut words = to_words_skip_empty(line);

            // skip "v"
            words.next().ok_or(ObjError::LineParse(i_line))?;

            let x = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::LineParse(i_line))?;

            let y = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::LineParse(i_line))?;

            let z = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::LineParse(i_line))?;

            mesh.add_vertex(P::new(x, y, z));
        } else if line.starts_with(b"vt ") {
            let mut words = to_words_skip_empty(line);

            // skip "vt"
            words.next().ok_or(ObjError::LineParse(i_line))?;

            let x = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::LineParse(i_line))?;

            let y = words
                .next()
                .and_then(|w| from_ascii(w))
                .or(Some(0f64))
                .unwrap();
            let z = words
                .next()
                .and_then(|w| from_ascii(w))
                .or(Some(0f64))
                .unwrap();

            material_info.uv.push_d(Point3D::new(x, y, z));
        } else if line.starts_with(b"f ") {
            let mut words = to_words_skip_empty(line);

            // skip "f"
            words.next().ok_or(ObjError::LineParse(i_line))?;

            let mut tmp = words.next().ok_or(ObjError::LineParse(i_line))?;
            let tmp_str = until_bytes(tmp, b'/');
            let a: usize = from_ascii(tmp_str).ok_or(ObjError::LineParse(i_line))?;
            let mut maybe_at: Option<usize> = None;
            if tmp_str.len() + 1 < tmp.len() {
                maybe_at = from_ascii(until_bytes(&tmp[(tmp_str.len() + 1)..], b'/')).or(None);
            }

            tmp = words.next().ok_or(ObjError::LineParse(i_line))?;
            let tmp_str = until_bytes(tmp, b'/');
            let b: usize = from_ascii(tmp_str).ok_or(ObjError::LineParse(i_line))?;
            let mut maybe_bt: Option<usize> = None;
            if tmp_str.len() + 1 < tmp.len() {
                maybe_bt = from_ascii(until_bytes(&tmp[(tmp_str.len() + 1)..], b'/')).or(None);
            }

            tmp = words.next().ok_or(ObjError::LineParse(i_line))?;
            let tmp_str = until_bytes(tmp, b'/');
            let c: usize = from_ascii(tmp_str).ok_or(ObjError::LineParse(i_line))?;
            let mut maybe_ct: Option<usize> = None;
            if tmp_str.len() + 1 < tmp.len() {
                maybe_ct = from_ascii(until_bytes(&tmp[(tmp_str.len() + 1)..], b'/')).or(None);
            }

            let mut maybe_d: Option<usize> = None;
            let mut maybe_dt: Option<usize> = None;
            match words.next() {
                Some(tmp) => {
                    let tmp_str = until_bytes(tmp, b'/');
                    if tmp_str.len() > 0 {
                        maybe_d = from_ascii(until_bytes(&tmp_str, b'/')).or(None);
                    }
                    if tmp_str.len() + 1 < tmp.len() {
                        maybe_dt =
                            from_ascii(until_bytes(&tmp[(tmp_str.len() + 1)..], b'/')).or(None);
                    }
                    // The second face of a quad.
                    if let Some(d) = maybe_d {
                        let face = Face3 {
                            a: VId { val: a - 1 },
                            b: VId { val: c - 1 },
                            c: VId { val: d - 1 },
                        };
                        material_info
                            .surfaces
                            .get_mut(&mtl_name)
                            .unwrap()
                            .faces
                            .insert(face);
                    }
                    if let (Some(at), Some(ct), Some(d), Some(dt)) =
                        (maybe_at, maybe_ct, maybe_d, maybe_dt)
                    {
                        material_info
                            .surfaces
                            .get_mut(&mtl_name)
                            .unwrap()
                            .uvs
                            .insert(
                                Face3 {
                                    a: VId { val: a - 1 },
                                    b: VId { val: c - 1 },
                                    c: VId { val: d - 1 },
                                },
                                Face3 {
                                    a: VId { val: at - 1 },
                                    b: VId { val: ct - 1 },
                                    c: VId { val: dt - 1 },
                                },
                            );
                    }
                }
                None => {}
            };

            // This triangle should be created for either a triangle or quad face.
            material_info
                .surfaces
                .get_mut(&mtl_name)
                .unwrap()
                .faces
                .insert(Face3 {
                    a: VId { val: a - 1 },
                    b: VId { val: b - 1 },
                    c: VId { val: c - 1 },
                });
            if let (Some(at), Some(bt), Some(ct)) = (maybe_at, maybe_bt, maybe_ct) {
                material_info
                    .surfaces
                    .get_mut(&mtl_name)
                    .unwrap()
                    .uvs
                    .insert(
                        Face3 {
                            a: VId { val: a - 1 },
                            b: VId { val: b - 1 },
                            c: VId { val: c - 1 },
                        },
                        Face3 {
                            a: VId { val: at - 1 },
                            b: VId { val: bt - 1 },
                            c: VId { val: ct - 1 },
                        },
                    );
            }
            // obj indexing starts at 1
            if let Some(_next) = words.next() {
                return Err(ObjError::NotTriangularMesh(i_line));
            }
        }
    }
    // Add all faces after vertices have already been added. This is
    // in case there are multiple groups of vertices and faces; in that
    // case it would be possible for the face to come before the vertex
    // it references in the file.
    for surface in &material_info.surfaces {
        for face in &surface.1.faces {
            match mesh.try_add_connection(face.a, face.b, face.c).or(Err(
                ObjError::InvalidMeshIndices(face.a.val, face.b.val, face.c.val),
            )) {
                Ok(_) => {}
                Err(_) => {
                    info!(
                        "Warning, face {},{},{} could not be added.",
                        face.a.val, face.b.val, face.c.val
                    );
                }
            }
        }
    }
    Ok(())
}

/// Loads IsPushable<Is3D> from the .obj file format
pub fn load_obj_points<IP, P, R>(read: &mut R, ip: &mut IP) -> ObjResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let mut line_buffer = Vec::new();
    let mut i_line = 0;

    while let Ok(line) = fetch_line(read, &mut line_buffer) {
        i_line += 1;

        if line.starts_with(b"v ") {
            let mut words = to_words_skip_empty(line);

            // skip "v"
            words.next().ok_or(ObjError::LineParse(i_line))?;

            let x = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::LineParse(i_line))?;

            let y = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::LineParse(i_line))?;

            let z = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::LineParse(i_line))?;

            ip.push(P::new(x, y, z));
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Error type for .obj file operations
pub enum ObjError {
    AccessFile,
    InvalidMeshIndices(usize, usize, usize),
    LineParse(usize),
    NotTriangularMesh(usize),
}

/// Result type for .obj file operations
pub type ObjResult<T> = std::result::Result<T, ObjError>;

impl fmt::Debug for ObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::InvalidMeshIndices(x, y, z) => {
                write!(f, "File contains invalid mesh indices: {}, {}, {}", x, y, z)
            }
            Self::NotTriangularMesh(x) => write!(
                f,
                "File contains face with more than 3 sets of indices on line {}",
                x
            ),
        }
    }
}

impl From<ioError> for ObjError {
    fn from(_error: ioError) -> Self {
        ObjError::AccessFile
    }
}
