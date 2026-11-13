use super::*;

#[test]
fn preamble_decoding_failure_is_terminal_at_its_byte_span() {
    let mut parser = Parser::new(b"P\xff<EOH><X:1>A<EOR>", ParseOpts::default());
    let error = next_error(&mut parser);
    assert_eq!(error.span(), 1..2);
    assert_exhausted(&mut parser);
    assert_exhausted(&mut parser);
}

#[test]
fn field_decoding_failure_preserves_only_completed_prefix() {
    let mut parser = Parser::new(
        b"<X:1>A<EOR><X:1>\xff<EOR><X:1>C<EOR>",
        ParseOpts::default(),
    );
    let _ = next_section(&mut parser);
    let error = next_error(&mut parser);
    assert_eq!(error.span(), 16..17);
    assert!(
        matches!(
            error.kind(),
            error::StructuralErrorKind::Decoding {
                cause: DecodeError::NonAscii { offset: 0 }
            }
        ),
        "unexpected value: {:?}",
        error.kind()
    );
    assert_exhausted(&mut parser);
    assert_exhausted(&mut parser);
}
