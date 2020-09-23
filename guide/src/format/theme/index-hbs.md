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

------

*If you would like other properties or helpers exposed, please [create a new
issue](https://github.com/rust-lang/mdBook/issues)*

### 3. fa

mdBook includes a copy of [Font Awesome Free's](https://fontawesome.com)
MIT-licensed SVG files. It accepts three positional arguments:

1. Type: one of "solid", "regular", and "brands" (light and duotone are not
   currently supported)
2. Icon: anything chosen from the
   [free icon set](https://fontawesome.com/icons?d=gallery&m=free)
3. ID (optional): if included, an HTML ID attribute will be added to the
   icon's wrapping `<span>` tag

For example, this handlebars syntax will become this HTML:

```handlebars
{{fa "solid" "print" "print-button"}}
```

```html
<span class=fa-svg id="print-button"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M448 192V77.25c0-8.49-3.37-16.62-9.37-22.63L393.37 9.37c-6-6-14.14-9.37-22.63-9.37H96C78.33 0 64 14.33 64 32v160c-35.35 0-64 28.65-64 64v112c0 8.84 7.16 16 16 16h48v96c0 17.67 14.33 32 32 32h320c17.67 0 32-14.33 32-32v-96h48c8.84 0 16-7.16 16-16V256c0-35.35-28.65-64-64-64zm-64 256H128v-96h256v96zm0-224H128V64h192v48c0 8.84 7.16 16 16 16h48v96zm48 72c-13.25 0-24-10.75-24-24 0-13.26 10.75-24 24-24s24 10.74 24 24c0 13.25-10.75 24-24 24z"/></svg></span>
```
