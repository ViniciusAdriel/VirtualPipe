// Imports

// Slint Imports - Importing UI
slint::slint!(
    export { MainWindow } from "app/ui/main.slint";
    // Go to path above to see more about UI.
);

fn main() -> anyhow::Result<()>
{
    // Main Window //

    // Creates Main Window from slint imports.
    let main_window = MainWindow::new()?;

    // pipe related callbacks
    main_window.on_create_pipe({
        ||{
            // Create pipe
            
            // Register pipe
            println!("Pipe Created");
        }
    });
    main_window.on_remove_pipe({
        |pipe_idx|{
            // Remove pipe

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
            println!("Pipe Disabled");
        }
    });
    
    // Main loop
    main_window.run()?;

    Ok(())
}
