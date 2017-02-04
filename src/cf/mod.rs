pub mod intersect;
pub mod triangulate;



pub trait Af : Default {


}

pub fn create<T : Af>() -> T {
    Default::default()
}