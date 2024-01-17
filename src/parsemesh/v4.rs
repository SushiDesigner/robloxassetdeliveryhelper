use std::io::{self, Read};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::parsemesh::mesh::*;

struct Mesh {
    header: MeshHeader4,

    vertices: Vec<Vertex>,
    envelopes: Vec<Envelope>,

    faces: Vec<Face>,
    lods: Vec<u16>,

    bones: Vec<Bone>,
    name_table: Vec<u8>,

    subsets: Vec<MeshSubset>,
}

struct Bone {
    bone_name_index: u32,

    parent_index: u16,
    lod_parent_index: u16,

    culling: f32,

    r00: f32,
    r01: f32,
    r02: f32,
    r10: f32,
    r11: f32,
    r12: f32,
    r20: f32,
    r21: f32,
    r22: f32,

    x: f32,
    y: f32,
    z: f32,
}

struct MeshSubset {
    // i never decided to read this value anyway because its not necessary to convert to 2.00
    faces_begin: u32,
    faces_length: u32,

    verts_begin: f32,
    verts_length: f32,

    num_bone_indices: f32,
    bone_indices: [u16; 26],
}
#[derive(Debug)]
pub struct Envelope {
    pub bones: [u8; 4],
    pub weights: [u8; 4],
}

impl Envelope {
    pub fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let mut bones = [0; 4];

        let mut weights = [0; 4];

        for i in 0..4 {
            let bone = rdr.read_u8().unwrap();
            bones[i] = bone;

            let weight = rdr.read_u8().unwrap();

            weights[i] = weight;
        }

        Ok(Envelope { bones, weights })
    }
}

#[derive(Debug)]
pub struct MeshHeader4 {
    //ushort = u16
    //byte = u8
    //uint = u32
    pub header_size: u16,
    pub lod_type: u16,

    pub num_verts: u32,
    pub num_faces: u32,

    pub num_lod: u16,
    pub num_bones: u16,

    pub bone_names_buffer_size: u32,
    pub num_subsets: u16,

    pub face_size: u8,
    pub lod_size: u16,

    pub num_high_quality_lods: u8,
    pub unused: u8,
}

impl MeshHeader4 {
    pub fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let header_size = rdr.read_u16::<LittleEndian>()?;
        let lod_type = rdr.read_u16::<LittleEndian>()?;

        let num_verts = rdr.read_u32::<LittleEndian>()?;
        let num_faces = rdr.read_u32::<LittleEndian>()?;

        let num_lod = rdr.read_u16::<LittleEndian>()?;
        let num_bones = rdr.read_u16::<LittleEndian>()?;

        let bone_names_buffer_size = rdr.read_u32::<LittleEndian>()?;
        let num_subsets = rdr.read_u16::<LittleEndian>()?;

        let face_size = rdr.read_u8()?;
        let lod_size = rdr.read_u16::<LittleEndian>()?;

        let num_high_quality_lods = rdr.read_u8()?;
        let unused = rdr.read_u8()?;

        Ok(MeshHeader4 {
            header_size,
            lod_type,
            num_verts,
            num_faces,
            num_lod,
            num_bones,
            bone_names_buffer_size,
            num_subsets,
            face_size,
            lod_size,
            num_high_quality_lods,
            unused,
        })
    }
}
