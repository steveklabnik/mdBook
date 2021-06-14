# SUMMARY.md

The summary file is used by mdBook to know what chapters to include, in what
order they should appear, what their hierarchy is and where the source files
are. Without this file, there is no book.

This markdown file must be named `SUMMARY.md`. Its formatting
is very strict and must follow the structure outlined below to allow for easy
parsing. Any element not specified below, be it formatting or textual, is likely
to be ignored at best, or may cause an error when attempting to build the book.

### Structure

1. ***Title*** - While optional, it's common practice to begin with a title, generally <code
   class="language-markdown"># Summary</code>. This is ignored by the parser however, and
   can be omitted.
   ```markdown
   # Summary
   ```

1. ***Prefix Chapter*** - Before the main numbered chapters, prefix chapters can be added
   that will not be numbered. This is useful for forewords,
   introductions, etc. There are, however, some constraints. Prefix chapters cannot be
   nested; they should all be on the root level. And you can not add
   prefix chapters once you have added numbered chapters.
   ```markdown
   [A Prefix Chapter](relative/path/to/markdown.md)

   - [First Chapter](relative/path/to/markdown2.md)
   ```

1. ***Part Title*** - Headers can be used as a title for the following numbered
   chapters. This can be used to logically separate different sections
   of the book. The title is rendered as unclickable text.
   Titles are optional, and the numbered chapters can be broken into as many
   parts as desired.
   ```markdown
   # My Part Tile

   - [First Chapter](relative/path/to/markdown.md)
   ```

1. ***Numbered Chapter*** - Numbered chapters outline the main content of the book
   and can be nested, resulting in a nice hierarchy
   (chapters, sub-chapters, etc.).
   ```markdown
   # Title of Part

   - [First Chapter](relative/path/to/markdown.md)
   - [Second Chapter](relative/path/to/markdown2.md)
      - [Sub Chapter](relative/path/to/markdown3.md)

   # Title of Another Part

   - [Another Chapter](relative/path/to/markdown4.md)
   ```
   Numbered chapters can be denoted with either `-` or `*` (do not mix delimiters). 
   
1. ***Suffix Chapter*** - Like prefix chapters, suffix chapters are unnumbered, but they come after 
   numbered chapters.
   ```markdown
   - [Last Chapter](relative/path/to/markdown.md)

   [Title of Suffix Chapter](relative/path/to/markdown2.md)
   ```

1. ***Draft chapters*** - Draft chapters are chapters without a file and thus content.
   The purpose of a draft chapter is to signal future chapters still to be written.
   Or when still laying out the structure of the book to avoid creating the files
   while you are still changing the structure of the book a lot.
   Draft chapters will be rendered in the HTML renderer as disabled links in the table
   of contents, as you can see for the next chapter in the table of contents on the left.
   Draft chapters are written like normal chapters but without writing the path to the file.
   ```markdown
   - [Draft Chapter]()
   ```

1. ***Separators*** - Separators can be added before, in between, and after any other element. They result
   in an HTML rendered line in the built table of contents.  A separator is
   a line containing exclusively dashes and at least three of them: `---`.
   ```markdown
   # My Part Title
   
   [A Prefix Chapter](relative/path/to/markdown.md)

   ---

   - [First Chapter](relative/path/to/markdown2.md)
   ```
  

### Example

Below is the markdown source for the `SUMMARY.md` for this guide, with the resulting table
of contents as rendered to the left.

```markdown
{{#include ../SUMMARY.md}}
```
