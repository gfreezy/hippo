use self::ElementValue::{
    AnnotationValue, ArrayValue, ClassInfoIndex, ConstantBooleanIndex, ConstantByteIndex,
    ConstantCharIndex, ConstantDoubleIndex, ConstantFloatIndex, ConstantIntIndex,
    ConstantLongIndex, ConstantShortIndex, ConstantStringIndex, EnumConstantValue,
};
use self::StackMapFrame::{
    AppendFrame, ChopFrame, FullFrame, SameFrame, SameFrameExtended,
    SameLocals1StackItemFramExtended, SameLocals1StackItemFrame,
};
use self::TargetInfo::{
    CatchTarget, EmptyTarget, FormalParameterTarget, LocalvarTarget, OffsetTarget, SupertypeTarget,
    ThrowsTarget, TypeArgumentTarget, TypeParameterBoundTarget, TypeParameterTarget,
};
use self::VerificationTypeInfo::{
    DoubleVariableInfo, FloatVariableInfo, IntegerVariableInfo, LongVariableInfo, NullVariableInfo,
    ObjectVariableInfo, TopVariableInfo, UninitializedThisVariableInfo, UninitializedVariableInfo,
};
use crate::class_parser::attribute_info::{parse_attribute_info, AttributeInfo};
use crate::nom_utils::length_many;
use nom::error::ParseError;
use nom::multi::{length_data, length_value, many1_count, many_m_n};
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::{IResult, InputLength, InputTake, ToUsize};

pub struct ConstantValueAttribute {
    attribute_name_index: u16,
    constant_value_index: u16,
}

pub fn parse_constant_value_attribute(buf: &[u8]) -> IResult<&[u8], ConstantValueAttribute> {
    let (left, attribute_name_index) = be_u16(buf)?;
    let (left, attribute_length) = be_u32(left)?;
    assert_eq!(attribute_length, 2);
    let (left, constant_value_index) = be_u16(left)?;
    Ok((
        left,
        ConstantValueAttribute {
            attribute_name_index,
            constant_value_index,
        },
    ))
}

