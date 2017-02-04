use bo::*;
use qm::*;
use cf::{Af,create};
use cf::intersect::plane_x_plane::{AfPxP, RafSimplePxP};
use cf::intersect::line_x_segment::{AfLxS, InfoLxS, RafSimpleLxS};
use cf::intersect::segment_x_segment::{AfSxS, InfoSxS, RafSimpleSxS};

#[derive(Debug)]
pub enum InfoTxT {
    Collinear,
    NotIntersecting,
    CoplanarIntersecting,
    CoplanarNotIntersecting,
    Intersecting
}

pub trait AfTxT : Af  {
    fn intersect(&mut self, tr1 : &Triangle, tr2 : &Triangle) -> (Option<Segment>, Option<Polygon>, InfoTxT);
}


#[derive(Default)]
pub struct RafSimpleTxT
<QM : QualityMetric = UselessQM,
    RafPxP : AfPxP = RafSimplePxP,
    RafLxS : AfLxS = RafSimpleLxS,
    RafSxS : AfSxS = RafSimpleSxS>
{
    pub qm : QM,
    pub raf_pxp : RafPxP,
    pub raf_lxs : RafLxS,
    pub raf_sxs : RafSxS
}

impl
<QM : QualityMetric,
    RafPxP : AfPxP,
    RafLxS : AfLxS,
    RafSxS : AfSxS>
Af for RafSimpleTxT<QM,RafPxP,RafLxS, RafSxS> {}

impl
<QM : QualityMetric,
    RafPxP : AfPxP,
    RafLxS : AfLxS,
    RafSxS : AfSxS>
 AfTxT for RafSimpleTxT<QM,RafPxP,RafLxS, RafSxS> {
    fn intersect(&mut self, tr1 : &Triangle, tr2 : &Triangle) -> (Option<Segment>, Option<Polygon>, InfoTxT) {
        let plane2 = tr2.gen_plane();

        if plane2.normal.is_zero() {
            panic!("plane normal is a zero vector!")
        }

        let dist1 = signed_distance(&tr1.p1, &plane2);
        let dist2 = signed_distance(&tr1.p2, &plane2);
        let dist3 = signed_distance(&tr1.p3, &plane2);

        //println!("plane2.normal = {} \n plane2.point = {} ", plane2.normal, plane2.point);
        //println!("d1 = {} \n d2 = {} \n d3 = {} \n", dist1, dist2, dist3);

        if eq_f32(dist1, 0.) & eq_f32(dist2, 0.) & eq_f32(dist3, 0.) {
            //TODO: IntersectingInThePlane
            return (None, None, InfoTxT::CoplanarNotIntersecting);
        }

        if eq_f32(dist1, dist2) & eq_f32(dist1, dist3) {
            return (None, None, InfoTxT::Collinear);
        }

        if (dist1 > 0.) & (dist2 > 0.) & (dist3 > 0.) | (dist1 < 0.) & (dist2 < 0.) & (dist3 < 0.) {
            return (None, None, InfoTxT::NotIntersecting);
        }

        let plane1 = tr1.gen_plane();

        let mut raf_pxp : RafPxP = create();
        let (op_line, _) = raf_pxp.intersect(&plane1, &plane2);
        let line = op_line.unwrap();

        //println!("line: {}\n", line);

        let s1 = intersect_line_and_triangle::<RafLxS>(&line, &tr1);
        let s2 = intersect_line_and_triangle::<RafLxS>(&line, &tr2);

        //println!("s1: {} \n s2: {} \n", s1, s2);

        let mut raf_sxs : RafSxS = create();
        let res = raf_sxs.intersect_segments_on_the_line(&s1,&s2);

        match res {
            (s, InfoSxS::IntersectingInASegment) => return (s, None, InfoTxT::Intersecting),
            _ => return (None, None, InfoTxT::NotIntersecting)
        }

    }
}

fn signed_distance(point : &Point, plane : &Plane) -> f32 {
    plane.normal.dot_product(&point.get_vector()) + plane.get_d()
}


