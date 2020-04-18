use entities::*;
use io_support::write_char;
use std::{borrow::Cow, io};

///
/// HTML entity-encodes a UTF8 slice for use in attributes values.
///
/// Entity-encodes a UTF8 slice using an extensive set of entities, returning a `Cow` suitable for use
/// in HTML attribute values. All entities from `encode_minimal` are used, and further, all
/// non-alphanumeric ASCII characters are hex-encoded (`&#x__;`).
/// See the [OWASP XSS Prevention Cheat Sheet](
/// https://www.owasp.org/index.php/XSS_(Cross_Site_Scripting)_Prevention_Cheat_Sheet) for more
/// information on entity-encoding for attribute values.
///
/// # Example
/// ~~~
/// let encoded = htmlescape2::encode_attribute("\"No\", he said.");
/// assert_eq!(&encoded, "&quot;No&quot;&#x2C;&#x20;he&#x20;said&#x2E;");
/// ~~~
pub fn encode(s: &str) -> Cow<'_, str> {
    let mut owned = Vec::new();
    match encode_internal(&mut owned, s).expect("failed to write to vec: this is a bug") {
        false => s.into(),
        true => String::from_utf8(owned)
            .expect("invalid UTF8: this is a bug")
            .into(),
    }
}

///
/// HTML entity-encodes a UTF8 slice, to a writer.
///
/// Similar to `encode_attribute`, except that the output is written to a writer rather
/// than returned as a `Cow`.
pub fn encode_to<W>(writer: &mut W, s: &str) -> io::Result<()>
where
    W: io::Write,
{
    match encode_internal(writer, s)? {
        true => Ok(()),
        false => writer.write_all(s.as_bytes()),
    }
}

/// Encodes the given `s` opportunistically, will attempt to avoid
/// using the writer until it encounters an escapable char
///
/// `Ok(false)` => writer was **not** used
fn encode_internal<W>(writer: &mut W, s: &str) -> io::Result<bool>
where
    W: io::Write,
{
    // Have we allocated?
    let mut pristine = true;

    s.char_indices()
        .try_for_each(|(byte_pos, c)| match (pristine, should_encode(c)) {
            // Char stream is pristine, we have not needed an allocation yet
            (true, None) => Ok(()),
            // We encountered a escapable char, thus we cannot avoid
            // allocation, update fn state and initialize the buffer
            (true, Some(entity)) => {
                pristine = false;

                writer
                    .write_all(&s.as_bytes()[..byte_pos])
                    .and_then(|_| writer.write_all(entity.as_ref()))
            }
            // We were required to allocate in the past, thus we
            // will be returning the new buffer so we must continue
            // to fill it
            (false, Some(entity)) => writer.write_all(entity.as_ref()),
            // Write unescaped char as UTF8 to buffer
            (false, None) => write_char(writer, c),
        })?;

    Ok(!pristine)
}

fn should_encode(c: char) -> Option<AType> {
    lookup_minimal(c)
        .map(|s| s.as_bytes().into())
        .or_else(|| should_hex_encode(c).map(|a| a.into()))
}

fn should_hex_encode(c: char) -> Option<[u8; 6]> {
    let b = c as usize;
    if b < 256 && (b > 127 || !is_ascii_alnum(c)) {
        Some(html_hex_encode(c))
    } else {
        None
    }
}

fn html_hex_encode(c: char) -> [u8; 6] {
    let n = c as u8;
    [
        b'&',
        b'#',
        b'x',
        HEX_LOOKUP[((n & 0xF0) >> 4) as usize],
        HEX_LOOKUP[(n & 0x0F) as usize],
        b';',
    ]
}

#[inline]
fn is_ascii_alnum(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9')
}

#[derive(Debug)]
enum AType {
    Owned([u8; 6]),
    Borrowed(&'static [u8]),
}

impl AsRef<[u8]> for AType {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Owned(own) => &*own,
            Self::Borrowed(brw) => brw,
        }
    }
}

impl From<[u8; 6]> for AType {
    fn from(own: [u8; 6]) -> Self {
        Self::Owned(own)
    }
}

impl From<&'static [u8]> for AType {
    fn from(brw: &'static [u8]) -> Self {
        Self::Borrowed(brw)
    }
}
