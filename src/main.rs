use std::env::args;
use std::io;

mod utils;
#[path = "2022/mod.rs"]
mod year22;
#[path = "2023/mod.rs"]
mod year23;

fn main() -> io::Result<()> {
    let year = args().nth(1).unwrap_or(String::from("2022"));
    let day = if let Some(day) = args().nth(2) {
        let n = day.parse::<usize>();

        if let Ok(n) = n {
            Some(n)
        } else {
            None
        }
    } else {
        None
    };

    match year.as_str() {
        "2022" => year22::run(day),
        "2023" => year23::run(day),
        _ => Ok(()),
    }
}
