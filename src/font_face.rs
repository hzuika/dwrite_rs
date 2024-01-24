use windows::Win32::{
    Foundation::BOOL,
    Graphics::DirectWrite::{IDWriteFontFace, IDWriteFontFile},
};

use crate::font_file;

pub fn get_number_of_files(face: &IDWriteFontFace) -> anyhow::Result<u32> {
    let mut number_of_files = 0;
    unsafe { face.GetFiles(&mut number_of_files, None) }?;
    return Ok(number_of_files);
}

pub fn get_files(face: &IDWriteFontFace) -> anyhow::Result<IDWriteFontFile> {
    let mut number_of_files = get_number_of_files(face)?;
    assert!(number_of_files == 1);
    let mut files: Vec<Option<IDWriteFontFile>> = vec![None; number_of_files as usize];
    unsafe { face.GetFiles(&mut number_of_files, Some(files.as_mut_ptr())) }?;
    let file = files[0].clone().unwrap();
    Ok(file)
}

pub fn get_filepath(face: &IDWriteFontFace) -> anyhow::Result<String> {
    let file = get_files(face)?;
    font_file::get_filepath(&file)
}

pub fn map_font_table<F: FnOnce(&[u8])>(
    face: &IDWriteFontFace,
    tag: u32,
    f: F,
) -> anyhow::Result<()> {
    let mut exists = BOOL::from(false);
    let mut data = std::ptr::null_mut();
    let mut size = 0_u32;
    let mut context = std::ptr::null_mut();
    unsafe { face.TryGetFontTable(tag, &mut data, &mut size, &mut context, &mut exists) }?;
    if exists.as_bool() {
        if data.is_null() {
            assert!(false);
            return Err(anyhow::anyhow!("data exists but is null"));
        }
        if size == 0 {
            assert!(false);
            return Err(anyhow::anyhow!("data exists but size is zero"));
        }
        let data = unsafe { std::slice::from_raw_parts(data as *const u8, size as usize) };
        f(data);
        unsafe { face.ReleaseFontTable(context) };
    }
    Ok(())
}

pub fn get_font_table(face: &IDWriteFontFace, tag: u32) -> anyhow::Result<Vec<u8>> {
    let mut table = Vec::new();
    map_font_table(face, tag, |data| {
        table = data.to_vec();
    })?;
    Ok(table)
}

pub fn is_variable_font(face: &IDWriteFontFace) -> anyhow::Result<bool> {
    let mut is_variable_font = false;
    map_font_table(face, u32::from_le_bytes(*b"fvar"), |data| {
        if data.len() > 0 {
            is_variable_font = true;
        }
    })?;
    Ok(is_variable_font)
}
