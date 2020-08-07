use crate::gc::mem::align_usize;
use crate::gc::oop::Oop;
use std::sync::atomic::AtomicPtr;
use std::{fmt, mem};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(C)]
pub struct Address(usize);

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address {{ {} }}", self.0)
    }
}

impl Address {
    #[inline(always)]
    pub fn plus(&self, bytes: usize) -> Address {
        Address(self.0 + bytes)
    }

    #[inline(always)]
    pub fn to_usize(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub unsafe fn load<T: Copy>(&self) -> T {
        *(self.0 as *mut T)
    }

    #[inline(always)]
    pub fn as_ptr<T>(&self) -> *const T {
        self.0 as *const T
    }

    #[inline(always)]
    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.0 as *mut T
    }

    #[inline(always)]
    pub fn as_atomic_ptr(&self) -> &AtomicPtr<Oop> {
        unsafe { &*self.as_ptr::<AtomicPtr<Oop>>() }
    }

    #[inline(always)]
    pub unsafe fn from_pointer<T: Copy>(p: *const T) -> Address {
        mem::transmute(p)
    }

    #[inline(always)]
    pub unsafe fn zero() -> Address {
        Address(0)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn align_up(&self, align: usize) -> Address {
        Address(align_usize(self.0, align))
    }

    pub fn diff(&self, addr: Address) -> usize {
        if self.0 > addr.0 {
            self.0 - addr.0
        } else {
            addr.0 - self.0
        }
    }
}
