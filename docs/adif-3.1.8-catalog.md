# ADIF 3.1.8 reference types

The maintained Rust definitions follow the released
[ADIF 3.1.8 specification](https://www.adif.org.uk/318/ADIF_318.htm), dated
2026-10-12. Rust definitions and their documentation are the crate's source of
truth. Official JSON exports serve as independent validation inputs.

## Field and member identity

`adif::StandardField` describes ordinary header and QSO fields from III.C.1.a/b:
canonical field names, section, data types and applicable enumeration. The
parser uses those definitions directly. USERDEFn is a numeric-name production;
application-defined and user-defined fields retain their document rules.

Reference types under `adif` describe the enumerations in III.B. Named enums
represent antenna paths, confirmation states, modes, awards and geographic codes.
Primary subdivisions have separate country-specific enums under
`adif::primary_subdivisions`, following each DXCC group in III.B.12. Each variant
carries its documented meaning and canonical wire spelling. Strum derives code
parsing and iteration from those variants. The existing
`AntPath`, `ArrlSection` and `Award` types participate in parser membership checks;
there is no parallel membership string catalog.

Reference parsing and parser membership use ASCII case-insensitive identity
without trimming. Original ADI spelling and the agreed `Value::Text` and
`Value::List` representations remain available independently of reference types.
ARRL sections retain historical dates and multiple DXCC associations.
`dxcc_entity_codes()` replaces the singular accessor to expose all associations
without selecting one arbitrarily.

## Coverage policy

Secondary subdivisions dispatch comma-separated county codes by their explicit
state prefix. The `AK` branch parses the county name using `AlaskaCounty`;
`HI` retains a nonempty county name, and other prefixes use the existing United
States primary-subdivision enum. A nested failure is terminal within the selected
branch. The leaf enum's codes contain only county names, without the state prefix.

Codes without commas are checked by constructing format-compatible
`SecondarySubdivision` candidates across the groups defined in III.B.22. Formats
may overlap; the parser accepts any successful candidate without assigning a
country. Japanese city, ku, gun and combined AJA forms share the Japan variant.
External membership is unchecked; inputs must satisfy a published code format.

Alternate subdivisions dispatch the name before `:`. `NZ_Regions` parses the
remaining region/district pair through `NzRegionDistrict`; nested failure cannot
fall through to another enumeration. Other nonempty names and codes are retained
as `SecondarySubdivisionAlt::Other`. List framing and duplicate-name checks remain
in the list parser. The leaf enum contains region/district codes without the
`NZ_Regions:` prefix. DOK membership remains unchecked.

`SecondarySubdivision::candidates` and `SecondarySubdivisionAlt::parse_member`
perform these checks without selecting the field's stored representation: ADI
values remain text or top-level text lists with their original spelling.
`Enumeration::contains` reports supplied membership, so externally assigned codes
can pass the parser's source-limited checks without being listed reference members.

Primary-subdivision membership tries the country-specific enum parsers and
accepts when any group recognizes the code. It requires no DXCC field and
neither identifies nor infers a country. Checking the record's country/subdivision
relationship remains a separate validation concern. Historical entries remain members independently of
QSO-date applicability.

Sponsored awards check the ADIF-supplied sponsor identifiers and compound grammar.
`AwardSponsor` represents the sponsor component, such as `ARRL`; the compound
parser consumes the `_` separators in `SPONSOR_PROGRAM_AWARD`. Program and award
assignments are external.

String-field enumeration recommendations do not replace the String data type.
ADIF_VER is metadata and does not select another catalog.

## Independent validation inputs

The official resource archive's `fields.json` and `enumerations.json` can be
used to check maintained names, type associations and membership against the
specification. They do not create Rust types or their documentation. Export
coverage is interpreted alongside the specification's prose and source limits.

The released exports used for cross-checking have these original-byte hashes:

- fields.json: `09ed6c7afa89420ccadcd60bd319d1c01cf86c2728264e2e4f0acf14a2bf6033`
- enumerations.json: `7a161acdec46e19e058cb626f44b2f8733608fb82546549b9cb7b04b7dca8054`

The exports are available from the
[official resources](https://www.adif.org.uk/318/ADIF_318_resources.htm).
