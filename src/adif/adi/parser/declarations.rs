use bigdecimal::num_bigint::BigUint;

use super::*;
use crate::adif::adi::{UserDefinition, UserDefinitionConstraint, UserDefinitionError};
use crate::adif::{number::parse_number, text::check_text};

pub(super) fn is_declaration(name: &str) -> bool {
    name.get(..7)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("USERDEF"))
        && name.len() > 7
        && name.as_bytes()[7..].iter().all(u8::is_ascii_digit)
}

pub(super) fn parse<'a>(input: &Stream<'a>, framed: FramedField<'a>) -> Result<UserDefinition<'a>> {
    let failure = |cause, span| {
        ContextError::from_external_error(
            input,
            AdiError::InvalidUserDefinition {
                field_name: framed.specifier.name.to_string(),
                span,
                cause,
                numeric_cause: None,
            },
        )
    };

    let suffix = framed.specifier.name.get(7..).ok_or_else(|| {
        failure(
            UserDefinitionError::InvalidDeclarationNumber,
            framed.descriptor_span.clone(),
        )
    })?;
    let number_span = framed.descriptor_span.start + 8
        ..framed.descriptor_span.start + 1 + framed.specifier.name.len();
    let number: BigUint = suffix.parse().map_err(|_| {
        failure(
            UserDefinitionError::InvalidDeclarationNumber,
            number_span.clone(),
        )
    })?;

    if number == BigUint::default() {
        return Err(failure(
            UserDefinitionError::InvalidDeclarationNumber,
            number_span,
        ));
    }

    let bytes = framed.raw_data;
    let comma = bytes.iter().position(|b| *b == b',');
    let name_end = comma.unwrap_or(bytes.len());
    let field_name = decode_located(
        input,
        &bytes[..name_end],
        input.state.parser_options.encoding,
        framed.value_span.start,
    )?
    .into_owned();
    let start = framed.value_span.start;

    let constraint = if let Some(comma) = comma {
        let open = comma + 1;

        if bytes.get(open) != Some(&b'{') {
            return Err(failure(
                UserDefinitionError::MalformedDeclaration,
                start + open..start + (open + 1).min(bytes.len()),
            ));
        }
        if bytes.last() != Some(&b'}') || bytes.len() <= open + 1 {
            return Err(failure(
                UserDefinitionError::MalformedDeclaration,
                start + bytes.len()..start + bytes.len(),
            ));
        }

        let body = &bytes[open + 1..bytes.len() - 1];
        let body_start = start + open + 1;

        if framed.specifier.type_indicator != Some(DataType::Enumeration)
            && let Some(colon) = body.iter().position(|b| *b == b':')
        {
            let bound = |part: &[u8], offset| {
                parse_number(part).map_err(|error| {
                    ContextError::from_external_error(
                        input,
                        AdiError::InvalidUserDefinition {
                            field_name: framed.specifier.name.to_string(),
                            span: offset + error.span.start..offset + error.span.end,
                            cause: UserDefinitionError::InvalidRangeBound,
                            numeric_cause: error.cause.map(Into::into),
                        },
                    )
                })
            };

            let lower = bound(&body[..colon], body_start)?;
            let upper = bound(&body[colon + 1..], body_start + colon + 1)?;

            if upper <= lower {
                return Err(failure(
                    UserDefinitionError::InvalidRangeOrder,
                    body_start + colon + 1..body_start + body.len(),
                ));
            }

            Some(UserDefinitionConstraint::Range { lower, upper })
        } else {
            let mut items = Vec::new();
            let mut offset = body_start;

            for part in body.split(|b| *b == b',') {
                if let Err(span) = check_text(part, input.state.parser_options.encoding, false) {
                    return Err(failure(
                        UserDefinitionError::InvalidEnumeration,
                        offset + span.start..offset + span.end,
                    ));
                }

                items.push(
                    decode_located(input, part, input.state.parser_options.encoding, offset)?
                        .into_owned(),
                );
                offset += part.len() + 1;
            }

            Some(UserDefinitionConstraint::Enumeration(items))
        }
    } else {
        None
    };

    Ok(UserDefinition {
        specifier: framed.specifier,
        number,
        field_name,
        constraint,
        raw_data: framed.raw_data,
        text: framed.text,
        descriptor_span: framed.descriptor_span,
        value_span: framed.value_span,
    })
}
