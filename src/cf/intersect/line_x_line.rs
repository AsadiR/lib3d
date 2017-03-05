use qm::*;
use bo::*;
use cf::Af;
use rulinalg::matrix::Matrix as rMatrix;
use rulinalg::vector::Vector as rVector;
use std::mem;


pub enum InfoLxL {
    Skew,
    Collinear,
    Coincidence,
    Intersecting
}

pub trait AfLxL : Af  {
    fn intersect(&mut self, a : &Line, b : &Line) -> (Option<Point>, InfoLxL);
}


#[derive(Default)]
pub struct RafSimpleLxL<QM : QualityMetric = UselessQM>{pub qm : QM}

impl<QM : QualityMetric>  Af for RafSimpleLxL<QM> {}

impl<QM : QualityMetric> AfLxL for RafSimpleLxL<QM> {

    // http://mathhelpplanet.com/static.php?p=vzaimnoe-raspolozhenie-pryamyh-v-prostranstve
    fn intersect(&mut self, a : &Line, b : &Line) -> (Option<Point>, InfoLxL) {
        // TODO полность переделать этот метод
        // TODO отладить поэлементное пересечение треугольников
        // TODO отобразить зоны пересечения и сравнить с действительностью


        //self.qm.start();
        let m1 : &Point = &a.org;
        let mut l1 : Vector = &a.dest - &a.org;
        let m2 : &Point = &b.org;
        let mut l2 : Vector = &b.dest - &b.org;
        let m : Vector = &b.org - &a.org;


        //Are lines skew?
        let mp = m.mixed_product(&l1,&l2);
        if !eq_f32(mp,0.0) {
            return (None, InfoLxL::Skew);
        }

        //Are lines coincidence?
        let c_cond = l1.is_collinear_to(&m);
        if c_cond {
            return (None, InfoLxL::Coincidence);
        }

        //Are lines parallel? If yes then return (None, None).
        let p_cond = l1.is_collinear_to(&l2);
        if !c_cond & p_cond {
            return (None, InfoLxL::Collinear);
        }


        /*
        1) m1.x + l1.x*t = m2.x + l2.x*s
        2) m1.y + l1.y*t = m2.y + l2.y*s
        3) m1.z + l1.z*t = m2.z + l2.z*s

        ax = y;

        t:       s:       y:
        l1.x    -l2.x   (m2.x - m1.x)
        l1.y    -l2.y   (m2.y - m1.y)
        l1.z    -l2.z   (m2.z - m1.z)
        */

        //I can improve it!
        let nv = l1.cross_product(&l2);

        let a = rMatrix::new(3, 3, vec![l1.x, -l2.x, nv.x,
                                        l1.y, -l2.y, nv.y,
                                        l1.z, -l2.z, nv.z]);
        let y = rVector::new(vec![m2.x-m1.x+nv.x, m2.y-m1.y+nv.y, m2.z-m1.z+nv.z]);


        //println!("matrix:");
        //println!("{}", a);
        //println!("{}", y);



        let x = a.solve(y).unwrap();
        //println!("{}", x);
        let (t,_) = (x[0],x[1]);

        let p = m1 + &(&l1 * t);

        //self.qm.end();
        (Some(p), InfoLxL::Intersecting)
    }
}

#[cfg(test)]
mod tests {
    use cf::intersect::*;
    use cf::create;
    use bo::*;
    //use qm::*;
    use std::option::Option;

    #[test]
    fn line_intersection_abc() {
        let p1 = Point { x: 1.0, y: 1.0, z: 1.0 };
        let p2 = Point { x: 0.0, y: 0.0, z: 0.0 };
        let p3 = Point { x: -1.0, y: -1.0, z: 1.0 };
        let p4 = Point { x: 0.0, y: 0.0, z: 0.0 };

        let l1 = Line { org: p1, dest: p2 };
        let l2 = Line { org: p3, dest: p4 };

        let mut simple_rli : RafSimpleLxL = create();
        let res = simple_rli.intersect(&l1, &l2);

        if let (Some(expected_p), InfoLxL::Intersecting) = res {
            if expected_p == (Point {x: 0.0, y: 0.0, z: 0.0}) {
                return;
            } else {
                panic!("Wrong result {}", expected_p);
            }
        } else {
            panic!("Wrong info");
        }
    }

    #[test]
    fn line_intersection_ab() {
        let p1 = Point { x: 1.0, y: 0.0, z: 1.0 };
        let p2 = Point { x: -1.0, y: 0.0, z: 1.0 };
        let p3 = Point { x: 0.0, y: 1.0, z: 1.0 };
        let p4 = Point { x: 0.0, y: -1.0, z: 1.0 };

        let l1 = Line { org: p1, dest: p2 };
        let l2 = Line { org: p3, dest: p4 };

        let mut simple_rli: RafSimpleLxL = create();
        let res = simple_rli.intersect(&l1, &l2);

        if let (Some(expected_p), InfoLxL::Intersecting) = res {
            if expected_p == (Point {x: 0.0, y: 0.0, z: 1.0}) {
                return;
            } else {
                panic!("Wrong result {}", expected_p);
            }
        } else {
            panic!("Wrong info");
        }
    }

    #[test]
    fn lines_skew() {
        let p1 = Point { x: 0.0, y: 0.0, z: 0.0 };
        let p2 = Point { x: 0.0, y: 0.0, z: 1.0 };
        let p3 = Point { x: 1.0, y: 6.0, z: 0.0 };
        let p4 = Point { x: 0.0, y: 6.0, z: 0.0 };

        let l1 = Line { org: p1, dest: p2 };
        let l2 = Line { org: p3, dest: p4 };

        let mut simple_rli: RafSimpleLxL = create();
        let res = simple_rli.intersect(&l1, &l2);

        if let (Option::None, InfoLxL::Skew) = res {
            return;
        } else {
            panic!("Wrong info");
        }
    }

    #[test]
    fn lines_coincidence() {
        let p1 = Point { x: 0.0, y: 0.0, z: 0.0 };
        let p2 = Point { x: 0.0, y: 0.0, z: 1.0 };
        let p3 = Point { x: 0.0, y: 0.0, z: -1.0 };
        let p4 = Point { x: 0.0, y: 0.0, z: 2.0 };

        let l1 = Line { org: p1, dest: p2 };
        let l2 = Line { org: p3, dest: p4 };

        let mut simple_rli: RafSimpleLxL = create();
        let res = simple_rli.intersect(&l1, &l2);

        if let (Option::None, InfoLxL::Coincidence) = res {
            return;
        } else {
            panic!("Wrong info");
        }
    }

    #[test]
    fn lines_parallel() {
        let p1 = Point { x: 1.0, y: 1.0, z: 1.0 };
        let p2 = Point { x: 0.0, y: 0.0, z: 1.0 };
        let p3 = Point { x: 1.0, y: 1.0, z: 0.0 };
        let p4 = Point { x: 0.0, y: 0.0, z: 0.0 };

        let l1 = Line { org: p1, dest: p2 };
        let l2 = Line { org: p3, dest: p4 };

        let mut simple_rli: RafSimpleLxL = create();
        let res = simple_rli.intersect(&l1, &l2);

        if let (Option::None, InfoLxL::Collinear) = res {
            return;
        } else {
            panic!("Wrong info");
        }
    }
}










