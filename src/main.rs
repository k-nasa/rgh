use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};
use surf::{http, http::method::Method, url};

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

fn main() -> RghResult<()> {
    let app = build_app();

    let matches = app.get_matches();

    let _tag = matches.value_of("tag").unwrap();
    let _pkg = matches.value_of("packages").unwrap();

    Ok(())
}

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .args(&[
            Arg::with_name("tag").help("tag").required(true),
            Arg::with_name("packages")
                .help("upload packages dir or file")
                .required(true),
        ])
}
