# Running `mdbook` in Continuous Integration

While the following examples use Travis CI, their principles should
straightforwardly transfer to other continuous integration providers as well.

## Ensuring Your Book Builds and Tests Pass

Here is a sample Travis CI `.travis.yml` configuration that ensures `mdbook
build` and `mdbook test` run successfully. The key to fast CI turnaround times
is caching `mdbook` installs, so that you aren't compiling `mdbook` on every CI
run.

```yaml
language: rust
sudo: false

cache:
  - cargo

rust:
  - stable

before_script:
  - (test -x $HOME/.cargo/bin/cargo-install-update || cargo install cargo-update)
  - (test -x $HOME/.cargo/bin/mdbook || cargo install --vers "^0.3" mdbook)
  - cargo install-update -a

script:
  - mdbook build path/to/mybook && mdbook test path/to/mybook
```

## Deploying Your Book to GitHub Pages

Following these instructions will result in your book being published to GitHub
pages after a successful CI run on your repository's `master` branch.

First, create a new GitHub "Personal Access Token" with the "public_repo"
permissions (or "repo" for private repositories). Go to your repository's Travis
CI settings page and add an environment variable named `GITHUB_TOKEN` that is
marked secure and *not* shown in the logs.

Then, append this snippet to your `.travis.yml` and update the path to the
`book` directory:

```yaml
deploy:
  provider: pages
  skip-cleanup: true
  github-token: $GITHUB_TOKEN
  local-dir: path/to/mybook/book
  keep-history: false
  on:
    branch: master
```

That's it!

### Deploying to GitHub Pages manually

If your CI doesn't support GitHub pages, or you're deploying somewhere else
with integrations such as Github Pages:
 *note: you may want to use different tmp dirs*:

```console
$> git worktree add /tmp/book gh-pages
$> mdbook build
$> rm -rf /tmp/book/* # this won't delete the .git directory
$> cp -rp book/* /tmp/book/
$> cd /tmp/book
$> git add -A
$> git commit 'new book message'
$> git push origin gh-pages
$> cd -
```

Or put this into a Makefile rule:

```makefile
.PHONY: deploy
deploy: book
	@echo "====> deploying to github"
	git worktree add /tmp/book gh-pages
	rm -rf /tmp/book/*
	cp -rp book/* /tmp/book/
	cd /tmp/book && \
		git add -A && \
		git commit -m "deployed on $(shell date) by ${USER}" && \
		git push origin gh-pages
```

## Alternative example running GitHub Actions

Here's a GitHub Actions workflow definition that will build your book, upload an epub version to Github Release and publish the html as a GitHub Pages, for any push. Just copy / paste this in your repository's `.github/workflows/mdbook.yml` ! 

```yaml
name: Generate ebook

on: 
  push:

jobs:
  ebook:
    name: Build and upload
    runs-on: ubuntu-latest
    steps:
      # Install Rust & its package manager Cargo
      - name: Install cargo
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal

      # Cache installation assets
      - name: Cache cargo registry
        uses: actions/cache@v1
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      - name: Cache cargo index
        uses: actions/cache@v1
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
      - name: Cache cargo build
        uses: actions/cache@v1
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}

      # Install mdbook and requirements
      - name: Install mdbook and epub plugin
        uses: actions-rs/cargo@v1
        with:
          command: install
          args: mdbook mdbook-epub

      # Ebook generation
      - name: Checkout
        uses: actions/checkout@v2

      # Here you can add one or more steps, like generating a summary or whatever
      # - name: Generate summary
      #   run: ./generate_summary.sh

      - name: Generate ebook from markdown
        run: mdbook build

      # ePub upload as a Github release asset
      - name: Create Release
        id: create-release
        uses: actions/create-release@v1.0.0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}
          draft: false
          prerelease: false

      - name: Upload epub to release
        id: upload-release-asset
        uses: actions/upload-release-asset@v1.0.1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          upload_url: ${{ steps.create-release.outputs.upload_url }} # This pulls from the CREATE RELEASE step above, referencing it's ID to get its outputs object, which include a `upload_url`. See this blog post for more info: https://jasonet.co/posts/new-features-of-github-actions/#passing-data-to-future-steps
          asset_path: ./book/epub/Songbook.epub
          asset_name: songbook.epub
          asset_content_type: application/epub+zip

      # HTML publication as Github Page
      - name: Publish HTML
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./book/html
          publich_branch: ${{ github.ref }}
```
