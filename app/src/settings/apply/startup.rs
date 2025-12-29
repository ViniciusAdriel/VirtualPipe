use std::fs;

pub fn enable(
    enable: bool
) -> anyhow::Result<()>

{
    let autostart_path = dirs::config_dir().unwrap()
        .join("autostart")
        .join("net.viniadrii.VirtualPipe.desktop");

    let autostart_desktop_file = include_str!("./assets/net.viniadrii.VirtualPipe.desktop");

    if enable
    {
        fs::create_dir_all(autostart_path.parent().unwrap())?;
        fs::write(autostart_path, autostart_desktop_file)?;
    } else
    {
        if autostart_path.exists() {
            fs::remove_file(autostart_path)?;
        }
    }

    Ok(())
}