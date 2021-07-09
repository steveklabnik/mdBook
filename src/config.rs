//! Mdbook's configuration system.
//!
//! The main entrypoint of the `config` module is the `Config` struct. This acts
//! essentially as a bag of configuration information, with a couple
//! pre-determined tables ([`BookConfig`] and [`BuildConfig`]) as well as support
//! for arbitrary data which is exposed to plugins and alternative backends.
//!
//!
//! # Examples
//!
//! ```rust
//! # use mdbook::errors::*;
//! use std::path::PathBuf;
//! use std::str::FromStr;
//! use mdbook::Config;
//! use toml::Value;
//!
//! # fn run() -> Result<()> {
//! let src = r#"
//! [book]
//! title = "My Book"
//! authors = ["Michael-F-Bryan"]
//!
//! [build]
//! src = "out"
//!
//! [other-table.foo]
//! bar = 123
//! "#;
//!
//! // load the `Config` from a toml string
//! let mut cfg = Config::from_str(src)?;
//!
//! // retrieve a nested value
//! let bar = cfg.get("other-table.foo.bar").cloned();
//! assert_eq!(bar, Some(Value::Integer(123)));
//!
//! // Set the `output.html.theme` directory
//! assert!(cfg.get("output.html").is_none());
//! cfg.set("output.html.theme", "./themes");
//!
//! // then load it again, automatically deserializing to a `PathBuf`.
//! let got: Option<PathBuf> = cfg.get_deserialized_opt("output.html.theme")?;
//! assert_eq!(got, Some(PathBuf::from("./themes")));
//! # Ok(())
//! # }
//! # run().unwrap()
//! ```

#![deny(missing_docs)]

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use toml::value::Table;
use toml::{self, Value};

use crate::errors::*;
use crate::utils::{self, toml_ext::TomlExt};

/// The overall configuration object for MDBook, essentially an in-memory
/// representation of `book.toml`.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// Metadata about the book.
    pub book: BookConfig,
    /// Information about the build environment.
    pub build: BuildConfig,
    /// Information about Rust language support.
    pub rust: RustConfig,
    rest: Value,
}

impl FromStr for Config {
    type Err = Error;

    /// Load a `Config` from some string.
    fn from_str(src: &str) -> Result<Self> {
        toml::from_str(src).with_context(|| "Invalid configuration file")
    }
}

impl Config {
    /// Load the configuration file from disk.
    pub fn from_disk<P: AsRef<Path>>(config_file: P) -> Result<Config> {
        let mut buffer = String::new();
        File::open(config_file)
            .with_context(|| "Unable to open the configuration file")?
            .read_to_string(&mut buffer)
            .with_context(|| "Couldn't read the file")?;

        Config::from_str(&buffer)
    }

    /// Updates the `Config` from the available environment variables.
    ///
    /// Variables starting with `MDBOOK_` are used for configuration. The key is
    /// created by removing the `MDBOOK_` prefix and turning the resulting
    /// string into `kebab-case`. Double underscores (`__`) separate nested
    /// keys, while a single underscore (`_`) is replaced with a dash (`-`).
    ///
    /// For example:
    ///
    /// - `MDBOOK_foo` -> `foo`
    /// - `MDBOOK_FOO` -> `foo`
    /// - `MDBOOK_FOO__BAR` -> `foo.bar`
    /// - `MDBOOK_FOO_BAR` -> `foo-bar`
    /// - `MDBOOK_FOO_bar__baz` -> `foo-bar.baz`
    ///
    /// So by setting the `MDBOOK_BOOK__TITLE` environment variable you can
    /// override the book's title without needing to touch your `book.toml`.
    ///
    /// > **Note:** To facilitate setting more complex config items, the value
    /// > of an environment variable is first parsed as JSON, falling back to a
    /// > string if the parse fails.
    /// >
    /// > This means, if you so desired, you could override all book metadata
    /// > when building the book with something like
    /// >
    /// > ```text
    /// > $ export MDBOOK_BOOK='{"title": "My Awesome Book", "authors": ["Michael-F-Bryan"]}'
    /// > $ mdbook build
    /// > ```
    ///
    /// The latter case may be useful in situations where `mdbook` is invoked
    /// from a script or CI, where it sometimes isn't possible to update the
    /// `book.toml` before building.
    pub fn update_from_env(&mut self) {
        debug!("Updating the config from environment variables");

        let overrides =
            env::vars().filter_map(|(key, value)| parse_env(&key).map(|index| (index, value)));

        for (key, value) in overrides {
            trace!("{} => {}", key, value);
            let parsed_value = serde_json::from_str(&value)
                .unwrap_or_else(|_| serde_json::Value::String(value.to_string()));

            if key == "book" || key == "build" {
                if let serde_json::Value::Object(ref map) = parsed_value {
                    // To `set` each `key`, we wrap them as `prefix.key`
                    for (k, v) in map {
                        let full_key = format!("{}.{}", key, k);
                        self.set(&full_key, v).expect("unreachable");
                    }
                    return;
                }
            }

            self.set(key, parsed_value).expect("unreachable");
        }
    }

