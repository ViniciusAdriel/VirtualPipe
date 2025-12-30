use std::{process::Command, rc::Rc};
use anyhow::bail;
use slint::{VecModel, Model};
use crate::Pipe;

pub fn suffix(
    pipelist: &Rc<VecModel<Pipe>>,
    default_sink_name: &str,
    default_source_name: &str
) -> i32
{
    let mut suffix = 0;

    loop {
        let s = suffix.to_string();

        let sink_name_candidate   = format!(
            "{default_sink_name}{}", if suffix == 0 {""} else {&s}
        );
        let source_name_candidate = format!(
            "{default_source_name}{}", if suffix == 0 {""} else {&s}
        );

        if !(0..pipelist.row_count())
            .filter_map(|i| pipelist.row_data(i))
            .any(|pipe| pipe.sink == sink_name_candidate)
        || !(0..pipelist.row_count())
            .filter_map(|i| pipelist.row_data(i))
            .any(|pipe| pipe.source == source_name_candidate)
        {
            return suffix;
        } else {
            suffix += 1;
            continue;
        }
    }
}

pub fn id(pipe: &Pipe)
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