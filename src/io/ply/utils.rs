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

//! Module for interal utility functions for IO operations of the ply file format

use rust_3d::*;

use super::{super::byte_reader::*, types::*};

use std::io::Read;

//------------------------------------------------------------------------------

#[inline(always)]
pub fn read_face_type<BR, R>(read: &mut R, t: FaceType) -> PlyResult<usize>
where
    BR: IsByteReader,
    R: Read,
{
    match t {
        FaceType::Char => Ok(BR::read_i8(read)? as usize),
        FaceType::UChar => Ok(BR::read_u8(read)? as usize),
        FaceType::Short => Ok(BR::read_i16(read)? as usize),
        FaceType::UShort => Ok(BR::read_u16(read)? as usize),
        FaceType::Int => Ok(BR::read_i32(read)? as usize),
        FaceType::UInt => Ok(BR::read_u32(read)? as usize),
        _ => Err(PlyError::InvalidFaceType),
    }
}

//------------------------------------------------------------------------------

#[inline(always)]
pub fn read_vertex_type<BR, R>(read: &mut R, t: VertexType) -> PlyResult<f64>
where
    BR: IsByteReader,
    R: Read,
{
    Ok(match t {
        VertexType::Float => BR::read_f32(read)? as f64,
        VertexType::Double => BR::read_f64(read)?,
    })
}

//------------------------------------------------------------------------------

#[inline(always)]
pub fn point_with_order<P>(fst: f64, snd: f64, third: f64, order: VertexOrder) -> P
where
    P: IsBuildable3D,
{
    match order {
        VertexOrder::Xyz => P::new(fst, snd, third),
        VertexOrder::Xzy => P::new(fst, third, snd),
        VertexOrder::Yxz => P::new(snd, fst, third),
        VertexOrder::Yzx => P::new(snd, third, fst),
        VertexOrder::Zxy => P::new(third, fst, snd),
        VertexOrder::Zyx => P::new(third, snd, fst),
    }
}
