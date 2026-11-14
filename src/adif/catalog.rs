//! Parser coverage policy over the maintained ADIF reference types.

use super::{Enumeration, SecondarySubdivision, SecondarySubdivisionAlt};
pub(crate) use super::{FieldSection, StandardField};

pub(crate) fn lookup(section: FieldSection, name: &str) -> Option<StandardField> {
    name.parse::<StandardField>()
        .ok()
        .filter(|field| field.section() == section)
}

pub(crate) fn is_standard_name(name: &str) -> bool {
    let userdef = name
        .get(..7)
        .is_some_and(|p| p.eq_ignore_ascii_case("USERDEF"))
        && name.len() > 7
        && name.as_bytes()[7..].iter().all(u8::is_ascii_digit)
        && name.as_bytes()[7..].iter().any(|b| *b != b'0');
    userdef || name.parse::<StandardField>().is_ok()
}

pub(crate) fn check_membership(set: Enumeration, value: &str) -> Result<(), &'static str> {
    let accepted = match set {
        Enumeration::SecondarySubdivision => {
            SecondarySubdivision::candidates(value).next().is_some()
        }
        Enumeration::SecondarySubdivisionAlt => {
            SecondarySubdivisionAlt::parse_member(value).is_some()
        }
        _ => set.contains(value),
    };
    if accepted { Ok(()) } else { Err(set.name()) }
}
