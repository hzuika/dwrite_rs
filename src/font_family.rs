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

pub struct FontFamilyIter<'a> {
    family: &'a FontFamily,
    index: usize,
}

impl<'a> Iterator for FontFamilyIter<'a> {
    type Item = Font;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len() {
            let font = self.family.get_font(self.index as u32);
            self.index += 1;
            match font {
                Ok(font) => Some(font),
                Err(e) => {
                    eprintln!("{}", e);
                    assert!(false);
                    None
                }
            }
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for FontFamilyIter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.family.get_font_count() as usize
    }
}

impl<'a> IntoIterator for &'a FontFamily {
    type Item = Font;
    type IntoIter = FontFamilyIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            family: self,
            index: 0,
        }
    }
}
