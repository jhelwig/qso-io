use super::*;
use crate::adif::{Hemisphere, TimePrecision, Value, ValueError};

fn document(raw: &[u8], kind: Option<char>, encoding: ParseEncoding, unit: LengthUnit) -> Vec<u8> {
    let length = if encoding == ParseEncoding::Utf8 && unit == LengthUnit::UnicodeScalars {
        std::str::from_utf8(raw)
            .expect(
                "UTF-8 scalar fixtures should decode because they contain ASCII or encoded Unicode",
            )
            .chars()
            .count()
    } else {
        raw.len()
    };
    let kind = kind.map(|c| format!(":{c}")).unwrap_or_default();
    let mut bytes = format!("<X:{length}{kind}>").into_bytes();
    bytes.extend_from_slice(raw);
    bytes.extend_from_slice(b"<EOR>");
    bytes
}

#[track_caller]
fn failure(raw: &[u8], kind: char, reason: ValueError, relative: Range<usize>) {
    let mut bytes = b"<X:1>A<EOR>".to_vec();
    let mut bad = document(raw, Some(kind), ParseEncoding::Ascii, LengthUnit::Bytes);
    let start = bytes.len()
        + bad.iter().position(|b| *b == b'>').expect(
            "descriptor should contain > because the test generator appends that delimiter",
        )
        + 1;
    bytes.append(&mut bad);
    bytes.extend_from_slice(b"<X:1>C<EOR>");
    let mut parser = Parser::new(&bytes, ParseOpts::default());
    let _ = next_section(&mut parser);
    let error = next_error(&mut parser);
    assert!(
        matches!(error.kind(),error::StructuralErrorKind::InvalidValue{cause,..} if *cause==reason),
        "input {raw:?}, type {kind}: expected {reason:?}, got {error:?}"
    );
    assert_eq!(
        error.span(),
        start + relative.start..start + relative.end,
        "{raw:?}: {error}"
    );
    assert_exhausted(&mut parser);
    assert_exhausted(&mut parser);
}

#[test]
fn invalid_declared_values_are_terminal() {
    for bytes in [
        &b"<X:3:N>abc<EOR>"[..],
        &b"<X:8:D>20260230<EOR>"[..],
        &b"<X:4:T>2400<EOR>"[..],
        &b"<X:1:S>\t<EOR>"[..],
    ] {
        let mut parser = Parser::new(bytes, ParseOpts::default());
        let _ = next_error(&mut parser);
        assert_exhausted(&mut parser);
    }
}

#[test]
fn numbers_follow_adif_grammar_without_narrowing_or_normalizing_spelling() {
    for text in [
        "0",
        "-0",
        ".5",
        "-.5",
        "1.",
        "001.2300",
        "9007199254740993.125",
    ] {
        let bytes = document(
            text.as_bytes(),
            Some('N'),
            ParseEncoding::Ascii,
            LengthUnit::Bytes,
        );
        let record = parse_file(&bytes, ParseEncoding::Ascii, LengthUnit::Bytes)
            .records
            .remove(0);
        let field = named(&record, "X");
        let Some(Value::Number(n)) = field.value() else {
            panic!("expected Some(Value::Number(n)), got {:?}", field.value());
        };
        assert_eq!(
            n,
            &text
                .parse::<bigdecimal::BigDecimal>()
                .expect("expected decimal should parse because its fixed test spelling uses decimal notation")
        );
        assert_eq!(field.text(), text);
        assert_eq!(field.raw_data(), text.as_bytes());
    }
    for (text, relative) in [
        (".", 1..1),
        ("-", 1..1),
        ("+1", 0..1),
        ("1e3", 1..2),
        ("1_0", 1..2),
        ("1,0", 1..2),
        ("1..2", 2..3),
        (" 1", 0..1),
        ("1 ", 1..2),
    ] {
        failure(text.as_bytes(), 'N', ValueError::InvalidNumber, relative);
    }
}

#[test]
fn boolean_values_accept_only_y_or_n() {
    for (raw, want) in [(b"Y", true), (b"y", true), (b"N", false), (b"n", false)] {
        let bytes = document(raw, Some('B'), ParseEncoding::Ascii, LengthUnit::Bytes);
        let rec = parse_file(&bytes, ParseEncoding::Ascii, LengthUnit::Bytes)
            .records
            .remove(0);
        assert_eq!(
            named(&rec, "X").value(),
            Some(&Value::Boolean(want)),
            "input {raw:?}"
        );
    }
    for (text, span) in [("true", 0..1), ("1", 0..1), ("Y ", 1..2)] {
        failure(text.as_bytes(), 'B', ValueError::InvalidBoolean, span);
    }
}

