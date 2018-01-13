//! Mdbook's configuration system.

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::env;
use toml::{self, Value};
use toml::value::Table;
use toml_query::read::TomlValueReadExt;
use toml_query::insert::TomlValueInsertExt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;

use errors::*;

/// The overall configuration object for MDBook.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// Metadata about the book.
    pub book: BookConfig,
    pub build: BuildConfig,
    rest: Value,
}

impl Config {
    /// Load a `Config` from some string.
    pub fn from_str(src: &str) -> Result<Config> {
        toml::from_str(src).chain_err(|| Error::from("Invalid configuration file"))
    }

    /// Load the configuration file from disk.
    pub fn from_disk<P: AsRef<Path>>(config_file: P) -> Result<Config> {
        let mut buffer = String::new();
        File::open(config_file)
            .chain_err(|| "Unable to open the configuration file")?
            .read_to_string(&mut buffer)
            .chain_err(|| "Couldn't read the file")?;

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
    /// > $ export MDBOOK_BOOK="{'title': 'My Awesome Book', authors: ['Michael-F-Bryan']}"
    /// > $ mdbook build
    /// > ```
    ///
    /// The latter case may be useful in situations where `mdbook` is invoked
    /// from a script or CI, where it sometimes isn't possible to update the
    /// `book.toml` before building.
    pub fn update_from_env(&mut self) {
        debug!("Updating the config from environment variables");

        let overrides = env::vars().filter_map(|(key, value)| match parse_env(&key) {
            Some(index) => Some((index, value)),
            None => None,
        });

        for (key, value) in overrides {
            trace!("{} => {}", key, value);
            let parsed_value = serde_json::from_str(&value)
                .unwrap_or_else(|_| serde_json::Value::String(value.to_string()));

            self.set(key, parsed_value).expect("unreachable");
        }
    }

    /// Fetch an arbitrary item from the `Config` as a `toml::Value`.
    ///
    /// You can use dotted indices to access nested items (e.g.
    /// `output.html.playpen` will fetch the "playpen" out of the html output
    /// table).
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self.rest.read(key) {
            Ok(inner) => inner,
            Err(_) => None,
        }
    }

    /// Fetch a value from the `Config` so you can mutate it.
    pub fn get_mut<'a>(&'a mut self, key: &str) -> Option<&'a mut Value> {
        match self.rest.read_mut(key) {
            Ok(inner) => inner,
            Err(_) => None,
        }
    }

    /// Convenience method for getting the html renderer's configuration.
    ///
    /// # Note
    ///
    /// This is for compatibility only. It will be removed completely once the
    /// HTML renderer is refactored to be less coupled to `mdbook` internals.
    #[doc(hidden)]
    pub fn html_config(&self) -> Option<HtmlConfig> {
        self.get_deserialized("output.html").ok()
    }

    /// Convenience function to fetch a value from the config and deserialize it
    /// into some arbitrary type.
    pub fn get_deserialized<'de, T: Deserialize<'de>, S: AsRef<str>>(&self, name: S) -> Result<T> {
        let name = name.as_ref();

        if let Some(value) = self.get(name) {
            value
                .clone()
                .try_into()
                .chain_err(|| "Couldn't deserialize the value")
        } else {
            bail!("Key not found, {:?}", name)
        }
    }

    /// Set a config key, clobbering any existing values along the way.
    ///
    /// The only way this can fail is if we can't serialize `value` into a
    /// `toml::Value`.
    pub fn set<S: Serialize, I: AsRef<str>>(&mut self, index: I, value: S) -> Result<()> {
        let index = index.as_ref();

        let value =
            Value::try_from(value).chain_err(|| "Unable to represent the item as a JSON Value")?;

        if index.starts_with("book.") {
            self.book.update_value(&index[5..], value);
        } else if index.starts_with("build.") {
            self.build.update_value(&index[6..], value);
        } else {
            self.rest.insert(index, value)?;
        }

        Ok(())
    }

    fn from_legacy(mut table: Table) -> Config {
        let mut cfg = Config::default();

        // we use a macro here instead of a normal loop because the $out
        // variable can be different types. This way we can make type inference
        // figure out what try_into() deserializes to.
        macro_rules! get_and_insert {
            ($table:expr, $key:expr => $out:expr) => {
                if let Some(value) = $table.remove($key).and_then(|v| v.try_into().ok()) {
                    $out = value;
                }
            };
        }

        get_and_insert!(table, "title" => cfg.book.title);
        get_and_insert!(table, "authors" => cfg.book.authors);
        get_and_insert!(table, "source" => cfg.book.src);
        get_and_insert!(table, "description" => cfg.book.description);

        // This complicated chain of and_then's is so we can move
        // "output.html.destination" to "build.build_dir" and parse it into a
        // PathBuf.
        let destination: Option<PathBuf> = table
            .get_mut("output")
            .and_then(|output| output.as_table_mut())
            .and_then(|output| output.get_mut("html"))
            .and_then(|html| html.as_table_mut())
            .and_then(|html| html.remove("destination"))
            .and_then(|dest| dest.try_into().ok());

        if let Some(dest) = destination {
            cfg.build.build_dir = dest;
        }

        cfg.rest = Value::Table(table);
        cfg
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            book: BookConfig::default(),
            build: BuildConfig::default(),
            rest: Value::Table(Table::default()),
        }
    }
}
impl<'de> Deserialize<'de> for Config {
    fn deserialize<D: Deserializer<'de>>(de: D) -> ::std::result::Result<Self, D::Error> {
        let raw = Value::deserialize(de)?;

