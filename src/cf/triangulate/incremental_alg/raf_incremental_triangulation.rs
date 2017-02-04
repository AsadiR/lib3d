use cf::triangulate::AfTriangulate;
use cf::Af;
use bo::*;
use qm::*;
use std::collections::BTreeSet;
use std::f32;


#[derive(Default)]
pub struct RafIncrementalTriangulation<QM : QualityMetric = UselessQM>{pub qm : QM}

impl<QM : QualityMetric>  Af for RafIncrementalTriangulation<QM> {}

impl<QM : QualityMetric> AfTriangulate for RafIncrementalTriangulation<QM> {
    fn triangulate(&mut self, mut points : Vec<Point>) -> Vec<Triangle> {
        assert!(points.len() >= 3, "Not enough points!");

        let mut ts : Vec<Triangle> = Vec::new();

        let mut p = Point::new(0.,0.,0.);
        let mut orientation : f32 = 0.;
        let mut normal_ = get_normal(&points, & mut orientation);
        let mut d_ = normal_.dot_product(&points[0].get_vector());

        check_points(&normal_, &d_, &points);
        let normal_type : NormalType = classify_normal(&normal_);

        modify_points(&mut points, &normal_type, &mut orientation, &mut normal_, &mut d_);

        if orientation < 0. {
            normal_ = &normal_ * -1.;
            d_ = normal_.dot_product(&points[0].get_vector());
        }
        check_points(&normal_, &d_, &points);

        //let n = points.len();
        let e : Segment = hull_edge(&mut points);
        println!("hull_edge_res = {}\n", e);

        let mut frontier : BTreeSet<Segment> = BTreeSet::new();
        frontier.insert(e);

        while !frontier.is_empty() {
            let e = frontier.iter().next_back().unwrap().clone();
            frontier.remove(&e);

            if mate(&e, &mut points, &mut p, &normal_, &d_) {
                println!("mate_point = {} \n", p);

                update_frontier(&mut frontier, &p, &e.org);
                update_frontier(&mut frontier, &e.dest, &p);
                let tr : Triangle;
                if orientation < 0. {
                    tr = Triangle {
                        p1: inv_point_transform(&e.org, &normal_type),
                        p2: inv_point_transform(&e.dest, &normal_type),
                        p3: inv_point_transform(&p, &normal_type)
                    }
                } else {
                    tr = Triangle {
                        p1: inv_point_transform(&e.dest, &normal_type),
                        p2: inv_point_transform(&e.org, &normal_type),
                        p3: inv_point_transform(&p, &normal_type)
                    }
                }
                ts.push(tr);
            }
        }

        return ts;
    }
}

fn inv_point_transform(p : &Point, normal_type : &NormalType) -> Point {
    let mut pc = p.clone();
    match *normal_type {
        //NormalType::ABC => {},
        NormalType::AB => {
            pc.swap_xy();
            pc.swap_yz();
        },
        //NormalType::AC => {},
        //NormalType::BC => {},
        NormalType::A => {
            pc.swap_xy();
            pc.swap_xz();
        },

        NormalType::B => {
            pc.swap_xy();
            pc.swap_yz();
        },
        //NormalType::C => {}
        _ => {}
    };
    return pc;
}

fn hull_edge(points : &mut Vec<Point>) -> Segment {
    let mut m = 0;
    let n = points.len();
    for i in 1..n {
        if points[i] < points[m] {
            m = i;
        }
    }
    points.swap(0,m);
    m = 1;
    for i in 2..n {
        let c = points[i].classify(&points[0], &points[m]);
        if (c == EClassify::Left) | (c == EClassify::Between) {
            m = i;
        }
    }

    Segment{
        org: points[0].clone(),
        dest: points[m].clone()
    }
}

