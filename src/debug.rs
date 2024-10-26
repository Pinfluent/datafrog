#[cfg(feature = "std")]
pub use std::io::Write;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

#[cfg(not(feature = "std"))]
pub trait Write {
    fn write_stats(&mut self, name: &str, round: u32, stable_count: usize, recent_count: usize);
}

#[cfg(not(feature = "std"))]
pub type BoxedWriter = Box<dyn Write>;

#[cfg(feature = "std")]
pub type BoxedWriter = Box<dyn std::io::Write>;
