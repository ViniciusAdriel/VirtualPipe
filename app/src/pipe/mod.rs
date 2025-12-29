mod create;
mod remove;
use std::process::Command;
use anyhow::bail;
pub use create::new;
pub use remove::remove;
use crate::Pipe;

pub fn get_id(pipe: &Pipe)
-> anyhow::Result<(String, String)>
{
    // get modules list
    let modules_list = Command::new("pactl").args([
        "list", "modules", "short"
    ]).output()?;

    let stdout = String::from_utf8_lossy(&modules_list.stdout);
    let mut lines = stdout.lines().collect::<Vec<&str>>();
    lines.retain(
        |l|{
            let t = l.split('\t').collect::<Vec<&str>>();
            t.contains(&"module-null-sink")
        }
    );

    // Verifies if exists
    let mut output = (String::new(), String::new());

    for line in lines {
        let t = line.split('\t').collect::<Vec<&str>>();
        
        if let Some(sink_name) = t[2]
            .split_once("sink_name=")
            .and_then(|(_, rest)| rest.split_once(" sink_properties="))
            .map(|(m, _)| m)
        {
            if pipe.sink.as_str() == sink_name {
                output.0 = t[0].to_string();
            }

            if pipe.source.as_str() == sink_name {
                output.1 = t[0].to_string();
            }

            if !output.0.is_empty()
            && !output.1.is_empty() {
                return Ok(output);
            }
        }
    }

    bail!("Pipe not found.")
}