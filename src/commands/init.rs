use std::fs;
use std::path::Path;

pub fn init() -> Result<(), anyhow::Error> {
    let mygit_dir = Path::new(".mygit");

    if mygit_dir.exists() {
        println!("Already initialized a mygit repository.");
        return Ok(());
    }

    fs::create_dir(mygit_dir)?;
    fs::create_dir(mygit_dir.join("objects"))?;
    fs::create_dir(mygit_dir.join("refs"))?;
    fs::write(mygit_dir.join("HEAD"), "ref: refs/heads/master\n")?;

    println!(
        "Initialized empty mygit repository in {}\n",
        mygit_dir.display()
    );
    Ok(())
}
