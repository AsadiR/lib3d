use qm::*;
use bo::*;
use cf::Af;
use rulinalg::matrix::Matrix as rMatrix;
use rulinalg::vector::Vector as rVector;



pub enum InfoPxP {
    Coincidence,
    Collinear,
    Intersecting
}

pub trait AfPxP : Af  {
    fn intersect(&mut self, p1 : &Plane, p2 : &Plane) -> (Option<Line>, InfoPxP);
}


#[derive(Default)]
pub struct RafSimplePxP<QM : QualityMetric = UselessQM>{pub qm : QM}

impl<QM : QualityMetric>  Af for RafSimplePxP<QM> {}

impl<QM : QualityMetric> AfPxP for RafSimplePxP<QM> {
    fn intersect(&mut self, plane1 : &Plane, plane2 : &Plane) -> (Option<Line>, InfoPxP) {

        // (p - p0)*n = p*n + d => d = -p0*n
        let a = plane1.normal.cross_product(&plane2.normal);
        let d1 = plane1.get_d();
        let d2 = plane2.get_d();
        let n1 = &plane1.normal;
        let n2 = &plane2.normal;

        if a.is_zero() {
            //println!("d1: {}     d2: {}", d1, d2);
            if eq_f32(d1, d2) {
                return (None, InfoPxP::Coincidence);
            }
            return (None, InfoPxP::Collinear);
        }

        /*
        a - direction vector of the line of intersection
        a = n1 x n2
        n1 u = -d1
        n2 u = -d2
        u - point on the line of intersection
        if a.z != 0 then:
            m = [n1 n2 k]^T
            k = [0 0 1]^T
            b = [-d1 -d2]^T
            m u = b
         else
            u.z == const
            m = [n1 n2 k]^T
            k = [1 0 0]^T | k = [0 1 0]^T
            b = [-d1 -d2]^T
            m u = b
        */

        let m : rMatrix<f32>;
        let b : rVector<f32> = rVector::new(vec![-d1, -d2, 0.]);

        let mv = match 1 {
            _ if !eq_f32(a.z, 0.) =>
                vec![n1.x, n1.y, n1.z,
                     n2.x, n2.y, n2.z,
                     0.0,  0.0,  1.0],

            _ if ! eq_f32(a.y, 0.) =>
                vec![n1.x, n1.y, n1.z,
                     n2.x, n2.y, n2.z,
                     0.0,  1.0,  0.0],
            _  =>
                vec![n1.x, n1.y, n1.z,
                     n2.x, n2.y, n2.z,
                     1.0,  0.0,  0.0],
        };

        m = rMatrix::new(3,3, mv);
        let u = m.solve(b).unwrap();
        let l_org = Point::new(u[0], u[1], u[2]);
        let l_dest = Point::new(u[0]+a.x, u[1]+a.y, u[2]+a.z);

        let l = Line {
            org: l_org,
            dest: l_dest
        };

        (Some(l), InfoPxP::Intersecting)
    }
}


#[cfg(test)]
mod tests {
    use cf::intersect::*;
    use cf::create;
    use bo::*;
    //use qm::*;
    //use std::option::Option;

    #[test]
    fn plane_x_plane_intersection() {
        let n = Vector::new(7.0, 0., 0.);
        let p0 = Point::new(0., 0., 0.);

        let plane1 = Plane { normal: n, point: p0};

        let n = Vector::new(0., 7.0, 0.);
        let p0 = Point::new(0., 0., 0.);

        let plane2 = Plane { normal: n, point: p0};

        let mut raf_simple_pxp : RafSimplePxP = create();
        let res = raf_simple_pxp.intersect(&plane1, &plane2);

        let expected_v = Vector::new(0.,0.,3.);

        if let (Some(l), InfoPxP::Intersecting) = res {
            let v = &l.dest - &l.org;
            if  expected_v.is_collinear_to(&v) {
                return;
            } else {
                panic!("Wrong result {}", l);
            }
        } else {
            panic!("Wrong info");
        }
    }

    #[test]
    fn collinear_planes() {
        let n = Vector::new(7.0, 0., 0.);
        let p0 = Point::new(1., 0., 0.);

        let plane1 = Plane { normal: n, point: p0};

        let n = Vector::new(7., 0.0, 0.);
        let p0 = Point::new(5., 0., 0.);

        let plane2 = Plane { normal: n, point: p0};

        let mut raf_simple_pxp : RafSimplePxP = create();
        let res = raf_simple_pxp.intersect(&plane1, &plane2);

        if let (None, InfoPxP::Collinear) = res {
            return;
        } else {
            panic!("Wrong info");
        }
    }

    #[test]
    fn coincidence_planes() {
        let n = Vector::new(7.0, 0., 0.);
        let p0 = Point::new(5., 0., 0.);

        let plane1 = Plane { normal: n, point: p0};

        let n = Vector::new(7., 0.0, 0.);
        let p0 = Point::new(5., 0., 0.);

        let plane2 = Plane { normal: n, point: p0};

        let mut raf_simple_pxp : RafSimplePxP = create();
        let res = raf_simple_pxp.intersect(&plane1, &plane2);

        if let (None, InfoPxP::Coincidence) = res {
            return;
        } else {
            panic!("Wrong info");
        }
    }
}