use std::{borrow::Cow, collections::HashSet};

use super::{DataType, ParseEncoding, ValueError, ValueFailure, identifiers, source_range};
use crate::adif::{Enumeration, catalog, text::check_text};

pub(super) fn parse<'a>(
    kind: DataType,
    text: Cow<'a, str>,
    raw: &'a [u8],
    encoding: ParseEncoding,
) -> Result<(Vec<Cow<'a, str>>, Cow<'a, str>), ValueFailure> {
    let (separator, minimum, cause) = match kind {
        DataType::AwardList => (',', 1, ValueError::InvalidAwardList),
        DataType::CreditList => (',', 1, ValueError::InvalidCreditList),
        DataType::SponsoredAwardList => (',', 1, ValueError::InvalidSponsoredAwardList),
        DataType::GridSquareList => (',', 1, ValueError::InvalidGridSquareList),
        DataType::POTARefList => (',', 1, ValueError::InvalidPOTARefList),
        DataType::SecondarySubdivisionList => (':', 2, ValueError::InvalidSecondarySubdivisionList),
        DataType::SecondaryAdministrativeSubdivisionListAlt => (
            ';',
            1,
            ValueError::InvalidSecondaryAdministrativeSubdivisionListAlt,
        ),
        _ => (',', 1, ValueError::InvalidString),
    };
    let fail = |range, enumeration| ValueFailure {
        cause,
        span: source_range(&text, range, encoding),
        enumeration,
    };
    check_text(raw, encoding, false).map_err(|span| ValueFailure {
        cause,
        span,
        enumeration: None,
    })?;

    let mut offset = 0;
    let mut count = 0;
    let mut names = HashSet::new();
    for member in text.split(separator) {
        let end = offset + member.len();
        if member.is_empty() {
            return Err(fail(offset..end, None));
        }
        let membership = |set| {
            catalog::check_membership(set, member).map_err(|set| fail(offset..end, Some(set)))
        };
        match kind {
            DataType::AwardList => membership(Enumeration::Award)?,
            DataType::CreditList => {
                let (credit, media) = match member.split_once(':') {
                    Some((credit, media)) => (credit, Some(media)),
                    None => (member, None),
                };
                catalog::check_membership(Enumeration::Credit, credit)
                    .map_err(|set| fail(offset..offset + credit.len(), Some(set)))?;
                if let Some(media) = media {
                    let mut token_offset = offset + credit.len() + 1;
                    for medium in media.split('&') {
                        catalog::check_membership(Enumeration::QslMedium, medium).map_err(
                            |set| fail(token_offset..token_offset + medium.len(), Some(set)),
                        )?;
                        token_offset += medium.len() + 1;
                    }
                }
            }
            DataType::SponsoredAwardList => {
                let parts: Vec<_> = member.splitn(3, '_').collect();
                if parts.len() != 3 || parts.iter().any(|p| p.is_empty()) {
                    return Err(fail(offset..end, None));
                }
                if !Enumeration::AwardSponsor.contains(parts[0]) {
                    return Err(fail(
                        offset..offset + parts[0].len() + 1,
                        Some("Award_Sponsor"),
                    ));
                }
            }
            DataType::SecondarySubdivisionList => membership(Enumeration::SecondarySubdivision)?,
            DataType::SecondaryAdministrativeSubdivisionListAlt => {
                let Some((name, code)) = member.split_once(':') else {
                    return Err(fail(end..end, None));
                };
                if name.is_empty() || code.is_empty() || !names.insert(name.to_ascii_uppercase()) {
                    return Err(fail(offset..end, None));
                }
                membership(Enumeration::SecondarySubdivisionAlt)?;
            }
            DataType::GridSquareList | DataType::POTARefList => {
                let member_kind = if kind == DataType::GridSquareList {
                    DataType::GridSquare
                } else {
                    DataType::POTARef
                };
                let raw_span = source_range(&text, offset..end, encoding);
                identifiers::check(member_kind, member, &raw[raw_span.clone()], encoding).map_err(
                    |e| ValueFailure {
                        cause: e.cause,
                        span: raw_span.start + e.span.start..raw_span.start + e.span.end,
                        enumeration: e.enumeration,
                    },
                )?;
            }
            _ => {}
        }
        count += 1;
        offset = end + separator.len_utf8();
    }
    if count < minimum {
        return Err(fail(text.len()..text.len(), None));
    }

    let members = match &text {
        Cow::Borrowed(source) => source.split(separator).map(Cow::Borrowed).collect(),
        Cow::Owned(source) => source
            .split(separator)
            .map(|part| Cow::Owned(part.to_owned()))
            .collect(),
    };
    Ok((members, text))
}
