use anyhow::Result;
use sha1::{Digest, Sha1};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn add(file_path: &str) -> Result<(), anyhow::Error> {
    let current_dir = std::env::current_dir()?;
    let mygit_dir = current_dir.join(".mygit");

    //Check if the .mygit directory exists
    if !mygit_dir.exists() {
        return Err(anyhow::anyhow!(
            "Not a mygit repository (or any of the parent directories)"
        ));
    }

    //Resolve the path to the file
    let target = PathBuf::from(file_path).canonicalize()?;
    if !target.exists() {
        return Err(anyhow::anyhow!("Path '{}' not found", file_path));
    }

    //Process the file
    process_path(&target, &mygit_dir)?;

    Ok(())
}

fn process_path(target: &Path, mygit_dir: &Path) -> Result<(), anyhow::Error> {
    //Skip the .mygit directory
    if target.starts_with(mygit_dir) {
        return Ok(());
    }

    if target.is_dir() {
        for entry in fs::read_dir(target)? {
            let entry = entry?;
            let entry_path = entry.path();

            process_path(&entry_path, mygit_dir)?;
        }
    } else if target.is_file() {
        process_file(target.to_path_buf(), mygit_dir)?;
    } else {
        println!("Unsuported file type: {}", target.display());
    }

    Ok(())
}

fn process_file(file: PathBuf, mygit_dir: &Path) -> Result<(), anyhow::Error> {
    let file_contents = fs::read(&file)?;

    //calculate sha-1
    let mut hasher = Sha1::new();
    hasher.update(&file_contents);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    let object_path = mygit_dir.join("objects").join(&hash_hex);

    //Store the file contents in objects
    if !object_path.exists() {
        fs::write(&object_path, file_contents)?;
        println!("Stored object: {}", object_path.display());
    } else {
        println!("Object already exists: {}", object_path.display());
    }

    let file_path_relative = file.strip_prefix(std::env::current_dir()?)?;

    let index_entry = format!("{} {}", hash_hex, file_path_relative.display());

    //Adding entry to index
    let index_path = mygit_dir.join("index");
    let mut index_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&index_path)?;
    index_file.write_all(index_entry.as_bytes())?;

    println!("Added to index: {} {}", hash_hex, file_path_relative.display());
    Ok(())
}
