slint::include_modules!();

use std::time::Duration;

use slint::spawn_local;

mod pipe;

fn main()
-> anyhow::Result<()>
{
    let mw = MainWindow::new()?;

    // PIPE CALLBACKS

    mw.on_create_pipe({
        let mw = mw.as_weak();
        move|pipe_type, sink_name, source_name|{
            let mw = mw.upgrade().unwrap();
            match pipe::create(pipe_type.as_str(), sink_name.as_str(), source_name.as_str()) {
                Ok(_)  => {
                    mw.invoke_set_status("Pipe created.".into());
                },
                Err(e) => {
                    mw.invoke_set_status(e.to_string().into());
                }
            }
        }
    });

    mw.on_remove_pipe({
        let mw = mw.as_weak();
        move|sink_name, source_name|{
            let mw = mw.upgrade().unwrap();
            
            match pipe::remove(sink_name.as_str(), source_name.as_str()) {
                Ok(_)  => {
                    mw.invoke_set_status("Pipe removed.".into());
                    return true;
                },
                Err(e) => {
                    mw.invoke_set_status(e.to_string().into());
                    return false;
                }
            }
        }
    });

    // MISC CALLBACKS

    mw.on_fix_text(move |text| {
        let mut text = text.replace(" ", "_");
        text.retain(|c| c.is_alphanumeric() || c == '_');

        text.into()
    });

    mw.on_set_status({
        let mw = mw.as_weak();
        move |text| {
            let mw = mw.upgrade().unwrap();

            let mw = mw.as_weak();
            spawn_local(async move {
                let mw = mw.upgrade().unwrap();
                mw.set_status(text.clone());
                slint::Timer::single_shot(Duration::from_secs_f32(2.0), move || {
                    if mw.get_status() == text {
                        mw.set_status("".into());
                    }
                });
            }).unwrap();
        }
    });

    mw.run()?;

    Ok(())
}