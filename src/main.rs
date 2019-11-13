mod github;

use github::{create_release, RequestCrateRelease};

use std::process::Command;
use std::str::FromStr;

use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};

type RghResult<T> = std::result::Result<T, RghError>;
type RghError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> RghResult<()> {
    let app = build_app();

    let matches = app.get_matches();

    let tag_name = matches.value_of("tag").unwrap().to_owned();
    let _pkg = matches.value_of("packages").unwrap().to_owned();

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

    let _id = result.map(|r| r.id)?;

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
