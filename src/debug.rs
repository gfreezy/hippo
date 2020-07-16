use crate::class::Class;
use crate::class_loader::get_class_by_id;
use crate::gc::block::Block;
use crate::gc::global_definition::{BasicType, JArray, JObject};
use crate::gc::mem::{align_usize, view_memory};
use crate::gc::oop::Oop;
use crate::gc::oop_desc::{ArrayOopDesc, InstanceOopDesc};
use crate::gc::tlab::iter_blocks;
use pretty::RcDoc;
use std::io::Write;

pub fn dump_block(block: &Block) {
    let mut addr = block.start.align_up(8);
    while addr < block.end {
        let oop = Oop::new(addr);
        if oop.class > 0 {
            let class = get_class_by_id(oop.class);
            println!("class: {:?}", class);
            let obj = JObject::make_from_oop(oop);
            if !class.is_array_class() {
                print!("addr: {:?} ", addr);
                pretty_print(obj);
                let obj_size = InstanceOopDesc::base_offset_in_bytes() + class.instance_size();
                addr = addr.plus(align_usize(obj_size, 8));
            } else {
                print!("addr: {:?} ", addr);
                let jarray: JArray = obj.into();
                pretty_print(jarray);
                let obj_size = ArrayOopDesc::array_size_in_bytes_with_basic_type(
                    class.element_type(),
                    jarray.len(),
                );
                addr = addr.plus(align_usize(obj_size, 8));
            }
        } else {
            println!("addr: {:?}", addr);
            addr = addr
                .plus(InstanceOopDesc::header_size_in_bytes())
                .align_up(8);
        }
    }
}

pub fn dump_space() {
    iter_blocks(|block| dump_block(block))
}

pub fn pretty_write(
    v: impl Into<RcDoc<'static>>,
    width: usize,
    f: &mut impl Write,
) -> std::io::Result<()> {
    let doc = v.into();
    doc.render(width, f)
}

pub fn pretty_print(v: impl Into<RcDoc<'static>>) {
    let mut w = Vec::new();
    pretty_write(v, 80, &mut w).unwrap();
    println!("{}", String::from_utf8(w).unwrap());
}

impl From<JObject> for RcDoc<'static> {
    fn from(o: JObject) -> Self {
        if o.is_null() {
            return RcDoc::text("null");
        }
        if o.class_id() == 0 {
            return RcDoc::nil();
        }

        fn add_class_fields(o: JObject, class: &Class, docs: &mut Vec<RcDoc<'static>>) {
            for iter_field in class.instance_fields().values() {
                let f_type = iter_field.basic_type();
                let f_value = o.get_field_by_basic_type_and_offset(f_type, iter_field.offset());
                let doc: RcDoc<'static> = match f_type {
                    BasicType::Array => f_value.as_jarray().into(),
                    BasicType::Object => f_value.as_jobject().into(),
                    _ => RcDoc::as_string(format!("{:?}", f_value)),
                };
                docs.push(
                    RcDoc::as_string(iter_field.name())
                        .append(": ")
                        .append(doc.nest(1).group()),
                );
            }
        }

        let class = get_class_by_id(o.class_id());
        let mut field_docs = vec![];
        add_class_fields(o, &class, &mut field_docs);

        RcDoc::as_string(class.name())
            .append("{")
            .append(RcDoc::intersperse(field_docs, ", ").nest(1).group())
            .append("}")
    }
}

impl From<JArray> for RcDoc<'static> {
    fn from(o: JArray) -> Self {
        if o.is_null() {
            return RcDoc::text("null");
        }
        let class = get_class_by_id(o.class_id());
        let mut docs = vec![];
        const MAX_LEN: usize = 30;
        match &class {
            Class::TypeArrayClass(c) => {
                for i in 0..o.len().min(MAX_LEN) {
                    docs.push(
                        RcDoc::as_string(i)
                            .append(": ")
                            .append(format!("{:?}", o.get_with_basic_type(c.ty(), i))),
                    );
                }
            }
            Class::ObjArrayClass(_) => {
                for i in 0..o.len().min(MAX_LEN) {
                    let doc: RcDoc = o.get::<JObject>(i).into();
                    docs.push(RcDoc::as_string(i).append(": ").append(doc.nest(1).group()));
                }
            }
            _ => unreachable!(),
        }
        RcDoc::as_string("#")
            .append(o.len().to_string())
            .append("[")
            .append(RcDoc::intersperse(docs, ", ").nest(1).group())
            .append("]")
    }
}

pub fn dump_jobject(src: JObject) {
    let src_class = get_class_by_id(src.class_id());
    view_memory::<u8>(
        src.oop().address().as_ptr(),
        InstanceOopDesc::base_offset_in_bytes() + src_class.instance_size(),
    );
}

pub fn dump_jarray(src: JArray) {
    let src_class = get_class_by_id(src.class_id());
    view_memory::<u8>(
        src.array_oop().oop().address().as_ptr(),
        ArrayOopDesc::array_size_in_bytes_with_basic_type(src_class.element_type(), src.len()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jenv::new_java_lang_string;
    use crate::jvm::Jvm;
    use std::io::stdout;

    #[test]
    fn test_pretty_print_jobject() {
        let _jvm = Jvm::new(Some("./jre".to_string()), Some("./jre/lib/rt".to_string()));
        let s = new_java_lang_string("hello");
        pretty_print(s);
        dump_space();
    }
}
