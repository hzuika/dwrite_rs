use dwrite_rs::factory::{Factory, FactoryType};

fn main() -> anyhow::Result<()> {
    let factory = Factory::new(FactoryType::Shared)?;
    let collection = factory.get_system_font_collection(true)?;
    for family in &collection {
        let family_names = family.get_family_names()?;
        for (i, family_name) in family_names.into_iter().enumerate() {
            println!("[{}] {}", i, family_name);
        }
    }
    Ok(())
}
