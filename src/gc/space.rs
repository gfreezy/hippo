use crate::gc::address::Address;
use crate::gc::block::{Block, BLOCK_SIZE};
use crate::gc::mem::{align_usize, page_align};
use crate::gc::os;
use crate::gc::os::ProtType;
use parking_lot::Mutex;
use std::collections::LinkedList;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Space {
    start: Address,
    end: Address,
    used_blocks: Mutex<LinkedList<Block>>,
    usable_blocks: Mutex<LinkedList<Block>>,
}

impl Space {
    pub unsafe fn empty() -> Space {
        Space {
            start: Address::zero(),
            end: Address::zero(),
            used_blocks: Mutex::new(LinkedList::new()),
            usable_blocks: Mutex::new(LinkedList::new()),
        }
    }

    pub fn new(size: usize) -> Space {
        let block_aligned_size = align_usize(size, BLOCK_SIZE);
        let page_aligned_size = page_align(block_aligned_size);
        let start = unsafe {
            let ptr = os::mmap(page_aligned_size, ProtType::Writable);
            Address::from_pointer(ptr)
        };
        let end = start.plus(page_aligned_size);
        let mut usable_blocks = LinkedList::new();
        for num in 0..page_aligned_size / BLOCK_SIZE {
            let block_start = start.plus(num * BLOCK_SIZE);
            let block_end = start.plus((num + 1) * BLOCK_SIZE);
            let block = Block {
                start: block_start,
                end: block_end,
            };
            usable_blocks.push_back(block);
        }
        Space {
            start,
            end,
            used_blocks: Mutex::new(LinkedList::new()),
            usable_blocks: Mutex::new(usable_blocks),
        }
    }

    pub fn return_used_block(&self, block: Block) {
        self.used_blocks.lock().push_back(block)
    }

    pub fn get_next_usable_block(&self) -> Block {
        self.usable_blocks.lock().pop_front().expect("OOM")
    }
}

impl Drop for Space {
    fn drop(&mut self) {
        let size = self.end.diff(self.start);
        unsafe { os::munmap(self.start.as_ptr(), size) }
    }
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Space")
    }
}