pub struct ExceptionHandler {
    attribute_name_index: u16,
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub fn parse_exception_handler(buf: &[u8]) -> IResult<&[u8], ExceptionHandler> {
    let (left, attribute_name_index) = be_u16(buf)?;
    let (left, _attribute_length) = be_u32(left)?;
    let (left, start_pc) = be_u16(left)?;
    let (left, end_pc) = be_u16(left)?;
    let (left, handler_pc) = be_u16(left)?;
    let (left, catch_type) = be_u16(left)?;
    Ok((
        left,
        ExceptionHandler {
            attribute_name_index,
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        },
    ))
}

pub struct CodeAttribute {
    attribute_name_index: u16,
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionHandler>,
    attributes: Vec<AttributeInfo>,
}

pub fn parse_code_attribute(buf: &[u8]) -> IResult<&[u8], CodeAttribute> {
    let (left, attribute_name_index) = be_u16(buf)?;
    let (left, attribute_length) = be_u32(left)?;
    let (left, max_stack) = be_u16(left)?;
    let (left, max_locals) = be_u16(left)?;
    let (left, code) = length_data(be_u32)(left)?;
    let (left, exception_table_length) = be_u16(left)?;
    let (left, exception_table) = many_m_n(
        exception_table_length as usize,
        exception_table_length as usize,
        parse_exception_handler,
    )(left)?;
    let (left, attributes_count) = be_u16(left)?;
    let (left, attributes) = many_m_n(
        attributes_count as usize,
        attributes_count as usize,
        parse_attribute_info,
    )(left)?;

    Ok((
        left,
        CodeAttribute {
            attribute_name_index,
            max_stack,
            max_locals,
            code: code.to_vec(),
            exception_table,
            attributes,
        },
    ))
}

pub enum VerificationTypeInfo {
    TopVariableInfo,
    IntegerVariableInfo,
    FloatVariableInfo,
    NullVariableInfo,
    UninitializedThisVariableInfo,
    ObjectVariableInfo { const_pool_index: u16 },
    UninitializedVariableInfo { offset: u16 },
    LongVariableInfo,
    DoubleVariableInfo,
}

pub fn parse_verification_type_info(buf: &[u8]) -> IResult<&[u8], VerificationTypeInfo> {
    let (buf, tag) = be_u8(buf)?;
    match tag {
        0 => Ok((buf, TopVariableInfo)),
        1 => Ok((buf, IntegerVariableInfo)),
        2 => Ok((buf, FloatVariableInfo)),
        3 => Ok((buf, DoubleVariableInfo)),
        4 => Ok((buf, LongVariableInfo)),
        5 => Ok((buf, NullVariableInfo)),
        6 => Ok((buf, UninitializedThisVariableInfo)),
        7 => {
            let (buf, cpool_index) = be_u16(buf)?;
            Ok((
                buf,
                ObjectVariableInfo {
                    const_pool_index: cpool_index,
                },
            ))
        }
        8 => {
            let (buf, offset) = be_u16(buf)?;
            Ok((buf, UninitializedVariableInfo { offset }))
        }
        _ => unreachable!(),
    }
}

pub enum StackMapFrame {
    SameFrame {
        offset_delta: u16,
    },
    SameLocals1StackItemFrame {
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemFramExtended {
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    ChopFrame {
        k: u16,
        offset_delta: u16,
    },
    SameFrameExtended {
        offset_delta: u16,
    },
    AppendFrame {
        k: u16,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

pub fn parse_stack_map_frame(buf: &[u8]) -> IResult<&[u8], StackMapFrame> {
    let (buf, frame_type) = be_u8(buf)?;
    match frame_type {
        ty @ 0..=63 => Ok((
            buf,
            SameFrame {
                offset_delta: ty as u16,
            },
        )),
        ty @ 64..=127 => {
            let offset_delta = (ty - 64) as u16;
            let (buf, stack) = parse_verification_type_info(buf)?;
            Ok((
                buf,
                SameLocals1StackItemFrame {
                    offset_delta,
                    stack,
                },
            ))
        }
        247 => {
            let (buf, offset_delta) = be_u16(buf)?;
            let (buf, stack) = parse_verification_type_info(buf)?;
            Ok((
                buf,
                SameLocals1StackItemFramExtended {
                    offset_delta,
                    stack,
                },
            ))
        }
        ty @ 248..=250 => {
            let k = 251 - ty;
            let (buf, offset_delta) = be_u16(buf)?;
            Ok((
                buf,
                ChopFrame {
                    k: k as u16,
                    offset_delta,
                },
            ))
        }
        251 => {
            let (buf, offset_delta) = be_u16(buf)?;
            Ok((buf, SameFrameExtended { offset_delta }))
        }
        ty @ 252..=254 => {
            let k = (ty - 251) as u16;
            let (buf, offset_delta) = be_u16(buf)?;
            let (buf, locals) =
                many_m_n(k as usize, k as usize, parse_verification_type_info)(buf)?;
            Ok((
                buf,
                AppendFrame {
                    k,
                    offset_delta,
                    locals,
                },
            ))
        }
        255 => {
            let (buf, offset_delta) = be_u16(buf)?;
            let (buf, locals) = length_many(be_u16, parse_verification_type_info)(buf)?;
            let (buf, stack) = length_many(be_u16, parse_verification_type_info)(buf)?;
            Ok((
                buf,
                FullFrame {
                    offset_delta,
                    locals,
                    stack,
                },
            ))
        }
        _ => unreachable!(),
    }
}

pub struct StackMapTableAttribute {
    attribute_name_index: u16,
    entries: Vec<StackMapFrame>,
}

pub fn parse_stack_map_table_attribute(buf: &[u8]) -> IResult<&[u8], StackMapTableAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, entries) = length_many(be_u16, parse_stack_map_frame)(buf)?;

    Ok((
        buf,
        StackMapTableAttribute {
            attribute_name_index,
            entries,
        },
    ))
}

pub struct ExceptionsAttribute {
    attribute_name_index: u16,
    exception_index_table: Vec<u16>,
}

pub fn parse_exceptions_attribute(buf: &[u8]) -> IResult<&[u8], ExceptionsAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, index_table) = length_many(be_u16, be_u16)(buf)?;
    Ok((
        buf,
        ExceptionsAttribute {
            attribute_name_index,
            exception_index_table: index_table,
        },
    ))
}

pub struct Class {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

fn parse_class(buf: &[u8]) -> IResult<&[u8], Class> {
    let (buf, inner_class_info_index) = be_u16(buf)?;
    let (buf, outer_class_info_index) = be_u16(buf)?;
    let (buf, inner_name_index) = be_u16(buf)?;
    let (buf, inner_class_access_flags) = be_u16(buf)?;
    Ok((
        buf,
        Class {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        },
    ))
}

pub struct InnerClasses {
    attribute_name_index: u16,
    classes: Vec<Class>,
}

pub fn parse_inner_class(buf: &[u8]) -> IResult<&[u8], InnerClasses> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, classes) = length_many(be_u16, parse_class)(buf)?;
    Ok((
        buf,
        InnerClasses {
            attribute_name_index,
            classes,
        },
    ))
}

