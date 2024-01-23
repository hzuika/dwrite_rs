use std::os::raw::c_void;

use windows::{core::PCWSTR, Win32::Graphics::DirectWrite::IDWriteLocalFontFileLoader};

pub fn get_file_path_from_key(
    loader: &IDWriteLocalFontFileLoader,
    key: *const c_void,
    size: u32,
) -> anyhow::Result<String> {
    let length = unsafe { loader.GetFilePathLengthFromKey(key, size) }?;
    let mut filepath = vec![0_u16; (length + 1) as usize];
    unsafe { loader.GetFilePathFromKey(key, size, filepath.as_mut_slice()) }?;
    Ok(unsafe { PCWSTR::from_raw(filepath.as_ptr()).to_string()? })
}
