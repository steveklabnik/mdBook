use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use super::MDBook;
use crate::config::{Config, Language};
use crate::errors::*;
use crate::theme;

/// A helper for setting up a new book and its directory structure.
#[derive(Debug, Clone, PartialEq)]
pub struct BookBuilder {
    root: PathBuf,
    create_gitignore: bool,
    config: Config,
    copy_theme: bool,
    language_ident: String,
}

fn add_default_language(cfg: &mut Config, language_ident: String) {
    let language = Language {
        name: String::from("English"),
        title: None,
        authors: None,
        description: None,
    };
    cfg.language.0.insert(language_ident.clone(), language);
    cfg.book.language = Some(language_ident);
}

impl BookBuilder {
    /// Create a new `BookBuilder` which will generate a book in the provided
    /// root directory.
    pub fn new<P: Into<PathBuf>>(root: P) -> BookBuilder {
        let language_ident = String::from("en");
        let mut cfg = Config::default();
        add_default_language(&mut cfg, language_ident.clone());

        BookBuilder {
            root: root.into(),
            create_gitignore: false,
            config: cfg,
            copy_theme: false,
            language_ident: language_ident,
        }
    }

    /// Get the output source directory of the builder.
    pub fn source_dir(&self) -> PathBuf {
        let src = self
            .config
            .get_localized_src_path(Some(&self.language_ident))
            .unwrap();
        self.root.join(src)
    }

    /// Set the `Config` to be used.
    pub fn with_config(&mut self, mut cfg: Config) -> &mut BookBuilder {
        add_default_language(&mut cfg, self.language_ident.clone());
        self.config = cfg;

        self
    }

    /// Get the config used by the `BookBuilder`.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Should the theme be copied into the generated book (so users can tweak
    /// it)?
    pub fn copy_theme(&mut self, copy: bool) -> &mut BookBuilder {
        self.copy_theme = copy;
        self
    }

    /// Should we create a `.gitignore` file?
    pub fn create_gitignore(&mut self, create: bool) -> &mut BookBuilder {
        self.create_gitignore = create;
        self
    }

    /// Generate the actual book. This will:
    ///
    /// - Create the directory structure.
    /// - Stub out some dummy chapters and the `SUMMARY.md`.
    /// - Create a `.gitignore` (if applicable)
    /// - Create a themes directory and populate it (if applicable)
    /// - Generate a `book.toml` file,
    /// - Then load the book so we can build it or run tests.
    pub fn build(&self) -> Result<MDBook> {
        info!("Creating a new book with stub content");

        self.create_directory_structure()
            .with_context(|| "Unable to create directory structure")?;

        self.create_stub_files()
            .with_context(|| "Unable to create stub files")?;

        if self.create_gitignore {
            self.build_gitignore()
                .with_context(|| "Unable to create .gitignore")?;
        }

        if self.copy_theme {
            self.copy_across_theme()
                .with_context(|| "Unable to copy across the theme")?;
        }

        self.write_book_toml()?;

        match MDBook::load(&self.root) {
            Ok(book) => Ok(book),
            Err(e) => {
                error!("{}", e);

                panic!(
                    "The BookBuilder should always create a valid book. If you are seeing this it \
                     is a bug and should be reported."
                );
            }
        }
    }

    fn write_book_toml(&self) -> Result<()> {
        debug!("Writing book.toml");
        let book_toml = self.root.join("book.toml");
        let cfg = toml::to_vec(&self.config).with_context(|| "Unable to serialize the config")?;

        File::create(book_toml)
            .with_context(|| "Couldn't create book.toml")?
            .write_all(&cfg)
            .with_context(|| "Unable to write config to book.toml")?;
        Ok(())
    }

    fn copy_across_theme(&self) -> Result<()> {
        debug!("Copying theme");

        let themedir = self
            .config
            .html_config()
            .and_then(|html| html.theme)
            .unwrap_or_else(|| self.config.book.src.join("theme"));
        let themedir = self.root.join(themedir);

        if !themedir.exists() {
            debug!(
                "{} does not exist, creating the directory",
                themedir.display()
            );
            fs::create_dir(&themedir)?;
        }

        let mut index = File::create(themedir.join("index.hbs"))?;
        index.write_all(theme::INDEX)?;

        let cssdir = themedir.join("css");
        fs::create_dir(&cssdir)?;

        let mut general_css = File::create(cssdir.join("general.css"))?;
        general_css.write_all(theme::GENERAL_CSS)?;

        let mut chrome_css = File::create(cssdir.join("chrome.css"))?;
        chrome_css.write_all(theme::CHROME_CSS)?;

        let mut print_css = File::create(cssdir.join("print.css"))?;
        print_css.write_all(theme::PRINT_CSS)?;

        let mut variables_css = File::create(cssdir.join("variables.css"))?;
        variables_css.write_all(theme::VARIABLES_CSS)?;

        let mut favicon = File::create(themedir.join("favicon.png"))?;
        favicon.write_all(theme::FAVICON_PNG)?;

        let mut favicon = File::create(themedir.join("favicon.svg"))?;
        favicon.write_all(theme::FAVICON_SVG)?;

        let mut js = File::create(themedir.join("book.js"))?;
        js.write_all(theme::JS)?;

        let mut highlight_css = File::create(themedir.join("highlight.css"))?;
        highlight_css.write_all(theme::HIGHLIGHT_CSS)?;

        let mut highlight_js = File::create(themedir.join("highlight.js"))?;
        highlight_js.write_all(theme::HIGHLIGHT_JS)?;

        Ok(())
    }

    fn build_gitignore(&self) -> Result<()> {
        debug!("Creating .gitignore");

        let mut f = File::create(self.root.join(".gitignore"))?;

        writeln!(f, "{}", self.config.build.build_dir.display())?;

        Ok(())
    }

    fn create_stub_files(&self) -> Result<()> {
        debug!("Creating example book contents");
        let src_dir = self.source_dir();

        let summary = src_dir.join("SUMMARY.md");
        if !summary.exists() {
            trace!("No summary found creating stub summary and chapter_1.md.");
            let mut f = File::create(&summary).with_context(|| "Unable to create SUMMARY.md")?;
            writeln!(f, "# Summary")?;
            writeln!(f)?;
            writeln!(f, "- [Chapter 1](./chapter_1.md)")?;

            let chapter_1 = src_dir.join("chapter_1.md");
            let mut f =
                File::create(&chapter_1).with_context(|| "Unable to create chapter_1.md")?;
            writeln!(f, "# Chapter 1")?;
        } else {
            trace!("Existing summary found, no need to create stub files.");
        }
        Ok(())
    }

    fn create_directory_structure(&self) -> Result<()> {
        debug!("Creating directory tree at {}", self.root.display());
        fs::create_dir_all(&self.root)?;

        let src = self.source_dir();
        fs::create_dir_all(&src)?;

        let build = self.root.join(&self.config.build.build_dir);
        fs::create_dir_all(&build)?;

        Ok(())
    }
}