        let mut table = match raw {
            Value::Table(t) => t,
            _ => {
                use serde::de::Error;
                return Err(D::Error::custom(
                    "A config file should always be a toml table",
                ));
            }
        };

        if is_legacy_format(&table) {
            warn!("It looks like you are using the legacy book.toml format.");
            warn!("We'll parse it for now, but you should probably convert to the new format.");
            warn!("See the mdbook documentation for more details, although as a rule of thumb");
            warn!("just move all top level configuration entries like `title`, `author` and");
            warn!("`description` under a table called `[book]`, move the `destination` entry");
            warn!("from `[output.html]`, renamed to `build-dir`, under a table called");
            warn!("`[build]`, and it should all work.");
            warn!("Documentation: http://rust-lang-nursery.github.io/mdBook/format/config.html");
            return Ok(Config::from_legacy(table));
        }

        let book: BookConfig = table
            .remove("book")
            .and_then(|value| value.try_into().ok())
            .unwrap_or_default();

        let build: BuildConfig = table
            .remove("build")
            .and_then(|value| value.try_into().ok())
            .unwrap_or_default();

        Ok(Config {
            book: book,
            build: build,
            rest: Value::Table(table),
        })
    }
}

impl Serialize for Config {
    fn serialize<S: Serializer>(&self, s: S) -> ::std::result::Result<S::Ok, S::Error> {
        use serde::ser::Error;

        let mut table = self.rest.clone();

        let book_config = match Value::try_from(self.book.clone()) {
            Ok(cfg) => cfg,
            Err(_) => {
                return Err(S::Error::custom("Unable to serialize the BookConfig"));
            }
        };

        table.insert("book", book_config).expect("unreachable");
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

fn is_legacy_format(table: &Table) -> bool {
    let top_level_items = ["title", "author", "authors"];

    top_level_items
        .iter()
        .any(|key| table.contains_key(&key.to_string()))
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
}

impl Default for BookConfig {
    fn default() -> BookConfig {
        BookConfig {
            title: None,
            authors: Vec::new(),
            description: None,
            src: PathBuf::from("src"),
            multilingual: false,
        }
    }
}

/// Configuration for the build procedure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BuildConfig {
    /// Where to put built artefacts relative to the book's root directory.
    pub build_dir: PathBuf,
    /// Should non-existent markdown files specified in `SETTINGS.md` be created
    /// if they don't exist?
    pub create_missing: bool,
}

impl Default for BuildConfig {
    fn default() -> BuildConfig {
        BuildConfig {
            build_dir: PathBuf::from("book"),
            create_missing: true,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct HtmlConfig {
    pub theme: Option<PathBuf>,
    pub curly_quotes: bool,
    pub mathjax_support: bool,
    pub google_analytics: Option<String>,
    pub additional_css: Vec<PathBuf>,
    pub additional_js: Vec<PathBuf>,
    pub playpen: Playpen,
    /// This is used as a bit of a workaround for the `mdbook serve` command.
    /// Basically, because you set the websocket port from the command line, the
    /// `mdbook serve` command needs a way to let the HTML renderer know where
    /// to point livereloading at, if it has been enabled.
    ///
    /// This config item *should not be edited* by the end user.
    #[doc(hidden)]
    pub livereload_url: Option<String>,
    pub no_section_label: bool,
}

/// Configuration for tweaking how the the HTML renderer handles the playpen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Playpen {
    pub editor: PathBuf,
    pub editable: bool,
}

impl Default for Playpen {
    fn default() -> Playpen {
        Playpen {
            editor: PathBuf::from("ace"),
            editable: false,
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

        {
            if let Ok(value) = Value::try_from(value) {
                let _ = raw.insert(key, value);
            } else {
                return;
            }
        }

        if let Ok(updated) = raw.try_into() {
            *self = updated;
        }
    }
}

impl<'de> Updateable<'de> for BookConfig {}
impl<'de> Updateable<'de> for BuildConfig {}

#[cfg(test)]
mod tests {
    use super::*;

    const COMPLEX_CONFIG: &'static str = r#"
        [book]
        title = "Some Book"
        authors = ["Michael-F-Bryan <michaelfbryan@gmail.com>"]
        description = "A completely useless book"
        multilingual = true
        src = "source"

        [build]
        build-dir = "outputs"
        create-missing = false

        [output.html]
        theme = "./themedir"
        curly-quotes = true
        google-analytics = "123456"
        additional-css = ["./foo/bar/baz.css"]

        [output.html.playpen]
        editable = true
        editor = "ace"
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
            ..Default::default()
        };
        let build_should_be = BuildConfig {
            build_dir: PathBuf::from("outputs"),
            create_missing: false,
        };
        let playpen_should_be = Playpen {
            editable: true,
            editor: PathBuf::from("ace"),
        };
        let html_should_be = HtmlConfig {
            curly_quotes: true,
            google_analytics: Some(String::from("123456")),
            additional_css: vec![PathBuf::from("./foo/bar/baz.css")],
            theme: Some(PathBuf::from("./themedir")),
            playpen: playpen_should_be,
            ..Default::default()
        };

        let got = Config::from_str(src).unwrap();

        assert_eq!(got.book, book_should_be);
        assert_eq!(got.build, build_should_be);
        assert_eq!(got.html_config().unwrap(), html_should_be);
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
        let got: RandomOutput = cfg.get_deserialized("output.random").unwrap();

        assert_eq!(got, should_be);

        let baz: Vec<bool> = cfg.get_deserialized("output.random.baz").unwrap();
        let baz_should_be = vec![true, true, false];

        assert_eq!(baz, baz_should_be);
    }

    #[test]
    fn mutate_some_stuff() {
        // really this is just a sanity check to make sure the borrow checker
        // is happy...
        let src = COMPLEX_CONFIG;
        let mut config = Config::from_str(src).unwrap();
        let key = "output.html.playpen.editable";

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

        let got: String = cfg.get_deserialized(key).unwrap();
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
            let should_be = should_be.map(|s| s.to_string());

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

        assert_eq!(cfg.get_deserialized::<String, _>(key).unwrap(), value);
    }

    #[test]
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
            cfg.get_deserialized::<serde_json::Value, _>(key).unwrap(),
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
}
