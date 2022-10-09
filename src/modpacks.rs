use dialoguer::{theme::ColorfulTheme, Select, Confirm};

pub async fn pick_modpack() -> &'static str {
    let modpacks = &[
        "1.18.2 fabric modpack",
        "1.19.2 preformance modpack",
    ];

    let picked_modpack = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick modpack to install:")
        .default(0)
        .items(&modpacks[..])
        .interact()
        .unwrap();

    let downloadurl = 
    if modpacks[picked_modpack] == "1.18.2 fabric modpack" {"https://drive.google.com/uc?export=download&id=1qa7gThngkqNooUweuyVs6Kes8w_pIJ0l&confirm=t"} 
    else if modpacks[picked_modpack] == "1.19.2 preformance modpack" {"https://drive.google.com/uc?export=download&id=1ga7ySowdBsR0QxxEyQbEaKWF95JhERXe&confirm=t"} 
    else {""};

    downloadurl
}

pub fn helpmsg(update: bool) {
    if update == false {
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
        }}
}