    /// Fetch an arbitrary item from the `Config` as a `toml::Value`.
    ///
    /// You can use dotted indices to access nested items (e.g.
    /// `output.html.playground` will fetch the "playground" out of the html output
    /// table).
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.rest.read(key)
    }

    /// Fetch a value from the `Config` so you can mutate it.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.rest.read_mut(key)
    }

    /// Convenience method for getting the html renderer's configuration.
    ///
    /// # Note
    ///
    /// This is for compatibility only. It will be removed completely once the
    /// HTML renderer is refactored to be less coupled to `mdbook` internals.
    #[doc(hidden)]
    pub fn html_config(&self) -> Option<HtmlConfig> {
        match self
            .get_deserialized_opt("output.html")
            .with_context(|| "Parsing configuration [output.html]")
        {
            Ok(Some(config)) => Some(config),
            Ok(None) => None,
            Err(e) => {
                utils::log_backtrace(&e);
                None
            }
        }
    }

    /// Deprecated, use get_deserialized_opt instead.
    #[deprecated = "use get_deserialized_opt instead"]
    pub fn get_deserialized<'de, T: Deserialize<'de>, S: AsRef<str>>(&self, name: S) -> Result<T> {
        let name = name.as_ref();
        match self.get_deserialized_opt(name)? {
            Some(value) => Ok(value),
            None => bail!("Key not found, {:?}", name),
        }
    }

    /// Convenience function to fetch a value from the config and deserialize it
    /// into some arbitrary type.
    pub fn get_deserialized_opt<'de, T: Deserialize<'de>, S: AsRef<str>>(
        &self,
        name: S,
    ) -> Result<Option<T>> {
        let name = name.as_ref();
        self.get(name)
            .map(|value| {
                value
                    .clone()
                    .try_into()
                    .with_context(|| "Couldn't deserialize the value")
            })
            .transpose()
    }

    /// Set a config key, clobbering any existing values along the way.
    ///
    /// The only way this can fail is if we can't serialize `value` into a
    /// `toml::Value`.
    pub fn set<S: Serialize, I: AsRef<str>>(&mut self, index: I, value: S) -> Result<()> {
        let index = index.as_ref();

        let value = Value::try_from(value)
            .with_context(|| "Unable to represent the item as a JSON Value")?;

        if index.starts_with("book.") {
            self.book.update_value(&index[5..], value);
        } else if index.starts_with("build.") {
            self.build.update_value(&index[6..], value);
        } else {
            self.rest.insert(index, value);
        }

        Ok(())
    }

    /// Get the table associated with a particular renderer.
    pub fn get_renderer<I: AsRef<str>>(&self, index: I) -> Option<&Table> {
        let key = format!("output.{}", index.as_ref());
        self.get(&key).and_then(Value::as_table)
    }

    /// Get the table associated with a particular preprocessor.
    pub fn get_preprocessor<I: AsRef<str>>(&self, index: I) -> Option<&Table> {
        let key = format!("preprocessor.{}", index.as_ref());
        self.get(&key).and_then(Value::as_table)
    }

    fn from_legacy(mut table: Value) -> Config {
        let mut cfg = Config::default();

        // we use a macro here instead of a normal loop because the $out
        // variable can be different types. This way we can make type inference
        // figure out what try_into() deserializes to.
        macro_rules! get_and_insert {
            ($table:expr, $key:expr => $out:expr) => {
                let got = $table
                    .as_table_mut()
                    .and_then(|t| t.remove($key))
                    .and_then(|v| v.try_into().ok());
                if let Some(value) = got {
                    $out = value;
                }
            };
        }

        get_and_insert!(table, "title" => cfg.book.title);
        get_and_insert!(table, "authors" => cfg.book.authors);
        get_and_insert!(table, "source" => cfg.book.src);
        get_and_insert!(table, "description" => cfg.book.description);

        if let Some(dest) = table.delete("output.html.destination") {
            if let Ok(destination) = dest.try_into() {
                cfg.build.build_dir = destination;
            }
        }

        cfg.rest = table;
        cfg
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            book: BookConfig::default(),
            build: BuildConfig::default(),
            rust: RustConfig::default(),
            rest: Value::Table(Table::default()),
        }
    }
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D: Deserializer<'de>>(de: D) -> std::result::Result<Self, D::Error> {
        let raw = Value::deserialize(de)?;

        if is_legacy_format(&raw) {
            warn!("It looks like you are using the legacy book.toml format.");
            warn!("We'll parse it for now, but you should probably convert to the new format.");
            warn!("See the mdbook documentation for more details, although as a rule of thumb");
            warn!("just move all top level configuration entries like `title`, `author` and");
            warn!("`description` under a table called `[book]`, move the `destination` entry");
            warn!("from `[output.html]`, renamed to `build-dir`, under a table called");
            warn!("`[build]`, and it should all work.");
            warn!("Documentation: http://rust-lang.github.io/mdBook/format/config.html");
            return Ok(Config::from_legacy(raw));
        }

        use serde::de::Error;
        let mut table = match raw {
            Value::Table(t) => t,
            _ => {
                return Err(D::Error::custom(
                    "A config file should always be a toml table",
                ));
            }
        };

        let book: BookConfig = table
            .remove("book")
            .map(|book| book.try_into().map_err(D::Error::custom))
            .transpose()?
            .unwrap_or_default();

        let build: BuildConfig = table
            .remove("build")
            .map(|build| build.try_into().map_err(D::Error::custom))
            .transpose()?
            .unwrap_or_default();

        let rust: RustConfig = table
            .remove("rust")
            .map(|rust| rust.try_into().map_err(D::Error::custom))
            .transpose()?
            .unwrap_or_default();

        Ok(Config {
            book,
            build,
            rust,
            rest: Value::Table(table),
        })
    }
}

