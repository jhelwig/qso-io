//! Read ADI records while preserving their original bytes and source locations.
//!
//! Start with [`adif::adi::Parser`] and [`adif::ParseOpts`]. Parsing yields complete
//! headers and records. Structural errors, including text-decoding failures,
//! terminate iteration while leaving previously yielded sections available.

/// ADIF parsing options, parsed fields, and reference data types.
pub mod adif;
