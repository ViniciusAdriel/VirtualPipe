// Imports

// Slint Imports - Importing UI
slint::slint!(
    export { MainWindow } from "app/ui/main.slint";
    // Go to path above to see more about UI.
);

fn main() -> anyhow::Result<()> {

    { // Main Window
        // Creates Main Window from slint imports.
        let main_window = MainWindow::new()?;

        main_window.run()?;
    }

    Ok(())
}
