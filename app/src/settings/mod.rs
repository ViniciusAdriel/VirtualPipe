pub mod load;
pub mod save;



use crate::Settings;
mod apply;

pub fn apply(
    settings: &Settings
) -> anyhow::Result<()>

{
    apply::startup::enable(settings.restore_on_system_startup)?;

    Ok(())
}