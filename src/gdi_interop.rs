use windows::Win32::{
    Foundation::BOOL,
    Graphics::{
        DirectWrite::{IDWriteFont, IDWriteGdiInterop},
        Gdi::LOGFONTW,
    },
};

use crate::font::Font;

pub fn convert_font_to_logfont(
    interop: &IDWriteGdiInterop,
    font: &IDWriteFont,
) -> anyhow::Result<(LOGFONTW, bool)> {
    let mut logfont = LOGFONTW::default();
    let mut is_system_font = BOOL::from(false);
    unsafe {
        interop.ConvertFontToLOGFONT(font, &mut logfont, &mut is_system_font)?;
    }
    Ok((logfont, is_system_font.as_bool()))
}

pub struct GdiInterop(pub IDWriteGdiInterop);

impl GdiInterop {
    pub fn convert_font_to_logfont(&self, font: &Font) -> anyhow::Result<(LOGFONTW, bool)> {
        convert_font_to_logfont(&self.0, &font.0)
    }
}
