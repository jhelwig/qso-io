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
    let bytes = document("USERDEF1", Some('E'), b"X,{A:B,C,,}");
    let h = parse_file(&bytes, ParseEncoding::Ascii, LengthUnit::Bytes)
        .header
        .expect("input should yield a header because it starts with a preamble");
    let def = definition(&h.entries()[0]);
    assert_eq!(
        def.constraint(),
        Some(&UserDefinitionConstraint::Enumeration(
            ["A:B", "C", "", ""].map(str::to_owned).to_vec()
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
    let bytes = b"P<USERDEF1:1:N>X<USERDEF01:1:N>X<EOH>";
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
