use proc_macro::TokenStream;

/// Lambda
///
/// # Examples
///
/// Without argumnts (not useful):
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// #
/// # use lambda::l;
/// #
/// assert_eq!(Some("foo").ok_or_else(l!(0)), Ok("foo"));
/// assert_eq!(Option::<&str>::None.ok_or_else(l!(0)), Err(0));
/// ```
///
/// With one explicit argumnt:
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// #
/// # use lambda::l;
/// #
/// assert_eq!(None.filter(l!($0 % 2 == 0)), None);
/// assert_eq!(Some(3).filter(l!($0 % 2 == 0)), None);
/// assert_eq!(Some(4).filter(l!($0 % 2 == 0)), Some(4));
/// ```
///
/// With two explicit argumnts:
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// #
/// # use lambda::l;
/// #
/// assert_eq!([1, 2, 3].iter().fold(0, l!($0 + $1)), 6);
/// ```
///
/// With one explicit, one implicit argumnts:
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// #
/// # use lambda::l;
/// #
/// assert_eq!([1, 2, 3].iter().fold(0, l!($1 + 1)), 4);
/// ```
#[proc_macro]
pub fn lambda(input: TokenStream) -> TokenStream {
    lambda::proc_macro(input)
}

/// See lambda
#[proc_macro]
pub fn l(input: TokenStream) -> TokenStream {
    lambda(input)
}

mod lambda;
