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
