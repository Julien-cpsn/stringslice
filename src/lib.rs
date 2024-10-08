//! A collection of methods to slice strings based on character indices rather than bytes.
//!
//! This crate implements the [`StringSlice`] trait for [`&str`],
//! containing the [`slice`], [`try_slice`], [`substring`], and [`try_substring`] methods.
//!
//! # Examples
//!
//! The [`slice`] method can be used to slice a [`&str`].
//!
//! ```
//! use stringslice::StringSlice;
//!
//! assert_eq!("Ùníc😎de".slice(4..5), "😎");
//! assert_eq!("世界こんにちは".slice(2..), "こんにちは");
//! ```
//!
//! The [`substring`] method is provided for convenience and accepts
//! separate parameters for the start and end of the slice.
//!
//! ```
//! use stringslice::StringSlice;
//!
//! assert_eq!("Γεια σου κόσμε".substring(9, 14), "κόσμε");
//! ```
//!
//! There are also equivalent [`try_slice`] and [`try_substring`] methods
//! which return [`None`] for invalid input.
//!
//! ```
//! use stringslice::StringSlice;
//!
//! assert_eq!("string".try_slice(4..2), None);
//! ```
//!
//! [`StringSlice`]: trait.StringSlice.html
//! [`&str`]: https://doc.rust-lang.org/std/primitive.str.html
//! [`slice`]: trait.StringSlice.html#method.slice
//! [`substring`]: trait.StringSlice.html#method.substring
//! [`try_slice`]: trait.StringSlice.html#method.try_slice
//! [`try_substring`]: trait.StringSlice.html#method.try_substring
//! [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
//!

#![no_std]
#![cfg(not(feature = "std"))]
use core::ops::{Bound, RangeBounds};

#[cfg(feature = "std")]
use std::ops::{Bound, RangeBounds};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "rayon")]
use rayon_::prelude::*;

#[inline]
fn range_to_begin_end(range: impl RangeBounds<usize>) -> (usize, usize) {
    let begin = match range.start_bound() {
        Bound::Included(&b) => b,
        Bound::Excluded(&b) => b + 1,
        Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
        Bound::Included(&b) => b + 1,
        Bound::Excluded(&b) => b,
        // Note: using core::usize::MAX rather than usize::MAX for compatibility with Rust < 1.43
        #[allow(clippy::legacy_numeric_constants)]
        Bound::Unbounded => core::usize::MAX,
    };

    (begin, end)
}

/// Provides the [`slice`], [`try_slice`], [`substring`], and [`try_substring`] methods.
///
/// [`slice`]: trait.StringSlice.html#method.slice
/// [`substring`]: trait.StringSlice.html#method.substring
/// [`try_slice`]: trait.StringSlice.html#method.try_slice
/// [`try_substring`]: trait.StringSlice.html#method.try_substring
pub trait StringSlice {
    /// Returns a string slice for the given range of characters
    ///
    /// This method will panic if the range is invalid,
    /// for example if the beginning is greater than the end.
    ///
    /// # Examples
    /// ```
    /// use stringslice::StringSlice;
    ///
    /// assert_eq!("Ùníc😎de".slice(4..5), "😎");
    /// ```
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;

    /// Returns an [`Option`] containing string slice for the given range of characters
    ///
    /// This method will return [`None`] if the range is invalid,
    /// for example if the beginning is greater than the end.
    ///
    /// # Examples
    /// ```
    /// use stringslice::StringSlice;
    ///
    /// assert_eq!("Ùníc😎de".try_slice(4..5), Some("😎"));
    /// ```
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    fn try_slice(&self, range: impl RangeBounds<usize>) -> Option<&str>;

    /// Returns a string slice between the given beginning and end characters
    ///
    /// This method will panic if the parameters are invalid,
    /// for example if the beginning is greater than the end.
    ///
    /// # Examples
    /// ```
    /// use stringslice::StringSlice;
    ///
    /// assert_eq!("Ùníc😎de".substring(4, 5), "😎");
    /// ```
    fn substring(&self, begin: usize, end: usize) -> &str;

    /// Returns an [`Option`] containing string slice between the given beginning and end characters
    ///
    /// This method will return [`None`] if the parameters are invalid,
    /// for example if the beginning is greater than the end.
    ///
    /// # Examples
    /// ```
    /// use stringslice::StringSlice;
    ///
    /// assert_eq!("Ùníc😎de".try_substring(4, 5), Some("😎"));
    /// ```
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    fn try_substring(&self, begin: usize, end: usize) -> Option<&str>;

    #[cfg(feature = "rayon")]
    /// Returns a string slice for the given range of characters
    ///
    /// This method will panic if the range is invalid,
    /// for example if the beginning is greater than the end.
    ///
    /// Runs in parallel
    ///
    /// # Examples
    /// ```
    /// use stringslice::StringSlice;
    ///
    /// assert_eq!("Ùníc😎de".slice(4..5), "😎");
    /// ```
    fn par_slice(&self, range: impl RangeBounds<usize>) -> &str;

    #[cfg(feature = "rayon")]
    /// Returns an [`Option`] containing string slice for the given range of characters
    ///
    /// This method will return [`None`] if the range is invalid,
    /// for example if the beginning is greater than the end.
    ///
    /// Runs in parallel
    ///
    /// # Examples
    /// ```
    /// use stringslice::StringSlice;
    ///
    /// assert_eq!("Ùníc😎de".try_slice(4..5), Some("😎"));
    /// ```
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    fn par_try_slice(&self, range: impl RangeBounds<usize>) -> Option<&str>;

