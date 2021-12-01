static FILE_CONTENTS: &'static [u8] = include_bytes!("../../Cargo.lock");
extern crate zip;

use std::io::{Seek, Write};
use std::fs::{File};

use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};

fn create_zip_archive<T: Seek + Write>(buf: &mut T) -> ZipResult<()> {
    let mut writer = ZipWriter::new(buf);
    writer.start_file("example.txt", FileOptions::default())?;
    writer.write(FILE_CONTENTS)?;
    writer.finish()?;
    Ok(())
}

fn main() {
    let mut file = File::create("example.zip").expect("Couldn't create file");
    create_zip_archive(&mut file).expect("Couldn't create archive");
}
