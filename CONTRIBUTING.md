# Contributing

Welcome stranger!

If you have come here to learn how to contribute to mdBook, we have some tips for you!

First of all, don't hesitate to ask questions!
Use the [issue tracker](https://github.com/rust-lang/mdBook/issues), no question is too simple.

### Issues to work on

Any issue is up for the grabbing, but if you are starting out, you might be interested in the
[E-Easy issues](https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AE-Easy).
Those are issues that are considered more straightforward for beginners to Rust or the codebase itself.
These issues can be a good launching pad for more involved issues. Easy tasks for a first time contribution
include documentation improvements, new tests, examples, updating dependencies, etc.

If you come from a web development background, you might be interested in issues related to web technologies tagged
[A-JavaScript](https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AA-JavaScript),
[A-Style](https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AA-Style),
[A-HTML](https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AA-HTML) or
[A-Mobile](https://github.com/rust-lang/mdBook/issues?q=is%3Aopen+is%3Aissue+label%3AA-Mobile).

When you decide you want to work on a specific issue, ping us on that issue so that we can assign it to you.
Again, do not hesitate to ask questions. We will gladly mentor anyone that want to tackle an issue.

Issues on the issue tracker are categorized with the following labels:

- **A**-prefixed labels state which area of the project an issue relates to.
- **E**-prefixed labels show an estimate of the experience necessary to fix the issue.
- **M**-prefixed labels are meta-issues used for questions, discussions, or tracking issues
- **S**-prefixed labels show the status of the issue
- **T**-prefixed labels show the type of issue

### Building mdBook

mdBook builds on stable Rust, if you want to build mdBook from source, here are the steps to follow:

1. Navigate to the directory of your choice
0. Clone this repository with git.

   ```
   git clone https://github.com/rust-lang/mdBook.git
   ```
0. Navigate into the newly created `mdBook` directory
0. Run `cargo build`

The resulting binary can be found in `mdBook/target/debug/` under the name `mdbook` or `mdbook.exe`.

### Code Quality

We love code quality and Rust has some excellent tools to assist you with contributions.

#### Formatting Code with rustfmt

Before you make your Pull Request to the project, please run it through the `rustfmt` utility.
This will ensure we have good quality source code that is better for us all to maintain.

[rustfmt](https://github.com/rust-lang/rustfmt) has a lot more information on the project.
The quick guide is

1. Install it
    ```
    rustup component add rustfmt
    ```
1. You can now run `rustfmt` on a single file simply by...
    ```
    rustfmt src/path/to/your/file.rs
    ```
   ... or you can format the entire project with
   ```
   cargo fmt
   ```
   When run through `cargo` it will format all bin and lib files in the current crate.

For more information, such as running it from your favourite editor, please see the `rustfmt` project. [rustfmt](https://github.com/rust-lang/rustfmt)


#### Finding Issues with Clippy

Clippy is a code analyser/linter detecting mistakes, and therefore helps to improve your code.
Like formatting your code with `rustfmt`, running clippy regularly and before your Pull Request will
help us maintain awesome code.

The best documentation can be found over at [rust-clippy](https://github.com/rust-lang/rust-clippy)

1. To install
    ```
    rustup component add clippy
    ```
2. Running clippy
    ```
    cargo clippy
    ```

Clippy has an ever growing list of checks, that are managed in [lint files](https://rust-lang.github.io/rust-clippy/master/index.html).

### Making a pull-request

When you feel comfortable that your changes could be integrated into mdBook, you can create a pull-request on GitHub.
One of the core maintainers will then approve the changes or request some changes before it gets merged.

If you want to make your pull-request even better, you might want to run [Clippy](https://github.com/Manishearth/rust-clippy)
and [rustfmt](https://github.com/rust-lang/rustfmt) on the code first.
This is not a requirement though and will never block a pull-request from being merged.

That's it, happy contributions! :tada: :tada: :tada:

## Browser compatibility and testing

Currently we don't have a strict browser compatibility matrix due to our limited resources.
We generally strive to keep mdBook compatible with a relatively recent browser on all of the most major platforms.
That is, supporting Chrome, Safari, Firefox, Edge on Windows, macOS, Linux, iOS, and Android.
If possible, do your best to avoid breaking older browser releases.

Any change to the HTML or styling is encouraged to manually check on as many browsers and platforms that you can.
Unfortunately at this time we don't have any automated UI or browser testing, so your assistance in testing is appreciated.
