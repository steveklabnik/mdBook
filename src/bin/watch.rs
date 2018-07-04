extern crate notify;

use std::path::Path;
use self::notify::Watcher;
use std::time::Duration;
use std::sync::mpsc::channel;
use mdbook::MDBook;
use mdbook::utils;
use mdbook::errors::Result;
use {get_book_dir, open};

#[derive(StructOpt)]
pub struct WatchArgs {
    #[structopt(help = "A directory for your book{n}(Defaults to Current Directory when omitted)")]
    dir: Option<String>,
    #[structopt(long = "open", short = "o", help = "Open the compiled book in a web browser")]
    open: bool,
}

// Watch command implementation
pub fn execute(args: WatchArgs) -> Result<()> {
    let book_dir = get_book_dir(args.dir);
    let book = MDBook::load(&book_dir)?;

    if args.open {
        book.build()?;
        open(book.build_dir_for("html").join("index.html"));
    }

    trigger_on_change(&book, |path, book_dir| {
        info!("File changed: {:?}\nBuilding book...\n", path);
        let result = MDBook::load(&book_dir).and_then(|b| b.build());

        if let Err(e) = result {
            error!("Unable to build the book");
            utils::log_backtrace(&e);
        }
    });

    Ok(())
}

/// Calls the closure when a book source file is changed, blocking indefinitely.
pub fn trigger_on_change<F>(book: &MDBook, closure: F)
where
    F: Fn(&Path, &Path),
{
    use self::notify::RecursiveMode::*;
    use self::notify::DebouncedEvent::*;

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    let mut watcher = match notify::watcher(tx, Duration::from_secs(1)) {
        Ok(w) => w,
        Err(e) => {
            error!("Error while trying to watch the files:\n\n\t{:?}", e);
            ::std::process::exit(1)
        }
    };

    // Add the source directory to the watcher
    if let Err(e) = watcher.watch(book.source_dir(), Recursive) {
        error!("Error while watching {:?}:\n    {:?}", book.source_dir(), e);
        ::std::process::exit(1);
    };

    let _ = watcher.watch(book.theme_dir(), Recursive);

    // Add the book.toml file to the watcher if it exists
    let _ = watcher.watch(book.root.join("book.toml"), NonRecursive);

    info!("Listening for changes...");

    for event in rx.iter() {
        debug!("Received filesystem event: {:?}", event);
        match event {
            Create(path) | Write(path) | Remove(path) | Rename(_, path) => {
                closure(&path, &book.root);
            }
            _ => {}
        }
    }
}
