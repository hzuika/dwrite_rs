use anyhow::Context;
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory, IDWriteFontCollection, IDWriteGdiInterop,
    DWRITE_FACTORY_TYPE_ISOLATED, DWRITE_FACTORY_TYPE_SHARED,
};

use crate::{font_collection::FontCollection, gdi_interop::GdiInterop};

pub struct Factory(pub IDWriteFactory);

pub enum FactoryType {
    Isolated,
    Shared,
}

pub fn create_factory(factory_type: FactoryType) -> anyhow::Result<IDWriteFactory> {
    match factory_type {
        FactoryType::Isolated => Ok(unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_ISOLATED) }?),
        FactoryType::Shared => Ok(unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED) }?),
    }
}

pub fn get_system_font_collection(
    factory: &IDWriteFactory,
    check_for_updates: bool,
) -> anyhow::Result<IDWriteFontCollection> {
    let mut collection = None;
    unsafe { factory.GetSystemFontCollection(&mut collection, check_for_updates) }?;
    collection.context("Collection is none")
}

pub fn get_gdi_interop(factory: &IDWriteFactory) -> anyhow::Result<IDWriteGdiInterop> {
    unsafe { Ok(factory.GetGdiInterop()?) }
}

impl Factory {
    pub fn new(factory_type: FactoryType) -> anyhow::Result<Self> {
        Ok(Self(create_factory(factory_type)?))
    }

    pub fn get_system_font_collection(
        &self,
        check_for_updates: bool,
    ) -> anyhow::Result<FontCollection> {
        get_system_font_collection(&self.0, check_for_updates).map(FontCollection)
    }

    pub fn get_gdi_interop(&self) -> anyhow::Result<GdiInterop> {
        get_gdi_interop(&self.0).map(GdiInterop)
    }
}
