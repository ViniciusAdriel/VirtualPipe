use std::process::Command;
use crate::{Pipe, pipe};

pub fn remove(
    pipe: &Pipe
)-> anyhow::Result<()>
{
    let ids = pipe::get::id(pipe)?;

    // Unlink sinks
    Command::new("pw-link").args([
        "-d", &pipe.sink, &pipe.source
    ]).output()?;

    // Remove Sinks
    for id in [ids.0, ids.1] {
        Command::new("pactl").args([
            "unload-module", &id
        ]).output()?;
    }
    
    Ok(())
}