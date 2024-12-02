use clap::{Parser, Subcommand};
use reqwest::header::{HeaderValue, COOKIE};
use reqwest::Client;
use std::fs::{self, File};
use std::io::Write;
use std::io::{self, Read};
use std::path::PathBuf;

mod utils;
#[path = "2022/mod.rs"]
mod year22;
#[path = "2023/mod.rs"]
mod year23;
#[path = "2024/mod.rs"]
mod year24;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Run the solution for <year> <day>. If <day> isn't specified, run all solutions for <year>
    Run {
        // year to run, default to 2024
        #[arg(short, long, default_value_t = String::from("2024"))]
        year: String,
        // day to run
        #[arg(short, long)]
        day: Option<usize>,
    },
    // Set up a template file for <year> <day>. Also puts the puzzle input in
    // inputs/<year>/day<day>.txt
    Create {
        #[arg(short, long)]
        year: String,
        #[arg(short, long)]
        day: usize,
        // Session key for getting input. If not specified, this command will only create a template
        // file
        #[arg(short, long)]
        session: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Run { year, day }) => run(year, *day),
        Some(Commands::Create { year, day, session }) => create(year, *day, session.clone()).await,
        _ => Ok(()),
    }
}

async fn create(year: &str, day: usize, session: Option<PathBuf>) -> io::Result<()> {
    let mut template = File::open("src/day_template.rs")?;
    let mut day_file = File::create(format!("src/{}/day{}.rs", year, day))?;
    let mut buf = Vec::new();

    template.read_to_end(&mut buf)?;
    day_file.write_all(&buf)?;

    if session.is_none() {
        return Ok(());
    }

    let key = fs::read_to_string(session.unwrap())?;
    let mut input = File::create(format!("inputs/{}/day{}.txt", year, day))?;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    if let Some(text) = send_request(&key, &url).await {
        input.write_all(text.as_bytes())?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Error getting puzzle input",
        ))
    }
}

async fn send_request(session: &str, url: &str) -> Option<String> {
    let client = Client::builder()
        .user_agent("benhartcheatham@gmail.com")
        .cookie_store(true)
        .build()
        .unwrap();

    let mut req = client.get(url).build().unwrap();
    req.headers_mut().insert(
        COOKIE,
        HeaderValue::from_str(&format!("session={}", session.trim())).unwrap(),
    );

    // println!("{:?}", client);
    // println!("{:?}", req);
    if let Ok(response) = client.execute(req).await {
        let text = response.text().await.unwrap();
        Some(text)
    } else {
        None
    }
}

fn run(year: &str, day: Option<usize>) -> io::Result<()> {
    match year {
        "2022" => year22::run(day),
        "2023" => year23::run(day),
        "2024" => year24::run(day),
        _ => Ok(()),
    }
}
