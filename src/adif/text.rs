use std::ops::Range;

use super::ParseEncoding;

pub(crate) fn check_text(
    bytes: &[u8],
    encoding: ParseEncoding,
    multiline: bool,
) -> Result<(), Range<usize>> {
    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];

        if multiline && byte == b'\r' && bytes.get(index + 1) == Some(&b'\n') {
            index += 2;
            continue;
        }

        let allowed = match encoding {
            ParseEncoding::Ascii => (0x20..=0x7e).contains(&byte),
            ParseEncoding::Latin1 => (0x20..=0x7e).contains(&byte) || byte >= 0xa0,
            // Already-decoded UTF-8: ASCII CR/LF cannot be part of a multibyte sequence.
            ParseEncoding::Utf8 => byte != b'\r' && byte != b'\n',
        };

        if !allowed {
            return Err(index..index + 1);
        }

        index += 1;
    }

    Ok(())
}
