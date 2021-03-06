use crate::runtime::class::Class;
use crate::runtime::jvm_env::JvmPC;
use crate::runtime::method::Method;

#[derive(Debug)]
pub struct CpCache {
    cache: Vec<CpCacheEntry>,
}

#[derive(Debug, Clone)]
enum CpCacheEntry {
    StaticField(Class, usize),
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

    pub fn resolve_static_field(&self, pc: JvmPC) -> Option<(Class, usize)> {
        match self.cache.get(pc) {
            Some(CpCacheEntry::StaticField(class, index)) => Some((class.clone(), *index)),
            Some(CpCacheEntry::Empty) => None,
            Some(_) => unreachable!(),
            None => None,
        }
    }

    pub fn resolve_field(&self, pc: JvmPC) -> Option<usize> {
        match self.cache.get(pc) {
            Some(CpCacheEntry::Field(index)) => Some(*index),
            Some(CpCacheEntry::Empty) => None,
            Some(_) => unreachable!(),
            None => None,
        }
    }

    pub fn set_field(&mut self, pc: JvmPC, field_index: usize) {
        self.cache.insert(pc, CpCacheEntry::Field(field_index));
    }

    pub fn set_static_field(&mut self, pc: JvmPC, class: Class, field_index: usize) {
        self.cache
            .insert(pc, CpCacheEntry::StaticField(class, field_index));
    }

    pub fn resolve_method(&mut self, _pc: JvmPC) -> CpCache {
        unimplemented!()
    }
}
