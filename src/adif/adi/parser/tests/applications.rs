use super::*;
use crate::adif::Value;

#[test]
fn application_types_inherit_and_keep_the_first_descriptor() {
    let bytes=b"<APP_EXAMPLE_SCORE:3:N>123<EOR><app_example_score:3>456<EOR><APP_EXAMPLE_SCORE:1:L>X<EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    for (expected, indicator) in [("123", Some(DataType::Number)), ("456", None)] {
        let section = next_section(&mut parser);
        let Section::Record(record) = section else {
            panic!("expected Number record, got {section:?}");
        };
        let field = named(&record, "APP_EXAMPLE_SCORE");
        assert_eq!(field.data_type(), Some(DataType::Number));
        assert_eq!(field.type_indicator(), indicator);
        assert!(
            matches!(field.value(),Some(Value::Number(n)) if n.to_string()==expected),
            "{field:?}"
        );
    }
    let error = next_error(&mut parser);
    let error::StructuralErrorKind::TypeConflict {
        definition_span: Some(span),
        supplied: DataType::Location,
        ..
    } = error.kind()
    else {
        panic!("expected conflicting Location, got {error:?}");
    };
    assert_eq!(&bytes[span.clone()], b"<APP_EXAMPLE_SCORE:3:N>");
    assert_eq!(&bytes[error.span()], b"<APP_EXAMPLE_SCORE:1:L>");
    assert_exhausted(&mut parser);
}

#[test]
fn application_empty_first_occurrence_establishes_type() {
    for first in ["<APP_EXAMPLE_SCORE:0:N>", "<APP_EXAMPLE_SCORE:0>"] {
        let bytes = format!("{first}<EOR><APP_EXAMPLE_SCORE:1:N>1<EOR>");
        let mut parser = Parser::new(bytes.as_bytes(), ParseOpts::default());
        let section = next_section(&mut parser);
        let Section::Record(record) = section else {
            panic!("expected empty record, got {section:?}");
        };
        assert!(named(&record, "APP_EXAMPLE_SCORE").value().is_none());
        if first.contains(":N>") {
            let section = next_section(&mut parser);
            let Section::Record(record) = section else {
                panic!("expected inherited Number, got {section:?}");
            };
            assert_eq!(
                named(&record, "APP_EXAMPLE_SCORE").data_type(),
                Some(DataType::Number)
            );
        } else {
            let error = next_error(&mut parser);
            assert!(
                matches!(error.kind(),error::StructuralErrorKind::TypeConflict {expected,..} if expected==&[DataType::MultilineString]),
                "{error:?}"
            );
        }
    }
}

#[test]
fn application_type_state_is_document_local_and_respects_userdef() {
    let mut first = Parser::new(b"<APP_EXAMPLE_SCORE:1:N>1<EOR>", ParseOpts::default());
    let _ = next_section(&mut first);
    let mut second = Parser::new(b"<APP_EXAMPLE_SCORE:1>A<EOR>", ParseOpts::default());
    let section = next_section(&mut second);
    let Section::Record(record) = section else {
        panic!("expected fresh parser record, got {section:?}");
    };
    assert_eq!(
        named(&record, "APP_EXAMPLE_SCORE").data_type(),
        Some(DataType::MultilineString)
    );

    let bytes=b"P<USERDEF1:17:N>APP_EXAMPLE_SCORE<EOH><APP_EXAMPLE_SCORE:1>1<EOR><APP_EXAMPLE_SCORE:1:S>A<EOR>";
    let mut parser = Parser::new(bytes, ParseOpts::default());
    drop(next_section(&mut parser));
    let _ = next_section(&mut parser);
    let error = next_error(&mut parser);
    let error::StructuralErrorKind::TypeConflict {
        definition_span: Some(span),
        ..
    } = error.kind()
    else {
        panic!("expected USERDEF conflict, got {error:?}");
    };
    assert_eq!(&bytes[span.clone()], b"<USERDEF1:17:N>");
}
