pub mod load;
pub mod save;

use crate::Settings;

pub fn apply(settings: &Settings) -> anyhow::Result<()>
{
    if settings.restore_on_system_startup {
        
    } else {
        
    }

    Ok(())
}
