use super::*;

fn document(name: &str, kind: Option<char>, payload: &[u8]) -> Vec<u8> {
    let kind = kind.map(|k| format!(":{k}")).unwrap_or_default();
    let mut bytes = format!("P<{name}:{}{kind}>", payload.len()).into_bytes();
    bytes.extend_from_slice(payload);
    bytes.extend_from_slice(b"<EOH>");
    bytes
}

#[track_caller]
fn definition<'r, 'a>(
    entry: &'r crate::adif::adi::HeaderEntry<'a>,
) -> &'r crate::adif::adi::UserDefinition<'a> {
    match entry {
        crate::adif::adi::HeaderEntry::UserDefinition(definition) => definition,
        other => panic!("expected USERDEF declaration, got {other:?}"),
    }
}

#[test]
fn declaration_constraints_use_the_agreed_precedence() {
    use crate::adif::adi::UserDefinitionConstraint;
    for (kind, expected_type) in [
        (Some('E'), Some(DataType::Enumeration)),
        (Some('N'), Some(DataType::Number)),
        (None, None),
    ] {
        let bytes = document("USERDEF1", kind, b"X,{5:20}");
        let file = parse_file(&bytes, ParseEncoding::Ascii, LengthUnit::Bytes);
        let header = file
            .header
            .expect("input should yield a header because it starts with a preamble");
        let def = definition(&header.entries()[0]);
        assert_eq!(def.data_type(), expected_type);
        match (
            kind,
            def.constraint()
                .unwrap_or_else(|| panic!("expected constraint, got {def:?}")),
        ) {
            (Some('E'), UserDefinitionConstraint::Enumeration(items)) => {
                assert_eq!(items, &["5:20"])
            }
            (_, UserDefinitionConstraint::Range { lower, upper }) => {
                assert_eq!(lower, &bigdecimal::BigDecimal::from(5));
                assert_eq!(upper, &bigdecimal::BigDecimal::from(20));
            }
            other => panic!("{other:?}"),
        }
    }
    let bytes = document("USERDEF1", Some('E'), b"X,{A:B,C}");
    let h = parse_file(&bytes, ParseEncoding::Ascii, LengthUnit::Bytes)
        .header
        .expect("input should yield a header because it starts with a preamble");
    let def = definition(&h.entries()[0]);
    assert_eq!(
        def.constraint(),
        Some(&UserDefinitionConstraint::Enumeration(
            ["A:B", "C"].map(str::to_owned).to_vec()
        ))
    );
}

#[test]
fn declaration_failures_have_original_component_spans() {
    use crate::adif::adi::UserDefinitionError;
    for (bytes, cause, span) in [
        (
            b"P<USERDEF1:8:N>X,{20:5}<EOH>".to_vec(),
            UserDefinitionError::InvalidRangeOrder,
            21..22,
        ),
        (
            b"P<USERDEF1:7>X,{5:5}<EOH>".to_vec(),
            UserDefinitionError::InvalidRangeOrder,
            18..19,
        ),
        (
            document("USERDEF0", None, b"X"),
            UserDefinitionError::InvalidDeclarationNumber,
            9..10,
        ),
        (
            document("USERDEF1", None, b"X,{abc:20}"),
            UserDefinitionError::InvalidRangeBound,
            17..18,
        ),
        (
            document("USERDEF1", None, b"X,no"),
            UserDefinitionError::MalformedDeclaration,
            15..16,
        ),
        (
            document("USERDEF1", None, b"X,{1:2"),
            UserDefinitionError::MalformedDeclaration,
            19..19,
        ),
    ] {
        let mut parser = Parser::new(&bytes, ParseOpts::default());
        let error = next_error(&mut parser);
        assert_eq!(error.span(), span, "{bytes:?}: {error}");
        assert!(
            matches!(error.kind(),error::StructuralErrorKind::InvalidUserDefinition {cause:actual,..} if actual==&cause),
            "input {bytes:?}: expected {cause:?}, got {error:?}"
        );
        assert_exhausted(&mut parser);
    }
}

