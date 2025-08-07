//! Mutable reference wrapper for avoiding aliasing.
//!
//! _The author of this crate is not good at English._  
//! _Forgive me if the document is hard to read._
//!
//! # Examples
//!
//! In this example, the mutable slice passed to `Array`
//! and the edit area of `ArrCursor` have aliasing.
//!
//! ```
//! use mut_flow::prelude::*;
//!
//! let arr = &mut [0; 6];
//! let arr = &mut Array::new(arr);
//! arr.open_cursor();
//! arr.set_by_cursor(1, 1);
//! arr.set_by_cursor(2, 2);
//! arr.close_cursor();
//! arr.as_slice_mut()[5] = 3;
//! assert_eq!(arr.src.value(), [0, 1, 0, 2, 0, 3]);
//!
//! struct Array<'a> {
//!     src: MutSrc<'a, [i32]>,
//!     cur: MutAlt<'a, ArrCursor<'a>>
//! }
//!
//! impl<'a> Array<'a> {
//!     pub fn new(src: &'a mut [i32]) -> Self {
//!         let src = MutSrc::new(src);
//!         let cur = src.alt();
//!         Self { src, cur }
//!     }
//!
//!     pub fn open_cursor(&mut self) {
//!         self.cur.switch(&mut self.src, |x| ArrCursor::new(x));
//!     }
//!
//!     pub fn close_cursor(&mut self) {
//!         self.src.switch(&mut self.cur);
//!     }
//!
//!     pub fn set_by_cursor(&mut self, n: isize, value: i32) {
//!         self.cur.value_mut().set_value(n, value);
//!     }
//!
//!     pub fn as_slice_mut(&mut self) -> &mut[i32] {
//!         self.src.value_mut()
//!     }
//! }
//!
//! struct ArrCursor<'a> {
//!     arr: &'a mut[i32],
//!     pos: usize,
//! }
//!
//! impl<'a> ArrCursor<'a> {
//!     pub fn new(arr: &'a mut [i32]) -> Self {
//!         Self { arr, pos: 0 }
//!     }
//!
//!     pub fn set_value(&mut self, n: isize, value: i32) {
//!         self.pos = self.pos.checked_add_signed(n).unwrap();
//!         self.arr[self.pos] = value;
//!     }
//! }
//! ```

#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

pub mod prelude;

pub use mut_alt::*;
pub use mut_src::*;

mod mut_alt;
mod mut_src;
