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
fn subdivision_membership_preserves_groups_without_requiring_or_inferring_dxcc() {
    use qso_io::adif::primary_subdivisions::{canada, united_states, venezuela};

    assert_eq!(
        "CA".parse::<united_states::PrimarySubdivision>()
            .expect("CA should identify California within the US group"),
        united_states::PrimarySubdivision::California
    );
    assert_eq!(
        "ca".parse::<venezuela::PrimarySubdivision>()
            .expect("CA should identify Carabobo within the Venezuela group"),
        venezuela::PrimarySubdivision::Carabobo
    );
    assert!(
        "CA".parse::<canada::PrimarySubdivision>().is_err(),
        "a shared code must not become a member of every country's enum"
    );

    for bytes in [
        &b"<STATE:2>ca<EOR>"[..],
        &b"<MY_STATE:2>ca<EOR>"[..],
        &b"<DXCC:1>1<STATE:2>ca<EOR>"[..],
        &b"<MY_DXCC:1>1<MY_STATE:2>ca<EOR>"[..],
    ] {
        let mut parser = Parser::new(bytes, ParseOpts::default());
        let parsed = record(next_section(&mut parser));
        let subdivision = parsed
            .field("STATE")
            .or_else(|| parsed.field("MY_STATE"))
            .unwrap_or_else(|| panic!("expected a subdivision field in {parsed:?}"));
        assert_eq!(subdivision.text(), "ca", "input: {bytes:?}");
        assert!(
            matches!(subdivision.value(), Some(qso_io::adif::Value::Text(text)) if text == "ca"),
            "membership must retain text without selecting a country's variant: {subdivision:?}"
        );
        assert_eq!(
            parsed.field("DXCC").is_some() || parsed.field("MY_DXCC").is_some(),
            bytes.starts_with(b"<DXCC") || bytes.starts_with(b"<MY_DXCC"),
            "parsing must not infer a DXCC field: {parsed:?}"
        );
        assert_exhausted(&mut parser);
    }

    let mut parser = Parser::new(b"<STATE:3>???<EOR>", ParseOpts::default());
    let error = next_error(&mut parser);
    assert!(
        matches!(
            error.kind(),
            StructuralErrorKind::InvalidValue {
                enumeration: Some("Primary_Administrative_Subdivision"),
                ..
            }
        ),
        "unknown codes must fail membership: {error:?}"
    );
    assert_eq!(error.span(), 9..12, "{error:?}");
    assert_exhausted(&mut parser);
}

#[test]
fn reference_types_and_parser_share_standard_membership() {
    use qso_io::adif::{AntPath, ArrlSection, Continent, QslMedium};

    assert_eq!(
        "g".parse::<AntPath>().expect("G should name grayline"),
        AntPath::G
    );
    assert_eq!(
        "eu".parse::<Continent>().expect("EU should name Europe"),
        Continent::Europe
    );
    assert_eq!(
        "lotw"
            .parse::<QslMedium>()
            .expect("LOTW should name Logbook of the World"),
        QslMedium::Lotw
    );
    let historical = "gta"
        .parse::<ArrlSection>()
        .expect("GTA should retain its historical identity");
    assert_eq!(historical.name(), "Greater Toronto Area");
    assert_eq!(
        historical
            .deleted_date()
            .map(|date| date.to_string())
            .as_deref(),
        Some("2023-01-01")
    );

    let bytes = b"<ANT_PATH:1>g<ARRL_SECT:3>gta<CONT:2>eu<CREDIT_GRANTED:8>WAS:lotw<EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let section = record(next_section(&mut parser));
    assert_eq!(
        named(&section, "ANT_PATH")
            .text()
            .parse::<AntPath>()
            .expect("accepted ANT_PATH should have a reference value"),
        AntPath::G
    );
    assert_eq!(
        named(&section, "ARRL_SECT")
            .text()
            .parse::<ArrlSection>()
            .expect("accepted section should have a reference value"),
        historical
    );
    assert_exhausted(&mut parser);

    assert!(
        " EU".parse::<Continent>().is_err(),
        "reference parsing must not trim values"
    );
    let mut parser = Parser::new(b"<CONT:3> EU<EOR>", ParseOpts::default());
    let error = next_error(&mut parser);
    assert!(
        matches!(
            error.kind(),
            StructuralErrorKind::InvalidValue {
                enumeration: Some("Continent"),
                ..
            }
        ),
        "{error:?}"
    );
}

