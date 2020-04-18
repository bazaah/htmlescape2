# A HTML entity encoding library for Rust

## ATTENTION

This is a fork from [rust-htmlescape](https://github.com/veddan/rust-htmlescape).
It is currently under development, and you should prefer using the upstream lib
until this message is removed.

## Example usage

All example assume a `extern crate htmlescape2;` and `use htmlescape2::{relevant functions here};` is present.

### Encoding

`htmlescape2::encode_minimal()` encodes an input string using a minimal set of HTML entities.

```rust
let title = "Cats & dogs";
let tag = format!("<title>{}</title>", encode_minimal(title));
assert_eq!(tag.as_slice(), "<title>Cats &amp; dogs</title>");
```

There is also a `htmlescape2::encode_attribute()` function for encoding strings that are to be used
as html attribute values.

### Decoding

`htmlescape2::decode_html()` decodes an encoded string, replacing HTML entities with the
corresponding characters. Named, hex, and decimal entities are supported. A `Result` value is
returned, with either the decoded string in `Ok`, or an error in `Err`.

```rust
let encoded = "Cats&#x20;&amp;&#32;dogs";
let decoded = match decode_html(encoded) {
  Err(reason) => panic!("Error {:?} at character {}", reason.kind, reason.position),
  Ok(s) => s
};
assert_eq!(decoded.as_slice(), "Cats & dogs");
```

### Avoiding allocations

Both the encoding and decoding functions are available in forms that take a `Writer` for output rather
than returning an `String`. These version can be used to avoid allocation and copying if the returned
`String` was just going to be written to a `Writer` anyway.


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
