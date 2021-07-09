//! The internal representation of a book and infrastructure for loading it from
//! disk and building it.
//!
//! For examples on using `MDBook`, consult the [top-level documentation][1].
//!
//! [1]: ../index.html

#[allow(clippy::module_inception)]
mod book;
mod init;
mod summary;

pub use self::book::{load_book, Book, BookItem, BookItems, Chapter};
pub use self::init::BookBuilder;
pub use self::summary::{parse_summary, Link, SectionNumber, Summary, SummaryItem};

use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::string::ToString;
use tempfile::Builder as TempFileBuilder;
use toml::Value;

use crate::errors::*;
use crate::preprocess::{
    CmdPreprocessor, IndexPreprocessor, LinkPreprocessor, Preprocessor, PreprocessorContext,
};
use crate::renderer::{CmdRenderer, HtmlHandlebars, MarkdownRenderer, RenderContext, Renderer};
use crate::utils;

use crate::config::{Config, RustEdition};

/// The object used to manage and build a book.
pub struct MDBook {
    /// The book's root directory.
    pub root: PathBuf,
    /// The configuration used to tweak now a book is built.
    pub config: Config,
    /// A representation of the book's contents in memory.
    pub book: Book,
    renderers: Vec<Box<dyn Renderer>>,

    /// List of pre-processors to be run on the book.
    preprocessors: Vec<Box<dyn Preprocessor>>,
}

impl MDBook {
    /// Load a book from its root directory on disk.
    pub fn load<P: Into<PathBuf>>(book_root: P) -> Result<MDBook> {
        let book_root = book_root.into();
        let config_location = book_root.join("book.toml");

        // the book.json file is no longer used, so we should emit a warning to
        // let people know to migrate to book.toml
        if book_root.join("book.json").exists() {
            warn!("It appears you are still using book.json for configuration.");
            warn!("This format is no longer used, so you should migrate to the");
            warn!("book.toml format.");
            warn!("Check the user guide for migration information:");
            warn!("\thttps://rust-lang.github.io/mdBook/format/config.html");
        }

        let mut config = if config_location.exists() {
            debug!("Loading config from {}", config_location.display());
            Config::from_disk(&config_location)?
        } else {
            Config::default()
        };

        config.update_from_env();

        if log_enabled!(log::Level::Trace) {
            for line in format!("Config: {:#?}", config).lines() {
                trace!("{}", line);
            }
        }

        MDBook::load_with_config(book_root, config)
    }

    /// Load a book from its root directory using a custom `Config`.
    pub fn load_with_config<P: Into<PathBuf>>(book_root: P, config: Config) -> Result<MDBook> {
        let root = book_root.into();

        let src_dir = root.join(&config.book.src);
        let book = book::load_book(&src_dir, &config.build)?;

        let renderers = determine_renderers(&config);
        let preprocessors = determine_preprocessors(&config)?;

        Ok(MDBook {
            root,
            config,
            book,
            renderers,
            preprocessors,
        })
    }

    /// Load a book from its root directory using a custom `Config` and a custom summary.
    pub fn load_with_config_and_summary<P: Into<PathBuf>>(
        book_root: P,
        config: Config,
        summary: Summary,
    ) -> Result<MDBook> {
        let root = book_root.into();

        let src_dir = root.join(&config.book.src);
        let book = book::load_book_from_disk(&summary, &src_dir)?;

        let renderers = determine_renderers(&config);
        let preprocessors = determine_preprocessors(&config)?;

        Ok(MDBook {
            root,
            config,
            book,
            renderers,
            preprocessors,
        })
    }