    #[cfg(feature = "rayon")]
    /// Returns a string slice between the given beginning and end characters
    ///
    /// This method will panic if the parameters are invalid,
    /// for example if the beginning is greater than the end.
    ///
    /// Runs in parallel
    ///
    /// # Examples
    /// ```
    /// use stringslice::StringSlice;
    ///
    /// assert_eq!("Ùníc😎de".substring(4, 5), "😎");
    /// ```
    fn par_substring(&self, begin: usize, end: usize) -> &str;

    #[cfg(feature = "rayon")]
    /// Returns an [`Option`] containing string slice between the given beginning and end characters
    ///
    /// This method will return [`None`] if the parameters are invalid,
    /// for example if the beginning is greater than the end.
    ///
    /// Runs in parallel
    ///
    /// # Examples
    /// ```
    /// use stringslice::StringSlice;
    ///
    /// assert_eq!("Ùníc😎de".try_substring(4, 5), Some("😎"));
    /// ```
    /// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    fn par_try_substring(&self, begin: usize, end: usize) -> Option<&str>;
}

impl StringSlice for str {
    #[inline]
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let (begin, end) = range_to_begin_end(range);
        self.substring(begin, end)
    }

    #[inline]
    fn try_slice(&self, range: impl RangeBounds<usize>) -> Option<&str> {
        let (begin, end) = range_to_begin_end(range);
        self.try_substring(begin, end)
    }

    #[inline]
    fn substring(&self, begin: usize, end: usize) -> &str {
        self.try_substring(begin, end)
            .expect("begin < end when slicing string")
    }

    fn try_substring(&self, begin: usize, end: usize) -> Option<&str> {
        if begin > end {
            None
        } else {
            let mut ch_idx = self.char_indices().map(|(i, _c)| i);

            let len = self.len();
            let begin_ch = ch_idx.nth(begin).unwrap_or(len);
            let end_ch = if end > begin {
                ch_idx.nth(end - begin - 1).unwrap_or(len)
            } else {
                begin_ch
            };

            // Note (unsafe): Since we iterate character indices we can be sure that `begin_ch` and
            // `end_ch` are on UTF-8 boundaries. For performance, we use get_unchecked rather than
            // simply indexing.
            unsafe { Some(self.get_unchecked(begin_ch..end_ch)) }
        }
    }

    #[cfg(feature = "rayon")]
    #[inline]
    fn par_slice(&self, range: impl RangeBounds<usize>) -> &str {
        let (begin, end) = range_to_begin_end(range);
        self.par_substring(begin, end)
    }

    #[cfg(feature = "rayon")]
    #[inline]
    fn par_try_slice(&self, range: impl RangeBounds<usize>) -> Option<&str> {
        let (begin, end) = range_to_begin_end(range);
        self.par_try_substring(begin, end)
    }

    #[cfg(feature = "rayon")]
    #[inline]
    fn par_substring(&self, begin: usize, end: usize) -> &str {
        self.par_try_substring(begin, end)
            .expect("begin < end when slicing string")
    }

    #[cfg(feature = "rayon")]
    fn par_try_substring(&self, begin: usize, end: usize) -> Option<&str> {
        if begin > end {
            None
        } else {
            let mut ch_idx = self.par_char_indices().map(|(i, _c)| i);

            let len = self.len();
            let begin_ch = ch_idx.nth(begin).unwrap_or(len);
            let end_ch = if end > begin {
                ch_idx.nth(end - begin - 1).unwrap_or(len)
            } else {
                begin_ch
            };

            // Note (unsafe): Since we iterate character indices we can be sure that `begin_ch` and
            // `end_ch` are on UTF-8 boundaries. For performance we use get_unchecked rather than
            // simply indexing.
            unsafe { Some(self.get_unchecked(begin_ch..end_ch)) }
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    use core::ops::Bound;

    #[cfg(feature = "std")]
    use std::ops::Bound;

    use super::StringSlice;

    #[test]
    fn test_utf8() {
        let str = "🗻∈🌏";
        assert_eq!("🗻", str.slice(0..1));
        assert_eq!("∈", str.slice(1..2));
        assert_eq!("🌏", str.slice(2..3));
    }

    #[test]
    fn test_zero_len() {
        let str = "test";
        assert_eq!("", str.slice(0..0));
        assert_eq!("", str.slice(..0));
        assert_eq!("", str.slice(1..1));
    }

    #[test]
    #[should_panic]
    fn test_bad_range() {
        "string".slice(4..1);
    }

    #[test]
    fn test_try_bad_range() {
        assert_eq!("string".try_slice(4..1), None);
    }

    #[test]
    fn test_large_range() {
        assert_eq!("test_string".slice(0..500), "test_string");
    }

    #[test]
    fn test_range_types() {
        assert_eq!("test_string".slice(..), "test_string");
        assert_eq!("test_string".slice(5..), "string");
        assert_eq!("test_string".slice(..8), "test_str");
        assert_eq!("test_string".slice(5..8), "str");
        assert_eq!("test_string".slice(5..=7), "str");
        assert_eq!(
            "test_string".slice((Bound::Excluded(4), Bound::Included(7))),
            "str"
        );
    }

    #[cfg(feature = "rayon")]
    #[test]
    fn par_test_utf8() {
        let str = "🗻∈🌏";
        assert_eq!("🗻", str.par_slice(0..1));
        assert_eq!("∈", str.par_slice(1..2));
        assert_eq!("🌏", str.par_slice(2..3));
    }
}
