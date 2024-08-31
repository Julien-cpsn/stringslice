# stringslice&emsp;[![Test status]][tests]&thinsp;[![Test coverage]][codecov]&thinsp;[![Crate version]][crates]&thinsp;[![Rust version]][crates]

[test status]: https://img.shields.io/github/actions/workflow/status/staticintlucas/stringslice/test.yml?branch=main&label=tests&style=flat-square
[test coverage]: https://img.shields.io/codecov/c/gh/staticintlucas/stringslice?style=flat-square
[crate version]: https://img.shields.io/crates/v/stringslice?style=flat-square
[rust version]: https://img.shields.io/badge/rust-1.30%2B-informational?style=flat-square

[tests]: https://github.com/staticintlucas/stringslice/actions/workflows/test.yml
[codecov]: https://app.codecov.io/gh/staticintlucas/stringslice
[crates]: https://crates.io/crates/stringslice

A collection of methods to slice strings based on character indices rather than bytes.

This crate implements the `StringSlice` trait for `&str`,
containing the `slice`, `try_slice`, `substring`, and `try_substring` methods.

## Features

* Uses primitive `&str` and standard `String` types
* `#[no_std]` compatible by default
* Small footprint
  * ~50 LoC excluding blank lines, comments, and tests
  * No additional dependencies (only dev-dependencies)

## Usage

Add `stringslice` to your `Cargo.toml` file:

```toml
[dependencies]
stringslice = "0.2"
```

## Examples

The `slice` method can be used to slice a `&str`.

```rust
use stringslice::StringSlice;

assert_eq!("Ùníc😎de".slice(4..5), "😎");
assert_eq!("世界こんにちは".slice(2..), "こんにちは");
```

The `substring` method is provided for convenience and accepts
separate parameters for the start and end of the slice.

```rust
use stringslice::StringSlice;

assert_eq!("Γεια σου κόσμε".substring(9, 14), "κόσμε");
```

There are also equivalent `try_slice` and `try_substring` methods
which return `None` for invalid input.

```rust
use stringslice::StringSlice;

assert_eq!("string".try_slice(4..2), None);
```

## Run in parallel

You can have access to parallelized methods by enabling the `rayon` feature. Thanks to the [rayon](https://github.com/rayon-rs/rayon) crate, the string slicing will execute through many threads.

**Par**allel methods:
- `par_slice`
- `par_try_slice`
- `par_substring`
- `par_try_substring`

> [!WARNING]
> Using the **par**allel methods on bigger strings is recommended. Parallelism scales greatly on bigger sizes.

## Licence

Licensed under either of

* Apache License, Version 2.0 ([LICENCE-APACHE](LICENCE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0][apache-licence])
* MIT license ([LICENCE-MIT](LICENCE-MIT) or [http://opensource.org/licenses/MIT][mit-licence])

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

[apache-licence]: http://www.apache.org/licenses/LICENSE-2.0
[mit-licence]: http://opensource.org/licenses/MIT