#[test]
fn alternative_enumeration_labels_outlive_the_error() {
    let mut parser = Parser::new(b"<CREDIT_GRANTED:8>AJA,NOPE<EOR>", ParseOpts::default());
    let labels: Vec<Option<&'static str>> = {
        let error = next_error(&mut parser);
        let StructuralErrorKind::NoMatchingType { failures, .. } = error.kind() else {
            panic!("expected alternative failures, got {error:?}");
        };
        failures
            .iter()
            .map(|failure| failure.enumeration())
            .collect()
    };

    assert_eq!(labels, [Some("Credit"), Some("Award")]);
    assert_exhausted(&mut parser);
}

#[test]
fn identifier_membership_errors_retain_the_expected_enumeration() {
    let bytes = b"<IOTA:6>ZZ-001<EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());

    let error = next_error(&mut parser);
    assert!(
        matches!(
            error.kind(),
            StructuralErrorKind::InvalidValue {
                field_name,
                cause: qso_io::adif::ValueError::InvalidIOTARefNo,
                enumeration: Some("Continent"),
            } if field_name == "IOTA"
        ),
        "IOTA continent membership should identify its expected enumeration: {error:?}"
    );
    assert_eq!(error.span(), 8..10, "{error:?}");
    assert_eq!(&bytes[error.span()], b"ZZ");
    assert_exhausted(&mut parser);
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

#[test]
fn values_preserve_absence_empty_contents_and_source_spelling() {
    use qso_io::adif::{DataType, Value};
    let bytes = b"<A:0><B:0:N><C:3>123<D:3:S>123<E:8:N>001.2300<EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let rec = record(next_section(&mut parser));
    assert!(
        rec.field("X").is_none(),
        "expected X to be absent, got {:?}",
        rec.field("X")
    );
    for name in ["A", "B"] {
        let f = named(&rec, name);
        assert_eq!(f.value(), None, "expected empty field {name:?}, got {f:?}");
        assert_eq!(f.text(), "");
    }
    assert_eq!(named(&rec, "A").data_type(), None);
    assert_eq!(named(&rec, "B").data_type(), Some(DataType::Number));
    for name in ["C", "D"] {
        assert!(
            matches!(named(&rec, name).value(),Some(Value::Text(s)) if s=="123"),
            "unexpected value: {:?}",
            named(&rec, name).value()
        );
    }
    let number = named(&rec, "E");
    assert!(
        matches!(number.value(),Some(Value::Number(n)) if n == &"1.2300".parse::<bigdecimal::BigDecimal>().expect("expected decimal should parse because its fixed test spelling uses decimal notation")),
        "unexpected value: {:?}",
        number.value()
    );
    assert_eq!(number.text(), "001.2300");
    assert_eq!(number.raw_data(), b"001.2300");
    for f in rec.fields() {
        assert_eq!(&bytes[f.value_span()], f.raw_data());
    }
}

#[test]
fn composite_values_and_definitions_outlive_the_parser() {
    use qso_io::adif::{Hemisphere, TimePrecision, Value};
    let bytes=b"P<USERDEF1:3:N>EPC<EOH><A:4:S>Jos\xe9<B:1:B>y<C:2:N>1.<D:8:D>20240229<E:4:T>1305<F:11:L>W122 30.125<EOR><E:6:T>130500<EPC:3>123<EOR>";
    let sections = {
        let parser = Parser::new(
            bytes,
            ParseOpts {
                encoding: ParseEncoding::Latin1,
                field_length: LengthUnit::Bytes,
            },
        );
        parser
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|error| {
                panic!("input {bytes:?}: expected complete sections, got {error:?}")
            })
    };
    assert_eq!(sections.len(), 3);
    let Section::Header(header) = &sections[0] else {
        panic!("expected Section::Header(header), got {:?}", sections[0]);
    };
    let def = definition(&header.entries()[0]);
    assert_eq!(def.field_name(), "EPC");
    assert_eq!(&bytes[def.value_span()], def.raw_data());
    let Section::Record(first) = &sections[1] else {
        panic!("expected Section::Record(first), got {:?}", sections[1]);
    };
    assert_eq!(first.fields().len(), 6);
    for (field, expected) in first
        .fields()
        .iter()
        .zip(["text", "boolean", "number", "date", "time", "location"])
    {
        let kind = match field
            .value()
            .unwrap_or_else(|| panic!("expected a nonempty value, got {field:?}"))
        {
            Value::Text(s) => {
                assert_eq!(s, "José");
                "text"
            }
            Value::Boolean(b) => {
                assert!(*b);
                "boolean"
            }
            Value::Integer(_) => "integer",
            Value::List(_) => "list",
            Value::Number(n) => {
                assert_eq!(n, &bigdecimal::BigDecimal::from(1));
                "number"
            }
            Value::Date(d) => {
                assert_eq!(d.to_string(), "2024-02-29");
                "date"
            }
            Value::Time(t) => {
                assert_eq!(t.precision(), TimePrecision::Minute);
                assert_eq!(t.time().to_string(), "13:05:00");
                "time"
            }
            Value::Location(l) => {
                assert_eq!(l.hemisphere(), Hemisphere::West);
                assert_eq!(l.degrees(), 122);
                assert_eq!(
                    l.minutes(),
                    &"30.125"
                        .parse::<bigdecimal::BigDecimal>()
                        .expect("expected decimal should parse because its fixed test spelling uses decimal notation")
                );
                "location"
            }
        };
        assert_eq!(kind, expected);
        assert_eq!(&bytes[field.value_span()], field.raw_data());
    }
    assert_eq!(named(first, "C").text(), "1.");
    let Section::Record(second) = &sections[2] else {
        panic!("expected Section::Record(second), got {:?}", sections[2]);
    };
    let Some(Value::Time(t)) = named(second, "E").value() else {
        panic!(
            "expected Some(Value::Time(t)), got {:?}",
            named(second, "E").value()
        );
    };
    assert_eq!(t.precision(), TimePrecision::Second);
    assert_eq!(t.time().to_string(), "13:05:00");
    assert!(
        matches!(named(second, "EPC").value(),Some(Value::Text(s)) if s=="123"),
        "unexpected value: {:?}",
        named(second, "EPC").value()
    );
}

