pub mod intersect;
pub mod triangulate;
pub mod enumerate;
pub mod perform_bool_op;

//TODO: interface for custom configuration

pub trait Af : Default {


}

pub fn create<T : Af>() -> T {
    Default::default()
}

/*
CF stands for Composite Functor.
Functor is a function with state.
Subdirectories of cf are called using verbs.
This subdirectories contain batches of RAFs and AF.
AF stands for Abstract Function. AFs describe type of action.
RAF stands for Realization of Abstract Function.
*/