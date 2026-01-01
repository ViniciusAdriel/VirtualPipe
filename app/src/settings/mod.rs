pub mod load;
pub mod save;

use ashpd::desktop::background::Background;
use crate::Settings;

pub async fn apply(settings: &Settings) -> anyhow::Result<()>
{
    Background::request()
        .reason("Start automatically on login")
        .auto_start(settings.restore_on_system_startup)
        .command(["virtualpipe", "--restore-only"])
        .send()
        .await?;
    
    Ok(())
}
