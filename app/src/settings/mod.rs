pub mod load;
pub mod save;

use ashpd::desktop::background::Background;
use slint::SharedString;
use crate::Settings;

pub async fn apply(settings: &Settings) -> anyhow::Result<String>
{
    Background::request()
        .reason("Start automatically on login")
        .auto_start(settings.restore_on_system_startup)
        .command(["virtualpipe", "--restore-only"])
        .send()
        .await?;

    // Apply Language
    let languages = [
        "PT-BR", "EN"
    ];

    let lang = if settings.app_language == SharedString::from("") {

        let default_locale = sys_locale::get_locale()
            .unwrap_or_else(|| "EN".into())
            .replace("_", "-")
            .to_uppercase();

        let a = if let Some(i) = languages.iter().position(|&x|{
            x == default_locale
        }) {
            languages[i]
        } else {
            let default_locale = default_locale
                .split(['-', '_']).next()
                .unwrap_or("EN");
            let a = if let Some(i) = languages.iter().position(|&x|{
                x == default_locale
            }) {
                languages[i]
            } else {
                eprintln!("Automatic language detection failed. Falling back to English (EN).");
                "EN"
            };

            a
        };

        a
    } else {
        settings.app_language.as_str()
    };

    Ok(match lang {
        "PT-BR" => "pt",
        "EN"    => "en",
        _       => "en"
    }.to_string())
}
