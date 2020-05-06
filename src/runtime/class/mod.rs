mod instance_class;
mod obj_array_class;
mod type_array_class;

pub use instance_class::InstanceClass;
pub use obj_array_class::ObjArrayClass;
pub use type_array_class::TypeArrayClass;

#[derive(Clone, Debug)]
pub enum Class {
    InstanceClass(InstanceClass),
    ObjArrayClass(ObjArrayClass),
    TypeArrayClass(TypeArrayClass),
}

impl Class {
    pub fn instance_class(&self) -> InstanceClass {
        match self {
            Class::InstanceClass(i) => i.clone(),
            _ => unreachable!(),
        }
    }

    pub fn obj_array_class(&self) -> ObjArrayClass {
        match self {
            Class::ObjArrayClass(i) => i.clone(),
            _ => unreachable!(),
        }
    }
    pub fn type_array_class(&self) -> TypeArrayClass {
        match self {
            Class::TypeArrayClass(i) => i.clone(),
            _ => unreachable!(),
        }
    }
}
impl From<InstanceClass> for Class {
    fn from(c: InstanceClass) -> Self {
        Class::InstanceClass(c)
    }
}

impl From<ObjArrayClass> for Class {
    fn from(c: ObjArrayClass) -> Self {
        Class::ObjArrayClass(c)
    }
}
impl From<TypeArrayClass> for Class {
    fn from(c: TypeArrayClass) -> Self {
        Class::TypeArrayClass(c)
    }
}