pub struct EnclosingMethodAttribute {
    attribute_name_index: u16,
    class_index: u16,
    method_index: u16,
}

pub fn parse_enclosing_method_attribute(buf: &[u8]) -> IResult<&[u8], EnclosingMethodAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, class_index) = be_u16(buf)?;
    let (buf, method_index) = be_u16(buf)?;
    Ok((
        buf,
        EnclosingMethodAttribute {
            attribute_name_index,
            class_index,
            method_index,
        },
    ))
}

pub struct SyntheticAttribute {
    attribute_name_index: u16,
}

pub fn parse_synthetic_attribute(buf: &[u8]) -> IResult<&[u8], SyntheticAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    Ok((
        buf,
        SyntheticAttribute {
            attribute_name_index,
        },
    ))
}

pub struct SignatureAttribute {
    attribute_name_index: u16,
    signature_index: u16,
}

pub fn parse_signature_attribute(buf: &[u8]) -> IResult<&[u8], SignatureAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, signature_index) = be_u16(buf)?;

    Ok((
        buf,
        SignatureAttribute {
            attribute_name_index,
            signature_index,
        },
    ))
}

pub struct SourceFileAttribute {
    attribute_name_index: u16,
    sourcefile_index: u16,
}

pub fn parse_source_file_attribute(buf: &[u8]) -> IResult<&[u8], SourceFileAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, sourcefile_index) = be_u16(buf)?;

    Ok((
        buf,
        SourceFileAttribute {
            attribute_name_index,
            sourcefile_index,
        },
    ))
}

pub struct SourceDebugExtensionAttribute {
    attribute_name_index: u16,
    debug_extension: Vec<u8>,
}

pub fn parse_source_debug_extension_attribute(
    buf: &[u8],
) -> IResult<&[u8], SourceDebugExtensionAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, debug_extension) = length_data(be_u32)(buf)?;

    Ok((
        buf,
        SourceDebugExtensionAttribute {
            attribute_name_index,
            debug_extension: debug_extension.to_vec(),
        },
    ))
}

pub struct LineNumberTable {
    start_pc: u16,
    line_number: u16,
}

fn parse_line_number_table(buf: &[u8]) -> IResult<&[u8], LineNumberTable> {
    let (buf, start_pc) = be_u16(buf)?;
    let (buf, line_number) = be_u16(buf)?;

    Ok((
        buf,
        LineNumberTable {
            start_pc,
            line_number,
        },
    ))
}

pub struct LineNumberTableAttribute {
    attribute_name_index: u16,
    line_number_table: Vec<LineNumberTable>,
}

pub fn parse_line_number_table_attribute(buf: &[u8]) -> IResult<&[u8], LineNumberTableAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, line_number_table) = length_many(be_u16, parse_line_number_table)(buf)?;

    Ok((
        buf,
        LineNumberTableAttribute {
            attribute_name_index,
            line_number_table,
        },
    ))
}

pub struct LocalVariableTable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

