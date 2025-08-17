use std::{env, path::Path};

use colored::Colorize;
use tokio::io::AsyncWriteExt;
use tokio::{fs::File, join};
use viuer::{print_from_file, Config};

use crate::smegameme::get_random_meme;

mod smegameme;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let meme = get_random_meme().await?;
    let temp_file = env::temp_dir().join("smeg.jpg");
    let frame = download(&temp_file, &meme.frame_url).await;

    println!();
    if frame.is_ok() {
        let conf = Config {
            absolute_offset: false,
            width: Some(55),
            ..Default::default()
        };
        if print_from_file(&temp_file, &conf).is_err() {
            println!("Image printing failed.");
        }
        println!();
    }

    println!("{}", meme.quote.bold().green());
    println!();

    println!(
        "{} - {}",
        meme.series.replace("Series", "Red Dwarf").red(),
        meme.episode.purple(),
    );

    println!("{}", meme.memelaburl.color("blue").underline());
    println!();

    Ok(())
}

async fn download(
    filename: impl AsRef<Path>,
    url: &str,
) -> Result<File, Box<dyn std::error::Error>> {
    let (response, frame) = join!(reqwest::get(url), File::create(filename));
    let mut response = response?;
    let mut frame = frame?;

    while let Some(chunk) = response.chunk().await? {
        frame.write_all(&chunk).await?;
    }

    Ok(frame)
}

// -s, --series      1 to 14
// -e, --episode     1 to 8 with series, 1 to 74 without
