#![doc(html_root_url = "https://docs.rs/slice-of-array/0.1.0")]

//! Traits for viewing a flat slice as a slice of arrays or vice versa.
//!
//! The fundamental tools provided are:
//!
//!  * **`nest`**: convert `&[T]` to `&[[T; n]]`
//!  * **`flat`**: convert `&[[T; n]]` to `&[T]`
//!  * **`as_array`**: convert `&[T]` to `&[T; n]` (the reverse is
//!    already provided by a coercion)
//!
//! Altogether, these let you swap between arbitrary representations
//! of contiguous, `T`-aligned streams of `T` data.  For instance,
//! to view a `[[i32; 6]; 5]` as a `&[[[i32; 3]; 2]; 5]`,
//! one could write `x.flat().flat().nest().nest().as_array()`.
//! Type inference generally works quite well, and as long as the
//! final shape is unambiguous there is no need to annotate types
//! in the middle of the method chain.
//!
//! In cases where type inference is unable to determine the target
//! array size, one can use a turbofish: e.g .`x.nest::<[_; 3]>()`.
//!
//! ```
//! use ::slice_of_array::prelude::*;
//! let vec = vec![[2i32, 2, 2], [7, 7, 7], [4, 4, 4], [1, 1, 1]];
//! assert_eq!(vec.flat(), &[2, 2, 2, 7, 7, 7, 4, 4, 4, 1, 1, 1]);
//!
//! // note: this requires an annotation only due to polymorphism in PartialEq
//! let slc = vec.nest::<[_; 2]>();
//! assert_eq!(slc, &[[[2i32, 2, 2], [7, 7, 7]], [[ 4, 4, 4], [1, 1, 1]]]);
//! ```
//!
//! `nest` and `as_array` panic on failure rather than returning options.
//! The rationale is that it is believed that these these conversions are
//! seldom needed on arbitrary user data which may be the wrong size; rather,
//! they are most likely used when bridging the gap between APIs that work
//! with flattened slices and APIs that work with slices of arrays.
//!
//! Zero-cost conversions in owned data (e.g. between `Vec<T>`
//! and `Vec<[T;n]>`) are not provided, and are probably impossible
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

#[cfg(test)]
#[macro_use]
extern crate version_sync;

pub mod prelude {
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
/// Unsafe because unsafe code relies on a number of properties of
/// arrays for any type that implements this trait.
pub unsafe trait IsSliceomorphic: Sized {
    type Element;
    fn array_len() -> usize;
}

macro_rules! impl_approved_array {
    ($($n:tt)+) => {$(
        unsafe impl<T> IsSliceomorphic for [T; $n] {
            type Element = T;

            #[inline(always)]
            fn array_len() -> usize { $n }
        }
    )+};
}

impl_approved_array!{
        1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16
    17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
}

// Validate some known assumptions of IsSliceomorphic "at runtime,"
//  in a manner which should get optimized into thin air.
fn validate_some_assumptions<V: IsSliceomorphic>() {
    use ::std::mem::{align_of, size_of};

    assert_eq!(
        align_of::<V::Element>(),
        align_of::<V>());

    assert_eq!(
        V::array_len() * size_of::<V::Element>(),
        size_of::<V>());
}

/// Trait for viewing a slice of arrays as a flat slice, without copying.
pub trait SliceFlatExt<T> {
    /// View `&[[T; n]]` as `&[T]`
    fn flat(&self) -> &[T];
    /// View `&mut [[T; n]]` as `&mut [T]`
    fn flat_mut(&mut self) -> &mut [T];
}

/// Trait for viewing a slice as a slice of arrays, without copying.
pub trait SliceNestExt<T> {
    /// View `&[T]` as `&[[T;n]]` without copying, panicking on bad input lengths.
    fn nest<V: IsSliceomorphic<Element=T>>(&self) -> &[V];
    /// View `&mut [T]` as `&mut [[T;n]]` without copying, panicking on bad input lengths.
    fn nest_mut<V: IsSliceomorphic<Element=T>>(&mut self) -> &mut [V];
}

/// Trait for viewing a slice as an array of known size, without copying.
///
/// The reverse is already provided by a coercion.
pub trait SliceArrayExt<T> {
    /// View `&[T]` as `&[T;n]`, panicking on incorrect length.
    fn as_array<V: IsSliceomorphic<Element=T>>(&self) -> &V;
    /// View `&mut [T]` as `&mut [T;n]`, panicking on incorrect length.
    fn as_mut_array<V: IsSliceomorphic<Element=T>>(&mut self) -> &mut V;
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
                self.len() * V::array_len(),
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
                self.len() * V::array_len(),
            )
        }
    }
}

impl<T> SliceNestExt<T> for [T] {
    fn nest<V: IsSliceomorphic<Element=T>>(&self) -> &[V] {
        validate_some_assumptions::<V>();
        assert_eq!(0, self.len() % V::array_len(),
            "cannot view slice of length {} as &[[_; {}]]",
            self.len(), V::array_len());

        // UNSAFETY: (::std::slice::from_raw_parts)
        // - pointer must be non-null (even for zero-length)
        // - pointer must be aligned
        // - pointer must be valid for given size
        // - lifetimes are unchecked
        unsafe { ::std::slice::from_raw_parts(
            self.as_ptr() as *const _,
            self.len() / V::array_len(),
        )}
    }

    fn nest_mut<V: IsSliceomorphic<Element=T>>(&mut self) -> &mut [V] {
        validate_some_assumptions::<V>();
        assert_eq!(0, self.len() % V::array_len(),
            "cannot view slice of length {} as &mut [[_; {}]]",
            self.len(), V::array_len());

        // UNSAFETY: (::std::slice::from_raw_parts_mut)
        // - pointer must be non-null (even for zero-length)
        // - pointer must be aligned
        // - pointer must be valid for given size
        // - lifetimes are unchecked
        // - aliasing guarantees of &mut are unchecked
        unsafe { ::std::slice::from_raw_parts_mut(
            self.as_ptr() as *mut _,
            self.len() / V::array_len(),
        )}
    }
}

impl<T> SliceArrayExt<T> for [T] {
    fn as_array<V: IsSliceomorphic<Element=T>>(&self) -> &V {
        assert_eq!(self.len(), V::array_len(),
            "cannot view slice of length {} as &[_; {}]",
            self.len(), V::array_len());

        &self.nest()[0]
    }

    fn as_mut_array<V: IsSliceomorphic<Element=T>>(&mut self) -> &mut V {
        assert_eq!(self.len(), V::array_len(),
            "cannot view slice of length {} as &mut [_; {}]",
            self.len(), V::array_len());

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
            assert_markdown_deps_updated!("README.md");
        }

        #[test]
        fn test_html_root_url() {
            assert_html_root_url_updated!("lib.rs");
        }
    }
}
