use std::{env, process::exit};
use reqwest::blocking::Client;
use serde::Deserialize;


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

    let repos = get_repos(handle);

    for repo in repos {
        println!("{}: {}", repo.name, repo.html_url);
    }
}

#[derive(Deserialize,Debug)]
struct Repository {
    name: String,
    html_url: String,
}

fn get_repos(handle: String) -> Vec<Repository> {
    let client = Client::new();

    let response = client
        .get(format!("https://api.github.com/users/{}/repos", handle))
        .header("User-Agent", "Test")
        .send();

    let repos = match response {
        Ok(d) => d,
        Err(_) => {
            println!("GitHub didn't like the username {}", handle);
            exit(1);
        }
    };

    let found_repos = match repos.json::<Vec<Repository>>() {
        Ok(r) => r,
        Err(_) => {
            println!("Failed to parse response - or something like that.");
            exit(1);
        }
    };

    return found_repos;
}
