# The init command

There is some minimal boilerplate that is the same for every new book. It's for
this purpose that mdBook includes an `init` command.

The `init` command is used like this:

```bash
mdbook init
```

When using the `init` command for the first time, a couple of files will be set
up for you:
```bash
book-test/
├── book
└── src
    ├── chapter_1.md
    └── SUMMARY.md
```

- The `src` directory is where you write your book in markdown. It contains all
  the source files, configuration files, etc.

- The `book` directory is where your book is rendered. All the output is ready
  to be uploaded to a server to be seen by your audience.

- The `SUMMARY.md` is the skeleton of your
  book, and is discussed in more detail [in another
  chapter](../format/summary.md).

#### Tip: Generate chapters from SUMMARY.md

When a `SUMMARY.md` file already exists, the `init` command will first parse it
and generate the missing files according to the paths used in the `SUMMARY.md`.
This allows you to think and create the whole structure of your book and then
let mdBook generate it for you.

#### Specify a directory

The `init` command can take a directory as an argument to use as the book's root
instead of the current working directory.

```bash
mdbook init path/to/book
```

#### --theme

When you use the `--theme` flag, the default theme will be copied into a
directory called `theme` in your source directory so that you can modify it.

The theme is selectively overwritten, this means that if you don't want to
overwrite a specific file, just delete it and the default file will be used.

#### --title

Specify a title for the book. If not supplied, an interactive prompt will ask for 
a title. 

```bash
mdbook init --title="my amazing book"
```

#### --ignore

Create a `.gitignore` file configured to ignore the `book` directory created when [building] a book. 
If not supplied, an interactive prompt will ask whether it should be created.

[building]: build.md
