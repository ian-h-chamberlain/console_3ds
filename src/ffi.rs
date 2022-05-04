#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod generated;
use std::lazy::OnceCell;

pub use generated::*;

use crate::Console;

static mut STDOUT: OnceCell<devoptab_t> = OnceCell::new();
static mut STDERR: OnceCell<devoptab_t> = OnceCell::new();
static mut DEVNULL: OnceCell<devoptab_t> = OnceCell::new();

#[allow(clippy::used_underscore_binding)]
unsafe fn get_stdout() -> *mut __FILE {
    (*__getreent())._stdout
}

#[allow(clippy::used_underscore_binding)]
unsafe fn get_stderr() -> *mut __FILE {
    (*__getreent())._stderr
}

pub(crate) unsafe extern "C" fn write_r<'c, C: Console<'c>>(
    r: *mut _reent,
    _fd: *mut ::libc::c_void,
    ptr: *const ::libc::c_char,
    len: libc::size_t,
) -> libc::ssize_t {
    let device = (*r).deviceData;
    if device.is_null() {
        *__errno() = libc::EINVAL;
        return -1;
    }

    let console: &mut C = &mut *(device).cast();
    let bytes = std::slice::from_raw_parts(ptr, len);

    // Aust write invalid bytes as bad character �
    let text = String::from_utf8_lossy(bytes);
    if let Ok(len) = isize::try_from(text.len()) {
        console.write(&text);
        len
    } else {
        *__errno() = libc::EFBIG;
        -1
    }
}

pub(crate) unsafe fn set_stdout<'c, C: Console<'c>>(console: &mut C) {
    STDOUT.get_or_init(|| devoptab_t {
        name: "console_stdout\0".as_ptr().cast(),
        structSize: std::mem::size_of::<C>(),
        write_r: Some(write_r::<C>),
        ..Default::default()
    });

    // We have to set the deviceData here instead the init code above,
    // so that we can set different
    {
        // UNWRAP: safe because we just initialized STDOUT
        let stdout = STDOUT.get_mut().unwrap();
        //
        // TODO: "tell" the other console it's no longer in use, if there is one
        //
        stdout.deviceData = (console as *mut C).cast();
    }

    let stdout: &'static devoptab_t = STDOUT.get().unwrap();

    let dev_list = devoptab_list.as_mut_ptr();
    *dev_list.add(STD_OUT as usize) = stdout;

    // UNWRAP: const is always a valid i32
    let mode = _IOLBF.try_into().unwrap();
    libc::setvbuf(get_stdout().cast(), std::ptr::null_mut(), mode, 0);
}

pub(crate) unsafe fn set_stderr<'c, C: Console<'c>>(console: &mut C) {
    STDERR.get_or_init(|| devoptab_t {
        name: "console_stderr\0".as_ptr().cast(),
        structSize: std::mem::size_of::<C>(),
        write_r: Some(write_r::<C>),
        ..Default::default()
    });

    // We have to set the deviceData here instead the init code above,
    // so that we can set different
    {
        // UNWRAP: safe because we just initialized STDERR
        let stderr = STDERR.get_mut().unwrap();

        // TODO: "tell" the other console it's no longer in use, if there is one

        stderr.deviceData = (console as *mut C).cast();
    }

    let stderr: &'static devoptab_t = STDERR.get().unwrap();

    let dev_list = devoptab_list.as_mut_ptr();
    *dev_list.add(STD_ERR as usize) = stderr;

    // UNWRAP: const is always a valid i32
    let mode = _IOLBF.try_into().unwrap();
    libc::setvbuf(get_stderr().cast(), std::ptr::null_mut(), mode, 0);
}
