use std::{env, process::exit};

fn main() {
    let mut args: env::Args = env::args();

    let handle: String = match args.nth(1) {
        Some(h) => h,
        None => {
            println!("Please provide a GitHub username.");
            exit(1);
        }
    };

    dbg!(handle);

    println!("Hello, world!");
}
