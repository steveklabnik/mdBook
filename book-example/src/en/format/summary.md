# SUMMARY.md

The summary file is used by mdBook to know what chapters to include, in what
order they should appear, what their hierarchy is and where the source files
are. Without this file, there is no book.

Even though `SUMMARY.md` is a markdown file, the formatting is very strict to
allow for easy parsing. Let's see how you should format your `SUMMARY.md` file.

#### Structure

1. ***Title*** It's common practice to begin with a title, generally <code
   class="language-markdown"># Summary</code>. But it is not mandatory, the
   parser just ignores it. So you can too if you feel like it.

2. ***Prefix Chapter*** Before the main numbered chapters you can add a couple
   of elements that will not be numbered. This is useful for forewords,
   introductions, etc. There are however some constraints. You can not nest
   prefix chapters, they should all be on the root level. And you can not add
   prefix chapters once you have added numbered chapters.
   ```markdown
   [Title of prefix element](relative/path/to/markdown.md)
   ```

3. ***Part Title:*** Headers can be used as a title for the following numbered
   chapters. This can be used to logically separate different sections
   of book. The title is rendered as unclickable text.
   Titles are optional, and the numbered chapters can be broken into as many
   parts as desired.

4. ***Numbered Chapter*** Numbered chapters are the main content of the book,
   they will be numbered and can be nested, resulting in a nice hierarchy
   (chapters, sub-chapters, etc.)
   ```markdown
   # Title of Part

   - [Title of the Chapter](relative/path/to/markdown.md)

   # Title of Another Part

   - [More Chapters](relative/path/to/markdown2.md)
   ```
   You can either use `-` or `*` to indicate a numbered chapter.

5. ***Suffix Chapter*** After the numbered chapters you can add a couple of
   non-numbered chapters. They are the same as prefix chapters but come after
   the numbered chapters instead of before.

All other elements are unsupported and will be ignored at best or result in an
error.

#### Other elements

- ***Separators*** In between chapters you can add a separator. In the HTML renderer
  this will result in a line being rendered in the table of contents. A separator is
  a line containing exclusively dashes and at least three of them: `---`.
- ***Draft chapters*** Draft chapters are chapters without a file and thus content.
  The purpose of a draft chapter is to signal future chapters still to be written.
  Or when still laying out the structure of the book to avoid creating the files
  while you are still changing the structure of the book a lot.
  Draft chapters will be rendered in the HTML renderer as disabled links in the table
  of contents, as you can see for the next chapter in the table of contents on the left.
  Draft chapters are written like normal chapters but without writing the path to the file
  ```markdown
  - [Draft chapter]()
  ```
