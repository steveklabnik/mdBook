# Alternative Backends

A "backend" is simply a program which `mdbook` will invoke during the book
rendering process. This program is passed a JSON representation of the book and
configuration information via `stdin`. Once the backend receives this
information it is free to do whatever it wants.

There are already several alternative backends on GitHub which can be used as a
rough example of how this is accomplished in practice.

- [mdbook-linkcheck] - a simple program for verifying the book doesn't contain
  any broken links
- [mdbook-epub] - an EPUB renderer
- [mdbook-test] - a program to run the book's contents through [rust-skeptic] to
  verify everything compiles and runs correctly (similar to `rustdoc --test`)

This page will step you through creating your own alternative backend in the form
of a simple word counting program. Although it will be written in Rust, there's
no reason why it couldn't be accomplished using something like Python or Ruby.


## Setting Up

First you'll want to create a new binary program and add `mdbook` as a
dependency.

```shell
$ cargo new --bin mdbook-wordcount
$ cd mdbook-wordcount
$ cargo add mdbook
```

When our `mdbook-wordcount` plugin is invoked, `mdbook` will send it a JSON
version of [`RenderContext`] via our plugin's `stdin`. For convenience, there's
a [`RenderContext::from_json()`] constructor which will load a `RenderContext`.

This is all the boilerplate necessary for our backend to load the book.

```rust
// src/main.rs
extern crate mdbook;

use std::io;
use mdbook::renderer::RenderContext;

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
}
```

> **Note:** The `RenderContext` contains a `version` field. This lets backends
  figure out whether they are compatible with the version of `mdbook` it's being
  called by. This `version` comes directly from the corresponding field in
  `mdbook`'s `Cargo.toml`.

  It is recommended that backends use the [`semver`] crate to inspect this field
  and emit a warning if there may be a compatibility issue.


## Inspecting the Book

Now our backend has a copy of the book, lets count how many words are in each
chapter!

Because the `RenderContext` contains a [`Book`] field (`book`), and a `Book` has
the [`Book::iter()`] method for iterating over all items in a `Book`, this step
turns out to be just as easy as the first.

```rust

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            let num_words = count_words(ch);
            println!("{}: {}", ch.name, num_words);
        }
    }
}

fn count_words(ch: &Chapter) -> usize {
    ch.content.split_whitespace().count()
}
```


## Enabling the Backend

Now we've got the basics running, we want to actually use it. First, install the
program.

```shell
$ cargo install --path .
```

Then `cd` to the particular book you'd like to count the words of and update its
`book.toml` file.

```diff
  [book]
  title = "mdBook Documentation"
  description = "Create book from markdown files. Like Gitbook but implemented in Rust"
  authors = ["Mathieu David", "Michael-F-Bryan"]

+ [output.html]

+ [output.wordcount]
```

When it loads a book into memory, `mdbook` will inspect your `book.toml` file to
try and figure out which backends to use by looking for all `output.*` tables.
If none are provided it'll fall back to using the default HTML renderer.

Notably, this means if you want to add your own custom backend you'll also need
to make sure to add the HTML backend, even if its table just stays empty.

Now you just need to build your book like normal, and everything should *Just
Work*.

```shell
$ mdbook build
...
2018-01-16 07:31:15 [INFO] (mdbook::renderer): Invoking the "mdbook-wordcount" renderer
mdBook: 126
Command Line Tool: 224
init: 283
build: 145
watch: 146
serve: 292
test: 139
Format: 30
SUMMARY.md: 259
Configuration: 784
Theme: 304
index.hbs: 447
Syntax highlighting: 314
MathJax Support: 153
Rust code specific features: 148
For Developers: 788
Alternative Backends: 710
Contributors: 85
```

The reason we didn't need to specify the full name/path of our `wordcount`
backend is because `mdbook` will try to *infer* the program's name via
convention. The executable for the `foo` backend is typically called
`mdbook-foo`, with an associated `[output.foo]` entry in the `book.toml`. To
explicitly tell `mdbook` what command to invoke (it may require command-line
arguments or be an interpreted script), you can use the `command` field.

```diff
  [book]
  title = "mdBook Documentation"
  description = "Create book from markdown files. Like Gitbook but implemented in Rust"
  authors = ["Mathieu David", "Michael-F-Bryan"]

  [output.html]

  [output.wordcount]
+ command = "python /path/to/wordcount.py"
```


## Configuration

Now imagine you don't want to count the number of words on a particular chapter
(it might be generated text/code, etc). The canonical way to do this is via the
usual `book.toml` configuration file by adding items to your `[output.foo]`
table.

The `Config` can be treated roughly as a nested hashmap which lets you call
methods like `get()` to access the config's contents, with a
`get_deserialized()` convenience method for retrieving a value and automatically
deserializing to some arbitrary type `T`.

To implement this, we'll create our own serializable `WordcountConfig` struct
which will encapsulate all configuration for this backend.

First add `serde` and `serde_derive` to your `Cargo.toml`,

```
$ cargo add serde serde_derive
```

And then you can create the config struct,

```rust
extern crate serde;
#[macro_use]
extern crate serde_derive;

...

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WordcountConfig {
  pub ignores: Vec<String>,
}
```

Now we just need to deserialize the `WordcountConfig` from our `RenderContext`
and then add a check to make sure we skip ignored chapters.

```diff
  fn main() {
      let mut stdin = io::stdin();
      let ctx = RenderContext::from_json(&mut stdin).unwrap();
+     let cfg: WordcountConfig = ctx.config
+         .get_deserialized("output.wordcount")
+         .unwrap_or_default();

      for item in ctx.book.iter() {
          if let BookItem::Chapter(ref ch) = *item {
+             if cfg.ignores.contains(&ch.name) {
+                 continue;
+             }
+
              let num_words = count_words(ch);
              println!("{}: {}", ch.name, num_words);
          }
      }
  }
```


