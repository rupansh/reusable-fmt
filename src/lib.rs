// Copyright (c) 2021 - devs of `reusable-fmt`
// SPDX-License-Identifier: WTFPL

#![cfg_attr(nightly, feature(extended_key_value_attributes))]
//! This crate provides compile-time defined format strings support for
//! [`std::fmt`] macros.
//!
//! # Example
//!
//! ```
//! use reusable_fmt:*;
//! 
//! // This defines your format strings
//! fmt_reuse! {
//!     TEST1 = "This is a test! {}";
//!     TEST2 = "You can pass multiple format args! {} {}";
//!     TEST3 = r#"Raw Strings work too!! {}"#;
//!     TEST4 = "Named args {arg}";
//!     TEST5 = "Positional args {1} {0}";
//!     TEST6 = "Mixed {} {2} {1} {arg}";
//! }
//! 
//! fn main() {
//! 	prntln!(TEST, "Hello World"); // This is a test! Hello World
//! 	let test = fmt!(TEST6, "Hello", "Test", "World", arg="Named"); // Mixed Hello World Test Named
//! 	prntln!("{}", "WOW This works too!");
//! }
//! ```
//!
//! # Syntax
//!
//! Format strings are explicitly defined using the [`fmt_reuse!`] macro first
//!
//! All format related macros follow a syntax similar to [`std::fmt`] macros (and even support the
//! same syntax as them!)
//!
//! The general syntax is:
//!
//! `fmt!(<KEY>, <ARGUMENTS>)`
//! 
//! `wrt!(<BUFFER>, <KEY>, <ARGUMENTS>)`


#![cfg_attr(nightly, feature(extended_key_value_attributes))]
/// Creates a reusable format identifier
///
/// In the `KEY = VALUE;` pair,
///
/// `KEY` must be a valid identifier
///
/// `VALUE` must be a string/raw string literal ⚠️[^w] which is valid for usage in [`std::fmt`] macro
///
/// ## Example
///
/// ```
/// # use reusable_fmt::fmt_reuse;
/// fmt_reuse! {
///     TEST1 = "This is a test! {}";
///     TEST2 = "You can pass multiple format args! {} {}";
///     TEST3 = r#"Raw Strings work too!! {}"#;
///     TEST4 = "Named args {arg}";
///     TEST5 = "Positional args {1} {0}";
///     TEST6 = "Mixed {} {2} {1} {arg}";
/// }
/// ```
///
/// [^w]: `VALUE` is not explicitly type checked, however only string literals will work with
/// our wrapper macros.
#[macro_export]
macro_rules! fmt_reuse {
    ($($key:ident = $value:literal;)*) => {
        $(
            #[doc(hidden)]
            #[macro_export]
            macro_rules! $key {
                () => {
                    $value
                }
            }
        )*
    }
}

/// See: https://github.com/rust-lang/rust/issues/35853
macro_rules! mac_workaround {
    ($($body:tt)*) => {
        macro_rules! __mac_wc { $($body)* }
        __mac_wc!($);
    }
}

#[cfg(nightly)]
macro_rules! fmt_usage {
    ($name:expr) => { concat!("\t", concat!(stringify!($name), "!(TEST, \"World\"); // Hello World")) }
}

#[cfg(nightly)]
macro_rules! write_usage {
    ($name:expr) => { concat!("\tlet mut s = String::new();\n\t", concat!(stringify!($name), "!(&mut s, TEST, \"World\"); // Hello World")) }
}

/// Automatically generates wrapper macros for [`std::fmt`] macros
macro_rules! fmt_macro_gen {
    ($name:ident, $mac:ident) => {
        mac_workaround!{
        ($d:tt) => {
        #[cfg_attr(nightly, cfg_attr(nightly, doc = concat!("Wrapper around [`", concat!(stringify!($mac), "!`]"))))]
        ///
        /// This wrapper is auto generated
        ///
        /// ## Example
        ///
        /// ```
        /// # use reusable_fmt::*;
        /// fmt_reuse! {
        ///     TEST = "Hello {}";
        /// }
        ///
        /// fn test() {
        #[cfg_attr(nightly, cfg_attr(nightly, doc = fmt_usage!($name)))]
        /// }
        /// ```
        #[macro_export]
        macro_rules! $name {
            ($key:ident, $d($dargs:tt)*) => {
                $mac!($key!(), $d($dargs)*)
            };
            ($d($dargs:tt)*) => {
                $mac!($d($dargs)*)
            };
        }
        }
        }
    }
}

/// Automatically generates wrapper macros for [`std::fmt`] write related macros
macro_rules! write_macro_gen {
    ($name:ident, $mac:ident) => {
        mac_workaround!{
        ($d:tt) => {
        #[cfg_attr(nightly, cfg_attr(nightly, doc = concat!("Wrapper around [`", concat!(stringify!($mac), "!`]"))))]
        ///
        /// This wrapper is auto generated
        ///
        /// ## Example
        ///
        /// ```
        /// # use reusable_fmt::*;
        /// # use std::fmt::Write;
        /// fmt_reuse! {
        ///     TEST = "Hello {}";
        /// }
        ///
        /// fn test() {
        #[cfg_attr(nightly, cfg_attr(nightly, doc = write_usage!($name)))]
        /// }
        /// ```
        #[macro_export]
        macro_rules! $name {
            ($dst:expr, $key:ident, $d($dargs:tt)*) => {
                $mac!($dst, $key!(), $d($dargs)*)
            };
            ($dst:expr, $d($dargs:tt)*) => {
                $mac!($dst, $d($dargs)*)
            };
        }
        }
        }
    }
}

fmt_macro_gen!(fmt, format);
fmt_macro_gen!(prnt, print);
fmt_macro_gen!(prntln, println);
fmt_macro_gen!(eprnt, eprintln);
fmt_macro_gen!(eprntln, eprintln);

write_macro_gen!(wrt, write);
write_macro_gen!(wrtln, writeln);
