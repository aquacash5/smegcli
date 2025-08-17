use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Smegameme {
    pub series: String,
    pub episode: String,
    pub quote: String,
    pub frame_url: String,
    pub memelaburl: String,
}

#[derive(Debug, Deserialize)]
struct SmegamemeResult {
    pub response: Smegameme,
}

pub async fn get_random_meme() -> Result<Smegameme, Box<dyn std::error::Error>> {
    Ok(reqwest::get("https://smegadrive.ganymede.tv/api/random")
        .await?
        .json::<SmegamemeResult>()
        .await?
        .response)
}
