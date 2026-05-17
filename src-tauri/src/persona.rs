

use serde::{Deserialize, Serialize};

use crate::error::AppResult;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    pub steam_id: String,
    pub persona_name: String,
    pub avatar_url: String,
    pub profile_url: String,
}

pub async fn fetch(steam_id: u64) -> AppResult<Persona> {
    let url = format!("https://steamcommunity.com/profiles/{steam_id}?xml=1");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| crate::error::AppError::other(format!("http client: {e}")))?;
    let body = client.get(&url).send().await
        .map_err(|e| crate::error::AppError::other(format!("persona fetch: {e}")))?
        .text().await
        .map_err(|e| crate::error::AppError::other(format!("persona body: {e}")))?;

    let persona_name = extract_cdata(&body, "steamID")
        .unwrap_or_else(|| format!("Steam User {steam_id}"));
    let avatar_url = extract_cdata(&body, "avatarFull")
        .or_else(|| extract_cdata(&body, "avatarMedium"))
        .unwrap_or_default();
    let profile_url = format!("https://steamcommunity.com/profiles/{steam_id}");

    Ok(Persona {
        steam_id: steam_id.to_string(),
        persona_name,
        avatar_url,
        profile_url,
    })
}

fn extract_cdata(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}><![CDATA[");
    let close = "]]>";
    let start = xml.find(&open)? + open.len();
    let rest = &xml[start..];
    let end = rest.find(close)?;
    Some(rest[..end].to_string())
}
