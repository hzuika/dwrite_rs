use std::os::raw::c_void;

use windows::Win32::Graphics::DirectWrite::{IDWriteFontFile, IDWriteFontFileLoader};

use crate::{font_file_loader::get_local, local_font_file_loader::get_file_path_from_key};

pub fn get_reference_key(file: &IDWriteFontFile) -> anyhow::Result<(*const c_void, u32)> {
    // GetReferenceKey には初期化されていないポインタを渡すことができる．
    let mut reference_key = std::mem::MaybeUninit::uninit();
    let mut size = 0_u32;
    unsafe { file.GetReferenceKey(reference_key.as_mut_ptr(), &mut size) }?;
    Ok((unsafe { *reference_key.as_ptr() }, size))
}

pub fn get_loader(file: &IDWriteFontFile) -> anyhow::Result<IDWriteFontFileLoader> {
    let loader = unsafe { file.GetLoader() }?;
    Ok(loader)
}

pub fn get_filepath(file: &IDWriteFontFile) -> anyhow::Result<String> {
    let (key, size) = get_reference_key(file)?;
    let loader = get_loader(file)?;
    let loader = get_local(&loader)?;
    get_file_path_from_key(&loader, key, size)
}
