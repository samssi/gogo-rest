use std::fs::write;
use std::path::Path;
use std::{fs, io};

fn read_entries(dir: &Path) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries: Vec<_> = fs::read_dir(dir)?.filter_map(Result::ok).collect();
    entries.sort_by_key(|e| e.path());
    Ok(entries)
}

fn create_mod_rs_lines(entries: &[fs::DirEntry]) -> io::Result<Vec<String>> {
    Ok(entries
        .iter()
        .filter_map(|entry| {
            let path = entry.path();
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            if path.is_dir() {
                Some(format!("pub mod {};", name))
            } else if path.extension()? == "rs" && name != "mod.rs" {
                Some(format!("pub mod {};", path.file_stem()?.to_string_lossy()))
            } else {
                None
            }
        })
        .collect())
}

fn write_mod_rs(dir: &Path, lines: &[String]) -> io::Result<()> {
    let mod_rs_path = dir.join("mod.rs");
    let contents = lines.join("\n");
    write(mod_rs_path, contents + "\n")
}

pub fn generate_mod_rs(dir: &Path) -> io::Result<()> {
    let entries = read_entries(dir)?;
    let mod_lines = create_mod_rs_lines(&entries)?;

    write_mod_rs(dir, &mod_lines)
}
