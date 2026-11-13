//! ADIF 3.1.8 IV.A.1–6 framing tests and the crate's explicit encoding policies.
//!
//! These exercise qso-io's assembly, boundaries and error separation, not ADIF
//! schema validation or the underlying libraries in isolation. Fixture
//! expectations follow tests/fixtures/README.md, not upstream filenames.

use super::error::StructuralError;
use super::*;

mod declarations;
mod decoding;
mod lists;
mod values;

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
fn parse_ok<'a, O>(
    mut parser: impl winnow::Parser<Stream<'a>, O, ContextError>,
    input: &mut Stream<'a>,
) -> O {
    let bytes = *input.input.as_ref();
    parser.parse_next(input).unwrap_or_else(|error| {
        panic!(
            "expected successful parsing of {bytes:?}, got {error:?} at byte {}; stream: {input:?}",
            input.current_token_start()
        )
    })
}

fn stream(bytes: &[u8], encoding: ParseEncoding, field_length: LengthUnit) -> Stream<'_> {
    Stream {
        input: LocatingSlice::new(bytes),
        state: State {
            parser_options: ParseOpts {
                encoding,
                field_length,
            },
        },
    }
}

fn ascii_stream(bytes: &[u8]) -> Stream<'_> {
    stream(bytes, ParseEncoding::Ascii, LengthUnit::Bytes)
}

#[track_caller]
fn parse_file(bytes: &[u8], encoding: ParseEncoding, unit: LengthUnit) -> AdiFile<'_> {
    adi_file
        .parse(stream(bytes, encoding, unit))
        .unwrap_or_else(|error| {
            panic!(
                "complete ADI file should parse ({encoding:?}, {unit:?}, input {bytes:?}):\n{error}"
            )
        })
}

#[track_caller]
fn named<'r, 'a>(record: &'r Record<'a>, name: &str) -> &'r Field<'a> {
    record
        .field(name)
        .unwrap_or_else(|| panic!("expected field {name:?}, got {record:?}"))
}

#[track_caller]
fn assert_two_records(bytes: &[u8]) {
    let file = parse_file(bytes, ParseEncoding::Ascii, LengthUnit::Bytes);
    assert_eq!(file.records.len(), 2);
    assert_eq!(named(&file.records[0], "NAME").text(), "Ray");
    assert_eq!(named(&file.records[1], "NAME").text(), "Ann");
}

#[track_caller]
fn assert_not_missing_terminator(error: &ContextError, diagnostic: &str) {
    assert!(
        !error.context().any(|context| matches!(
            context,
            StrContext::Expected(StrContextValue::StringLiteral("<EOH>" | "<EOR>"))
        )),
        "field error was incorrectly reported as a missing section terminator:\n{diagnostic}"
    );
}

#[track_caller]
fn assert_truncation(
    error: &winnow::error::ParseError<Stream<'_>, ContextError>,
    field_name: &str,
    declared: usize,
    available: usize,
    unit: LengthUnit,
) {
    let expected = format!(
        "TruncatedValue for {field_name}: declared {declared} {unit:?}, \
         available {available}; position at EOF (byte {})",
        error.input().eof_offset(),
    );
    let cause = error
        .inner()
        .cause()
        .and_then(|cause| cause.downcast_ref::<AdiError>());
    assert!(
        matches!(cause, Some(AdiError::TruncatedValue {
            field_name: actual_name,
            declared: actual_declared,
            available: actual_available,
            unit: actual_unit,
        }) if actual_name == field_name
            && *actual_declared == declared
            && *actual_available == available
            && *actual_unit == unit),
        "expected {expected}\nactual cause: {cause:?}\nactual error:\n{error}"
    );
    assert_eq!(
        error.offset(),
        error.input().eof_offset(),
        "expected {expected}\nactual error:\n{error}"
    );
    assert_not_missing_terminator(error.inner(), &error.to_string());
}

fn fixture_record(bytes: &[u8], encoding: ParseEncoding, unit: LengthUnit) -> Record<'_> {
    let file = parse_file(bytes, encoding, unit);
    assert!(file.header.is_some());
    assert_eq!(file.records.len(), 1);
    file.records
        .into_iter()
        .next()
        .expect("a record should be present because the fixture record count was checked as one")
}

#[test]
fn descriptors_preserve_names_and_accept_zero_and_leading_zero_lengths() {
    for (bytes, name, length) in [
        (&b"<nAmE:003>Ray"[..], "nAmE", 3),
        (&b"<NAME:0><EOR>"[..], "NAME", 0),
    ] {
        let mut input = ascii_stream(bytes);
        let parsed = parse_ok(data_specifier, &mut input);
        assert_eq!(parsed.name, name);
        assert_eq!(parsed.length, length);
        assert_eq!(parsed.type_indicator, None);
        assert_eq!(
            *input.input.as_ref(),
            &bytes[bytes.iter().position(|b| *b == b'>').expect(
                "descriptor should contain > because the test case supplies a complete descriptor"
            ) + 1..]
        );
    }
}

#[test]
fn all_defined_type_indicators_are_case_insensitive() {
    // Known ADX-only indicators are recognized separately from ADI eligibility.
    for (indicator, expected) in [
        ('B', DataType::Boolean),
        ('N', DataType::Number),
        ('D', DataType::Date),
        ('T', DataType::Time),
        ('S', DataType::String),
        ('I', DataType::IntlString),
        ('M', DataType::MultilineString),
        ('G', DataType::IntlMultilineString),
        ('E', DataType::Enumeration),
        ('L', DataType::Location),
    ] {
        for supplied in [indicator, indicator.to_ascii_lowercase()] {
            let bytes = format!("<APP_TEST_VALUE:0:{supplied}><EOR>");
            let mut input = ascii_stream(bytes.as_bytes());
            if matches!(
                expected,
                DataType::IntlString | DataType::IntlMultilineString
            ) {
                assert_eq!(
                    data_type(&mut ascii_stream(supplied.to_string().as_bytes())).unwrap_or_else(
                        |error| panic!("expected indicator {supplied:?} to parse: {error:?}")
                    ),
                    expected
                );
                field(&mut input).expect_err(&format!(
                    "indicator {supplied} should fail because it is reserved for ADX; input {bytes:?}"
                ));
                continue;
            }
            let parsed = parse_ok(field, &mut input);
            assert_eq!(
                parsed.specifier.type_indicator,
                Some(expected),
                "{supplied}"
            );
            assert_eq!(parsed.raw_data, b"");
            assert_eq!(*input.input.as_ref(), b"<EOR>");
        }
    }
}

