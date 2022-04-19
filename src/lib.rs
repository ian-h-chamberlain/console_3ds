#![feature(once_cell)]

mod ffi;
mod unbuffered;

pub use unbuffered::Console as UnbufferedConsole;

/// A `Console` is an object that can be used to render the standard output and
/// error streams on the 3DS, allowing the conventional use of e.g.
/// [`println!`]
///
/// This is meant as an alternative to [`ctru::console::Console`], which can
/// provide more control over how graphics are rendered to the screen.
pub trait Console<'screen> {
    /// Select this as the console to render when standard output is written to.
    fn select_stdout(&mut self);

    /// Select this as the console to render when standard error is written to.
    fn select_stderr(&mut self);

    /// Write directly to the console. In most cases it should be preferable
    /// to call [`select_stdout`] or [`select_stderr`] and use the standard
    /// [`print!`]/[`eprint!`] or similar.
    ///
    /// [`select_stdout`]: Self::select_stdout
    /// [`select_stderr`]: Self::select_stderr
    fn write(&mut self, text: &str) -> isize;
}
