use crate::gc::allocator_local::AllocatorLocal;
use crate::gc::block::Block;
use crate::gc::oop::Oop;
use crate::gc::space::Space;
use crate::jvm::SPACE_SIZE;
use std::cell::RefCell;
use std::sync::atomic::Ordering;
use std::sync::Arc;

thread_local! {
    pub static ALLOCATOR_LOCAL: RefCell<AllocatorLocal> = RefCell::new(AllocatorLocal::new(Arc::new(Space::new(SPACE_SIZE.load(Ordering::SeqCst)))));
}

pub fn alloc_tlab(size: usize, align: usize) -> Oop {
    let addr = ALLOCATOR_LOCAL.with(|alloc| alloc.borrow_mut().alloc(size, align));
    Oop::new(addr)
}

pub fn occupy_remaining_tlab(align: usize) -> Option<(Oop, usize)> {
    let (addr, size) = ALLOCATOR_LOCAL.with(|alloc| alloc.borrow().remaining(align))?;
    Some((Oop::new(addr), size))
}

pub fn iter_blocks(f: impl Fn(&Block) + Copy) {
    ALLOCATOR_LOCAL.with(|alloc| {
        let al = alloc.borrow();
        al.space().iter_blocks(f);
        al.current_block().map(f);
    })
}
