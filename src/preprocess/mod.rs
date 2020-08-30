//! Book preprocessing.

pub use self::cmd::CmdPreprocessor;
pub use self::index::IndexPreprocessor;
pub use self::links::LinkPreprocessor;

mod cmd;
mod index;
mod links;

use crate::book::Book;
use crate::build_opts::BuildOpts;
use crate::config::Config;
use crate::errors::*;

use std::path::PathBuf;

/// Extra information for a `Preprocessor` to give them more context when
/// processing a book.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreprocessorContext {
    /// The location of the book directory on disk.
    pub root: PathBuf,
    /// The language of the book being built. Is only `Some` if the book is part
    /// of a multilingual build output.
    pub language_ident: Option<String>,
    /// The build options passed from the frontend.
    pub build_opts: BuildOpts,
    /// The book configuration (`book.toml`).
    pub config: Config,
    /// The `Renderer` this preprocessor is being used with.
    pub renderer: String,
    /// The calling `mdbook` version.
    pub mdbook_version: String,
    #[serde(skip)]
    __non_exhaustive: (),
}

impl PreprocessorContext {
    /// Create a new `PreprocessorContext`.
    pub(crate) fn new(
        root: PathBuf,
        language_ident: Option<String>,
        build_opts: BuildOpts,
        config: Config,
        renderer: String,
    ) -> Self {
        PreprocessorContext {
            root,
            language_ident,
            build_opts,
            config,
            renderer,
            mdbook_version: crate::MDBOOK_VERSION.to_string(),
            __non_exhaustive: (),
        }
    }

    /// Get the directory containing this book's source files.
    pub fn source_dir(&self) -> PathBuf {
        self.root.join(&self.config.book.src)
    }
}

/// An operation which is run immediately after loading a book into memory and
/// before it gets rendered.
pub trait Preprocessor {
    /// Get the `Preprocessor`'s name.
    fn name(&self) -> &str;

    /// Run this `Preprocessor`, allowing it to update the book before it is
    /// given to a renderer.
    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book>;

    /// A hint to `MDBook` whether this preprocessor is compatible with a
    /// particular renderer.
    ///
    /// By default, always returns `true`.
    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}
