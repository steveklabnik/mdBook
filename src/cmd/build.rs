use crate::{get_book_dir, open};
use clap::{App, ArgMatches, SubCommand};
use mdbook::errors::Result;
use mdbook::MDBook;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("build")
        .about("Builds a book from its markdown files")
        .arg_from_usage(
            "-d, --dest-dir=[dest-dir] 'Output directory for the book{n}\
             Relative paths are interpreted relative to the book's root directory.{n}\
             If omitted, mdBook uses build.build-dir from book.toml or defaults to `./book`.'",
        )
        .arg_from_usage(
            "[dir] 'Root directory for the book{n}\
             (Defaults to the Current Directory when omitted)'",
        )
        .arg_from_usage("-o, --open 'Opens the compiled book in a web browser'")
        .arg_from_usage(
            "--auto-summary 'Automatically generate the book's summary{n}\
             from the sources directory structure.'",
        )
}

// Build command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let mut book = MDBook::load(&book_dir, args.is_present("auto-summary"))?;

    if let Some(dest_dir) = args.value_of("dest-dir") {
        book.config.build.build_dir = dest_dir.into();
    }

    book.build()?;

    if args.is_present("open") {
        // FIXME: What's the right behaviour if we don't use the HTML renderer?
        open(book.build_dir_for("html").join("index.html"));
    }

    Ok(())
}
