use std::{cmp::Ordering, io::Write};

use dwrite_rs::localized_strings::LocalizedString;
use windows::Win32::Graphics::DirectWrite::{
    DWriteCreateFactory, IDWriteFactory6, IDWriteLocalizedStrings, DWRITE_FACTORY_TYPE_ISOLATED,
    DWRITE_FONT_FAMILY_MODEL, DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC,
    DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE, DWRITE_FONT_SIMULATIONS_NONE,
};

// (string, locale) の tuple を返す．
fn get_strings(strings: &IDWriteLocalizedStrings) -> anyhow::Result<Vec<LocalizedString>> {
    let mut v = Vec::new();
    unsafe {
        let count = strings.GetCount();
        for i in 0..count {
            v.push(LocalizedString::from(strings, i)?);
        }
    }
    sort(&mut v);
    Ok(v)
}

struct FamilyInfo {
    names: Vec<LocalizedString>,
    faces: Vec<Vec<LocalizedString>>,
}

fn sort(s: &mut Vec<LocalizedString>) {
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

fn join(s: &[LocalizedString]) -> String {
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

enum Model {
    Typo,
    WSS,
}

impl Model {
    fn get_model(&self) -> DWRITE_FONT_FAMILY_MODEL {
        match self {
            Self::Typo => DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC,
            Self::WSS => DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE,
        }
    }
}

fn run(
    include_downloadable_fonts: bool,
    check_for_updates: bool,
    model: Model,
    filename: &str,
) -> anyhow::Result<()> {
    unsafe {
        let factory: IDWriteFactory6 = DWriteCreateFactory(DWRITE_FACTORY_TYPE_ISOLATED)?;

        // 一度実行だけする．
        {
            let mut collection = None;
            factory.GetSystemFontCollection(&mut collection, check_for_updates)?;
        }

        let collection =
            factory.GetSystemFontCollection3(include_downloadable_fonts, model.get_model())?;
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
                let font = family.GetFont(j);
                match font {
                    Ok(font) => {
                        let simulations = font.GetSimulations();
                        if simulations != DWRITE_FONT_SIMULATIONS_NONE {
                            continue;
                        }
                        let face_names = font.GetFaceNames()?;
                        let face_names = get_strings(&face_names)?;
                        info.faces.push(face_names);
                    }
                    Err(e) => {
                        info.faces.push(vec![LocalizedString {
                            string: e.to_string(),
                            locale: "Error".to_string(),
                        }]);
                    }
                }
            }

            infos.push(info);
        }

        infos.sort_by(|a, b| a.names[0].string.cmp(&b.names[0].string));

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
    run(false, false, Model::Typo, "typo.txt")?;
    run(false, true, Model::Typo, "update_typo.txt")?;
    run(true, false, Model::Typo, "download_typo.txt")?;
    run(true, true, Model::Typo, "download_update_typo.txt")?;

    run(false, false, Model::WSS, "wss.txt")?;
    run(false, true, Model::WSS, "update_wss.txt")?;
    run(true, false, Model::WSS, "download_wss.txt")?;
    run(true, true, Model::WSS, "download_update_wss.txt")?;
    Ok(())
}
