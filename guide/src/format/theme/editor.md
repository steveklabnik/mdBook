# Editor

In addition to providing runnable code playgrounds, mdBook optionally allows them
to be editable. In order to enable editable code blocks, the following needs to
be added to the ***book.toml***:

```toml
[output.html.playground]
editable = true
```

To make a specific block available for editing, the attribute `editable` needs
to be added to it:

<pre><code class="language-markdown">```rust,editable
fn main() {
    let number = 5;
    print!("{}", number);
}
```</code></pre>

The above will result in this editable playground:

```rust,editable
fn main() {
    let number = 5;
    print!("{}", number);
}
```

Note the new `Undo Changes` button in the editable playgrounds.

## Customizing the Editor

By default, the editor is the [Ace](https://ace.c9.io/) editor, but, if desired,
the functionality may be overridden by providing a different folder:

```toml
[output.html.playground]
editable = true
editor = "/path/to/editor"
```

Note that for the editor changes to function correctly, the `book.js` inside of
the `theme` folder will need to be overridden as it has some couplings with the
default Ace editor.
