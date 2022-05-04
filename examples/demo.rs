use std::io::Write;
use std::time::Instant;

use console_3ds::{BufferedConsole, Console};
use ctru::gfx::{Gfx, Screen, Side};
use ctru::services::apt::Apt;
use ctru::services::gspgpu::FramebufferFormat;
use ctru::services::hid::{Hid, KeyPad};

/// Workaround for <https://github.com/Meziu/ctru-rs/pull/56>
#[no_mangle]
static __stacksize__: usize = 2 * 1024 * 1024;

fn main() {
    ctru::init();

    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::init().unwrap();

    let start = Instant::now();

    let mut top_screen = gfx.top_screen.borrow_mut();
    top_screen.set_double_buffering(false);
    // TODO:it would be nice if we could select a wider font for wide mode
    top_screen.set_framebuffer_format(FramebufferFormat::Rgb565);

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();
    bottom_screen.set_double_buffering(false);
    bottom_screen.set_framebuffer_format(FramebufferFormat::Rgb565);

    // Commit the screen configuration by swapping buffers once.
    gfx.swap_buffers();
    // TODO allow swapping buffers for separate screens independently. Needs support in ctru-rs

    // Start a console on the top screen (left side for non-3d)
    let mut top_console = BufferedConsole::init(top_screen.get_raw_framebuffer(Side::Left), 11);
    top_console.select_stdout();

    println!(
        r#"Loaded top screen in {:.2?}. Controls:
    A: print "Hello, world!".
    B: print a longer string of random text.
    X: toggle subpixel rendering.
    Y: Clear the screen."#,
        start.elapsed(),
    );

    let start = Instant::now();

    // Use the bottom screen for stderr
    let mut bottom_console = BufferedConsole::init(bottom_screen.get_raw_framebuffer(), 12);
    bottom_console.select_stderr();

    eprintln!(
        "Loaded bottom screen in {:.2?}. Hold L or R to use this screen instead of the top screen.",
        start.elapsed()
    );

    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();

    while apt.main_loop() {
        hid.scan_input();
        let keys_pressed = hid.keys_down();
        if keys_pressed.contains(KeyPad::KEY_START) {
            break;
        }

        let keys_held = hid.keys_held();

        let (stream, console): (&mut dyn Write, &mut BufferedConsole) =
            if keys_held.contains(KeyPad::KEY_L) || keys_held.contains(KeyPad::KEY_L) {
                (&mut stderr, &mut bottom_console)
            } else {
                (&mut stdout, &mut top_console)
            };

        if keys_pressed.contains(KeyPad::KEY_A) {
            writeln!(stream, "Hello world!").unwrap();
        } else if keys_pressed.contains(KeyPad::KEY_B) {
            writeln!(stream, "{}", lipsum::lipsum_words(35)).unwrap();
        } else if keys_pressed.contains(KeyPad::KEY_X) {
            console.use_subpixel_rendering = !console.use_subpixel_rendering;
            writeln!(stream, "Toggled subpixel rendering").unwrap();
        } else if keys_pressed.contains(KeyPad::KEY_Y) {
            console.clear();
        }

        gfx.wait_for_vblank();
        gfx.swap_buffers();
    }
}
