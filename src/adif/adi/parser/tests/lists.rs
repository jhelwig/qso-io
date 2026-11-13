use super::*;
use crate::adif::Value;

fn document(raw: &[u8]) -> Vec<u8> {
    let mut bytes = format!("<X:{}>", raw.len()).into_bytes();
    bytes.extend_from_slice(raw);
    bytes
}

fn list<'a>(
    bytes: &'a [u8],
    kind: DataType,
    encoding: ParseEncoding,
) -> std::result::Result<Field<'a>, StructuralError> {
    let mut input = stream(bytes, encoding, LengthUnit::Bytes);
    let framed = parse_ok(framed_field, &mut input);
    crate::adif::adi::parser::values::parse(&input, framed, Some(kind)).map_err(|error| {
        StructuralError::from_context(error, input.current_token_start(), bytes.len())
    })
}

#[test]
fn list_members_keep_top_level_boundaries_and_spelling() {
    for (kind, text, expected) in [
        (
            DataType::CreditList,
            "IOTA,WAS:LOTW&CARD,DXCC:CARD",
            vec!["IOTA", "WAS:LOTW&CARD", "DXCC:CARD"],
        ),
        (
            DataType::SecondarySubdivisionList,
            "MA,Franklin:MA,Hampshire",
            vec!["MA,Franklin", "MA,Hampshire"],
        ),
        (
            DataType::SponsoredAwardList,
            "aRrL_PROGRAM_AWARD,cq_OTHER_UNKNOWN",
            vec!["aRrL_PROGRAM_AWARD", "cq_OTHER_UNKNOWN"],
        ),
        (DataType::GridSquareList, "EM98,fm08", vec!["EM98", "fm08"]),
        (DataType::POTARefList, "K-4562@US-CA", vec!["K-4562@US-CA"]),
    ] {
        let bytes = document(text.as_bytes());
        let field = list(&bytes, kind, ParseEncoding::Ascii)
            .unwrap_or_else(|e| panic!("{kind:?} {text}: {e}"));
        let Some(Value::List(items)) = field.value() else {
            panic!("expected List, got {field:?}");
        };
        assert_eq!(items, &expected, "{kind:?}");
        assert_eq!(field.text(), text);
        assert_eq!(field.raw_data(), text.as_bytes());
    }
}

#[test]
fn list_member_checks_obey_adif_source_coverage() {
    for (kind, text, bad) in [
        (DataType::CreditList, "IOTA,WAS:NOPE", "NOPE"),
        (DataType::CreditList, "NOPE", "NOPE"),
        (
            DataType::SecondarySubdivisionList,
            "AK,Imaginary:MA,Franklin",
            "AK,Imaginary",
        ),
        (
            DataType::SecondaryAdministrativeSubdivisionListAlt,
            "NZ_Regions:Hawkes Bay/Wairoa;NZ_Regions:Hawkes Bay/Wairoa",
            "NZ_Regions:Hawkes Bay/Wairoa",
        ),
        (
            DataType::SecondaryAdministrativeSubdivisionListAlt,
            "NZ_Regions:Imaginary",
            "NZ_Regions:Imaginary",
        ),
        (DataType::SponsoredAwardList, "NOPE_PROGRAM_AWARD", "NOPE_"),
        (DataType::SponsoredAwardList, "ARRL__AWARD", "ARRL__AWARD"),
        (
            DataType::SponsoredAwardList,
            "ARRL_PROGRAM_",
            "ARRL_PROGRAM_",
        ),
        (DataType::SponsoredAwardList, "ARRL_PROGRAM", "ARRL_PROGRAM"),
        (DataType::GridSquareList, "EM98,", ""),
    ] {
        let bytes = document(text.as_bytes());
        let error = list(&bytes, kind, ParseEncoding::Ascii).expect_err("invalid list should fail");
        assert_eq!(
            &bytes[error.span()],
            bad.as_bytes(),
            "{kind:?} {text}: {error:?}"
        );
    }
    for text in ["NZ_Regions:Hawkes Bay/Wairoa", "OTHER:Unlisted"] {
        let bytes = document(text.as_bytes());
        list(
            &bytes,
            DataType::SecondaryAdministrativeSubdivisionListAlt,
            ParseEncoding::Ascii,
        )
        .unwrap_or_else(|e| panic!("source-bounded {text}: {e}"));
    }
}

#[test]
fn latin1_list_members_own_their_text_safely() {
    let bytes = document(b"MA,Jos\xe9:MA,Franklin");
    let fields = vec![
        list(
            &bytes,
            DataType::SecondarySubdivisionList,
            ParseEncoding::Latin1,
        )
        .unwrap_or_else(|e| panic!("Latin-1 list: {e}")),
    ];
    for field in fields {
        assert_eq!(field.text(), "MA,José:MA,Franklin");
        let Some(Value::List(items)) = field.value() else {
            panic!("expected List, got {field:?}");
        };
        assert_eq!(items, &["MA,José", "MA,Franklin"]);
        assert_eq!(field.raw_data(), b"MA,Jos\xe9:MA,Franklin");
    }
}
