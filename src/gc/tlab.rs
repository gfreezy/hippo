use crate::gc::allocator_local::AllocatorLocal;
use crate::gc::oop::Oop;
use std::cell::RefCell;

thread_local! {
    pub static ALLOCATOR_LOCAL: RefCell<AllocatorLocal> = RefCell::new(unsafe {AllocatorLocal::empty()});
}

pub fn initialize_tlab(allocator_local: AllocatorLocal) {
    ALLOCATOR_LOCAL.with(|alloc| {
        *alloc.borrow_mut() = allocator_local;
    });
}

pub fn alloc_tlab(size: usize) -> Oop {
    let addr = ALLOCATOR_LOCAL.with(|alloc| alloc.borrow_mut().alloc(size, 8));
    Oop::new(addr)
}
