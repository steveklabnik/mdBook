extern crate chrono;
extern crate clap;
extern crate env_logger;
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate mdbook;
extern crate open;
#[macro_use]
extern crate structopt;

use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::io::Write;
use chrono::Local;
use log::LevelFilter;
use env_logger::Builder;
use mdbook::utils;
use structopt::StructOpt;

pub mod build;
pub mod clean;
pub mod init;
pub mod test;
#[cfg(feature = "serve")]
pub mod serve;
#[cfg(feature = "watch")]
pub mod watch;

fn main() {
    init_logger();

    let opt = Opts::from_args();

    let res = match opt {
        Opts::Init(x) => init::execute(x),
        Opts::Build(x) => build::execute(x),
        Opts::Clean(x) => clean::execute(x),
        #[cfg(feature = "watch")]
        Opts::Watch(x) => watch::execute(x),
        #[cfg(feature = "serve")]
        Opts::Serve(x) => serve::execute(x),
        Opts::Test(x) => test::execute(x),
    };

    if let Err(e) = res {
        utils::log_backtrace(&e);

        ::std::process::exit(101);
    }
}

/// Subcommands and their respective parameters.
#[derive(StructOpt)]
#[structopt(about = "Create a book in form of a static website from markdown files",
            after_help = "For more information about a specific command, \
                          try `mdbook <command> --help`\n\
                          Source code for mdbook available \
                          at: https://github.com/rust-lang-nursery/mdBook",
            author = "Mathieu David <mathieudavid@mathieudavid.org>")]
enum Opts {
    #[structopt(name = "init", about = "Create boilerplate structure and files in the directory")]
    Init(init::InitArgs),
    #[structopt(name = "build", about = "Build the book from the markdown files")]
    Build(build::BuildArgs),
    #[structopt(name = "clean", about = "Delete built book")] Clean(clean::CleanArgs),
    #[cfg(feature = "watch")]
    #[structopt(name = "watch", about = "Watch the files for changes")]
    Watch(watch::WatchArgs),
    #[cfg(feature = "serve")]
    #[structopt(name = "serve",
                about = "Serve the book at http://localhost:3000. Rebuild and reload on change.")]
    Serve(serve::ServeArgs),
    #[structopt(name = "test", about = "Test that code samples compile")] Test(test::TestArgs),
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
        builder.parse(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
        // Filter extraneous html5ever not-implemented messages
        builder.filter(Some("html5ever"), LevelFilter::Error);
    }

    builder.init();
}

fn get_book_dir(book_dir: Option<String>) -> PathBuf {
    if let Some(ref dir) = book_dir {
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
