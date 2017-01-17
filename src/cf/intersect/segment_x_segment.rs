
use bo::*;
use qm::*;
use cf::Af;
use std::option;
use core::default::Default;
use std::clone::Clone;
use cf::intersect::line_x_line::{AfLxL, InfoLxL, RafSimpleLxL};

#[derive(Debug)]
pub enum InfoSxS {
    SegmentIntersection,
    PointIntersection,
    Collinear,
    NoIntersectionOnTheLine,
    NoIntersectionInThePlane,
    Skew
}

// You can use different algorithms to implement this trait
pub trait AfSxS: Af  {
    fn intersect(&mut self, a : &Segment, b : &Segment) -> (Option<Point>, Option<Segment>, InfoSxS);
}


#[derive(Default)]
pub struct RafSimpleSxS
<QM  : QualityMetric = UselessQM, RLI : AfLxL = RafSimpleLxL>
{
    pub qm : QM,
    pub rli: RLI
}

impl<QM  : QualityMetric, RLI : AfLxL> Af for RafSimpleSxS<QM,RLI> {}

// Intersect segments lying on the same line
pub fn intersect_segments_on_the_line(sa : &Segment, sb : &Segment) -> (Option<Segment>, InfoSxS) {
    match 1 {
        _ if (sa.org <= sb.org) &
            (sa.dest >= sb.dest) => {
            (Some(sb.clone()), InfoSxS::SegmentIntersection)
        },
        _ if (sb.org <= sa.org) &
            (sb.dest >= sa.dest) => {
            (Some(sa.clone()), InfoSxS::SegmentIntersection)
        },
        _ if (sa.dest > sb.org) &
            (sa.dest < sb.dest) => {
            (Some(Segment {org: sb.org.clone(), dest: sa.dest.clone()}),
             InfoSxS::SegmentIntersection)
        },
        _ if (sb.dest > sa.org) &
            (sb.dest < sa.dest) => {
            (Some(Segment {org: sa.org.clone(), dest: sb.dest.clone()}),
             InfoSxS::SegmentIntersection)
        },
        _ => (None, InfoSxS::NoIntersectionOnTheLine)
    }
}

impl<QM  : QualityMetric, RLI : AfLxL> AfSxS for RafSimpleSxS<QM,RLI> {

