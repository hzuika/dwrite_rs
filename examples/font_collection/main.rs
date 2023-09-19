use std::{cmp::Ordering, io::Write};

use windows::{
    core::PCWSTR,
    Win32::Graphics::DirectWrite::{
        DWriteCreateFactory, IDWriteFactory, IDWriteLocalizedStrings, DWRITE_FACTORY_TYPE_ISOLATED,
        DWRITE_FONT_SIMULATIONS_NONE,
    },
};

// (string, locale) の tuple を返す．
fn get_strings(strings: &IDWriteLocalizedStrings) -> anyhow::Result<Vec<LocaleName>> {
    let mut v = Vec::new();
    unsafe {
        let count = strings.GetCount();
        for i in 0..count {
            let len = strings.GetStringLength(i)?;
            let mut buf = vec![0_u16; len as usize + 1];
            strings.GetString(i, buf.as_mut_slice())?;
            let wstr = PCWSTR::from_raw(buf.as_ptr());
            let name = wstr.to_string().unwrap();

            let len = strings.GetLocaleNameLength(i)?;
            let mut buf = vec![0_u16; len as usize + 1];
            strings.GetLocaleName(i, buf.as_mut_slice())?;
            let wstr = PCWSTR::from_raw(buf.as_ptr());
            let locale = wstr.to_string().unwrap();

            v.push(LocaleName { name, locale });
        }
    }
    sort(&mut v);
    Ok(v)
}

struct LocaleName {
    name: String,
    locale: String,
}

impl LocaleName {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.locale)
    }
}

struct FamilyInfo {
    names: Vec<LocaleName>,
    faces: Vec<Vec<LocaleName>>,
}

fn sort(s: &mut Vec<LocaleName>) {
    s.sort_by(|a, b| {
        if a.locale == "ja-jp" {
            Ordering::Less
        } else if b.locale == "ja-jp" {
            Ordering::Greater
        } else if a.locale == "en-us" {
            Ordering::Less
        } else if b.locale == "en-us" {
            Ordering::Greater
        } else {
            a.locale.cmp(&b.locale)
        }
    })
}

fn join(s: &[LocaleName]) -> String {
    s.iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

impl FamilyInfo {
    fn to_string(&self) -> String {
        let names = join(&self.names);
        let mut s = names;
        for face in &self.faces {
            let face_names = join(face);
            s = format!("{}\n\t{}", s, face_names);
        }
        s
    }
}

fn run(check_for_updates: bool, filename: &str) -> anyhow::Result<()> {
    unsafe {
        let factory: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_ISOLATED)?;
        let mut collection = None;
        factory.GetSystemFontCollection(&mut collection, check_for_updates)?;
        let collection = collection.unwrap();
        let family_count = collection.GetFontFamilyCount();

        let mut infos = vec![];

        for i in 0..family_count {
            let family = collection.GetFontFamily(i)?;
            let family_names = family.GetFamilyNames()?;
            let family_names = get_strings(&family_names)?;
            let font_count = family.GetFontCount();
            let mut info = FamilyInfo {
                names: family_names,
                faces: vec![],
            };
            for j in 0..font_count {
                let font = family.GetFont(j)?;
                let simulations = font.GetSimulations();
                if simulations != DWRITE_FONT_SIMULATIONS_NONE {
                    continue;
                }
                let face_names = font.GetFaceNames()?;
                let face_names = get_strings(&face_names)?;
                info.faces.push(face_names);
            }

            infos.push(info);
        }

        infos.sort_by(|a, b| a.names[0].name.cmp(&b.names[0].name));

        let this_file = std::path::Path::new(file!());
        let parent = this_file.parent().unwrap();
        let path = parent.join(filename);
        let mut file = std::fs::File::create(path)?;
        writeln!(&mut file, "family count {}", family_count)?;
        for info in infos {
            writeln!(&mut file, "{}", info.to_string())?;
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    run(false, "false.txt")?;
    run(true, "true.txt")?;
    Ok(())
}