fn parse_local_variable_table(buf: &[u8]) -> IResult<&[u8], LocalVariableTable> {
    let (buf, start_pc) = be_u16(buf)?;
    let (buf, length) = be_u16(buf)?;
    let (buf, name_index) = be_u16(buf)?;
    let (buf, descriptor_index) = be_u16(buf)?;
    let (buf, index) = be_u16(buf)?;

    Ok((
        buf,
        LocalVariableTable {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        },
    ))
}

pub struct LocalVariableTableAttribute {
    attribute_name_index: u16,
    local_variable_table: Vec<LocalVariableTable>,
}

pub fn parse_local_variable_table_attribute(
    buf: &[u8],
) -> IResult<&[u8], LocalVariableTableAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, local_variable_table) = length_many(be_u16, parse_local_variable_table)(buf)?;

    Ok((
        buf,
        LocalVariableTableAttribute {
            attribute_name_index,
            local_variable_table,
        },
    ))
}

pub struct LocalVariableTypeTable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

fn parse_local_variable_type_table(buf: &[u8]) -> IResult<&[u8], LocalVariableTypeTable> {
    let (buf, start_pc) = be_u16(buf)?;
    let (buf, length) = be_u16(buf)?;
    let (buf, name_index) = be_u16(buf)?;
    let (buf, signature_index) = be_u16(buf)?;
    let (buf, index) = be_u16(buf)?;

    Ok((
        buf,
        LocalVariableTypeTable {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        },
    ))
}

pub struct LocalVariableTypeTableAttribute {
    attribute_name_index: u16,
    local_variable_table: Vec<LocalVariableTypeTable>,
}

pub fn parse_local_variable_type_table_attribute(
    buf: &[u8],
) -> IResult<&[u8], LocalVariableTypeTableAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, local_variable_table) = length_many(be_u16, parse_local_variable_type_table)(buf)?;

    Ok((
        buf,
        LocalVariableTypeTableAttribute {
            attribute_name_index,
            local_variable_table,
        },
    ))
}

pub struct DeprecatedAttribute {
    attribute_name_index: u16,
}

pub fn parse_deprecated_attribute(buf: &[u8]) -> IResult<&[u8], DeprecatedAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;

    Ok((
        buf,
        DeprecatedAttribute {
            attribute_name_index,
        },
    ))
}

pub enum ElementValue {
    ConstantByteIndex(u16),
    ConstantCharIndex(u16),
    ConstantDoubleIndex(u16),
    ConstantFloatIndex(u16),
    ConstantIntIndex(u16),
    ConstantLongIndex(u16),
    ConstantShortIndex(u16),
    ConstantBooleanIndex(u16),
    ConstantStringIndex(u16),
    EnumConstantValue {
        type_name_index: u16,
        const_name_index: u16,
    },
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue(Vec<ElementValue>),
}

pub fn parse_element_value(buf: &[u8]) -> IResult<&[u8], ElementValue> {
    let (buf, tag) = be_u8(buf)?;
    match tag {
        b'B' => {
            let (buf, index) = be_u16(buf)?;
            Ok((buf, ConstantByteIndex(index)))
        }
        b'C' => {
            let (buf, index) = be_u16(buf)?;
            Ok((buf, ConstantCharIndex(index)))
        }
        b'D' => {
            let (buf, index) = be_u16(buf)?;
            Ok((buf, ConstantDoubleIndex(index)))
        }
        b'F' => {
            let (buf, index) = be_u16(buf)?;
            Ok((buf, ConstantFloatIndex(index)))
        }
        b'I' => {
            let (buf, index) = be_u16(buf)?;
            Ok((buf, ConstantIntIndex(index)))
        }
        b'J' => {
            let (buf, index) = be_u16(buf)?;
            Ok((buf, ConstantLongIndex(index)))
        }
        b'S' => {
            let (buf, index) = be_u16(buf)?;
            Ok((buf, ConstantShortIndex(index)))
        }
        b'Z' => {
            let (buf, index) = be_u16(buf)?;
            Ok((buf, ConstantBooleanIndex(index)))
        }
        b's' => {
            let (buf, index) = be_u16(buf)?;
            Ok((buf, ConstantStringIndex(index)))
        }
        b'e' => {
            let (buf, type_name_index) = be_u16(buf)?;
            let (buf, const_name_index) = be_u16(buf)?;
            Ok((
                buf,
                EnumConstantValue {
                    type_name_index,
                    const_name_index,
                },
            ))
        }
        b'c' => {
            let (buf, class_info_index) = be_u16(buf)?;
            Ok((buf, ClassInfoIndex(class_info_index)))
        }
        b'@' => {
            let (buf, annotation) = parse_annotation(buf)?;
            Ok((buf, AnnotationValue(annotation)))
        }
        b'[' => {
            let (buf, values) = length_many(be_u16, parse_element_value)(buf)?;
            Ok((buf, ArrayValue(values)))
        }
        _ => unreachable!(),
    }
}

