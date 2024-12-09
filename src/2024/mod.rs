use std::io;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

static DAYS: [fn(bool) -> io::Result<()>; 11] = [
    day1::run,
    day2::run,
    day3::run,
    day4::run,
    day5::run,
    day6::run,
    day7::run,
    day8::run,
    day9::run,
    day10::run,
    day11::run,
];

fn run_all(benchmark: bool) {
    for i in 0..(DAYS.len() - 1) {
        run_day(i, benchmark);
        println!();
    }

    run_day(DAYS.len() - 1, benchmark);
}

fn run_day(day: usize, benchmark: bool) {
    println!("day{}:", day + 1);
    let result = DAYS[day](benchmark);

    match result {
        Ok(()) => (),
        Err(e) => println!("Error! ({:?})", e),
    }
}

pub fn run(day: Option<usize>, benchmark: bool) -> io::Result<()> {
    if let Some(n) = day {
        let n = n.checked_sub(1).unwrap_or_else(|| {
            println!("Invalid day: {}", n);
            DAYS.len()
        });

        if n < DAYS.len() {
            run_day(n, benchmark);
        }
    } else {
        run_all(benchmark);
    }

    Ok(())
}
