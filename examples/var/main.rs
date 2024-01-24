use dwrite_rs::{
    factory::{Factory, FactoryType},
    localized_strings::LocalizedStrings,
};

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
    let factory = Factory::new(FactoryType::Shared)?;
    let collection = factory.get_system_font_collection(true)?;
    for family in &collection {
        let family_name = get_string(&family.get_family_names()?)?;
        for font in &family {
            if font.is_simulation() {
                continue;
            }

            if font.is_variable_font()? {
                let face_name = get_string(&font.get_face_names()?)?;
                println!("{} {}", family_name, face_name);
            }
        }
    }
    Ok(())
}
