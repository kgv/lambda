# Lambda

[![Build Status](https://travis-ci.org/kgv/lambda.svg?branch=master)](https://travis-ci.org/kgv/lambda)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/kgv/format?svg=true)](https://ci.appveyor.com/project/kgv/format)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

Procedural macros for closures with shorthand argument names (like in Swift)

## Usage

Without argumnts (not useful):

```rust
let _ = Some("foo").ok_or_else(l!(0));
// expanded:
let _ = Some("foo").ok_or_else(|| 0);
```

With one explicit argumnt:

```rust
let _ = Some(3).filter(l!($0 % 2 == 0));
// expanded:
let _ = Some(3).filter(|_0| _0 % 2 == 0);
```

With two explicit argumnts:

```rust
let _ = [1, 2, 3].iter().fold(0, l!($0 + $1));
// expanded:
let _ = [1, 2, 3].iter().fold(0, |_0, _1| _0 + _1);
```

With one explicit, one implicit argumnts:

```rust
let _ = [1, 2, 3].iter().fold(0, l!($1 + 1));
// expanded:
let _ = [1, 2, 3].iter().fold(0, |_, _1| _1 + 1);
```

etc.

**Limitation**: the last argument must be explicit.
