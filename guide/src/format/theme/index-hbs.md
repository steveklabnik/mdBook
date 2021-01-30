# index.hbs

`index.hbs` is the handlebars template that is used to render the book. The
markdown files are processed to html and then injected in that template.

If you want to change the layout or style of your book, chances are that you
will have to modify this template a little bit. Here is what you need to know.

## Data

A lot of data is exposed to the handlebars template with the "context". In the
handlebars template you can access this information by using

```handlebars
{{name_of_property}}
```

Here is a list of the properties that are exposed:

- ***language*** Language of the book in the form `en`, as specified in `book.toml` (if not specified, defaults to `en`). To use in <code
  class="language-html">\<html lang="{{ language }}"></code> for example.
- ***title*** Title used for the current page. This is identical to `{{ chapter_title }} - {{ book_title }}` unless `book_title` is not set in which case it just defaults to the `chapter_title`.
- ***book_title*** Title of the book, as specified in `book.toml`
- ***chapter_title*** Title of the current chapter, as listed in `SUMMARY.md`

- ***path*** Relative path to the original markdown file from the source
  directory
- ***content*** This is the rendered markdown.
- ***path_to_root*** This is a path containing exclusively `../`'s that points
  to the root of the book from the current file. Since the original directory
  structure is maintained, it is useful to prepend relative links with this
  `path_to_root`.

- ***chapters*** Is an array of dictionaries of the form
  ```json
  {"section": "1.2.1", "name": "name of this chapter", "path": "dir/markdown.md"}
  ```
  containing all the chapters of the book. It is used for example to construct
  the table of contents (sidebar).

## Handlebars Helpers

In addition to the properties you can access, there are some handlebars helpers
at your disposal.

### 1. toc

The toc helper is used like this

```handlebars
{{#toc}}{{/toc}}
```

and outputs something that looks like this, depending on the structure of your
book

```html
<ul class="chapter">
    <li><a href="link/to/file.html">Some chapter</a></li>
    <li>
        <ul class="section">
            <li><a href="link/to/other_file.html">Some other Chapter</a></li>
        </ul>
    </li>
</ul>
```

If you would like to make a toc with another structure, you have access to the
chapters property containing all the data. The only limitation at the moment
is that you would have to do it with JavaScript instead of with a handlebars
helper.

```html
<script>
var chapters = {{chapters}};
// Processing here
</script>
```

### 2. previous / next

The previous and next helpers expose a `link` and `name` property to the
previous and next chapters.

They are used like this

```handlebars
{{#previous}}
    <a href="{{link}}" class="nav-chapters previous">
        <i class="fa fa-angle-left"></i>
    </a>
{{/previous}}
```

The inner html will only be rendered if the previous / next chapter exists.
Of course the inner html can be changed to your liking.

### 3. resource

The path to a static file.
It implicitly includes `path_to_root`,
and accounts for files that are renamed with a hash in their filename.

```handlebars
<link rel="stylesheet" href="{{ resource "css/chrome.css" }}">
```

------

*If you would like other properties or helpers exposed, please [create a new
issue](https://github.com/rust-lang/mdBook/issues)*