#[test]
fn malformed_lengths_and_descriptors_fail_structurally() {
    for bytes in [
        &b"<NAME>Ray"[..],
        &b"<NAME:>Ray"[..],
        &b"<NAME:-1>Ray"[..],
        &b"<NAME:+3>Ray"[..],
        &b"<NAME:x>Ray"[..],
        &b"<NAME:3x>Ray"[..],
        &b"<NAME:3"[..],
    ] {
        field(&mut ascii_stream(bytes)).expect_err(&format!(
            "input {bytes:?} should fail because its length descriptor is malformed"
        ));
    }
    let overflow = format!("<NAME:{}0>", usize::MAX);
    field(&mut ascii_stream(overflow.as_bytes())).expect_err(&format!(
        "length {overflow:?} should fail because it exceeds usize::MAX"
    ));
}

#[test]
fn unknown_empty_and_multiple_type_indicators_fail_structurally() {
    for bytes in [
        &b"<NAME:3:Z>Ray"[..],
        &b"<NAME:3:z>Ray"[..],
        &b"<NAME:3:>Ray"[..],
        &b"<NAME:3:SS>Ray"[..],
        &b"<NAME:3:1>Ray"[..],
    ] {
        field(&mut ascii_stream(bytes)).expect_err(&format!(
            "input {bytes:?} should fail because its type indicator is unknown, empty, or multiple characters"
        ));
    }
}

#[test]
fn invalid_declared_field_values_are_structural_errors() {
    use crate::adif::ValueError;
    for (bytes, reason, span) in [
        (
            &b"<QSO_DATE:8:D>20260230"[..],
            ValueError::InvalidDate,
            20..22,
        ),
        (
            &b"<APP_TEST_VALUE:3:N>abc"[..],
            ValueError::InvalidNumber,
            20..21,
        ),
    ] {
        let mut parser = Parser::new(bytes, ParseOpts::default());
        let error = next_error(&mut parser);
        assert_eq!(error.span(), span);
        assert!(
            matches!(error.kind(),error::StructuralErrorKind::InvalidValue {cause,..} if *cause==reason),
            "unexpected value: {:?}",
            error.kind()
        );
    }
}

#[test]
fn zero_length_fields_leave_the_next_tag_untouched_in_every_mode() {
    for encoding in [
        ParseEncoding::Ascii,
        ParseEncoding::Latin1,
        ParseEncoding::Utf8,
    ] {
        for unit in [LengthUnit::Bytes, LengthUnit::UnicodeScalars] {
            let mut input = stream(b"<NAME:0><EOR>", encoding, unit);
            let parsed = parse_ok(field, &mut input);
            assert_eq!(parsed.raw_data, b"", "{encoding:?}, {unit:?}");
            assert_eq!(parsed.text(), "", "{encoding:?}, {unit:?}");
            assert_eq!(*input.input.as_ref(), b"<EOR>", "{encoding:?}, {unit:?}");
        }
    }
}

#[test]
fn value_lengths_protect_embedded_tags_punctuation_and_line_breaks() {
    for (bytes, expected) in [
        (&b"<COMMENT:5><EOR><EOR>"[..], &b"<EOR>"[..]),
        (&b"<COMMENT:5><EOH><EOR>"[..], &b"<EOH>"[..]),
        (&b"<COMMENT:5>a:\r\nb<EOR>"[..], &b"a:\r\nb"[..]),
    ] {
        let mut input = ascii_stream(bytes);
        let parsed = parse_ok(record, &mut input);
        assert_eq!(parsed.fields.len(), 1);
        assert_eq!(parsed.fields[0].raw_data, expected);
        assert_eq!(input.eof_offset(), 0);
    }
}

#[test]
fn files_detect_headers_from_the_first_byte_and_preserve_preambles() {
    for (bytes, expected_preamble) in [
        (&b"<NAME:3>Ray<EOR>"[..], None),
        (
            &b"Generated<eOh><NAME:3>Ray<eOr>"[..],
            Some(&b"Generated"[..]),
        ),
        (
            &b"First\r\nSecond\r\n\r\n<EOH><NAME:3>Ray<EOR>"[..],
            Some(&b"First\r\nSecond\r\n\r\n"[..]),
        ),
        (&b" \n<EOH><NAME:3>Ray<EOR>"[..], Some(&b" \n"[..])),
    ] {
        let file = parse_file(bytes, ParseEncoding::Ascii, LengthUnit::Bytes);
        assert_eq!(
            file.header.as_ref().map(|h| h.text_bytes),
            expected_preamble
        );
        if let Some(header) = file.header {
            assert_eq!(
                header.text().as_bytes(),
                expected_preamble.expect(
                    "expected preamble should be present because header presence was checked above"
                )
            );
            assert!(header.entries().is_empty());
        }
        assert_eq!(file.records.len(), 1);
        assert_eq!(named(&file.records[0], "NAME").text(), "Ray");
    }
}

#[test]
fn malformed_headers_never_fall_back_to_headerless_parsing() {
    for bytes in [
        &b"Generated"[..],
        &b"Generated<PROGRAMID:3>abc"[..],
        &b"Generated<PROGRAMID:x>abc<EOH><NAME:3>Ray<EOR>"[..],
        &b"\n<NAME:3>Ray<EOR>"[..],
    ] {
        adi_file(&mut ascii_stream(bytes)).expect_err(&format!(
            "input {bytes:?} should fail because its header is malformed or lacks EOH"
        ));
    }
}

