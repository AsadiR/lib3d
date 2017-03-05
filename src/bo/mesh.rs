

use bo::vector::Vector;
use bo::point::Point;
use bo::triangle::Triangle;
use std::io::{Result, ErrorKind, Error};
use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};

pub struct MeshTriangle {
    pub normal: Vector,
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
    pub attr_byte_count: u16,
    pub neighbors: Vec<usize>
}

impl MeshTriangle {
    fn add_neighbor(&mut self, n : usize) {
        self.neighbors.push(n);
    }
}

impl PartialEq for MeshTriangle {
    fn eq(&self, rhs: &MeshTriangle) -> bool {
        self.normal == rhs.normal
            && self.v1 == rhs.v1
            && self.v2 == rhs.v2
            && self.v3 == rhs.v3
            && self.attr_byte_count == rhs.attr_byte_count
    }
}

impl Eq for MeshTriangle {}

pub type Mesh = BinaryStlFile;

pub struct BinaryStlHeader {
    pub header: [u8; 80],
    pub num_triangles: u32
}

pub struct BinaryStlFile {
    pub header: BinaryStlHeader,
    pub triangles: Vec<MeshTriangle>,
    pub points: Vec<Point>
}

impl BinaryStlFile {
    pub fn new() -> BinaryStlFile {
        BinaryStlFile {
            header: BinaryStlHeader { header: [0u8; 80], num_triangles: 0 },
            triangles: Vec::new(),
            points: Vec::new()
        }
    }

    pub fn add_triangle(&mut self, tr : Triangle) {
        let mut m_tr = MeshTriangle {
            normal: tr.get_normal(),
            v1: 0, v2: 0, v3: 0,
            attr_byte_count: 0,
            neighbors: Vec::new()
        };

        m_tr.v1 = add_point_to_vec(tr.p1, &mut self.points, &mut m_tr, &mut self.triangles);
        m_tr.v2 = add_point_to_vec(tr.p2, &mut self.points, &mut m_tr, &mut self.triangles);
        m_tr.v3 = add_point_to_vec(tr.p3, &mut self.points, &mut m_tr, &mut self.triangles);

        self.triangles.push(m_tr);
        self.header.num_triangles += 1;
    }

    pub fn add_triangles(&mut self, ts : Vec<Triangle>) {
        for t in ts {
            self.add_triangle(t);
        }
    }

    pub fn write_stl<T: WriteBytesExt>(
        &self,
        out: &mut T,
    ) -> Result<()> {
        write_stl(out, self)
    }

    pub fn get_triangle(&self, index : usize) -> Triangle {
        let mt : &MeshTriangle = &self.triangles[index];
        let p1 = self.points[mt.v1].clone();
        let p2 = self.points[mt.v2].clone();
        let p3 = self.points[mt.v3].clone();

        Triangle::new(p1,p2,p3)
    }
}

fn read_point<T: ReadBytesExt>(input: &mut T) -> Result<Point> {
    let x1 = input.read_f32::<LittleEndian>()?;
    let x2 = input.read_f32::<LittleEndian>()?;
    let x3 = input.read_f32::<LittleEndian>()?;

    Ok(Point {x: x1, y: x2, z: x3})
}

// it adds neighbors and collects unique points in vector
fn add_point_to_vec(point : Point, points : &mut Vec<Point>, triangle : &mut MeshTriangle, ts : &mut Vec<MeshTriangle>) -> usize {
    let res : Option<usize>;
    {
        let f = |x: &(usize, &Point)| *(x.1) == point;
        let op = points.iter().enumerate().find(f);
        res = match op {
            Option::None => None,
            Some((i,_)) => Some(i)
        }
    }

    match res {
        Option::None => {
            points.push(point);
            return points.len() - 1;
        }
        Some(i) => {
            let len = ts.len();
            for (tr_index, tr) in ts.iter_mut().enumerate() {
                if (tr.v1 == i) | (tr.v2 == i) | (tr.v3 == i) {
                    triangle.add_neighbor(tr_index);
                    tr.add_neighbor(len);
                }
            }
            return i;
        }
    }
}


fn read_triangle<T: ReadBytesExt>(
    input: &mut T, points: &mut Vec<Point>,
    ts: &mut Vec<MeshTriangle>
) -> Result<MeshTriangle> {
    let normal = read_point(input)?.convert_to_vector();
    let v1 = read_point(input)?;
    let v2 = read_point(input)?;
    let v3 = read_point(input)?;
    let attr_count = input.read_u16::<LittleEndian>()?;

    let mut tr = MeshTriangle {
        normal: normal,
        v1: 0, v2: 0, v3: 0,
        attr_byte_count: attr_count,
        neighbors: Vec::new()
    };

    tr.v1 = add_point_to_vec(v1, points, &mut tr, ts);
    tr.v2 = add_point_to_vec(v2, points, &mut tr, ts);
    tr.v3 = add_point_to_vec(v3, points, &mut tr, ts);

    Ok(tr)
}

