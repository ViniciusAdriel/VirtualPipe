use std::{ fs, path::PathBuf };
use crate::Settings;

pub fn to_file(
    file_path: PathBuf,
    settings: &Settings
) -> anyhow::Result<()>

{
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            let _ = fs::create_dir_all(parent);   
        }
    }

    let json = serde_json::to_string_pretty(settings)?;

    fs::write(&file_path, json)?;
    println!("Settings saved in {:?}", file_path);

    Ok(())
}