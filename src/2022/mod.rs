use std::io;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

static DAYS: [fn() -> io::Result<()>; 14] = [
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
    day12::run,
    day13::run,
    day14::run,
];

fn run_all() {
    for i in 0..DAYS.len() {
        run_day(i);
    }
}

fn run_day(day: usize) {
    println!("\nday{}:", day + 1);
    let result = DAYS[day]();

    match result {
        Ok(()) => (),
        Err(e) => println!("Error! ({:?})", e),
    }
}

pub fn run(day: Option<usize>) -> io::Result<()> {
    if let Some(n) = day {
        let n = n.checked_sub(1).unwrap_or_else(|| {
            println!("Invalid day: {}", n);
            DAYS.len()
        });

        if n < DAYS.len() {
            run_day(n);
        }
    } else {
        run_all();
    }

    Ok(())
}
