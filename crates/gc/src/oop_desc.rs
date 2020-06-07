use crate::address::Address;
use crate::global_definition::type_to_basic_type::{type_to_basic_type, TypeToBasicType};
use crate::global_definition::{
    BasicType, JObject, HEAP_OOP_SIZE, HEAP_WORDS_PER_LONG, HEAP_WORD_SIZE, LOG_HEAP_WORD_SIZE,
};
use crate::mem::align_usize;
use std::mem::size_of;

pub type ClassId = usize;

pub struct Class;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OopDesc {
    mark: u64,
    class: ClassId,
}

impl OopDesc {
    pub fn header_size() -> usize {
        size_of::<OopDesc>() / HEAP_WORD_SIZE
    }

    fn size_given_class(&self, class: ClassId) -> usize {
        unimplemented!()
    }

    pub fn size(&self) -> usize {
        self.size_given_class(self.class)
    }

    pub fn is_instance(&self) -> bool {
        unimplemented!()
    }

    pub fn is_array(&self) -> bool {
        unimplemented!()
    }

    pub fn is_obj_array(&self) -> bool {
        unimplemented!()
    }

    pub fn is_type_array(&self) -> bool {
        unimplemented!()
    }

    pub fn field_addr(&self, offset: usize) -> Address {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InstanceOopDesc(OopDesc);

impl InstanceOopDesc {
    pub fn header_size() -> usize {
        size_of::<InstanceOopDesc>() / HEAP_WORD_SIZE
    }

    pub fn base_offset_in_bytes() -> usize {
        size_of::<InstanceOopDesc>()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArrayOopDesc(OopDesc);

impl ArrayOopDesc {
    pub fn header_size_in_bytes() -> usize {
        align_usize(
            Self::length_offset_in_bytes() + size_of::<usize>(),
            HEAP_WORD_SIZE,
        )
    }

    fn length_offset_in_bytes() -> usize {
        size_of::<ArrayOopDesc>()
    }

    pub fn base_offset_in_bytes(ty: BasicType) -> usize {
        Self::header_size(ty) * HEAP_WORD_SIZE
    }

    pub fn header_size(ty: BasicType) -> usize {
        let type_size_in_bytes = Self::header_size_in_bytes();
        if Self::element_type_should_be_aligned(ty) {
            align_usize(type_size_in_bytes / HEAP_WORD_SIZE, HEAP_WORDS_PER_LONG)
        } else {
            type_size_in_bytes / HEAP_WORD_SIZE
        }
    }

    pub fn element_type_should_be_aligned(ty: BasicType) -> bool {
        matches!(ty, BasicType::Double | BasicType::Long)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ObjArrayOopDesc(ArrayOopDesc);

impl ObjArrayOopDesc {
    pub fn base_offset_in_bytes() -> usize {
        ArrayOopDesc::base_offset_in_bytes(BasicType::Object)
    }

    pub fn element_offset<T>(index: usize) -> usize {
        Self::base_offset_in_bytes() + size_of::<T>() * index
    }

    pub fn array_size(len: usize) -> usize {
        const OOP_PER_HEAP_WORD: usize = HEAP_WORD_SIZE / HEAP_OOP_SIZE;
        (len + OOP_PER_HEAP_WORD - 1) / OOP_PER_HEAP_WORD
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TypeArrayOopDesc(ArrayOopDesc);

impl TypeArrayOopDesc {
    pub fn element_offset<T>(index: usize) -> usize
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        ArrayOopDesc::base_offset_in_bytes(type_to_basic_type::<T>()) + size_of::<T>() * index
    }
}
