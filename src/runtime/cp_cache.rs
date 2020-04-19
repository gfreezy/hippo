use crate::runtime::jvm_env::JvmPC;
use crate::runtime::method::Method;
use tracing::debug;

#[derive(Debug)]
pub struct CpCache {
    cache: Vec<CpCacheEntry>,
}

#[derive(Debug, Clone)]
enum CpCacheEntry {
    Field(usize),
    Method(Method),
    Empty,
}

impl CpCache {
    pub fn new(size: usize) -> Self {
        CpCache {
            cache: vec![CpCacheEntry::Empty; size],
        }
    }

    pub fn resolve_field(&self, pc: JvmPC) -> Option<usize> {
        let i = match self.cache.get(pc) {
            Some(CpCacheEntry::Field(index)) => Some(*index),
            Some(CpCacheEntry::Empty) => None,
            Some(_) => unreachable!(),
            None => None,
        };
        debug!(cache = ?self.cache, %pc, resolved_index = ?i, "resolve_field");
        i
    }

    pub fn set_field(&mut self, pc: JvmPC, field_index: usize) {
        self.cache.insert(pc, CpCacheEntry::Field(field_index));
    }

    pub fn resolve_method(&mut self, _pc: JvmPC) -> CpCache {
        unimplemented!()
    }
}
