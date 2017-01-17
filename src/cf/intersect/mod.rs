pub mod segment_x_segment;
pub mod line_x_line;
pub mod line_x_plane;

/*
pub import Af (Abstract function), Raf (Realization of abstract function), Info
*/
pub use self::line_x_line::{AfLxL, InfoLxL, RafSimpleLxL};
pub use self::segment_x_segment::{AfSxS, InfoSxS, RafSimpleSxS};