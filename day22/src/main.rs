/*
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate custom_derive;

use derive_builder::Builder;

#[derive(Debug)]
struct Resolution {
    width: u32,
    height: u32,
}
impl Default for Resolution {
    fn default() -> Resolution {
        Resolution {
            width: 1920,
            height: 1080,
        }
    }
}

custom_derive! {
    //#[derive(Debug, Default, Builder)]
    #[derive(Builder)]
    struct GameConfig {
        resolution: Resolution,
        save_dir: Option<String>,
        autosave: bool,
        fov: f32,
        render_distance: u32,
    }

}

fn main() {
    let mut conf = GameConfig::default();
    conf.save_dir("saves".to_string()).fov(70.0).render_distance(1000u32);
    println!("{:?}", conf);
}
*/

#[macro_use]
extern crate derive_builder;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]

struct Channel {
    token: i32,
    special_info: i32,
}

fn main() {
    let ch = ChannelBuilder::default()
        .special_info(42u8)
        .token(19124)
        .build()
        .unwrap();

    println!("{:?}", ch);
}
