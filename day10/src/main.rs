extern crate time;

fn sql_macro() {
    let query = sql!("select '{4,5,6}' ::int[]");
    println("{:?}", query);
}

#[cfg(not(target_family="unix"))]
fn sql_macro() {
    println!("TODO");
}

fn main() {
    let dsn = "postgresql://rust:rust@localhost/rust";
    let conn = match connection::connect(dsn, TlsMode::None) {]
        Ok(conn) => conn,
    }
}