#[test]
fn latin1_declaration_error_uses_original_bytes_after_an_accent() {
    use qso_io::adif::adi::UserDefinitionError;
    let bytes = b"P<PROGRAMID:1>A<USERDEF1:8:E>X\xe9,{A,\x85}<EOH>";
    let mut parser = Parser::new(
        bytes,
        ParseOpts {
            encoding: ParseEncoding::Latin1,
            field_length: LengthUnit::Bytes,
        },
    );
    let error = next_error(&mut parser);
    assert_eq!(error.span(), 35..36);
    assert!(
        matches!(
            error.kind(),
            StructuralErrorKind::InvalidUserDefinition {
                cause: UserDefinitionError::InvalidEnumeration,
                ..
            }
        ),
        "unexpected value: {:?}",
        error.kind()
    );
    assert_exhausted(&mut parser);
}

#[test]
fn inline_indicator_is_distinct_from_value_presence() {
    use qso_io::adif::{DataType, Value};

    let mut parser = Parser::new(b"<X:0:N><EOR><X:3>123<EOR>", ParseOpts::default());
    let empty = record(next_section(&mut parser));
    let field = named(&empty, "X");
    assert_eq!(field.data_type(), Some(DataType::Number));
    assert_eq!(field.type_indicator(), Some(DataType::Number));
    assert!(field.value().is_none(), "empty field: {field:?}");
    assert_eq!(field.text(), "");
    assert_eq!(field.raw_data(), b"");

    let untyped = record(next_section(&mut parser));
    let field = named(&untyped, "X");
    assert_eq!(field.data_type(), None);
    assert_eq!(field.type_indicator(), None);
    assert!(
        matches!(field.value(), Some(Value::Text(text)) if text == "123"),
        "field: {field:?}"
    );
}
