use std::process::Command;
use crate::{Pipe, pipe::get_id};

pub fn remove_pipe(
    pipe: &Pipe
)-> anyhow::Result<()>
{
    let ids = get_id(pipe)?;

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