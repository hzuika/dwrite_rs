use windows::Win32::Graphics::DirectWrite::{IDWriteFontFace, IDWriteFontFile};

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
