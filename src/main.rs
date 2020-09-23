use crate::errors::GbRepoError;
use clap::{App, Arg};

mod errors;
mod file;
mod models;
mod repository_service;

#[tokio::main]
async fn main() -> Result<(), GbRepoError> {
    let matches = App::new("ghstar")
        .version("v0.1")
        .author("Naaman C. <naaman.the.dev@gmail.com>")
        .about("Search for a GitHub user's most popular repositories")
        .arg(
            Arg::with_name("username")
                .required(true)
                .index(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("save_format")
                .short("f")
                .long("format")
                .requires("path")
                .multiple(true)
                .takes_value(true)
                .possible_values(&["json", "toml", "JSON", "TOML"]),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .requires("save_format")
                .takes_value(true),
        )
        .get_matches();

    let username = matches.value_of("username").expect("Argument is required");
    let repositories = repository_service::fetch_repositories(username).await?;

    if !matches.is_present("save_format") {
        repositories.iter().for_each(|r| r.print_to_terminal());
        return Ok(());
    }

    let formats = matches
        .values_of("save_format")
        .expect("Already checked save_format is present");
    let path = matches
        .value_of("path")
        .expect("Argument should already be validated");

    for format in formats {
        match format.to_lowercase().as_str() {
            "json" => file::create_json_file(path, &repositories)?,
            "toml" => file::create_toml_file(path, &repositories)?,
            _ => return Err(GbRepoError::InvalidOutputFormat),
        }
    }

    Ok(())
}
