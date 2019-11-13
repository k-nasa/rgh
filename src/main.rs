use std::process::Command;
use std::str::FromStr;

use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};
use serde::{Deserialize, Serialize};
use surf::{http, http::method::Method, url};

type RghResult<T> = std::result::Result<T, RghError>;
type RghError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> RghResult<()> {
    let app = build_app();

    let matches = app.get_matches();

    let tag_name = matches.value_of("tag").unwrap().to_owned();
    let pkg = matches.value_of("packages").unwrap().to_owned();

    let (owner, repo) = read_gitconfig()?;

    let token = if let Some(token) = matches.value_of("token") {
        token.to_string()
    } else {
        match std::env::var("GITHUB_TOKEN") {
            Ok(t) => t,
            Err(_) => {
                println!("GITHUB_TOKEN is not setted");
                println!("Please set it via `GITHUB_TOKEN` env variable or `-t` option");
                std::process::exit(1);
            }
        }
    };
    let draft = bool::from_str(matches.value_of("draft").unwrap_or("false"))?;
    let prerelease = bool::from_str(matches.value_of("prerelease").unwrap_or("false"))?;

    let request = RequestCrateRelease {
        tag_name,
        target_commitish: matches
            .value_of("target_commitish")
            .unwrap_or_default()
            .to_owned(),
        name: matches.value_of("name").unwrap_or_default().to_owned(),
        body: matches.value_of("body").unwrap_or_default().to_owned(),
        draft,
        prerelease,
    };

    let result =
        async_std::task::block_on(
            async move { create_release(&owner, &repo, token, request).await },
        );

    let id = result.map(|r| r.id)?;

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

fn github_client(
    method: http::Method,
    url: String,
    token: String,
) -> Result<surf::Request<impl surf::middleware::HttpClient>, RghError> {
    let url = url::Url::parse(&format!("https://api.github.com{}", url))?;

    Ok(surf::Request::new(method, url).set_header("Authorization", format!("token {}", token)))
}

async fn create_release(
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
