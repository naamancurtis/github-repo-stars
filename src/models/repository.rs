use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    pub(crate) name: String,
    #[serde(rename(deserialize = "html_url"))]
    pub(crate) url: String,
    pub(crate) description: Option<String>,
    #[serde(rename(deserialize = "stargazers_count"))]
    pub(crate) stars: u32,
}

impl Repository {
    pub(crate) fn print_to_terminal(&self) {
        let description = self
            .description
            .clone()
            .unwrap_or_else(|| "N/a".to_string());

        println!("======================");
        println!("{:12}: {}", "Name".green().bold(), self.name);
        println!("{:12}: {}", "Url".green().bold(), self.url.italic());
        println!(
            "{:12}: {}",
            "Description".green().bold(),
            description.italic()
        );
        println!("{:12}: {}", "Stars".yellow().bold(), self.stars);
        println!("======================");
        println!();
    }
}
