use bytes::Bytes;
use clap::Parser;
use reqwest;
use serde::Deserialize;
use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize)]
struct WaifuResponse {
    url: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    for i in 1..=args.count {
        let response = reqwest::get("https://api.waifu.pics/sfw/waifu")
            .await?
            .json::<WaifuResponse>()
            .await?;
        println!("caught waifu: {}", response.url);

        let filename = format!("waifu#{}", i);

        let img_bytes: Bytes = reqwest::get(response.url).await?.bytes().await?;
        let mut file = File::create(filename.clone()).await?;
        file.write_all(&img_bytes).await?;

        Command::new("chafa")
            .args(&["--size=100x100", &filename])
            .status()?;

        println!("bagged waifu: {}", filename);
        println!("");
    }

    Ok(())
}
