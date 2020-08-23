use super::method::Method;
use crate::class::Class;
use crate::gc::global_definition::BasicType;

#[derive(Debug)]
pub struct CpCache {
    cache: Vec<CpCacheEntry>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum CpCacheEntry {
    StaticField(Class, BasicType, usize),
    Field(BasicType, usize),
    Method(Method),
    Empty,
}

impl CpCache {
    pub fn new(size: usize) -> Self {
        CpCache {
            cache: vec![CpCacheEntry::Empty; size],
        }
    }

    pub fn resolve_static_field(&self, pc: usize) -> Option<(Class, BasicType, usize)> {
        match self.cache.get(pc) {
            Some(CpCacheEntry::StaticField(class, ty, index)) => Some((class.clone(), *ty, *index)),
            Some(CpCacheEntry::Empty) => None,
            Some(_) => unreachable!(),
            None => None,
        }
    }

    pub fn resolve_field(&self, pc: usize) -> Option<(BasicType, usize)> {
        match self.cache.get(pc) {
            Some(CpCacheEntry::Field(ty, index)) => Some((*ty, *index)),
            Some(CpCacheEntry::Empty) => None,
            Some(_) => unreachable!(),
            None => None,
        }
    }

    pub fn set_field(&mut self, pc: usize, ty: BasicType, field_index: usize) {
        self.cache.insert(pc, CpCacheEntry::Field(ty, field_index));
    }

    pub fn set_static_field(&mut self, pc: usize, class: Class, ty: BasicType, field_index: usize) {
        self.cache
            .insert(pc, CpCacheEntry::StaticField(class, ty, field_index));
    }

    pub fn resolve_method(&mut self, _pc: usize) -> CpCache {
        todo!()
    }
}
