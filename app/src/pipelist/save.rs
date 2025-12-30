use std::{fs::{create_dir_all, write}, path::PathBuf, rc::Rc};
use slint::{Model, VecModel};
use crate::Pipe;

pub fn to_file(
    file_path: PathBuf,
    pipelist: Rc<VecModel<Pipe>>
) -> anyhow::Result<()>
{
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            let _ = create_dir_all(parent);   
        }
    }

    let json = serde_json::to_string_pretty(
        &pipelist.iter().collect::<Vec<_>>()
    )?;

    write(file_path, json)?;

    Ok(())
}