pub struct ElementValuePair {
    element_name_index: u16,
    value: ElementValue,
}

pub fn parse_element_value_pairs(buf: &[u8]) -> IResult<&[u8], ElementValuePair> {
    let (buf, element_name_index) = be_u16(buf)?;
    let (buf, value) = parse_element_value(buf)?;

    Ok((
        buf,
        ElementValuePair {
            element_name_index,
            value,
        },
    ))
}

pub struct Annotation {
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

pub fn parse_annotation(buf: &[u8]) -> IResult<&[u8], Annotation> {
    let (buf, type_index) = be_u16(buf)?;
    let (buf, element_value_pairs) = length_many(be_u16, parse_element_value_pairs)(buf)?;

    Ok((
        buf,
        Annotation {
            type_index,
            element_value_pairs,
        },
    ))
}

pub struct RuntimeVisibleAnnotationsAttribute {
    attribute_name_index: u16,
    annotations: Vec<Annotation>,
}

pub fn parse_runtime_visible_annotations_attribute(
    buf: &[u8],
) -> IResult<&[u8], RuntimeVisibleAnnotationsAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, annotations) = length_many(be_u16, parse_annotation)(buf)?;

    Ok((
        buf,
        RuntimeVisibleAnnotationsAttribute {
            attribute_name_index,
            annotations,
        },
    ))
}

pub struct RuntimeInvisibleAnnotationsAttribute {
    attribute_name_index: u16,
    annotations: Vec<Annotation>,
}

pub fn parse_runtime_invisible_annotations_attribute(
    buf: &[u8],
) -> IResult<&[u8], RuntimeInvisibleAnnotationsAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, annotations) = length_many(be_u16, parse_annotation)(buf)?;

    Ok((
        buf,
        RuntimeInvisibleAnnotationsAttribute {
            attribute_name_index,
            annotations,
        },
    ))
}

pub struct ParameterAnnotation {
    annotations: Vec<Annotation>,
}

pub fn parse_parameter_annotation(buf: &[u8]) -> IResult<&[u8], ParameterAnnotation> {
    let (buf, annotations) = length_many(be_u16, parse_annotation)(buf)?;

    Ok((buf, ParameterAnnotation { annotations }))
}

pub struct RuntimeVisibleParameterAnnotationsAttribute {
    attribute_name_index: u16,
    parameter_annotations: Vec<ParameterAnnotation>,
}

pub fn parse_runtime_visible_parameter_annotations_attribute(
    buf: &[u8],
) -> IResult<&[u8], RuntimeVisibleParameterAnnotationsAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, parameter_annotations) = length_many(be_u16, parse_parameter_annotation)(buf)?;

    Ok((
        buf,
        RuntimeVisibleParameterAnnotationsAttribute {
            attribute_name_index,
            parameter_annotations,
        },
    ))
}

pub struct RuntimeInvisibleParameterAnnotationsAttribute {
    attribute_name_index: u16,
    parameter_annotations: Vec<ParameterAnnotation>,
}

pub fn parse_runtime_invisible_parameter_annotations_attribute(
    buf: &[u8],
) -> IResult<&[u8], RuntimeInvisibleParameterAnnotationsAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, parameter_annotations) = length_many(be_u16, parse_parameter_annotation)(buf)?;

    Ok((
        buf,
        RuntimeInvisibleParameterAnnotationsAttribute {
            attribute_name_index,
            parameter_annotations,
        },
    ))
}

