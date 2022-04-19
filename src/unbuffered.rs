use ctru::gfx::RawFrameBuffer;
use fontdue::layout::{
    CoordinateSystem, GlyphPosition, Layout, LayoutSettings, TextStyle, WrapStyle,
};
use fontdue::{Font, FontSettings};

use crate::ffi;

const DEFAULT_FONT: &[u8] = include_bytes!("../fonts/Ubuntu_Mono/UbuntuMono-Regular.ttf");

/// A console that merely renders to the screen and discards the input string.
pub struct Console<'screen> {
    fonts: Vec<Font>,
    layout: Layout<()>,
    frame_buffer: RawFrameBuffer<'screen>,
    pub use_subpixel_rendering: bool,
}

impl<'screen> Console<'screen> {
    // TODO: configurable size? probably would be nice
    const SCALE: f32 = 11.0;

    /// Initialize the console from a frame buffer.
    #[must_use]
    pub fn init(frame_buffer: RawFrameBuffer<'screen>) -> Self {
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
            max_width: Some(f32::from(frame_buffer.height)),
            max_height: Some(f32::from(frame_buffer.width)),
            wrap_style: WrapStyle::Letter,
            ..Default::default()
        });

        Self {
            fonts: vec![font],
            layout,
            frame_buffer,
            use_subpixel_rendering: false,
        }
    }
}

impl<'screen> crate::Console<'screen> for Console<'screen> {
    fn select_stdout(&mut self) {
        unsafe {
            ffi::set_stdout(self);
        }
    }

    fn select_stderr(&mut self) {
        unsafe {
            ffi::set_stderr(self);
        }
    }

    fn write(&mut self, text: &str) -> libc::ssize_t {
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
                    let framebuffer_x = self.frame_buffer.width as usize - (glyph_y as usize + j);

                    // RGB656 == 2 bytes per pixel
                    let offset =
                        (framebuffer_x + framebuffer_y * self.frame_buffer.width as usize) * 2;

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
                        self.frame_buffer
                            .ptr
                            .add(offset)
                            .copy_from(rgb_bytes.as_ptr(), rgb_bytes.len());
                    }
                }
            }
        }

        text.len().try_into().unwrap()
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
