//! Build options.

/// Build options passed from the frontend to control how the book is built.
/// Separate from `Config`, which is global to all book languages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
pub struct BuildOpts {
    /// Language of the book to render.
    pub language_ident: Option<String>,
}
