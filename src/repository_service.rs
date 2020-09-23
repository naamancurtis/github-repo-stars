use crate::errors::GbRepoError;
use crate::models::Repository;
use lazy_static::lazy_static;
use reqwest::header;

const BASE_GITHUB_URL: &str = "https://api.github.com/users";

lazy_static! {
    static ref HEADERS: header::HeaderMap = {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Accept",
            header::HeaderValue::from_static("application/vnd.github.v3+json"),
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Carv-Cli-Dev-Test"),
        );
        headers
    };
}

pub(crate) async fn fetch_repositories(username: &str) -> Result<Vec<Repository>, GbRepoError> {
    let url = format!("{}/{}/repos", BASE_GITHUB_URL, username);

    let client = reqwest::ClientBuilder::new()
        .default_headers(HEADERS.clone())
        .build()?;
    let response = client.get(&url).send().await?;
    if response.status() == 404 {
        return Err(GbRepoError::UserNotFound);
    }
    let mut repositories: Vec<Repository> = response.json::<Vec<Repository>>().await?;

    repositories.sort_by(|a, b| b.stars.cmp(&a.stars));

    Ok(repositories.into_iter().take(10).collect())
}