    /// Returns a flat depth-first iterator over the elements of the book,
    /// it returns a [`BookItem`] enum:
    /// `(section: String, bookitem: &BookItem)`
    ///
    /// ```no_run
    /// # use mdbook::MDBook;
    /// # use mdbook::book::BookItem;
    /// # let book = MDBook::load("mybook").unwrap();
    /// for item in book.iter() {
    ///     match *item {
    ///         BookItem::Chapter(ref chapter) => {},
    ///         BookItem::Separator => {},
    ///         BookItem::PartTitle(ref title) => {}
    ///     }
    /// }
    ///
    /// // would print something like this:
    /// // 1. Chapter 1
    /// // 1.1 Sub Chapter
    /// // 1.2 Sub Chapter
    /// // 2. Chapter 2
    /// //
    /// // etc.
    /// ```
    pub fn iter(&self) -> BookItems<'_> {
        self.book.iter()
    }

    /// `init()` gives you a `BookBuilder` which you can use to setup a new book
    /// and its accompanying directory structure.
    ///
    /// The `BookBuilder` creates some boilerplate files and directories to get
    /// you started with your book.
    ///
    /// ```text
    /// book-test/
    /// ├── book
    /// └── src
    ///     ├── chapter_1.md
    ///     └── SUMMARY.md
    /// ```
    ///
    /// It uses the path provided as the root directory for your book, then adds
    /// in a `src/` directory containing a `SUMMARY.md` and `chapter_1.md` file
    /// to get you started.
    pub fn init<P: Into<PathBuf>>(book_root: P) -> BookBuilder {
        BookBuilder::new(book_root)
    }

    /// Tells the renderer to build our book and put it in the build directory.
    pub fn build(&self) -> Result<()> {
        info!("Book building has started");

        for renderer in &self.renderers {
            self.execute_build_process(&**renderer)?;
        }

        Ok(())
    }

    /// Run the entire build process for a particular [`Renderer`].
    pub fn execute_build_process(&self, renderer: &dyn Renderer) -> Result<()> {
        let mut preprocessed_book = self.book.clone();
        let preprocess_ctx = PreprocessorContext::new(
            self.root.clone(),
            self.config.clone(),
            renderer.name().to_string(),
        );

        for preprocessor in &self.preprocessors {
            if preprocessor_should_run(&**preprocessor, renderer, &self.config) {
                debug!("Running the {} preprocessor.", preprocessor.name());
                preprocessed_book = preprocessor.run(&preprocess_ctx, preprocessed_book)?;
            }
        }

        let name = renderer.name();
        let build_dir = self.build_dir_for(name);

        let mut render_context = RenderContext::new(
            self.root.clone(),
            preprocessed_book,
            self.config.clone(),
            build_dir,
        );
        render_context
            .chapter_titles
            .extend(preprocess_ctx.chapter_titles.borrow_mut().drain());

        info!("Running the {} backend", renderer.name());
        renderer
            .render(&render_context)
            .with_context(|| "Rendering failed")
    }

    /// You can change the default renderer to another one by using this method.
    /// The only requirement is that your renderer implement the [`Renderer`]
    /// trait.
    pub fn with_renderer<R: Renderer + 'static>(&mut self, renderer: R) -> &mut Self {
        self.renderers.push(Box::new(renderer));
        self
    }

    /// Register a [`Preprocessor`] to be used when rendering the book.
    pub fn with_preprocessor<P: Preprocessor + 'static>(&mut self, preprocessor: P) -> &mut Self {
        self.preprocessors.push(Box::new(preprocessor));
        self
    }

    /// Run `rustdoc` tests on the book, linking against the provided libraries.
    pub fn test(&mut self, library_paths: Vec<&str>) -> Result<()> {
        let library_args: Vec<&str> = (0..library_paths.len())
            .map(|_| "-L")
            .zip(library_paths.into_iter())
            .flat_map(|x| vec![x.0, x.1])
            .collect();

        let temp_dir = TempFileBuilder::new().prefix("mdbook-").tempdir()?;

        // FIXME: Is "test" the proper renderer name to use here?
        let preprocess_context =
            PreprocessorContext::new(self.root.clone(), self.config.clone(), "test".to_string());

        let book = LinkPreprocessor::new().run(&preprocess_context, self.book.clone())?;
        // Index Preprocessor is disabled so that chapter paths continue to point to the
        // actual markdown files.

        let mut failed = false;
        for item in book.iter() {
            if let BookItem::Chapter(ref ch) = *item {
                let chapter_path = match ch.path {
                    Some(ref path) if !path.as_os_str().is_empty() => path,
                    _ => continue,
                };

                let path = self.source_dir().join(&chapter_path);
                info!("Testing file: {:?}", path);

                // write preprocessed file to tempdir
                let path = temp_dir.path().join(&chapter_path);
                let mut tmpf = utils::fs::create_file(&path)?;
                tmpf.write_all(ch.content.as_bytes())?;

                let mut cmd = Command::new("rustdoc");
                cmd.arg(&path).arg("--test").args(&library_args);

                if let Some(edition) = self.config.rust.edition {
                    match edition {
                        RustEdition::E2015 => {
                            cmd.args(&["--edition", "2015"]);
                        }
                        RustEdition::E2018 => {
                            cmd.args(&["--edition", "2018"]);
                        }
                        RustEdition::E2021 => {
                            cmd.args(&["--edition", "2021"])
                                .args(&["-Z", "unstable-options"]);
                        }
                    }
                }

                let output = cmd.output()?;

                if !output.status.success() {
                    failed = true;
                    error!(
                        "rustdoc returned an error:\n\
                        \n--- stdout\n{}\n--- stderr\n{}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        }
        if failed {
            bail!("One or more tests failed");
        }
        Ok(())
    }

    /// The logic for determining where a backend should put its build
    /// artefacts.
    ///
    /// If there is only 1 renderer, put it in the directory pointed to by the
    /// `build.build_dir` key in [`Config`]. If there is more than one then the
    /// renderer gets its own directory within the main build dir.
    ///
    /// i.e. If there were only one renderer (in this case, the HTML renderer):
    ///
    /// - build/
    ///   - index.html
    ///   - ...
    ///
    /// Otherwise if there are multiple:
    ///
    /// - build/
    ///   - epub/
    ///     - my_awesome_book.epub
    ///   - html/
    ///     - index.html
    ///     - ...
    ///   - latex/
    ///     - my_awesome_book.tex
    ///
    pub fn build_dir_for(&self, backend_name: &str) -> PathBuf {
        let build_dir = self.root.join(&self.config.build.build_dir);

        if self.renderers.len() <= 1 {
            build_dir
        } else {
            build_dir.join(backend_name)
        }
    }

    /// Get the directory containing this book's source files.
    pub fn source_dir(&self) -> PathBuf {
        self.root.join(&self.config.book.src)
    }

    /// Get the directory containing the theme resources for the book.
    pub fn theme_dir(&self) -> PathBuf {
        self.config
            .html_config()
            .unwrap_or_default()
            .theme_dir(&self.root)
    }
}

/// Look at the `Config` and try to figure out what renderers to use.
fn determine_renderers(config: &Config) -> Vec<Box<dyn Renderer>> {
    let mut renderers = Vec::new();

    if let Some(output_table) = config.get("output").and_then(Value::as_table) {
        renderers.extend(output_table.iter().map(|(key, table)| {
            if key == "html" {
                Box::new(HtmlHandlebars::new()) as Box<dyn Renderer>
            } else if key == "markdown" {
                Box::new(MarkdownRenderer::new()) as Box<dyn Renderer>
            } else {
                interpret_custom_renderer(key, table)
            }
        }));
    }

    // if we couldn't find anything, add the HTML renderer as a default
    if renderers.is_empty() {
        renderers.push(Box::new(HtmlHandlebars::new()));
    }

    renderers
}

fn default_preprocessors() -> Vec<Box<dyn Preprocessor>> {
    vec![
        Box::new(LinkPreprocessor::new()),
        Box::new(IndexPreprocessor::new()),
    ]
}

fn is_default_preprocessor(pre: &dyn Preprocessor) -> bool {
    let name = pre.name();
    name == LinkPreprocessor::NAME || name == IndexPreprocessor::NAME
}

/// Look at the `MDBook` and try to figure out what preprocessors to run.
fn determine_preprocessors(config: &Config) -> Result<Vec<Box<dyn Preprocessor>>> {
    let mut preprocessors = Vec::new();

    if config.build.use_default_preprocessors {
        preprocessors.extend(default_preprocessors());
    }

    if let Some(preprocessor_table) = config.get("preprocessor").and_then(Value::as_table) {
        for key in preprocessor_table.keys() {
            match key.as_ref() {
                "links" => preprocessors.push(Box::new(LinkPreprocessor::new())),
                "index" => preprocessors.push(Box::new(IndexPreprocessor::new())),
                name => preprocessors.push(interpret_custom_preprocessor(
                    name,
                    &preprocessor_table[name],
                )),
            }
        }
    }

    Ok(preprocessors)
}

fn interpret_custom_preprocessor(key: &str, table: &Value) -> Box<CmdPreprocessor> {
    let command = table
        .get("command")
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .unwrap_or_else(|| format!("mdbook-{}", key));

    Box::new(CmdPreprocessor::new(key.to_string(), command))
}

fn interpret_custom_renderer(key: &str, table: &Value) -> Box<CmdRenderer> {
    // look for the `command` field, falling back to using the key
    // prepended by "mdbook-"
    let table_dot_command = table
        .get("command")
        .and_then(Value::as_str)
        .map(ToString::to_string);

    let command = table_dot_command.unwrap_or_else(|| format!("mdbook-{}", key));

    Box::new(CmdRenderer::new(key.to_string(), command))
}

/// Check whether we should run a particular `Preprocessor` in combination
/// with the renderer, falling back to `Preprocessor::supports_renderer()`
/// method if the user doesn't say anything.
///
/// The `build.use-default-preprocessors` config option can be used to ensure
/// default preprocessors always run if they support the renderer.
fn preprocessor_should_run(
    preprocessor: &dyn Preprocessor,
    renderer: &dyn Renderer,
    cfg: &Config,
) -> bool {
    // default preprocessors should be run by default (if supported)
    if cfg.build.use_default_preprocessors && is_default_preprocessor(preprocessor) {
        return preprocessor.supports_renderer(renderer.name());
    }

    let key = format!("preprocessor.{}.renderers", preprocessor.name());
    let renderer_name = renderer.name();

    if let Some(Value::Array(ref explicit_renderers)) = cfg.get(&key) {
        return explicit_renderers
            .iter()
            .filter_map(Value::as_str)
            .any(|name| name == renderer_name);
    }

    preprocessor.supports_renderer(renderer_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use toml::value::{Table, Value};

    #[test]
    fn config_defaults_to_html_renderer_if_empty() {
        let cfg = Config::default();

        // make sure we haven't got anything in the `output` table
        assert!(cfg.get("output").is_none());

        let got = determine_renderers(&cfg);

        assert_eq!(got.len(), 1);
        assert_eq!(got[0].name(), "html");
    }

    #[test]
    fn add_a_random_renderer_to_the_config() {
        let mut cfg = Config::default();
        cfg.set("output.random", Table::new()).unwrap();

        let got = determine_renderers(&cfg);

        assert_eq!(got.len(), 1);
        assert_eq!(got[0].name(), "random");
    }

    #[test]
    fn add_a_random_renderer_with_custom_command_to_the_config() {
        let mut cfg = Config::default();

        let mut table = Table::new();
        table.insert("command".to_string(), Value::String("false".to_string()));
        cfg.set("output.random", table).unwrap();

        let got = determine_renderers(&cfg);

        assert_eq!(got.len(), 1);
        assert_eq!(got[0].name(), "random");
    }

    #[test]
    fn config_defaults_to_link_and_index_preprocessor_if_not_set() {
        let cfg = Config::default();

        // make sure we haven't got anything in the `preprocessor` table
        assert!(cfg.get("preprocessor").is_none());

        let got = determine_preprocessors(&cfg);

        assert!(got.is_ok());
        assert_eq!(got.as_ref().unwrap().len(), 2);
        assert_eq!(got.as_ref().unwrap()[0].name(), "links");
        assert_eq!(got.as_ref().unwrap()[1].name(), "index");
    }

    #[test]
    fn use_default_preprocessors_works() {
        let mut cfg = Config::default();
        cfg.build.use_default_preprocessors = false;

        let got = determine_preprocessors(&cfg).unwrap();

        assert_eq!(got.len(), 0);
    }

    #[test]
    fn can_determine_third_party_preprocessors() {
        let cfg_str = r#"
        [book]
        title = "Some Book"

        [preprocessor.random]

        [build]
        build-dir = "outputs"
        create-missing = false
        "#;

        let cfg = Config::from_str(cfg_str).unwrap();

        // make sure the `preprocessor.random` table exists
        assert!(cfg.get_preprocessor("random").is_some());

        let got = determine_preprocessors(&cfg).unwrap();

        assert!(got.into_iter().any(|p| p.name() == "random"));
    }

    #[test]
    fn preprocessors_can_provide_their_own_commands() {
        let cfg_str = r#"
        [preprocessor.random]
        command = "python random.py"
        "#;

        let cfg = Config::from_str(cfg_str).unwrap();

        // make sure the `preprocessor.random` table exists
        let random = cfg.get_preprocessor("random").unwrap();
        let random = interpret_custom_preprocessor("random", &Value::Table(random.clone()));

        assert_eq!(random.cmd(), "python random.py");
    }

    #[test]
    fn config_respects_preprocessor_selection() {
        let cfg_str = r#"
        [preprocessor.links]
        renderers = ["html"]
        "#;

        let cfg = Config::from_str(cfg_str).unwrap();

        // double-check that we can access preprocessor.links.renderers[0]
        let html = cfg
            .get_preprocessor("links")
            .and_then(|links| links.get("renderers"))
            .and_then(Value::as_array)
            .and_then(|renderers| renderers.get(0))
            .and_then(Value::as_str)
            .unwrap();
        assert_eq!(html, "html");
        let html_renderer = HtmlHandlebars::default();
        let pre = LinkPreprocessor::new();

        let should_run = preprocessor_should_run(&pre, &html_renderer, &cfg);
        assert!(should_run);
    }

    struct BoolPreprocessor(bool);
    impl Preprocessor for BoolPreprocessor {
        fn name(&self) -> &str {
            "bool-preprocessor"
        }

        fn run(&self, _ctx: &PreprocessorContext, _book: Book) -> Result<Book> {
            unimplemented!()
        }

        fn supports_renderer(&self, _renderer: &str) -> bool {
            self.0
        }
    }

    #[test]
    fn preprocessor_should_run_falls_back_to_supports_renderer_method() {
        let cfg = Config::default();
        let html = HtmlHandlebars::new();

        let should_be = true;
        let got = preprocessor_should_run(&BoolPreprocessor(should_be), &html, &cfg);
        assert_eq!(got, should_be);

        let should_be = false;
        let got = preprocessor_should_run(&BoolPreprocessor(should_be), &html, &cfg);
        assert_eq!(got, should_be);
    }
}
