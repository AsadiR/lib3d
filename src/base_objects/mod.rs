pub use self::point::Point;
pub use self::vector::Vector;

pub mod point;
pub mod vector;
pub mod segment;
pub mod curve;
pub mod plane;
pub mod triangle;
pub mod mesh;

const EPS : f64 = 0.000001;

fn eq_f64(a : f64, b : f64) -> bool {
    return (a < b + EPS) & (a > b - EPS)
}



