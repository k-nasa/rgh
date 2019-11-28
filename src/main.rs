mod github;

use github::{create_release, upload_asset, RequestCrateRelease};

use std::process::Command;
use std::str::FromStr;
use std::sync::Arc;

use async_std::fs;
use async_std::path::Path;
use async_std::prelude::*;
use async_std::task;
use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};
use indicatif::{ProgressBar, ProgressStyle};

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

    let result: RghResult<()> = task::block_on(async move {
        let r = create_release(&owner, &repo, &token, request).await?;

        let path = Path::new(&pkg);

        if path.is_file().await {
            upload_asset(&owner, &repo, &token, r.id, &pkg).await?;
        } else if path.is_dir().await {
            let mut dir = fs::read_dir(pkg).await?;

            let owner = Arc::new(owner);
            let repo = Arc::new(repo);
            let token = Arc::new(token);
            let r = Arc::new(r);

            let mut futures = vec![];

            while let Some(res) = dir.next().await {
                let entry = res?;
                if entry.path().is_file().await {
                    let owner = owner.clone();
                    let repo = repo.clone();
                    let r = r.clone();
                    let token = token.clone();

                    futures.push(task::spawn(async move {
                        println!("uploading {:?}", entry.path().into_os_string());
                        upload_asset(&owner, &repo, &token, r.id, &entry.path().to_str().unwrap())
                            .await
                    }));
                }
            }

            let pb = ProgressBar::new(futures.iter().count() as u64);
            let mut position = 0;
            pb.set_style(
                ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})",
                )
                .progress_chars("#>-"),
            );
            pb.finish_with_message("finished");

            for f in futures {
                position += 1;
                pb.set_position(position);
                if let Err(e) = f.await {
                    println!("failed upload file: {}", e)
                }
            }
        }
        Ok(())
    });

    match result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }

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
