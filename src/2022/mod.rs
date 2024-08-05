use std::io;
mod day1;
mod day2;
mod day3;
mod day4;

static DAYS: [fn() -> io::Result<()>; 4] = [day1::run, day2::run, day3::run, day4::run];

fn run_all() {
    for i in 0..DAYS.len() {
        run_day(i);
    }
}

fn run_day(day: usize) {
    println!("day{}:", day);
    let result = DAYS[day]();

    match result {
        Ok(()) => (),
        Err(e) => println!("Error! ({:?})", e),
    }

    println!();
}

pub fn run(day: Option<usize>) -> io::Result<()> {
    match day {
        Some(n) => run_day(n),
        None => run_all(),
    }

    Ok(())
}