#[test]
fn ignored_inter_field_text_preserves_field_order() {
    let file = parse_file(
        b"<NAME:3>Ray # arbitrary text\r\n<QSO_DATE:8>19960513 text <qth:3>Ann tail <eOr>",
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    let fields = &file.records[0].fields;
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].specifier.name, "NAME");
    assert_eq!(fields[0].text(), "Ray");
    assert_eq!(fields[1].specifier.name, "QSO_DATE");
    assert_eq!(fields[1].text(), "19960513");
    assert_eq!(fields[2].specifier.name, "qth");
    assert_eq!(fields[2].text(), "Ann");
}

#[test]
fn adjacent_records_remain_separate() {
    assert_two_records(b"<NAME:3>Ray<EOR><NAME:3>Ann<EOR>");
}

#[test]
fn line_breaks_between_records_are_ignored() {
    assert_two_records(b"<NAME:3>Ray<EOR>\r\n<NAME:3>Ann<EOR>");
}

#[test]
fn arbitrary_text_between_records_is_ignored() {
    assert_two_records(b"<NAME:3>Ray<EOR>record separator <NAME:3>Ann<EOR>");
}

#[test]
fn terminal_lf_is_accepted_as_an_explicit_compatibility_policy() {
    let file = parse_file(
        b"<NAME:3>Ray<EOR>\n",
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    assert_eq!(file.records.len(), 1);
}

#[test]
fn terminal_crlf_is_accepted_as_an_explicit_compatibility_policy() {
    let file = parse_file(
        b"<NAME:3>Ray<EOR>\r\n",
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    assert_eq!(file.records.len(), 1);
}

#[test]
fn empty_or_unterminated_records_fail_structurally() {
    for bytes in [
        &b"<EOR>"[..],
        &b"<NAME:3>Ray"[..],
        &b"<NAME:3>Ray ignored text"[..],
    ] {
        record(&mut ascii_stream(bytes)).expect_err(&format!(
            "input {bytes:?} should fail because a record needs fields and an EOR terminator"
        ));
    }
}

#[test]
fn empty_input_is_an_empty_sequence_under_the_crate_contract() {
    let file = parse_file(b"", ParseEncoding::Ascii, LengthUnit::Bytes);
    assert!(file.header.is_none());
    assert!(file.records.is_empty());
}

#[test]
fn encoding_and_length_options_select_the_expected_value_boundaries() {
    for (encoding, unit, bytes, raw, text) in [
        (
            ParseEncoding::Ascii,
            LengthUnit::Bytes,
            &b"<NAME:3>Ray<EOR>"[..],
            &b"Ray"[..],
            "Ray",
        ),
        (
            ParseEncoding::Ascii,
            LengthUnit::UnicodeScalars,
            &b"<NAME:3>Ray<EOR>"[..],
            &b"Ray"[..],
            "Ray",
        ),
        (
            ParseEncoding::Latin1,
            LengthUnit::Bytes,
            &b"<NAME:4>Jos\xe9<EOR>"[..],
            &b"Jos\xe9"[..],
            "José",
        ),
        (
            ParseEncoding::Latin1,
            LengthUnit::UnicodeScalars,
            &b"<NAME:4>Jos\xe9<EOR>"[..],
            &b"Jos\xe9"[..],
            "José",
        ),
        (
            ParseEncoding::Utf8,
            LengthUnit::Bytes,
            "<NAME:5>José<EOR>".as_bytes(),
            "José".as_bytes(),
            "José",
        ),
        (
            ParseEncoding::Utf8,
            LengthUnit::UnicodeScalars,
            "<NAME:4>José<EOR>".as_bytes(),
            "José".as_bytes(),
            "José",
        ),
    ] {
        let mut input = stream(bytes, encoding, unit);
        let parsed = parse_ok(field, &mut input);
        assert_eq!(parsed.raw_data, raw, "{encoding:?}/{unit:?}");
        assert_eq!(
            parsed.text(),
            text,
            "{encoding:?}, {unit:?}, input {bytes:?}"
        );
        assert_eq!(
            *input.input.as_ref(),
            b"<EOR>",
            "{encoding:?}, {unit:?}, input {bytes:?}"
        );
    }
}

#[test]
fn utf8_scalar_lengths_count_all_widths_and_combining_scalars() {
    for (bytes, expected) in [
        ("<NAME:4>Aé€😀<NAME:3>Ray".as_bytes(), "Aé€😀"),
        ("<NAME:2>e\u{301}<NAME:3>Ray".as_bytes(), "e\u{301}"),
    ] {
        let mut input = stream(bytes, ParseEncoding::Utf8, LengthUnit::UnicodeScalars);
        let parsed = parse_ok(field, &mut input);
        assert_eq!(parsed.raw_data, expected.as_bytes());
        assert_eq!(parsed.text(), expected);
        assert_eq!(*input.input.as_ref(), b"<NAME:3>Ray");
        assert_eq!(parse_ok(field, &mut input).text(), "Ray");
        assert_eq!(input.eof_offset(), 0);
    }
}

#[test]
fn decoding_failures_stop_before_following_fields() {
    for (encoding, bytes, incomplete) in [
        (
            ParseEncoding::Ascii,
            &b"<NAME:2>\xc3\xa9<NAME:3>Ray"[..],
            false,
        ),
        (ParseEncoding::Ascii, &b"<NAME:1>\xff<NAME:3>Ray"[..], false),
        (ParseEncoding::Utf8, &b"<NAME:1>\xff<NAME:3>Ray"[..], false),
        (ParseEncoding::Utf8, &b"<NAME:1>\xc3<NAME:3>Ray"[..], true),
    ] {
        let mut parser = Parser::new(
            bytes,
            ParseOpts {
                encoding,
                field_length: LengthUnit::Bytes,
            },
        );
        let error = next_error(&mut parser);
        assert_eq!(error.span(), 8..9);
        match (encoding, error.kind()) {
            (
                ParseEncoding::Ascii,
                error::StructuralErrorKind::Decoding {
                    cause: DecodeError::NonAscii { offset },
                },
            ) => assert_eq!(*offset, 0),
            (
                ParseEncoding::Utf8,
                error::StructuralErrorKind::Decoding {
                    cause: DecodeError::Utf8(cause),
                },
            ) => {
                assert_eq!(cause.valid_up_to(), 0);
                assert_eq!(cause.error_len(), if incomplete { None } else { Some(1) });
            }
            other => panic!("{other:?}"),
        }
        assert_exhausted(&mut parser);
    }
}

#[test]
fn non_ascii_errors_locate_the_first_offending_byte_in_the_original_document() {
    for unit in [LengthUnit::Bytes, LengthUnit::UnicodeScalars] {
        for (raw, expected_offset, document_offset) in [
            (&b"\x80AB\xff"[..], 0, 19),
            (&b"A\x80B\xff"[..], 1, 20),
            (&b"ABC\xff"[..], 3, 22),
            (&b"A\xc3\xa9!"[..], 1, 20),
        ] {
            let mut bytes = b"<CALL:3>Ray<NAME:4>".to_vec();
            bytes.extend_from_slice(raw);
            bytes.extend_from_slice(b"<QTH:3>Ann<EOR>");
            let mut parser = Parser::new(
                &bytes,
                ParseOpts {
                    encoding: ParseEncoding::Ascii,
                    field_length: unit,
                },
            );
            let error = next_error(&mut parser);
            assert_eq!(error.span(), document_offset..document_offset + 1);
            assert!(
                matches!(error.kind(),error::StructuralErrorKind::Decoding {
                cause:DecodeError::NonAscii { offset }
            } if *offset == expected_offset),
                "unexpected value: {:?}",
                error.kind()
            );
            assert_exhausted(&mut parser);
        }
    }
}

#[test]
fn byte_lengths_splitting_a_utf8_scalar_fail_at_the_bounded_sequence() {
    let mut parser = Parser::new(
        "<NAME:4>José<EOR>".as_bytes(),
        ParseOpts {
            encoding: ParseEncoding::Utf8,
            field_length: LengthUnit::Bytes,
        },
    );
    let error = next_error(&mut parser);
    assert_eq!(error.span(), 11..12);
    let error::StructuralErrorKind::Decoding {
        cause: DecodeError::Utf8(cause),
    } = error.kind()
    else {
        panic!("{error:?}")
    };
    assert_eq!(cause.valid_up_to(), 3);
    assert_eq!(cause.error_len(), None);
    assert_exhausted(&mut parser);
}

#[test]
fn latin1_decoding_keeps_c1_controls_instead_of_windows_1252_mapping() {
    let mut input = stream(
        b"<NAME:3>\x80\xe9\xff<EOR>",
        ParseEncoding::Latin1,
        LengthUnit::Bytes,
    );
    let parsed = parse_ok(record, &mut input);
    assert_eq!(parsed.fields[0].raw_data, b"\x80\xe9\xff");
    assert_eq!(parsed.fields[0].text(), "\u{80}éÿ");
}

#[test]
fn ascii_decoding_does_not_enforce_printable_field_value_rules() {
    let mut input = ascii_stream(b"<COMMENT:4>\0\r\n\x7f<EOR>");
    let parsed = parse_ok(record, &mut input);
    assert_eq!(parsed.fields[0].text().as_bytes(), b"\0\r\n\x7f");
}

#[test]
fn header_decoding_errors_are_terminal_with_relative_causes() {
    for encoding in [ParseEncoding::Ascii, ParseEncoding::Utf8] {
        let mut parser = Parser::new(
            b"Generated \xff\n<EOH><NAME:3>Ray<EOR>",
            ParseOpts {
                encoding,
                field_length: LengthUnit::Bytes,
            },
        );
        let error = next_error(&mut parser);
        assert_eq!(error.span(), 10..11);
        match (encoding, error.kind()) {
            (
                ParseEncoding::Ascii,
                error::StructuralErrorKind::Decoding {
                    cause: DecodeError::NonAscii { offset },
                },
            ) => assert_eq!(*offset, 10),
            (
                ParseEncoding::Utf8,
                error::StructuralErrorKind::Decoding {
                    cause: DecodeError::Utf8(cause),
                },
            ) => assert_eq!(cause.valid_up_to(), 10),
            other => panic!("{other:?}"),
        }
        assert_exhausted(&mut parser);
    }
    let file = parse_file(
        b"Jos\xe9<EOH><NAME:3>Ray<EOR>",
        ParseEncoding::Latin1,
        LengthUnit::Bytes,
    );
    assert_eq!(
        file.header
            .expect("input should yield a header because it starts with a preamble")
            .text(),
        "José"
    );
}

#[test]
fn missing_eoh_at_field_boundaries_reports_the_header_terminator_at_eof() {
    for bytes in [
        &b"Generated"[..],
        &b"Generated<PROGRAMID:3>abc"[..],
        &b"Generated<PROGRAMID:3>abc trailing text\r\n"[..],
        &b"Generated<PROGRAMID:0>"[..],
    ] {
        let error = adi_file.parse(ascii_stream(bytes)).expect_err(&format!(
            "header should fail because this input ends before EOH: {bytes:?}"
        ));
        assert_eq!(error.offset(), bytes.len(), "{bytes:?}");
        assert!(
            error
                .inner()
                .context()
                .any(|context| matches!(context, StrContext::Label("header"))),
            "input {bytes:?}: expected header context, got:\n{error}"
        );
        assert!(
            error.inner().context().any(|context| matches!(
                context,
                StrContext::Expected(StrContextValue::StringLiteral("<EOH>"))
            )),
            "input {bytes:?}: expected missing <EOH>, got:\n{error}"
        );
    }
}

#[test]
fn missing_eor_at_field_boundaries_reports_the_record_terminator_at_eof() {
    for bytes in [
        &b"<NAME:3>Ray"[..],
        &b"<NAME:3>Ray trailing text\r\n"[..],
        &b"<NAME:0>"[..],
    ] {
        let error = adi_file.parse(ascii_stream(bytes)).expect_err(&format!(
            "record should fail because this input ends before EOR: {bytes:?}"
        ));
        assert_eq!(error.offset(), bytes.len(), "{bytes:?}");
        assert!(
            error
                .inner()
                .context()
                .any(|context| matches!(context, StrContext::Label("record"))),
            "input {bytes:?}: expected record context, got:\n{error}"
        );
        assert!(
            error.inner().context().any(|context| matches!(
                context,
                StrContext::Expected(StrContextValue::StringLiteral("<EOR>"))
            )),
            "input {bytes:?}: expected missing <EOR>, got:\n{error}"
        );
    }
}

#[test]
fn byte_shortages_report_typed_counts_and_eof_in_headers_and_records() {
    for encoding in [
        ParseEncoding::Ascii,
        ParseEncoding::Latin1,
        ParseEncoding::Utf8,
    ] {
        for (bytes, field_name, declared, available) in [
            (&b"<NAME:3>Ra"[..], "NAME", 3, 2),
            (&b"<NAME:1>"[..], "NAME", 1, 0),
            (&b"Generated<PROGRAMID:3>ab"[..], "PROGRAMID", 3, 2),
        ] {
            let error = adi_file
                .parse(stream(bytes, encoding, LengthUnit::Bytes))
                .expect_err(
                    "field should fail because its declared byte count exceeds the available bytes",
                );
            assert_truncation(&error, field_name, declared, available, LengthUnit::Bytes);
        }
    }
    // A multibyte value must report bytes rather than decoded scalar count.
    let error = adi_file
        .parse(stream(
            "<NAME:4>Ré".as_bytes(),
            ParseEncoding::Utf8,
            LengthUnit::Bytes,
        ))
        .expect_err("field should fail because four bytes were declared but Ré supplies three");
    assert_truncation(&error, "NAME", 4, 3, LengthUnit::Bytes);
}

#[test]
fn scalar_shortages_report_typed_counts_and_eof_in_headers_and_records() {
    for encoding in [
        ParseEncoding::Ascii,
        ParseEncoding::Latin1,
        ParseEncoding::Utf8,
    ] {
        for (bytes, field_name, declared, available) in [
            (&b"<NAME:4>Jos"[..], "NAME", 4, 3),
            (&b"<NAME:4>"[..], "NAME", 4, 0),
            (&b"Generated<PROGRAMID:4>Jos"[..], "PROGRAMID", 4, 3),
        ] {
            let error = adi_file
                .parse(stream(bytes, encoding, LengthUnit::UnicodeScalars))
                .expect_err("field should fail because its declared scalar count exceeds the available scalars");
            assert_truncation(
                &error,
                field_name,
                declared,
                available,
                LengthUnit::UnicodeScalars,
            );
        }
    }
    for (bytes, encoding) in [
        ("<NAME:4>Ré".as_bytes(), ParseEncoding::Utf8),
        (&b"<NAME:4>R\xe9"[..], ParseEncoding::Latin1),
    ] {
        let error = adi_file
            .parse(stream(bytes, encoding, LengthUnit::UnicodeScalars))
            .expect_err("field should fail because four scalars were declared but Ré supplies two");
        assert_truncation(&error, "NAME", 4, 2, LengthUnit::UnicodeScalars);
    }
}

#[test]
fn eof_guards_preserve_length_and_indicator_errors_in_headers_and_records() {
    for (bytes, label, offset) in [
        (&b"<NAME:"[..], "length", 6),
        (&b"Generated<PROGRAMID:"[..], "length", 20),
        (&b"<NAME:3:Z>Ray"[..], "data type", 8),
        (&b"Generated<PROGRAMID:3:Z>abc"[..], "data type", 22),
    ] {
        let error = adi_file.parse(ascii_stream(bytes)).expect_err(
            "field should fail because its length is malformed or its type indicator is unknown",
        );
        assert_eq!(
            error.offset(),
            offset,
            "expected {label} error at byte {offset}, input {bytes:?}\nactual error:\n{error}"
        );
        assert!(
            error.inner().context().any(|context| matches!(
                context,
                StrContext::Label(actual) if *actual == label
            )),
            "expected a {label} error, input {bytes:?}\nactual error:\n{error}"
        );
        assert_not_missing_terminator(error.inner(), &error.to_string());
        if label == "data type" {
            let cause = error
                .inner()
                .cause()
                .and_then(|cause| cause.downcast_ref::<crate::adif::DataTypeError>());
            assert!(
                matches!(cause,
                    Some(crate::adif::DataTypeError::UnknownTypeSpecifier(indicator)) if indicator == "Z"
                ),
                "expected UnknownTypeSpecifier for Z\nactual cause: {cause:?}\nactual error:\n{error}"
            );
        }
    }
}

#[test]
fn invalid_or_incomplete_utf8_reports_the_sequence_start_not_truncation_or_missing_end() {
    for (bytes, offset) in [
        (&b"<NAME:4>Jos\xff"[..], 11),
        (&b"<NAME:4>Jos\xc3"[..], 11),
        (&b"Generated<PROGRAMID:4>Jos\xff"[..], 25),
        (&b"Generated<PROGRAMID:4>Jos\xc3"[..], 25),
    ] {
        let error = adi_file
            .parse(stream(
                bytes,
                ParseEncoding::Utf8,
                LengthUnit::UnicodeScalars,
            ))
            .expect_err(
                "value should fail because invalid or incomplete UTF-8 prevents counting scalars",
            );
        assert_eq!(
            error.offset(),
            offset,
            "expected UTF-8 framing error at the sequence start (byte {offset}), input {bytes:?}\nactual error:\n{error}"
        );
        assert!(
            error
                .inner()
                .context()
                .any(|context| matches!(context, StrContext::Label("field data"))),
            "expected a field-value framing error\nactual error:\n{error}"
        );
        // Require a UTF-8 framing expectation without coupling the test to
        // its English description.
        assert!(
            error.inner().context().any(|context| matches!(
                context,
                StrContext::Expected(StrContextValue::Description(_))
            )),
            "expected a description of the UTF-8 framing requirement\nactual error:\n{error}"
        );
        let truncation = error
            .inner()
            .cause()
            .and_then(|cause| cause.downcast_ref::<AdiError>());
        assert!(
            !matches!(truncation, Some(AdiError::TruncatedValue { .. })),
            "invalid UTF-8 must not be reported as a known scalar shortage\nactual error:\n{error}"
        );
        assert_not_missing_terminator(error.inner(), &error.to_string());
    }
}

#[test]
fn scalar_boundary_finding_does_not_validate_later_bytes() {
    let mut input = stream(
        b"<NAME:1>A\xff",
        ParseEncoding::Utf8,
        LengthUnit::UnicodeScalars,
    );
    assert_eq!(parse_ok(field, &mut input).text(), "A");
    assert_eq!(*input.input.as_ref(), b"\xff");
}

macro_rules! fixture {
    ($path:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/",
            $path
        ))
        .as_slice()
    };
}

