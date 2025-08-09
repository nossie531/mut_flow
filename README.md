# mut_flow

Mutable reference wrapper for avoiding aliasing.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## What is this?

This crate provides mutable reference wrapper for avoiding aliasing.

## Background

As an important principle of Rust, mutable reference aliasings are not allowed
by borrow checker. This restriction eliminates edits from multiple locations. 
This is the fundamental for various kinds of safety and optimization.

However, aliasing rules sometimes lead us into dead end,
even if we are not trying to make dangerous edits.
We just want to switch editing methods in order.

The problem occurs as follows...

1. There is a mutable reference _A_.
0. From _A_, get value _B_ containing _A_.
0. _B_ must has a state that requires updating, So it must be saved.
0. Then, _A_ must be dropped and cannot be used in the future.

## Main items

- `MutSrc` - Mutable reference wrapper.
- `MutAlt` - Mutable reference alternate.

## Examples

In this example, the mutable slice passed to `Array`
and the edit area of `ArrCursor` have aliasing.

```rust
use mut_flow::prelude::*;

let arr = &mut [0; 6];
let arr = &mut Array::new(arr);
arr.open_cursor();
arr.set_by_cursor(1, 1);
arr.set_by_cursor(2, 2);
arr.close_cursor();
arr.as_slice_mut()[5] = 3;
assert_eq!(arr.src.value(), [0, 1, 0, 2, 0, 3]);

struct Array<'a> {
    src: MutSrc<'a, [i32]>,
    cur: MutAlt<'a, ArrCursor<'a>>
}

impl<'a> Array<'a> {
    pub fn new(src: &'a mut [i32]) -> Self {
        let src = MutSrc::new(src);
        let cur = src.alt();
        Self { src, cur }
    }

    pub fn open_cursor(&mut self) {
        self.cur.switch(&mut self.src, |x| ArrCursor::new(x));
    }

    pub fn close_cursor(&mut self) {
        self.src.switch(&mut self.cur);
    }

    pub fn set_by_cursor(&mut self, n: isize, value: i32) {
        self.cur.value_mut().set_value(n, value);
    }

    pub fn as_slice_mut(&mut self) -> &mut[i32] {
        self.src.value_mut()
    }
}

struct ArrCursor<'a> {
    arr: &'a mut[i32],
    pos: usize,
}

impl<'a> ArrCursor<'a> {
    pub fn new(arr: &'a mut [i32]) -> Self {
        Self { arr, pos: 0 }
    }

    pub fn set_value(&mut self, n: isize, value: i32) {
        self.pos = self.pos.checked_add_signed(n).unwrap();
        self.arr[self.pos] = value;
    }
}
```

## Other options

This crate aims to provide a second-best solution. 

So, If you can fix the API which you are using,
that is probably the best solution.

How to fix? One good guideline can be found at slice. Mutable iterator of
slice can get by [`iter_mut`](std1) method. And the iterator can convert
back to the original slice using [`as_mut_slice`](std2) method (nightly
as of 2025). In this way, in many cases, it works well to provide reverse
conversion for editable objects.

[std1]: https://doc.rust-lang.org/std/slice/struct.IterMut.html#method.as_mut_slice
[std2]: (https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)

## Under the hood

This crate uses `unsafe` internally.

Because we convert mutable references to pointers to avoid aliasing, and
convert them back only when required. This cannot be done without unsafe.

## History

See [CHANGELOG](CHANGELOG.md).
