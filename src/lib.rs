//! Procedural macros for closures with shorthand argument names (like in Swift)
//!
//! # Examples
//!
//! Without argumnts (not useful):
//!
//! ```
//! # #![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
//! #
//! # use lambda::l;
//! #
//! assert_eq!(Some("foo").ok_or_else(l!(0)), Ok("foo"));
//! assert_eq!(Option::<&str>::None.ok_or_else(l!(0)), Err(0));
//! ```
//!
//! With one explicit argumnt:
//!
//! ```
//! # #![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
//! #
//! # use lambda::l;
//! #
//! assert_eq!(None.filter(l!($0 % 2 == 0)), None);
//! assert_eq!(Some(3).filter(l!($0 % 2 == 0)), None);
//! assert_eq!(Some(4).filter(l!($0 % 2 == 0)), Some(4));
//! ```
//!
//! With two explicit argumnts:
//!
//! ```
//! # #![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
//! #
//! # use lambda::l;
//! #
//! assert_eq!([1, 2, 3].iter().fold(0, l!($0 + $1)), 6);
//! ```
//!
//! With one explicit, one implicit argumnts:
//!
//! ```
//! # #![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]
//! #
//! # use lambda::l;
//! #
//! assert_eq!([1, 2, 3].iter().fold(0, l!($1 + 1)), 4);
//! ```

/// Lambda macro
#[cfg_attr(not(feature = "nightly"), proc_macro_hack)]
pub use lambda_macro::lambda as l;

#[cfg(not(feature = "nightly"))]
use proc_macro_hack::proc_macro_hack;
