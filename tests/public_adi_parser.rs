//! Consumer tests: only the public API is available in this module.
use qso_io::adif::adi::StructuralError;
use qso_io::adif::{
    DecodeError, LengthUnit, ParseEncoding, ParseOpts,
    adi::{Parser, Section, StructuralErrorKind, SyntaxErrorKind},
};

#[track_caller]
fn next_section<'a>(parser: &mut Parser<'a>) -> Section<'a> {
    match parser.next() {
        Some(Ok(section)) => section,
        other => panic!("expected a complete section, got {other:?}; remaining parser: {parser:?}"),
    }
}

#[track_caller]
fn next_error(parser: &mut Parser<'_>) -> StructuralError {
    match parser.next() {
        Some(Err(error)) => error,
        other => {
            panic!("expected a terminal parsing error, got {other:?}; remaining parser: {parser:?}")
        }
    }
}

#[track_caller]
fn assert_exhausted(parser: &mut Parser<'_>) {
    let actual = parser.next();
    assert!(
        actual.is_none(),
        "expected permanent exhaustion, got {actual:?}; remaining parser: {parser:?}"
    );
}

#[track_caller]
fn record(section: Section<'_>) -> qso_io::adif::adi::Record<'_> {
    match section {
        Section::Record(record) => record,
        other => panic!("expected record, got {other:?}"),
    }
}

#[track_caller]
fn named<'r, 'a>(
    record: &'r qso_io::adif::adi::Record<'a>,
    name: &str,
) -> &'r qso_io::adif::Field<'a> {
    record
        .field(name)
        .unwrap_or_else(|| panic!("expected field {name:?}, got {record:?}"))
}

#[track_caller]
fn definition<'r, 'a>(
    entry: &'r qso_io::adif::adi::HeaderEntry<'a>,
) -> &'r qso_io::adif::adi::UserDefinition<'a> {
    match entry {
        qso_io::adif::adi::HeaderEntry::UserDefinition(definition) => definition,
        other => panic!("expected USERDEF declaration, got {other:?}"),
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
        let record = record(next_section(&mut parser));
        let field = named(&record, "NAME");
        assert_eq!(
            field.name(),
            "nAmE",
            "{encoding:?}, {field_length:?}, input {bytes:?}"
        );
        assert_eq!(
            field.text(),
            value,
            "{encoding:?}, {field_length:?}, input {bytes:?}"
        );
        assert_eq!(
            field.data_type(),
            Some(qso_io::adif::DataType::String),
            "{encoding:?}, {field_length:?}, input {bytes:?}"
        );
        assert_eq!(
            field.declared_length(),
            if field_length == LengthUnit::Bytes {
                field.raw_data().len()
            } else {
                value.chars().count()
            },
            "{encoding:?}, {field_length:?}, input {bytes:?}"
        );
        assert_eq!(
            &bytes[field.value_span()],
            field.raw_data(),
            "{encoding:?}, {field_length:?}, input {bytes:?}"
        );
        assert_exhausted(&mut parser);
    }
}

#[test]
fn sections_have_syntactic_spans_and_can_outlive_the_parser() {
    let bytes = b"P<PROGRAMID:0><EOH>\r\n<NAME:1>A<EOR>\nignore<NAME:0><EOR>\r\n";
    let sections: Vec<_> = Parser::new(bytes, ParseOpts::default())
        .collect::<Result<_, _>>()
        .unwrap_or_else(|error| {
            panic!("input {bytes:?}: expected complete sections, got {error:?}")
        });
    assert_eq!(sections.len(), 3);
    let Section::Header(header) = &sections[0] else {
        panic!("expected Section::Header(header), got {:?}", sections[0]);
    };
    assert_eq!(header.span(), 0..19);
    assert_eq!(header.text_span(), 0..1);
    assert_eq!(header.raw_text(), b"P");
    assert_eq!(header.text(), "P");
    assert_eq!(header.entries().len(), 1);
    assert_eq!(
        header
            .field("programid")
            .expect("header should contain PROGRAMID from the supplied descriptor")
            .value_span(),
        14..14
    );
    for (section, expected) in sections.into_iter().skip(1).zip([21..35, 42..55]) {
        let record = record(section);
        assert_eq!(record.span(), expected);
        let field = named(&record, "name");
        assert_eq!(&bytes[field.value_span()], field.raw_data());
        assert!(field.descriptor_span().end <= field.value_span().start);
    }
}

#[test]
fn a_valid_prefix_is_yielded_before_one_terminal_error() {
    let bytes = b"P<EOH><NAME:1>A<EOR><NAME:1>B<EOR><NAME:3>Ra";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let section = next_section(&mut parser);
    assert!(
        matches!(section, Section::Header(_)),
        "expected header, got {section:?}"
    );
    assert_eq!(
        record(next_section(&mut parser))
            .field("NAME")
            .expect("section should contain NAME from the supplied descriptor")
            .text(),
        "A"
    );
    assert_eq!(
        record(next_section(&mut parser))
            .field("NAME")
            .expect("section should contain NAME from the supplied descriptor")
            .text(),
        "B"
    );
    let error = next_error(&mut parser);
    assert_eq!(error.span(), bytes.len()..bytes.len());
    assert!(
        matches!(error.kind(), StructuralErrorKind::TruncatedValue { field_name, declared: 3, available: 2, unit: LengthUnit::Bytes } if field_name == "NAME"),
        "unexpected value: {:?}",
        error.kind()
    );
    for _ in 0..3 {
        assert_exhausted(&mut parser);
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
            let _ = next_section(&mut parser);
        }
        let error = next_error(&mut parser);
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
        assert_exhausted(&mut parser);
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
        let records: Vec<_> = parser
            .by_ref()
            .map(|section| {
                record(section.unwrap_or_else(|error| panic!("input {bytes:?}: {error}")))
            })
            .collect();
        assert_eq!(
            records.len(),
            usize::from(!bytes.is_empty()),
            "input {bytes:?}: {records:?}"
        );
        if let Some(record) = records.first() {
            assert_eq!(
                record
                    .field("NAME")
                    .expect("record should retain NAME from the supplied <NAME:1>A field")
                    .text(),
                "A",
                "input {bytes:?}"
            );
        }
        for _ in 0..3 {
            assert_exhausted(&mut parser);
        }
    }
}

