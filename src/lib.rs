// we use a lot of manual indices instead of `take` and `skip`, so allow that
#![allow(clippy::needless_range_loop)]
// make sure any use of unsafe is documented
#![deny(clippy::undocumented_unsafe_blocks)]

pub mod blocks;
pub mod cliffs;
pub mod config;
pub mod contours;
pub mod crop;
pub mod io;
pub mod knolls;
pub mod merge;
pub mod process;
pub mod render;
pub mod util;
pub mod vec2d;
pub mod vegetation;

#[cfg(feature = "shapefile")]
pub mod shapefile;
