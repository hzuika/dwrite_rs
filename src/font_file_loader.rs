use windows::{
    core::ComInterface,
    Win32::Graphics::DirectWrite::{IDWriteFontFileLoader, IDWriteLocalFontFileLoader},
};

pub fn get_local(loader: &IDWriteFontFileLoader) -> anyhow::Result<IDWriteLocalFontFileLoader> {
    let loader = loader.cast::<IDWriteLocalFontFileLoader>()?;
    Ok(loader)
}
