#![doc = include_str!("../README.md")]

mod rss;

mod ns {
    pub mod itunes;
    pub mod podcast;
}

pub use ns::itunes;
pub use ns::podcast;
pub use rss::*;
