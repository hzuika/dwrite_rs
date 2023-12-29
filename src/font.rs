use bitflags::bitflags;
use windows::Win32::{
    Foundation::BOOL,
    Graphics::DirectWrite::{
        IDWriteFont, IDWriteFontFamily, IDWriteLocalizedStrings, DWRITE_FONT_SIMULATIONS,
        DWRITE_FONT_SIMULATIONS_BOLD, DWRITE_FONT_SIMULATIONS_NONE,
        DWRITE_FONT_SIMULATIONS_OBLIQUE, DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE,
        DWRITE_INFORMATIONAL_STRING_DESCRIPTION, DWRITE_INFORMATIONAL_STRING_DESIGNER,
        DWRITE_INFORMATIONAL_STRING_DESIGNER_URL,
        DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG,
        DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL, DWRITE_INFORMATIONAL_STRING_FULL_NAME,
        DWRITE_INFORMATIONAL_STRING_ID, DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION,
        DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL, DWRITE_INFORMATIONAL_STRING_MANUFACTURER,
        DWRITE_INFORMATIONAL_STRING_NONE, DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME,
        DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
        DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES,
        DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES,
        DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT,
        DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG,
        DWRITE_INFORMATIONAL_STRING_TRADEMARK,
        DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES,
        DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES,
        DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS,
        DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME,
        DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES,
        DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES,
        DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME,
    },
};

use crate::{font_family::FontFamily, localized_strings::LocalizedStrings};

pub fn get_simulations(font: &IDWriteFont) -> DWRITE_FONT_SIMULATIONS {
    unsafe { font.GetSimulations() }
}

pub fn get_face_names(font: &IDWriteFont) -> anyhow::Result<IDWriteLocalizedStrings> {
    unsafe { Ok(font.GetFaceNames()?) }
}

pub fn get_informational_strings(
    font: &IDWriteFont,
    informational_string_id: DWRITE_INFORMATIONAL_STRING_ID,
) -> anyhow::Result<Option<IDWriteLocalizedStrings>> {
    let mut exists = BOOL::from(false);
    let mut strings = None;
    unsafe {
        font.GetInformationalStrings(informational_string_id, &mut strings, &mut exists)?;
    }
    if exists.as_bool() {
        Ok(strings)
    } else {
        Ok(None)
    }
}

pub fn get_font_family(font: &IDWriteFont) -> anyhow::Result<IDWriteFontFamily> {
    unsafe {
        let family = font.GetFontFamily()?;
        Ok(family)
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Simulations: i32 {
        const None = DWRITE_FONT_SIMULATIONS_NONE.0;
        const Bold = DWRITE_FONT_SIMULATIONS_BOLD.0;
        const Oblique = DWRITE_FONT_SIMULATIONS_OBLIQUE.0;
    }
}

pub enum InformationalStringId {
    None,
    CopyrightNotice,
    VersionStrings,
    Trademark,
    Manufacturer,
    Designer,
    DesignerUrl,
    Description,
    FontVendorUrl,
    LicenseDescription,
    LicenseInfoUrl,
    Win32FamilyNames,
    Win32SubfamilyNames,
    TypographicFamilyNames,
    TypographicSubfamilyNames,
    SampleText,
    FullName,
    PostscriptName,
    PostscriptCidName,
    WeightStretchStyleFamilyName,
    DesignScriptLanguageTag,
    SupportedScriptLanguageTag,
    PreferredFamilyNames,
    PreferredSubfamilyNames,
    WwsFamilyName,
}

pub struct Font(pub IDWriteFont);

impl Font {
    pub fn get_font_family(&self) -> anyhow::Result<FontFamily> {
        get_font_family(&self.0).map(FontFamily)
    }

    pub fn get_simulations(&self) -> Simulations {
        Simulations::from_bits_retain(get_simulations(&self.0).0)
    }

    pub fn get_face_names(&self) -> anyhow::Result<LocalizedStrings> {
        get_face_names(&self.0).map(LocalizedStrings)
    }

    pub fn get_informational_strings(
        &self,
        informational_string_id: InformationalStringId,
    ) -> anyhow::Result<Option<LocalizedStrings>> {
        let informational_string_id = match informational_string_id {
            InformationalStringId::None => DWRITE_INFORMATIONAL_STRING_NONE,
            InformationalStringId::CopyrightNotice => DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE,
            InformationalStringId::VersionStrings => DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS,
            InformationalStringId::Trademark => DWRITE_INFORMATIONAL_STRING_TRADEMARK,
            InformationalStringId::Manufacturer => DWRITE_INFORMATIONAL_STRING_MANUFACTURER,
            InformationalStringId::Designer => DWRITE_INFORMATIONAL_STRING_DESIGNER,
            InformationalStringId::DesignerUrl => DWRITE_INFORMATIONAL_STRING_DESIGNER_URL,
            InformationalStringId::Description => DWRITE_INFORMATIONAL_STRING_DESCRIPTION,
            InformationalStringId::FontVendorUrl => DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL,
            InformationalStringId::LicenseDescription => {
                DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION
            }
            InformationalStringId::LicenseInfoUrl => DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL,
            InformationalStringId::Win32FamilyNames => {
                DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES
            }
            InformationalStringId::Win32SubfamilyNames => {
                DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES
            }
            InformationalStringId::TypographicFamilyNames => {
                DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES
            }
            InformationalStringId::TypographicSubfamilyNames => {
                DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES
            }
            InformationalStringId::SampleText => DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT,
            InformationalStringId::FullName => DWRITE_INFORMATIONAL_STRING_FULL_NAME,
            InformationalStringId::PostscriptName => DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
            InformationalStringId::PostscriptCidName => {
                DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME
            }
            InformationalStringId::WeightStretchStyleFamilyName => {
                DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME
            }
            InformationalStringId::DesignScriptLanguageTag => {
                DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG
            }
            InformationalStringId::SupportedScriptLanguageTag => {
                DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG
            }
            InformationalStringId::PreferredFamilyNames => {
                DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES
            }
            InformationalStringId::PreferredSubfamilyNames => {
                DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES
            }
            InformationalStringId::WwsFamilyName => DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME,
        };
        Ok(get_informational_strings(&self.0, informational_string_id)?.map(LocalizedStrings))
    }
}
