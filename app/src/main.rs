use std::rc::Rc;
use slint::{
    Model, VecModel
};
mod settings;
mod pipelist;
mod cli;
mod pipe;

slint::slint!(
    export { MainWindow } from "app/ui/main.slint";
    // Go to path above to see more about UI.
);


#[tokio::main]
async fn main() -> anyhow::Result<()>
{
    let args = cli::parse();

    let data_path = dirs::data_dir()
        .unwrap()
        .join("virtualpipe");
        
    let cnfg_path = dirs::config_dir()
        .unwrap()
        .join("virtualpipe");
    
    let pipelist = pipelist::load::from_file(data_path.join("pipelist.json"));
    let settings = settings::load::from_file(cnfg_path.join("settings.json"));

    // Restore pipes
    for pipe in &pipelist {
        if pipe.enabled {
            match pipe::get::id(pipe) {
                Ok(_) => (),
                Err(_) => {
                    // If fails to get id, so the pipe exists.
                    match pipe::new(pipe) {
                        Ok(_) => (),
                        Err(_) => ()
                    }
                }
            };
        }
    }

    if args.restore_only {
        return Ok(());
    }
    
    // Main Window //
    let main_window = MainWindow::new()?;

    main_window.set_settings(settings.clone());

    let pipelist = Rc::new(VecModel::from(pipelist));
    main_window.set_pipelist(pipelist.clone().into());

    
    // pipe related callbacks
    main_window.on_create_pipe({
        let pipelist = pipelist.clone();
        let main_window = main_window.as_weak();

        move ||{
            let pipelist = pipelist.clone();
            let main_window = main_window.upgrade().unwrap();

            // Define default pipe's name //
            let (default_sink_name, default_source_name) = (
                "VirtualSpeaker",
                "VirtualMicrophone"
            );
             
            let suffix =
                pipe::get::suffix(
                    &pipelist,
                    default_sink_name,
                    default_source_name
                ).to_string();
            
            let (sink_name, source_name) = (
                format!("{}{}", default_sink_name,   if suffix == "0" {""} else {&suffix}),
                format!("{}{}", default_source_name, if suffix == "0" {""} else {&suffix})
            );

            // Create pipe //
            let pipe = Pipe {
                channel: 1,
                enabled: true,
                sink: sink_name.into(),
                source: source_name.into()
            };
            
            match pipe::new(&pipe) {
                Ok(_) => {
                    pipelist.insert(0, pipe);

                    main_window.invoke_update_pipelist_file();
                    println!("Pipe Created");
                }
                Err(e) => {
                    println!("Failed to create pipe: {}", e);
                }
            }
        }
    });

    main_window.on_remove_pipe({
        let pipelist = pipelist.clone();
        let main_window = main_window.as_weak();

        move |pipe, index|{
            let pipelist = pipelist.clone();
            let main_window = main_window.upgrade().unwrap();
            
            // Remove pipe
            if pipe.enabled {
                match pipe::remove(&pipe) {
                    Ok(_) => {
                        pipelist.remove(index as usize);
                        
                        main_window.invoke_update_pipelist_file();
                    }
                    Err(e) => {
                        println!("Failed to remove pipe: {}", e);
                    }
                }
            } else {
                pipelist.remove(index as usize);
            }
            
            main_window.invoke_update_pipelist_file();
            println!("Pipe {} Removed", index);
        }
    });

    main_window.on_update_pipe({
        let main_window = main_window.as_weak();
        let pipelist = pipelist.clone();

        move |old_pipe, new_pipe, index|{
            let main_window = main_window.upgrade().unwrap();
            let pipelist = pipelist.clone();

            // Remove old pipe
            match pipe::remove(&old_pipe) {
                Ok(_) => (),
                Err(e) => {
                    pipelist.set_row_data(
                        index as usize,
                        old_pipe.clone()
                    );
                    eprintln!("Failed updating pipe (Deleting old pipe): {}", e);
                    return;
                }
            }

            // create new pipe
            match pipe::new(&new_pipe) {
                Ok(_) => {
                    pipelist.set_row_data(
                        index as usize,
                        new_pipe.clone()
                    );

                    println!("Pipe {} Updated", index);
                    main_window.invoke_update_pipelist_file();
                },
                Err(e) => {
                    eprintln!("Failed updating pipe (Creating new pipe): {}", e);

                    // If fails, restores old pipe
                    match pipe::new(&old_pipe) {
                        Ok(_) => {
                            pipelist.set_row_data(
                                index as usize,
                                old_pipe.clone()
                            );
                        }
                        Err(e) => {
                            eprintln!("Failed restoring pipe: {}", e);
                            main_window.invoke_update_pipelist_file();
                        }
                    };
                }
            }
        }
    });

    main_window.on_enable_pipe({
        let main_window = main_window.as_weak();

        move |enable, pipe, index|{
            let main_window = main_window.upgrade().unwrap();

            // Enable/Disable pipe
            if enable {
                let mut pipe = pipe;
                pipe.enabled = true;

                match pipe::new(&pipe) {
                    Ok(_) => {
                        println!("Pipe enabled.");
                    }
                    Err(e) => {
                        println!("Failed to enable pipe: {}", e);
                    }
                }
            } else {
                match pipe::remove(&pipe) {
                    Ok(_) => {
                        println!("Pipe {} disabled", index);
                    }
                    Err(e) => {
                        println!("Failed to disable pipe: {}", e);
                    }
                }
            }

            main_window.invoke_update_pipelist_file();
        }
    });
    
    // Restore pipes from pipelist.json
    main_window.on_update_pipelist_file({
        let pipelist = pipelist.clone();

        move || {
            let pipelist = pipelist.clone();

            // Register changes in pipelist file
            pipelist::save::to_file(data_path.join("pipelist.json"), pipelist)
                .unwrap_or_else(|e| {
                    eprintln!("Error saving pipelist state: {}", e);
                });
        }
    });

    main_window.invoke_update_pipelist_file();

    // Get settings from settings.json
    main_window.on_apply_settings({
        move |settings| {
            let settings = settings.clone();
            let cnfg_path = cnfg_path.clone();

            tokio::spawn(async move {
                match settings::apply(&settings).await {
                    Ok(_) => {
                        if let Err(e) = settings::save::to_file(
                            cnfg_path.join("settings.json"),
                            &settings,
                        ) {
                            eprintln!("Error saving settings: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error applying settings: {}", e);
                    }
                }
            });
        }
    });

    main_window.invoke_apply_settings(settings);

    // launches GUI
    main_window.run()?;

    Ok(())
}
