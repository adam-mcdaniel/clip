// A commandline parsing tool without alloc or std crates!
#![no_std]


mod arg;
pub use arg::{NUM_VALUES, Arg, Values, args};

mod app;
pub use app::{NUM_ARGS, App};