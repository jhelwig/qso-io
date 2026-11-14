use super::*;
use crate::adif::Value;

#[test]
fn standard_fields_resolve_without_inline_indicators() {
    let file = parse_file(
        b"P<PROGRAMID:1>X<EOH><FREQ:6>14.074<MY_CQ_ZONE:2>41<EOR>",
        ParseEncoding::Ascii,
        LengthUnit::Bytes,
    );
    let header = file.header.expect("preamble should produce a header");
    assert_eq!(
        header
            .field("PROGRAMID")
            .expect("header should contain PROGRAMID")
            .data_type(),
        Some(DataType::String)
    );
    let record = &file.records[0];
    let frequency = named(record, "freq");
    assert_eq!(frequency.data_type(), Some(DataType::Number));
    assert_eq!(frequency.type_indicator(), None);
    assert!(
        matches!(frequency.value(), Some(Value::Number(n)) if n.to_string() == "14.074"),
        "{frequency:?}"
    );
    let zone = named(record, "MY_CQ_ZONE");
    assert_eq!(zone.data_type(), Some(DataType::PositiveInteger));
    assert!(
        matches!(zone.value(), Some(Value::Integer(n)) if n.to_string() == "41"),
        "{zone:?}"
    );
}

#[test]
fn standard_indicators_and_adi_eligibility_apply_to_empty_values() {
    for (bytes, kind) in [
        (&b"<MY_CQ_ZONE:0:N><EOR>"[..], DataType::PositiveInteger),
        (&b"<NAME_INTL:0><EOR>"[..], DataType::IntlString),
    ] {
        let mut parser = Parser::new(bytes, ParseOpts::default());
        let error = next_error(&mut parser);
        if kind == DataType::PositiveInteger {
            assert!(
                matches!(error.kind(), error::StructuralErrorKind::TypeConflict { expected, supplied: DataType::Number, definition_span: None, .. } if expected == &[kind]),
                "{error:?}"
            );
        } else {
            assert!(
                matches!(error.kind(), error::StructuralErrorKind::TypeUnavailableInAdi {data_type} if *data_type==kind),
                "{error:?}"
            );
        }
        assert_eq!(
            &bytes[error.span()],
            bytes
                .split_inclusive(|b| *b == b'>')
                .next()
                .expect("descriptor exists")
        );
        assert_exhausted(&mut parser);
    }
    let mut parser = Parser::new(b"<MY_CQ_ZONE:4>14.5<EOR>", ParseOpts::default());
    let error = next_error(&mut parser);
    assert_eq!(error.span(), 16..17);
}

#[test]
fn standard_membership_uses_ascii_identity_without_date_relationships() {
    let bytes = b"<MODE:3>ssb<ARRL_SECT:3>GTA<QSO_DATE:8>20250101<DARC_DOK:11>NOT-A-DOK-X<EOR>";
    let file = parse_file(bytes, ParseEncoding::Ascii, LengthUnit::Bytes);
    assert_eq!(named(&file.records[0], "MODE").text(), "ssb");
    assert_eq!(
        named(&file.records[0], "DARC_DOK").data_type(),
        Some(DataType::Enumeration)
    );
    for raw in ["ſſb", " SSB", "SSB "] {
        let bytes = format!("<MODE:{}>{raw}<EOR>", raw.len());
        let mut parser = Parser::new(
            bytes.as_bytes(),
            ParseOpts {
                encoding: ParseEncoding::Utf8,
                field_length: LengthUnit::Bytes,
            },
        );
        let error = next_error(&mut parser);
        assert!(
            matches!(
                error.kind(),
                error::StructuralErrorKind::InvalidValue {
                    enumeration: Some("Mode"),
                    ..
                }
            ),
            "{error:?}"
        );
        assert_eq!(&bytes.as_bytes()[error.span()], raw.as_bytes());
    }
}

#[test]
fn standard_alternatives_prefer_credit_and_retain_both_failures() {
    for (value, kind) in [
        ("DXCC,IOTA", DataType::CreditList),
        ("AJA", DataType::AwardList),
    ] {
        let bytes = format!("<CREDIT_GRANTED:{}>{value}<EOR>", value.len());
        let file = parse_file(bytes.as_bytes(), ParseEncoding::Ascii, LengthUnit::Bytes);
        assert_eq!(
            named(&file.records[0], "CREDIT_GRANTED").data_type(),
            Some(kind)
        );
    }
    let bytes = b"<CREDIT_GRANTED:8>AJA,NOPE<EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    let error = next_error(&mut parser);
    let error::StructuralErrorKind::NoMatchingType { failures, .. } = error.kind() else {
        panic!("expected alternatives, got {error:?}");
    };
    assert_eq!(failures.len(), 2);
    assert_eq!(failures[0].data_type(), DataType::CreditList);
    assert_eq!(failures[1].data_type(), DataType::AwardList);
    assert_eq!(&bytes[failures[0].span()], b"AJA");
    assert_eq!(&bytes[failures[1].span()], b"NOPE");
    assert_eq!(error.span(), failures[0].span());
    assert_exhausted(&mut parser);
}

