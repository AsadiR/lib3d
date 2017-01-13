use base_objects::triangle::Triangle;

pub struct Mesh {
    triangles: Vec<Triangle>
}

impl Mesh {
    fn number_of_polygons(&self) -> usize {
        self.triangles.len()
    }
}




#[cfg(test)]
mod tests {
    use base_objects::point::Point;
    use base_objects::triangle::Triangle;
    use base_objects::mesh::Mesh;

    #[test]
    fn number_of_segments_check() {
        let p1 = Point { x: 1.0, y: 1.0, z: 0.0 };
        let p2 = Point { x: 2.0, y: 2.0, z: 1.0 };
        let p3 = Point { x: 3.0, y: 3.0, z: 2.0 };
        let t1 = Triangle {a : p1, b : p2, c : p3};
        let triangles : Vec<Triangle> = vec![t1];
        let mesh = Mesh {triangles: triangles};
        assert!(mesh.number_of_polygons() == 1);
    }
}