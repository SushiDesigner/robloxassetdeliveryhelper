use std::{
    io::{self, Read, Write},
    mem,
};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(bytemuck::NoUninit, Clone, Copy, Debug)]
#[repr(C)]
pub struct Vertex {
    px: f32,
    py: f32,
    pz: f32, // Position
    nx: f32,
    ny: f32,
    nz: f32, // Normal Vector
    tu: f32,
    tv: f32, // UV Texture Coordinates
    tx: i8,
    ty: i8,
    tz: i8,
    ts: i8, // Tangent Vector & Bi-Normal Direction
    r: u8,
    g: u8,
    b: u8,
    a: u8, // RGBA Color Tinting
}

#[derive(bytemuck::NoUninit, Clone, Copy, Debug)]
#[repr(C)]
pub struct Face {
    a: u32, // 1st Vertex Index
    b: u32, // 2nd Vertex Index
    c: u32, // 3rd Vertex Index
}

impl Face {
    pub fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let a = rdr.read_u32::<LittleEndian>()?;
        let b = rdr.read_u32::<LittleEndian>()?;
        let c = rdr.read_u32::<LittleEndian>()?;

        Ok(Face { a, b, c })
    }
    pub fn get_normal_faces(faces: Vec<Face>, lods: Vec<u16>) -> Vec<Face> {
        if lods.len() > 1 {
            return faces[..lods[1] as usize].to_vec();
        }
        return faces;
    }
}

impl Vertex {
    pub fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let px = rdr.read_f32::<LittleEndian>()?;
        let py = rdr.read_f32::<LittleEndian>()?;
        let pz = rdr.read_f32::<LittleEndian>()?;
        let nx = rdr.read_f32::<LittleEndian>()?;
        let ny = rdr.read_f32::<LittleEndian>()?;
        let nz = rdr.read_f32::<LittleEndian>()?;
        let tu = rdr.read_f32::<LittleEndian>()?;
        let tv = rdr.read_f32::<LittleEndian>()?;
        let tx = rdr.read_i8()?;
        let ty = rdr.read_i8()?;
        let tz = rdr.read_i8()?;
        let ts = rdr.read_i8()?;
        let r = rdr.read_u8()?;
        let g = rdr.read_u8()?;
        let b = rdr.read_u8()?;
        let a = rdr.read_u8()?;

        Ok(Vertex {
            px,
            py,
            pz,
            nx,
            ny,
            nz,
            tu,
            tv,
            tx,
            ty,
            tz,
            ts,
            r,
            g,
            b,
            a,
        })
    }
}

#[derive(bytemuck::NoUninit, Clone, Copy, Debug)]
#[repr(C)]
pub struct MeshHeader2 {
    pub header_size: u16,
    pub vert_size: u8,
    pub face_size: u8,

    pub num_verts: u32,
    pub num_faces: u32,
}

pub fn constructv2(vertices: Vec<Vertex>, mut faces: Vec<Face>, lods: Vec<u16>) -> Vec<u8> {
    let buffer = Vec::new();

    let mut file = std::io::BufWriter::new(buffer);
    file.write_all(b"version 2.00\n").unwrap();

    faces = Face::get_normal_faces(faces, lods);

    let mesh_header2: MeshHeader2 = MeshHeader2 {
        header_size: mem::size_of::<MeshHeader2>() as u16,
        vert_size: mem::size_of::<Vertex>() as u8,
        face_size: mem::size_of::<Face>() as u8,
        num_verts: vertices.len() as u32,
        num_faces: faces.len() as u32,
    };
    #[cfg(debug_assertions)]
    println!("{:?}", mesh_header2);

    let bytes: &[u8] = bytemuck::bytes_of(&mesh_header2);

    file.write_all(bytes).unwrap();

    for vertex in vertices {
        let bytes: &[u8] = bytemuck::bytes_of(&vertex);

        file.write_all(bytes).unwrap();
    }

    for face in faces {
        let bytes: &[u8] = bytemuck::bytes_of(&face);

        file.write_all(bytes).unwrap();
    }
    #[cfg(debug_assertions)]
    println!("Done!");

    file.flush().unwrap();

    return file.into_inner().unwrap();
}
