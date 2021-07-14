# mdBook

[![Build Status](https://github.com/rust-lang/mdBook/workflows/CI/badge.svg?event=push)](https://github.com/rust-lang/mdBook/actions?workflow=CI)
[![crates.io](https://img.shields.io/crates/v/mdbook.svg)](https://crates.io/crates/mdbook)
[![LICENSE](https://img.shields.io/github/license/rust-lang/mdBook.svg)](LICENSE)

mdBook is a utility to create modern online books from Markdown files.


## What does it look like?

The [User Guide] for mdBook has been written in Markdown and is using mdBook to
generate the online book-like website you can read. The documentation uses the
latest version on GitHub and showcases the available features.

## Installation

There are multiple ways to install mdBook.

1. **Binaries**

   Binaries are available for download [here][releases]. Make sure to put the
   path to the binary into your `PATH`.

2. **From Crates.io**

   This requires at least [Rust] 1.39 and Cargo to be installed. Once you have installed
   Rust, type the following in the terminal:

   ```
   cargo install mdbook
   ```

   This will download and compile mdBook for you, the only thing left to do is
   to add the Cargo bin directory to your `PATH`.

   **Note for automatic deployment**

   If you are using a script to do automatic deployments using Travis or
   another CI server, we recommend that you specify a semver version range for
   mdBook when you install it through your script!

   This will constrain the server to install the latest **non-breaking**
   version of mdBook and will prevent your books from failing to build because
   we released a new version.

   You can also disable default features to speed up compile time.

   Example:

   ```
   cargo install mdbook --no-default-features --features output --vers "^0.1.0"
   ```

3. **From Git**

   The version published to crates.io will ever so slightly be behind the
   version hosted here on GitHub. If you need the latest version you can build
   the git version of mdBook yourself. Cargo makes this ***super easy***!

   ```
   cargo install --git https://github.com/rust-lang/mdBook.git mdbook
   ```

   Again, make sure to add the Cargo bin directory to your `PATH`.

4. **For Contributions**

   If you want to contribute to mdBook you will have to clone the repository on
   your local machine:

   ```
   git clone https://github.com/rust-lang/mdBook.git
   ```

   `cd` into `mdBook/` and run

   ```
   cargo build
   ```

   The resulting binary can be found in `mdBook/target/debug/` under the name
   `mdBook` or `mdBook.exe`.


## Usage

mdBook is primarily used as a command line tool, even though it exposes
all its functionality as a Rust crate for integration in other projects.

Here are the main commands you will want to run. For a more exhaustive
explanation, check out the [User Guide].

- `mdbook init <directory>`

    The init command will create a directory with the minimal boilerplate to
    start with. If the `<directory>` parameter is omitted, the current 
    directory will be used.

    ```
    book-test/
    ├── book
    └── src
        ├── chapter_1.md
        └── SUMMARY.md
    ```

    `book` and `src` are both directories. `src` contains the markdown files
    that will be used to render the output to the `book` directory.

    Please, take a look at the [CLI docs] for more information and some neat tricks.

- `mdbook build`

    This is the command you will run to render your book, it reads the
    `SUMMARY.md` file to understand the structure of your book, takes the
    markdown files in the source directory as input and outputs static html
    pages that you can upload to a server.

- `mdbook watch`

    When you run this command, mdbook will watch your markdown files to rebuild
    the book on every change. This avoids having to come back to the terminal
    to type `mdbook build` over and over again.

- `mdbook serve`

    Does the same thing as `mdbook watch` but additionally serves the book at
    `http://localhost:3000` (port is changeable) and reloads the browser when a
    change occurs.

- `mdbook clean`

    Delete directory in which generated book is located.

### Dockerized usage

Simply cd to any directory, and issue the following command:

```
docker run --rm -p 3000-3001:3000-3001 -v $(pwd):/book -it rust-lang/mdbook
```

If there is no **mdbook** structure inside this directory, a new one will be created using ```mdbook init``` accepting the defaults.

If there is an existing **mdbook** structure inside, then ```mdbook serve``` will be launched automatically.

### 3rd Party Plugins

The way a book is loaded and rendered can be configured by the user via third
party plugins. These plugins are just programs which will be invoked during the
build process and are split into roughly two categories, *preprocessors* and
*renderers*.

Preprocessors are used to transform a book before it is sent to a renderer.
One example would be to replace all occurrences of
`{{#include some_file.ext}}` with the contents of that file. Some existing
preprocessors are:

- `index` - a built-in preprocessor (enabled by default) which will transform
  all `README.md` chapters to `index.md` so `foo/README.md` can be accessed via
  the url `foo/` when published to a browser
- `links` - a built-in preprocessor (enabled by default) for expanding the
  `{{# playground}}` and `{{# include}}` helpers in a chapter.
- [`katex`](https://github.com/lzanini/mdbook-katex) - a preprocessor rendering LaTex equations to HTML.

Renderers are given the final book so they can do something with it. This is
typically used for, as the name suggests, rendering the document in a particular
format, however there's nothing stopping a renderer from doing static analysis
of a book in order to validate links or run tests. Some existing renderers are:

- `html` - the built-in renderer which will generate a HTML version of the book
- `markdown` - the built-in renderer (disabled by default) which will run
  preprocessors then output the resulting Markdown. Useful for debugging
  preprocessors.
- [`linkcheck`] - a backend which will check that all links are valid
- [`epub`] - an experimental EPUB generator

> **Note for Developers:** Feel free to send us a PR if you've developed your
> own plugin and want it mentioned here.

A preprocessor or renderer is enabled by installing the appropriate program and
then mentioning it in the book's `book.toml` file.

```console
$ cargo install mdbook-linkcheck
$ edit book.toml && cat book.toml
[book]
title = "My Awesome Book"
authors = ["Michael-F-Bryan"]

[output.html]

[output.linkcheck]  # enable the "mdbook-linkcheck" renderer

$ mdbook build
2018-10-20 13:57:51 [INFO] (mdbook::book): Book building has started
2018-10-20 13:57:51 [INFO] (mdbook::book): Running the html backend
2018-10-20 13:57:53 [INFO] (mdbook::book): Running the linkcheck backend
```

For more information on the plugin system, consult the [User Guide].

### As a library

Aside from the command line interface, this crate can also be used as a
library. This means that you could integrate it in an existing project, like a
web-app for example. Since the command line interface is just a wrapper around
the library functionality, when you use this crate as a library you have full
access to all the functionality of the command line interface with an easy to
use API and more!

See the [User Guide] and the [API docs] for more information.

## Contributions

Contributions are highly appreciated and encouraged! Don't hesitate to
participate to discussions in the issues, propose new features and ask for
help.

If you are just starting out with Rust, there are a series of issues that are
tagged [E-Easy] and **we will gladly mentor you** so that you can successfully
go through the process of fixing a bug or adding a new feature! Let us know if
you need any help.

For more info about contributing, check out our [contribution guide] which helps
you go through the build and contribution process!

There is also a [rendered version][master-docs] of the latest API docs
available, for those hacking on `master`.


## License

All the code in this repository is released under the ***Mozilla Public License v2.0***, for more information take a look at the [LICENSE] file.


[User Guide]: https://rust-lang.github.io/mdBook/
[API docs]: https://docs.rs/mdbook/*/mdbook/
[E-Easy]: https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AE-Easy
[contribution guide]: https://github.com/rust-lang/mdBook/blob/master/CONTRIBUTING.md
[LICENSE]: https://github.com/rust-lang/mdBook/blob/master/LICENSE
[releases]: https://github.com/rust-lang/mdBook/releases
[Rust]: https://www.rust-lang.org/
[CLI docs]: http://rust-lang.github.io/mdBook/cli/init.html
[master-docs]: http://rust-lang.github.io/mdBook/
[`linkcheck`]: https://crates.io/crates/mdbook-linkcheck
[`epub`]: https://crates.io/crates/mdbook-epub
