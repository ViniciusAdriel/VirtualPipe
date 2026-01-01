use std::{
    fs,
    fs::File,
    path::PathBuf,
};
use serde_json::Value;
use crate::Pipe;



pub fn from_file(file_path: PathBuf) -> Vec<Pipe>
{
    if !file_path.exists() {
        if let Some(parent) = file_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = File::create(&file_path);
    }

    let content = fs::read_to_string(&file_path).unwrap_or_default();

    let v: Value = serde_json::from_str(&content)
        .unwrap_or_else(|_| Value::Array(vec![]));


    
    let mut pipelist = Vec::new();

    if let Some(arr) = v.as_array() {
        for i in arr {
            let Some(sink) = i
                .get("sink")
                .and_then(|v| v.as_str())
                .map(Into::into)
            else {
                continue;
            };

            let Some(source) = i
                .get("source")
                .and_then(|v| v.as_str())
                .map(Into::into)
            else {
                continue;
            };

            let enabled = i
                .get("enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            let channel = i
                .get("channel")
                .and_then(|v| v.as_i64())
                .unwrap_or(1) as i32;

            pipelist.push(Pipe {
                channel,
                enabled,
                sink,
                source,
            });
        }
    }

    pipelist
}