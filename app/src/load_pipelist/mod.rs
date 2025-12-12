use std::{fs::{File, create_dir_all, read_to_string}, path::PathBuf, vec};
use serde_json::Value;
use crate::Pipe;
mod get;



pub fn from_file(file_path: PathBuf)
-> Vec<Pipe>
{
    // Create file if does not exists //
    if !file_path.exists() {
        if let Some(parent) = file_path.parent() {
            let _ = create_dir_all(parent);
        }
        let _ = File::create(&file_path);
    }

    // Read pipelist //
    let pipelist_json: Vec<Value> = match serde_json::from_str(
        &read_to_string(&file_path).unwrap_or_default()
    ) {
        Ok(o) => o,
        Err(e) => {
            println!(
                "Error reading pipelist, {e} in {:?}",
                file_path
            );
            vec![]
        }
    };

    let mut pipelist = vec![];

    for i in pipelist_json {
        let sink = match get::string(&i, "sink") {
            Some(s) => s,
            None => continue,
        };

        let source = match get::string(&i, "source") {
            Some(s) => s,
            None => continue,
        };

        let enabled = get::bool_or(&i, "enabled", true);        

        let idx = get::i32(&i, "idx")
            .unwrap_or_else(||{
                let mut candidate = 0;
                while pipelist.iter().any(|p: &Pipe|p.idx == candidate) {
                    candidate += 1;
                }
                candidate
            });

        let channel = get::i32(&i, "channel")
            .unwrap_or(1);
        
        pipelist.push(Pipe { channel, enabled, idx, sink, source });

    }

    return pipelist;
}