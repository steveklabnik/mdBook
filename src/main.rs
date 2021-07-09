#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use anyhow::anyhow;
use chrono::Local;
use clap::{App, AppSettings, Arg, ArgMatches, Shell, SubCommand};
use env_logger::Builder;
use log::LevelFilter;
use mdbook::utils;
use std::env;
use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};

mod cmd;

const VERSION: &str = concat!("v", crate_version!());

fn main() {
    init_logger();

    let app = create_clap_app();

    // Check which subcomamnd the user ran...
    let res = match app.get_matches().subcommand() {
        ("init", Some(sub_matches)) => cmd::init::execute(sub_matches),
        ("build", Some(sub_matches)) => cmd::build::execute(sub_matches),
        ("clean", Some(sub_matches)) => cmd::clean::execute(sub_matches),
        #[cfg(feature = "watch")]
        ("watch", Some(sub_matches)) => cmd::watch::execute(sub_matches),
        #[cfg(feature = "serve")]
        ("serve", Some(sub_matches)) => cmd::serve::execute(sub_matches),
        ("test", Some(sub_matches)) => cmd::test::execute(sub_matches),
        ("completions", Some(sub_matches)) => (|| {
            let shell: Shell = sub_matches
                .value_of("shell")
                .ok_or_else(|| anyhow!("Shell name missing."))?
                .parse()
                .map_err(|s| anyhow!("Invalid shell: {}", s))?;

            create_clap_app().gen_completions_to("mdbook", shell, &mut std::io::stdout().lock());
            Ok(())
        })(),
        (_, _) => unreachable!(),
    };

    if let Err(e) = res {
        utils::log_backtrace(&e);

        std::process::exit(101);
    }
}

/// Create a list of valid arguments and sub-commands
fn create_clap_app<'a, 'b>() -> App<'a, 'b> {
    let app = App::new(crate_name!())
        .about(crate_description!())
        .author("Mathieu David <mathieudavid@mathieudavid.org>")
        .version(VERSION)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .after_help(
            "For more information about a specific command, try `mdbook <command> --help`\n\
             The source code for mdBook is available at: https://github.com/rust-lang/mdBook",
        )
        .subcommand(cmd::init::make_subcommand())
        .subcommand(cmd::build::make_subcommand())
        .subcommand(cmd::test::make_subcommand())
        .subcommand(cmd::clean::make_subcommand())
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate shell completions for your shell to stdout")
                .arg(
                    Arg::with_name("shell")
                        .takes_value(true)
                        .possible_values(&Shell::variants())
                        .help("the shell to generate completions for")
                        .value_name("SHELL")
                        .required(true),
                ),
        );

    #[cfg(feature = "watch")]
    let app = app.subcommand(cmd::watch::make_subcommand());
    #[cfg(feature = "serve")]
    let app = app.subcommand(cmd::serve::make_subcommand());

    app
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
        // Filter extraneous html5ever not-implemented messages
        builder.filter(Some("html5ever"), LevelFilter::Error);
    }

    builder.init();
}

fn get_book_dir(args: &ArgMatches) -> PathBuf {
    if let Some(dir) = args.value_of("dir") {
        // Check if path is relative from current dir, or absolute...
        let p = Path::new(dir);
        if p.is_relative() {
            env::current_dir().unwrap().join(dir)
        } else {
            p.to_path_buf()
        }
    } else {
        env::current_dir().expect("Unable to determine the current directory")
    }
}

fn open<P: AsRef<OsStr>>(path: P) {
    if let Err(e) = open::that(path) {
        error!("Error opening web browser: {}", e);
    }
}
