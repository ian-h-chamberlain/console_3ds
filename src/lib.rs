#![feature(once_cell)]
#![doc = include_str!("../README.md")]

mod buffered;
mod ffi;

pub use buffered::Console as BufferedConsole;

mod private {
    pub trait Sealed {}

    impl<'s> Sealed for super::BufferedConsole<'s> {}
}

/// A `Console` is an object that can be used to render text to the 3DS screen.
///
/// This is meant as an alternative to [`ctru::console::Console`], and it can
/// provide more control over how graphics are rendered to the screen.
pub trait Console<'screen>: std::io::Write + private::Sealed {
    /// Clear the console screen. It's up to the implementation if this also means
    /// clearing any internal buffers or other state.
    fn clear(&mut self);
}

/// A trait for marking a [`Console`] to be used by standard output streams.
/// By using this trait, you can call things like [`println`] and [`eprintln`]
/// to print text to the selected console.
pub trait ConsoleSelector {
    /// Select `console` as the destination to render writes to `self`.
    fn use_console<'a>(&self, console: &mut impl Console<'a>);
}

impl ConsoleSelector for std::io::Stdout {
    fn use_console<'a>(&self, console: &mut impl Console<'a>) {
        unsafe { ffi::set_stdout(console) }
    }
}

impl ConsoleSelector for std::io::Stderr {
    fn use_console<'a>(&self, console: &mut impl Console<'a>) {
        unsafe { ffi::set_stderr(console) }
    }
}
