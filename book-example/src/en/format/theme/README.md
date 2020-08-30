# Theme

The default renderer uses a [handlebars](http://handlebarsjs.com/) template to
render your markdown files and comes with a default theme included in the mdBook
binary.

The theme is totally customizable, you can selectively replace every file from
the theme by your own by adding a `theme` directory next to `src` folder in your
project root. Create a new file with the name of the file you want to override
and now that file will be used instead of the default file.

Here are the files you can override:

- **_index.hbs_** is the handlebars template.
- **_head.hbs_** is appended to the HTML `<head>` section.
- **_header.hbs_** content is appended on top of every book page.
- **_book.css_** is the style used in the output. If you want to change the
  design of your book, this is probably the file you want to modify. Sometimes
  in conjunction with `index.hbs` when you want to radically change the layout.
- **_book.js_** is mostly used to add client side functionality, like hiding /
  un-hiding the sidebar, changing the theme, ...
- **_highlight.js_** is the JavaScript that is used to highlight code snippets,
  you should not need to modify this.
- **_highlight.css_** is the theme used for the code highlighting.
- **_favicon.svg_** and **_favicon.png_** the favicon that will be used. The SVG
  version is used by [newer browsers].

Generally, when you want to tweak the theme, you don't need to override all the
files. If you only need changes in the stylesheet, there is no point in
overriding all the other files. Because custom files take precedence over
built-in ones, they will not get updated with new fixes / features.

**Note:** When you override a file, it is possible that you break some
functionality. Therefore I recommend to use the file from the default theme as
template and only add / modify what you need. You can copy the default theme
into your source directory automatically by using `mdbook init --theme` just
remove the files you don't want to override.

If you completely replace all built-in themes, be sure to also set
[`output.html.preferred-dark-theme`] in the config, which defaults to the
built-in `navy` theme.

[`output.html.preferred-dark-theme`]: ../config.md#html-renderer-options
[newer browsers]: https://caniuse.com/#feat=link-icon-svg