pub struct Table {
    start_pc: u16,
    length: u16,
    index: u16,
}

pub fn parse_table(buf: &[u8]) -> IResult<&[u8], Table> {
    let (buf, start_pc) = be_u16(buf)?;
    let (buf, length) = be_u16(buf)?;
    let (buf, index) = be_u16(buf)?;

    Ok((
        buf,
        Table {
            start_pc,
            length,
            index,
        },
    ))
}

pub enum TargetInfo {
    TypeParameterTarget {
        type_parameter_index: u8,
    },
    SupertypeTarget {
        supertype_index: u16,
    },
    TypeParameterBoundTarget {
        type_parameter_index: u8,
        bound_index: u8,
    },
    EmptyTarget,
    FormalParameterTarget {
        formal_parameter_index: u8,
    },
    ThrowsTarget {
        throws_type_index: u16,
    },
    LocalvarTarget {
        table: Vec<Table>,
    },
    CatchTarget {
        exception_table_index: u16,
    },
    OffsetTarget {
        offset: u16,
    },
    TypeArgumentTarget {
        offset: u16,
        type_parameter_index: u8,
    },
}

fn parse_target_info(target_type: u8, buf: &[u8]) -> IResult<&[u8], TargetInfo> {
    match target_type {
        0x00 | 0x01 => {
            let (buf, type_parameter_index) = be_u8(buf)?;
            Ok((
                buf,
                TypeParameterTarget {
                    type_parameter_index,
                },
            ))
        }
        0x10 => {
            let (buf, supertype_index) = be_u16(buf)?;
            Ok((buf, SupertypeTarget { supertype_index }))
        }
        0x11 | 0x12 => {
            let (buf, type_parameter_index) = be_u8(buf)?;
            let (buf, bound_index) = be_u8(buf)?;
            Ok((
                buf,
                TypeParameterBoundTarget {
                    type_parameter_index,
                    bound_index,
                },
            ))
        }
        0x13 | 0x14 | 0x15 => Ok((buf, EmptyTarget)),
        0x16 => {
            let (buf, formal_parameter_index) = be_u8(buf)?;
            Ok((
                buf,
                FormalParameterTarget {
                    formal_parameter_index,
                },
            ))
        }
        0x17 => {
            let (buf, throws_type_index) = be_u16(buf)?;
            Ok((buf, ThrowsTarget { throws_type_index }))
        }
        0x40 | 0x41 => {
            let (buf, table) = length_many(be_u16, parse_table)(buf)?;
            Ok((buf, LocalvarTarget { table }))
        }
        0x42 => {
            let (buf, exception_table_index) = be_u16(buf)?;
            Ok((
                buf,
                CatchTarget {
                    exception_table_index,
                },
            ))
        }
        0x43 | 0x44 | 0x45 | 0x46 => {
            let (buf, offset) = be_u16(buf)?;
            Ok((buf, OffsetTarget { offset }))
        }
        0x47 | 0x48 | 0x49 | 0x4A | 0x4B => {
            let (buf, offset) = be_u16(buf)?;
            let (buf, type_parameter_index) = be_u8(buf)?;
            Ok((
                buf,
                TypeArgumentTarget {
                    offset,
                    type_parameter_index,
                },
            ))
        }
        _ => unreachable!(),
    }
}

struct Path {
    type_path_kind: u8,
    type_argument_index: u8,
}

fn parse_path(buf: &[u8]) -> IResult<&[u8], Path> {
    let (buf, type_path_kind) = be_u8(buf)?;
    let (buf, type_argument_index) = be_u8(buf)?;
    Ok((
        buf,
        Path {
            type_path_kind,
            type_argument_index,
        },
    ))
}

struct TypePath {
    path: Vec<Path>,
}

fn parse_type_path(buf: &[u8]) -> IResult<&[u8], TypePath> {
    let (buf, path) = length_many(be_u8, parse_path)(buf)?;
    Ok((buf, TypePath { path }))
}

