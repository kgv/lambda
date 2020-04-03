#![cfg_attr(feature = "nightly", feature(proc_macro_hygiene))]

use lambda::l;

#[test]
fn without_args() {
    assert_eq!(Some("foo").ok_or_else(l!(0)), Ok("foo"));
    assert_eq!(Option::<&str>::None.ok_or_else(l!(0)), Err(0));
}

#[test]
fn with_one_explicit_argumnt() {
    assert_eq!(None.filter(l!($0 % 2 == 0)), None);
    assert_eq!(Some(3).filter(l!($0 % 2 == 0)), None);
    assert_eq!(Some(4).filter(l!($0 % 2 == 0)), Some(4));
}

#[test]
fn two_explicit_argumnts() {
    assert_eq!([1, 2, 3].iter().fold(0, l!($0 + $1)), 6);
}

#[test]
fn one_explicit_one_implicit_argumnts() {
    assert_eq!([1, 2, 3].iter().fold(0, l!($1 + 1)), 4);
}
