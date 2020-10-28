#![allow(missing_docs)] // FIXME: Document this

pub use self::hbs_renderer::HtmlHandlebars;
pub use self::static_files::StaticFiles;

mod hbs_renderer;
mod helpers;
mod static_files;

#[cfg(feature = "search")]
mod search;
