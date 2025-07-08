use std::env::current_dir;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn find_root(target: &Path) -> Option<PathBuf> {
    let mut dir = current_dir().ok()?;
    println!("Searching for {:?} starting from {}", target, dir.display());

    loop {
        if dir.join(target).exists() {
            println!("Found {:?} at {}", target, dir.display());
            return Some(dir);
        }

        match dir.parent() {
            Some(parent) => {
                dir = parent.to_path_buf();
            }
            None => {
                println!("Reached filesystem root without finding {target:?}");
                return None;
            }
        }
    }
}

pub fn set_permissions(target: &PathBuf, mode: u32) {
    #[cfg(unix)]
    fs::set_permissions(target, fs::Permissions::from_mode(mode))
        .unwrap_or_else(|_| panic!("Failed to set {target:?} as executable"));
}
