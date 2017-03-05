use bo::*;
use qm::*;
use cf::Af;
use std::clone::Clone;
use cf::intersect::line_x_line::{AfLxL, InfoLxL, RafSimpleLxL};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum InfoLxS {
    IntersectingInASegment,
    IntersectingInAPoint,
    Collinear,
    DisjointInThePlane,
    Skew
}

// You can use different algorithms to implement this trait
pub trait AfLxS: Af  {
    fn intersect(&mut self, a : &Line, b : &Segment) -> (Option<Point>, Option<Segment>, InfoLxS);
}


#[derive(Default)]
pub struct RafSimpleLxS
<QM  : QualityMetric = UselessQM, RafLxL: AfLxL = RafSimpleLxL>
{
    pub qm : QM,
    pub raf_lxl: RafLxL
}

impl<QM  : QualityMetric, RafLxL : AfLxL> Af for RafSimpleLxS<QM,RafLxL> {}


impl<QM  : QualityMetric, RafLxL : AfLxL> AfLxS for RafSimpleLxS<QM,RafLxL> {
    fn intersect(&mut self, line : &Line, segment : &Segment) -> (Option<Point>, Option<Segment>, InfoLxS) {
        let la = if line.org >= line.dest  {
            Line {org: line.dest.clone(), dest: line.org.clone()}
        } else {
            Line {org: line.org.clone(), dest: line.dest.clone()}
        };

        //println!("s {}", segment);
        let lb = if segment.org >= segment.dest {
            //println!("swaped\n");
            Line {org: segment.dest.clone(), dest: segment.org.clone()}
        } else {
            Line {org: segment.org.clone(), dest: segment.dest.clone()}
        };
        //println!("my la {}", la);
        //println!("lb {}", lb);

        let (sp, info) = self.raf_lxl.intersect(&la, &lb);
        match info {
            InfoLxL::Skew => (None, None, InfoLxS::Skew),
            InfoLxL::Collinear => (None, None, InfoLxS::Collinear),
            InfoLxL::Coincidence => {
                //let (os, info) = intersect_segments_on_the_line(&la.convert_to_segment(), &lb.convert_to_segment());
                (None, Some(lb.convert_to_segment()), InfoLxS::IntersectingInASegment)
            },


            InfoLxL::Intersecting => {
                let p = sp.unwrap();
                //println!("point: {:?}, lb.org: {:?}, lb.dest: {:?}", p, lb.org, lb.dest);
                if (p >= lb.org) & (p <= lb.dest) {
                    (Some(p), None, InfoLxS::IntersectingInAPoint)
                } else {
                    //println!("Bad: {}, lb.org: {}, {}", p, lb.org, (p >= lb.org));
                    (None, None, InfoLxS::DisjointInThePlane)
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
    //use qm::*;

    #[test]
    fn point_intersection() {
        let p1 = Point {x: 10., y: 0.0, z: 0.0};
        let p2 = Point {x: 9., y: 0.0, z: 0.0};
        let p3 = Point {x: 0.0, y: 1.0, z: 0.0};
        let p4 = Point {x: 0.0, y: -1.0, z: 0.0};

        let line = Line {org: p1, dest: p2};
        let segment = Segment {org: p3, dest: p4};

        let mut raf_simple_lxs : RafSimpleLxS = create();
        let res = raf_simple_lxs.intersect(&line, &segment);

        if let (Some(p),Option::None, InfoLxS::IntersectingInAPoint) = res  {
            if p != (Point {x:0., y:0., z:0.}) {
                panic!("Wrong result: {}", p);
            }
        } else {
            panic!("Wrong info: {:?}", res.2);
        };

    }

}

