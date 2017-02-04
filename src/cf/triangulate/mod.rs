use bo::*;
use cf::Af;

pub mod incremental_alg;

pub use self::incremental_alg::raf_incremental_triangulation::RafIncrementalTriangulation;


pub trait AfTriangulate : Af  {
    fn triangulate(&mut self, points : Vec<Point>) -> Vec<Triangle>;
}
