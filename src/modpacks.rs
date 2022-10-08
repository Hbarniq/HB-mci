use dialoguer::{theme::ColorfulTheme, Select};

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