fn mate(
    e : &Segment,
    points : &mut Vec<Point>,
    p : &mut Point,
    normal_ : &Vector,
    d_ : &f32
) -> bool {
    let n = points.len();
    let mut bestp : Option<Point> = None;
    let mut t : f32 = 0.;
    let mut bestt : f32 = f32::MAX;
    let mut f = e.clone();
    f.rot(normal_, d_);

    println!("mate_info:");
    println!("normal: {}", normal_);
    println!("d: {}", d_);
    println!("e: {}", e);
    println!("rot_e: {}", f);
    println!("\n");


    for i in 0..n {
        let c = points[i].classify(&e.org, &e.dest);
        if c == EClassify::Right {
            let mut g = Segment {
                org: e.dest.clone(),
                dest: points[i].clone()
            };
            g.rot(normal_, d_);
            f.intersect(&g, &mut t, normal_, d_);
            if t < bestt {
                bestp = Some(points[i].clone());
                bestt = t;
            }
        }
    }
    match bestp {
        Some(pt) => {
            *p = pt;
            true
        },
        Option::None => false
    }
}

fn update_frontier(
    frontier : &mut BTreeSet<Segment>,
    a : &Point,
    b : &Point
) {
    let mut e = Segment{
        org: a.clone(),
        dest: b.clone()
    };
    if frontier.contains(&e) {
        frontier.remove(&e);
    } else {
        e.flip();
        frontier.insert(e);
    }
}



fn get_normal(points : & Vec<Point>, orientation : &mut f32) -> Vector {
    for i in 0..(points.len()-2) {
        let v1 = &points[i] - &points[i+1];
        let v2 = &points[i+1] - &points[i+2];
        let mut normal = v1.cross_product(&v2);
        let iv = Vector::new(1., 0., 0.);
        let jv = Vector::new(0., 1., 0.);
        *orientation = iv.mixed_product(&jv, &normal);
        if !normal.is_zero() {
            normal.normalize();
            return normal;
        }
    }
    panic!("All points are collinear");
}

fn check_points(normal_ : &Vector, d_ : &f32, points : &Vec<Point> ) {
    //println!("normal: {}\n", normal_);
    //println!("d: {}\n", d_);
    for i in 0..points.len() {
        //println!("point: {} \n", points[i]);
        assert!(is_point_in_plane(&points[i], normal_, d_), "Point are not co-planar!");
    }
}

fn is_point_in_plane(point : &Point, normal : &Vector, d : &f32) -> bool {
    eq_f32(normal.dot_product(&point.get_vector()) - d, 0.)
}

enum NormalType {
    ABC, AB, AC, BC, A, B, C
}

fn classify_normal(n : &Vector) -> NormalType {
    let nx = eq_f32(n.x, 0.);
    let ny = eq_f32(n.y, 0.);
    let nz = eq_f32(n.z, 0.);

    match (nx, ny, nz) {
        (false, false, false) => return NormalType::ABC,
        (false, false, true)  => return NormalType::AB,
        (false, true, false)  => return NormalType::AC,
        (true, false, false)  => return NormalType::BC,
        (false, true, true)   => return NormalType::A,
        (true, false, true)   => return NormalType::B,
        (true, true, false)   => return NormalType::C,
        _ => panic!("Normal vector cannot be zero!")
    }
}

fn modify_points(
    points : &mut Vec<Point>,
    nt : &NormalType,
    orientation : &mut f32,
    normal_ : &mut Vector,
    d_ : &mut f32
) {
    match *nt {
        NormalType::ABC => return,
        NormalType::AB => {
            for p in &mut *points {
                p.swap_yz();
                p.swap_xy();
            }
            *normal_ = get_normal(points, orientation);
            *d_ = normal_.dot_product(&points[0].get_vector());
            return;
        },
        NormalType::AC => return,
        NormalType::BC => return,
        NormalType::A => {
            for p in &mut *points {
                p.swap_xz();
                p.swap_xy();
            }
            *normal_ = get_normal(points, orientation);
            *d_ = normal_.dot_product(&points[0].get_vector());
            return;
        }
        NormalType::B => {
            for p in &mut *points {
                p.swap_yz();
                p.swap_xy();
            }
            *normal_ = get_normal(points, orientation);
            *d_ = normal_.dot_product(&points[0].get_vector());
            return;
        }
        NormalType::C => return,
    }
}


