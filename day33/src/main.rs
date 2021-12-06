#[feature(proc_macro)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tempfile;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {}
}

use errors::*;


#[derive(Debug, Serialize, Deserialize)]
struct Schedule {
    rules: Vec<Rule>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    frequency: Frequency,
    command: String

}
#[derive(Debug, Serialize, Deserialize)]
enum Frequency {
    Hourly(i32),
    Daily(i32),
    Monthly(i32),
}



fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let schedule = load_schedule("data/schedule.json").chain_err(|| " failed to load schedule")?;
    if schedule.rules.is_empty() {
        bail!("the schedule is empty!");
    }

    update_crontab(&schedule).chain_err(|| "failed to update crontab")
}

fn load_schedule<P: AsRef<Path>>(path: P) -> Result<Schedule> {
    let file = File::open(path).chain_err(|| "failed to open input file");
    serde_json::from_reader(&file).chain_err(|| "failed to read  JSON")
}

fn update_crontab(schedule: &Schedule) -> Result<()> {
    let mut file =
        tempfile::NamedTempFile::new().chain_err(|| "failed to create a temporary file")?;
    let schedule_str = format!("{}", schedule);
    file.write_all(&schedule_str.into_bytes()[..])
        .chain_err(|| "failed to write schedule")?;
    let path = file.path().to_str().ok_or("temporary path is not UTF-8")?;

    Command::new("crontab")
        .arg(path)
        .spawn()
        .chain_err(|| "failed to run crontab command")?;
    Ok(())
}
