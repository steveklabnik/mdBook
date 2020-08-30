use crate::book::{Book, BookItem, LoadedBook};
use crate::errors::*;
use crate::renderer::{RenderContext, Renderer};
use crate::utils;

use std::fs;
use std::path::Path;

#[derive(Default)]
/// A renderer to output the Markdown after the preprocessors have run. Mostly useful
/// when debugging preprocessors.
pub struct MarkdownRenderer;

impl MarkdownRenderer {
    /// Create a new `MarkdownRenderer` instance.
    pub fn new() -> Self {
        MarkdownRenderer
    }
}

impl Renderer for MarkdownRenderer {
    fn name(&self) -> &str {
        "markdown"
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        let destination = &ctx.destination;
        let book = &ctx.book;

        if destination.exists() {
            utils::fs::remove_dir_content(destination)
                .with_context(|| "Unable to remove stale Markdown output")?;
        }

        match book {
            LoadedBook::Localized(books) => {
                for (lang_ident, book) in books.0.iter() {
                    let localized_destination = destination.join(lang_ident);
                    render_book(&localized_destination, book)?;
                }
            }
            LoadedBook::Single(book) => render_book(destination, &book)?,
        }

        Ok(())
    }
}

fn render_book(destination: &Path, book: &Book) -> Result<()> {
    fs::create_dir_all(destination)
        .with_context(|| "Unexpected error when constructing destination path")?;

    trace!("markdown render");
    for item in book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            if !ch.is_draft_chapter() {
                utils::fs::write_file(
                    destination,
                    &ch.path.as_ref().expect("Checked path exists before"),
                    ch.content.as_bytes(),
                )?;
            }
        }
    }

    Ok(())
}
