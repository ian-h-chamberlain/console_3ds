use std::io::Write;
use std::time::Instant;

use console_3ds::{Console, UnbufferedConsole};
use ctru::gfx::{Gfx, Screen, Side};
use ctru::services::apt::Apt;
use ctru::services::gspgpu::FramebufferFormat;
use ctru::services::hid::{Hid, KeyPad};

fn main() {
    ctru::init();

    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();

    let start = Instant::now();

    let mut top_screen = gfx.top_screen.borrow_mut();
    top_screen.set_double_buffering(false);
    top_screen.set_framebuffer_format(FramebufferFormat::Rgb565);

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    bottom_screen.set_double_buffering(false);
    bottom_screen.set_framebuffer_format(FramebufferFormat::Rgb565);

    // Commit the screen configuration by swapping buffers once.
    gfx.swap_buffers();
    // TODO allow swapping buffers for separate screens independently. Needs support in ctru-rs

    // Start a console on the top screen (left side for non-3d)
    let mut top_console = UnbufferedConsole::init(top_screen.get_raw_framebuffer(Side::Left));
    top_console.select_stdout();

    let duration = Instant::now() - start;
    println!("Loaded top screen in {:.2?}", duration);

    let start = Instant::now();

    // Use the bottom screen for stderr
    let mut bottom_console = UnbufferedConsole::init(bottom_screen.get_raw_framebuffer());
    bottom_console.select_stderr();

    let duration = Instant::now() - start;
    eprintln!("Loaded bottom screen in {:.2?}", duration);

    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();

    while apt.main_loop() {
        hid.scan_input();
        let keys_pressed = hid.keys_down();
        if keys_pressed.contains(KeyPad::KEY_START) {
            break;
        }

        let keys_held = hid.keys_held();

        let (stream, console): (&mut dyn Write, &mut UnbufferedConsole) =
            if keys_held.contains(KeyPad::KEY_L) {
                (&mut stderr, &mut bottom_console)
            } else {
                (&mut stdout, &mut top_console)
            };

        if keys_pressed.contains(KeyPad::KEY_A) {
            writeln!(stream, "Hello world!").unwrap();
        } else if keys_pressed.contains(KeyPad::KEY_B) {
            writeln!(
                stream,
                "This is a really long line of text to test the wrapping behavior of the library"
            )
            .unwrap();
        } else if keys_pressed.contains(KeyPad::KEY_X) {
            console.use_subpixel_rendering = !console.use_subpixel_rendering;
            writeln!(stream, "Toggled subpixel rendering").unwrap();
        } else if keys_pressed.contains(KeyPad::KEY_Y) {
            console.clear();
        }

        gfx.swap_buffers();
        gfx.wait_for_vblank();
    }
}
