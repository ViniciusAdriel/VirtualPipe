use std::process::Command;
use anyhow::{bail, Result};

pub fn create(
    pipe_type:&str,
    sink_name:&str, source_name:&str
) -> Result<()>
{
    if !sink_name.replace("_", "").chars().all(|c| c.is_alphanumeric())
    || !source_name.replace("_", "").chars().all(|c| c.is_alphanumeric()){
        bail!("Inputs can have only alphanumeric chars or underline!")
    }

    match get_id(sink_name, source_name) {
        Ok(_) => {
            bail!("Source or speaker already exists!")
        },
        Err(_) => ()
    }

    fn load_module(channel_map: &str, sink_name:&str, source_name:&str) -> Result<()> {
        for a in [
            (sink_name, "Audio/Sink"),
            (source_name, "Audio/Source/Virtual")
        ] {
            Command::new("pactl").args([
                "load-module",
                "module-null-sink",
                &format!("sink_name={}", a.0),
                &format!("sink_properties=device.description={}", a.0),
                &format!("media.class={}", a.1),
                &format!("channel_map={}", channel_map)
            ]).output()?;
        }

        Ok(())
    }

    match pipe_type.to_uppercase().as_str() {
        "MONO"|"1.0" => {
            load_module("mono", sink_name, source_name)?;
            Command::new("pw-link").args([
                &format!("{sink_name}:monitor_0"),
                &format!("{source_name}:input_0")
            ]).output()?;
        },
        "STEREO"|"2.0" => {
            load_module("front-left,front-right", sink_name, source_name)?;
            for a in ["FL", "FR"] {
                Command::new("pw-link").args([
                    &format!("{sink_name}:monitor_{a}"),
                    &format!("{source_name}:input_{a}")
                ]).output()?;
            }
        },
        _ => bail!("No such a type. Avaliable: 'MONO', 'STEREO'")
    }

    Ok(())
}

pub fn remove(
    sink_name:&str, source_name:&str
) -> Result<()>
{

    let ids = match get_id(sink_name, source_name) {
        Ok(o) => o,
        Err(e) => bail!(e)
    };

    Command::new("pw-link").args([
        "-d",
        sink_name,
        source_name
    ]).output()?;

    for id in [ids.0, ids.1] {
        Command::new("pactl").args([
            "unload-module",
            &id
        ]).output()?;
    }

    Ok(())
}

pub fn get_id(
    sink_name:&str, source_name:&str
) -> Result<(String, String)>
{
    let obj_lists = Command::new("pactl").args([
        "list", "modules", "short"
    ]).output()?;

    let mut output = (String::new(), String::new());

    for line in String::from_utf8_lossy(&obj_lists.stdout).lines() {
        let tabs = line.split('\t').collect::<Vec<&str>>();

        if tabs.contains(&"module-null-sink") {
            if line.contains(&format!("sink_name={sink_name} ")) {
                output.0 = tabs[0].to_string()
            }
            if line.contains(&format!("sink_name={source_name} ")) {
                output.1 = tabs[0].to_string()
            }

            if output.0 != ""
            && output.1 != "" {
                return Ok(output);
            }
        }
    }

    bail!("Pipe not found.")
}