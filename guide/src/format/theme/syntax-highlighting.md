# Syntax Highlighting

mdBook uses [Highlight.js](https://highlightjs.org) with a custom theme
for syntax highlighting.

Automatic language detection has been turned off, so you will probably want to
specify the programming language you use like this:

~~~markdown
```rust
fn main() {
    // Some code
}
```
~~~

## Supported languages

These languages are supported by default, but you can add more by supplying
your own `highlight.js` file:

- apache
- armasm
- bash
- c
- coffeescript
- cpp
- csharp
- css
- d
- diff
- go
- handlebars
- haskell
- http
- ini
- java
- javascript
- json
- julia
- kotlin
- less
- lua
- makefile
- markdown
- nginx
- objectivec
- perl
- php
- plaintext
- properties
- python
- r
- ruby
- rust
- scala
- scss
- shell
- sql
- swift
- typescript
- x86asm
- xml
- yaml

## Custom theme
Like the rest of the theme, the files used for syntax highlighting can be
overridden with your own.

- ***highlight.js*** normally you shouldn't have to overwrite this file, unless
  you want to use a more recent version.
- ***highlight.css*** theme used by highlight.js for syntax highlighting.

If you want to use another theme for `highlight.js` download it from their
website, or make it yourself, rename it to `highlight.css` and put it in
the `theme` folder of your book.

Now your theme will be used instead of the default theme.

## Hiding code lines

There is a feature in mdBook that lets you hide code lines by prepending them
with a `#`.


```bash
# fn main() {
    let x = 5;
    let y = 6;

    println!("{}", x + y);
# }
```

Will render as

```rust
# fn main() {
    let x = 5;
    let y = 7;

    println!("{}", x + y);
# }
```

Hiding is only enabled by default for code examples with the language `rust`,
since it collides with ordinary use of `#` in some other programming languages.
To enable code hiding for a specific non-Rust code block, append `,hidelines`
after the language selector:

~~~markdown
```haskell,hidelines
~~~

## Improve default theme

If you think the default theme doesn't look quite right for a specific language,
or could be improved, feel free to [submit a new
issue](https://github.com/rust-lang/mdBook/issues) explaining what you
have in mind and I will take a look at it.

You could also create a pull-request with the proposed improvements.

Overall the theme should be light and sober, without too many flashy colors.
