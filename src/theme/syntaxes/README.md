# Note

This folder is not copied over to the book directory when using `mdbook init`, nor is it indexed at runtime. To add new syntaxes, first add a `.sublime-syntax` file in this directory, then run (from inside the `/src/theme` directory relative to the repository root):
```shell
$ bat cache --build --source="." --target="."
$ rm themes.bin
$ rm metadata.yaml
```
Make sure you have [`bat`](https://github.com/sharkdp/bat) installed, otherwise this won't work.

Don't worry if bat says:
```
No themes were found in './themes', using the default set
```

Don't forget to delete `themes.bin` and `metadata.yaml` as they are not necessary.

Now, you may rebuild the `mdBook` binary and the new syntaxes should be included.
