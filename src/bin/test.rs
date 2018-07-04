use mdbook::MDBook;
use mdbook::errors::Result;
use get_book_dir;

#[derive(StructOpt)]
pub struct TestArgs {
    #[structopt(help = " Directories to add to crate search path")] library_paths: Vec<String>,
}

// test command implementation
pub fn execute(args: TestArgs) -> Result<()> {
    let book_dir = get_book_dir(None);
    let mut book = MDBook::load(&book_dir)?;

    book.test(args.library_paths.iter().map(AsRef::as_ref).collect())?;

    Ok(())
}
