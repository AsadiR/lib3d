use base_objects::point::Point;

pub struct Curve {
    points: Vec<Point>
}

impl Curve {
    fn number_of_points(&self) -> usize {
        self.points.len()
    }
}




#[cfg(test)]
mod tests {
    use base_objects::point::Point;
    use base_objects::curve::Curve;

    #[test]
    fn number_of_segments_check() {
        let p1 = Point { x: 1.0, y: 1.0, z: 0.0 };
        let p2 = Point { x: 2.0, y: 2.0, z: 1.0 };
        let p3 = Point { x: 3.0, y: 3.0, z: 2.0 };
        let points : Vec<Point> = vec![p1,p2,p3];
        let curve = Curve {points: points};
        assert!(curve.number_of_points() == 3);
    }
}