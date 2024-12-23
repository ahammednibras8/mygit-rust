use std::fs;
use std::path::Path;
use sha1::{Digest, Sha1};

pub fn add(file_path: &str) -> Result<(), anyhow::Error> {
    //Checking .mygit directory exists
    let mygit_dir = Path::new(".mygit");
    if !mygit_dir.exists() {
        return Err(anyhow::anyhow!("Not a mygit repository (or any of the parent directories)"));
    }

    //Checking if the file exists
    let file = Path::new(file_path);
    if !file.exists() {
        return Err(anyhow::anyhow!("File '{}' not found", file_path));
    }

    //Read files
    let file_contents = fs::read(file)?;

    //Calculate hash using sha1
    let mut hasher = Sha1::new();
    hasher.update(&file_contents);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    //Strorng the files in .mygit/objects/<hash>
    let object_path = mygit_dir.join("objects").join(&hash_hex);
    fs::write(object_path, file_contents)?;

    //Update the index file
    let index_entry = format!("{} {}\n", hash_hex, file_path);
    fs::write(mygit_dir.join("index"), index_entry.as_bytes())?;

    println!("File '{}' added to staging area", file_path);

    Ok(())
}