#[test]
fn declaration_numbers_are_positive_and_not_machine_width_limited() {
    use crate::adif::adi::UserDefinitionConstraint;
    let bytes = document(
        "uSeRdEf123456789012345678901234567890",
        Some('n'),
        b"X,{-.5:1.}",
    );
    let header = parse_file(&bytes, ParseEncoding::Ascii, LengthUnit::Bytes)
        .header
        .expect("input should yield a header because it starts with a preamble");
    let def = definition(&header.entries()[0]);
    assert_eq!(def.number().to_string(), "123456789012345678901234567890");
    assert!(
        matches!(def.constraint(),Some(UserDefinitionConstraint::Range{lower,upper}) if lower.to_string()=="-0.5" && upper==&bigdecimal::BigDecimal::from(1)),
        "unexpected value: {:?}",
        def.constraint()
    );
}

#[test]
fn declaration_members_follow_selected_string_rules() {
    use crate::adif::adi::UserDefinitionConstraint;
    for (encoding, payload, expected) in [
        (
            ParseEncoding::Latin1,
            &b"X,{Jos\xe9,\xa0}"[..],
            vec!["José", "\u{a0}"],
        ),
        (
            ParseEncoding::Utf8,
            &b"X,{\0,\t,\x7f,\xc2\x85}"[..],
            vec!["\0", "\t", "\x7f", "\u{85}"],
        ),
    ] {
        let bytes = document("USERDEF1", Some('E'), payload);
        let header = parse_file(&bytes, encoding, LengthUnit::Bytes)
            .header
            .expect("input should yield a header because it starts with a preamble");
        let def = definition(&header.entries()[0]);
        assert_eq!(
            def.constraint(),
            Some(&UserDefinitionConstraint::Enumeration(
                expected.iter().map(|s| (*s).to_owned()).collect()
            )),
            "encoding {encoding:?}, payload {payload:?}"
        );
    }
    let bytes = b"P<PROGRAMID:1>A<USERDEF1:8:E>X\xe9,{A,\x85}<EOH>";
    let mut parser = Parser::new(
        bytes,
        ParseOpts {
            encoding: ParseEncoding::Latin1,
            field_length: LengthUnit::Bytes,
        },
    );
    assert_eq!(next_error(&mut parser).span(), 35..36);
}

#[test]
fn adx_only_indicators_are_terminal_even_on_empty_fields() {
    for indicator in ['I', 'i', 'G', 'g'] {
        for (name, payload, header) in [
            ("X", &b""[..], false),
            ("X", &b"A"[..], false),
            ("USERDEF1", &b""[..], true),
            ("USERDEF1", &b"X"[..], true),
        ] {
            let mut bytes = if header { b"P".to_vec() } else { vec![] };
            bytes.extend_from_slice(format!("<{name}:{}:{indicator}>", payload.len()).as_bytes());
            let position = bytes.len() - 2;
            bytes.extend_from_slice(payload);
            bytes.extend_from_slice(if header { b"<EOH>" } else { b"<EOR>" });
            let mut parser = Parser::new(&bytes, ParseOpts::default());
            let error = next_error(&mut parser);
            assert_eq!(
                error.span(),
                position..position + 1,
                "indicator {indicator}, name {name}, payload {payload:?}: {error:?}"
            );
            assert!(
                matches!(
                    error.kind(),
                    error::StructuralErrorKind::TypeUnavailableInAdi { .. }
                ),
                "indicator {indicator}, name {name}, payload {payload:?}: expected ADI-ineligible type error, got {error:?}"
            );
            assert_exhausted(&mut parser);
        }
    }
}

#[test]
fn declaration_uniqueness_uses_original_descriptor_names() {
    let bytes = b"P<USERDEF1:1:N>X<USERDEF01:1:N>Y<EOH>";
    let header = parse_file(bytes, ParseEncoding::Ascii, LengthUnit::Bytes)
        .header
        .expect("input should yield a header because it starts with a preamble");
    assert_eq!(header.entries().len(), 2);
    let bytes = b"P<USERDEF1:1:N>X<userdef1:1:N>X<EOH>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let error = next_error(&mut parser);
    let error::StructuralErrorKind::DuplicateField {
        first_span,
        second_span,
        ..
    } = error.kind()
    else {
        panic!("{error:?}")
    };
    assert_eq!(&bytes[first_span.clone()], b"<USERDEF1:1:N>");
    assert_eq!(&bytes[second_span.clone()], b"<userdef1:1:N>");
}

