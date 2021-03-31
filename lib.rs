#![doc(html_root_url = "https://docs.rs/slice-of-array/0.2.1")]

//! Extension traits for viewing a slice as a slice of arrays or vice versa.
//!
//! Provides the following methods on `[T]`:
//!
//!  * **[`nest`]**: `&[T] -> &[[T; n]]`
//!  * **[`flat`]**: `&[[T; n]] -> &[T]`
//!  * **[`as_array`]**: `&[T] -> &[T; n]` (the reverse is
//!    already provided by a coercion)
//!  * **`nest_mut`, `flat_mut`, `as_mut_array`** for `&mut [_]`.
//!
//! Altogether, these let you swap between arbitrary representations
//! of contiguous, `T`-aligned streams of `T` data.  For instance,
//! to view a `[[i32; 6]; 5]` as a `&[[[i32; 3]; 2]; 5]`,
//! one could write
//!
//! ```
//! # // FIXME: Dumb/confusing example. I actually wrote it wrong
//! # //        the first time, calling `flat()` twice because it
//! # //        didn't occur to me that the outer '; 5' is already
//! # //        automatically eliminated by coercion.
//! # //
//! # //        Almost makes a case for providing `.as_slice()`
//! # //        as an explicit form of this coercion.
//! #
//! # use slice_of_array::prelude::*;
//! # let _ = || {
//! #     let x: [[i32; 6]; 5] = unimplemented!();
//! #     let _: &[[[i32; 3]; 2]; 5] =
//! x.flat().nest().nest().as_array()
//! #     ;
//! # };
//! ```
//!
//! Type inference generally works quite well, and as long as the
//! final shape is unambiguous there is no need to annotate types
//! in the middle of the method chain.
//!
//! In cases where type inference is unable to determine the target
//! array size, one can use a turbofish: e.g .`x.nest::<[_; 3]>()`.
//!
//! ```
//! use ::slice_of_array::prelude::*;
//!
//! let vec = vec![[2i32, 2, 2], [7, 7, 7], [4, 4, 4], [1, 1, 1]];
//! assert_eq!(vec.flat(), &[2, 2, 2, 7, 7, 7, 4, 4, 4, 1, 1, 1]);
//!
//! // note: this requires an annotation only due to polymorphism in PartialEq
//! let slc = vec.nest::<[_; 2]>();
//! assert_eq!(slc, &[[[2i32, 2, 2], [7, 7, 7]], [[4, 4, 4], [1, 1, 1]]]);
//! ```
//!
//! [`nest`] and [`as_array`] panic on failure rather than returning options.
//! The rationale is that it is believed that these these conversions are
//! seldom needed on arbitrary user data which may be the wrong size; rather,
//! they are most likely used when bridging the gap between APIs that work
//! with flattened slices and APIs that work with slices of arrays.
//!
//! Zero-cost conversions in owned data (e.g. between `Vec<T>`
//! and `Vec<[T; n]>`) are not provided, and are probably impossible
//! in consideration of e.g. custom allocators. If you need to
//! convert between such types, you can use these traits in tandem
//! with `<[T]>::to_vec` to perform a copy:
//!
//! ```
//! # use ::slice_of_array::prelude::*;
//! let vec = vec![[2i32, 2, 2], [7, 7, 7]];
//!
//! // copying into a Vec<i32>
//! let flattened = vec.flat().to_vec();
//! assert_eq!(flattened, vec![2i32, 2, 2, 7, 7, 7]);
//! ```
//!
//! [`nest`]: [`SliceNestExt::nest`]
//! [`flat`]: [`SliceFlatExt::flat`]
//! [`as_array`]: [`SliceArrayExt::as_array`]

pub mod prelude {
    //! This module contains extension traits from `slice_of_array`.
    //!
    //! It is meant to be glob imported, by users who may find it obnoxious to remember
    //! the precise names of the traits that each method belongs to.
    //!
    //! ```rust
    //! use slice_of_array::prelude::*;
    //! ```
    //!
    //! `slice_of_array` follows an opinionated policy on what preludes should and should
    //! not contain. This prelude will never contain anything that the user will likely
    //! want to refer to by name.

