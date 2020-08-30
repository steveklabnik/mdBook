# For Developers

While `mdbook` is mainly used as a command line tool, you can also import the
underlying library directly and use that to manage a book. It also has a fairly
flexible plugin mechanism, allowing you to create your own custom tooling and
consumers (often referred to as *backends*) if you need to do some analysis of
the book or render it in a different format.

The *For Developers* chapters are here to show you the more advanced usage of
`mdbook`.

The two main ways a developer can hook into the book's build process is via,

- [Preprocessors](preprocessors.md)
- [Alternative Backends](backends.md)


## The Build Process

The process of rendering a book project goes through several steps.

1. Load the book
    - Parse the `book.toml`, falling back to the default `Config` if it doesn't
       exist
    - Load the book chapters into memory
    - Discover which preprocessors/backends should be used
2. Run the preprocessors
3. Call each backend in turn


## Using `mdbook` as a Library

The `mdbook` binary is just a wrapper around the `mdbook` crate, exposing its
functionality as a command-line program. As such it is quite easy to create your
own programs which use `mdbook` internally, adding your own functionality (e.g.
a custom preprocessor) or tweaking the build process.

The easiest way to find out how to use the `mdbook` crate is by looking at the
[API Docs]. The top level documentation explains how one would use the
[`MDBook`] type to load and build a book, while the [config] module gives a good
explanation on the configuration system.


[`MDBook`]: https://docs.rs/mdbook/*/mdbook/book/struct.MDBook.html
[API Docs]: https://docs.rs/mdbook/*/mdbook/
[config]: https://docs.rs/mdbook/*/mdbook/config/index.html
