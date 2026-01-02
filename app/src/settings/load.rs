use std::{
    fs,
    fs::File,
    path::PathBuf,
};
use serde_json::Value;
use crate::Settings;



pub fn from_file(file_path: PathBuf) -> Settings
{
    if !file_path.exists() {
        if let Some(parent) = file_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = File::create(&file_path);
    }
    
    let content = fs::read_to_string(&file_path).unwrap_or_default();

    let v: Value = serde_json::from_str(&content)
        .unwrap_or_else(|_| Value::Object(Default::default()));

    Settings {
        first_run: v
            .get("first_run")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
        
        app_language: v
            .get("app_language")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .into(),

        restore_on_system_startup: v
            .get("restore_on_system_startup")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),

        confirm_to_delete_pipe: v
            .get("confirm_to_delete_pipe")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
    }
}
