use dwrite_rs::factory::{Factory, FactoryType};

fn main() -> anyhow::Result<()> {
    let factory = Factory::new(FactoryType::Shared)?;
    let collection = factory.get_system_font_collection(true)?;
    let family = collection.find_family("ＭＳ Ｐゴシック").unwrap();
    let family_names = family.get_family_names()?;
    for family_name in &family_names {
        println!("{}", family_name);
    }
    let family = collection.find_family("MS PGothic").unwrap();
    let family_names = family.get_family_names()?;
    for family_name in &family_names {
        println!("{}", family_name);
    }
    Ok(())
}
