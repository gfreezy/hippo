use crate::gc::address::Address;

pub const BLOCK_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug)]
pub struct Block {
    pub(crate) start: Address,
    pub(crate) end: Address,
}
