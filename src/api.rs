use std::process::exit;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct Repository {
    pub name: String,
    pub html_url: String,
}

pub fn get_repos(handle: String) -> Vec<Repository> {
    let response = Client::new()
        .get(format!("https://api.github.com/users/{}/repos?per_page=100", handle))
        .header("User-Agent", "Test User Agent")
        .send();

    let repos = match response {
        Ok(d) => d,
        Err(_) => {
            println!("GitHub didn't like the username {}", handle);
            exit(1);
        }
    };

    match repos.json::<Vec<Repository>>() {
        Ok(r) => r,
        Err(_) => {
            println!("Failed to parse response - or something like that.");
            exit(1);
        }
    }
}