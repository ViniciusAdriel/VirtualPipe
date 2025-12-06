use std::rc::Rc;
use slint::{Model, VecModel};
mod load_pipelist;
mod save_pipelist;
use clap::{Arg, ArgAction, Command};

slint::slint!(
    export { MainWindow } from "app/ui/main.slint";
    // Go to path above to see more about UI.
);



fn main() -> anyhow::Result<()>
{
    let matches = Command::new("myapp")
        .arg(
            Arg::new("restore-only")
                .long("restore-only").short('r')
                .help("Restore pipes and exit")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Main Window //
    let main_window = MainWindow::new()?;

    // load pipelist
    let data_path = dirs::data_dir()
        .unwrap()
        .join("virtualpipe");
    
    let pipelist_path = data_path.join("pipelist.json");

    let pipelist = load_pipelist::from_file(pipelist_path.clone());

    let pipelist = Rc::new(VecModel::from(pipelist));
    main_window.set_pipelist(pipelist.clone().into());

    // pipe related callbacks
    main_window.on_create_pipe({
        let main_window = main_window.as_weak();

        move ||{
            let main_window = main_window.upgrade().unwrap();
            // Create pipe
            
            main_window.invoke_update_pipelist_file();
            println!("Pipe Created");
        }
    });

    main_window.on_remove_pipe({
        let pipelist = pipelist.clone();
        let main_window = main_window.as_weak();

        move |pipe_idx|{
            let pipelist = pipelist.clone();
            let main_window = main_window.upgrade().unwrap();

            // Remove pipe from pipelist
            for i in 0..pipelist.row_count() {
                if let Some(pipe) = pipelist.row_data(i){
                    if pipe.idx == pipe_idx {
                        pipelist.remove(i);
                    }
                }
            }

            main_window.invoke_update_pipelist_file();
        }
    });

    main_window.on_update_pipe({
        let main_window = main_window.as_weak();

        move |new_pipe|{
            let main_window = main_window.upgrade().unwrap();
            // Update pipe
            
            main_window.invoke_update_pipelist_file();
            println!(
                "Pipe {} Updated",
                new_pipe.idx
            );
        }
    });

    main_window.on_enable_pipe({
        let main_window = main_window.as_weak();

        move |enable, pipe_idx|{
            let main_window = main_window.upgrade().unwrap();
            // Enable/Disable pipe

            main_window.invoke_update_pipelist_file();
            println!(
                "Pipe {} {}",
                pipe_idx,
                if enable {"Enabled"} else {"Disabled"}
            );
        }
    });

    // Save/load state related callbacks
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

    // Restore pipes from pipelist //
    
    // launches GUI //
    if !matches.get_flag("restore-only") {
        main_window.run()?;
    }

    Ok(())
}
