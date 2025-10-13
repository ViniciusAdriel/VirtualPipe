use std::process::Command;
use anyhow::{bail, Result};
use crate::Pipe;

pub fn create(pipe: Pipe)
-> Result<Pipe>
{
    if !pipe.sink.replace("_", "").chars().all(|c| c.is_alphanumeric())
    || !pipe.source.replace("_", "").chars().all(|c| c.is_alphanumeric()){
        bail!("Inputs can have only alphanumeric chars or underline!")
    }

    match get_id(pipe.clone()) {
        Ok(_) => {
            bail!("Sink or source already exists!")
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

    match pipe.channel.to_uppercase().as_str() {
        "MONO"|"1.0" => {
            load_module("mono", &pipe.sink, &pipe.source)?;
            Command::new("pw-link").args([
                &format!("{}:monitor_0", pipe.sink),
                &format!("{}:input_0", pipe.source)
            ]).output()?;
        },
        "STEREO"|"2.0" => {
            load_module("front-left,front-right", &pipe.sink, &pipe.source)?;
            for a in ["FL", "FR"] {
                Command::new("pw-link").args([
                    &format!("{}:monitor_{a}", pipe.sink),
                    &format!("{}:input_{a}", pipe.source)
                ]).output()?;
            }
        },
        _ => bail!("No such a type. Avaliable: 'MONO', 'STEREO'")
    }

    Ok(pipe)
}

pub fn remove(pipe: Pipe)
-> Result<()>
{

    let ids = match get_id(pipe.clone()) {
        Ok(o) => o,
        Err(e) => bail!(e)
    };

    Command::new("pw-link").args([
        "-d",
        &pipe.sink,
        &pipe.source
    ]).output()?;

    for id in [ids.0, ids.1] {
        Command::new("pactl").args([
            "unload-module",
            &id
        ]).output()?;
    }

    Ok(())
}

pub fn get_id(pipe: Pipe)
-> Result<(String, String)>
{
    let obj_lists = Command::new("pactl").args([
        "list", "modules", "short"
    ]).output()?;

    let mut output = (String::new(), String::new());

    for line in String::from_utf8_lossy(&obj_lists.stdout).lines() {
        let tabs = line.split('\t').collect::<Vec<&str>>();

        if tabs.contains(&"module-null-sink") {
            if line.contains(&format!("sink_name={} ", pipe.sink)) {
                output.0 = tabs[0].to_string()
            }
            if line.contains(&format!("sink_name={} ", pipe.source)) {
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