struct TypeAnnotation {
    target_type: u8,
    target_info: TargetInfo,
    target_path: TypePath,
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

fn parse_type_annotation(buf: &[u8]) -> IResult<&[u8], TypeAnnotation> {
    let (buf, target_type) = be_u8(buf)?;
    let (buf, target_info) = parse_target_info(target_type, buf)?;
    let (buf, target_path) = parse_type_path(buf)?;
    let (buf, type_index) = be_u16(buf)?;
    let (buf, element_value_pairs) = length_many(be_u16, parse_element_value_pairs)(buf)?;
    Ok((
        buf,
        TypeAnnotation {
            target_type,
            target_info,
            target_path,
            type_index,
            element_value_pairs,
        },
    ))
}

pub struct RuntimeVisibleTypeAnnotationsAttribute {
    attribute_name_index: u16,
    annotations: Vec<TypeAnnotation>,
}

pub fn parse_runtime_visible_type_annotations_attribute(
    buf: &[u8],
) -> IResult<&[u8], RuntimeVisibleTypeAnnotationsAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, annotations) = length_many(be_u16, parse_type_annotation)(buf)?;

    Ok((
        buf,
        RuntimeVisibleTypeAnnotationsAttribute {
            attribute_name_index,
            annotations,
        },
    ))
}

pub struct RuntimeInvisibleTypeAnnotationsAttribute {
    attribute_name_index: u16,
    annotations: Vec<TypeAnnotation>,
}

pub fn parse_runtime_invisible_type_annotations_attribute(
    buf: &[u8],
) -> IResult<&[u8], RuntimeInvisibleTypeAnnotationsAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, annotations) = length_many(be_u16, parse_type_annotation)(buf)?;

    Ok((
        buf,
        RuntimeInvisibleTypeAnnotationsAttribute {
            attribute_name_index,
            annotations,
        },
    ))
}

pub struct AnnotationDefaultAttribute {
    attribute_name_index: u16,
    default_value: ElementValue,
}

pub fn parse_annotation_default_attribute(
    buf: &[u8],
) -> IResult<&[u8], AnnotationDefaultAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, default_value) = parse_element_value(buf)?;

    Ok((
        buf,
        AnnotationDefaultAttribute {
            attribute_name_index,
            default_value,
        },
    ))
}

pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

fn parse_bootstrap_method(buf: &[u8]) -> IResult<&[u8], BootstrapMethod> {
    let (buf, bootstrap_method_ref) = be_u16(buf)?;
    let (buf, bootstrap_arguments) = length_many(be_u16, be_u16)(buf)?;

    Ok((
        buf,
        BootstrapMethod {
            bootstrap_method_ref,
            bootstrap_arguments,
        },
    ))
}

pub struct BootstrapMethodsAttribute {
    attribute_name_index: u16,
    bootstrap_methods: Vec<BootstrapMethod>,
}

fn parse_bootstrap_methods_attribute(buf: &[u8]) -> IResult<&[u8], BootstrapMethodsAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, bootstrap_methods) = length_many(be_u16, parse_bootstrap_method)(buf)?;

    Ok((
        buf,
        BootstrapMethodsAttribute {
            attribute_name_index,
            bootstrap_methods,
        },
    ))
}

pub struct Parameter {
    name_index: u16,
    access_flags: u16,
}

fn parse_parameter(buf: &[u8]) -> IResult<&[u8], Parameter> {
    let (buf, name_index) = be_u16(buf)?;
    let (buf, access_flags) = be_u16(buf)?;

    Ok((
        buf,
        Parameter {
            name_index,
            access_flags,
        },
    ))
}

pub struct MethodParametersAttribute {
    attribute_name_index: u16,
    parameters: Vec<Parameter>,
}

fn parse_method_parameters_attribute(buf: &[u8]) -> IResult<&[u8], MethodParametersAttribute> {
    let (buf, attribute_name_index) = be_u16(buf)?;
    let (buf, attribute_length) = be_u32(buf)?;
    let (buf, parameters) = length_many(be_u16, parse_parameter)(buf)?;

    Ok((
        buf,
        MethodParametersAttribute {
            attribute_name_index,
            parameters,
        },
    ))
}
