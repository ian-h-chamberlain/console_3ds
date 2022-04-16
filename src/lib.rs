use ctru::gfx::Screen;
use fontdue::{Font, FontSettings};

const DEFAULT_FONT: &[u8] = include_bytes!("../fonts/Ubuntu_Mono/UbuntuMono-Regular.ttf");

pub struct Console<'screen> {
    pub font: fontdue::Font,
    _screen: &'screen mut dyn Screen,
}

impl<'screen> Console<'screen> {
    pub fn init(screen: &'screen mut dyn Screen) -> Self {
        let font = Font::from_bytes(
            DEFAULT_FONT,
            FontSettings {
                scale: 6.0,
                ..Default::default()
            },
        )
        .unwrap();

        Self {
            font,
            _screen: screen,
        }
    }
}