fn intersect_line_and_triangle<RafLxS : AfLxS>(line : &Line, tr : &Triangle) -> Segment {
    let mut raf_lxs : RafLxS = create();

    let s1 = Segment {org: tr.p1.clone(), dest: tr.p2.clone()};
    let s2 = Segment {org: tr.p2.clone(), dest: tr.p3.clone()};
    let s3 = Segment {org: tr.p3.clone(), dest: tr.p1.clone()};

    let lp1 : bool = line.check_accessory(&tr.p1);
    let lp2 : bool = line.check_accessory(&tr.p2);
    let lp3 : bool = line.check_accessory(&tr.p3);

    //println!("lp1: {}, lp2: {}, lp3: {}", lp1, lp2, lp3);
    match (lp1,lp2,lp3) {
        // two points on the line
        (true, true, false) => return s1,
        (false, true, true) => return s2,
        (true, false, true) => return s3,

        // one point on the line
        (true, false, false) => {
            let res2 = raf_lxs.intersect(line, &s2);
            return  Segment {
                org:  s1.org,
                dest: res2.0.unwrap()
            };
        }

        (false, true, false) => {
            let res3 = raf_lxs.intersect(line, &s3);
            return  Segment {
                org:  s2.org,
                dest: res3.0.unwrap()
            };
        }

        (false, false, true) => {
            let res1 = raf_lxs.intersect(line, &s1);
            return  Segment {
                org:  s3.org,
                dest: res1.0.unwrap()
            };
        }

        // zero points on the line
        (false, false, false) => {
            let res1 = raf_lxs.intersect(line, &s1);
            let res2 = raf_lxs.intersect(line, &s2);
            let res3 = raf_lxs.intersect(line, &s3);
            match (res1.2, res2.2, res3.2) {
                (InfoLxS::IntersectingInAPoint, InfoLxS::IntersectingInAPoint, _) => {
                    return Segment {org: res1.0.unwrap(), dest: res2.0.unwrap()};
                }

                (InfoLxS::IntersectingInAPoint, _, InfoLxS::IntersectingInAPoint) => {
                    return Segment {org: res3.0.unwrap(), dest: res1.0.unwrap()};
                }

                (_, InfoLxS::IntersectingInAPoint, InfoLxS::IntersectingInAPoint) => {
                    return Segment {org: res2.0.unwrap(), dest: res3.0.unwrap()};
                }

                (r1, r2, r3) => panic!("Smth goes wrong! {:?} {:?} {:?}", r1, r2, r3)
            }
        }


        _ => panic!("Smth goes wrong!")
    }
}

#[cfg(test)]
mod tests {
    use cf::intersect::*;
    use cf::create;
    use bo::*;
    use std::option::Option;
    //use qm::*;

    #[test]
    fn triangles_in_the_plane() {
        let p1 = Point::new(1., 0., 0.);
        let p2 = Point::new(0., 1., 0.);
        let p3 = Point::new(1., 1., 0.);
        let tr1 = Triangle::new(p1,p2,p3);

        let p1 = Point::new(1., -10., 0.);
        let p2 = Point::new(0., -10., 0.);
        let p3 = Point::new(1., -9., 0.);
        let tr2 = Triangle::new(p1,p2,p3);

        let mut raf_simple_txt : RafSimpleTxT = create();
        let res = raf_simple_txt.intersect(&tr1, &tr2);

        if let (Option::None, Option::None, InfoTxT::CoplanarNotIntersecting) = res  {
            return;
        } else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn collinear_triangles() {
        let p1 = Point::new(1., 0., 0.);
        let p2 = Point::new(0., 1., 0.);
        let p3 = Point::new(1., 1., 0.);
        let tr1 = Triangle::new(p1,p2,p3);

        let p1 = Point::new(1., 0., 10.);
        let p2 = Point::new(0., 1., 10.);
        let p3 = Point::new(1., 1., 10.);
        let tr2 = Triangle::new(p1,p2,p3);

        //println!("tr1: {:?},\n tr2 {:?}", tr1, tr2);

        let mut raf_simple_txt : RafSimpleTxT = create();
        let res = raf_simple_txt.intersect(&tr1, &tr2);

        if let (Option::None, Option::None, InfoTxT::Collinear) = res  {
            return;
        } else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn intersect_triangles_1p_on_the_line() {
        let p1 = Point::new(-1., 0., 0.);
        let p2 = Point::new(1., 0., 0.);
        let p3 = Point::new(0., 0., 1.);
        let tr1 = Triangle::new(p1,p2,p3);

        let p1 = Point::new(0., -2., 0.);
        let p2 = Point::new(-2., 0., 0.);
        let p3 = Point::new(0., 1., 0.);
        let tr2 = Triangle::new(p1,p2,p3);

        let ep1 = Point {x: -1., y: 0., z: 0.};
        let ep2 = Point {x: 0., y: 0., z: 0.};

        let es = Segment {org: ep1, dest: ep2};

        //println!("tr1: {:?},\n tr2 {:?}", tr1, tr2);

        let mut raf_simple_txt : RafSimpleTxT = create();
        let res = raf_simple_txt.intersect(&tr1, &tr2);

        if let (Some(s), Option::None, InfoTxT::Intersecting) = res  {
            if s != es {
                panic!("Wrong result: {}", s);
            }
        } else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn intersect_triangles_2p_on_the_line() {
        let p1 = Point::new(-2., 0., 0.);
        let p2 = Point::new(0., 2., 0.);
        let p3 = Point::new(2., 0., 0.);
        let tr1 = Triangle::new(p1,p2,p3);

        let p1 = Point::new(-1., 1., 0.);
        let p2 = Point::new(0., 1., 1.);
        let p3 = Point::new(1., 1., 0.);
        let tr2 = Triangle::new(p1,p2,p3);

        let ep1 = Point {x: 1., y: 1., z: 0.};
        let ep2 = Point {x: -1., y: 1., z: 0.};

        let es = Segment {org: ep1, dest: ep2};

        //println!("tr1: {:?},\n tr2 {:?}", tr1, tr2);

        let mut raf_simple_txt : RafSimpleTxT = create();
        let res = raf_simple_txt.intersect(&tr1, &tr2);

        if let (Some(s), Option::None, InfoTxT::Intersecting) = res  {
            if s != es {
                panic!("Wrong result: {}", s);
            }
        } else {
            panic!("Wrong info: {:?}", res.2);
        };
    }


    //TODO: tests for intersection of triangles
}

