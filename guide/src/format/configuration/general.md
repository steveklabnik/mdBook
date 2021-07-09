# General Configuration

You can configure the parameters for your book in the ***book.toml*** file.

Here is an example of what a ***book.toml*** file might look like:

```toml
[book]
title = "Example book"
author = "John Doe"
description = "The example book covers examples."

[rust]
edition = "2018"

[build]
build-dir = "my-example-book"
create-missing = false

[preprocessor.index]

[preprocessor.links]

[output.html]
additional-css = ["custom.css"]

[output.html.search]
limit-results = 15
```

## Supported configuration options

It is important to note that **any** relative path specified in the
configuration will always be taken relative from the root of the book where the
configuration file is located.

### General metadata

This is general information about your book.

- **title:** The title of the book
- **authors:** The author(s) of the book
- **description:** A description for the book, which is added as meta
  information in the html `<head>` of each page
- **src:** By default, the source directory is found in the directory named
  `src` directly under the root folder. But this is configurable with the `src`
  key in the configuration file.
- **language:** The main language of the book, which is used as a language attribute `<html lang="en">` for example.

**book.toml**
```toml
[book]
title = "Example book"
authors = ["John Doe", "Jane Doe"]
description = "The example book covers examples."
src = "my-src"  # the source files will be found in `root/my-src` instead of `root/src`
language = "en"
```

### Rust options

Options for the Rust language, relevant to running tests and playground
integration.

- **edition**: Rust edition to use by default for the code snippets. Default
  is "2015". Individual code blocks can be controlled with the `edition2015`, 
  `edition2018` or `edition2021` annotations, such as:

  ~~~text
  ```rust,edition2015
  // This only works in 2015.
  let try = true;
  ```
  ~~~

### Build options

This controls the build process of your book.

- **build-dir:** The directory to put the rendered book in. By default this is
  `book/` in the book's root directory.
- **create-missing:** By default, any missing files specified in `SUMMARY.md`
  will be created when the book is built (i.e. `create-missing = true`). If this
  is `false` then the build process will instead exit with an error if any files
  do not exist.
- **use-default-preprocessors:** Disable the default preprocessors of (`links` &
  `index`) by setting this option to `false`.

  If you have the same, and/or other preprocessors declared via their table
  of configuration, they will run instead.

  - For clarity, with no preprocessor configuration, the default `links` and
    `index` will run.
  - Setting `use-default-preprocessors = false` will disable these
    default preprocessors from running.
  - Adding `[preprocessor.links]`, for example, will ensure, regardless of
    `use-default-preprocessors` that `links` it will run.
