use std::env::args;
use std::io;

#[path = "2022/mod.rs"]
mod year22;
#[path = "2023/mod.rs"]
mod year23;

fn main() -> io::Result<()> {
    let year = args().nth(1).unwrap_or(String::from("2022"));

    match year.as_str() {
        "2022" => year22::run(None),
        "2023" => year23::run(None),
        _ => Ok(()),
    }
}