#[test]
fn userdef_enumeration_members_are_nonempty() {
    for payload in [b"X,{}".as_slice(), b"X,{,A}", b"X,{A,}", b"X,{A,,B}"] {
        let bytes = document("USERDEF1", Some('E'), payload);
        let mut parser = Parser::new(&bytes, ParseOpts::default());
        let error = next_error(&mut parser);
        assert!(
            matches!(
                error.kind(),
                error::StructuralErrorKind::InvalidUserDefinition {
                    cause: crate::adif::adi::UserDefinitionError::InvalidEnumeration,
                    ..
                }
            ),
            "{payload:?}: {error:?}"
        );
        assert!(
            error.span().is_empty(),
            "missing member should point between delimiters: {error:?}"
        );
        assert_exhausted(&mut parser);
    }
    let bytes = document("USERDEF1", Some('E'), b"X,{ }");
    parse_file(&bytes, ParseEncoding::Ascii, LengthUnit::Bytes);
}

#[test]
fn userdef_collisions_fail_when_the_declaration_is_added() {
    let bytes = b"P<USERDEF1:5:N>SCORE<USERDEF2:5:N>score<BROKEN";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let error = next_error(&mut parser);
    let error::StructuralErrorKind::DuplicateUserDefinition {
        first_span,
        second_span,
        ..
    } = error.kind()
    else {
        panic!("expected duplicate target before BROKEN, got {error:?}");
    };
    assert_eq!(&bytes[first_span.clone()], b"SCORE");
    assert_eq!(&bytes[second_span.clone()], b"score");
    assert_eq!(error.span(), *second_span);
    assert_exhausted(&mut parser);

    let bytes = b"P<USERDEF1:4:N>FREQ<EOH>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let error = next_error(&mut parser);
    assert!(
        matches!(
            error.kind(),
            error::StructuralErrorKind::InvalidUserDefinition {
                cause: crate::adif::adi::UserDefinitionError::StandardFieldName,
                ..
            }
        ),
        "{error:?}"
    );
    assert_eq!(&bytes[error.span()], b"FREQ");
}

#[test]
fn userdef_types_and_ranges_survive_dropping_the_header() {
    let bytes = b"P<USERDEF1:5:N>SCORE<USERDEF2:11>SIZE,{5:20}<EOH><SCORE:3>123<SIZE:1>5<EOR><SIZE:2>20<EOR><SIZE:2>21<EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    drop(next_section(&mut parser));
    let section = next_section(&mut parser);
    let Section::Record(record) = section else {
        panic!("expected first record, got {section:?}");
    };
    assert_eq!(named(&record, "SCORE").data_type(), Some(DataType::Number));
    assert!(
        matches!(named(&record, "SCORE").value(), Some(crate::adif::Value::Number(n)) if n.to_string()=="123")
    );
    assert_eq!(named(&record, "SIZE").data_type(), None);
    assert!(matches!(named(&record, "SIZE").value(), Some(crate::adif::Value::Text(t)) if t=="5"));
    let _ = next_section(&mut parser);
    let error = next_error(&mut parser);
    let error::StructuralErrorKind::UserDefinitionConstraintViolation {
        constraint_span,
        cause: crate::adif::adi::UserDefinitionValueError::OutOfRange { lower, upper },
        ..
    } = error.kind()
    else {
        panic!("expected range violation, got {error:?}");
    };
    assert_eq!(&bytes[error.span()], b"21");
    assert_eq!(&bytes[constraint_span.clone()], b"{5:20}");
    assert_eq!(lower, &bigdecimal::BigDecimal::from(5));
    assert_eq!(upper, &bigdecimal::BigDecimal::from(20));
    assert_exhausted(&mut parser);
}

#[test]
fn userdef_empty_fields_skip_constraints_but_not_descriptors() {
    let bytes = b"P<USERDEF1:12:N>SCORE,{5:20}<EOH><SCORE:0><EOR><SCORE:0:S><EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let _ = next_section(&mut parser);
    let section = next_section(&mut parser);
    let Section::Record(record) = section else {
        panic!("expected empty-value record, got {section:?}");
    };
    assert!(named(&record, "SCORE").value().is_none());
    let error = next_error(&mut parser);
    assert!(
        matches!(
            error.kind(),
            error::StructuralErrorKind::TypeConflict {
                definition_span: Some(_),
                supplied: DataType::String,
                ..
            }
        ),
        "{error:?}"
    );
}