    pub use super::SliceFlatExt;
    pub use super::SliceNestExt;
    pub use super::SliceArrayExt;
}

/// Marker trait used in bounds of `Slice{Flat,Nest,Array}Ext`.
///
/// This marks the array types approved for use with `slice_of_array`.
///
/// It is deliberately not implemented for arrays of size 0,
/// because said traits are otherwise perfect isomorphisms for
/// the inputs that they don't fail on;
/// Having `.flat().nest()` turn a `&[[i32; 0]]` of length 18
/// into a `&[[i32; 0]]` of length 0 gives me the heebie jeebies.
///
/// # Safety
///
/// For any implementation, `Self` must have the same size and
/// alignment as `[Self::Element; Self::LEN]`.  Furthermore, you
/// must be comfortable with the possibility of `[Self]` being
/// reinterpreted bitwise as `[[Self::Element; Self::LEN]]` (or
/// vice versa) in any possible context.
///
/// # Notice
///
/// **Please do NOT use this trait in public interfaces in your code.**
///
/// `slice_of_array` is not yet 1.0, is not ready (or even designed)
/// to be used as a public dependency.
///
/// However, feel free to implement this trait on your own private
/// wrapper types around arrays and/or `#[repr(C)]` structs. (these use
/// cases are explicitly supported because the author does it himself,
/// and quite frankly, it's pretty convenient!)
pub unsafe trait IsSliceomorphic: Sized {
    type Element;
    const LEN: usize;
}

macro_rules! impl_approved_array {
    ($($n:tt)+) => {$(
        unsafe impl<T> IsSliceomorphic for [T; $n] {
            type Element = T;
            const LEN: usize = $n;
        }
    )+};
}

impl_approved_array!{
     1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16  17  18  19  20
    21  22  23  24  25  26  27  28  29  30  31  32  43  44  45  46  47  48  49  50
    51  52  53  54  55  56  57  58  59  60  61  62  63  64  65  66  67  68  69  70
    71  72  73  74  75  76  77  78  79  80  81  82  83  84  85  86  87  88  89  90
    91  92  93  94  95  96  97  98  99 100 101 102 103 104 105 106 107 108 109 110
   111 112 113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128
   256
   512
  1024
  2048
  4096
  8192
  1000
 10000
}

// Validate some known assumptions of IsSliceomorphic "at runtime,"
//  in a manner which should get optimized into thin air.
fn validate_some_assumptions<V: IsSliceomorphic>() {
    use ::std::mem::{align_of, size_of};

    assert_eq!(
        align_of::<V::Element>(),
        align_of::<V>(),
    );

    assert_eq!(
        V::LEN * size_of::<V::Element>(),
        size_of::<V>(),
    );
}

/// Permits viewing a slice of arrays as a flat slice.
///
/// # Implementors
///
/// The methods are available on `&[[T; n]]` and `&mut [[T; n]]`
/// for all `T`, and `1 <= n <= 128` (and a couple other sizes).
/// Of course, they are also available on `Vec<[T; n]>` and any
/// other type that derefs or unsizes to `[[T; n]]`.
///
/// # Notice
///
/// The existence of this trait is an implementation detail.
///
/// **Please do NOT use this trait as a generic bound in your code.**
pub trait SliceFlatExt<T> {
    /// View `&[[T; n]]` as `&[T]`.
    fn flat(&self) -> &[T];

    /// View `&mut [[T; n]]` as `&mut [T]`
    fn flat_mut(&mut self) -> &mut [T];
}

