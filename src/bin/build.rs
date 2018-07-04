use std::path::PathBuf;
use mdbook::MDBook;
use mdbook::errors::Result;
use {get_book_dir, open};

#[derive(StructOpt)]
pub struct BuildArgs {
    #[structopt(long = "open", short = "o", help = "Open the compiled book in a web browser")]
    open: bool,
    #[structopt(help = "A directory for your book{n}(Defaults to Current Directory when omitted)")]
    dir: Option<String>,
    #[structopt(long = "dest-dir", short = "d",
                help = "The output directory for your book{n}(Defaults to ./book when omitted)",
                parse(from_os_str))]
    dest_dir: Option<PathBuf>,
}

// Build command implementation
pub fn execute(args: BuildArgs) -> Result<()> {
    let book_dir = get_book_dir(args.dir);
    let mut book = MDBook::load(&book_dir)?;

    if let Some(dest_dir) = args.dest_dir {
        book.config.build.build_dir = dest_dir;
    }

    book.build()?;

    if args.open {
        // FIXME: What's the right behaviour if we don't use the HTML renderer?
        open(book.build_dir_for("html").join("index.html"));
    }

    Ok(())
}