#[test]
fn userdef_unicode_membership_preserves_source_locations() {
    for (encoding, unit) in [
        (ParseEncoding::Utf8, LengthUnit::Bytes),
        (ParseEncoding::Utf8, LengthUnit::UnicodeScalars),
        (ParseEncoding::Latin1, LengthUnit::Bytes),
        (ParseEncoding::Latin1, LengthUnit::UnicodeScalars),
    ] {
        for (member, value, accepted) in [
            ("Café", "CAFÉ", true),
            ("Straße", "STRASSE", true),
            ("Café", "Cafe\u{301}", false),
            ("S", " S", false),
            ("S", "S ", false),
        ] {
            if encoding == ParseEncoding::Latin1 && !value.chars().all(|c| u32::from(c) <= 255) {
                continue;
            }
            let encode = |s: &str| -> Vec<u8> {
                if encoding == ParseEncoding::Latin1 {
                    s.chars().map(|c| c as u8).collect()
                } else {
                    s.as_bytes().to_vec()
                }
            };
            let length = |s: &str| {
                if unit == LengthUnit::UnicodeScalars {
                    s.chars().count()
                } else {
                    encode(s).len()
                }
            };
            let payload = format!("X,{{{member}}}");
            let mut bytes = format!("P<USERDEF1:{}:E>", length(&payload)).into_bytes();
            bytes.extend(encode(&payload));
            bytes.extend(b"<EOH>");
            bytes.extend(format!("<X:{}>", length(value)).as_bytes());
            let start = bytes.len();
            bytes.extend(encode(value));
            let end = bytes.len();
            bytes.extend(b"<EOR>");
            let mut parser = Parser::new(
                &bytes,
                ParseOpts {
                    encoding,
                    field_length: unit,
                },
            );
            let _ = next_section(&mut parser);
            if accepted {
                let section = next_section(&mut parser);
                let Section::Record(record) = section else {
                    panic!("expected record, got {section:?}");
                };
                assert_eq!(named(&record, "X").text(), value);
            } else {
                let error = next_error(&mut parser);
                assert_eq!(error.span(), start..end, "{member:?} {value:?}: {error:?}");
                let error::StructuralErrorKind::UserDefinitionConstraintViolation {
                    constraint_span,
                    ..
                } = error.kind()
                else {
                    panic!("expected constraint failure, got {error:?}");
                };
                assert_eq!(
                    &bytes[constraint_span.clone()],
                    encode(&format!("{{{member}}}"))
                );
            }
        }
    }
}

#[test]
fn userdef_precedence_keeps_untyped_application_shaped_fields_untyped() {
    let bytes=b"P<USERDEF1:24>APP_EXAMPLE_SCORE,{5:20}<EOH><APP_EXAMPLE_SCORE:1>5<EOR><APP_EXAMPLE_SCORE:1:N>6<EOR><APP_EXAMPLE_SCORE:1>7<EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let _ = next_section(&mut parser);
    for kind in [None, Some(DataType::Number), None] {
        let section = next_section(&mut parser);
        let Section::Record(record) = section else {
            panic!("expected record, got {section:?}");
        };
        assert_eq!(named(&record, "APP_EXAMPLE_SCORE").data_type(), kind);
    }
}

#[test]
fn userdef_constraints_compare_original_spelling_and_check_nonempty_whitespace() {
    for (bytes, expected) in [
        (b"P<USERDEF1:7:N>X,{1,2}<EOH><X:2>01<EOR>".as_slice(), "01"),
        (b"P<USERDEF1:8>X,{5:20}<EOH><X:1> <EOR>".as_slice(), " "),
        (
            b"P<USERDEF1:5:E>X,{A}<EOH><X:0><EOR><X:1> <EOR>".as_slice(),
            " ",
        ),
    ] {
        let mut parser = Parser::new(bytes, ParseOpts::default());
        let _ = next_section(&mut parser);
        if bytes.windows(5).any(|w| w == b"<X:0>") {
            let section = next_section(&mut parser);
            let Section::Record(record) = section else {
                panic!("expected empty field record, got {section:?}");
            };
            assert!(named(&record, "X").value().is_none());
        }
        let error = next_error(&mut parser);
        assert!(
            matches!(
                error.kind(),
                error::StructuralErrorKind::UserDefinitionConstraintViolation { .. }
            ),
            "{error:?}"
        );
        assert_eq!(&bytes[error.span()], expected.as_bytes());
        assert_exhausted(&mut parser);
    }
}
