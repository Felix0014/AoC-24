use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn copy_dir_all(src: &Path, dest: &Path) -> std::io::Result<()> {
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if path.is_dir() {
            copy_dir_all(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap(); // Gets the `target/debug` or `target/release` directory.
    let data_dir = PathBuf::from("data");

    if data_dir.exists() {
        copy_dir_all(&data_dir, &target_dir).expect("Failed to copy data folder");
    }
}
