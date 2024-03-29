use core::fmt;

use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{Foundation::BOOL, Graphics::DirectWrite::IDWriteLocalizedStrings},
};

pub fn get_string(strings: &IDWriteLocalizedStrings, index: u32) -> anyhow::Result<String> {
    unsafe {
        let len = strings.GetStringLength(index)?;
        let mut buf = vec![0_u16; len as usize + 1];
        strings.GetString(index, buf.as_mut_slice())?;
        Ok(PCWSTR::from_raw(buf.as_ptr()).to_string()?)
    }
}

pub fn get_locale_name(strings: &IDWriteLocalizedStrings, index: u32) -> anyhow::Result<String> {
    unsafe {
        let len = strings.GetLocaleNameLength(index)?;
        let mut buf = vec![0_u16; len as usize + 1];
        strings.GetLocaleName(index, buf.as_mut_slice())?;
        Ok(PCWSTR::from_raw(buf.as_ptr()).to_string()?)
    }
}

pub fn find_locale_name(
    strings: &IDWriteLocalizedStrings,
    locale: &str,
) -> anyhow::Result<Option<u32>> {
    let mut index: u32 = 0;
    let mut exists = BOOL::from(false);
    unsafe {
        strings.FindLocaleName(&HSTRING::from(locale), &mut index, &mut exists)?;
    }
    if exists.as_bool() {
        Ok(Some(index))
    } else {
        Ok(None)
    }
}

pub struct LocalizedStrings(pub IDWriteLocalizedStrings);

impl LocalizedStrings {
    pub fn get_count(&self) -> u32 {
        unsafe { self.0.GetCount() }
    }

    pub fn get_string(&self, index: u32) -> anyhow::Result<String> {
        get_string(&self.0, index)
    }

    pub fn get_locale_name(&self, index: u32) -> anyhow::Result<String> {
        get_locale_name(&self.0, index)
    }

    pub fn find_locale_name(&self, locale: &str) -> anyhow::Result<Option<u32>> {
        find_locale_name(&self.0, locale)
    }

    pub fn get(&self, locale: &str) -> anyhow::Result<Option<String>> {
        let index = self.find_locale_name(locale)?;
        match index {
            Some(index) => Ok(Some(self.get_string(index)?)),
            None => Ok(None),
        }
    }
}

pub struct LocalizedStringsIter<'a> {
    strings: &'a LocalizedStrings,
    index: usize,
}

impl<'a> Iterator for LocalizedStringsIter<'a> {
    type Item = LocalizedString;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len() {
            let string = LocalizedString::from(&self.strings.0, self.index as u32);
            self.index += 1;
            match string {
                Ok(string) => Some(string),
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

impl<'a> ExactSizeIterator for LocalizedStringsIter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.strings.get_count() as usize
    }
}

impl<'a> IntoIterator for &'a LocalizedStrings {
    type Item = LocalizedString;
    type IntoIter = LocalizedStringsIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            strings: self,
            index: 0,
        }
    }
}

// IDWriteLocalizedStrings に含まれるロケール文字列一つ分を表す構造体．
pub struct LocalizedString {
    pub string: String,
    pub locale: String,
}

impl LocalizedString {
    pub fn from(strings: &IDWriteLocalizedStrings, index: u32) -> anyhow::Result<Self> {
        Ok(Self {
            string: get_string(strings, index)?,
            locale: get_locale_name(strings, index)?,
        })
    }
}

impl fmt::Display for LocalizedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.string, self.locale)
    }
}