#[cfg(test)]
mod tests {
    use cf::triangulate::*;
    use cf::create;
    //use bo::*;
    //use qm::*;
    use std::fs::File;

/*
    #[test]
    fn triangles_in_the_plane() {
        let mut v = vec![1,2,3,4,5];
        let vr = &mut v;

        for x in &mut *vr {
            *x *= 2;
        }
        //let v : Vec<i32> = v.iter_mut().map(f).collect();

        println!("{:?}", *vr);
        //panic!();
    }
*/


    #[test]
    fn triangulation_abc() {
        // x+y+z = 1
        let a = Point::new(0.0, 0.0, 1.0);
        let b = Point::new(0.5, 0.0, 0.5);
        let c = Point::new(1.0, 0.0, 0.0);
        let d = Point::new(0.5, 0.5, 0.0);
        let e = Point::new(0.0, 1.0, 0.0);
        let f = Point::new(1./3., 1./3., 1./3.);

        let ps = vec![a, b, c, d, e, f];

        let mut raf_inc_tr : RafIncrementalTriangulation = create();
        let ts = raf_inc_tr.triangulate(ps.clone());

        println!("len: {} \n", ts.len());
        println!("vec: {:?} \n", ts);

        assert!(ts.len() == 5);

        let mut mesh = Mesh::new();
        mesh.add_triangles(ts);


        let mut f = File::create("res_of_tests/inc_tr/test_abc.stl").unwrap();

        assert!(mesh.points.len() == 6);

        match mesh.write_stl(&mut f) {
            Ok(_) => (),
            Err(_) => panic!()
        };
    }

    #[test]
    fn triangulation_ab() {
        // x+y = 1
        let a = Point::new(0.0, 1.0, 0.0);
        let b = Point::new(1.0, 0.0, 1.0);
        let c = Point::new(-1.0, 2.0, 1.0);
        let d = Point::new(1.0, 0.0, -1.0);
        let e = Point::new(-1.0, 2.0, -1.0);

        let ps = vec![a, b, c, d, e];

        let mut raf_inc_tr : RafIncrementalTriangulation = create();
        let ts = raf_inc_tr.triangulate(ps.clone());

        println!("len: {} \n", ts.len());
        println!("vec: {:?} \n", ts);

        assert!(ts.len() == 4);

        let mut mesh = Mesh::new();
        mesh.add_triangles(ts);


        let mut f = File::create("res_of_tests/inc_tr/test_ab.stl").unwrap();

        assert!(mesh.points.len() == 5);

        match mesh.write_stl(&mut f) {
            Ok(_) => (),
            Err(_) => panic!()
        };
    }

    #[test]
    fn triangulation_ac() {
        // x+z = 1
        let a = Point::new(-1.0, 0.0, 2.0);
        let b = Point::new(1.0, 2.0, 0.0);
        let c = Point::new(-2.0, -1.0, 3.0);
        let d = Point::new(-1./3., -1./3., 4./3.);
        let e = Point::new(5., -3., -4.);

        let ps = vec![a, b, c, d, e];

        let mut raf_inc_tr : RafIncrementalTriangulation = create();
        let ts = raf_inc_tr.triangulate(ps.clone());

        println!("len: {} \n", ts.len());
        println!("vec: {:?} \n", ts);

        assert!(ts.len() == 4);

        let mut mesh = Mesh::new();
        mesh.add_triangles(ts);


        let mut f = File::create("res_of_tests/inc_tr/test_ac.stl").unwrap();

        assert!(mesh.points.len() == 5);

        match mesh.write_stl(&mut f) {
            Ok(_) => (),
            Err(_) => panic!()
        };
    }

