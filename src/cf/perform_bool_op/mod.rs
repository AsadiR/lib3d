pub mod raf_simple_bool_op;

pub use self::raf_simple_bool_op::RafSimpleBoolOp;

use bo::*;
use cf::Af;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum InfoBoolOp {
    Intersecting,
    NotIntersecting,
}

pub trait AfBoolOp : Af  {
    fn split(&mut self, ma : &Mesh, mb : &Mesh) -> (Vec<Triangle>, Vec<Triangle>, Vec<Triangle>, Vec<Triangle>);
    fn union(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp);
    fn intersect(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp);
    fn difference(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp);
}