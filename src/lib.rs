#![feature(once_cell)]

mod ffi;
mod unbuffered;

pub use unbuffered::Console as UnbufferedConsole;
