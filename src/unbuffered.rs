#![feature(once_cell)]

use std::cell::{Cell, RefMut};
use std::lazy::OnceCell;
use std::ptr;

use ctru::gfx::BottomScreen;
use fontdue::layout::WrapStyle;
use fontdue::layout::{CoordinateSystem, GlyphPosition, Layout, LayoutSettings, TextStyle};
use fontdue::{Font, FontSettings};

mod ffi;

const DEFAULT_FONT: &[u8] = include_bytes!("../fonts/Ubuntu_Mono/UbuntuMono-Regular.ttf");

static mut STDOUT: OnceCell<ffi::devoptab_t> = OnceCell::new();

pub struct Console<'screen> {
    fonts: Vec<Font>,
    layout: Layout<()>,
    screen: RefMut<'screen, BottomScreen>,
    is_stdout_selected: Cell<bool>,
    pub use_subpixel_rendering: bool,
}

unsafe extern "C" fn write_r(
    r: *mut ffi::_reent,
    fd: *mut ::libc::c_void,
    ptr: *const ::libc::c_char,
    len: libc::size_t,
) -> libc::ssize_t {
    let device = (*r).deviceData;
    if device.is_null() {
        -1
    } else {
        let console: &mut Console = &mut *(device).cast();
        console.write(fd, ptr, len as usize) as libc::ssize_t
    }
}

impl<'screen> Console<'screen> {
    const SCALE: f32 = 11.0;

    pub fn init(mut screen: RefMut<'screen, BottomScreen>) -> Self {
        let font = Font::from_bytes(
            DEFAULT_FONT,
            FontSettings {
                scale: Self::SCALE,
                ..Default::default()
            },
        )
        .unwrap();

        // Framebuffer is in portrait mode, so swap X/Y and use a positive-y up
        // instead of down, which is the framebuffer's order.
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x: 0.0,
            y: 0.0,
            max_width: Some(f32::from(screen.get_raw_framebuffer().height)),
            max_height: Some(f32::from(screen.get_raw_framebuffer().width)),
            wrap_style: WrapStyle::Letter,
            ..Default::default()
        });

        Self {
            fonts: vec![font],
            layout,
            screen,
            is_stdout_selected: Cell::new(false),
            use_subpixel_rendering: false,
        }
    }

    pub fn select_stdout(&mut self) {
        self.is_stdout_selected.set(true);

        unsafe {
            STDOUT.get_or_init(|| ffi::devoptab_t {
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
                if !stdout.deviceData.is_null() {
                    // "tell" the other console it's no longer in use
                    (*(stdout.deviceData as *const Self))
                        .is_stdout_selected
                        .set(false);
                }
                stdout.deviceData = (self as *mut Self).cast();
            }

            let stdout: &'static ffi::devoptab_t = STDOUT.get().unwrap();

            let devoptab_list = ffi::devoptab_list.as_mut_ptr();
            *devoptab_list.add(ffi::STD_OUT as usize) = stdout as *const ffi::devoptab_t;

            #[allow(clippy::used_underscore_binding)]
            let stdout_file = (*ffi::__getreent())._stdout;
            libc::setvbuf(stdout_file.cast(), ptr::null_mut(), ffi::_IONBF as _, 0);
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

        let text = if let Ok(s) = std::str::from_utf8(bytes) {
            s
        } else {
            // just fail for now
            return 0;
        };

        let frame_buffer = self.screen.get_raw_framebuffer();

        self.layout
            .append(&self.fonts, &TextStyle::new(text, Self::SCALE, 0));

        for &glyph in self.layout.glyphs() {
            let GlyphPosition {
                parent,
                x: glyph_x,
                y: glyph_y,
                width,
                height,
                char_data,
                font_index,
                ..
            } = glyph;

            if !char_data.rasterize() {
                continue;
            }

            let (_, pixels) = if self.use_subpixel_rendering {
                self.fonts[font_index].rasterize_subpixel(parent, Self::SCALE)
            } else {
                self.fonts[font_index].rasterize(parent, Self::SCALE)
            };

            for j in 0..height {
                for i in 0..width {
                    // Swap x + y for portrait-mode framebuffer.
                    let framebuffer_y = glyph_x as usize + i;

                    // positive y in glyph == negative y in framebuffer
                    let framebuffer_x = frame_buffer.width as usize - (glyph_y as usize + j);

                    // RGB656 == 2 bytes per pixel
                    let offset = (framebuffer_x + framebuffer_y * frame_buffer.width as usize) * 2;

                    let px_offset = j * width + i;
                    let rgb_bytes = if self.use_subpixel_rendering {
                        let px_offset = px_offset * 3;
                        let r = pixels[px_offset];
                        let g = pixels[px_offset + 1];
                        let b = pixels[px_offset + 2];
                        rgb565(r, g, b).to_le_bytes()
                    } else {
                        let value = pixels[px_offset];
                        rgb565(value, value, value).to_le_bytes()
                    };

                    unsafe {
                        frame_buffer
                            .ptr
                            .add(offset)
                            .copy_from(rgb_bytes.as_ptr(), rgb_bytes.len());
                    }
                }
            }
        }

        len as _
    }
}

impl<'gfx> Drop for Console<'gfx> {
    fn drop(&mut self) {
        // // TODO: something here is segfaulting...

        // static mut DEVNULL: OnceCell<ffi::devoptab_t> = OnceCell::new();
        // if self.is_stdout_selected.get() {
        //     unsafe {
        //         if let Some(stdout) = STDOUT.get_mut() {
        //             stdout.deviceData = ptr::null_mut();
        //         }

        //         let devnull = DEVNULL.get_or_init(ffi::devoptab_t::default);
        //         let devoptab_list = ffi::devoptab_list.as_mut_ptr();
        //         *devoptab_list.add(ffi::STD_OUT as usize) = devnull as *const _;
        //     }
        // }
    }
}

fn rgb565(r: u8, g: u8, b: u8) -> u16 {
    let r = u16::from(r) * 0x1F / 0xFF;
    let g = u16::from(g) * 0x3F / 0xFF;
    let b = u16::from(b) * 0x1F / 0xFF;
    r << 11 | g << 5 | b
}
