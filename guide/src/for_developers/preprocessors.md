# Preprocessors

A *preprocessor* is simply a bit of code which gets run immediately after the
book is loaded and before it gets rendered, allowing you to update and mutate
the book. Possible use cases are:

- Creating custom helpers like `\{{#include /path/to/file.md}}`
- Updating links so `[some chapter](some_chapter.md)` is automatically changed
  to `[some chapter](some_chapter.html)` for the HTML renderer
- Substituting in latex-style expressions (`$$ \frac{1}{3} $$`) with their
  mathjax equivalents


## Hooking Into MDBook

MDBook uses a fairly simple mechanism for discovering third party plugins.
A new table is added to `book.toml` (e.g. `preprocessor.foo` for the `foo`
preprocessor) and then `mdbook` will try to invoke the `mdbook-foo` program as
part of the build process.

A preprocessor can be hard-coded to specify which backend(s) it should be run
for with the `preprocessor.foo.renderer` key. For example, it doesn't make sense for 
[MathJax](../format/mathjax.md) to be used for non-HTML renderers.

```toml
[book]
title = "My Book"
authors = ["Michael-F-Bryan"]

[preprocessor.foo]
# The command can also be specified manually
command = "python3 /path/to/foo.py"
# Only run the `foo` preprocessor for the HTML and EPUB renderer
renderer = ["html", "epub"]
```

In typical unix style, all inputs to the plugin will be written to `stdin` as
JSON and `mdbook` will read from `stdout` if it is expecting output.

The easiest way to get started is by creating your own implementation of the
`Preprocessor` trait (e.g. in `lib.rs`) and then creating a bash binary which
translates inputs to the correct `Preprocessor` method. For convenience, there
is [an example no-op preprocessor] in the `examples/` directory which can easily
be adapted for other preprocessors.

<details>
<summary>Example no-op preprocessor</summary>

```rust
// nop-preprocessors.rs

{{#include ../../../examples/nop-preprocessor.rs}}
```
</details>

## Hints For Implementing A Preprocessor

By pulling in `mdbook` as a library, preprocessors can have access to the
existing infrastructure for dealing with books.

For example, a custom preprocessor could use the
[`CmdPreprocessor::parse_input()`] function to deserialize the JSON written to
`stdin`. Then each chapter of the `Book` can be mutated in-place via
[`Book::for_each_mut()`], and then written to `stdout` with the `serde_json`
crate.

Chapters can be accessed either directly (by recursively iterating over
chapters) or via the `Book::for_each_mut()` convenience method.

The `chapter.content` is just a string which happens to be markdown. While it's
entirely possible to use regular expressions or do a manual find & replace,
you'll probably want to process the input into something more computer-friendly.
The [`pulldown-cmark`][pc] crate implements a production-quality event-based
Markdown parser, with the [`pulldown-cmark-to-cmark`][pctc] allowing you to
translate events back into markdown text.

The following code block shows how to remove all emphasis from markdown,
without accidentally breaking the document.

```rust
fn remove_emphasis(
    num_removed_items: &mut usize,
    chapter: &mut Chapter,
) -> Result<String> {
    let mut buf = String::with_capacity(chapter.content.len());

    let events = Parser::new(&chapter.content).filter(|e| {
        let should_keep = match *e {
            Event::Start(Tag::Emphasis)
            | Event::Start(Tag::Strong)
            | Event::End(Tag::Emphasis)
            | Event::End(Tag::Strong) => false,
            _ => true,
        };
        if !should_keep {
            *num_removed_items += 1;
        }
        should_keep
    });

    cmark(events, &mut buf, None).map(|_| buf).map_err(|err| {
        Error::from(format!("Markdown serialization failed: {}", err))
    })
}
```

For everything else, have a look [at the complete example][example].

[preprocessor-docs]: https://docs.rs/mdbook/latest/mdbook/preprocess/trait.Preprocessor.html
[pc]: https://crates.io/crates/pulldown-cmark
[pctc]: https://crates.io/crates/pulldown-cmark-to-cmark
[example]: https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs
[an example no-op preprocessor]: https://github.com/rust-lang/mdBook/blob/master/examples/nop-preprocessor.rs
[`CmdPreprocessor::parse_input()`]: https://docs.rs/mdbook/latest/mdbook/preprocess/trait.Preprocessor.html#method.parse_input
[`Book::for_each_mut()`]: https://docs.rs/mdbook/latest/mdbook/book/struct.Book.html#method.for_each_mut
