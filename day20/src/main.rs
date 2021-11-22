#[macro_use]
extern crate slog;
extern crate time;
extern crate slog_term;

use slog::Drain;

fn main() {
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stderr());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let root_logger = slog::Logger::root(drain, o!("version" => "0.5"));
    info!(root_logger, "Application started";
          "started_at" => format!("{}", time::now().rfc3339()));
}
