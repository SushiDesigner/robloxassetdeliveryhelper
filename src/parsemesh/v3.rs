use std::io::{self, Read};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct MeshHeader3 {
    pub header_size: u16,
    pub vert_size: u8,
    pub face_size: u8,
    pub lod_size: u16,

    pub num_lod: u16,
    pub num_verts: u32,
    pub num_faces: u32,
}

impl MeshHeader3 {
    pub fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let header_size = rdr.read_u16::<LittleEndian>()?;
        let vert_size = rdr.read_u8()?;
        let face_size = rdr.read_u8()?;
        let lod_size = rdr.read_u16::<LittleEndian>()?;
        let num_lod = rdr.read_u16::<LittleEndian>()?;
        let num_verts = rdr.read_u32::<LittleEndian>()?;
        let num_faces = rdr.read_u32::<LittleEndian>()?;

        Ok(MeshHeader3 {
            header_size,
            vert_size,
            face_size,
            lod_size,
            num_lod,
            num_verts,
            num_faces,
        })
    }
}
