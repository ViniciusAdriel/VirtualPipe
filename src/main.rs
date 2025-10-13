use std::rc::Rc;
use anyhow::bail;
use slint::{Model, VecModel};

mod pipe;

slint::slint!{
    export { MainWindow } from "gui/main_window/main.slint";
}

fn main()
-> anyhow::Result<()>
{
    let mw = MainWindow::new()?;

    let pipes = Rc::new(VecModel::from(vec![]));
    mw.set_pipes(pipes.clone().into());

    // Pipe related callbacks

    // create(Pipe, i32) -> SharedString
    //
    // If `Pipe.sink` is empty:
    //   Creates a generic stereo pipe with automatically generated names:
    //     * VirtualPipe_Speaker_x
    //     * VirtualPipe_Microphone_x
    //   Repeats until successful, then returns "".
    //
    // Otherwise
    //   Creates a pipe based on the provided `Pipe` object.
    //   On success, returns "". Else, returns "Failed to create pipe: {error}".
    //
    // The `i32` parameter defines the target index:
    //   * A negative value refers to "last index" behavior.
    //   * Any non-negative value is used directly as the insertion index.
    //
    mw.on_create({
        let pipes = pipes.clone();
        let mw = mw.as_weak();

        move |mut pipe, idx|
        {
            let mw = mw.upgrade().unwrap();

            let fix_flag = pipe.sink.is_empty();
            let mut i = 0;

            loop {
                match pipe::create(pipe.clone()) {
                    Ok(p) => {
                        if idx < 0 {
                            pipes.push(p);
                        } else {
                            pipes.insert(idx as usize, p);
                        }
                        
                        mw.invoke_set_status(format!(
                            "Pipe created: '{} → {}' ({})",
                            pipe.sink, pipe.source, pipe.channel
                        ).into());

                        return "".into();
                    }
                    Err(e) => {

                        if !fix_flag {
                            mw.invoke_set_status(format!(
                            "Failed to create pipe: '{} → {}' ({}).\n{e}",
                            pipe.sink, pipe.source, pipe.channel
                        ).into());
                            return format!("Failed to create pipe: {e}").into();
                        }

                        let incr = if i > 0 {format!("_{}",i)} else {"".to_string()};
                        pipe = Pipe {
                               sink: format!("virtual_speaker{incr}").into(),
                             source: format!("virtual_mic{incr}").into(),
                            channel: "Stereo".into(),
                        };
                        i += 1;
                    }
                }   
            }
        }
    });

    // remove(Pipe) -> SharedString
    //
    // Deletes the specified pipe and removes it from the list.
    // On success, returns "". Else, returns "Failed to remove pipe: {error}".
    //
    mw.on_remove({
        let pipes = pipes.clone();
        let mw = mw.as_weak();

        move |pipe|
        {
            let mw = mw.upgrade().unwrap();

            match get_pipe_index(pipes.clone(), pipe.clone()) {
                Ok(i) => {
                    match pipe::remove(pipe.clone()) {
                        Ok(_) => {
                            pipes.remove(i);
                        },
                        Err(e) => {
                            return format!("Failed to delete pipe: {e}").into();
                        }
                    };

                    mw.set_page(0);
                    mw.invoke_set_status(format!(
                        "Pipe removed: '{} → {}' ({})",
                        pipe.sink, pipe.source, pipe.channel
                    ).into());
                    return "".into();
                },
                Err(e) => {
                    mw.invoke_set_status(format!(
                        "Failed to remove pipe: '{} → {}' ({})\n{e}",
                        pipe.sink, pipe.source, pipe.channel
                    ).into());
                    return format!("Failed to delete pipe: {e}").into();
                }
            }
        }
    });

    // change(Pipe1, Pipe2) -> SharedString
    //
    // Replaces `Pipe1` with `Pipe2`.
    // (Internally, it deletes `Pipe1` and inserts `Pipe2` in its place.)
    // On success, returns "". Else, returns "Failed to modify pipe: {error}".
    //
    mw.on_change({
        let pipes = pipes.clone();
        let mw = mw.as_weak();

        move |pipe, new_pipe|
        {
            let mw = mw.upgrade().unwrap();

            let idx = match get_pipe_index(pipes.clone(), pipe.clone()) {
                Ok(idx) => idx,
                Err(e) => return format!("Failed to modify pipe: {e}").into(),
            };

            let e = mw.invoke_create(new_pipe.clone(), idx as i32);
            if e != "" {
                return format!("Failed to modify pipe: {e}").into();
            }

            let e = mw.invoke_remove(pipe.clone());
            if e != "" {
                return format!("Failed to modify pipe: {e}").into();
            }

            mw.invoke_set_status(format!(
                "Modified pipe: '{} → {}' ({})\nto: '{} → {}' ({})",
                pipe.sink, pipe.source, pipe.channel,
                new_pipe.sink, new_pipe.source, new_pipe.channel
            ).into());
            mw.set_page(0);
            "".into()
        }
    });

    // Misc. callbacks

    mw.on_set_status({
        let mw = mw.as_weak();
        move |text| {
            let mw = mw.upgrade().unwrap();

            println!("{text}");

            let mw = mw.as_weak();
            slint::spawn_local(async move {
                let mw = mw.upgrade().unwrap();

                // Set Status
                mw.set_status(text.clone());

                // Close after inativity
                slint::Timer::single_shot(std::time::Duration::from_secs_f32(2.5), move || {
                    if mw.get_status() == text {
                        mw.set_status("".into());
                    }
                });
            }).unwrap();
        }
    });

    mw.on_fix_text(move |text| {
        let mut text = text.replace(" ", "_");
        text.retain(|c| c.is_alphanumeric() || c == '_');

        text.into()
    });

    mw.run()?;

    Ok(())
}

fn get_pipe_index(pipes: Rc<VecModel<Pipe>>, pipe: Pipe)
-> anyhow::Result<usize> {
    for i in 0..pipes.row_count() {
        if let Some(p) = pipes.row_data(i) {
            if p == pipe {
                return Ok(i);
            }
        }
    }

    bail!("Pipe not listed.")
}