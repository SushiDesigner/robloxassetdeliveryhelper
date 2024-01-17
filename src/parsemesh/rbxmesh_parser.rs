use crate::parsemesh::mesh::constructv2;
use crate::parsemesh::mesh::*;
use crate::parsemesh::v3::MeshHeader3;
use crate::parsemesh::v4::Envelope;
use crate::parsemesh::v4::MeshHeader4;
use actix_web::web::Buf;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::io::SeekFrom;
use std::io::{Read, Seek};
use std::mem;
use std::str;

pub fn parse(buffer: &mut [u8]) -> Option<Vec<u8>> {
    let mut buf: Vec<u8> = vec![0; 4];

    let mut cursor = Cursor::new(&buffer);

    cursor.seek(SeekFrom::Current(8)).unwrap(); // skip first 8 bytes

    cursor.read_exact(&mut buf).unwrap();

    let version = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(_) => "",
    };

    match version {
        "1.00" | "1.01" | "2.00" => return None, // skip old mesh
        "3.00" | "3.01" | "4.00" | "4.01" => return Some(parse_bin(buffer, version)),
        _ => {
            println!("Unsupported mesh version {}", version);
            return None;
        }
    };
}

fn parse_bin(buffer: &mut [u8], version: &str) -> Vec<u8> {
    let mut cursor = Cursor::new(&buffer);
    cursor.seek(SeekFrom::Start(13)).unwrap();

    if version == "3.00" || version == "3.01" {
        let header: MeshHeader3 = match MeshHeader3::from_reader(cursor.clone().reader()) {
            Ok(mesh_header3) => mesh_header3,
            Err(_) => {
                panic!("")
            }
        };

        cursor
            .seek(SeekFrom::Current(mem::size_of::<MeshHeader3>() as i64))
            .unwrap();

        let mut vertices: Vec<Vertex> = Vec::new();

        for _ in 0..header.num_verts {
            let vertex = match Vertex::from_reader(cursor.clone().reader()) {
                Ok(vertex) => vertex,
                Err(_) => {
                    panic!("")
                }
            };

            vertices.push(vertex);

            cursor
                .seek(SeekFrom::Current(mem::size_of::<Vertex>() as i64))
                .unwrap();
        }

        let mut faces: Vec<Face> = Vec::new();

        for _ in 0..header.num_faces {
            let face = match Face::from_reader(cursor.clone().reader()) {
                Ok(face) => face,
                Err(_) => {
                    panic!("")
                }
            };

            faces.push(face);

            cursor
                .seek(SeekFrom::Current(mem::size_of::<Face>() as i64))
                .unwrap();
        }

        let mut lods: Vec<u16> = Vec::new();

        for _ in 0..header.num_lod {
            let lod = cursor.read_u16::<LittleEndian>().unwrap();

            lods.push(lod);

            cursor
                .seek(SeekFrom::Current(mem::size_of::<u16>() as i64))
                .unwrap();
        }

        assert_eq!(mem::size_of::<MeshHeader3>(), header.header_size as usize);
        #[cfg(debug_assertions)]
        {
            println!("{:?}", header);

            println!("Verts: {:?}", vertices.len());

            println!("Faces: {:?}", faces.len());

            println!("Lods: {:?}", lods.len());
        }

        return constructv2(vertices, faces, lods);
    }

    if version == "4.00" || version == "4.01" {
        let header: MeshHeader4 = match MeshHeader4::from_reader(cursor.clone().reader()) {
            Ok(mesh_header4) => mesh_header4,
            Err(_) => {
                panic!("")
            }
        };

        cursor.seek(SeekFrom::Current(24 as i64)).unwrap();

        let mut vertices: Vec<Vertex> = Vec::new();

        for _ in 0..header.num_verts {
            let vertex = match Vertex::from_reader(cursor.clone().reader()) {
                Ok(vertex) => vertex,
                Err(_) => {
                    panic!("")
                }
            };

            vertices.push(vertex);

            cursor
                .seek(SeekFrom::Current(mem::size_of::<Vertex>() as i64))
                .unwrap();
        }

        let mut envelopes: Vec<Envelope> = Vec::new();

        if header.num_bones > 0 {
            for _ in 0..header.num_verts {
                let envelope = match Envelope::from_reader(cursor.clone().reader()) {
                    Ok(envelope) => envelope,
                    Err(_) => {
                        panic!("")
                    }
                };

                envelopes.push(envelope);

                cursor
                    .seek(SeekFrom::Current(mem::size_of::<Envelope>() as i64))
                    .unwrap();
            }
        }

        let mut faces: Vec<Face> = Vec::new();

        for _ in 0..header.num_faces {
            let face = match Face::from_reader(cursor.clone().reader()) {
                Ok(face) => face,
                Err(_) => {
                    panic!("")
                }
            };

            faces.push(face);

            cursor
                .seek(SeekFrom::Current(mem::size_of::<Face>() as i64))
                .unwrap();
        }

        let mut lods: Vec<u16> = Vec::new();

        for _ in 0..header.num_lod {
            let lod = cursor.read_u16::<LittleEndian>().unwrap();

            lods.push(lod);

            cursor
                .seek(SeekFrom::Current(mem::size_of::<u16>() as i64))
                .unwrap();
        }

        assert_eq!(24, header.header_size as usize);

        #[cfg(debug_assertions)]
        {
            println!("{:?}", header);

            println!("Verts: {:?}", vertices.len());

            println!("Envelopes: {:?}", envelopes.len());

            println!("Faces: {:?}", faces.len());

            println!("Lods: {:?}", lods.len());
        }

        return constructv2(vertices, faces, lods);
    }

    return Vec::new();
}
