use std::process::Command;
use anyhow::bail;
use crate::Pipe;

fn load_module(
    channel_map: &str,
    sink_name:&str,
    source_name:&str
) -> anyhow::Result<()>
{
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

pub fn new(
    pipe: &Pipe
) -> anyhow::Result<()>
{
    match pipe.channel {
        0 => {
            load_module("mono", &pipe.sink, &pipe.source)?;
            Command::new("pw-link").args([
                &format!("{}:monitor_0", pipe.sink),
                &format!("{}:input_0", pipe.source)
            ]).output()?;
        }
        1 => {
            load_module("front-left,front-right", &pipe.sink, &pipe.source)?;
            for a in ["FL", "FR"] {
                Command::new("pw-link").args([
                    &format!("{}:monitor_{a}", pipe.sink),
                    &format!("{}:input_{a}", pipe.source)
                ]).output()?;
            }
        }
        2 => bail!("Not implemented yet.\nAvailable: 'MONO', 'STEREO'"),
        _ => bail!("Channel type unavailable.\nAvailable: 'MONO', 'STEREO'")
    }
    
    Ok(())
}