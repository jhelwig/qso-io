use std::ops::Range;

use winnow::error::{ContextError, FromExternalError};

use super::error::{AdiError, StructuralErrorKind, TypeAlternativeFailure};
use super::{DataSpecifier, DataType, Field, FramedField, State, Stream, values};
use crate::adif::{
    ValueError,
    catalog::{self, FieldSection, StandardField},
};

#[derive(Debug, Clone, Copy)]
pub(super) enum TypeSelection {
    Untyped,
    Single(DataType),
    Alternatives(&'static [DataType]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DefinitionSource {
    Standard,
    Application,
    InlineOrUnknown,
}

pub(super) struct ResolvedDefinition {
    pub selection: TypeSelection,
    pub source: DefinitionSource,
    pub standard: Option<StandardField>,
}

pub(super) fn is_application(name: &str) -> bool {
    name.get(..4)
        .is_some_and(|p| p.eq_ignore_ascii_case("APP_"))
        && name.get(4..).is_some_and(|tail| {
            tail.split_once('_')
                .is_some_and(|(program, field)| !program.is_empty() && !field.is_empty())
        })
}

pub(super) fn resolve(
    _state: &State,
    section: FieldSection,
    specifier: &DataSpecifier<'_>,
    descriptor_span: Range<usize>,
) -> Result<ResolvedDefinition, AdiError> {
    let standard = catalog::lookup(section, &specifier.name);
    let source = if standard.is_some() {
        DefinitionSource::Standard
    } else if section == FieldSection::Record && is_application(&specifier.name) {
        DefinitionSource::Application
    } else {
        DefinitionSource::InlineOrUnknown
    };
    let selection = if let Some(definition) = standard {
        if let Some(supplied) = specifier.type_indicator
            && !definition.data_types().contains(&supplied)
        {
            return Err(AdiError::Defined {
                span: descriptor_span,
                kind: StructuralErrorKind::TypeConflict {
                    field_name: specifier.name.to_string(),
                    expected: definition.data_types().to_vec(),
                    supplied,
                    definition_span: None,
                },
            });
        }
        if let [only] = definition.data_types() {
            TypeSelection::Single(*only)
        } else {
            TypeSelection::Alternatives(definition.data_types())
        }
    } else if source == DefinitionSource::Application {
        TypeSelection::Single(
            specifier
                .type_indicator
                .unwrap_or(DataType::MultilineString),
        )
    } else {
        specifier
            .type_indicator
            .map_or(TypeSelection::Untyped, TypeSelection::Single)
    };
    if let TypeSelection::Single(
        kind @ (DataType::IntlCharacter | DataType::IntlString | DataType::IntlMultilineString),
    ) = selection
    {
        return Err(AdiError::TypeUnavailableInAdi {
            data_type: kind,
            span: descriptor_span,
        });
    }

    Ok(ResolvedDefinition {
        selection,
        source,
        standard,
    })
}

fn parse_type<'a>(
    input: &Stream<'a>,
    framed: FramedField<'a>,
    kind: Option<DataType>,
    standard: Option<StandardField>,
) -> winnow::Result<Field<'a>> {
    let field = values::parse(input, framed, kind)?;
    if kind == Some(DataType::Enumeration)
        && field.value().is_some()
        && let Some(set) = standard.and_then(StandardField::enumeration)
    {
        catalog::check_membership(set, field.text()).map_err(|set| {
            ContextError::from_external_error(
                input,
                AdiError::InvalidValue {
                    field_name: field.name().to_owned(),
                    span: field.value_span(),
                    cause: ValueError::InvalidEnumeration,
                    enumeration: Some(set),
                    numeric_cause: None,
                },
            )
        })?;
    }
    Ok(field)
}

pub(super) fn parse_resolved<'a>(
    input: &Stream<'a>,
    framed: FramedField<'a>,
    resolution: &ResolvedDefinition,
) -> winnow::Result<Field<'a>> {
    let alternatives = match resolution.selection {
        TypeSelection::Untyped => return parse_type(input, framed, None, resolution.standard),
        TypeSelection::Single(kind) => {
            return parse_type(input, framed, Some(kind), resolution.standard);
        }
        TypeSelection::Alternatives(types) => types,
    };
    let mut failures = Vec::new();
    for kind in alternatives {
        match parse_type(input, framed.clone(), Some(*kind), resolution.standard) {
            Ok(field) => return Ok(field),
            Err(error) => {
                if let Some(AdiError::InvalidValue {
                    span,
                    cause,
                    enumeration,
                    ..
                }) = error.cause().and_then(|c| c.downcast_ref::<AdiError>())
                {
                    failures.push(TypeAlternativeFailure {
                        data_type: *kind,
                        span: span.clone(),
                        cause: *cause,
                        enumeration: *enumeration,
                    });
                } else {
                    return Err(error);
                }
            }
        }
    }
    let span = failures
        .first()
        .map_or_else(|| framed.value_span.clone(), |f| f.span.clone());
    Err(ContextError::from_external_error(
        input,
        AdiError::Defined {
            span,
            kind: StructuralErrorKind::NoMatchingType {
                field_name: framed.specifier.name.to_string(),
                failures,
            },
        },
    ))
}
