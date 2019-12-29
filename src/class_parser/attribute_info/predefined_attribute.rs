use crate::class_parser::attribute_info::AttributeInfo;

pub struct ConstantValueAttribute {
    constant_value_index: u16,
}

pub struct ExceptionHandler {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionHandler>,
    attributes: Vec<AttributeInfo>,
}
