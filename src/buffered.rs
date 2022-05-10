use std::io;

use ctru::gfx::RawFrameBuffer;
use fontdue::layout::{
    CoordinateSystem, GlyphPosition, Layout, LayoutSettings, TextStyle, WrapStyle,
};
use fontdue::{Font, FontSettings};

const DEFAULT_FONT: &[u8] = include_bytes!("../fonts/Ubuntu_Mono/UbuntuMono-Regular.ttf");

/// A console that merely renders to the screen and discards the input string.
pub struct Console<'screen> {
    fonts: Vec<Font>,
    layout: Layout<()>,
    negative_y_offset: f32,
    scale: f32,
    line_height: u16,
    frame_buffer: RawFrameBuffer<'screen>,
    // TODO: setter/getter ?
    pub use_subpixel_rendering: bool,
}

impl<'screen> Console<'screen> {
    /// Initialize the console from a frame buffer, with a given scale (in pixels).
    #[must_use]
    pub fn init(frame_buffer: RawFrameBuffer<'screen>, scale: u16) -> Self {
        // alternative: just find the nearest factor of height, and use that
        // panicking when the user is trying to create a console, is... not very
        // ergonomic or easy to debug, lol
        // assert!(
        //     frame_buffer.width % scale == 0,
        //     "invalid scale {scale:?} (must be a factor of {:?})",
        //     frame_buffer.height
        // );

        let scale = f32::from(scale);

        let font = Font::from_bytes(
            DEFAULT_FONT,
            FontSettings {
                scale,
                ..Default::default()
            },
        )
        .unwrap();

        // The framebuffer is laid out in portrait mode, so swap X and Y...
        let max_width = f32::from(frame_buffer.height);

        // ...and use a positive-y up instead of down to get the right orientation.
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x: 0.0,
            y: 0.0,
            max_height: None,
            max_width: Some(max_width),
            wrap_style: WrapStyle::Letter,
            ..Default::default()
        });

        let metric_line_height = font
            .horizontal_line_metrics(scale)
            .unwrap()
            .new_line_size
            .ceil();

        let line_height = metric_line_height.max(scale) as u16;

        Self {
            fonts: vec![font],
            layout,
            scale,
            negative_y_offset: 0.0,
            line_height,
            frame_buffer,
            use_subpixel_rendering: false,
        }
    }

    fn draw_pixel(
        frame_buffer: &mut RawFrameBuffer<'screen>,
        pixel_x: usize,
        pixel_y: usize,
        color: (u8, u8, u8),
    ) {
        let mut frame_buffer = Rgb565FrameBuffer(frame_buffer);

        let (r, g, b) = color;
        let r = u16::from(r) * 0x1F / 0xFF;
        let g = u16::from(g) * 0x3F / 0xFF;
        let b = u16::from(b) * 0x1F / 0xFF;

        let rgb565 = r << 11 | g << 5 | b;

        let px_offset = pixel_x + pixel_y * frame_buffer.width();
        unsafe {
            frame_buffer.ptr().add(px_offset).write(rgb565.to_le());
        }
    }

    fn scroll_framebuffer_up(frame_buffer: &mut RawFrameBuffer<'screen>, amount: u16) {
        let mut frame_buffer = Rgb565FrameBuffer(frame_buffer);
        let copy_count = frame_buffer.width() - usize::from(amount);

        for y in 0..frame_buffer.height() {
            let src_x = 0;
            let src_offset = src_x + y * frame_buffer.width();

            let dst_x = usize::from(amount);
            let dst_offset = dst_x + y * frame_buffer.width();

            unsafe {
                let src = frame_buffer.ptr().add(src_offset);
                let dst = frame_buffer.ptr().add(dst_offset);
                std::ptr::copy(src, dst, copy_count);
            }

            // clear the last row of "fresh" pixels
            unsafe {
                let src = frame_buffer.ptr().add(src_offset);
                src.write_bytes(0, usize::from(amount));
            }
        }
    }
}

/// Helper struct for copying frame buffer data by pixel ([`u16`]) instead of
/// by byte ([`u8`])
struct Rgb565FrameBuffer<'a, 'screen>(&'a mut RawFrameBuffer<'screen>);

impl<'a, 'screen> Rgb565FrameBuffer<'a, 'screen> {
    /// Get a pointer to the pixel frame buffer data.
    fn ptr(&mut self) -> *mut u16 {
        self.0.ptr.cast()
    }

    /// Get the width of the frame buffer, in pixels.
    fn width(&self) -> usize {
        usize::from(self.0.width)
    }

    /// Get the height of the frame buffer, in pixels.
    fn height(&self) -> usize {
        usize::from(self.0.height)
    }
}

impl<'screen> crate::Console<'screen> for Console<'screen> {
    fn clear(&mut self) {
        self.layout.clear();
        self.negative_y_offset = 0.0;

        for y in 0..self.frame_buffer.height {
            for x in 0..self.frame_buffer.width {
                Self::draw_pixel(&mut self.frame_buffer, x.into(), y.into(), (0, 0, 0));
            }
        }
    }
}

impl<'screen> io::Write for Console<'screen> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let text =
            std::str::from_utf8(buf).map_err(|_| io::Error::from_raw_os_error(libc::EINVAL))?;

        self.layout
            .append(&self.fonts, &TextStyle::new(text, self.scale, 0));

        // This almost works, it seems. Just need to nail down the x-offset stuff

        let rendered_height = self.layout.height() - self.negative_y_offset;
        // int divide the frame buffer width by the line height
        let max_height = self.line_height * (self.frame_buffer.width / self.line_height);

        let height_diff = rendered_height - f32::from(max_height);
        if height_diff > 0.0 {
            // we may need to scroll by more than one line
            let scroll_amount = self.line_height * height_diff as u16 / self.line_height;

            self.negative_y_offset += f32::from(scroll_amount);
            Self::scroll_framebuffer_up(&mut self.frame_buffer, scroll_amount);

            // TODO: it would be nice if we could "prune" the scroll buffer for stuff
            // that's not rendered anymore, but might require saving all the text we've
            // already rendered...
        }

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

            let glyph_y = glyph_y - self.negative_y_offset;

            let (_, pixels) = if self.use_subpixel_rendering {
                self.fonts[font_index].rasterize_subpixel(parent, self.scale)
            } else {
                self.fonts[font_index].rasterize(parent, self.scale)
            };

            for j in 0..height {
                for i in 0..width {
                    let pixel_y = glyph_y + j as f32;
                    if pixel_y as u16 >= self.frame_buffer.width || pixel_y < 0.0 {
                        continue;
                    }

                    // Swap x + y for portrait-mode framebuffer.
                    let framebuffer_y = glyph_x as usize + i;

                    // Positive y in glyph == negative y in framebuffer.
                    // Subtract 1, since the offset at `width` is actually the next line
                    // and `pixel_y` is guaranteed less than `width`.
                    let framebuffer_x = self.frame_buffer.width as usize - 1 - pixel_y as usize;

                    let px_offset = j * width + i;
                    let color = if self.use_subpixel_rendering {
                        let px_offset = px_offset * 3;
                        let r = pixels[px_offset];
                        let g = pixels[px_offset + 1];
                        let b = pixels[px_offset + 2];
                        (r, g, b)
                    } else {
                        let value = pixels[px_offset];
                        (value, value, value)
                    };

                    Self::draw_pixel(&mut self.frame_buffer, framebuffer_x, framebuffer_y, color);
                }
            }
        }

        Ok(text.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
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
