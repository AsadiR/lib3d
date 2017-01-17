
mod point;
mod vector;
mod segment;
mod line;
mod curve;
mod plane;
mod mesh;
mod base_object;

pub use self::base_object::BaseObject;
pub use self::point::Point;
pub use self::vector::Vector;
pub use self::line::Line;
pub use self::curve::Curve;
pub use self::mesh::Mesh;
pub use self::segment::Segment;
pub use self::plane::Plane;


const EPS : f32 = 0.000001;

pub fn eq_f32(a : f32, b : f32) -> bool {
    return (a < b + EPS) & (a > b - EPS)
}



