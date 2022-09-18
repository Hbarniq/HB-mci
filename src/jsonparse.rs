use {
    std::path::{Path},
    serde::{Deserialize, Serialize}
};

#[derive(Serialize, Deserialize)]
struct Profile {
    created: String,
    gameDir: String,
    icon: String,
    javaArgs: String,
    lastUsed: String,
    lastVersionId: String,
    name: String,
    r#type: String
}

#[derive(Serialize, Deserialize)]
struct Profiles {
    profile: Vec<Profile>
}

pub fn main() {
    let update: bool = Path::new("./mods").exists();
    let _mods_dir = if update == false {"./modpack/"} else {"./"};

    let profile_json = r#"
    "profile": "b4071d616fc1a349afd293e8508ddfe2" : [{
            "created": null,
            "gameDir": null,
            "icon": "furnace",
            "javaArgs": "-Xmx4G -XX:+UnlockExperimentalVMOptions -XX:+UseG1GC -XX:G1NewSizePercent=20 -XX:G1ReservePercent=20 -XX:MaxGCPauseMillis=50 -XX:G1HeapRegionSize=32M",
            "lastUsed": null,
            "lastVersionId": fabric-loader-0.14.9-1.18.2,
            "name": 1.18 fabric_modpack,
            "r#type": "custom"
        }]
    "#;
    
    let parsed: Profiles = read_json_typed(profile_json);
    println!("\n\n The versionID is {}", parsed.profile[0].lastVersionId);

}

fn read_json_typed(raw_json: &str) -> Profiles {
    let parsed: Profiles = serde_json::from_str(raw_json).unwrap();
    return parsed
}