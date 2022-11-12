mod modpacks;
use {
    directories::BaseDirs,
    fs_extra::{move_items, dir::CopyOptions},
    futures_util::StreamExt,
    indicatif::{ProgressBar, ProgressStyle},
    reqwest::Client,
    std::{
        cmp::min,
        env, fs,
        fs::File,
        io,
        io::Write,
        path::{Path, PathBuf},
    },
    zip_extensions::*,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let downloadurl = if args.get(1) == None {
        modpacks::pick_modpack().await
    } else if args[1] == "gameupdate" {
        "https://drive.google.com/uc?export=download&id=1qa7gThngkqNooUweuyVs6Kes8w_pIJ0l&confirm=t"
    } else {
        println!("Argument not found\nInvalid argument: {}", args[1]); end(); ""
    };
    let update: bool = Path::new("./mods").exists();
    let emptydir: bool = if update == false && fs::read_dir("./").unwrap().count() == 1 {true} else {false};
    let mcmodszip = if update == false && emptydir == false {"./modpack/mcmods.zip"} else {"./mcmods.zip"};
    let mods_dir = if update == false && emptydir == false { "./modpack/" } else { "./" };
    if update == false {
        if emptydir == false {fs::create_dir(&mods_dir).ok();}
    } else {
        println!("Found existing modpack updating...");
        fs::remove_dir_all(format!("{}mods", mods_dir)).unwrap_or_else(|_e| {
            println!("Unable to delete old mods this might be because you are running the game.");
            modpacks::tryagain(mods_dir)
        });
        fs::remove_dir_all(format!("{}versions", mods_dir)).ok();

        println!("Deleted old mods and installations")
    };

    download_file(&Client::new(), downloadurl, mcmodszip).await.unwrap();

    println!("\nExtracting archive...");
    let mczip = PathBuf::from(&mcmodszip);
    let extract_dir = PathBuf::from(&mods_dir);
    zip_extract(&mczip, &extract_dir).expect("Could not extract zip file");
    println!("Extracted archive!");

    if let Some(base_dirs) = BaseDirs::new() {
        let appdata = base_dirs.data_dir().to_str().expect("An error occured");
        let options = CopyOptions { overwrite: false, skip_exist: true, buffer_size: 64000, copy_inside: false, content_only: false, depth: 0 };
        move_items(
            &[format!("{}versions", &mods_dir)],
            format!(r"{}\.minecraft", &appdata),
            &options,
        ).expect("Couldnt do shit");
        println!(r"Sent versions to {}\.minecraft\versions", &appdata)
    };

    println!("Cleaning up");
    fs::remove_file(&mcmodszip).ok();
    fs::remove_dir_all(format!("{}versions", mods_dir)).ok();

    if update == false {
        modpacks::helpmsg();
    }

    end()
}
async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    println!("Waiting for response...");
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

pub fn end() {
    //i know this is stupid but its just to stop the code before exiting
    let mut end = String::new();
    println!("\nAll done! Press enter to exit");
    io::stdin().read_line(&mut end).ok();
    std::process::exit(0)
}