    // http://mathhelpplanet.com/static.php?p=vzaimnoe-raspolozhenie-pryamyh-v-prostranstve
    fn intersect(&mut self, a : &Segment, b : &Segment) -> (Option<Point>, Option<Segment>, InfoSxS) {
        let la = if a.org >= a.dest  {
            Line {org: a.dest.clone(), dest: a.org.clone()}
        } else {
            Line {org: a.org.clone(), dest: a.dest.clone()}
        };

        let lb = if b.org >= b.dest {
            Line {org: b.dest.clone(), dest: b.org.clone()}
        } else {
            Line {org: b.org.clone(), dest: b.dest.clone()}
        };
        //println!("la {}", a);
        //println!("lb {}", b);

        let (sp, info) = self.rli.intersect(&la, &lb);
        match info {
            InfoLxL::Skew => (None, None, InfoSxS::Skew),
            InfoLxL::Collinear => (None, None, InfoSxS::Collinear),
            InfoLxL::Coincidence => {
                let (os, info) = intersect_segments_on_the_line(&la.convert_to_segment(), &lb.convert_to_segment());
                (None, os, info)
            },
            InfoLxL::Intersect => {
                let p = sp.unwrap();
                if (p >= la.org) & (p <= la.dest) & (p >= lb.org) & (p <= lb.dest) {
                    //println!("Good");
                    (Some(p), None, InfoSxS::PointIntersection)
                } else {
                    //println!("Bad: {}, lb.org: {}, {}", p, lb.org, (p >= lb.org));
                    (None, None, InfoSxS::NoIntersectionInThePlane)
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use cf::intersect::*;
    use cf::create;
    use bo::*;
    use std::option::Option;
    use qm::*;

    #[test]
    fn point_intersection() {
        let p1 = Point {x: 1.0, y: 0.0, z: 0.0};
        let p2 = Point {x: -1.0, y: 0.0, z: 0.0};
        let p3 = Point {x: 0.0, y: 1.0, z: 0.0};
        let p4 = Point {x: 0.0, y: -1.0, z: 0.0};

        let s1 = Segment {org: p1, dest: p2};
        let s2 = Segment {org: p3, dest: p4};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Some(p),Option::None, InfoSxS::PointIntersection) = res  {
            if p != (Point {x:0., y:0., z:0.}) {
                panic!("Wrong result: {}", p);
            }
        } else {
            panic!("Wrong info: {:?}", res.2);
        };

    }

    #[test]
    fn no_intersection_in_the_plane() {
        let p1 = Point {x: -1.0, y: 0.0, z: 0.0};
        let p2 = Point {x: -2.0, y: 0.0, z: 0.0};
        let p3 = Point {x: 0.0, y: 1.0, z: 0.0};
        let p4 = Point {x: 0.0, y: -1.0, z: 0.0};

        let s1 = Segment {org: p1, dest: p2};
        let s2 = Segment {org: p3, dest: p4};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Option::None, Option::None, InfoSxS::NoIntersectionInThePlane) = res  {}
        else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn skew_segments() {
        let p1 = Point {x: -1.0, y: 0.0, z: 1.0};
        let p2 = Point {x: -2.0, y: 0.0, z: 1.0};
        let p3 = Point {x: 0.0, y: 1.0, z: 0.0};
        let p4 = Point {x: 0.0, y: -1.0, z: 0.0};

        let s1 = Segment {org: p1, dest: p2};
        let s2 = Segment {org: p3, dest: p4};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Option::None, Option::None, InfoSxS::Skew) = res  {}
        else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn collinear_segments() {
        let p1 = Point {x: -1., y: 0., z: 1.};
        let p2 = Point {x: -2., y: 0., z: 1.};
        let p3 = Point {x: -1., y: 0., z: 0.};
        let p4 = Point {x: 5., y: 0., z: 0.};

        let s1 = Segment {org: p1, dest: p2};
        let s2 = Segment {org: p3, dest: p4};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Option::None, Option::None, InfoSxS::Collinear) = res  {}
        else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn coincidence_segments() {
        let p1 = Point {x: -1., y: 0., z: 0.};
        let p2 = Point {x: -2., y: 0., z: 0.};
        let p3 = Point {x: -1., y: 0., z: 0.};
        let p4 = Point {x: -2., y: 0., z: 0.};

        let s1 = Segment {org: p1, dest: p2};
        let s2 = Segment {org: p3, dest: p4};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Option::None, Some(s), InfoSxS::SegmentIntersection) = res  {
            let pt1 = Point {x: -2., y: 0., z: 0.};
            let pt2 = Point {x: -1., y: 0., z: 0.};
            let expected_s = Segment {org: pt1, dest: pt2};
            if s != expected_s {
                panic!("Wrong result: {}", s);
            }
        } else {
                panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn coincidence_segments_s1_gr_s2() {
        let p1 = Point {x: -5., y: 0., z: 0.};
        let p2 = Point {x: 5., y: 0., z: 0.};
        let p3 = Point {x: -1., y: 0., z: 0.};
        let p4 = Point {x: 2., y: 0., z: 0.};

        let s1 = Segment {org: p1, dest: p2};
        let s2 = Segment {org: p3, dest: p4};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Option::None, Some(s), InfoSxS::SegmentIntersection) = res  {
            let pt1 = Point {x: -1., y: 0., z: 0.};
            let pt2 = Point {x: 2., y: 0., z: 0.};
            let expected_s = Segment {org: pt1, dest: pt2};
            if s != expected_s {
                panic!("Wrong result: {}", s);
            }
        } else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn coincidence_segments_s2_gr_s1() {
        let p1 = Point {x: -1., y: 0., z: 0.};
        let p2 = Point {x: 2., y: 0., z: 0.};
        let p3 = Point {x: -5., y: 0., z: 0.};
        let p4 = Point {x: 5., y: 0., z: 0.};

        let s1 = Segment {org: p1, dest: p2};
        let s2 = Segment {org: p3, dest: p4};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Option::None, Some(s), InfoSxS::SegmentIntersection) = res  {
            let pt1 = Point {x: -1., y: 0., z: 0.};
            let pt2 = Point {x: 2., y: 0., z: 0.};
            let expected_s = Segment {org: pt1, dest: pt2};
            if s != expected_s {
                panic!("Wrong result: {}", s);
            }
        } else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn partial_coincidence1() {
        let p1 = Point {x: -2., y: 0., z: 0.};
        let p2 = Point {x: 2., y: 0., z: 0.};
        let p3 = Point {x: 1., y: 0., z: 0.};
        let p4 = Point {x: 3., y: 0., z: 0.};

        let s1 = Segment {org: p1, dest: p2};
        let s2 = Segment {org: p3, dest: p4};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Option::None, Some(s), InfoSxS::SegmentIntersection) = res  {
            let pt1 = Point {x: 1., y: 0., z: 0.};
            let pt2 = Point {x: 2., y: 0., z: 0.};
            let expected_s = Segment {org: pt1, dest: pt2};
            if s != expected_s {
                panic!("Wrong result: {}", s);
            }
        } else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn partial_coincidence1_flipped() {
        let p1 = Point {x: -2., y: 0., z: 0.};
        let p2 = Point {x: 2., y: 0., z: 0.};
        let p3 = Point {x: 1., y: 0., z: 0.};
        let p4 = Point {x: 3., y: 0., z: 0.};

        let s1 = Segment {org: p2, dest: p1};
        let s2 = Segment {org: p4, dest: p3};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Option::None, Some(s), InfoSxS::SegmentIntersection) = res  {
            let pt1 = Point {x: 1., y: 0., z: 0.};
            let pt2 = Point {x: 2., y: 0., z: 0.};
            let expected_s = Segment {org: pt1, dest: pt2};
            if s != expected_s {
                panic!("Wrong result: {}", s);
            }
        } else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn partial_coincidence2() {
        let p1 = Point {x: 1., y: 0., z: 0.};
        let p2 = Point {x: 3., y: 0., z: 0.};
        let p3 = Point {x: -2., y: 0., z: 0.};
        let p4 = Point {x: 2., y: 0., z: 0.};

        let s1 = Segment {org: p1, dest: p2};
        let s2 = Segment {org: p3, dest: p4};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Option::None, Some(s), InfoSxS::SegmentIntersection) = res  {
            let pt1 = Point {x: 1., y: 0., z: 0.};
            let pt2 = Point {x: 2., y: 0., z: 0.};
            let expected_s = Segment {org: pt1, dest: pt2};
            if s != expected_s {
                panic!("Wrong result: {}", s);
            }
        } else {
            panic!("Wrong info: {:?}", res.2);
        };
    }

    #[test]
    fn no_intersection_on_the_line() {
        let p1 = Point {x: 2., y: 0., z: 0.};
        let p2 = Point {x: 4., y: 0., z: 0.};
        let p3 = Point {x: -2., y: 0., z: 0.};
        let p4 = Point {x: -4., y: 0., z: 0.};

        let s1 = Segment {org: p1, dest: p2};
        let s2 = Segment {org: p3, dest: p4};

        let mut raf_simple_sxs : RafSimpleSxS = create();
        let res = raf_simple_sxs.intersect(&s1, &s2);

        if let (Option::None, Option::None, InfoSxS::NoIntersectionOnTheLine) = res  {}
        else {
            panic!("Wrong info: {:?}", res.2);
        };
    }


}

