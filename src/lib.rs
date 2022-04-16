#![feature(once_cell)]

use std::cell::{Cell, RefCell, RefMut};
use std::lazy::OnceCell;
use std::ptr;

use ctru::gfx::BottomScreen;
use fontdue::{Font, FontSettings, Metrics};

mod devoptab {
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(clippy::all)]

    mod bindings;
    pub use bindings::*;
}

use devoptab::devoptab_t;

use crate::devoptab::_reent;

const DEFAULT_FONT: &[u8] = include_bytes!("../fonts/Ubuntu_Mono/UbuntuMono-Regular.ttf");

extern "C" {
    #[link_name = "devoptab_list"]
    static mut DEVOPTAB_LIST: [*const devoptab_t; devoptab::STD_MAX as usize];

    fn __getreent() -> *mut devoptab::_reent;
}

const _IONBF: libc::c_int = 2;
static mut STDOUT: OnceCell<devoptab_t> = OnceCell::new();

pub struct Console<'screen> {
    pub font: fontdue::Font,
    screen: RefMut<'screen, BottomScreen>,
    pixel_pos: RefCell<Position>,
    is_stdout_selected: Cell<bool>,
}

#[derive(Default)]
struct Position {
    x: usize,
    y: usize,
}

unsafe extern "C" fn write_r(
    r: *mut _reent,
    fd: *mut ::libc::c_void,
    ptr: *const ::libc::c_char,
    len: devoptab::size_t,
) -> devoptab::ssize_t {
    let console: &mut Console = &mut *((*r).deviceData).cast();
    console.write(fd, ptr, len as usize) as devoptab::ssize_t
}

impl<'screen> Console<'screen> {
    const SCALE: f32 = 6.0;

    pub fn init(screen: RefMut<'screen, BottomScreen>) -> Self {
        let font = Font::from_bytes(
            DEFAULT_FONT,
            FontSettings {
                scale: Self::SCALE,
                ..Default::default()
            },
        )
        .unwrap();

        Self {
            font,
            screen,
            pixel_pos: RefCell::default(),
            is_stdout_selected: Cell::new(false),
        }
    }

    pub fn select_stdout(&mut self) {
        self.is_stdout_selected.set(true);

        unsafe {
            STDOUT.get_or_init(|| devoptab_t {
                name: "console_stdout\0".as_ptr().cast(),
                // structSize: std::mem::size_of::<Self>() as u32,
                write_r: Some(write_r),
                ..Default::default()
            });

            // We have to set the deviceData here instead the init code above,
            // so that we can set different
            {
                // UNWRAP: safe because we just initialized STDOUT
                let stdout = STDOUT.get_mut().unwrap();

                // if !stdout.deviceData.is_null() {
                //     // "tell" the other console it's no longer in use
                //     (*(stdout.deviceData as *const Self))
                //         .is_stdout_selected
                //         .set(false);
                // }
                stdout.deviceData = (self as *mut Self).cast();
            }

            let stdout: &'static devoptab_t = STDOUT.get().unwrap();

            let devoptab_list = &mut DEVOPTAB_LIST;

            devoptab_list[devoptab::STD_OUT as usize] = stdout as *const devoptab_t;

            #[allow(clippy::used_underscore_binding)]
            let stdout_file = (*__getreent())._stdout;
            libc::setvbuf(stdout_file.cast(), ptr::null_mut(), _IONBF, 0);
        }
    }

    pub fn select_stderr(&self) {
        todo!()
    }

    fn write(
        &mut self,
        _fd: *mut libc::c_void,
        ptr: *const libc::c_char,
        len: usize,
    ) -> libc::ssize_t {
        let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };

        let frame_buffer = self.screen.get_raw_framebuffer();

        let mut count = 0;
        let mut pixel_pos = self.pixel_pos.borrow_mut();

        for c in bytes.iter().map(|&b| char::from(b)) {
            let (
                Metrics {
                    width,
                    height,
                    advance_width,
                    ..
                },
                pixels,
            ) = self.font.rasterize(c, Self::SCALE);

            for j in 0..height {
                for i in 0..width {
                    let draw_y = pixel_pos.y + height - j;
                    let draw_x = pixel_pos.x + i;
                    let offset: usize = (draw_y + draw_x * height) * 3;

                    let value = pixels[j * width + i];
                    let rgb = rgb565(value, value, value);

                    let b: u8 = (((rgb >> 11) & 0x1F) << 3) as u8;
                    let g: u8 = (((rgb >> 5) & 0x3F) << 2) as u8;
                    let r: u8 = ((rgb & 0x1F) << 3) as u8;

                    unsafe {
                        *frame_buffer.ptr.offset(offset as isize) = r;
                        *frame_buffer.ptr.offset(offset as isize + 1) = g;
                        *frame_buffer.ptr.offset(offset as isize + 2) = b;
                    }
                }
            }

            pixel_pos.x += advance_width.ceil() as usize;
            count += 1;
        }

        count
    }
}

impl<'gfx> Drop for Console<'gfx> {
    fn drop(&mut self) {
        if self.is_stdout_selected.get() {
            unsafe {
                // TODO: probably should just set devoptab_list entry to devnull
                STDOUT.get_mut().unwrap().deviceData = ptr::null_mut();
                STDOUT.get_mut().unwrap().write_r = None;
            }
        }
    }
}

fn rgb565(r: u8, g: u8, b: u8) -> u16 {
    (u16::from(b) & 0x1f) | ((u16::from(g) & 0x3f) << 5) | ((u16::from(r) & 0x1f) << 11)
}
