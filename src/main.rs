use std::{env, process::exit};

mod api;

fn main() {
    let mut args: env::Args = env::args();

    let handle: String = match args.nth(1) {
        Some(h) => h,
        None => {
            println!("Please provide a GitHub username.");
            exit(1);
        }
    };

    println!("Fetching GitHub Repos for {}", handle);

    let repos = api::get_repos(handle);

    for repo in repos {
        println!("{}: {}", repo.name, repo.html_url);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        assert_eq!(true, true);
    }
}