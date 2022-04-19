use std::time::Instant;

use console_3ds::UnbufferedConsole;
use ctru::gfx::{Gfx, Screen};
use ctru::services::apt::Apt;
use ctru::services::gspgpu::FramebufferFormat;
use ctru::services::hid::{Hid, KeyPad};

fn main() {
    ctru::init();

    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    bottom_screen.set_double_buffering(false);
    bottom_screen.set_framebuffer_format(FramebufferFormat::Rgb565);

    // TODO allow swapping buffers for separate screens independently.
    // Need to support in ctru-rs
    gfx.swap_buffers();

    let start = Instant::now();

    // Start a console on the top screen
    let mut top_console = UnbufferedConsole::init(bottom_screen);
    top_console.select_stdout();

    let duration = Instant::now() - start;
    println!("Loaded font in {:.2?}", duration);

    while apt.main_loop() {
        hid.scan_input();
        let keys = hid.keys_down();
        if keys.contains(KeyPad::KEY_START) {
            break;
        }

        if keys.contains(KeyPad::KEY_A) {
            println!("Hello world!");
        } else if keys.contains(KeyPad::KEY_B) {
            println!(
                "This is a really long line of text to test the wrapping behavior of the library"
            );
        } else if keys.contains(KeyPad::KEY_X) {
            top_console.use_subpixel_rendering = !top_console.use_subpixel_rendering;
            println!("Toggled subpixel rendering");
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}
