use std::io::Write;

use dwrite_rs::{
    factory::{Factory, FactoryType},
    font::{Font, InformationalStringId, Simulations},
};
use gdi_rs::UTF16String;

fn get_string(font: &Font, id: InformationalStringId) -> anyhow::Result<String> {
    let strings = font.get_informational_strings(id)?;
    match strings {
        Some(strings) => match strings.get("ja-jp")? {
            Some(string) => Ok(string),
            None => match strings.get("en-us")? {
                Some(string) => Ok(string),
                None => strings.get_string(0),
            },
        },
        None => Ok("".to_string()),
    }
}

fn main() -> anyhow::Result<()> {
    let factory = Factory::new(FactoryType::Shared)?;
    let gdi_interop = factory.get_gdi_interop()?;
    let collection = factory.get_system_font_collection(true)?;
    let family_count = collection.get_font_family_count();

    let filename = "name.tsv";
    let path = filename;
    let mut file = std::fs::File::create(path)?;
    writeln!(
        &mut file,
        "lfFaceName\tFull name\tWin32 family name\tWin32 subfamily name\tTypographic family name\tTypographic subfamily name"
    )?;

    for i in 0..family_count {
        let family = collection.get_font_family(i)?;
        let font_count = family.get_font_count();
        for j in 0..font_count {
            let font = family.get_font(j)?;
            if font.get_simulations() != Simulations::None {
                continue;
            }

            let (logfont, _) = gdi_interop.convert_font_to_logfont(&font)?;

            let facename = UTF16String(logfont.lfFaceName);

            let typographic_family_name =
                get_string(&font, InformationalStringId::TypographicFamilyNames)?;
            let typographic_subfamily_name =
                get_string(&font, InformationalStringId::TypographicSubfamilyNames)?;

            let fullname = get_string(&font, InformationalStringId::FullName)?;

            let win32_family_name = get_string(&font, InformationalStringId::Win32FamilyNames)?;
            let win32_subfamily_name =
                get_string(&font, InformationalStringId::Win32SubfamilyNames)?;

            writeln!(
                &mut file,
                "{}\t{}\t{}\t{}\t{}\t{}",
                facename,
                fullname,
                win32_family_name,
                win32_subfamily_name,
                typographic_family_name,
                typographic_subfamily_name
            )?;
        }
    }
    Ok(())
}
