#![crate_name = "rust_plot"]
//#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A small library for real time plotting of data in piston

extern crate graphics;

pub use plot::Plot;

mod plot;
