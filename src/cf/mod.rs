pub mod intersect;



pub trait Af : Default {


}

pub fn create<T : Af>() -> T {
    Default::default()
}