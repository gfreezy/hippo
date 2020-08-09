use crate::gc::global_definition::JObject;
use fxhash::FxBuildHasher;
use indexmap::set::IndexSet;
use parking_lot::Mutex;

pub type FxIndexSet<T> = IndexSet<T, FxBuildHasher>;

pub struct StringTable {
    inner: Mutex<Inner>,
}

struct Inner {
    set: FxIndexSet<String>,
    jobjects: Vec<JObject>,
}

impl StringTable {
    pub fn new() -> Self {
        StringTable {
            inner: Mutex::new(Inner {
                set: FxIndexSet::default(),
                jobjects: Vec::with_capacity(1000),
            }),
        }
    }

    pub fn get(&self, s: &str) -> Option<JObject> {
        let inner = &mut *self.inner.lock();
        let (index, _) = inner.set.get_full(s)?;
        Some(inner.jobjects[index])
    }

    pub fn intern(&self, s: &str, jobject: JObject) -> JObject {
        let inner = &mut *self.inner.lock();
        if let Some((index, _)) = inner.set.get_full(s) {
            return inner.jobjects[index];
        }
        let (index, _) = inner.set.insert_full(s.to_string());
        inner.jobjects.resize(index + 1, JObject::null());
        inner.jobjects[index] = jobject;
        jobject
    }
}