impl Serialize for Config {
    fn serialize<S: Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        // TODO: This should probably be removed and use a derive instead.
        let mut table = self.rest.clone();

        let book_config = Value::try_from(&self.book).expect("should always be serializable");
        table.insert("book", book_config);

        if self.build != BuildConfig::default() {
            let build_config = Value::try_from(&self.build).expect("should always be serializable");
            table.insert("build", build_config);
        }

        if self.rust != RustConfig::default() {
            let rust_config = Value::try_from(&self.rust).expect("should always be serializable");
            table.insert("rust", rust_config);
        }

        table.serialize(s)
    }
}

fn parse_env(key: &str) -> Option<String> {
    const PREFIX: &str = "MDBOOK_";

    if key.starts_with(PREFIX) {
        let key = &key[PREFIX.len()..];

        Some(key.to_lowercase().replace("__", ".").replace("_", "-"))
    } else {
        None
    }
}

fn is_legacy_format(table: &Value) -> bool {
    let legacy_items = [
        "title",
        "authors",
        "source",
        "description",
        "output.html.destination",
    ];

    for item in &legacy_items {
        if table.read(item).is_some() {
            return true;
        }
    }

    false
}

/// Configuration options which are specific to the book and required for
/// loading it from disk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BookConfig {
    /// The book's title.
    pub title: Option<String>,
    /// The book's authors.
    pub authors: Vec<String>,
    /// An optional description for the book.
    pub description: Option<String>,
    /// Location of the book source relative to the book's root directory.
    pub src: PathBuf,
    /// Does this book support more than one language?
    pub multilingual: bool,
    /// The main language of the book.
    pub language: Option<String>,
}