#[test]
fn standard_catalog_is_not_selected_by_adif_ver() {
    for version in ["2.0.0", "9.9.9"] {
        let bytes = format!("P<ADIF_VER:5>{version}<EOH><FREQ:1>1<X_NEW:3>123<EOR>");
        let file = parse_file(bytes.as_bytes(), ParseEncoding::Ascii, LengthUnit::Bytes);
        assert_eq!(
            named(&file.records[0], "FREQ").data_type(),
            Some(DataType::Number)
        );
        assert_eq!(named(&file.records[0], "X_NEW").data_type(), None);
    }
}

#[test]
fn secondary_subdivision_failures_preserve_field_diagnostics() {
    for code in [
        "ak,Imaginary",
        "ZZ,County",
        "AK,",
        "HI,",
        "nonsense",
        "12",
        "1234567",
    ] {
        let bytes = format!("<CNTY:{}>{code}<EOR><CALL:1>X<EOR>", code.len());
        let mut parser = Parser::new(bytes.as_bytes(), ParseOpts::default());
        let error = next_error(&mut parser);
        assert!(
            matches!(error.kind(), error::StructuralErrorKind::InvalidValue {
                field_name, cause: crate::adif::ValueError::InvalidEnumeration,
                enumeration: Some("Secondary_Administrative_Subdivision"), ..
            } if field_name == "CNTY"),
            "{code:?}: {error:?}"
        );
        assert_eq!(
            &bytes.as_bytes()[error.span()],
            code.as_bytes(),
            "{code:?}: {error}"
        );
        assert_exhausted(&mut parser);
    }
}

#[test]
fn secondary_subdivisions_preserve_text_without_dxcc_inference() {
    for code in [
        "ak,Aleutians East",
        "hi,Hawaii",
        "ma,Franklin",
        "AB-01",
        "0101",
        "01001",
        "010101",
        "001",
        "33024",
    ] {
        for prefix in ["", "<DXCC:1>1<MY_DXCC:1>1"] {
            let bytes = format!(
                "{prefix}<CNTY:{}>{code}<MY_CNTY:{}>{code}<EOR>",
                code.len(),
                code.len()
            );
            let file = parse_file(bytes.as_bytes(), ParseEncoding::Ascii, LengthUnit::Bytes);
            for name in ["CNTY", "MY_CNTY"] {
                let field = named(&file.records[0], name);
                assert_eq!(field.text(), code, "{bytes}");
                assert!(
                    matches!(field.value(), Some(Value::Text(text)) if text == code),
                    "{bytes}: {field:?}"
                );
            }
        }
    }
}

#[test]
fn nested_subdivision_errors_preserve_encoded_list_member_spans() {
    for (encoding, first) in [
        (ParseEncoding::Utf8, &b"Jos\xc3\xa9"[..]),
        (ParseEncoding::Latin1, &b"Jos\xe9"[..]),
    ] {
        for unit in [LengthUnit::Bytes, LengthUnit::UnicodeScalars] {
            for (name, before, after, bad, enumeration) in [
                (
                    "USACA_COUNTIES",
                    "MA,",
                    ":",
                    "AK,Imaginary",
                    "Secondary_Administrative_Subdivision",
                ),
                (
                    "CNTY_ALT",
                    "OTHER:",
                    ";",
                    "NZ_Regions:Imaginary",
                    "Secondary_Administrative_Subdivision_Alt",
                ),
            ] {
                let mut value = before.as_bytes().to_vec();
                value.extend_from_slice(first);
                value.extend_from_slice(after.as_bytes());
                let bad_offset = value.len();
                value.extend_from_slice(bad.as_bytes());
                let length =
                    if encoding == ParseEncoding::Utf8 && unit == LengthUnit::UnicodeScalars {
                        std::str::from_utf8(&value)
                            .expect("UTF-8 case should contain valid text")
                            .chars()
                            .count()
                    } else {
                        value.len()
                    };
                let mut bytes = format!("<{name}:{length}>").into_bytes();
                let value_start = bytes.len();
                bytes.extend_from_slice(&value);
                bytes.extend_from_slice(b"<EOR><CALL:1>X<EOR>");
                let mut parser = Parser::new(
                    &bytes,
                    ParseOpts {
                        encoding,
                        field_length: unit,
                    },
                );
                let error = next_error(&mut parser);
                assert!(
                    matches!(error.kind(), error::StructuralErrorKind::InvalidValue {
                    field_name, enumeration: Some(set), ..
                } if field_name == name && *set == enumeration),
                    "{encoding:?}/{unit:?} {name}: {error:?}"
                );
                assert_eq!(
                    error.span(),
                    value_start + bad_offset..value_start + value.len(),
                    "{encoding:?}/{unit:?} {name}: {error}"
                );
                assert_eq!(&bytes[error.span()], bad.as_bytes());
                assert_exhausted(&mut parser);
            }
        }
    }
}
