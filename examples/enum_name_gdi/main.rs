use std::collections::{HashMap, HashSet};
use std::io::Write;

use dwrite_rs::{
    factory::{Factory, FactoryType},
    font::{Font, InformationalStringId},
    localized_strings::LocalizedStrings,
};
use gdi_rs::{enum_font_families_ex, is_vertical, UTF16String};
use windows::Win32::Graphics::Gdi::DEFAULT_CHARSET;

#[derive(PartialEq, PartialOrd, Eq, Hash)]
struct FontInfo {
    face_name: String,
    weight: i32,
    italic: bool,
}

fn get_string(strings: &LocalizedStrings) -> anyhow::Result<String> {
    match strings.get("ja-jp")? {
        Some(string) => Ok(string),
        None => match strings.get("en-us")? {
            Some(string) => Ok(string),
            None => strings.get_string(0),
        },
    }
}

fn get_string_id(font: &Font, id: InformationalStringId) -> anyhow::Result<String> {
    let strings = font.get_informational_strings(id)?;
    match strings {
        Some(strings) => get_string(&strings),
        None => Ok("".to_string()),
    }
}

fn main() -> anyhow::Result<()> {
    let mut lffacenames = HashSet::<[u16; 32]>::new();
    enum_font_families_ex([0; 32], DEFAULT_CHARSET, |args| {
        if !args.is_opentype() {
            return 1;
        }

        let logfont = args.get_logfont().unwrap();
        if is_vertical(logfont) {
            return 1;
        }
        lffacenames.insert(logfont.lfFaceName);
        return 1;
    });

    let mut lffacenames: Vec<[u16; 32]> = lffacenames.into_iter().collect();
    lffacenames.sort();

    let mut font_info_map = HashMap::new();
    for lffacename in lffacenames {
        let face_name = UTF16String(lffacename).to_string();
        enum_font_families_ex(lffacename, DEFAULT_CHARSET, |args| {
            if !args.is_opentype() {
                return 1;
            }
            let logfont = args.get_logfont().unwrap();
            if is_vertical(logfont) {
                return 1;
            }
            let weight = logfont.lfWeight;
            let italic = logfont.lfItalic != 0;
            let font_info = FontInfo {
                face_name: face_name.clone(),
                weight,
                italic,
            };

            font_info_map.insert(font_info, *logfont);

            return 1;
        });
    }

    let strings = [
        "Family",
        "Face",
        "Full",
        "Win32 family",
        "Win32 Subfamily",
        "lfFaceName",
        "Weight",
        "Italic",
        "Typographic family",
        "Typographic subfamily",
        "Preferred family",
        "Preferred subfamily",
    ]
    .join("\t");

    let parent = std::path::Path::new(file!()).parent().unwrap();
    let path = parent.join("gdi.tsv");
    let mut file = std::fs::File::create(path).unwrap();
    writeln!(&mut file, "{}", strings)?;

    let factory = Factory::new(FactoryType::Shared)?;
    let interop = factory.get_gdi_interop()?;
    for (_, logfont) in font_info_map {
        let lf_facename = UTF16String(logfont.lfFaceName).to_string();
        let weight = logfont.lfWeight;
        let italic = logfont.lfItalic != 0;

        let font = interop.create_font_from_logfont(&logfont);
        match font {
            Ok(font) => {
                let font = Font(font);
                let family = font.get_font_family()?;

                let face_name = get_string(&font.get_face_names()?)?;
                let family_name = get_string(&family.get_family_names()?)?;

                let win32_family_name =
                    get_string_id(&font, InformationalStringId::Win32FamilyNames)?;
                let win32_subfamily_name =
                    get_string_id(&font, InformationalStringId::Win32SubfamilyNames)?;

                let typographic_family_name =
                    get_string_id(&font, InformationalStringId::TypographicFamilyNames)?;
                let typographic_subfamily_name =
                    get_string_id(&font, InformationalStringId::TypographicSubfamilyNames)?;

                let preferred_family_name =
                    get_string_id(&font, InformationalStringId::PreferredFamilyNames)?;
                let preferred_subfamily_name =
                    get_string_id(&font, InformationalStringId::PreferredSubfamilyNames)?;

                let full_name = get_string_id(&font, InformationalStringId::FullName)?;

                writeln!(
                    &mut file,
                    "{}",
                    [
                        family_name.clone(),
                        face_name,
                        full_name,
                        win32_family_name,
                        win32_subfamily_name,
                        lf_facename,
                        weight.to_string(),
                        italic.to_string(),
                        typographic_family_name,
                        typographic_subfamily_name,
                        preferred_family_name,
                        preferred_subfamily_name
                    ]
                    .join("\t")
                )?;
            }
            Err(_) => {
                writeln!(
                    &mut file,
                    "{}",
                    [
                        "",
                        "",
                        "",
                        "",
                        "",
                        &lf_facename,
                        &(weight.to_string()),
                        &(italic.to_string()),
                        "",
                        "",
                        "",
                        ""
                    ]
                    .join("\t")
                )?;
            }
        }
    }

    Ok(())
}