impl Default for BookConfig {
    fn default() -> BookConfig {
        BookConfig {
            title: None,
            authors: Vec::new(),
            description: None,
            src: PathBuf::from("src"),
            multilingual: false,
            language: Some(String::from("en")),
        }
    }
}

/// Configuration for the build procedure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BuildConfig {
    /// Where to put built artefacts relative to the book's root directory.
    pub build_dir: PathBuf,
    /// Should non-existent markdown files specified in `SUMMARY.md` be created
    /// if they don't exist?
    pub create_missing: bool,
    /// Should the default preprocessors always be used when they are
    /// compatible with the renderer?
    pub use_default_preprocessors: bool,
}

impl Default for BuildConfig {
    fn default() -> BuildConfig {
        BuildConfig {
            build_dir: PathBuf::from("book"),
            create_missing: true,
            use_default_preprocessors: true,
        }
    }
}

/// Configuration for the Rust compiler(e.g., for playground)
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct RustConfig {
    /// Rust edition used in playground
    pub edition: Option<RustEdition>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
/// Rust edition to use for the code.
pub enum RustEdition {
    /// The 2021 edition of Rust
    #[serde(rename = "2021")]
    E2021,
    /// The 2018 edition of Rust
    #[serde(rename = "2018")]
    E2018,
    /// The 2015 edition of Rust
    #[serde(rename = "2015")]
    E2015,
}

/// Configuration for the HTML renderer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmlConfig {
    /// The theme directory, if specified.
    pub theme: Option<PathBuf>,
    /// The default theme to use, defaults to 'light'
    pub default_theme: Option<String>,
    /// The theme to use if the browser requests the dark version of the site.
    /// Defaults to 'navy'.
    pub preferred_dark_theme: Option<String>,
    /// Use "smart quotes" instead of the usual `"` character.
    pub curly_quotes: bool,
    /// Should mathjax be enabled?
    pub mathjax_support: bool,
    /// Whether to fonts.css and respective font files to the output directory.
    pub copy_fonts: bool,
    /// An optional google analytics code.
    pub google_analytics: Option<String>,
    /// Additional CSS stylesheets to include in the rendered page's `<head>`.
    pub additional_css: Vec<PathBuf>,
    /// Additional JS scripts to include at the bottom of the rendered page's
    /// `<body>`.
    pub additional_js: Vec<PathBuf>,
    /// Fold settings.
    pub fold: Fold,
    /// Playground settings.
    #[serde(alias = "playpen")]
    pub playground: Playground,
    /// Print settings.
    pub print: Print,
    /// Don't render section labels.
    pub no_section_label: bool,
    /// Search settings. If `None`, the default will be used.
    pub search: Option<Search>,
    /// Git repository url. If `None`, the git button will not be shown.
    pub git_repository_url: Option<String>,
    /// FontAwesome icon class to use for the Git repository link.
    /// Defaults to `fa-github` if `None`.
    pub git_repository_icon: Option<String>,
    /// Input path for the 404 file, defaults to 404.md, set to "" to disable 404 file output
    pub input_404: Option<String>,
    /// Absolute url to site, used to emit correct paths for the 404 page, which might be accessed in a deeply nested directory
    pub site_url: Option<String>,
    /// The DNS subdomain or apex domain at which your book will be hosted. This
    /// string will be written to a file named CNAME in the root of your site,
    /// as required by GitHub Pages (see [*Managing a custom domain for your
    /// GitHub Pages site*][custom domain]).
    ///
    /// [custom domain]: https://docs.github.com/en/github/working-with-github-pages/managing-a-custom-domain-for-your-github-pages-site
    pub cname: Option<String>,
    /// Edit url template, when set shows a "Suggest an edit" button for
    /// directly jumping to editing the currently viewed page.
    /// Contains {path} that is replaced with chapter source file path
    pub edit_url_template: Option<String>,
    /// This is used as a bit of a workaround for the `mdbook serve` command.
    /// Basically, because you set the websocket port from the command line, the
    /// `mdbook serve` command needs a way to let the HTML renderer know where
    /// to point livereloading at, if it has been enabled.
    ///
    /// This config item *should not be edited* by the end user.
    #[doc(hidden)]
    pub livereload_url: Option<String>,
    /// The mapping from old pages to new pages/URLs to use when generating
    /// redirects.
    pub redirect: HashMap<String, String>,
}