fn read_header<T: ReadBytesExt>(input: &mut T) -> Result<BinaryStlHeader> {
    let mut header = [0u8; 80];

    match input.read(&mut header) {
        Ok(n) => if n == header.len() {
            ()
        }
            else {
                return Err(Error::new(ErrorKind::Other,
                                      "Couldn't read STL header"));
            },
        Err(e) => return Err(e)
    };

    let num_triangles = input.read_u32::<LittleEndian>()?;

    Ok(BinaryStlHeader{ header: header, num_triangles: num_triangles })
}


pub fn read_stl<T: ReadBytesExt>(input: &mut T) -> Result<BinaryStlFile> {

    // read the header
    let header = read_header(input)?;

    let mut ts : Vec<MeshTriangle> = Vec::new();
    let mut ps : Vec<Point> = Vec::new();


    for _ in 0 .. header.num_triangles {
        let tr : MeshTriangle;
        {
            tr = read_triangle(input, &mut ps, &mut ts)?
        }
        ts.push(tr);
    }

    Ok(BinaryStlFile {
        header: header,
        triangles: ts,
        points: ps
    })
}

fn write_point<T: WriteBytesExt>(out: &mut T, p: &Point) -> Result<()> {

    out.write_f32::<LittleEndian>(p.x)?;
    out.write_f32::<LittleEndian>(p.y)?;
    out.write_f32::<LittleEndian>(p.z)?;

    Ok(())
}
/*
fn write_point<T: WriteBytesExt>(out: &mut T, p: [f32; 3]) -> Result<()> {
    for x in p.iter() {
        try!(out.write_f32::<LittleEndian>(*x));
    }
    Ok(())
}
*/


pub fn write_stl<T: WriteBytesExt>(
    out: &mut T,
    stl: &BinaryStlFile
) -> Result<()> {
    assert!(stl.header.num_triangles as usize == stl.triangles.len());

    //write the header.
    out.write(&stl.header.header)?;
    out.write_u32::<LittleEndian>(stl.header.num_triangles)?;

    // write all the triangles
    for t in stl.triangles.iter() {
        write_point(out, &t.normal.gen_point())?;
        write_point(out, &stl.points[t.v1])?;
        write_point(out, &stl.points[t.v2])?;
        write_point(out, &stl.points[t.v3])?;
        out.write_u16::<LittleEndian>(t.attr_byte_count)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{BinaryStlFile, BinaryStlHeader, write_stl, read_stl, MeshTriangle};
    use std::io::Cursor;
    use bo::point::Point;
    use bo::vector::Vector;
    //use std::io::prelude::*;
    use std::fs::File;

    #[test]
    fn write_read() {
        // Make sure we can write and read a simple file.
        let file = BinaryStlFile {
            header: BinaryStlHeader { header: [0u8; 80],
                num_triangles: 1 },
            triangles: vec![
                MeshTriangle {
                    normal: Vector::new(0f32, 1f32, 0f32),
                    v1: 0,
                    v2: 1,
                    v3: 2,
                    attr_byte_count: 0,
                    neighbors: Vec::new()
                }
            ],
            points: vec![
                Point::new(0f32, 0f32, 0f32),
                Point::new(0f32, 0f32, 1f32),
                Point::new(1f32, 0f32, 1f32),
            ]
        };

        let mut buffer = Vec::new();

        match write_stl(&mut buffer, &file) {
            Ok(_) => (),
            Err(_) => panic!()
        }

        match read_stl(&mut Cursor::new(buffer)) {
            Ok(stl) => {
                assert!(stl.header.num_triangles == file.header.num_triangles);
                assert!(stl.triangles.len() == 1);
                assert!(stl.triangles[0] == file.triangles[0])
            },
            Err(_) => panic!()
        }
    }

    #[test]
    fn file_write_read() {
        let b_stl_file = BinaryStlFile {
            header: BinaryStlHeader { header: [0u8; 80],
                num_triangles: 1 },
            triangles: vec![
                MeshTriangle {
                    normal: Vector::new(0f32, 1f32, 0f32),
                    v1: 0,
                    v2: 1,
                    v3: 2,
                    attr_byte_count: 0,
                    neighbors: Vec::new()
                }
            ],
            points: vec![
                Point::new(0f32, 0f32, 0f32),
                Point::new(0f32, 0f32, 1f32),
                Point::new(1f32, 0f32, 1f32),
            ]
        };
        let mut f = File::create("test.stl").unwrap();



        match write_stl(&mut f, &b_stl_file) {
            Ok(_) => (),
            Err(_) => panic!()
        };

        let mut f = File::open("test.stl").unwrap();

        match read_stl(&mut f) {
            Ok(stl) => {
                assert!(stl.header.num_triangles == b_stl_file.header.num_triangles);
                assert!(stl.triangles.len() == 1);
                assert!(stl.triangles[0] == b_stl_file.triangles[0]);
                assert!(stl.points[0] == b_stl_file.points[0]);
                assert!(stl.points[1] == b_stl_file.points[1]);
                assert!(stl.points[2] == b_stl_file.points[2]);
            },
            Err(_) => panic!()
        }
    }
}




