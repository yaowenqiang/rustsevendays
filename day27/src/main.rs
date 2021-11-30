#[macro_use]
extern crate clap;

#[macro_use]
extern crate slog;
extern crate slog_term;

use std::process;
use clap::{Arg, ArgMatches, App, SubCommand};
use slog::Drain;
fn main() {
    let matches = App::new("24daysofrust")
        .version("0.1")
        .author("jack")
        .about("learning rust")
        .arg(Arg::with_name("verbose")
             .short("v")
             .multiple(true)
             .help("verbosity level"))
        .get_matches();

    /*
    if let Err(e) = run(matches) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    */
}

fn run(matches: ArgMatches)  {
    let min_log_level = match matches.occurrences_of("verbose") {
        0 => slog::Level::Info,
        1 => slog::Level::Debug,
        2 | _ => slog::Level::Trace,
    };

    //let drain = slog::level_filter(min_log_level, slog_term::streamer().build()).fuse();
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stderr());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let logger = slog::Logger::root(drain, o!());

    trace!(logger, "app_setup");
    debug!(logger, "load_configuration");
    trace!(logger, "app_setup_complete");
    info!(logger, "processing_started");
}
