# mdBook-specific features

## Hiding code lines

There is a feature in mdBook that lets you hide code lines by prepending them
with a `#` [like you would with Rustdoc][rustdoc-hide].

[rustdoc-hide]: https://doc.rust-lang.org/stable/rustdoc/documentation-tests.html#hiding-portions-of-the-example

<pre><code class="language-markdown">```rust
# fn main() {
    let x = 5;
    let y = 6;

    println!("{}", x + y);
# }
```</code></pre>

Will render as

```rust
# fn main() {
    let x = 5;
    let y = 7;

    println!("{}", x + y);
# }
```

By default, this only works for code examples that are annotated with `rust`. However, you can define 
custom prefixes for other languages by adding a new line-hiding prefix in your `book.toml` with the 
language name and prefix character (you can even do multi-character prefixes if you really want to):

```toml
[output.html.playground.line-hiding-prefixes]
python = "~"
```

The prefix will hide any lines that begin with the given prefix, but the prefix can be escaped using 
a backslash. With the python prefix shown above, this:

<pre><code class="language-markdown">```python
~def fib():
    a, b = 0, 1
    while a < n:
        print(a, end=' ')
        a, b = b, a+b
    print()
    ~# hide me!
    \~# leave me be!
~fib(1000)
```</code></pre>

will render as

```python
~def fib():
    a, b = 0, 1
    while a < n:
        print(a, end=' ')
        a, b = b, a+b
    print()
    ~# hide me!
    \~# leave me be!
~fib(1000)
```

If you need something more advanced than a simple prefix (e.g. rust uses `#` but doesn't hide `#[...]` lines), 
you can define a custom regular expression using [line-hiding patterns](./config.md#line-hiding-patterns).

## Including files

With the following syntax, you can include files into your book:

```hbs
\{{#include file.rs}}
```

The path to the file has to be relative from the current source file.

mdBook will interpret included files as Markdown. Since the include command
is usually used for inserting code snippets and examples, you will often
wrap the command with ```` ``` ```` to display the file contents without
interpreting them.

````hbs
```
\{{#include file.rs}}
```
````

## Including portions of a file
Often you only need a specific part of the file, e.g. relevant lines for an
example. We support four different modes of partial includes:

```hbs
\{{#include file.rs:2}}
\{{#include file.rs::10}}
\{{#include file.rs:2:}}
\{{#include file.rs:2:10}}
```

The first command only includes the second line from file `file.rs`. The second
command includes all lines up to line 10, i.e. the lines from 11 till the end of
the file are omitted. The third command includes all lines from line 2, i.e. the
first line is omitted. The last command includes the excerpt of `file.rs`
consisting of lines 2 to 10.

To avoid breaking your book when modifying included files, you can also
include a specific section using anchors instead of line numbers.
An anchor is a pair of matching lines. The line beginning an anchor must
match the regex `ANCHOR:\s*[\w_-]+` and similarly the ending line must match
the regex `ANCHOR_END:\s*[\w_-]+`. This allows you to put anchors in
any kind of commented line.

Consider the following file to include:
```rs
/* ANCHOR: all */

// ANCHOR: component
struct Paddle {
    hello: f32,
}
// ANCHOR_END: component

////////// ANCHOR: system
impl System for MySystem { ... }
////////// ANCHOR_END: system

/* ANCHOR_END: all */
```

Then in the book, all you have to do is:
````hbs
Here is a component:
```rust,no_run,noplayground
\{{#include file.rs:component}}
```

Here is a system:
```rust,no_run,noplayground
\{{#include file.rs:system}}
```

This is the full file.
```rust,no_run,noplayground
\{{#include file.rs:all}}
```
````

Lines containing anchor patterns inside the included anchor are ignored.

## Including a file but initially hiding all except specified lines

The `rustdoc_include` helper is for including code from external Rust files that contain complete
examples, but only initially showing particular lines specified with line numbers or anchors in the
same way as with `include`.

The lines not in the line number range or between the anchors will still be included, but they will
be prefaced with `#`. This way, a reader can expand the snippet to see the complete example, and
Rustdoc will use the complete example when you run `mdbook test`.

For example, consider a file named `file.rs` that contains this Rust program:

```rust
fn main() {
    let x = add_one(2);
    assert_eq!(x, 3);
}

fn add_one(num: i32) -> i32 {
    num + 1
}
```

We can include a snippet that initially shows only line 2 by using this syntax:

````hbs
To call the `add_one` function, we pass it an `i32` and bind the returned value to `x`:

```rust
\{{#rustdoc_include file.rs:2}}
```
````

This would have the same effect as if we had manually inserted the code and hidden all but line 2
using `#`:

````hbs
To call the `add_one` function, we pass it an `i32` and bind the returned value to `x`:

```rust
# fn main() {
    let x = add_one(2);
#     assert_eq!(x, 3);
# }
#
# fn add_one(num: i32) -> i32 {
#     num + 1
# }
```
````

That is, it looks like this (click the "expand" icon to see the rest of the file):

```rust
# fn main() {
    let x = add_one(2);
#     assert_eq!(x, 3);
# }
#
# fn add_one(num: i32) -> i32 {
#     num + 1
# }
```

## Inserting runnable Rust files

With the following syntax, you can insert runnable Rust files into your book:

```hbs
\{{#playground file.rs}}
```

The path to the Rust file has to be relative from the current source file.

When play is clicked, the code snippet will be sent to the [Rust Playground] to be
compiled and run. The result is sent back and displayed directly underneath the
code.

Here is what a rendered code snippet looks like:

{{#playground example.rs}}

[Rust Playground]: https://play.rust-lang.org/

## Controlling page \<title\>

A chapter can set a \<title\> that is different from its entry in the table of
contents (sidebar) by including a `\{{#title ...}}` near the top of the page.

```hbs
\{{#title My Title}}
```
