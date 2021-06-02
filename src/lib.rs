//! Look up the canonical combining class for a character.
//!
//! ### Example
//!
//! ```
//! use unicode_canonical_combining_class::{get_canonical_combining_class, CanonicalCombiningClass};
//!
//! assert_eq!(get_canonical_combining_class('ཱ'), CanonicalCombiningClass::CCC129);
//! ```

mod canonical_combining_class;
mod tables;

pub use canonical_combining_class::get_canonical_combining_class;
pub use tables::CanonicalCombiningClass;

/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-joining-type was generated from.
pub const UNICODE_VERSION: (u64, u64, u64) = (13, 0, 0);

#[cfg(test)]
mod test {
    use super::{get_canonical_combining_class, CanonicalCombiningClass};

    #[test]
    fn test_get_canonical_combining_class() {
        assert_eq!(
            get_canonical_combining_class('\u{05B0}'),
            CanonicalCombiningClass::CCC10
        );
        assert_eq!(
            get_canonical_combining_class('\u{08F0}'),
            CanonicalCombiningClass::CCC27
        );
        assert_eq!(
            get_canonical_combining_class('\u{0670}'),
            CanonicalCombiningClass::CCC35
        );
        assert_eq!(
            get_canonical_combining_class('\u{0E39}'),
            CanonicalCombiningClass::CCC103
        );
        assert_eq!(
            get_canonical_combining_class('\u{0E48}'),
            CanonicalCombiningClass::CCC107
        );
        assert_eq!(
            get_canonical_combining_class('\u{1DCE}'),
            CanonicalCombiningClass::AttachedAbove
        );
        assert_eq!(
            get_canonical_combining_class('\u{0F39}'),
            CanonicalCombiningClass::AttachedAboveRight
        );
        assert_eq!(
            get_canonical_combining_class('\u{0359}'),
            CanonicalCombiningClass::Below
        );
        assert_eq!(
            get_canonical_combining_class('\u{1939}'),
            CanonicalCombiningClass::BelowRight
        );
        assert_eq!(
            get_canonical_combining_class('\u{ABED}'),
            CanonicalCombiningClass::Virama
        );
        assert_eq!(
            get_canonical_combining_class('\u{081A}'),
            CanonicalCombiningClass::NotReordered
        );
        assert_eq!(
            get_canonical_combining_class('\u{1259}'),
            CanonicalCombiningClass::NotReordered
        );
    }
}
