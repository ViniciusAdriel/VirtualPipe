use std::rc::Rc;
use slint::{Model, VecModel};
mod load_pipelist;
mod save_pipelist;
mod pipelist;
mod cli;
mod pipe;

slint::slint!(
    export { MainWindow } from "app/ui/main.slint";
    // Go to path above to see more about UI.
);



fn main() -> anyhow::Result<()>
{
    let data_path = dirs::data_dir()
        .unwrap()
        .join("virtualpipe");
    let pipelist_path = data_path.join("pipelist.json");
    let args = cli::parse();
    
    // load pipelist
    let pipelist = load_pipelist::from_file(pipelist_path.clone());
    
    // Main Window //
    let main_window = MainWindow::new()?;

    // Give pipelist to slint
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
                pipelist::get_suffix(
                    &pipelist,
                    default_sink_name,
                    default_source_name
                );
            
            let (sink_name, source_name) = (
                format!("{}{}", default_sink_name, suffix),
                format!("{}{}", default_source_name, suffix)
            );

            // Create pipe //
            let pipe = Pipe {
                channel: 1,
                enabled: true,
                idx: -1,
                sink: sink_name.into(),
                source: source_name.into()
            };
            
            match pipe::create_new_pipe(&pipe) {
                Ok(_) => {
                    pipelist.push(
                        pipe
                    );

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

        move |pipe|{
            let pipelist = pipelist.clone();
            let main_window = main_window.upgrade().unwrap();
            
            // Remove pipe
            if pipe.enabled {
                match pipe::remove_pipe(&pipe) {
                    Ok(_) => {
                        pipelist.remove((pipe.idx - 1) as usize);
                        
                        main_window.invoke_update_pipelist_file();
                    }
                    Err(e) => {
                        println!("Failed to remove pipe: {}", e);
                    }
                }
            } else {
                pipelist.remove((pipe.idx - 1) as usize);
            }
            
            println!("Pipe {} Removed", pipe.idx);
        }
    });

    main_window.on_update_pipe({
        let main_window = main_window.as_weak();

        move |old_pipe, new_pipe|{
            let main_window = main_window.upgrade().unwrap();

            // Remove old pipe
            let removed = if old_pipe.enabled {
                match pipe::remove_pipe(&old_pipe) {
                    Ok(_) => true,
                    Err(e) => {
                        println!("Failed to update pipe: {}", e);
                        false
                    }
                }
            } else {
                true
            };

            // create new pipe
            if removed {
                match pipe::create_new_pipe(&new_pipe) {
                    Ok(_) => {
                        println!("Pipe updated");
                    }
                    Err(e) => {
                        println!("Failed to update pipe: {}", e);
                    }
                }
            }

            main_window.invoke_update_pipelist_file();
            println!("Pipe {} Updated", old_pipe.idx);
        }
    });

    main_window.on_enable_pipe({
        let main_window = main_window.as_weak();

        move |enable, pipe|{
            let main_window = main_window.upgrade().unwrap();

            // Enable/Disable pipe
            if enable {
                let mut pipe = pipe;
                pipe.enabled = true;

                match pipe::create_new_pipe(&pipe) {
                    Ok(_) => {
                        println!("Pipe enabled.");
                    }
                    Err(e) => {
                        println!("Failed to enable pipe: {}", e);
                    }
                }
            } else {
                match pipe::remove_pipe(&pipe) {
                    Ok(_) => {
                        println!("Pipe {} disabled", pipe.idx);
                    }
                    Err(e) => {
                        println!("Failed to disable pipe: {}", e);
                    }
                }
            }

            main_window.invoke_update_pipelist_file();
        }
    });

    

    // Restore pipes from pipelist //
    main_window.on_update_pipelist_file({
        let pipelist = pipelist.clone();

        move || {
            let pipelist = pipelist.clone();

            // Order pipelist by pipe idx
            let mut v: Vec<Pipe> = pipelist.iter().collect();
            v.sort_by_key(|p| p.idx);

            for (i, p) in v.iter_mut().enumerate() {
                p.idx = (i + 1) as i32;
            }

            pipelist.set_vec(v);

            // Register changes in pipelist file
            match save_pipelist::update_file(pipelist_path.clone(), pipelist) {
                Err(e) => {
                    eprintln!("Error saving pipelist state: {}", e)
                },
                Ok(_) => ()
            };
        }
    });

    main_window.invoke_update_pipelist_file();
    
    // launches GUI //
    if !args.restore_only {
        main_window.run()?;
    }

    Ok(())
}
