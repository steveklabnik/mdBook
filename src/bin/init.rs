use std::io;
use std::process::Command;
use std::io::Write;
use mdbook::MDBook;
use mdbook::errors::Result;
use mdbook::config;
use get_book_dir;

#[derive(StructOpt)]
pub struct InitArgs {
    #[structopt(long = "theme", help = "Copies the default theme into your source folder")]
    pub theme: bool,
    #[structopt(long = "force", help = "Skip confirmation prompts")] pub force: bool,
    #[structopt(help = "A directory for your book{n}(Defaults to Current Directory when omitted)")]
    pub dir: Option<String>,
}

// Init command implementation
pub fn execute(args: InitArgs) -> Result<()> {
    let book_dir = get_book_dir(args.dir);
    let mut builder = MDBook::init(&book_dir);
    let mut config = config::Config::default();

    // If flag `--theme` is present, copy theme to src
    if args.theme {
        config.set("output.html.theme", "src/theme")?;
        // Skip this if `--force` is present
        if !args.force {
            // Print warning
            println!();
            println!(
                "Copying the default theme to {}",
                builder.config().book.src.display()
            );
            println!("This could potentially overwrite files already present in that directory.");
            print!("\nAre you sure you want to continue? (y/n) ");

            // Read answer from user and exit if it's not 'yes'
            if confirm() {
                builder.copy_theme(true);
            }
        } else {
            builder.copy_theme(true);
        }
    }

    println!("\nDo you want a .gitignore to be created? (y/n)");

    if confirm() {
        builder.create_gitignore(true);
    }

    config.book.title = request_book_title();

    if let Some(author) = get_author_name() {
        debug!("Obtained user name from gitconfig: {:?}", author);
        config.book.authors.push(author);
        builder.with_config(config);
    }

    builder.build()?;
    println!("\nAll done, no errors...");

    Ok(())
}

/// Obtains author name from git config file by running the `git config` command.
fn get_author_name() -> Option<String> {
    let output = Command::new("git")
        .args(&["config", "--get", "user.name"])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        None
    }
}

/// Request book title from user and return if provided.
fn request_book_title() -> Option<String> {
    println!("What title would you like to give the book? ");
    io::stdout().flush().unwrap();
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();
    let resp = resp.trim();
    if resp.is_empty() {
        None
    } else {
        Some(resp.into())
    }
}

// Simple function for user confirmation
fn confirm() -> bool {
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    match &*s.trim() {
        "Y" | "y" | "yes" | "Yes" => true,
        _ => false,
    }
}
