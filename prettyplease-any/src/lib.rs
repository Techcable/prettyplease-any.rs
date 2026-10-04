//! Pretty print individual items and expressions, using the [`prettyprint` crate] undearneath.
//!
//! Necessary because prettyprint only provides functionality to print an entire [`syn::File`].
//! The [`prettyprint::unparse(&File)`](prettyplease::unparse) function is the only item in the whole `prettyplease` crate.
//!
//! This crate works around this issue and offers a [`PrettyPlease`] trait for most syn types.
//!
//! The implementation makes some assumptions about the formatted code,
//! so it could break if prettyplease makes major changes to its output.
//!
//!
//! [`prettyprint` crate]: https://github.com/dtolnay/prettyplease
#![cfg_attr(feature = "_internal_nightly_test", feature(non_exhaustive_omitted_patterns_lint))]

mod basics;
mod expr;
mod internal;
mod item;
mod stmt;
mod types;

/// Pretty print an item or expression using the [`prettyplease`] crate.
pub trait PrettyPlease: internal::Sealed {
    /// Pretty print this syntax tree node.
    fn pretty_print(&self) -> String;
    /// Pretty print this syntax tree node, consuming ownership of the value.
    fn pretty_print_owned(self) -> String
    where
        Self: Sized,
    {
        self.pretty_print()
    }
}
