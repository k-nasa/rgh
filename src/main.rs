use std::process::Command;

use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};
use serde::{Deserialize, Serialize};
use surf::{http, http::method::Method, url};

type RghResult<T> = std::result::Result<T, RghError>;
type RghError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> RghResult<()> {
    let app = build_app();

    let matches = app.get_matches();

    let _tag = matches.value_of("tag").unwrap();
    let _pkg = matches.value_of("packages").unwrap();

    let (owner, repo) = read_gitconfig()?;
    // TODO read GITHUB_TOKEN environment variable

    // TODO parse arguments (create arguments struct)

    Ok(())
}

// TODO let outputを外に括りだしてテストコードを書く
fn read_gitconfig() -> RghResult<(String, String)> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("remote.origin.url")
        .output()
        .expect("Failed run git config command");

    let origin_url = std::str::from_utf8(&output.stdout)?.trim();

    let owner = origin_url
        .split('/')
        .nth(3)
        .ok_or_else(|| "Reading of origin url failed")?;

    let repo = origin_url
        .split('/')
        .nth(4)
        .ok_or_else(|| "Reading of origin url failed")?
        .trim_end_matches(".git");

    Ok((owner.to_owned(), repo.to_owned()))
}

#[derive(Deserialize, Serialize)]
struct RequestCrateRelease {
    tag_name: String,
    target_commitish: String,
    name: String,
    body: String,
    draft: bool,
    prerelease: bool,
}

#[derive(Deserialize, Serialize)]
struct ResponseCreateRelease {
    id: usize,
}

#[allow(dead_code)]
fn github_client(
    method: http::Method,
    url: String,
    token: String,
) -> Result<surf::Request<impl surf::middleware::HttpClient>, RghError> {
    let url = url::Url::parse(&format!("https://api.github.com{}", url))?;

    Ok(surf::Request::new(method, url).set_header("Authorization", format!("token {}", token)))
}

#[allow(dead_code)]
async fn create_release(
    owner: &str,
    repo: &str,
    token: String,
    arg: RequestCrateRelease,
) -> RghResult<ResponseCreateRelease> {
    let res: ResponseCreateRelease = github_client(
        Method::POST,
        format!("/repos/{}/{}/releases", owner, repo),
        token,
    )?
    .body_json(&arg)?
    .recv_json()
    .await?;

    Ok(res)
}

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
        .args(&[
            Arg::with_name("tag").help("tag").required(true),
            Arg::with_name("packages")
                .help("upload packages dir or file")
                .required(true),
        ])
        .arg(
            Arg::with_name("commit")
                .help("Specifies the commitish value that determines where the Git tag is created from. Can be any branch or commit SHA. Unused if the Git tag already exists. Default: the repository's default branch (usually master).")
                .long("commit")
                .value_name("target-commitish"),
        )
        .arg(
            Arg::with_name("token")
            .help("Set Github API Token (By default reads the GITHUB_TOKEN environment variable)")
            .long("token")
            .short("t")
            .value_name("token"),
            )
        .arg(
            Arg::with_name("title")
            .help("The title of the release")
            .long("title")
            .value_name("name"),
            )
        .arg(
            Arg::with_name("body")
            .help("Text describing the contents of the tag.")
            .long("body")
            .short("b")
            .value_name("body"),
            )
        .arg(
            Arg::with_name("draft")
            .long("draft")
            .value_name("draft")
            .possible_values(&["true", "false"])
            )
        .arg(
            Arg::with_name("prerelease")
            .long("prerelease")
            .value_name("prerelease")
            .possible_values(&["true", "false"])
            )
}
