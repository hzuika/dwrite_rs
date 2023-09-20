use windows::{core::PCWSTR, Win32::Graphics::DirectWrite::IDWriteLocalizedStrings};

pub fn get_string(strings: &IDWriteLocalizedStrings, index: u32) -> anyhow::Result<String> {
    unsafe {
        let len = strings.GetStringLength(index)?;
        let mut buf = vec![0_u16; len as usize + 1];
        strings.GetString(index, buf.as_mut_slice())?;
        let wstr = PCWSTR::from_raw(buf.as_ptr());
        let str = wstr.to_string().unwrap();
        Ok(str)
    }
}

pub fn get_locale(strings: &IDWriteLocalizedStrings, index: u32) -> anyhow::Result<String> {
    unsafe {
        let len = strings.GetLocaleNameLength(index)?;
        let mut buf = vec![0_u16; len as usize + 1];
        strings.GetLocaleName(index, buf.as_mut_slice())?;
        let wstr = PCWSTR::from_raw(buf.as_ptr());
        let locale = wstr.to_string().unwrap();
        Ok(locale)
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
            locale: get_locale(strings, index)?,
        })
    }

    pub fn to_string(&self) -> String {
        format!("{} ({})", self.string, self.locale)
    }
}
