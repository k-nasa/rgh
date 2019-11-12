use std::collections::HashMap;

use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};
use surf::{http, http::method::Method, url};

type RghResult<T> = std::result::Result<T, RghError>;
type RghError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> RghResult<()> {
    let app = build_app();

    let matches = app.get_matches();

    let _tag = matches.value_of("tag").unwrap();
    let _pkg = matches.value_of("packages").unwrap();

    Ok(())
}

fn github_client(
    method: http::Method,
    url: String,
    token: String,
) -> Result<surf::Request<impl surf::middleware::HttpClient>, RghError> {
    let url = url::Url::parse(&format!("https://api.github.com{}", url))?;

    Ok(surf::Request::new(method, url).set_header("Authorization", format!("token {}", token)))
}

async fn create_release(owner: &str, repo: &str, token: String) -> RghResult<()> {
    let mut body = HashMap::new();

    body.insert("tag_name", "");
    body.insert("target_commitish", "");
    body.insert("name", "");
    body.insert("body", "");
    body.insert("draft", "");
    body.insert("prerelease", "");

    github_client(
        Method::POST,
        format!("/repos/{}/{}/releases", owner, repo),
        token
    )?
    .body_json(&body)?
    .recv_string()
    .await?;

    Ok(())
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
