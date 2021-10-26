extern crate csv;
use std::error::Error;
use csv::{Reader, Writer};
fn main() {
    write();
}

fn write()->Result<(), Box<dyn Error>>{

    let doller_files = vec![
        ["A Fistful of Dollars", "Rojo", "1964"],
        ["For a Few Dollars More","Ei Indio", "1965"],
        ["the Good, the Bad and the Ugly", "Tuco", "1966"],
    ];

    let path = "westerns.csv";
    let mut writer = Writer::from_path(path).unwrap();
    for row in doller_files {
        writer.write_record(&row)?;
        writer.flush()?;
    }
    Ok(())
}

