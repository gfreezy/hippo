use crate::class::{Class, InstanceMirrorClass};
use crate::class_loader::{get_class_by_id, load_mirror_class};
use crate::gc::global_definition::{BasicType, JObject};
use crate::gc::oop::ArrayOop;
use crate::gc::oop_desc::ArrayOopDesc;
use crate::instruction::can_cast_to;
use crate::jenv::{alloc_jobject, new_jclass};
use crate::jthread::JvmThread;
use crossbeam::atomic::AtomicCell;
use std::ptr::copy_nonoverlapping;
use std::sync::Arc;

#[derive(Clone)]
pub struct TypeArrayClass {
    inner: Arc<InnerTypeArrayClass>,
}

struct InnerTypeArrayClass {
    ty: BasicType,
    name: String,
    class_loader: JObject,
    mirror_class: AtomicCell<JObject>,
}

impl TypeArrayClass {
    pub fn new(name: String, ty: BasicType, loader: JObject) -> Self {
        TypeArrayClass {
            inner: Arc::new(InnerTypeArrayClass {
                ty,
                name,
                class_loader: loader,
                mirror_class: AtomicCell::new(JObject::null()),
            }),
        }
    }

    pub fn ty(&self) -> BasicType {
        self.inner.ty
    }
    pub fn name(&self) -> &str {
        &self.inner.name
    }
    pub fn class_loader(&self) -> JObject {
        self.inner.class_loader.clone()
    }
    pub fn mirror_class(&self) -> JObject {
        let mirror = self.inner.mirror_class.load();
        if mirror.is_null() {
            let mirror_class = load_mirror_class(self.class_loader(), self.name());
            let mirror = new_jclass(&mirror_class.into());
            self.inner.mirror_class.store(mirror);
            mirror
        } else {
            mirror
        }
    }
}

#[derive(Clone)]
pub struct ObjArrayClass {
    inner: Arc<InnerObjArrayClass>,
}

struct InnerObjArrayClass {
    name: String,
    element_class: Class,
    class_loader: JObject,
    mirror_class: AtomicCell<JObject>,
}

impl ObjArrayClass {
    pub fn new(name: String, class: Class, loader: JObject) -> Self {
        ObjArrayClass {
            inner: Arc::new(InnerObjArrayClass {
                name,
                class_loader: loader,
                element_class: class,
                mirror_class: AtomicCell::new(JObject::null()),
            }),
        }
    }
    pub fn element_class(&self) -> Class {
        self.inner.element_class.clone()
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }
    pub fn class_loader(&self) -> JObject {
        self.inner.class_loader.clone()
    }
    pub fn is_inited(&self) -> bool {
        self.element_class().is_inited()
    }
    pub fn mirror_class(&self) -> JObject {
        let mirror = self.inner.mirror_class.load();
        if mirror.is_null() {
            let mirror_class = load_mirror_class(self.class_loader(), self.name());
            let mirror = new_jclass(&mirror_class.into());
            self.inner.mirror_class.store(mirror);
            mirror
        } else {
            mirror
        }
    }
}

pub fn copy_array(
    thread: &mut JvmThread,
    src: ArrayOop,
    src_pos: usize,
    dst: ArrayOop,
    dst_pos: usize,
    length: usize,
) {
    assert!(length + src_pos <= src.len() || length + dst_pos <= dst.len());
    let src_class = get_class_by_id(src.class_id());
    let dst_class = get_class_by_id(dst.class_id());
    assert!(can_cast_to(thread, src_class.clone(), dst_class.clone()));
    if length == 0 {
        return;
    }

    let data_offset = ArrayOopDesc::base_offset_in_bytes();
    let src_ptr = src.oop().address().plus(data_offset).plus(src_pos);
    let dst_ptr = dst.oop().address().plus(data_offset).plus(dst_pos);
    unsafe { copy_nonoverlapping::<u8>(src_ptr.as_ptr(), dst_ptr.as_mut_ptr(), length) };
}
