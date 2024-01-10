use dwrite_rs::{
    factory::{Factory, FactoryType},
    font::{Font, InformationalStringId, Simulations},
    localized_strings::LocalizedStrings,
};
use gdi_rs::UTF16String;
use std::io::Write;

fn get_string_id(font: &Font, id: InformationalStringId) -> anyhow::Result<String> {
    let strings = font.get_informational_strings(id)?;
    match strings {
        Some(strings) => get_string(&strings),
        None => Ok("".to_string()),
    }
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

fn main() -> anyhow::Result<()> {
    println!("GDI と DWrite のフォント列挙の名前をすべて表示する");
    let factory = Factory::new(FactoryType::Shared)?;
    let interop = factory.get_gdi_interop()?;
    let collection = factory.get_system_font_collection(true)?;
    let strings = [
        "Family",
        "Face",
        "Full",
        "Win32 family",
        "Win32 Subfamily",
        "lfFaceName",
        "lfWeight",
        "lfItalic",
        "Weight",
        "Style",
        "Stretch",
        "Typographic family",
        "Typographic subfamily",
        "Preferred family",
        "Preferred subfamily",
    ]
    .join("\t");

    let parent = std::path::Path::new(file!()).parent().unwrap();
    let path = parent.join("dwrite.tsv");
    let mut file = std::fs::File::create(path).unwrap();
    writeln!(&mut file, "{}", strings)?;

    for family in &collection {
        let family_name = get_string(&family.get_family_names()?)?;
        for font in &family {
            if font.get_simulations() != Simulations::None {
                continue;
            }

            let face_name = get_string(&font.get_face_names()?)?;

            let win32_family_name = get_string_id(&font, InformationalStringId::Win32FamilyNames)?;
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

            let (logfont, _) = interop.convert_font_to_logfont(&font)?;
            let lf_facename = UTF16String(logfont.lfFaceName).to_string();
            let lf_weight = logfont.lfWeight;
            let lf_italic = logfont.lfItalic != 0;

            let weight = font.get_weight();
            let style = font.get_style();
            let stretch = font.get_stretch();

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
                    lf_weight.to_string(),
                    lf_italic.to_string(),
                    weight.to_string(),
                    style.to_string(),
                    stretch.to_string(),
                    typographic_family_name,
                    typographic_subfamily_name,
                    preferred_family_name,
                    preferred_subfamily_name
                ]
                .join("\t")
            )?;
        }
    }

    Ok(())
}
