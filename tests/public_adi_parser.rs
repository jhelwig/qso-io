//! Consumer tests: only the public API is available in this module.
use qso_io::adif::{
    DecodeError, LengthUnit, ParseEncoding, ParseOpts,
    adi::{Parser, Section, StructuralErrorKind, SyntaxErrorKind},
};

fn record(section: Section<'_>) -> qso_io::adif::adi::Record<'_> {
    match section {
        Section::Record(record) => record,
        other => panic!("expected record, got {other:?}"),
    }
}

#[test]
fn options_and_read_only_values_are_usable_by_a_consumer() {
    for (encoding, field_length, bytes, value) in [
        (
            ParseEncoding::Ascii,
            LengthUnit::Bytes,
            &b"<nAmE:3:S>Ray<EOR>"[..],
            "Ray",
        ),
        (
            ParseEncoding::Ascii,
            LengthUnit::UnicodeScalars,
            &b"<nAmE:3:S>Ray<EOR>"[..],
            "Ray",
        ),
        (
            ParseEncoding::Latin1,
            LengthUnit::Bytes,
            &b"<nAmE:4:S>Jos\xe9<EOR>"[..],
            "José",
        ),
        (
            ParseEncoding::Latin1,
            LengthUnit::UnicodeScalars,
            &b"<nAmE:4:S>Jos\xe9<EOR>"[..],
            "José",
        ),
        (
            ParseEncoding::Utf8,
            LengthUnit::Bytes,
            "<nAmE:5:S>José<EOR>".as_bytes(),
            "José",
        ),
        (
            ParseEncoding::Utf8,
            LengthUnit::UnicodeScalars,
            "<nAmE:4:S>José<EOR>".as_bytes(),
            "José",
        ),
    ] {
        let mut parser = Parser::new(
            bytes,
            ParseOpts {
                encoding,
                field_length,
            },
        );
        let record = record(parser.next().unwrap().unwrap());
        let field = record.field("NAME").unwrap();
        assert_eq!(field.name(), "nAmE");
        assert_eq!(field.text().unwrap(), value);
        assert_eq!(field.data_type(), Some(qso_io::adif::DataType::String));
        assert_eq!(
            field.declared_length(),
            if field_length == LengthUnit::Bytes {
                field.raw_data().len()
            } else {
                value.chars().count()
            }
        );
        assert_eq!(&bytes[field.value_span()], field.raw_data());
        assert!(parser.next().is_none());
    }
}

#[test]
fn sections_have_syntactic_spans_and_can_outlive_the_parser() {
    let bytes = b"P<PROGRAMID:0><EOH>\r\n<NAME:1>A<EOR>\nignore<NAME:0><EOR>\r\n";
    let sections: Vec<_> = Parser::new(bytes, ParseOpts::default())
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(sections.len(), 3);
    let Section::Header(header) = &sections[0] else {
        panic!("missing header");
    };
    assert_eq!(header.span(), 0..19);
    assert_eq!(header.text_span(), 0..1);
    assert_eq!(header.raw_text(), b"P");
    assert_eq!(header.text().unwrap(), "P");
    assert_eq!(header.fields().len(), 1);
    assert_eq!(header.field("programid").unwrap().value_span(), 14..14);
    for (section, expected) in sections.into_iter().skip(1).zip([21..35, 42..55]) {
        let record = record(section);
        assert_eq!(record.span(), expected);
        let field = record.field("name").unwrap();
        assert_eq!(&bytes[field.value_span()], field.raw_data());
        assert!(field.descriptor_span().end <= field.value_span().start);
    }
}

#[test]
fn a_valid_prefix_is_yielded_before_one_terminal_error() {
    let bytes = b"P<EOH><NAME:1>A<EOR><NAME:1>B<EOR><NAME:3>Ra";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    assert!(matches!(
        parser.next().unwrap().unwrap(),
        Section::Header(_)
    ));
    assert_eq!(
        record(parser.next().unwrap().unwrap())
            .field("NAME")
            .unwrap()
            .text()
            .unwrap(),
        "A"
    );
    assert_eq!(
        record(parser.next().unwrap().unwrap())
            .field("NAME")
            .unwrap()
            .text()
            .unwrap(),
        "B"
    );
    let error = parser.next().unwrap().unwrap_err();
    assert_eq!(error.span(), bytes.len()..bytes.len());
    assert!(
        matches!(error.kind(), StructuralErrorKind::TruncatedValue { field_name, declared: 3, available: 2, unit: LengthUnit::Bytes } if field_name == "NAME")
    );
    for _ in 0..3 {
        assert!(parser.next().is_none());
    }
}

#[test]
fn duplicate_errors_keep_original_locations_and_do_not_yield_the_bad_section() {
    for bytes in [
        &b"<CALL:1>A<EOR><NAME:1>B<name:1>B<EOR>"[..],
        &b"P<NAME:1>B<name:1>B<EOH>"[..],
    ] {
        let mut parser = Parser::new(bytes, ParseOpts::default());
        if bytes.starts_with(b"<CALL") {
            assert!(parser.next().unwrap().is_ok());
        }
        let error = parser.next().unwrap().unwrap_err();
        let StructuralErrorKind::DuplicateField {
            first_span,
            second_span,
            field_name,
        } = error.kind()
        else {
            panic!("{error:?}");
        };
        assert!(field_name.eq_ignore_ascii_case("NAME"));
        assert_eq!(&bytes[first_span.clone()], b"<NAME:1>");
        assert_eq!(&bytes[second_span.clone()], b"<name:1>");
        assert_eq!(error.span(), *second_span);
        assert!(parser.next().is_none());
    }
}

#[test]
fn clean_exhaustion_is_fused_and_accepts_ignored_separators() {
    fn requires_fused(_: impl std::iter::FusedIterator) {}
    requires_fused(Parser::new(b"", ParseOpts::default()));
    for bytes in [
        &b""[..],
        &b"<NAME:1>A<EOR>\r\n\n"[..],
        &b"<NAME:1>A<EOR> trailing text"[..],
    ] {
        let mut parser = Parser::new(bytes, ParseOpts::default());
        for section in parser.by_ref() {
            section.unwrap();
        }
        for _ in 0..3 {
            assert!(parser.next().is_none());
        }
    }
}

#[test]
fn decoding_failures_are_locatable_and_do_not_stop_later_records() {
    let bytes = b"P\xff<EOH><NAME:3>A\xffB<EOR><NAME:1>C<EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let Section::Header(header) = parser.next().unwrap().unwrap() else {
        panic!("missing header");
    };
    let Err(DecodeError::NonAscii { offset }) = header.text() else {
        panic!("expected non-ASCII error");
    };
    assert_eq!(header.text_span(), 0..2);
    assert_eq!(bytes[header.text_span().start + offset], 0xff);
    let bad_record = record(parser.next().unwrap().unwrap());
    let field = bad_record.field("name").unwrap();
    let Err(DecodeError::NonAscii { offset }) = field.text() else {
        panic!("expected non-ASCII error");
    };
    assert_eq!(bytes[field.value_span().start + offset], 0xff);
    assert_eq!(field.raw_data(), b"A\xffB");
    assert_eq!(
        record(parser.next().unwrap().unwrap())
            .field("name")
            .unwrap()
            .text()
            .unwrap(),
        "C"
    );
    assert!(parser.next().is_none());
}

