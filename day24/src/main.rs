//#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

#[derive(Serialize, Deserialize, Debug, Default)]

struct GameConfig {
    save_dir: Option<String>,
    autosave: bool,
    fov: f32,
    render_distance: u32,
}

#[derive(Deserialize, Debug)]
struct Task {
    name: String,
    command: String,
}

#[derive(Deserialize, Debug)]
struct Play {
    #[serde(rename = "hosts")]
    host_list: String,
    tasks: Vec<Task>,
}

fn main() {
    let config = GameConfig::default();
    let json = serde_json::to_string(&config).expect("couldn't serialize config");
    let json2 = serde_json::to_string_pretty(&config).expect("couldn't serialize config");
    println!("{}", json);
    println!("{}", json2);

    type Playbook = Vec<Play>;
    let yaml = include_str!("../playbook.yaml");
    println!("{}", yaml);
    let playbook = serde_yaml::from_str::<Playbook>(&yaml);
    println!("{:#?}", playbook);
}
