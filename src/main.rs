use clap::{Parser, Subcommand};
use reqwest::header::{HeaderValue, COOKIE};
use reqwest::Client;
use std::fs::{self, File};
use std::io::Write;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

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
        // time the solution
        #[arg(long, action)]
        nobenchmark: bool,
    },
    // Set up a template file for <year> <day>. Also puts the puzzle input in
    // inputs/<year>/day<day>.txt
    Create {
        #[arg(short, long)]
        year: String,
        #[arg(short, long)]
        day: Option<usize>,
        // Session key for getting input. If not specified, this command will only create a template
        // file
        #[arg(short, long)]
        session: Option<PathBuf>,
        // Only get inputs for given year (day is optional)
        #[arg(long)]
        download: bool,
    },
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Run { year, day, nobenchmark }) => run(year, *day, !*nobenchmark),
        Some(Commands::Create {
            year,
            day,
            session,
            download,
        }) => create(year, *day, session.clone(), *download).await,
        _ => Ok(()),
    }
}

async fn create(
    year: &str,
    day: Option<usize>,
    session: Option<PathBuf>,
    download: bool,
) -> io::Result<()> {
    let client = create_client();

    let lower = day.unwrap_or(1);
    let upper = day.unwrap_or(25);

    if !download {
        for i in lower..=upper {
            create_day(year, i)?;
        }
    }

    if session.is_none() && download {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Session cookie file (-s option) required for --download option",
        ));
    }

    let key = fs::read_to_string(session.expect("Session cookie file not provided"))
        .expect("Could not find session cookie file");
    for i in lower..=upper {
        download_inputs(&client, &key, year, i).await?;
    }

    Ok(())
}

fn create_day(year: &str, day: usize) -> io::Result<()> {
    let path = format!("src/{}/day{}.rs", year, day);
    if Path::new(&path).exists() {
        return Ok(());
    }

    let mut template = File::open("src/day_template.rs").expect("Could not find day template");
    let mut day_file = File::create(path).expect("Error creating day file");
    let mut buf = Vec::new();

    template.read_to_end(&mut buf)?;
    day_file.write_all(&buf)
}

async fn download_inputs(client: &Client, key: &str, year: &str, day: usize) -> io::Result<()> {
    let year_path = format!("inputs/{}", year);

    if !Path::new(&year_path).exists() {
        fs::create_dir(year_path)?;
    }

    let day_path = format!("inputs/{}/day{}.txt", year, day);
    if Path::new(&day_path).exists() {
        return Ok(());
    }

    let mut input = File::create(day_path)?;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    if let Some(text) = send_request(client, key, &url).await {
        input.write_all(text.as_bytes())?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Error getting puzzle input for {} day {}", year, day),
        ));
    }

    Ok(())
}

fn create_client() -> Client {
    Client::builder()
        .user_agent("benhartcheatham@gmail.com")
        .cookie_store(true)
        .build()
        .unwrap()
}

async fn send_request(client: &Client, session: &str, url: &str) -> Option<String> {
    let mut req = client.get(url).build().unwrap();
    req.headers_mut().insert(
        COOKIE,
        HeaderValue::from_str(&format!("session={}", session.trim())).unwrap(),
    );

    if let Ok(response) = client.execute(req).await {
        let text = response.text().await.unwrap();
        Some(text)
    } else {
        None
    }
}

fn run(year: &str, day: Option<usize>, benchmark: bool) -> io::Result<()> {
    match year {
        "2022" => year22::run(day, benchmark),
        "2023" => year23::run(day),
        "2024" => year24::run(day, benchmark),
        _ => Ok(()),
    }
}
