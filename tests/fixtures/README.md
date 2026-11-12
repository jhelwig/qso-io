# Upstream ADI fixtures

These 12 fixtures are unmodified upstream files, downloaded directly from the
pinned revisions in [provenance.json](provenance.json). Their original bytes,
line endings, spelling, framing mistakes and encoding discrepancies are retained.
The `.adif` extension on difa's sample still contains ADI text.

This directory supplies inputs for independently written tests. It contains no
parser implementation or executable test harness. The classifications below
describe inspected properties; they are not a claim that every file passes all
ADIF semantic validation.

## Licenses and provenance

The third-party files retain their upstream licenses; the project's dual license
does not replace them. Preserve the applicable notices when redistributing these
fixtures. No public-domain or CC0 status is claimed.

| Directory | Upstream at pinned revision | License retained here |
| --- | --- | --- |
| `matir-adifparser` | [Matir/adifparser](https://github.com/Matir/adifparser/tree/2c465737f437b70867819bb986aa79722057bd1c) | [BSD-2-Clause](matir-adifparser/COPYING) |
| `difa` | [cammeresi/adif](https://github.com/cammeresi/adif/tree/63783b458d52026d944af49594a7d2a5a397cdb8) | [BSD-3-Clause](difa/LICENSE) |
| `adif-parser-ts` | [k0swe/adif-parser-ts](https://github.com/k0swe/adif-parser-ts/tree/a77ad183a79148749f9d9d21bd5f010fcd2dcf3f) | [Apache-2.0](adif-parser-ts/LICENSE) |
| `transadif` | [ham2k/transadif](https://github.com/ham2k/transadif/tree/90586ecf600623a51b560a2cfa3a443b3d9c8d56) | [MIT](transadif/LICENSE) |

The provenance manifest records every fixture and license file's repository,
commit, original path, raw download URL, upstream Git blob ID and byte count.
Files live under a source-specific directory with their upstream paths intact.
The selected upstream trees contain no separate NOTICE file. Full license texts
are included, not substituted with abbreviated license identifiers.

## Fixture classification

Paths below are relative to their source-specific directory. ASCII inputs have
the same boundaries under byte and Unicode-scalar length settings.

| Source and file | Classification | Useful properties and caveats |
| --- | --- | --- |
| Matir `testdata/header_none.adi` | Complete framing; ASCII | Headerless single record. `MYCALL` is not a standard-field recognition requirement. |
| Matir `testdata/header_comment.adi` | Complete framing; ASCII | Preamble-only header followed by one record. |
| Matir `testdata/header_version.adi` | **Malformed structural input** | `<mycall>` omits the required length. It also starts directly with `<adif_ver:4>`, then supplies EOH; that is not the standard header form with a non-`<` opening character. Do not treat the filename as proof of valid framing. |
| Matir `testdata/readrecord.adi` | **Malformed: unterminated final record** | Two complete records precede `<mycall:4>W1AW` without EOR. Useful for testing a reliable prefix and an unfinished final record. |
| difa `examples/sample.adif` | Complete framing; ASCII | Header and three records; four- and six-digit times, several bands/modes. No claim that every mode spelling is canonical for the current standard. |
| TypeScript `test/data/wsjtx.adi` | Complete framing; ASCII | WSJT-X export, exact decimal frequency text, six-digit times and signal reports. |
| TypeScript `test/data/js8call.adi` | Complete framing; ASCII | JS8Call export with mode/submode and explicit zero-length GRIDSQUARE/RST_RCVD fields. |
| TransADIF `test-cases/01-plain-examples/1-plain-ascii-in.adi` | Complete framing; ASCII | Header, CRLF line endings, decimal frequency and minute-precision time. |
| TransADIF `test-cases/01-plain-examples/2-plain-iso-in.adi` | **Latin-1 compatibility input** | Original name bytes include `E1` and `F1`; they are not valid UTF-8. NAME has 14 original bytes. Use explicit Latin-1, not the decoded preview from a file API. Non-ASCII text is outside strict ASCII ADI. |
| TransADIF `test-cases/01-plain-examples/3-plain-utf-in.adi` | **UTF-8 compatibility input with scalar-counted NAME** | NAME declares 14; its intended line value has 14 scalars and 16 UTF-8 bytes. With byte lengths, the framed value ends at `Juán Muñoz U`, leaving `TF` outside the field. Do not silently reinterpret lengths or require a structural error merely because intended text differs. |
| TransADIF `test-cases/02-field-length/1-undercount-in.adi` | **Mixed compatibility and invalid field content** | UTF-8 NAME declares 18, matching its 18 bytes/16 scalars. In byte mode, NOTES and COMMENT each declare 35 but their line text is 34 bytes, so each consumes a lone CR. A lone CR is invalid multiline/single-line field content, separately from reliable byte framing. See details below. |
| TransADIF `test-cases/02-field-length/2-overcount-in.adi` | **Misleading encoding label; length-mode-dependent input** | The preamble calls this ISO-8859-1, but the actual name uses UTF-8. NAME declares 24, matching its 24 bytes/21 scalars. Scalar mode consumes the following CRLF and the opening `<` of the next descriptor. It is not a valid scalar-counted example of the intended fields. |

## Malformed and compatibility cases

The Matir missing-length and unfinished-record cases are structural negatives.
The TransADIF files must be interpreted using explicit encoding and length units;
their upstream names and explanatory preambles are evidence, not test oracles.

In `1-undercount-in.adi`, byte mode frames NAME as intended, but the trailing
lone CR in NOTES/COMMENT is a field-content problem. Under UTF-8 scalar lengths,
NAME consumes its following CRLF as well. Keep these distinctions separate from
malformed descriptor, truncated-value and unfinished-record errors.

In `2-overcount-in.adi`, the original NAME bytes contain `C3 A1`, `C3 B1` and
`C3 AD`, not Latin-1's corresponding single bytes. Byte mode preserves the name;
scalar mode includes the next descriptor's opening delimiter. Whether following
inter-field text can still be parsed depends on the actual grammar, so do not
derive a mandatory structural rejection or recovery rule from the filename.

TransADIF repairs encodings and field counts. Its expected-output files were
intentionally not imported: they would encode behavior that is not the explicit
parser contract here. Mojibake/entity-repair cases were also left out of this
initial selection.

The official ADIF generated corpus is not included because its redistribution
terms were not established by the source inspection. A generator's software
license alone was not treated as a license for its separately distributed output.

## Byte preservation and verification

Download from the pinned raw URLs in the manifest, not from a rendered page or a
text API that may transcode bytes. Do not normalize newlines or repair fixture
contents in place. Add separately named derived cases if changes are needed,
preserving provenance and explaining those changes.

The nested `.gitattributes` disables Git text normalization for fixture and
upstream license files. Before importing a source update, compare each raw file's
byte count and Git blob ID with the corresponding pinned upstream tree entry.
For example, from the repository root:

```sh
git hash-object --no-filters tests/fixtures/transadif/test-cases/01-plain-examples/2-plain-iso-in.adi
```

The result for that file is `791ccc7781ad0206f14a74a3f4eec01c7fd39422`.
This checks identity without running or depending on any parser implementation.
