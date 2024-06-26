#![doc = include_str!("../README.md")]

mod language;
mod rss;

mod ns {
    pub mod itunes;
    pub mod podcast;
}

pub use language::*;
pub use ns::itunes;
pub use ns::podcast;
pub use rss::*;

pub mod ser {
    pub use yaserde::ser::*;
}
