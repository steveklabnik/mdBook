use std::fs;
use std::path::PathBuf;
use mdbook::MDBook;
use mdbook::errors::*;
use get_book_dir;

#[derive(StructOpt)]
pub struct CleanArgs {
    #[structopt(long = "dest-dir", short = "d",
                help = "The output directory for your book{n}(Defaults to ./book when omitted)",
                parse(from_os_str))]
    dest_dir: Option<PathBuf>,
}

// Clean command implementation
pub fn execute(args: CleanArgs) -> ::mdbook::errors::Result<()> {
    let book_dir = get_book_dir(None);
    let book = MDBook::load(&book_dir)?;

    let dir_to_remove = match args.dest_dir {
        Some(dest_dir) => PathBuf::from(dest_dir),
        None => book.root.join(&book.config.build.build_dir),
    };
    fs::remove_dir_all(&dir_to_remove).chain_err(|| "Unable to remove the build directory")?;

    Ok(())
}
