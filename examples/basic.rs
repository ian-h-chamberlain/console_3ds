use std::cell::RefCell;
use std::io::Write;

use console_3ds::Console;
use ctru::gfx::{Gfx, Screen};
use ctru::services::apt::Apt;
use ctru::services::gspgpu::FramebufferFormat;
use ctru::services::hid::{Hid, KeyPad};

fn main() {
    ctru::init();

    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let mut gfx = Gfx::init().unwrap();

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    bottom_screen.set_double_buffering(false);
    bottom_screen.set_framebuffer_format(FramebufferFormat::Rgb565);

    // TODO allow swapping buffers for separate screens independently.
    // Need to support in ctru-rs
    gfx.swap_buffers();

    // Start a console on the top screen
    let mut top_console = Console::init(bottom_screen);
    top_console.select_stdout();

    while apt.main_loop() {
        hid.scan_input();
        let keys = hid.keys_down();
        if keys.contains(KeyPad::KEY_START) {
            break;
        } else if keys.contains(KeyPad::KEY_A) {
            print!("H");
            std::io::stdout().flush().unwrap();
        }

        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}
