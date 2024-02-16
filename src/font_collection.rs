use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::BOOL,
        Graphics::DirectWrite::{IDWriteFontCollection, IDWriteFontFamily},
    },
};

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

pub fn find_family_name(
    collection: &IDWriteFontCollection,
    family_name: &str,
) -> anyhow::Result<(u32, bool)> {
    unsafe {
        let mut index = 0_u32;
        let mut exists = BOOL::from(false);
        let mut family_name: Vec<u16> = family_name.encode_utf16().collect();
        family_name.push(0);
        let family_name = PCWSTR::from_raw(family_name.as_ptr());
        collection.FindFamilyName(family_name, &mut index, &mut exists)?;
        Ok((index, exists.as_bool()))
    }
}

pub fn find_family(
    collection: &IDWriteFontCollection,
    family_name: &str,
) -> Option<IDWriteFontFamily> {
    let (index, exists) = find_family_name(collection, family_name).ok()?;
    if exists {
        get_font_family(collection, index).ok()
    } else {
        None
    }
}

pub struct FontCollection(pub IDWriteFontCollection);
impl FontCollection {
    pub fn get_font_family_count(&self) -> u32 {
        get_font_family_count(&self.0)
    }

    pub fn get_font_family(&self, index: u32) -> anyhow::Result<FontFamily> {
        get_font_family(&self.0, index).map(FontFamily)
    }

    pub fn find_family(&self, family_name: &str) -> Option<FontFamily> {
        find_family(&self.0, family_name).map(FontFamily)
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