#[test]
fn dates_enforce_adif_years_and_calendar_components() {
    for text in ["19300101", "20240229", "99991231"] {
        let bytes = document(
            text.as_bytes(),
            Some('D'),
            ParseEncoding::Ascii,
            LengthUnit::Bytes,
        );
        let rec = parse_file(&bytes, ParseEncoding::Ascii, LengthUnit::Bytes)
            .records
            .remove(0);
        let Some(Value::Date(d)) = named(&rec, "X").value() else {
            panic!(
                "expected Some(Value::Date(d)), got {:?}",
                named(&rec, "X").value()
            );
        };
        assert_eq!(d.format("%Y%m%d").to_string(), text);
    }
    for (text, span) in [
        ("19291231", 0..4),
        ("20241301", 4..6),
        ("20260230", 6..8),
        ("2024010", 7..7),
        ("202401010", 8..9),
        ("2024A101", 4..5),
        ("20A", 2..3),
    ] {
        failure(text.as_bytes(), 'D', ValueError::InvalidDate, span);
    }
}

#[test]
fn times_preserve_explicit_seconds_and_reject_invalid_clock_components() {
    for (text, precision, clock) in [
        ("0000", TimePrecision::Minute, "00:00:00"),
        ("2359", TimePrecision::Minute, "23:59:00"),
        ("1305", TimePrecision::Minute, "13:05:00"),
        ("130500", TimePrecision::Second, "13:05:00"),
        ("235959", TimePrecision::Second, "23:59:59"),
    ] {
        let bytes = document(
            text.as_bytes(),
            Some('T'),
            ParseEncoding::Ascii,
            LengthUnit::Bytes,
        );
        let rec = parse_file(&bytes, ParseEncoding::Ascii, LengthUnit::Bytes)
            .records
            .remove(0);
        let Some(Value::Time(t)) = named(&rec, "X").value() else {
            panic!(
                "expected Some(Value::Time(t)), got {:?}",
                named(&rec, "X").value()
            );
        };
        assert_eq!(t.precision(), precision);
        assert_eq!(t.time().to_string(), clock, "input {text:?}");
    }
    for (text, span) in [
        ("2400", 0..2),
        ("1260", 2..4),
        ("235960", 4..6),
        ("13050", 5..5),
        ("1305000", 6..7),
        ("1305.0", 4..5),
        ("1305Z", 4..5),
        ("+105", 0..1),
    ] {
        failure(text.as_bytes(), 'T', ValueError::InvalidTime, span);
    }
}

#[test]
fn locations_preserve_adif_components_and_direction_at_zero() {
    for (text, hemisphere, degrees, minutes) in [
        ("W122 30.125", Hemisphere::West, 122, "30.125"),
        ("N000 00.000", Hemisphere::North, 0, "0"),
        ("S000 00.000", Hemisphere::South, 0, "0"),
        ("E180 59.999", Hemisphere::East, 180, "59.999"),
    ] {
        let bytes = document(
            text.as_bytes(),
            Some('L'),
            ParseEncoding::Ascii,
            LengthUnit::Bytes,
        );
        let rec = parse_file(&bytes, ParseEncoding::Ascii, LengthUnit::Bytes)
            .records
            .remove(0);
        let Some(Value::Location(l)) = named(&rec, "X").value() else {
            panic!(
                "expected Some(Value::Location(l)), got {:?}",
                named(&rec, "X").value()
            );
        };
        assert_eq!(l.hemisphere(), hemisphere);
        assert_eq!(l.degrees(), degrees);
        assert_eq!(
            l.minutes(),
            &minutes
                .parse::<bigdecimal::BigDecimal>()
                .expect("expected decimal should parse because its fixed test spelling uses decimal notation")
        );
        assert_eq!(named(&rec, "X").text(), text);
    }
    for (text, span) in [
        ("X122 30.125", 0..1),
        ("W181 30.125", 1..4),
        ("W122_30.125", 4..5),
        ("W122 60.000", 5..11),
        ("W122 30,125", 7..8),
        ("W122 30.12", 10..10),
        ("W122 30.1250", 11..12),
        ("N000 A0", 5..6),
        ("W1A", 2..3),
    ] {
        failure(text.as_bytes(), 'L', ValueError::InvalidLocation, span);
    }
}

