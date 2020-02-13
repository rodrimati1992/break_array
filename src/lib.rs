//! Want indexing arrays to not work anymore?
//! 
//! Simply do this:
//! 
//! ```compile_fail
//! use break_array;
//! let array=[0,1,2,3];
//! assert_eq!( array[0], 0 );
//! ```
//! and marvel at the compiler error:
//! ```text
//!   |
//! 6 | assert_eq!( array[0], 0 );
//!   |                   ^ expected struct `break_array::MyType`, found integer
//! 
//! error: aborting due to previous error
//! 
//! For more information about this error, try `rustc --explain E0308`.
//! ```
//! 

#![no_std]

use core::ops::{Index,IndexMut};

struct MyType;

macro_rules! array_impls {
    ( $($size:expr),* ) => (
        $(
            impl<T> Index<MyType> for [T;$size]{
                type Output=T;
                fn index(&self,_:MyType)->&T{
                    &coerce_slice(self)[0]
                }
            }

            impl<T> IndexMut<MyType> for [T;$size]{
                fn index_mut(&mut self,_:MyType)->&mut T{
                    &mut coerce_slice_mut(self)[0]
                }
            }
        )*
    )
}


array_impls!{
    0,1,2,3,4,5,6,7,8,9,
    10,11,12,13,14,15,16,17,18,19,
    20,21,22,23,24,25,26,27,28,29,
    30,31,32
}


/// Used to coerce `&[T;N]` to `&[T]`.
#[inline(always)]
const fn coerce_slice<'a, T>(slic: &'a [T]) -> &'a [T] {
    slic
}

/// Used to coerce `&mut [T;N]` to `&mut [T]`.
#[inline(always)]
fn coerce_slice_mut<'a, T>(slic: &'a mut [T]) -> &'a mut [T] {
    slic
}


#[cfg(test)]
mod tests{
    use crate::MyType;

    #[test]
    fn can_index_my_type(){
        assert_eq!(&[3,4,5][MyType], &3);
        assert_eq!(&mut [6,7,8][MyType], &mut 6);
    }
}