#[test]
fn scalar_utf8_errors_retain_causes_and_do_not_search_for_recovery() {
    use std::error::Error;
    for (bytes, expected_span, error_len) in [
        (&b"<NAME:1>\xff<EOR><CALL:1>A<EOR>"[..], 8..9, Some(1)),
        (&b"<NAME:1>\xe2A"[..], 8..9, Some(1)),
        (&b"<NAME:1>\xe2\x82"[..], 8..10, None),
    ] {
        let mut parser = Parser::new(
            bytes,
            ParseOpts {
                encoding: ParseEncoding::Utf8,
                field_length: LengthUnit::UnicodeScalars,
            },
        );
        let error = parser.next().unwrap().unwrap_err();
        assert_eq!(error.span(), expected_span);
        let StructuralErrorKind::InvalidUtf8 { cause } = error.kind() else {
            panic!("{error:?}");
        };
        assert_eq!(cause.error_len(), error_len);
        assert!(
            error
                .source()
                .unwrap()
                .downcast_ref::<std::str::Utf8Error>()
                .is_some()
        );
        assert!(parser.next().is_none());
    }
}

#[test]
fn public_syntax_errors_distinguish_missing_ends_and_bad_descriptors() {
    for (bytes, reason) in [
        (&b"P<NAME:1>A"[..], SyntaxErrorKind::MissingEoh),
        (&b"<NAME:1>A"[..], SyntaxErrorKind::MissingEor),
        (&b"<NAME:x>A<EOR>"[..], SyntaxErrorKind::InvalidLength),
        (
            &b"<NAME:1:Z>A<EOR>"[..],
            SyntaxErrorKind::UnknownTypeIndicator,
        ),
        (&b"<NAME:1"[..], SyntaxErrorKind::MalformedDescriptor),
    ] {
        let mut parser = Parser::new(bytes, ParseOpts::default());
        let error = parser.next().unwrap().unwrap_err();
        assert!(
            matches!(error.kind(), StructuralErrorKind::Syntax { reason: actual } if *actual == reason),
            "{error:?}"
        );
        assert!(parser.next().is_none());
    }
}
