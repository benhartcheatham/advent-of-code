#[macro_export]
macro_rules! time {
    ( $($args:tt)+ ) => {
        {
            use std::time::Instant;

            let start = Instant::now();
            print!(format_args!($($args)+));
            print!(" (Ran in {:.2?})", instant.elapsed());
        }
    };
}

#[macro_export]
macro_rules! timeln {
    ( $($args:tt)+ ) => {
        {
            use std::time::Instant;

            let start = Instant::now();
            print!("{}", format_args!($($args)+));
            println!(" (Ran in {:.2?})", start.elapsed());
        }
    };
}
