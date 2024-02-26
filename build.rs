use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("icons.rs");
    let mut f = File::create(dest_path)?;

    let icons_dir = Path::new("icons");

    // First, generate the constants for each SVG file
    for entry in fs::read_dir(icons_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let name = path.file_stem().unwrap().to_str().unwrap();
            let const_name = to_screaming_snake_case(name);
            let mut file_content = String::new();
            File::open(&path)?.read_to_string(&mut file_content)?;

            writeln!(f, "const {}: &str = r##\"{}\"##;", const_name, file_content)?;
        }
    }

    // Then, declare the Icon enum
    writeln!(f, "\n#[derive(Clone, Copy, Debug)]")?;
    writeln!(f, "\npub enum Icon {{")?;
    for entry in fs::read_dir(icons_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let name = path.file_stem().unwrap().to_str().unwrap();
            let enum_name = to_camel_case(name);

            writeln!(f, "    {},", enum_name)?;
        }
    }
    writeln!(f, "}}")?;

    // Generate the get_svg function
    writeln!(f, "\nimpl Icon {{")?;
    writeln!(f, "    pub const fn get_svg(&self) -> &'static str {{")?;
    writeln!(f, "        match self {{")?;
    for entry in fs::read_dir(icons_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let name = path.file_stem().unwrap().to_str().unwrap();
            let enum_name = to_camel_case(name);
            let const_name = to_screaming_snake_case(name);

            writeln!(f, "            Icon::{} => {},", enum_name, const_name)?;
        }
    }
    writeln!(f, "        }}")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;

    Ok(())
}

fn to_camel_case(s: &str) -> String {
    s.split('-')
        .map(|part| part[0..1].to_uppercase() + &part[1..])
        .collect()
}

fn to_screaming_snake_case(s: &str) -> String {
    s.to_uppercase().replace('-', "_")
}
