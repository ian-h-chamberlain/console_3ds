# Notes

## Fonts

Favorites so far:

* Hack (license is MIT but with a weird clause for Bitstream Vera). Need to figure out what's up with that
* Inconsolata (OFL)
* Ubuntu Mono seems... okay. Also a custom Ubuntu license

To look into:
* Droid Sans Mono (Apache)

- [ ] Check bold + italic
- [ ] Need to figure out if underlines can work?
- [ ] Unicode char support?

## Implementation

Consider something like `trait ConsoleWriter<T: Console>` and implementing it
for `io::Stdout` and `io::Stderr` types? Then usage would look like

```rs
use console_3ds::ConsoleWriter;

fn main() {
    // pseudocode...
    let top_console = console_3ds::from(top_screen);
    std::io::stdout().use_console(console);

    let bottom_console = console_3ds::from(bottom_screen);
    std::io::stderr().use_console(console);
}