#[test]
fn upstream_matir_headerless_fixture_has_one_record() {
    let file = parse_file(
        fixture!("matir-adifparser/testdata/header_none.adi"),
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    assert!(file.header.is_none());
    assert_eq!(file.records.len(), 1);
    assert_eq!(named(&file.records[0], "MYCALL").text(), "KF4MDV");
}

#[test]
fn upstream_matir_preamble_fixture_preserves_header_text() {
    let file = parse_file(
        fixture!("matir-adifparser/testdata/header_comment.adi"),
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    assert_eq!(
        file.header
            .expect("input should yield a header because it starts with a preamble")
            .text(),
        "This is a comment at the start of the file."
    );
    assert_eq!(file.records.len(), 1);
    assert_eq!(named(&file.records[0], "MYCALL").text(), "KF4MDV");
}

#[test]
fn upstream_difa_fixture_has_three_records_with_distinct_times() {
    let file = parse_file(
        fixture!("difa/examples/sample.adif"),
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    assert_eq!(
        file.header
            .expect("input should yield a header because it starts with a preamble")
            .field("ADIF_VER")
            .expect("header should contain ADIF_VER from the fixture descriptor")
            .text(),
        "3.1.4"
    );
    assert_eq!(file.records.len(), 3);
    for (record, call, time) in [
        (&file.records[0], "W1AW", "1430"),
        (&file.records[1], "AB9BH", "1445"),
        (&file.records[2], "W6RQ", "083000"),
    ] {
        assert_eq!(named(record, "CALL").text(), call);
        assert_eq!(named(record, "TIME_ON").text(), time);
    }
}

#[test]
fn upstream_wsjtx_fixture_preserves_frequency_and_signal_report_text() {
    let file = parse_file(
        fixture!("adif-parser-ts/test/data/wsjtx.adi"),
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    assert_eq!(file.records.len(), 1);
    assert_eq!(named(&file.records[0], "CALL").text(), "KA9FOX");
    assert_eq!(named(&file.records[0], "FREQ").text(), "7.076150");
    assert_eq!(named(&file.records[0], "RST_SENT").text(), "-07");
    assert_eq!(named(&file.records[0], "TIME_ON").text(), "154515");
}

#[test]
fn upstream_js8call_fixture_preserves_explicit_empty_values() {
    let file = parse_file(
        fixture!("adif-parser-ts/test/data/js8call.adi"),
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    assert_eq!(file.records.len(), 1);
    let record = &file.records[0];
    assert_eq!(named(record, "CALL").text(), "VE4DL");
    assert_eq!(named(record, "SUBMODE").text(), "JS8");
    for name in ["GRIDSQUARE", "RST_RCVD"] {
        let field = named(record, name);
        assert_eq!(field.specifier.length, 0);
        assert_eq!(field.raw_data, b"");
        assert_eq!(field.text(), "");
    }
}

#[test]
fn upstream_transadif_ascii_fixture_preserves_crlf_preamble_and_values() {
    let file = parse_file(
        fixture!("transadif/test-cases/01-plain-examples/1-plain-ascii-in.adi"),
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    assert!(
        file.header
            .expect("input should yield a header because it starts with a preamble")
            .text_bytes
            .starts_with(b"Plain ASCII file with no encoding specified.\r\n\r\n")
    );
    assert_eq!(file.records.len(), 1);
    assert_eq!(named(&file.records[0], "NAME").text(), "John ASCII");
    assert_eq!(named(&file.records[0], "TIME_ON").text(), "0100");
}

// Require successful completion and exactly one record without prescribing
// which parser consumes the fixture's separators.
#[test]
fn upstream_latin1_fixture_uses_original_single_byte_accents() {
    let record = fixture_record(
        fixture!("transadif/test-cases/01-plain-examples/2-plain-iso-in.adi"),
        ParseEncoding::Latin1,
        LengthUnit::Bytes,
    );
    let name = named(&record, "NAME");
    assert_eq!(name.raw_data, b"Ju\xe1n Mu\xf1oz ISO");
    assert_eq!(name.text(), "Juán Muñoz ISO");
}

#[test]
fn upstream_utf8_fixture_respects_explicit_byte_or_scalar_lengths() {
    let bytes = fixture!("transadif/test-cases/01-plain-examples/3-plain-utf-in.adi");
    for (unit, expected) in [
        (LengthUnit::Bytes, "Juán Muñoz U"),
        (LengthUnit::UnicodeScalars, "Juán Muñoz UTF"),
    ] {
        let record = fixture_record(bytes, ParseEncoding::Utf8, unit);
        assert_eq!(named(&record, "NAME").raw_data, expected.as_bytes());
        assert_eq!(named(&record, "NAME").text(), expected);
        assert_eq!(named(&record, "QSO_DATE").text(), "20250901");
    }
}

#[test]
fn upstream_undercount_fixture_preserves_framed_cr_without_value_validation() {
    let bytes = fixture!("transadif/test-cases/02-field-length/1-undercount-in.adi");
    for (unit, expected_name) in [
        (LengthUnit::Bytes, "Juán Muñoz UNDER"),
        (LengthUnit::UnicodeScalars, "Juán Muñoz UNDER\r\n"),
    ] {
        let record = fixture_record(bytes, ParseEncoding::Utf8, unit);
        assert_eq!(named(&record, "NAME").raw_data, expected_name.as_bytes());
        for name in ["NOTES", "COMMENT"] {
            assert_eq!(
                named(&record, name).raw_data,
                b"UTF with undercount if read as ISO\r"
            );
        }
    }
}

#[test]
fn upstream_overcount_fixture_has_utf8_bytes_despite_its_preamble_label() {
    let record = fixture_record(
        fixture!("transadif/test-cases/02-field-length/2-overcount-in.adi"),
        ParseEncoding::Utf8,
        LengthUnit::Bytes,
    );
    assert_eq!(
        named(&record, "NAME").raw_data,
        "Juán Muñoz Marín OVER".as_bytes()
    );
    assert_eq!(named(&record, "NAME").text(), "Juán Muñoz Marín OVER");
    assert_eq!(named(&record, "QSO_DATE").text(), "20250902");
}

#[test]
fn upstream_overcount_fixture_scalar_boundary_includes_the_next_open_bracket() {
    let mut input = stream(
        fixture!("transadif/test-cases/02-field-length/2-overcount-in.adi"),
        ParseEncoding::Utf8,
        LengthUnit::UnicodeScalars,
    );
    parse_ok(header, &mut input);
    // These original fixture fields precede NAME. Parse them individually so
    // we can assert the disputed boundary without requiring recovery afterward.
    for name in ["band", "call", "freq", "mode"] {
        let parsed = parse_ok(terminated(field, take_till(0.., b'<')), &mut input);
        assert_eq!(parsed.specifier.name, name);
    }
    let name = parse_ok(field, &mut input);
    assert_eq!(name.raw_data, "Juán Muñoz Marín OVER\r\n<".as_bytes());
    assert!(input.input.as_ref().starts_with(b"qso_date:8>"));
}

#[test]
fn upstream_missing_length_fixture_is_structurally_invalid() {
    let bytes = fixture!("matir-adifparser/testdata/header_version.adi");
    adi_file(&mut ascii_stream(bytes)).expect_err(&format!(
        "fixture should fail because its header is unprefixed and MYCALL has no length: {bytes:?}"
    ));
    // Also reach the missing-length descriptor directly: the whole file is
    // already invalid because it starts with '<' but later supplies EOH.
    let missing_length = bytes
        .strip_prefix(b"<adif_ver:4>1.00<eoh>\n")
        .expect("fixture should have the ADIF version prefix preserved in its upstream bytes");
    assert!(missing_length.starts_with(b"<mycall>"));
    field(&mut ascii_stream(missing_length)).expect_err(&format!(
        "MYCALL should fail because its descriptor omits the length: {missing_length:?}"
    ));
}

#[test]
fn upstream_unterminated_fixture_has_two_complete_records_before_failure() {
    let bytes = fixture!("matir-adifparser/testdata/readrecord.adi");
    adi_file(&mut ascii_stream(bytes)).expect_err(&format!(
        "fixture should fail because its final record lacks EOR: {bytes:?}"
    ));
    let mut input = ascii_stream(bytes);
    assert_eq!(
        named(&parse_ok(record, &mut input), "MYCALL").text(),
        "KF4MDV"
    );
    assert_eq!(
        named(&parse_ok(record, &mut input), "MYCALL").text(),
        "KG4JEL"
    );
    assert_eq!(*input.input.as_ref(), b"<mycall:4>W1AW\n");
    record(&mut input).expect_err(&format!(
        "fixture should fail because its final record lacks EOR: {bytes:?}"
    ));
}

#[test]
fn public_accessors_preserve_syntactic_spans_across_records() {
    let bytes = b"skip<NAME:1>A<EOR>\r\ntext<NAME:0><EOR>\n";
    let mut input = ascii_stream(bytes);
    let first = parse_ok(record, &mut input);
    let second = parse_ok(record, &mut input);
    assert_eq!(first.span(), 4..18);
    assert_eq!(second.span(), 24..37);
    assert_eq!(first.fields().len(), 1);
    let a = first
        .field("name")
        .expect("section should contain NAME from the supplied descriptor");
    assert_eq!(a.name(), "NAME");
    assert_eq!(a.declared_length(), 1);
    assert_eq!(a.data_type(), None);
    assert_eq!(a.descriptor_span(), 4..12);
    assert_eq!(a.value_span(), 12..13);
    assert_eq!(&bytes[a.value_span()], a.raw_data());
    assert_eq!(
        second
            .field("NAME")
            .expect("section should contain NAME from the supplied descriptor")
            .value_span(),
        32..32
    );
}

#[test]
fn header_spans_use_original_bytes_and_exclude_following_separators() {
    for (bytes, encoding, preamble_end, section_end) in [
        (
            &b"Jos\xc3\xa9\r\n<PROGRAMID:0><EOH>\r\n"[..],
            ParseEncoding::Utf8,
            7,
            25,
        ),
        (
            &b"Jos\xe9\r\n<PROGRAMID:0><EOH>\r\n"[..],
            ParseEncoding::Latin1,
            6,
            24,
        ),
        (&b"A\xff\n<EOH>\n"[..], ParseEncoding::Ascii, 3, 8),
    ] {
        if encoding == ParseEncoding::Ascii {
            let mut parser = Parser::new(bytes, ParseOpts::default());
            assert_eq!(next_error(&mut parser).span(), 1..2);
            continue;
        }
        let parsed = parse_ok(header, &mut stream(bytes, encoding, LengthUnit::Bytes));
        assert_eq!(parsed.text_span(), 0..preamble_end);
        assert_eq!(parsed.span(), 0..section_end);
        assert_eq!(&bytes[parsed.text_span()], parsed.raw_text());
        assert_eq!(
            parsed
                .field("programid")
                .expect("header should contain PROGRAMID from the supplied descriptor")
                .text(),
            ""
        );
    }
}

#[test]
fn duplicate_names_are_rejected_with_both_descriptor_locations() {
    for (bytes, first, second) in [
        (&b"<NAME:3>Ray<name:3>Ray<EOR>"[..], 0..8, 11..19),
        (&b"<NAME:3>Ray<NAME:3>Ann<EOR>"[..], 0..8, 11..19),
        (&b"<NAME:3:S>Ray<name:3:S>Ray<EOR>"[..], 0..10, 13..23),
        (&b"<NAME:3:S>Ray<name:3:N>123<EOR>"[..], 0..10, 13..23),
    ] {
        for is_header in [false, true] {
            let mut document = Vec::new();
            if is_header {
                document.extend_from_slice(b"P");
            }
            document.extend_from_slice(bytes.strip_suffix(b"<EOR>").expect(
                "case should end with EOR because each test input supplies that terminator",
            ));
            document.extend_from_slice(if is_header { b"<EOH>" } else { b"<EOR>" });
            let shift = usize::from(is_header);
            let error = adi_file.parse(ascii_stream(&document)).expect_err(&format!(
                "section should fail because NAME repeats ignoring ASCII case: {document:?}"
            ));
            assert_eq!(error.offset(), second.start + shift);
            let cause = error
                .inner()
                .cause()
                .and_then(|c| c.downcast_ref::<AdiError>())
                .unwrap_or_else(|| panic!("expected typed duplicate-field cause, got {error:?}"));
            let AdiError::DuplicateField {
                first_span,
                second_span,
                field_name,
            } = cause
            else {
                panic!("expected duplicate error, got {cause:?}");
            };
            assert!(field_name.eq_ignore_ascii_case("NAME"));
            assert_eq!(*first_span, (first.start + shift)..(first.end + shift));
            assert_eq!(*second_span, (second.start + shift)..(second.end + shift));
        }
    }
}

#[test]
fn duplicate_tracking_is_independent_for_each_section() {
    let file = parse_file(
        b"P<NAME:1>H<EOH><name:1>A<EOR><NAME:1>B<EOR>",
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    assert_eq!(
        file.header
            .expect("input should yield a header because it starts with a preamble")
            .field("name")
            .expect("section should contain NAME from the supplied descriptor")
            .text(),
        "H"
    );
    assert_eq!(file.records.len(), 2);
    assert_eq!(
        file.records[0]
            .field("NAME")
            .expect("section should contain NAME from the supplied descriptor")
            .text(),
        "A"
    );
    assert_eq!(
        file.records[1]
            .field("name")
            .expect("section should contain NAME from the supplied descriptor")
            .text(),
        "B"
    );
}

#[test]
fn structural_errors_have_typed_categories_and_original_locations() {
    use super::error::{StructuralError, StructuralErrorKind, SyntaxErrorKind};
    for (bytes, reason, span) in [
        (
            &b"<NAME:x>Ray<EOR>"[..],
            SyntaxErrorKind::InvalidLength,
            6..7,
        ),
        (
            &b"<NAME:3:Z>Ray<EOR>"[..],
            SyntaxErrorKind::UnknownTypeIndicator,
            8..9,
        ),
        (&b"<NAME:3>Ray"[..], SyntaxErrorKind::MissingEor, 11..11),
        (&b"P<NAME:1>A"[..], SyntaxErrorKind::MissingEoh, 10..10),
    ] {
        let mut input = ascii_stream(bytes);
        let error =
            adi_file(&mut input).expect_err(&format!("input {bytes:?} should fail with {reason:?} because it violates that syntax requirement"));
        let error = StructuralError::from_context(error, input.current_token_start(), bytes.len());
        assert_eq!(error.span(), span);
        assert!(
            matches!(error.kind(), StructuralErrorKind::Syntax { reason: actual } if *actual == reason),
            "input {bytes:?}: expected {reason:?}, got {error:?}"
        );
    }
    let bytes = format!("<NAME:{}0>", usize::MAX);
    let mut input = ascii_stream(bytes.as_bytes());
    let context = field(&mut input).expect_err(&format!(
        "field should fail because its declared length exceeds usize::MAX: {bytes:?}"
    ));
    let error = StructuralError::from_context(context, input.current_token_start(), bytes.len());
    assert!(
        matches!(
            error.kind(),
            StructuralErrorKind::Syntax {
                reason: SyntaxErrorKind::LengthOverflow
            }
        ),
        "unexpected value: {:?}",
        error.kind()
    );
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn scalar_utf8_errors_retain_the_standard_cause_and_sequence_span() {
    use super::error::{StructuralError, StructuralErrorKind};
    for (bytes, span, error_len) in [
        (&b"<NAME:1>\xe2A"[..], 8..9, Some(1)),
        (&b"<NAME:1>\xe2\x82"[..], 8..10, None),
        (&b"<NAME:1>\xff"[..], 8..9, Some(1)),
    ] {
        let mut input = stream(bytes, ParseEncoding::Utf8, LengthUnit::UnicodeScalars);
        let context = field(&mut input).expect_err(&format!(
            "field should fail because its requested scalar is malformed or incomplete: {bytes:?}"
        ));
        let error =
            StructuralError::from_context(context, input.current_token_start(), bytes.len());
        assert_eq!(error.span(), span);
        let StructuralErrorKind::Decoding {
            cause: DecodeError::Utf8(cause),
        } = error.kind()
        else {
            panic!("{error:?}");
        };
        assert_eq!(cause.error_len(), error_len);
        assert_eq!(cause.valid_up_to(), 0);
    }
}
