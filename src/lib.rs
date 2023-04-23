#![deny(missing_docs)]

//! Commonly used utils for aoc puzzles.
//! Check out each module doc for more information.

pub mod main_template;

pub mod fail;
pub mod output;

// re-export of common dependencies
pub use itertools;
pub use itertools::Itertools;
pub use nom;
