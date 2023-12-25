use windows::Win32::Graphics::DirectWrite::{
    IDWriteFont, IDWriteFontFamily, IDWriteLocalizedStrings,
};

use crate::{font::Font, localized_strings::LocalizedStrings};

pub fn get_family_names(family: &IDWriteFontFamily) -> anyhow::Result<IDWriteLocalizedStrings> {
    unsafe { Ok(family.GetFamilyNames()?) }
}

pub fn get_font_count(family: &IDWriteFontFamily) -> u32 {
    unsafe { family.GetFontCount() }
}

pub fn get_font(family: &IDWriteFontFamily, index: u32) -> anyhow::Result<IDWriteFont> {
    unsafe { Ok(family.GetFont(index)?) }
}

pub struct FontFamily(pub IDWriteFontFamily);
impl FontFamily {
    pub fn get_family_names(&self) -> anyhow::Result<LocalizedStrings> {
        get_family_names(&self.0).map(LocalizedStrings)
    }

    pub fn get_font_count(&self) -> u32 {
        get_font_count(&self.0)
    }

    pub fn get_font(&self, index: u32) -> anyhow::Result<Font> {
        get_font(&self.0, index).map(Font)
    }
}
