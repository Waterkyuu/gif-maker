# Contributing

## Git hooks

This project uses [Lefthook](https://lefthook.dev/) to run local checks before a commit.

Install Lefthook 1.10.10 or newer with one of the package managers from the Lefthook installation guide, then install the repository hooks:

```sh
lefthook install
```

The pre-commit hook runs when staged Rust or Cargo files are present:

- `rustfmt` formats staged Rust files and stages the formatting changes.
- `cargo clippy --fix` applies safe lint fixes and stages fixed files.
- `cargo check` verifies the crate before the commit is created.

`rustfmt` is scoped to staged Rust files. Cargo commands run at the crate level, so keep unrelated Rust work out of the working tree when relying on automatic Clippy fixes.

You can run the same hook manually:

```sh
lefthook run pre-commit
```

For local-only overrides, create `lefthook-local.yml`. That file is intentionally ignored by Git.
