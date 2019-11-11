use clap::{
    crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand,
};

pub type RghResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> RghResult<()> {
    let mut app = build_app();
    match app.clone().get_matches().subcommand() {
        ("help", Some(_)) | _ => app.print_help()?,
    }
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
        .arg(Arg::with_name("tag").help("tag"))
        .arg(Arg::with_name("packages").help("upload packages dir or file"))
        .subcommand(SubCommand::with_name("help").alias("h").about("Show help"))
}
