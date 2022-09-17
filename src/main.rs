use std::{io, io::Write, fs, fs::File, path::{Path, PathBuf}, cmp::min};
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;
use zip_extensions::*;
use directories::BaseDirs;
use fs_extra::dir::{move_dir, CopyOptions};

#[tokio::main]
async fn main() {
    let update: bool = Path::new("./mods").exists();
    let mcmodszip = if update == false {"./modpack/mcmods.zip"} else {"./mcmods.zip"};
    let mods_dir = if update == false {"./modpack/"} else {"./"};
    if update == false {fs::create_dir(&mods_dir).ok();}
    else {
        println!("Found existing modpack updating...");
        fs::remove_dir_all(format!("{}mods", mods_dir)).ok();
        fs::remove_dir_all(format!("{}versions", mods_dir)).ok();
        println!("Deleted old mods and installations")
    }

    download_file(&Client::new(), "https://drive.google.com/uc?export=download&id=1qa7gThngkqNooUweuyVs6Kes8w_pIJ0l&confirm=t", mcmodszip).await.unwrap();
    
    println!("\nExtracting archive...");
    let mczip = PathBuf::from(&mcmodszip);
    let extract_dir = PathBuf::from(&mods_dir);
    zip_extract(&mczip, &extract_dir).expect("Could not extract zip file");
    println!("Extracted archive!");

    if let Some(base_dirs) = BaseDirs::new() {
        let appdata = base_dirs.data_dir().to_str().expect("An error occured");
        let options = CopyOptions::new();
        move_dir(format!("{}versions", &mods_dir), format!(r"{}\.minecraft\", &appdata), &options).ok();
        println!(r"Sent versions to {}\.minecraft\versions", &appdata)
    }
    
    println!("Cleaning up");
    fs::remove_file(&mcmodszip).ok();
    fs::remove_dir_all(format!("{}versions", mods_dir)).ok();
        
    end()
}
//download fn
pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
        let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
    
        // Indicatif
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
        pb.set_message(&format!("Downloading to {}", path));
        
        let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();
        
        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(format!("Error while downloading file")))?;
            file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }
        
        pb.finish_with_message(&format!("Downloaded modpack to {}", path));
        return Ok(());
    }
    
fn end() {
    //i know this is stupid but its just to stop the code before exiting
    let mut end = String::new();
    println!("\nPress enter to exit");
    io::stdin().read_line(&mut end).ok();
    std::process::exit(0)
}