# MathJax Support

mdBook has optional support for math equations through
[MathJax](https://www.mathjax.org/).

To enable MathJax, you need to add the `mathjax-support` key to your `book.toml`
under the `output.html` section.

```toml
[output.html]
mathjax-support = true
```

>**Note:** The usual delimiters MathJax uses are not yet supported. You can't
currently use `$$ ... $$` as delimiters and the `\[ ... \]` delimiters need an
extra backslash to work. Hopefully this limitation will be lifted soon.

>**Note:** When you use double backslashes in MathJax blocks (for example in
> commands such as `\begin{cases} \frac 1 2 \\ \frac 3 4 \end{cases}`) you need
> to add _two extra_ backslashes (e.g., `\begin{cases} \frac 1 2 \\\\ \frac 3 4
> \end{cases}`).


### Inline equations
Inline equations are delimited by `\\(` and `\\)`. So for example, to render the
following inline equation \\( \int x dx = \frac{x^2}{2} + C \\) you would write
the following:
```
\\( \int x dx = \frac{x^2}{2} + C \\)
```

### Block equations
Block equations are delimited by `\\[` and `\\]`. To render the following
equation

\\[ \mu = \frac{1}{N} \sum_{i=0} x_i \\]


you would write:

```bash
\\[ \mu = \frac{1}{N} \sum_{i=0} x_i \\]
```
