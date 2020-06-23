//! Module for IO of the mtl file format

use super::utils::*;
use super::Material;
use rust_3d::*;
use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, Error as ioError};

//------------------------------------------------------------------------------

/// Loads materials from a .mtl file
pub fn load_mtl<R>(read: &mut R) -> MtlResult<HashMap<String, Material>>
where
    R: BufRead,
{
    let mut line_buffer = Vec::new();
    let mut i_line = 0;
    let mut mtl_name: Option<String> = None;
    let mut result: HashMap<String, Material> = HashMap::new();
    while let Ok(line) = fetch_line(read, &mut line_buffer) {
        i_line += 1;
        if line.starts_with(b"newmtl ") {
            // skip "newmtl"
            let mut words = to_words_skip_empty(line);
            words.next().ok_or(MtlError::LineParse(i_line))?;
            if let Some(next_word) = words.next().and_then(|w| from_ascii(w)) {
                mtl_name = Some(next_word);
                result.insert(mtl_name.clone().unwrap(), Material::new());
            }
        } else if line.starts_with(b"map_Kd ") {
            let mut words = to_words_skip_empty(line);
            // skip "map_Kd"
            words.next().ok_or(MtlError::LineParse(i_line))?;
            if let Some(next_word) = words.next().and_then(|w| from_ascii(w)) {
                if let Some(mtl_name) = mtl_name.clone() {
                    if let Some(mtl) = result.get_mut(&mtl_name) {
                        mtl.texture_name = Some(next_word);
                    } else {
                        return Err(MtlError::NoMaterialError(i_line));
                    }
                }
            }
        } else if line.starts_with(b"Ns ") {
            let mut words = to_words_skip_empty(line);
            // skip "map_Kd"
            words.next().ok_or(MtlError::LineParse(i_line))?;
            if let Some(next_word) = words.next().and_then(|w| from_ascii(w)) {
                if let Some(mtl_name) = mtl_name.clone() {
                    if let Some(mtl) = result.get_mut(&mtl_name) {
                        mtl.specular_intensity = next_word;
                    } else {
                        return Err(MtlError::NoMaterialError(i_line));
                    }
                }
            }
        } else if line.starts_with(b"Kd ") {
            let mut words = to_words_skip_empty(line);
            // skip "Kd"
            words.next().ok_or(MtlError::LineParse(i_line))?;
            if let Some(mtl_name) = mtl_name.clone() {
                if let Some(mtl) = result.get_mut(&mtl_name) {
                    let mut first_word = words.next().ok_or(MtlError::LineParse(i_line))?;
                    // Skip "=" in some OBJ files
                    if first_word == b"=" {
                        first_word = words.next().ok_or(MtlError::LineParse(i_line))?;
                    }
                    let x = from_ascii(first_word).ok_or(MtlError::LineParse(i_line))?;

                    let y = words
                        .next()
                        .and_then(|w| from_ascii(w))
                        .ok_or(MtlError::LineParse(i_line))?;

                    let z = words
                        .next()
                        .and_then(|w| from_ascii(w))
                        .ok_or(MtlError::LineParse(i_line))?;
                    mtl.diffuse_color = three_d::Vec3::new(x, y, z);
                } else {
                    return Err(MtlError::NoMaterialError(i_line));
                }
            }
        }

        i_line += 1;
    }
    Ok(result)
}

/// Error type for .mtl file operations
pub enum MtlError {
    AccessFile,
    LineParse(usize),
    NoMaterialError(usize),
}

impl fmt::Debug for MtlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::NoMaterialError(x) => write!(f, "Line {} occurs before newmtl.", x),
        }
    }
}

/// Result type for .obj file operations
pub type MtlResult<T> = std::result::Result<T, MtlError>;

impl From<ioError> for MtlError {
    fn from(_error: ioError) -> Self {
        MtlError::AccessFile
    }
}
