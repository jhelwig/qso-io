//! Read ADI records while preserving their original bytes and source locations.
//!
//! Start with [`adif::adi::Parser`] and [`adif::ParseOpts`]. Parsing yields complete
//! headers and records, with terminal structural errors separate from cached
//! text-decoding errors.

/// ADIF parsing options, parsed fields, and reference data types.
pub mod adif;
