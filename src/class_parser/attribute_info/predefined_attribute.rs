use crate::class_parser::attribute_info::{parse_attribute_info, AttributeInfo};
use nom::multi::{length_data, many_m_n};
use nom::number::complete::{be_u16, be_u32};
use nom::IResult;

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
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub fn parse_exception_handler(buf: &[u8]) -> IResult<&[u8], ExceptionHandler> {
    let (left, start_pc) = be_u16(buf)?;
    let (left, end_pc) = be_u16(left)?;
    let (left, handler_pc) = be_u16(left)?;
    let (left, catch_type) = be_u16(left)?;
    Ok((
        left,
        ExceptionHandler {
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

enum VerificationTypeInfo {
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

enum StackMapFrame {
    SameFrame,
    SameLocals1StackItemFrame {
        stack: [VerificationTypeInfo; 1],
    },
    SameLocals1StackItemFramExtended {
        offset_delta: u16,
        stack: [VerificationTypeInfo; 1],
    },
    ChopFrame {
        offset_delta: u16,
    },
    SameFrameExtended {
        offset_delta: u16,
    },
    AppendFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

struct StackMapTableAttribute {
    attribute_name_index: u16,
    entries: Vec<StackMapFrame>,
}

struct ExceptionsAttribute {
    attribute_name_index: u16,
    exception_index_table: Vec<u16>,
}

struct Class {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

struct InnerClasses {
    attribute_name_index: u16,
    classes: Vec<Class>,
}

struct EnclosingMethodAttribute {
    attribute_name_index: u16,
    class_index: u16,
    method_index: u16,
}

struct SyntheticAttribute {
    attribute_name_index: u16,
}
struct SignatureAttribute {
    attribute_name_index: u16,
    signature_index: u16,
}
struct SourceFileAttribute {
    attribute_name_index: u16,
    sourcefile_index: u16,
}

struct SourceDebugExtensionAttribute {
    attribute_name_index: u16,
    debug_extension: Vec<u8>,
}

struct LineNumberTable {
    start_pc: u16,
    line_number: u16,
}

struct LineNumberTableAttribute {
    attribute_name_index: u16,
    line_number_table: Vec<LineNumberTable>,
}
struct LocalVariableTable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}
struct LocalVariableTableAttribute {
    attribute_name_index: u16,
    local_variable_table: Vec<LocalVariableTable>,
}

struct LocalVariableTypeTable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}
struct LocalVariableTypeTableAttribute {
    attribute_name_index: u16,
    local_variable_table: Vec<LocalVariableTypeTable>,
}

struct DeprecatedAttribute {
    attribute_name_index: u16,
}

enum ElementValue {
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
    ArrayValue {
        num_values: u16,
        values: Vec<ElementValue>,
    },
}

struct ElementValuePair {
    element_name_index: u16,
    value: ElementValue,
}
struct Annotation {
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

struct RuntimeVisibleAnnotationsAttribute {
    attribute_name_index: u16,
    annotations: Vec<Annotation>,
}

struct RuntimeInvisibleAnnotationsAttribute {
    attribute_name_index: u16,
    annotations: Vec<Annotation>,
}

struct ParameterAnnotation {
    annotations: Vec<Annotation>,
}

struct RuntimeVisibleParameterAnnotationsAttribute {
    attribute_name_index: u16,
    parameter_annotations: Vec<ParameterAnnotation>,
}

struct RuntimeInvisibleParameterAnnotationsAttribute {
    attribute_name_index: u16,
    parameter_annotations: Vec<ParameterAnnotation>,
}

struct Table {
    start_pc: u16,
    length: u16,
    index: u16,
}
enum TargetInfo {
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

struct Path {
    type_path_kind: u8,
    type_argument_index: u8,
}

struct TypePath {
    path: Vec<Path>,
}

struct TypeAnnotation {
    target_type: u8,
    target_info: TargetInfo,
    target_path: TypePath,
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

struct RuntimeVisibleTypeAnnotationsAttribute {
    attribute_name_index: u16,
    annotations: Vec<TypeAnnotation>,
}

struct RuntimeInvisibleTypeAnnotationsAttribute {
    attribute_name_index: u16,
    annotations: Vec<TypeAnnotation>,
}

struct AnnotationDefaultAttribute {
    attribute_name_index: u16,
    default_value: ElementValue,
}

struct BootstrapMethod {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

struct BootstrapMethodsAttribute {
    attribute_name_index: u16,
    bootstrap_methods: Vec<BootstrapMethod>,
}

struct Parameter {
    name_index: u16,
    access_flag: u16,
}

struct MethodParametersAttribute {
    attribute_name_index: u16,
    parameters: Vec<Parameter>,
}
