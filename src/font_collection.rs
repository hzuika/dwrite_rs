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

pub struct FontCollectionIter<'a> {
    collection: &'a FontCollection,
    index: usize,
}

impl<'a> Iterator for FontCollectionIter<'a> {
    type Item = FontFamily;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len() {
            let family = self.collection.get_font_family(self.index as u32);
            self.index += 1;
            match family {
                Ok(family) => Some(family),
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

impl<'a> ExactSizeIterator for FontCollectionIter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.collection.get_font_family_count() as usize
    }
}

impl<'a> IntoIterator for &'a FontCollection {
    type Item = FontFamily;
    type IntoIter = FontCollectionIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            collection: self,
            index: 0,
        }
    }
}
