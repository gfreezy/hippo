use crate::gc::address::Address;

use crate::gc::oop_desc::{ArrayOopDesc, InstanceOopDesc, OopDesc};

use crate::class::ClassId;
use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Oop(Address);

impl Oop {
    pub fn new(addr: Address) -> Oop {
        Oop(addr)
    }
    pub fn address(&self) -> Address {
        self.0
    }
    pub fn empty() -> Oop {
        unsafe { Oop(Address::zero()) }
    }
    pub fn clear(&self, size: usize) {
        unsafe { std::ptr::write_bytes(self.address().as_mut_ptr::<u8>(), 0, size) }
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for Oop {
    type Target = OopDesc;

    fn deref(&self) -> &Self::Target {
        assert!(!self.is_empty());
        unsafe { mem::transmute(self.0.as_ptr::<Self::Target>()) }
    }
}

impl DerefMut for Oop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.is_empty());
        unsafe { mem::transmute(self.0.as_ptr::<Self::Target>()) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct InstanceOop(pub Oop);

impl InstanceOop {
    pub fn empty() -> Self {
        Oop::empty().into()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for InstanceOop {
    type Target = InstanceOopDesc;

    fn deref(&self) -> &Self::Target {
        assert!(!self.is_empty());
        unsafe { mem::transmute((self.0).0.as_ptr::<Self::Target>()) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct ArrayOop(Oop);

impl ArrayOop {
    fn addr(&self) -> Address {
        (self.0).0
    }
    pub fn oop(&self) -> Oop {
        self.0
    }
    pub fn class_id(&self) -> ClassId {
        self.oop().class
    }
    pub fn empty() -> ArrayOop {
        ArrayOop(Oop::empty())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for ArrayOop {
    type Target = ArrayOopDesc;

    fn deref(&self) -> &Self::Target {
        assert!(!self.is_empty());
        unsafe { mem::transmute((self.0).0.as_ptr::<Self::Target>()) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct ObjArrayOop(Oop);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct TypeArrayOop(Oop);

macro_rules! to_oop {
    ($ty: ty) => {
        impl From<$ty> for Oop {
            fn from(oop: $ty) -> Self {
                oop.0
            }
        }
    };
}

macro_rules! from_oop {
    ($ty: ident) => {
        impl From<Oop> for $ty {
            fn from(oop: Oop) -> Self {
                $ty(oop)
            }
        }
    };
}

to_oop!(InstanceOop);
to_oop!(ArrayOop);
to_oop!(ObjArrayOop);
to_oop!(TypeArrayOop);
from_oop!(InstanceOop);
from_oop!(ArrayOop);
from_oop!(ObjArrayOop);
from_oop!(TypeArrayOop);
