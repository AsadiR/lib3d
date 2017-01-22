use cf::Af;
use bo::eq_f32;
use bo::*;
use qm::*;

pub enum InfoLxP {
    Collinear,
    LineContainedInPlane,
    Intersecting
}

pub trait AfLxP : Af  {
    fn intersect(&mut self, l : &Line, p : &Plane) -> (Option<Point>, InfoLxP);
}

#[derive(Default)]
pub struct RafSimpleLxP<QM : QualityMetric = UselessQM>{pub qm : QM}

impl<QM : QualityMetric>  Af for RafSimpleLxP<QM> {}

impl<QM : QualityMetric> AfLxP for RafSimpleLxP<QM> {
    fn intersect(&mut self, l : &Line, p : &Plane) -> (Option<Point>, InfoLxP) {
        let dir_v = l.get_dir_vector();
        let dp = dir_v.dot_product(&p.normal);

        // numerator = (l.org - p.point)*p.normal
        let numerator = p.normal.dot_product(&(&l.org - &p.point));
        match 1 {
            _ if eq_f32(dp, 0.) & eq_f32(numerator, 0.) => (None, InfoLxP::LineContainedInPlane),
            _ if eq_f32(dp, 0.) => (None, InfoLxP::Collinear),
            _ => {
                let d = -numerator/dp;
                let point = &l.org + &(&dir_v*d);
                (Some(point), InfoLxP::Intersecting)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use cf::intersect::*;
    use cf::create;
    use bo::*;
    use qm::*;
    use std::option::Option;

    #[test]
    fn intersecting_line_and_plane() {
        // n*p - 1 = 0 <=> n*(p-p0)=0

        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(0.0, 2.0, 0.0);

        let l = Line { org: p1, dest: p2 };

        let n = Vector::new(1.0, 1.0, 1.0);
        let p0 = Point::new(1.0, 0.0, 0.0);

        let p = Plane { normal: n, point: p0};

        let mut raf_simple_lxp : RafSimpleLxP = create();
        let res = raf_simple_lxp.intersect(&l, &p);

        if let (Some(expected_p), InfoLxP::Intersecting) = res {
            if expected_p == (Point { x: 0.0, y: 1.0, z: 0.0 }) {
                return;
            } else {
                panic!("Wrong result {}", expected_p);
            }
            return;
        } else {
            panic!("Wrong info");
        }
    }

    #[test]
    fn collinear_line_and_plane() {
        // n*p - 1 = 0 <=> n*(p-p0)=0

        let p1 = Point::new(0.0, 2.0, 0.0);
        let p2 = Point::new(2.0, 0.0, 0.0);

        let l = Line { org: p1, dest: p2 };

        let n = Vector::new(1.0, 1.0, 1.0);
        let p0 = Point::new(1.0, 0.0, 0.0);

        let p = Plane { normal: n, point: p0};

        let mut raf_simple_lxp : RafSimpleLxP = create();
        let res = raf_simple_lxp.intersect(&l, &p);

        if let (Option::None, InfoLxP::Collinear) = res {
            return;
        } else {
            panic!("Wrong info");
        }
    }

    #[test]
    fn line_contained_in_plane() {
        // n*p - 1 = 0 <=> n*(p-p0)=0

        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);

        let l = Line { org: p1, dest: p2 };

        let n = Vector::new(1.0, 1.0, 1.0);
        let p0 = Point::new(1.0, 0.0, 0.0);

        let p = Plane { normal: n, point: p0};

        let mut raf_simple_lxp : RafSimpleLxP = create();
        let res = raf_simple_lxp.intersect(&l, &p);

        if let (Option::None, InfoLxP::LineContainedInPlane) = res {
            return;
        } else {
            panic!("Wrong info");
        }
    }
}