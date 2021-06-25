use crate::get_book_dir;
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
            "--auto-summary 'Automatically generate the book's summary{n}\
             from the sources directory structure.'",
        )
}

// Clean command implementation
pub fn execute(args: &ArgMatches) -> mdbook::errors::Result<()> {
    let book_dir = get_book_dir(args);
    let book = MDBook::load(&book_dir, args.is_present("auto-summary"))?;

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