## Output and Signalling Failure

While it's nice to print word counts to the terminal when a book is built, it
might also be a good idea to output them to a file somewhere. `mdbook` tells a
backend where it should place any generated output via the `destination` field
in [`RenderContext`].

```diff
+ use std::fs::{self, File};
+ use std::io::{self, Write};
- use std::io;
  use mdbook::renderer::RenderContext;
  use mdbook::book::{BookItem, Chapter};

  fn main() {
    ...

+     let _ = fs::create_dir_all(&ctx.destination);
+     let mut f = File::create(ctx.destination.join("wordcounts.txt")).unwrap();
+
      for item in ctx.book.iter() {
          if let BookItem::Chapter(ref ch) = *item {
              ...

              let num_words = count_words(ch);
              println!("{}: {}", ch.name, num_words);
+             writeln!(f, "{}: {}", ch.name, num_words).unwrap();
          }
      }
  }
```

> **Note:** There is no guarantee that the destination directory exists or is
> empty (`mdbook` may leave the previous contents to let backends do caching),
> so it's always a good idea to create it with `fs::create_dir_all()`.
>
> If the destination directory already exists, don't assume it will be empty.
> To allow backends to cache the results from previous runs, `mdbook` may leave
> old content in the directory.

There's always the possibility that an error will occur while processing a book
(just look at all the `unwrap()`'s we've written already), so `mdbook` will
interpret a non-zero exit code as a rendering failure.

For example, if we wanted to make sure all chapters have an *even* number of
words, erroring out if an odd number is encountered, then you may do something
like this:

```diff
+ use std::process;
  ...

  fn main() {
      ...

      for item in ctx.book.iter() {
          if let BookItem::Chapter(ref ch) = *item {
              ...

              let num_words = count_words(ch);
              println!("{}: {}", ch.name, num_words);
              writeln!(f, "{}: {}", ch.name, num_words).unwrap();

+             if cfg.deny_odds && num_words % 2 == 1 {
+               eprintln!("{} has an odd number of words!", ch.name);
+               process::exit(1);
              }
          }
      }
  }

  #[derive(Debug, Default, Serialize, Deserialize)]
  #[serde(default, rename_all = "kebab-case")]
  pub struct WordcountConfig {
      pub ignores: Vec<String>,
+     pub deny_odds: bool,
  }
```

Now, if we reinstall the backend and build a book,

```shell
$ cargo install --path . --force
$ mdbook build /path/to/book
...
2018-01-16 21:21:39 [INFO] (mdbook::renderer): Invoking the "wordcount" renderer
mdBook: 126
Command Line Tool: 224
init: 283
init has an odd number of words!
2018-01-16 21:21:39 [ERROR] (mdbook::renderer): Renderer exited with non-zero return code.
2018-01-16 21:21:39 [ERROR] (mdbook::utils): Error: Rendering failed
2018-01-16 21:21:39 [ERROR] (mdbook::utils):    Caused By: The "mdbook-wordcount" renderer failed
```

As you've probably already noticed, output from the plugin's subprocess is
immediately passed through to the user. It is encouraged for plugins to follow
the "rule of silence" and only generate output when necessary (e.g. an error in
generation or a warning).

All environment variables are passed through to the backend, allowing you to use
the usual `RUST_LOG` to control logging verbosity.

## Handling missing backends

If you enable a backend that isn't installed, the default behavior is to throw an error:

```text
The command `mdbook-wordcount` wasn't found, is the "wordcount" backend installed?
If you want to ignore this error when the "wordcount" backend is not installed,
set `optional = true` in the `[output.wordcount]` section of the book.toml configuration file.
```

This behavior can be changed by marking the backend as optional.

```diff
  [book]
  title = "mdBook Documentation"
  description = "Create book from markdown files. Like Gitbook but implemented in Rust"
  authors = ["Mathieu David", "Michael-F-Bryan"]

  [output.html]

  [output.wordcount]
  command = "python /path/to/wordcount.py"
+ optional = true
```

This demotes the error to a warning, and it will instead look like this:

```text
The command was not found, but was marked as optional.
    Command: wordcount
```


## Wrapping Up

Although contrived, hopefully this example was enough to show how you'd create
an alternative backend for `mdbook`. If you feel it's missing something, don't
hesitate to create an issue in the [issue tracker] so we can improve the user
guide.

The existing backends mentioned towards the start of this chapter should serve
as a good example of how it's done in real life, so feel free to skim through
the source code or ask questions.


[mdbook-linkcheck]: https://github.com/Michael-F-Bryan/mdbook-linkcheck
[mdbook-epub]: https://github.com/Michael-F-Bryan/mdbook-epub
[mdbook-test]: https://github.com/Michael-F-Bryan/mdbook-test
[rust-skeptic]: https://github.com/budziq/rust-skeptic
[`RenderContext`]: https://docs.rs/mdbook/*/mdbook/renderer/struct.RenderContext.html
[`RenderContext::from_json()`]: https://docs.rs/mdbook/*/mdbook/renderer/struct.RenderContext.html#method.from_json
[`semver`]: https://crates.io/crates/semver
[`Book`]: https://docs.rs/mdbook/*/mdbook/book/struct.Book.html
[`Book::iter()`]: https://docs.rs/mdbook/*/mdbook/book/struct.Book.html#method.iter
[`Config`]: https://docs.rs/mdbook/*/mdbook/config/struct.Config.html
[issue tracker]: https://github.com/rust-lang/mdBook/issues
