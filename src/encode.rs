use entities::*;
use io_support::write_char;
use std::{
    borrow::Cow,
    io::{self, Write},
};

///
/// HTML entity-encodes a string for use in attributes values.
///
/// Entity-encodes a string using an extensive set of entities, giving a string suitable for use
/// in HTML attribute values. All entities from `encode_minimal` are used, and further, all
/// non-alphanumeric ASCII characters are hex-encoded (`&#x__;`).
/// See the [OWASP XSS Prevention Cheat Sheet](
/// https://www.owasp.org/index.php/XSS_(Cross_Site_Scripting)_Prevention_Cheat_Sheet) for more
/// information on entity-encoding for attribute values.
///
/// # Arguments
/// - `s` - The string to encode.
///
/// # Return value
/// The encoded string.
///
/// # Example
/// ~~~
/// let encoded = htmlescape2::encode_attribute("\"No\", he said.");
/// assert_eq!(&encoded, "&quot;No&quot;&#x2C;&#x20;he&#x20;said&#x2E;");
/// ~~~
pub fn encode_attribute(s: &str) -> Cow<'_, str> {
    let mut writer = Vec::with_capacity(s.len() * 3);
    match encode_attribute_w(s, &mut writer) {
        Err(_) => panic!(),
        Ok(_) => String::from_utf8(writer).unwrap(),
    }
}

///
/// HTML entity-encodes a string, for use in attributes values, to a writer.
///
/// Similar to `encode_attribute`, except that the output is written to a writer rather
/// than returned as a `String`.
///
/// # Arguments
/// - `s` - The string to encode.
/// - `writer` - Output is written to here.
pub fn encode_attribute_w<W: Write>(writer: &mut W, s: &str) -> io::Result<()> {
    s.chars().try_for_each(|c| encode_char(writer, c))
}

fn encode_char<W: Write>(writer: &mut W, c: char) -> io::Result<()> {
    match lookup_minimal(c) {
        Some(entity) => writer.write_all(entity.as_bytes()),
        None => {
            let b = c as usize;
            if b < 256 && (b > 127 || !is_ascii_alnum(c)) {
                write_hex(writer, c)
            } else {
                write_char(writer, c)
            }
        }
    }
}

fn write_hex<W: Write>(writer: &mut W, c: char) -> io::Result<()> {
    let hex = b"0123456789ABCDEF";
    writer.write(b"&#x")?;
    let n = c as u8;
    let bytes = [
        hex[((n & 0xF0) >> 4) as usize],
        hex[(n & 0x0F) as usize],
        b';',
    ];
    writer.write_all(&bytes)
}

fn is_ascii_alnum(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9')
}
