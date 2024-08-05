use std::io;
mod day1;
mod day2;

static DAYS: [fn() -> io::Result<()>; 2] = [day1::run, day2::run];

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
