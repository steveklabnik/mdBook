use crate::{get_book_dir, get_build_opts};
use anyhow::Context;
use clap::{App, ArgMatches, SubCommand};
use mdbook::MDBook;
use std::fs;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("clean")
        .about("Deletes a built book")
        .arg_from_usage(
            "-d, --dest-dir=[dest-dir] 'Output directory for the book{n}\
             Relative paths are interpreted relative to the book's root directory.{n}\
             Running this command deletes this directory.{n}\
             If omitted, mdBook uses build.build-dir from book.toml or defaults to `./book`.'",
        )
        .arg_from_usage(
            "[dir] 'Root directory for the book{n}\
             (Defaults to the Current Directory when omitted)'",
        )
        .arg_from_usage(
            "-l, --language=[language] 'Language to render the compiled book in.{n}\
                         Only valid if the [language] table in the config is not empty.{n}\
                         If omitted, builds all translations and provides a menu in the generated output for switching between them.'",
        )
}

// Clean command implementation
pub fn execute(args: &ArgMatches) -> mdbook::errors::Result<()> {
    let book_dir = get_book_dir(args);
    let build_opts = get_build_opts(args);
    let book = MDBook::load_with_build_opts(&book_dir, build_opts)?;

    let dir_to_remove = match args.value_of("dest-dir") {
        Some(dest_dir) => dest_dir.into(),
        None => book.root.join(&book.config.build.build_dir),
    };

    if dir_to_remove.exists() {
        fs::remove_dir_all(&dir_to_remove)
            .with_context(|| "Unable to remove the build directory")?;
    }

    Ok(())
}
