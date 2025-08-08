//! Provider of [`MutSrc`].

use crate::prelude::*;
use core::marker::PhantomData;

/// Mutable reference wrapper.
#[derive(Default)]
pub struct MutSrc<'a, T>
where
    T: 'a + ?Sized,
{
    /// Enabled state.
    enabled: bool,
    /// Pointer to mutation target.
    ptr: *mut T,
    /// Dummy.
    pd: PhantomData<&'a ()>,
}

impl<'a, T> MutSrc<'a, T>
where
    T: 'a + ?Sized,
{
    /// Creates a new value.
    pub fn new(src: &'a mut T) -> Self {
        Self {
            enabled: true,
            ptr: src,
            pd: PhantomData,
        }
    }

    /// Creates a new alternate.
    pub fn alt<U: 'a>(&self) -> MutAlt<'a, U> {
        MutAlt::new(self)
    }

    /// Returns `true` if this is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns shared reference to mutation target value.
    ///
    /// # Panics
    ///
    /// Panics if this is not enabled.
    pub fn value(&self) -> &T {
        assert!(self.is_enabled());
        unsafe { &*self.ptr }
    }

    /// Returns mutable reference to mutation target value.
    ///
    /// # Panics
    ///
    /// Panics if this is not enabeld.
    pub fn value_mut(&mut self) -> &mut T {
        assert!(self.is_enabled());
        unsafe { &mut *self.ptr }
    }

    /// Switch this object to the mutation handler.
    ///
    /// # Panics
    ///
    /// Panics if any of following occurs.
    ///
    /// - `alt` is not enabled.
    /// - `self` and `alt` has different source.
    pub fn switch<U: 'a>(&mut self, alt: &mut MutAlt<'a, U>) {
        assert!(alt.is_enabled());
        assert!(alt.is_from(self));
        self.set_enabled(true);
        alt.set_disabled();
    }

    /// Returns address to mutation target.
    pub(crate) fn addr(&self) -> usize {
        self.ptr.addr()
    }

    /// Sets enabled state.
    pub(crate) fn set_enabled(&mut self, value: bool) {
        self.enabled = value
    }

    /// Returns mutable pointer to mutation target.
    pub(crate) fn ptr(&mut self) -> *mut T {
        self.ptr
    }
}
