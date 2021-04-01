# `slice-of-array`

Extension traits on `[T]` for converting to and from `[[T; n]]`.

```toml
[dependencies]
slice-of-array = "0.3"
```

```rust
extern crate slice_of_array;
use ::slice_of_array::prelude::*;

let vec = vec![[1, 0], [0, 1]];
let _: &[i32] = vec.flat();
let _: &[[[i32; 2]; 1]] = vec.nest();
let _: &[[i32; 2]; 2] = vec.as_array();
```

**Use types like `&[[f64; 3]]` in your public interfaces without fear!**

You'll be able to flatten that stuff just fine when delegating work to (insert your favorite linear algebra library here)!

Not to mention everybody using your library will have no problem creating the necessary slice types to use your interface.

...uh, that is, assuming that they also have found this or a similar crate.  And that they aren't internally using something like `(f64, f64, f64)` or some `#[repr(rust)]` struct.  Or a structure of arrays.

Can't win 'em all.

## Q&A

### Oh god not another slice-to-array library

Hey hey hey, hold on here! This is **not** a slice-to-array library. It is a **slice _of_ array** library!  Everybody and their mother has published their own crate for converting between `&[T]` and `&[T; n]` on crates.io, but **only** `slice-of-array`™ lets you convert between `&[T]` and `&[[T; n]]`.

That said... uh, yes, one of its features is indeed casting slices to arrays.  I'm sorry.

### Panics by default?

Panics by default.

It's main purpose is for bridging between APIs that work on flattened vectors and APIs that work on slices of arrays.  In these places where it is intended to be used, you already know that your data is of the appropriate shape, and `Option` would get in the way.

### I want to make a `Vec<_>` instead of `&[_]`

`.flat().to_vec()` or `.nest().to_vec()`

### No no no, I want to make a `Vec<_>` with *zero cost*

Yikes!  You're messing with metadata that's going to be handed to the allocator when your vec falls out of scope, and I don't think the allocator likes surprises.  I don't want that on my conscience!

If you're convinced that this problem has a solution, then please submit a PR.

## [Changelog](CHANGELOG.md)
