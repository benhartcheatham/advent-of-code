use std::io;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod pipe;

static DAYS: [fn() -> io::Result<()>; 25] = [
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
    day15::run,
    day16::run,
    day17::run,
    day18::run,
    day19::run,
    day20::run,
    day21::run,
    day22::run,
    day23::run,
    day24::run,
    day25::run,
];

fn run_all() {
    for i in 0..(DAYS.len() - 1) {
        run_day(i);
        println!();
    }

    run_day(DAYS.len() - 1);
}

fn run_day(day: usize) {
    println!("day{}:", day + 1);
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
