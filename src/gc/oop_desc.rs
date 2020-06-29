use crate::class::ClassId;
use crate::gc::address::Address;
use crate::gc::global_definition::type_to_basic_type::{
    size_of_java_type, type_to_basic_type, TypeToBasicType,
};
use crate::gc::global_definition::{BasicType, JArray, JObject, HEAP_WORD_SIZE};
use crate::gc::mark_word::MarkWord;
use crate::gc::mem::{align_usize, is_aligned, view_memory};

use std::any::type_name;
use std::mem::{size_of, transmute};

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct OopDesc {
    pub mark: MarkWord,
    pub class: ClassId,
}

impl OopDesc {
    pub fn header_size() -> usize {
        size_of::<OopDesc>() / HEAP_WORD_SIZE
    }

    pub fn size(&self) -> usize {
        unimplemented!()
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

    pub fn field_addr(&self, _offset: usize) -> Address {
        unimplemented!()
    }

    pub fn mark_offset_in_bytes() -> usize {
        let offset = field_offset::offset_of!(OopDesc => mark);
        offset.get_byte_offset()
    }

    pub fn identity_hash(&self) -> i32 {
        self.mark.hash()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct InstanceOopDesc(pub OopDesc);

impl InstanceOopDesc {
    pub fn header_size() -> usize {
        Self::header_size_in_bytes() / HEAP_WORD_SIZE
    }

    pub fn header_size_in_bytes() -> usize {
        align_usize(size_of::<InstanceOopDesc>(), HEAP_WORD_SIZE)
    }

    pub fn base_offset_in_bytes() -> usize {
        Self::header_size_in_bytes()
    }

    pub fn set_field_by_offset<T>(&self, offset: usize, value: T)
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        let base_offset = Self::base_offset_in_bytes();
        unsafe {
            let self_offset: *mut T = transmute(self);
            let relative_offset = base_offset + offset;
            assert!(is_aligned(relative_offset, size_of::<T>()));
            let field_offset = self_offset.offset((relative_offset / size_of::<T>()) as isize);
            field_offset.write(value)
        }
    }

    pub fn get_field_by_offset<T>(&self, offset: usize) -> T
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        let base_offset = Self::base_offset_in_bytes();
        unsafe {
            let self_offset: *const T = transmute(self);
            let relative_offset = base_offset + offset;
            assert!(is_aligned(relative_offset, size_of::<T>()));
            let field_offset = self_offset.offset((relative_offset / size_of::<T>()) as isize);
            field_offset.read()
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct ArrayOopDesc(OopDesc);

impl ArrayOopDesc {
    fn header_size_in_bytes() -> usize {
        align_usize(
            Self::length_offset_in_bytes() + size_of::<usize>(),
            HEAP_WORD_SIZE,
        )
    }

    pub fn length_offset_in_bytes() -> usize {
        size_of::<ArrayOopDesc>()
    }

    pub fn base_offset_in_bytes(_ty: BasicType) -> usize {
        Self::header_size_in_bytes()
    }

    pub fn header_size(_ty: BasicType) -> usize {
        let type_size_in_bytes = Self::header_size_in_bytes();
        type_size_in_bytes / HEAP_WORD_SIZE
    }

    pub fn len(&self) -> usize {
        let length_offset = ArrayOopDesc::length_offset_in_bytes();
        unsafe {
            let self_offset: *const usize = transmute(self);
            assert!(is_aligned(length_offset, size_of::<usize>()));
            let len_pointer = self_offset.offset((length_offset / size_of::<usize>()) as isize);
            len_pointer.read()
        }
    }

    pub fn set_len(&self, len: usize) {
        let length_offset = ArrayOopDesc::length_offset_in_bytes();
        unsafe {
            let self_offset: *mut usize = transmute(self);
            assert!(is_aligned(length_offset, size_of::<usize>()));
            let len_pointer = self_offset.offset((length_offset / size_of::<usize>()) as isize);
            len_pointer.write(len)
        }
    }

    pub fn as_slice<T>(&self) -> &[T]
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        let data_offset = ArrayOopDesc::base_offset_in_bytes(type_to_basic_type::<T>(None));
        unsafe {
            let self_offset: *const T = transmute(self);
            assert!(is_aligned(data_offset, size_of::<T>()));
            let data_pointer = self_offset.offset((data_offset / size_of::<T>()) as isize);
            unsafe { std::slice::from_raw_parts(data_pointer, self.len()) }
        }
    }

    pub fn as_mut_slice<T>(&self) -> &mut [T]
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        let data_offset = ArrayOopDesc::base_offset_in_bytes(type_to_basic_type::<T>(None));
        unsafe {
            let self_offset: *mut T = transmute(self);
            assert!(is_aligned(data_offset, size_of::<T>()));
            let data_pointer = self_offset.offset((data_offset / size_of::<T>()) as isize);
            std::slice::from_raw_parts_mut(data_pointer, self.len())
        }
    }

    pub fn copy_from<T>(&self, src: &[T])
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        assert_eq!(src.len(), self.len());
        let data_offset = ArrayOopDesc::base_offset_in_bytes(type_to_basic_type::<T>(None));
        assert!(is_aligned(data_offset, size_of::<T>()));

        unsafe {
            let self_offset: *mut T = transmute(self);
            let data_pointer = self_offset.offset((data_offset / size_of::<T>()) as isize);
            std::ptr::copy_nonoverlapping(src.as_ptr(), data_pointer, self.len());
        }
    }

    pub fn element_at<T>(&self, index: usize) -> T
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        let data_offset = ArrayOopDesc::base_offset_in_bytes(type_to_basic_type::<T>(None));
        unsafe {
            let self_offset: *const T = transmute(self);
            let offset = data_offset + size_of_java_type::<T>(None) * index;
            assert!(is_aligned(offset, size_of::<T>()));
            let el_pointer = self_offset.offset((offset / size_of::<T>()) as isize);
            el_pointer.read()
        }
    }

    pub fn set_element_at<T>(&self, index: usize, val: T)
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        let data_offset = ArrayOopDesc::base_offset_in_bytes(type_to_basic_type::<T>(None));
        unsafe {
            let self_offset: *mut T = transmute(self);
            let offset = data_offset + size_of_java_type::<T>(None) * index;
            assert!(is_aligned(offset, size_of::<T>()));
            let el_pointer = self_offset.offset((offset / size_of::<T>()) as isize);
            el_pointer.write(val)
        }
    }

    pub fn array_size_in_bytes<T>(len: usize) -> usize
    where
        TypeToBasicType<T>: Into<BasicType>,
    {
        let header_size = ArrayOopDesc::header_size_in_bytes();
        header_size + size_of_java_type::<T>(None) * len
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct ObjArrayOopDesc(ArrayOopDesc);

impl ObjArrayOopDesc {}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct TypeArrayOopDesc(ArrayOopDesc);

impl TypeArrayOopDesc {}
