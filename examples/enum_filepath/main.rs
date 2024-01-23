use dwrite_rs::{factory::Factory, font::Simulations};

fn main() -> anyhow::Result<()> {
    let factory = Factory::new(dwrite_rs::factory::FactoryType::Shared)?;
    let collection = factory.get_system_font_collection(true)?;
    for family in &collection {
        for font in &family {
            if font.get_simulations() != Simulations::None {
                continue;
            }

            let filepath = font.get_filepath()?;
            println!("{}", filepath);
        }
    }
    Ok(())
}
