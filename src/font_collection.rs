use windows::Win32::Graphics::DirectWrite::{IDWriteFontCollection, IDWriteFontFamily};

use crate::font_family::FontFamily;

pub fn get_font_family_count(collection: &IDWriteFontCollection) -> u32 {
    unsafe { collection.GetFontFamilyCount() }
}

pub fn get_font_family(
    collection: &IDWriteFontCollection,
    index: u32,
) -> anyhow::Result<IDWriteFontFamily> {
    Ok(unsafe { collection.GetFontFamily(index) }?)
}

pub struct FontCollection(pub IDWriteFontCollection);
impl FontCollection {
    pub fn get_font_family_count(&self) -> u32 {
        get_font_family_count(&self.0)
    }

    pub fn get_font_family(&self, index: u32) -> anyhow::Result<FontFamily> {
        get_font_family(&self.0, index).map(FontFamily)
    }
}
