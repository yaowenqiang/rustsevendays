#[macro_use]
extern crate slog;
extern crate time;
extern crate slog_term;

use slog::Drain;

struct User {
    username: String,
    logger: slog::Logger,
}

impl User {
    fn new(username: &str, logger: &slog::Logger) -> Self {
        User {
            username: username.to_string(),
            logger: logger.new(o!("username" => username.to_string())),
        }
    }

    fn sign_in(&self) {
        info!(self.logger, "user signed in");
    }
}

fn main() {
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stderr());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let root_logger = slog::Logger::root(drain, o!("version" => "0.5"));
    info!(root_logger, "Application started";
          "started_at" => format!("{}", time::now().rfc3339()));

    let user = User::new("jacky", &root_logger);
    user.sign_in();

}
