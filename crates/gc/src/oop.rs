use crate::address::Address;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oop(Address);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InstanceOop(Address);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArrayOop(Address);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ObjArrayOop(Address);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TypeArrayOop(Address);