#[test]
fn decoding_failures_are_locatable_and_terminal() {
    for (bytes, expected) in [
        (&b"P\xff<EOH><NAME:1>C<EOR>"[..], 1..2),
        (&b"<NAME:3>A\xffB<EOR><NAME:1>C<EOR>"[..], 9..10),
    ] {
        let mut parser = Parser::new(bytes, ParseOpts::default());
        let error = next_error(&mut parser);
        assert_eq!(error.span(), expected);
        assert!(
            matches!(
                error.kind(),
                StructuralErrorKind::Decoding {
                    cause: DecodeError::NonAscii { .. }
                }
            ),
            "unexpected value: {:?}",
            error.kind()
        );
        assert_exhausted(&mut parser);
    }
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
        let error = next_error(&mut parser);
        assert_eq!(error.span(), expected_span);
        let StructuralErrorKind::Decoding {
            cause: DecodeError::Utf8(cause),
        } = error.kind()
        else {
            panic!("{error:?}");
        };
        assert_eq!(cause.error_len(), error_len);
        let decoding = error
            .source()
            .unwrap_or_else(|| panic!("expected decoding cause for {bytes:?}: {error:?}"));
        let utf8 = decoding
            .source()
            .unwrap_or_else(|| panic!("expected UTF-8 source for {bytes:?}: {decoding:?}"));
        assert!(
            utf8.downcast_ref::<std::str::Utf8Error>().is_some(),
            "expected UTF-8 source for {bytes:?}, got {utf8:?}"
        );
        assert_exhausted(&mut parser);
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
        let error = next_error(&mut parser);
        assert!(
            matches!(error.kind(), StructuralErrorKind::Syntax { reason: actual } if *actual == reason),
            "input {bytes:?}: expected {reason:?}, got {error:?}"
        );
        assert_exhausted(&mut parser);
    }
}

#[test]
fn headers_distinguish_definitions_from_fields() {
    let bytes = b"P<PROGRAMID:1>A<USERDEF1:3:N>EPC<COMMENT:1>B<USERDEF2:19:E>SweaterSize,{S,M,L}<USERDEF3:15>ShoeSize,{5:20}<EOH><EPC:3>123<EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let header = match next_section(&mut parser) {
        Section::Header(header) => header,
        other => panic!("expected header, got {other:?}"),
    };
    assert_eq!(header.entries().len(), 5);
    let epc = definition(&header.entries()[1]);
    assert_eq!(epc.field_name(), "EPC");
    assert_eq!(epc.data_type(), Some(qso_io::adif::DataType::Number));
    assert_eq!(epc.text(), "EPC");
    assert_eq!(epc.constraint(), None, "declaration {epc:?}");
    assert!(
        header.field("USERDEF1").is_none(),
        "expected a declaration rather than an ordinary field, got {:?}",
        header.field("USERDEF1")
    );
    assert!(
        header.entry("userdef1").is_some(),
        "expected USERDEF1 entry, got {header:?}"
    );
    let sizes = definition(&header.entries()[3]);
    assert_eq!(
        sizes.constraint(),
        Some(&qso_io::adif::adi::UserDefinitionConstraint::Enumeration(
            ["S", "M", "L"].map(str::to_owned).to_vec()
        ))
    );
    let range = definition(&header.entries()[4]);
    assert_eq!(range.data_type(), None);
    assert!(
        matches!(range.constraint(), Some(qso_io::adif::adi::UserDefinitionConstraint::Range {lower,upper}) if lower == &bigdecimal::BigDecimal::from(5) && upper == &bigdecimal::BigDecimal::from(20)),
        "unexpected value: {:?}",
        range.constraint()
    );
    for entry in header.entries() {
        if let qso_io::adif::adi::HeaderEntry::UserDefinition(def) = entry {
            assert_eq!(&bytes[def.value_span()], def.raw_data());
            assert!(bytes[def.descriptor_span()].starts_with(b"<USERDEF"));
        }
    }
    assert_eq!(
        record(next_section(&mut parser))
            .field("EPC")
            .expect("record should contain EPC from the supplied <EPC:3>123 field")
            .text(),
        "123"
    );
}