/// Permits viewing a slice as a slice of arrays.
///
/// The new array dimension can often be inferred.
/// When it is not, a turbofish can be used: `.nest::<[_; 3]>()`.
///
/// # Panics
///
/// All methods panic if the input length is not divisible by `n`.
///
/// # Implementors
///
/// The methods are available on `&[T]` and `&mut [T]` for all `T`.
/// Of course, they are also available on `Vec<T>` and any other type
/// that derefs or unsizes to `[T]`.
///
/// # Notice
///
/// The existence of this trait is an implementation detail.
///
/// **Please do NOT use this trait as a generic bound in your code.**
pub trait SliceNestExt<T> {
    /// View `&[T]` as `&[[T; n]]` without copying.
    fn nest<V: IsSliceomorphic<Element=T>>(&self) -> &[V];

    /// View `&mut [T]` as `&mut [[T; n]]` without copying.
    fn nest_mut<V: IsSliceomorphic<Element=T>>(&mut self) -> &mut [V];
}

/// Permits viewing a slice as an array.
///
/// The output array length can often be inferred.
/// When it is not, a turbofish can be used: `.as_array::<[_; 3]>()`.
///
/// # Panics
///
/// All methods panic if the slice is not exactly the requested length.
///
/// # Implementors
///
/// The methods are available on `&[T]` and `&mut [T]` for all `T`.
/// Of course, they are also available on `Vec<T>` and any other type
/// that derefs or unsizes to `[T]`.
///
/// # Notice
///
/// The existence of this trait is an implementation detail.
///
/// **Please do NOT use this trait as a generic bound in your code.**
pub trait SliceArrayExt<T> {
    /// View `&[T]` as `&[T; n]`.
    fn as_array<V: IsSliceomorphic<Element=T>>(&self) -> &V;

    /// View `&mut [T]` as `&mut [T; n]`.
    fn as_mut_array<V: IsSliceomorphic<Element=T>>(&mut self) -> &mut V;

    /// Clone `&[T]` to `[T; n]`.
    ///
    /// This is provided because `.as_array().clone()` tends to cause trouble for
    /// type inference.
    fn to_array<V: IsSliceomorphic<Element=T>>(&self) -> V where V: Clone
    { self.as_array::<V>().clone() }
}

impl<V: IsSliceomorphic> SliceFlatExt<V::Element> for [V] {
    fn flat(&self) -> &[V::Element] {
        // UNSAFETY: (::std::slice::from_raw_parts)
        // - pointer must be non-null (even for zero-length)
        // - pointer must be aligned
        // - pointer must be valid for given size
        // - lifetimes are unchecked
        unsafe {
            validate_some_assumptions::<V>();
            ::std::slice::from_raw_parts(
                self.as_ptr() as *const _,
                self.len() * V::LEN,
            )
        }
    }

    fn flat_mut(&mut self) -> &mut [V::Element] {
        // UNSAFETY: (::std::slice::from_raw_parts_mut)
        // - pointer must be non-null (even for zero-length)
        // - pointer must be aligned
        // - pointer must be valid for given size
        // - lifetimes are unchecked
        // - aliasing guarantees of &mut are unchecked
        unsafe {
            validate_some_assumptions::<V>();
            ::std::slice::from_raw_parts_mut(
                self.as_mut_ptr() as *mut _,
                self.len() * V::LEN,
            )
        }
    }
}

impl<T> SliceNestExt<T> for [T] {
    fn nest<V: IsSliceomorphic<Element=T>>(&self) -> &[V] {
        validate_some_assumptions::<V>();
        assert_eq!(
            0, self.len() % V::LEN,
            "cannot view slice of length {} as &[[_; {}]]",
            self.len(), V::LEN,
        );

        // UNSAFETY: (::std::slice::from_raw_parts)
        // - pointer must be non-null (even for zero-length)
        // - pointer must be aligned
        // - pointer must be valid for given size
        // - lifetimes are unchecked
        unsafe { ::std::slice::from_raw_parts(
            self.as_ptr() as *const _,
            self.len() / V::LEN,
        )}
    }