    #[test]
    fn triangulation_bc() {
        // y+z = 1
        let a = Point::new(-1., -1., 2.);
        let b = Point::new(1./2., 1./2., 1./2.);
        let c = Point::new(2., 0., 1.);
        let d = Point::new(3., -1., 2.);
        let e = Point::new(5., 1., 0.);

        let ps = vec![a, b, c, d, e];

        let mut raf_inc_tr : RafIncrementalTriangulation = create();
        let ts = raf_inc_tr.triangulate(ps.clone());

        println!("len: {} \n", ts.len());
        println!("vec: {:?} \n", ts);

        assert!(ts.len() == 4);

        let mut mesh = Mesh::new();
        mesh.add_triangles(ts);


        let mut f = File::create("res_of_tests/inc_tr/test_bc.stl").unwrap();

        assert!(mesh.points.len() == 5);

        match mesh.write_stl(&mut f) {
            Ok(_) => (),
            Err(_) => panic!()
        };
    }

    #[test]
    fn triangulation_a() {
        // y+z = 1
        let a = Point::new(1., 2., 0.);
        let b = Point::new(1., 0., 2.);
        let c = Point::new(1., 0., 0.);
        let d = Point::new(1., 2., 2.);
        let e = Point::new(1., 1., 3.);

        let ps = vec![a, b, c, d, e];

        let mut raf_inc_tr : RafIncrementalTriangulation = create();
        let ts = raf_inc_tr.triangulate(ps.clone());

        println!("len: {} \n", ts.len());
        println!("vec: {:?} \n", ts);

        assert!(ts.len() == 3);

        let mut mesh = Mesh::new();
        mesh.add_triangles(ts);


        let mut f = File::create("res_of_tests/inc_tr/test_a.stl").unwrap();

        assert!(mesh.points.len() == 5);

        match mesh.write_stl(&mut f) {
            Ok(_) => (),
            Err(_) => panic!()
        };
    }

    #[test]
    fn triangulation_b() {
        // y = 1
        let a = Point::new(-4., 1., 3.);
        let b = Point::new(-2., 1., 0.);
        let c = Point::new(-1., 1., 2.);
        let d = Point::new(1./3., 1., 1./3.);
        let e = Point::new(5., 1., -1.);

        let ps = vec![a, b, c, d, e];

        let mut raf_inc_tr : RafIncrementalTriangulation = create();
        let ts = raf_inc_tr.triangulate(ps.clone());

        println!("len: {} \n", ts.len());
        println!("vec: {:?} \n", ts);

        assert!(ts.len() == 4);

        let mut mesh = Mesh::new();
        mesh.add_triangles(ts);


        let mut f = File::create("res_of_tests/inc_tr/test_b.stl").unwrap();

        assert!(mesh.points.len() == 5);

        match mesh.write_stl(&mut f) {
            Ok(_) => (),
            Err(_) => panic!()
        };
    }

    #[test]
    fn triangulation_c() {
        // z = 1
        let a = Point::new(1., 1., 1.);
        let b = Point::new(1., 2., 1.);
        let c = Point::new(-1., 1., 1.);
        let d = Point::new(-1., 2., 1.);
        let e = Point::new(-1., 3., 1.);
        let f = Point::new(1., 3., 1.);

        let ps = vec![a, b, c, d, e, f];

        let mut raf_inc_tr : RafIncrementalTriangulation = create();
        let ts = raf_inc_tr.triangulate(ps.clone());

        println!("len: {} \n", ts.len());
        println!("vec: {:?} \n", ts);

        assert!(ts.len() == 4);

        let mut mesh = Mesh::new();
        mesh.add_triangles(ts);


        let mut f = File::create("res_of_tests/inc_tr/test_b.stl").unwrap();

        assert!(mesh.points.len() == 6);

        match mesh.write_stl(&mut f) {
            Ok(_) => (),
            Err(_) => panic!()
        };
    }





}