use {
    crate::end,
    dialoguer::{theme::ColorfulTheme, Confirm, Select},
    std::fs,
};

pub async fn pick_modpack() -> &'static str {
    let modpacks = &["1.18.2 fabric modpack", "1.19.2 preformance modpack"];

    let picked_modpack = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick modpack to install:")
        .default(0)
        .items(&modpacks[..])
        .interact()
        .unwrap();

    let downloadurl = if modpacks[picked_modpack] == "1.18.2 fabric modpack" {
        "https://drive.google.com/uc?export=download&id=1qa7gThngkqNooUweuyVs6Kes8w_pIJ0l&confirm=t"
    } else if modpacks[picked_modpack] == "1.19.2 preformance modpack" {
        "https://drive.google.com/uc?export=download&id=1ga7ySowdBsR0QxxEyQbEaKWF95JhERXe&confirm=t"
    } else {
        ""
    };

    downloadurl
}

pub fn helpmsg() {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Show install help?")
        .interact()
        .unwrap()
    {
        println!("\nInstall help:");
        println!("\nCreate a new installation in your minecraft launcher\nwith the path of where you installed the modpack");
        println!("Then find the version corresponding to what modpack you installed\nfor example fabric 1.18.2 will need \nsomething similar to: \"fabric-loader-0.14.9-1.18.2\"");
        println!("Your loader version has alredy been installed to your minecraft launcher");
        println!("\nAdditionally it is recommended to give the game between 4-10gb of ram in the advanced options");
    }
}

pub fn tryagain(mods_dir: &str) {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Try again?")
        .interact()
        .unwrap()
    {
        fs::remove_dir_all(format!("{}mods", mods_dir)).unwrap_or_else(|error| {
            println!("Failed again here is your error:\n{}", error);
            end();
        })
    } else {
        std::process::exit(0)
    }
}
