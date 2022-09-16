use std::cmp::min;
use std::fs::File;
use std::io;
use std::io::Write;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;
use zip_extensions::*;


//todo - delete the mods folder if update = true
//install the versions into the mclauncher


#[tokio::main]
async fn main() {
    let update = Path::new("./mods").exists();
    let mcmodszip = if update == false {"./modpack/mcmods.zip"} else {"./mcmods.zip"};
    let mods_dir = if update == false {"./modpack"} else {"./"};
    if update == false {fs::create_dir(&mods_dir).ok();}

    download_file(&Client::new(), "https://drive.google.com/uc?export=download&id=1qa7gThngkqNooUweuyVs6Kes8w_pIJ0l&confirm=t", mcmodszip).await.unwrap();
    
    println!("\nExtracting archive...");
    let mczip = PathBuf::from(&mcmodszip);
    let extract_dir = PathBuf::from(&mods_dir);
    zip_extract(&mczip, &extract_dir).expect("Could not extract zip file");
    println!("Extracted archive!");
    
    //fs::copy(format!("{mods_dir}/versions") ,format!("{mods_dir}/config")).expect("failed to move file");
    
    println!("Cleaning up");
    fs::remove_file(mcmodszip).expect("File delete failed");
        
    end()    
}
//download fn
pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
        let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
    
        // Indicatif setup
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
        pb.set_message(&format!("Downloading to {}", path));
        
        // download chunks
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
    let mut end = String::new();
    println!("Press enter to exit");
    io::stdin().read_line(&mut end).ok();
}