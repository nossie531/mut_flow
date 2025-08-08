//! Provider of [`MutAlt`].

use crate::prelude::*;
use core::marker::PhantomData;

/// Mutable reference alternate.
#[derive(Default)]
pub struct MutAlt<'a, T: 'a> {
    /// Address to mutation source.
    addr: usize,
    /// Target value for mutation.
    value: Option<T>,
    /// Dummy.
    pd: PhantomData<&'a ()>,
}

impl<'a, T: 'a> MutAlt<'a, T> {
    /// Creates a new value.
    pub(crate) fn new<S>(src: &MutSrc<'a, S>) -> Self
    where
        S: 'a + ?Sized,
    {
        Self {
            addr: src.addr(),
            value: None,
            pd: PhantomData,
        }
    }

    /// Returns enabled state.
    pub fn is_enabled(&self) -> bool {
        self.value.is_some()
    }

    /// Returns `true` if this is created by `src`.
    pub fn is_from<S>(&self, src: &MutSrc<'a, S>) -> bool
    where
        S: 'a + ?Sized,
    {
        self.addr == src.addr()
    }

    /// Returns shared reference to mutation target value.
    ///
    /// # Panics
    ///
    /// Panics if this is not enabled.
    pub fn value(&self) -> &T {
        assert!(self.is_enabled());
        self.value.as_ref().unwrap()
    }

    /// Returns mutable reference to mutation target value.
    ///
    /// # Panics
    ///
    /// Panics if this is not enabled.
    pub fn value_mut(&mut self) -> &mut T {
        assert!(self.is_enabled());
        self.value.as_mut().unwrap()
    }

    /// Switch this object to the mutation handler.
    ///
    /// # Panics
    ///
    /// Panics if any of following occurs.
    ///
    /// - `src` is not enabled.
    /// - `self` and `src` has different source.
    pub fn switch<S, F>(&mut self, src: &mut MutSrc<'a, S>, f: F)
    where
        S: 'a + ?Sized,
        F: Fn(&'a mut S) -> T,
    {
        assert!(src.is_enabled());
        assert!(self.is_from(src));
        self.value = Some(f(unsafe { &mut *src.ptr() }));
        src.set_enabled(false);
    }

    /// Sets enabled state to `false`.
    pub(crate) fn set_disabled(&mut self) {
        self.value = None;
    }
}
