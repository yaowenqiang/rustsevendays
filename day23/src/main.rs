#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

#![feature(plugin)]
#![plugin(phf_macros)]

#[macro_use]
extern crate phf;

static COLORS: phf::Map<&'static str, Color> = phf_map! {
    "amber" => Color {r: 255, g: 191, b: 0},
    "zinnwaldite brown" => Some(Color {r: 44, g: 22, b: 8}),
};

pub fn find_color_phf(name: &str) -> Option<Color> {
    COLORS.get(name.to_lowercase().as_str()).cloned()
}

lazy_static! {
    static ref COLORS_MAP : HashMap<&'static str, Color> = {;
        let mut map = HashMap::new();
        map.insert("amber", Color {r:255, g: 191, b: 0});
        map.insert("zinnwaldite brown", Color {r: 44, g: 22, b: 0});
        map
    };

}

pub fn find_color_lazy_static(name: &str) -> Option<Color> {
    COLORS_MAP.get(name.to_lowercase().as_str()).cloned()
}

#[derive(Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}


pub fn find_color(name: &str) -> Option<Color> {
    match name.to_lowercase().as_str() {
        "amber" => Some(Color {r:255, g: 191, b: 0}),
        "zinnwaldite brown" => Some(Color {r: 44, g: 22, b: 8}),
        _ => None,
    }
}

fn main() {
    println!("Hello, world!");
}