impl Default for HtmlConfig {
    fn default() -> HtmlConfig {
        HtmlConfig {
            theme: None,
            default_theme: None,
            preferred_dark_theme: None,
            curly_quotes: false,
            mathjax_support: false,
            copy_fonts: true,
            google_analytics: None,
            additional_css: Vec::new(),
            additional_js: Vec::new(),
            fold: Fold::default(),
            playground: Playground::default(),
            print: Print::default(),
            no_section_label: false,
            search: None,
            git_repository_url: None,
            git_repository_icon: None,
            edit_url_template: None,
            input_404: None,
            site_url: None,
            cname: None,
            livereload_url: None,
            redirect: HashMap::new(),
        }
    }
}

impl HtmlConfig {
    /// Returns the directory of theme from the provided root directory. If the
    /// directory is not present it will append the default directory of "theme"
    pub fn theme_dir(&self, root: &Path) -> PathBuf {
        match self.theme {
            Some(ref d) => root.join(d),
            None => root.join("theme"),
        }
    }
}

/// Configuration for how to render the print icon, print.html, and print.css.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Print {
    /// Whether print support is enabled.
    pub enable: bool,
}

impl Default for Print {
    fn default() -> Self {
        Self { enable: true }
    }
}

/// Configuration for how to fold chapters of sidebar.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Fold {
    /// When off, all folds are open. Default: `false`.
    pub enable: bool,
    /// The higher the more folded regions are open. When level is 0, all folds
    /// are closed.
    /// Default: `0`.
    pub level: u8,
}

/// Configuration for tweaking how the the HTML renderer handles the playground.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Playground {
    /// Should playground snippets be editable? Default: `false`.
    pub editable: bool,
    /// Display the copy button. Default: `true`.
    pub copyable: bool,
    /// Copy JavaScript files for the editor to the output directory?
    /// Default: `true`.
    pub copy_js: bool,
    /// Display line numbers on playground snippets. Default: `false`.
    pub line_numbers: bool,
}

impl Default for Playground {
    fn default() -> Playground {
        Playground {
            editable: false,
            copyable: true,
            copy_js: true,
            line_numbers: false,
        }
    }
}

/// Configuration of the search functionality of the HTML renderer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Search {
    /// Enable the search feature. Default: `true`.
    pub enable: bool,
    /// Maximum number of visible results. Default: `30`.
    pub limit_results: u32,
    /// The number of words used for a search result teaser. Default: `30`.
    pub teaser_word_count: u32,
    /// Define the logical link between multiple search words.
    /// If true, all search words must appear in each result. Default: `false`.
    pub use_boolean_and: bool,
    /// Boost factor for the search result score if a search word appears in the header.
    /// Default: `2`.
    pub boost_title: u8,
    /// Boost factor for the search result score if a search word appears in the hierarchy.
    /// The hierarchy contains all titles of the parent documents and all parent headings.
    /// Default: `1`.
    pub boost_hierarchy: u8,
    /// Boost factor for the search result score if a search word appears in the text.
    /// Default: `1`.
    pub boost_paragraph: u8,
    /// True if the searchword `micro` should match `microwave`. Default: `true`.
    pub expand: bool,
    /// Documents are split into smaller parts, separated by headings. This defines, until which
    /// level of heading documents should be split. Default: `3`. (`### This is a level 3 heading`)
    pub heading_split_level: u8,
    /// Copy JavaScript files for the search functionality to the output directory?
    /// Default: `true`.
    pub copy_js: bool,
}

impl Default for Search {
    fn default() -> Search {
        // Please update the documentation of `Search` when changing values!
        Search {
            enable: true,
            limit_results: 30,
            teaser_word_count: 30,
            use_boolean_and: false,
            boost_title: 2,
            boost_hierarchy: 1,
            boost_paragraph: 1,
            expand: true,
            heading_split_level: 3,
            copy_js: true,
        }
    }
}

