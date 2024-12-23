use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn init(target_dir: &str) -> Result<()> {
    //Define the path to the .mygit directory
    let path = PathBuf::from(target_dir).canonicalize()?;
    let mygit_dir = path.join(".mygit");

    //Check if the directory already exists
    if mygit_dir.exists() {
        println!(
            "Already initialized a mygit repository in {}",
            mygit_dir.display()
        );
        return Ok(());
    }

    //Create the .mygit directory and its subdirectories
    fs::create_dir(&mygit_dir)?;
    fs::create_dir_all(mygit_dir.join("objects"))?;
    fs::create_dir_all(mygit_dir.join("refs"))?;

    //Create the HEAD file pointing to the master branch
    let head_file_path = mygit_dir.join("HEAD");
    fs::write(&head_file_path, "ref: refs/heads/master\n")?;

    println!(
        "Initialized empty mygit repository in {}\n",
        mygit_dir.display()
    );
    Ok(())
}
