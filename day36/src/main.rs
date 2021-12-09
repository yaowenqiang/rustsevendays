extern crate directories;
use directories::{BaseDirs, UserDirs, ProjectDirs};

fn main() {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Foo Corp", "Bar App") {
        println!("{:?}", proj_dirs.config_dir());
    }

    if let Some(base_dirs) = BaseDirs::new() {
        println!("{:?}",base_dirs.executable_dir());
    }

    if let Some(user_dirs) = UserDirs::new() {
        println!("{:?}",user_dirs.audio_dir());
    }
}
