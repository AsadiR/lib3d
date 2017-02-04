use bo::point::Point;
use bo::vector::Vector;
use std::fmt;
use std::cmp::Ordering;
use bo::EPS;
use std::f32;
use std::mem::swap;


/*
TODO:
rotate
scale
translate
*/

#[derive(PartialEq,Eq)]
#[derive(Clone)]
pub struct Segment {
    pub org : Point,
    pub dest: Point
}

impl Segment {
    pub fn rot(&mut self, normal : &Vector, d : &f32) {
        let m = Point {
            x: (self.dest.x + self.org.x)/2.,
            y: (self.dest.y + self.org.y)/2.,
            z: (self.dest.z + self.org.z)/2.
        };
        /*
        let mut temp = self.dest;

        self.org.rotate_around_axis(&m, &normal, 90.);
        self.dest = self.org;
        temp.rotate_around_axis(&m, &normal, 90.);
        self.org = temp;
        */
        self.org.rotate_around_axis(&m, &normal, 90.);
        self.dest.rotate_around_axis(&m, &normal, 90.);
        //swap(&mut self.org, &mut self.dest);


        //check rotate
        assert!(f32::abs(self.org.get_vector().dot_product(&normal) - d) < EPS);
        assert!(f32::abs(self.dest.get_vector().dot_product(&normal) - d) < EPS);
        assert!(&(&self.dest.get_vector() + &self.org.get_vector()) * 0.5 == m.get_vector());
    }

    pub fn flip(&mut self) {
        swap(&mut self.org, &mut self.dest);
    }

    pub fn intersect(&self, e : &Segment, t : &mut f32, normal_ : &Vector, d_ : &f32) -> SegmentsInfo {
        let a : &Point = &self.org;
        let b : &Point = &self.dest;
        let c : &Point = &e.org;
        //let d : &Point = &e.dest;
        let mut f : Segment = e.clone();
        f.rot(normal_, &d_);
        let n : Vector = &f.dest - &f.org;
        let denom = n.dot_product(&(b-a));
        if f32::abs(denom) <= EPS {
            *t = f32::MAX;
            return SegmentsInfo::Parallel
        }
        let num = n.dot_product(&(a-c));
        *t = -num/denom;
        return SegmentsInfo::Skew;

    }
}

pub enum SegmentsInfo {Parallel, Skew}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Segment: {}, {})", self.org, self.dest)
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Segment) -> Ordering {
        match self {
            _ if *self == *other => Ordering::Equal,
            _ if self.org < other.org => Ordering::Less,
            _ if self.org > other.org => Ordering::Greater,
            _ if self.dest < other.dest => Ordering::Less,
            _ if self.dest > other.dest => Ordering::Greater,
            _ => panic!("Smth goes wrong!")
        }
    }
}


impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Segment) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[cfg(test)]
mod tests {
    use bo::*;
    //use qm::*;


    #[test]
    fn rotation() {
        // x+y+z = 1
        let p1 = Point::new(0., 0., 1.);
        let p2 = Point::new(1., 0., 0.);
        let p3 = Point::new(0., 1., 0.);

        let mut e = Segment { org: p1.clone(), dest: p2.clone() };

        let v1 = &p1 - &p2;
        let v2 = &p3 - &p2;
        let mut normal: Vector = v1.cross_product(&v2);
        normal.normalize();

        let d = normal.dot_product(&p1.get_vector());
        e.rot(&normal, &d);
        e.rot(&normal, &d);

        assert!(p2 == e.org);
        assert!(p1 == e.dest);
    }
}



