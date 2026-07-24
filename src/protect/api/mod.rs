pub mod cameras;
pub mod chimes;
pub mod lights;
pub mod meta;
pub mod nvr;
pub mod sensors;
pub mod viewers;

use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

// Encode everything except RFC 3986 unreserved characters, so an ID cannot
// smuggle path separators or query/fragment delimiters into the request URL.
const PATH_SEGMENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

/// Percent-encode a resource ID for use as a single URL path segment.
pub(crate) fn encode_id(id: &str) -> String {
    utf8_percent_encode(id, PATH_SEGMENT).to_string()
}
