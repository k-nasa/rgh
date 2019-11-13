use async_std::path::Path;
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
    token: &str,
) -> Result<surf::Request<impl surf::middleware::HttpClient>, RghError> {
    let url = url::Url::parse(&url)?;

    Ok(surf::Request::new(method, url).set_header("Authorization", format!("token {}", token)))
}

pub async fn create_release(
    owner: &str,
    repo: &str,
    token: &str,
    arg: RequestCrateRelease,
) -> RghResult<ResponseCreateRelease> {
    let mut res = github_client(
        Method::POST,
        format!("https://api.github.com/repos/{}/{}/releases", owner, repo),
        token,
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

pub async fn upload_asset(
    owner: &str,
    repo: &str,
    token: &str,
    release_id: usize,
    filepath: &str,
) -> RghResult<()> {
    let bytes = async_std::fs::read(filepath).await?.len();

    let filename = Path::new(filepath).file_name().unwrap();

    let mut res = github_client(
        Method::POST,
        format!(
            "https://uploads.github.com/repos/{}/{}/releases/{}/assets?name={:?}",
            owner, repo, release_id, filename,
        ),
        token,
    )?
    .set_header("content-length", format!("{}", bytes))
    .body_file(filepath)?
    .await?;

    if res.status() != 201 {
        let e = res.body_string().await?;
        // FIXME なぜio errorなのか
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed upload_assets: response is {}", e),
        )));
    }

    Ok(())
}
