use serde::{Deserialize, Serialize};
use surf::{http, http::method::Method, url};

#[derive(Deserialize, Serialize)]
pub struct RequestCrateRelease {
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseCreateRelease {
    pub id: usize,
}

type RghResult<T> = std::result::Result<T, RghError>;
type RghError = Box<dyn std::error::Error + Send + Sync>;

fn github_client(
    method: http::Method,
    url: String,
    token: String,
) -> Result<surf::Request<impl surf::middleware::HttpClient>, RghError> {
    let url = url::Url::parse(&format!("https://api.github.com{}", url))?;

    Ok(surf::Request::new(method, url).set_header("Authorization", format!("token {}", token)))
}

pub async fn create_release(
    owner: &str,
    repo: &str,
    token: String,
    arg: RequestCrateRelease,
) -> RghResult<ResponseCreateRelease> {
    let mut res = github_client(
        Method::POST,
        format!("/repos/{}/{}/releases", owner, repo),
        token.clone(),
    )?
    .body_json(&arg)?
    .await?;

    if res.status() != 201 {
        let e = res.body_string().await?;
        // FIXME なぜio errorなのか
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed create_release: response is {}", e),
        )));
    }

    Ok(serde_json::from_str(&res.body_string().await?)?)
}
