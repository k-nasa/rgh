use clap::{
    crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand,
};

pub type RghResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> RghResult<()> {
    let mut app = build_app();

    let matches = app.get_matches();

    let tag = matches.value_of("tag").unwrap();
    let pkg = matches.value_of("packages").unwrap();

    Ok(())
}

fn exec(matches: &ArgMatches) -> RghResult<()> {
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
