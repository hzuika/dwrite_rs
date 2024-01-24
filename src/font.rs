use core::fmt;

use bitflags::bitflags;
use windows::Win32::{
    Foundation::BOOL,
    Graphics::DirectWrite::{
        IDWriteFont, IDWriteFontFace, IDWriteFontFamily, IDWriteLocalizedStrings,
        DWRITE_FONT_SIMULATIONS, DWRITE_FONT_SIMULATIONS_BOLD, DWRITE_FONT_SIMULATIONS_NONE,
        DWRITE_FONT_SIMULATIONS_OBLIQUE, DWRITE_FONT_STRETCH, DWRITE_FONT_STYLE,
        DWRITE_FONT_STYLE_ITALIC, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_STYLE_OBLIQUE,
        DWRITE_FONT_WEIGHT, DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE,
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

use crate::{font_face, font_family::FontFamily, localized_strings::LocalizedStrings};

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weight(pub i32);

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.0 {
            100 => "Thin".to_string(),
            200 => "ExtraLight".to_string(),
            300 => "Light".to_string(),
            350 => "SemiLight".to_string(),
            400 => "Regular".to_string(),
            500 => "Medium".to_string(),
            600 => "SemiBold".to_string(),
            700 => "Bold".to_string(),
            800 => "ExtraBold".to_string(),
            900 => "Black".to_string(),
            950 => "ExtraBlack".to_string(),
            _ => self.0.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl From<DWRITE_FONT_WEIGHT> for Weight {
    fn from(value: DWRITE_FONT_WEIGHT) -> Self {
        Self(value.0)
    }
}

pub fn get_weight(font: &IDWriteFont) -> DWRITE_FONT_WEIGHT {
    unsafe { font.GetWeight() }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Style {
    Normal,
    Oblique,
    Italic,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Italic => "Italic",
            Self::Normal => "Normal",
            Self::Oblique => "Oblique",
        };
        write!(f, "{}", s)
    }
}

impl From<DWRITE_FONT_STYLE> for Style {
    fn from(value: DWRITE_FONT_STYLE) -> Self {
        match value {
            DWRITE_FONT_STYLE_ITALIC => Self::Italic,
            DWRITE_FONT_STYLE_NORMAL => Self::Normal,
            DWRITE_FONT_STYLE_OBLIQUE => Self::Oblique,
            _ => panic!("invalid style"),
        }
    }
}

pub fn get_style(font: &IDWriteFont) -> DWRITE_FONT_STYLE {
    unsafe { font.GetStyle() }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stretch {
    Undefined = 0,
    UltraCondensed = 1,
    ExtraCondensed = 2,
    Condensed = 3,
    SemiCondensed = 4,
    Normal = 5,
    SemiExpanded = 6,
    Expanded = 7,
    ExtraExpanded = 8,
    UltraExpanded = 9,
}

impl fmt::Display for Stretch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Stretch::Undefined => "Undefined",
            Stretch::UltraCondensed => "UltraCondensed",
            Stretch::ExtraCondensed => "ExtraCondensed",
            Stretch::Condensed => "Condensed",
            Stretch::SemiCondensed => "SemiCondensed",
            Stretch::Normal => "Normal",
            Stretch::SemiExpanded => "SemiExpanded",
            Stretch::Expanded => "Expanded",
            Stretch::ExtraExpanded => "ExtraExpanded",
            Stretch::UltraExpanded => "UltraExpanded",
        };
        write!(f, "{}", s)
    }
}

impl From<DWRITE_FONT_STRETCH> for Stretch {
    fn from(value: DWRITE_FONT_STRETCH) -> Self {
        match value.0 {
            0 => Stretch::Undefined,
            1 => Stretch::UltraCondensed,
            2 => Stretch::ExtraCondensed,
            3 => Stretch::Condensed,
            4 => Stretch::SemiCondensed,
            5 => Stretch::Normal,
            6 => Stretch::SemiExpanded,
            7 => Stretch::Expanded,
            8 => Stretch::ExtraExpanded,
            9 => Stretch::UltraExpanded,
            _ => panic!("invalid stretch"),
        }
    }
}

pub fn get_stretch(font: &IDWriteFont) -> DWRITE_FONT_STRETCH {
    unsafe { font.GetStretch() }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

pub fn create_font_face(font: &IDWriteFont) -> anyhow::Result<IDWriteFontFace> {
    let face = unsafe { font.CreateFontFace() }?;
    Ok(face)
}

pub fn get_filepath(font: &IDWriteFont) -> anyhow::Result<String> {
    let face = create_font_face(font)?;
    font_face::get_filepath(&face)
}

pub struct Font(pub IDWriteFont);

impl Font {
    pub fn get_font_family(&self) -> anyhow::Result<FontFamily> {
        get_font_family(&self.0).map(FontFamily)
    }

    pub fn get_simulations(&self) -> Simulations {
        Simulations::from_bits_retain(get_simulations(&self.0).0)
    }

    pub fn get_weight(&self) -> Weight {
        get_weight(&self.0).into()
    }

    pub fn get_style(&self) -> Style {
        get_style(&self.0).into()
    }

    pub fn get_stretch(&self) -> Stretch {
        get_stretch(&self.0).into()
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

    pub fn get_filepath(&self) -> anyhow::Result<String> {
        get_filepath(&self.0)
    }
}
