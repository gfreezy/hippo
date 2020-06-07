use crate::allocator_local::AllocatorLocal;
use std::cell::RefCell;

thread_local! {
    pub static ALLOCATOR_LOCAL: RefCell<AllocatorLocal> = RefCell::new(unsafe {AllocatorLocal::empty()});
}

pub fn initialize_tlab(allocator_local: AllocatorLocal) {
    ALLOCATOR_LOCAL.with(|alloc| {
        *alloc.borrow_mut() = allocator_local;
    });
}
