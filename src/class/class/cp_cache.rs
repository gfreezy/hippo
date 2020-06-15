use crate::class::class::method::Method;
use crate::class::class::Class;

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

    pub fn resolve_static_field(&self, pc: usize) -> Option<(Class, usize)> {
        match self.cache.get(pc) {
            Some(CpCacheEntry::StaticField(class, index)) => Some((class.clone(), *index)),
            Some(CpCacheEntry::Empty) => None,
            Some(_) => unreachable!(),
            None => None,
        }
    }

    pub fn resolve_field(&self, pc: usize) -> Option<usize> {
        match self.cache.get(pc) {
            Some(CpCacheEntry::Field(index)) => Some(*index),
            Some(CpCacheEntry::Empty) => None,
            Some(_) => unreachable!(),
            None => None,
        }
    }

    pub fn set_field(&mut self, pc: usize, field_index: usize) {
        self.cache.insert(pc, CpCacheEntry::Field(field_index));
    }

    pub fn set_static_field(&mut self, pc: usize, class: Class, field_index: usize) {
        self.cache
            .insert(pc, CpCacheEntry::StaticField(class, field_index));
    }

    pub fn resolve_method(&mut self, _pc: usize) -> CpCache {
        unimplemented!()
    }
}