    fn nest_mut<V: IsSliceomorphic<Element=T>>(&mut self) -> &mut [V] {
        validate_some_assumptions::<V>();
        assert_eq!(
            0, self.len() % V::LEN,
            "cannot view slice of length {} as &mut [[_; {}]]",
            self.len(), V::LEN,
        );

        // UNSAFETY: (::std::slice::from_raw_parts_mut)
        // - pointer must be non-null (even for zero-length)
        // - pointer must be aligned
        // - pointer must be valid for given size
        // - lifetimes are unchecked
        // - aliasing guarantees of &mut are unchecked
        unsafe { ::std::slice::from_raw_parts_mut(
            self.as_ptr() as *mut _,
            self.len() / V::LEN,
        )}
    }
}

impl<T> SliceArrayExt<T> for [T] {
    fn as_array<V: IsSliceomorphic<Element=T>>(&self) -> &V {
        assert_eq!(
            self.len(), V::LEN,
            "cannot view slice of length {} as &[_; {}]",
            self.len(), V::LEN,
        );

        &self.nest()[0]
    }

    fn as_mut_array<V: IsSliceomorphic<Element=T>>(&mut self) -> &mut V {
        assert_eq!(
            self.len(), V::LEN,
            "cannot view slice of length {} as &mut [_; {}]",
            self.len(), V::LEN,
        );

        &mut self.nest_mut()[0]
    }
}

#[cfg(test)]
mod tests {
    pub use super::prelude::*;

    #[test]
    fn inference_lattice() {
        // Checks that chaining nest().nest() or nest().as_array()
        // can be done without explicit annotations on the first method call.
        let mut v = vec![(); 9];

        { let _: &[[(); 3]; 3] = v.nest().as_array(); }
        { let _: &[[[(); 3]; 3]] = v.nest().nest(); }
        { let _: &mut [[(); 3]; 3] = v.nest_mut().as_mut_array(); }
        { let _: &mut [[[(); 3]; 3]] = v.nest_mut().nest_mut(); }
        { let _: [[(); 3]; 3] = v.nest().to_array(); }
        { let _: Vec<[(); 3]> = v.nest().to_vec(); }
    }

    mod failures {
        use super::super::*;

        #[test]
        #[should_panic(expected = "cannot view slice of length 8")]
        fn fail_nest_not_multiple() {
            let v = vec![(); 8];
            let _: &[[(); 3]] = v.nest();
        }

        #[test]
        #[should_panic(expected = "cannot view slice of length 8")]
        fn nest_mut_not_multiple() {
            let mut v = vec![(); 8];
            let _: &mut [[(); 3]] = v.nest_mut();
        }

        // bad array size tests;
        //  we try converting slices of length 1 or 6 into a length 3 array.
        //  These sizes were chosen to catch accidental acceptance in
        //    the case of sizes that divide evenly
        #[test]
        #[should_panic(expected = "cannot view slice of length 1")]
        fn as_array_too_small() {
            let v = vec![(); 1];
            let _: &[(); 3] = v.as_array();
        }

        #[test]
        #[should_panic(expected = "cannot view slice of length 6")]
        fn as_array_too_large() {
            let v = vec![(); 6];
            let _: &[(); 3] = v.as_array();
        }

        #[test]
        #[should_panic(expected = "cannot view slice of length 1")]
        fn as_mut_array_too_small() {
            let mut v = vec![(); 1];
            let _: &mut [(); 3] = v.as_mut_array();
        }

        #[test]
        #[should_panic(expected = "cannot view slice of length 6")]
        fn as_mut_array_too_large() {
            let mut v = vec![(); 6];
            let _: &mut [(); 3] = v.as_mut_array();
        }
    }

    mod dox {
        #[test]
        fn test_readme_version() {
            version_sync::assert_markdown_deps_updated!("README.md");
        }

        #[test]
        fn test_html_root_url() {
            version_sync::assert_html_root_url_updated!("lib.rs");
        }
    }
}
