use crate::gc::address::Address;
use crate::gc::block::{Block, BLOCK_SIZE};
use crate::gc::mem::{align_usize, page_align};
use crate::gc::os;
use crate::gc::os::ProtType;
use parking_lot::Mutex;
use std::collections::LinkedList;
use std::fmt;

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
                inited: false,
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

    pub fn get_next_usable_block(&self) -> Option<Block> {
        let mut block = self.usable_blocks.lock().pop_front()?;
        block.inited = true;
        Some(block)
    }

    pub fn iter_blocks(&self, f: impl Fn(&Block)) {
        for block in self.used_blocks.lock().iter() {
            f(block);
        }

        for block in self.usable_blocks.lock().iter().filter(|b| b.inited) {
            f(block);
        }
    }
}

impl Drop for Space {
    fn drop(&mut self) {
        let size = self.end.diff(self.start);
        if size > 0 {
            unsafe { os::munmap(self.start.as_ptr(), size) }
        }
    }
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Space")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_spaces() {
        test_new_space(1);
        test_new_space(10);
        test_new_space(1024);
        test_new_space(20 * 1024 * 1024);
    }

    fn test_new_space(size: usize) {
        let space = Space::new(size);
        assert_eq!(space.end.diff(space.start), align_usize(size, BLOCK_SIZE));
    }

    #[test]
    fn test_no_more_block() {
        let size = 1024 * 1024;
        let space = Space::new(size);
        let block = space.get_next_usable_block().unwrap();
        assert_eq!(block.start.diff(block.end), align_usize(size, BLOCK_SIZE));
        let block = space.get_next_usable_block();
        assert!(block.is_none());
    }

    #[test]
    fn test_no_more_block2() {
        let size = BLOCK_SIZE;
        let space = Space::new(size);
        let block = space.get_next_usable_block().unwrap();
        assert_eq!(block.start.diff(block.end), align_usize(size, BLOCK_SIZE));
        let block = space.get_next_usable_block();
        assert!(block.is_none());
    }

    #[test]
    fn test_have_2_blocks() {
        let size = BLOCK_SIZE + 1;
        let space = Space::new(size);
        let block = space.get_next_usable_block().unwrap();
        assert_eq!(block.start.diff(block.end), BLOCK_SIZE);
        let block = space.get_next_usable_block();
        assert!(block.is_some());
        let block = space.get_next_usable_block();
        assert!(block.is_none());
    }
}
