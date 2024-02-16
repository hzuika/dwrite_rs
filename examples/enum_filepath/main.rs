use dwrite_rs::factory::{Factory, FactoryType};

fn main() -> anyhow::Result<()> {
    let factory = Factory::new(FactoryType::Shared)?;
    let collection = factory.get_system_font_collection(true)?;
    for family in &collection {
        for font in &family {
            if font.is_simulation() {
                continue;
            }

            let filepath = font.get_filepath()?;
            println!("{}", filepath);
        }
    }
    Ok(())
}
