use anyhow::Result;
use config::Config;
use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Deserialize)]
struct Settings {
    api_key: String,
    api_url: String,
    api_account_key: String,
}

fn load_settings() -> Result<Settings, config::ConfigError> {
    dotenvy::dotenv().ok();

    Config::builder()
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize()
}

pub async fn busstop_request<T: DeserializeOwned>(busstopcode: &str) -> Result<T> {
    let settings = load_settings()?;
    println!("{}", settings.api_key);
    println!("{}", settings.api_url);
    println!("{}", settings.api_account_key);

    let client = reqwest::Client::new();
    let rsp = client
        .get(&format!("{}/BusArrival", settings.api_url))
        .query(&[("BusStopCode", busstopcode)])
        .header("AccountKey", settings.api_account_key)
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?
        .error_for_status()?;
    Ok(rsp.json::<T>().await?)
}
