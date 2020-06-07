use crate::address::Address;
use crate::block::{Block, BLOCK_SIZE};
use crate::space::Space;
use std::sync::Arc;

pub struct AllocatorLocal {
    cursor: Address,
    end: Address,
    space: Arc<Space>,
    block: Option<Block>,
}

impl AllocatorLocal {
    pub unsafe fn empty() -> AllocatorLocal {
        AllocatorLocal {
            cursor: Address::zero(),
            end: Address::zero(),
            space: Arc::new(Space::empty()),
            block: None,
        }
    }
    pub fn new(space: Arc<Space>) -> AllocatorLocal {
        let block = space.get_next_usable_block();
        AllocatorLocal {
            cursor: block.start,
            end: block.end,
            space,
            block: Some(block),
        }
    }

    fn alloc_from_global(&mut self, size: usize, align: usize) -> Address {
        if let Some(block) = self.block.take() {
            self.space.return_used_block(block);
        }

        self.block = Some(self.space.get_next_usable_block());
        self.alloc(size, align)
    }

    pub fn alloc(&mut self, size: usize, align: usize) -> Address {
        assert!(size <= BLOCK_SIZE);
        let start = self.cursor.align_up(align);
        let end = start.plus(size);

        // not enough space for current block
        if end > self.end {
            self.alloc_from_global(size, align)
        } else {
            self.cursor = end;
            start
        }
    }
}