#[test]
fn declared_text_uses_encoding_specific_rules() {
    // Literal accepted/rejected outcomes derive from the approved mode table.
    // Each array is ordered ASCII, Latin-1, UTF-8.
    type TextCase<'a> = (&'a [u8], [bool; 3], [bool; 3], [bool; 3]);
    let cases: &[TextCase<'_>] = &[
        (
            b"hello",
            [true, true, true],
            [true, true, true],
            [true, true, true],
        ),
        (
            b"\t",
            [true, true, true],
            [false, false, true],
            [false, false, true],
        ),
        (
            b"\0",
            [true, true, true],
            [false, false, true],
            [false, false, true],
        ),
        (
            b"\x7f",
            [true, true, true],
            [false, false, true],
            [false, false, true],
        ),
        (
            b"a\r\nb",
            [true, true, true],
            [false, false, false],
            [true, true, true],
        ),
        (
            b"a\rb",
            [true, true, true],
            [false, false, false],
            [false, false, false],
        ),
        (
            b"a\nb",
            [true, true, true],
            [false, false, false],
            [false, false, false],
        ),
        (
            b"a\xc2\x85",
            [false, true, true],
            [false, false, true],
            [false, false, true],
        ),
        (
            b"a\xe2\x80\xa8",
            [false, true, true],
            [false, false, true],
            [false, false, true],
        ),
        (
            b"a\xe2\x80\xa9",
            [false, true, true],
            [false, false, true],
            [false, false, true],
        ),
        (
            b"\xc2\xa0\xc3\xbf",
            [false, true, true],
            [false, true, true],
            [false, true, true],
        ),
    ];
    for (mode, encoding) in [
        ParseEncoding::Ascii,
        ParseEncoding::Latin1,
        ParseEncoding::Utf8,
    ]
    .into_iter()
    .enumerate()
    {
        for unit in [LengthUnit::Bytes, LengthUnit::UnicodeScalars] {
            for &(raw, untyped, string, multiline) in cases {
                for (indicator, allowed) in [
                    (None, untyped[mode]),
                    (Some('S'), string[mode]),
                    (Some('M'), multiline[mode]),
                    (Some('E'), string[mode]),
                ] {
                    let bytes = document(raw, indicator, encoding, unit);
                    let result = Parser::new(
                        &bytes,
                        ParseOpts {
                            encoding,
                            field_length: unit,
                        },
                    )
                    .next()
                    .unwrap_or_else(|| {
                        panic!(
                            "expected a section for {raw:?}, {encoding:?}, {unit:?}, {indicator:?}"
                        )
                    });
                    assert_eq!(
                        result.is_ok(),
                        allowed,
                        "{raw:?}, {encoding:?}, {unit:?}, {indicator:?}: {result:?}"
                    );
                    if let Ok(section) = result {
                        let rec = match section {
                            Section::Record(rec) => rec,
                            other => panic!(
                                "expected record for {raw:?}, {encoding:?}, {unit:?}, {indicator:?}, got {other:?}"
                            ),
                        };
                        assert_eq!(named(&rec, "X").raw_data(), raw);
                    }
                }
            }
            if encoding == ParseEncoding::Latin1 {
                for (raw, typed_allowed) in [
                    (&b"\x80"[..], false),
                    (&b"\x9f"[..], false),
                    (&b"\xa0"[..], true),
                    (&b"\xff"[..], true),
                ] {
                    for indicator in [None, Some('S'), Some('M'), Some('E')] {
                        let bytes = document(raw, indicator, encoding, unit);
                        let result = Parser::new(
                            &bytes,
                            ParseOpts {
                                encoding,
                                field_length: unit,
                            },
                        )
                        .next()
                        .unwrap_or_else(|| panic!("expected a section for {raw:?}, {encoding:?}, {unit:?}, {indicator:?}"));
                        assert_eq!(
                            result.is_ok(),
                            indicator.is_none() || typed_allowed,
                            "{raw:?}, {encoding:?}, {unit:?}, {indicator:?}: {result:?}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn numeric_grammars_remain_ascii_in_utf8_mode() {
    for (kind, text, reason, relative) in [
        ('N', "1١", ValueError::InvalidNumber, 1),
        ('D', "2024١101", ValueError::InvalidDate, 4),
        ('T', "13١5", ValueError::InvalidTime, 2),
        ('L', "N0١0 00.000", ValueError::InvalidLocation, 2),
    ] {
        for unit in [LengthUnit::Bytes, LengthUnit::UnicodeScalars] {
            let bytes = document(text.as_bytes(), Some(kind), ParseEncoding::Utf8, unit);
            let start = bytes.iter().position(|b| *b == b'>').expect(
                "descriptor should contain > because the test generator appends that delimiter",
            ) + 1;
            let mut parser = Parser::new(
                &bytes,
                ParseOpts {
                    encoding: ParseEncoding::Utf8,
                    field_length: unit,
                },
            );
            let error = next_error(&mut parser);
            assert_eq!(
                error.span(),
                start + relative..start + relative + 1,
                "input {text:?}, type {kind}, unit {unit:?}: {error:?}"
            );
            assert!(
                matches!(error.kind(),error::StructuralErrorKind::InvalidValue{cause,..} if *cause==reason),
                "input {text:?}, type {kind}, unit {unit:?}: expected {reason:?}, got {error:?}"
            );
        }
    }
}
