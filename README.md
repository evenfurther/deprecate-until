# deprecate-until

[![Current Version](https://img.shields.io/crates/v/deprecate_until.svg)](https://crates.io/crates/deprecate_until)
[![Documentation](https://docs.rs/deprecate_until/badge.svg)](https://docs.rs/deprecate_until)
[![License: Apache-2.0/MIT](https://img.shields.io/crates/l/deprecate_until.svg)](#license)

This crate introduces a new `deprecate_until` attribute which helps
crate authors not to remove to delete obsolete items when some version
is reached. When the specified semver condition is verified, the crate
will not compile anymore and hopefully this will be caught by the CI
or by `cargo install` before the release actually happens.

## Usage

The `deprecate_until` attribute supports the same arguments as
`deprecate`, *i.e.*, `note` and `since`. It also requires a `remove`
argument, which is a [semver](https://semver.org/) requirement
expression in a string.

## Example

The following code

```rust
use deprecate_until::deprecate_until;

#[deprecate_until(remove = ">= 4.x", note = "use `some_new_function` instead")]
fn old_function() {
    todo!()
}
```

will give a warning when version 3.8 of the crate is used in a project:

```txt
warning: use of deprecated function `old_function`: use `some_new_function` instead (removal scheduled for version >= 4.x)
  |
4 | fn old_function() {
  |    ^^^^^^^^^^^^
```

It will also cause a compilation error in version 4.0.0 of the crate if you forgot to remove it:

```txt
error: version `4.0.0` matches `>= 4.x`, item should be removed
  |
3 | #[deprecate_until(remove = ">= 4.x", note = "use `some_new_function` instead")]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