/// Allows you to "update" any arbitrary field in a struct by round-tripping via
/// a `toml::Value`.
///
/// This is definitely not the most performant way to do things, which means you
/// should probably keep it away from tight loops...
trait Updateable<'de>: Serialize + Deserialize<'de> {
    fn update_value<S: Serialize>(&mut self, key: &str, value: S) {
        let mut raw = Value::try_from(&self).expect("unreachable");

        if let Ok(value) = Value::try_from(value) {
            let _ = raw.insert(key, value);
        } else {
            return;
        }

        if let Ok(updated) = raw.try_into() {
            *self = updated;
        }
    }
}

impl<'de, T> Updateable<'de> for T where T: Serialize + Deserialize<'de> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fs::get_404_output_file;

    const COMPLEX_CONFIG: &str = r#"
        [book]
        title = "Some Book"
        authors = ["Michael-F-Bryan <michaelfbryan@gmail.com>"]
        description = "A completely useless book"
        multilingual = true
        src = "source"
        language = "ja"

        [build]
        build-dir = "outputs"
        create-missing = false
        use-default-preprocessors = true

        [output.html]
        theme = "./themedir"
        default-theme = "rust"
        curly-quotes = true
        google-analytics = "123456"
        additional-css = ["./foo/bar/baz.css"]
        git-repository-url = "https://foo.com/"
        git-repository-icon = "fa-code-fork"

        [output.html.playground]
        editable = true
        editor = "ace"

        [output.html.redirect]
        "index.html" = "overview.html"
        "nexted/page.md" = "https://rust-lang.org/"

        [preprocessor.first]

        [preprocessor.second]
        "#;

    #[test]
    fn load_a_complex_config_file() {
        let src = COMPLEX_CONFIG;

        let book_should_be = BookConfig {
            title: Some(String::from("Some Book")),
            authors: vec![String::from("Michael-F-Bryan <michaelfbryan@gmail.com>")],
            description: Some(String::from("A completely useless book")),
            multilingual: true,
            src: PathBuf::from("source"),
            language: Some(String::from("ja")),
        };
        let build_should_be = BuildConfig {
            build_dir: PathBuf::from("outputs"),
            create_missing: false,
            use_default_preprocessors: true,
        };
        let rust_should_be = RustConfig { edition: None };
        let playground_should_be = Playground {
            editable: true,
            copyable: true,
            copy_js: true,
            line_numbers: false,
        };
        let html_should_be = HtmlConfig {
            curly_quotes: true,
            google_analytics: Some(String::from("123456")),
            additional_css: vec![PathBuf::from("./foo/bar/baz.css")],
            theme: Some(PathBuf::from("./themedir")),
            default_theme: Some(String::from("rust")),
            playground: playground_should_be,
            git_repository_url: Some(String::from("https://foo.com/")),
            git_repository_icon: Some(String::from("fa-code-fork")),
            redirect: vec![
                (String::from("index.html"), String::from("overview.html")),
                (
                    String::from("nexted/page.md"),
                    String::from("https://rust-lang.org/"),
                ),
            ]
            .into_iter()
            .collect(),
            ..Default::default()
        };

        let got = Config::from_str(src).unwrap();

        assert_eq!(got.book, book_should_be);
        assert_eq!(got.build, build_should_be);
        assert_eq!(got.rust, rust_should_be);
        assert_eq!(got.html_config().unwrap(), html_should_be);
    }

    #[test]
    fn edition_2015() {
        let src = r#"
        [book]
        title = "mdBook Documentation"
        description = "Create book from markdown files. Like Gitbook but implemented in Rust"
        authors = ["Mathieu David"]
        src = "./source"
        [rust]
        edition = "2015"
        "#;

        let book_should_be = BookConfig {
            title: Some(String::from("mdBook Documentation")),
            description: Some(String::from(
                "Create book from markdown files. Like Gitbook but implemented in Rust",
            )),
            authors: vec![String::from("Mathieu David")],
            src: PathBuf::from("./source"),
            ..Default::default()
        };

        let got = Config::from_str(src).unwrap();
        assert_eq!(got.book, book_should_be);

        let rust_should_be = RustConfig {
            edition: Some(RustEdition::E2015),
        };
        let got = Config::from_str(src).unwrap();
        assert_eq!(got.rust, rust_should_be);
    }

    #[test]
    fn edition_2018() {
        let src = r#"
        [book]
        title = "mdBook Documentation"
        description = "Create book from markdown files. Like Gitbook but implemented in Rust"
        authors = ["Mathieu David"]
        src = "./source"
        [rust]
        edition = "2018"
        "#;

        let rust_should_be = RustConfig {
            edition: Some(RustEdition::E2018),
        };

        let got = Config::from_str(src).unwrap();
        assert_eq!(got.rust, rust_should_be);
    }

    #[test]
    fn edition_2021() {
        let src = r#"
        [book]
        title = "mdBook Documentation"
        description = "Create book from markdown files. Like Gitbook but implemented in Rust"
        authors = ["Mathieu David"]
        src = "./source"
        [rust]
        edition = "2021"
        "#;

        let rust_should_be = RustConfig {
            edition: Some(RustEdition::E2021),
        };

        let got = Config::from_str(src).unwrap();
        assert_eq!(got.rust, rust_should_be);
    }

    #[test]
    fn load_arbitrary_output_type() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct RandomOutput {
            foo: u32,
            bar: String,
            baz: Vec<bool>,
        }

        let src = r#"
        [output.random]
        foo = 5
        bar = "Hello World"
        baz = [true, true, false]
        "#;

        let should_be = RandomOutput {
            foo: 5,
            bar: String::from("Hello World"),
            baz: vec![true, true, false],
        };

        let cfg = Config::from_str(src).unwrap();
        let got: RandomOutput = cfg.get_deserialized_opt("output.random").unwrap().unwrap();

        assert_eq!(got, should_be);

        let got_baz: Vec<bool> = cfg
            .get_deserialized_opt("output.random.baz")
            .unwrap()
            .unwrap();
        let baz_should_be = vec![true, true, false];

        assert_eq!(got_baz, baz_should_be);
    }

    #[test]
    fn mutate_some_stuff() {
        // really this is just a sanity check to make sure the borrow checker
        // is happy...
        let src = COMPLEX_CONFIG;
        let mut config = Config::from_str(src).unwrap();
        let key = "output.html.playground.editable";

        assert_eq!(config.get(key).unwrap(), &Value::Boolean(true));
        *config.get_mut(key).unwrap() = Value::Boolean(false);
        assert_eq!(config.get(key).unwrap(), &Value::Boolean(false));
    }

    /// The config file format has slightly changed (metadata stuff is now under
    /// the `book` table instead of being at the top level) so we're adding a
    /// **temporary** compatibility check. You should be able to still load the
    /// old format, emitting a warning.
    #[test]
    fn can_still_load_the_previous_format() {
        let src = r#"
        title = "mdBook Documentation"
        description = "Create book from markdown files. Like Gitbook but implemented in Rust"
        authors = ["Mathieu David"]
        source = "./source"

        [output.html]
        destination = "my-book" # the output files will be generated in `root/my-book` instead of `root/book`
        theme = "my-theme"
        curly-quotes = true
        google-analytics = "123456"
        additional-css = ["custom.css", "custom2.css"]
        additional-js = ["custom.js"]
        "#;

        let book_should_be = BookConfig {
            title: Some(String::from("mdBook Documentation")),
            description: Some(String::from(
                "Create book from markdown files. Like Gitbook but implemented in Rust",
            )),
            authors: vec![String::from("Mathieu David")],
            src: PathBuf::from("./source"),
            ..Default::default()
        };

        let build_should_be = BuildConfig {
            build_dir: PathBuf::from("my-book"),
            create_missing: true,
            use_default_preprocessors: true,
        };

        let html_should_be = HtmlConfig {
            theme: Some(PathBuf::from("my-theme")),
            curly_quotes: true,
            google_analytics: Some(String::from("123456")),
            additional_css: vec![PathBuf::from("custom.css"), PathBuf::from("custom2.css")],
            additional_js: vec![PathBuf::from("custom.js")],
            ..Default::default()
        };

        let got = Config::from_str(src).unwrap();
        assert_eq!(got.book, book_should_be);
        assert_eq!(got.build, build_should_be);
        assert_eq!(got.html_config().unwrap(), html_should_be);
    }

    #[test]
    fn set_a_config_item() {
        let mut cfg = Config::default();
        let key = "foo.bar.baz";
        let value = "Something Interesting";

        assert!(cfg.get(key).is_none());
        cfg.set(key, value).unwrap();

        let got: String = cfg.get_deserialized_opt(key).unwrap().unwrap();
        assert_eq!(got, value);
    }

    #[test]
    fn parse_env_vars() {
        let inputs = vec![
            ("FOO", None),
            ("MDBOOK_foo", Some("foo")),
            ("MDBOOK_FOO__bar__baz", Some("foo.bar.baz")),
            ("MDBOOK_FOO_bar__baz", Some("foo-bar.baz")),
        ];

        for (src, should_be) in inputs {
            let got = parse_env(src);
            let should_be = should_be.map(ToString::to_string);

            assert_eq!(got, should_be);
        }
    }

    fn encode_env_var(key: &str) -> String {
        format!(
            "MDBOOK_{}",
            key.to_uppercase().replace('.', "__").replace("-", "_")
        )
    }

    #[test]
    fn update_config_using_env_var() {
        let mut cfg = Config::default();
        let key = "foo.bar";
        let value = "baz";

        assert!(cfg.get(key).is_none());

        let encoded_key = encode_env_var(key);
        env::set_var(encoded_key, value);

        cfg.update_from_env();

        assert_eq!(
            cfg.get_deserialized_opt::<String, _>(key).unwrap().unwrap(),
            value
        );
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn update_config_using_env_var_and_complex_value() {
        let mut cfg = Config::default();
        let key = "foo-bar.baz";
        let value = json!({"array": [1, 2, 3], "number": 3.14});
        let value_str = serde_json::to_string(&value).unwrap();

        assert!(cfg.get(key).is_none());

        let encoded_key = encode_env_var(key);
        env::set_var(encoded_key, value_str);

        cfg.update_from_env();

        assert_eq!(
            cfg.get_deserialized_opt::<serde_json::Value, _>(key)
                .unwrap()
                .unwrap(),
            value
        );
    }

    #[test]
    fn update_book_title_via_env() {
        let mut cfg = Config::default();
        let should_be = "Something else".to_string();

        assert_ne!(cfg.book.title, Some(should_be.clone()));

        env::set_var("MDBOOK_BOOK__TITLE", &should_be);
        cfg.update_from_env();

        assert_eq!(cfg.book.title, Some(should_be));
    }

    #[test]
    fn file_404_default() {
        let src = r#"
        [output.html]
        destination = "my-book"
        "#;

        let got = Config::from_str(src).unwrap();
        let html_config = got.html_config().unwrap();
        assert_eq!(html_config.input_404, None);
        assert_eq!(&get_404_output_file(&html_config.input_404), "404.html");
    }

    #[test]
    fn file_404_custom() {
        let src = r#"
        [output.html]
        input-404= "missing.md"
        output-404= "missing.html"
        "#;

        let got = Config::from_str(src).unwrap();
        let html_config = got.html_config().unwrap();
        assert_eq!(html_config.input_404, Some("missing.md".to_string()));
        assert_eq!(&get_404_output_file(&html_config.input_404), "missing.html");
    }

    #[test]
    #[should_panic(expected = "Invalid configuration file")]
    fn invalid_language_type_error() {
        let src = r#"
        [book]
        title = "mdBook Documentation"
        language = ["en", "pt-br"]
        description = "Create book from markdown files. Like Gitbook but implemented in Rust"
        authors = ["Mathieu David"]
        src = "./source"
        "#;

        Config::from_str(src).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid configuration file")]
    fn invalid_title_type() {
        let src = r#"
        [book]
        title = 20
        language = "en"
        description = "Create book from markdown files. Like Gitbook but implemented in Rust"
        authors = ["Mathieu David"]
        src = "./source"
        "#;

        Config::from_str(src).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid configuration file")]
    fn invalid_build_dir_type() {
        let src = r#"
        [build]
        build-dir = 99
        create-missing = false
        "#;

        Config::from_str(src).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid configuration file")]
    fn invalid_rust_edition() {
        let src = r#"
        [rust]
        edition = "1999"
        "#;

        Config::from_str(src).unwrap();
    }
}
