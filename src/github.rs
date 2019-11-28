use async_std::path::Path;
use serde::{Deserialize, Serialize};
use surf::url;

use crate::RghResult;

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

pub async fn create_release(
    owner: &str,
    repo: &str,
    token: &str,
    arg: RequestCrateRelease,
) -> RghResult<ResponseCreateRelease> {
    let token = format!("token {}", token);
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);

    let mut res = surf::post(url)
        .set_header("Authorization", &token)
        .body_json(&arg)?
        .await?;

    if res.status() != 201 {
        let e = res.body_string().await?;
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

    let url = format!(
        "https://uploads.github.com/repos/{}/{}/releases/{}/assets?name={:?}",
        owner, repo, release_id, filename,
    );
    let url = url::Url::parse(&url)?;

    let token = format!("token {}", token);
    let bytes = format!("{}", bytes);

    let mut res = surf::post(url)
        .set_header("Authorization", &token)
        .set_header("content-length", &bytes)
        .body_file(filepath)?
        .await?;

    if res.status() != 201 {
        let e = res.body_string().await?;
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed upload_assets: response is {}", e),
        )));
    }

    Ok(())
}
