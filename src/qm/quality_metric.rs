use core::default::Default;
use time::{PreciseTime,Duration};
use std::fmt::Display;
use std::fmt;

pub trait QualityMetric : Default + Display {
    fn start(&mut self);
    fn end(&mut self);
}


pub struct UselessQM;
impl QualityMetric for UselessQM {
    fn start(&mut self) {}
    fn end(&mut self) {}
}
impl Default for UselessQM {
    fn default() -> UselessQM { UselessQM }
}
impl fmt::Display for UselessQM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(UselessQM)")
    }
}


pub struct TimeQM {pub start: PreciseTime, pub duration: Duration}
impl QualityMetric for TimeQM {
    fn start(&mut self) {self.start = PreciseTime::now()}
    fn end(&mut self) {self.duration = self.start.to(PreciseTime::now())}
}
impl Default for TimeQM {
    fn default() -> TimeQM { TimeQM {start: PreciseTime::now(), duration: Duration::zero()}}
}
impl fmt::Display for TimeQM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(TimeQM {})", self.duration)
    }
}
