use std::rc::Rc;
use slint::{Model, VecModel};
mod load_pipelist;

slint::slint!(
    export { MainWindow } from "app/ui/main.slint";
    // Go to path above to see more about UI.
);



fn main() -> anyhow::Result<()>
{
    // Main Window //

    let main_window = MainWindow::new()?;

    // load pipelist
    let data_path = dirs::data_dir()
        .unwrap()
        .join("virtualpipe");

    let pipelist = load_pipelist::from_file(
        data_path.join("pipelist.json")
    );

    let pipelist = Rc::new(VecModel::from(pipelist));
    main_window.set_pipelist(pipelist.clone().into());

    // pipe related callbacks

    main_window.on_create_pipe({
        ||{
            // Create pipe
            
            // Register pipe
            println!("Pipe Created");
        }
    });
    main_window.on_remove_pipe({
        let pipelist = pipelist.clone();
        move |pipe_idx|{
            let pipelist = pipelist.clone();
            // Remove pipe
            for i in 0..pipelist.row_count() {
                if let Some(pipe) = pipelist.row_data(i){
                    if pipe.idx == pipe_idx {
                        pipelist.remove(i);
                    }
                }
            }
            // Deregister the pipe.
            println!("Pipe Removed");
        }
    });

    main_window.on_update_pipe({
        |new_pipe|{
            // Update pipe
            
            // The pipe registry is fully changed in the UI.
            // See in /app/ui/00.MainPage/components/Card.slint
            // In the 'Pipe Tasks' section.
            println!("Pipe Updated");
        }
    });
    main_window.on_enable_pipe({
        |enable, pipe_idx|{
            // Enable/Disable pipe

            // The pipe registry is fully changed in the UI.
            // See in /app/ui/00.MainPage/components/Card.slint
            // In the 'Pipe Tasks' section.
            println!("Pipe Enabled");
        }
    });
    
    // Main loop
    main_window.run()?;

    Ok(())
}
