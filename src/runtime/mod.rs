use crate::class_parser::constant_pool::ConstPoolInfo;
use nom::lib::std::collections::VecDeque;

pub type JvmPC = u64;

type OperandStack = VecDeque<i32>;

struct LocalVariableArray {
    local_variables: Vec<i32>,
}

struct JvmFrame {
    local_variable_array: LocalVariableArray,
    operand_stack: OperandStack,
    constant_pool_index: usize,
}

pub struct JvmStack {
    frames: VecDeque<JvmFrame>,
}

pub struct JvmThread {
    stack: JvmStack,
    pc: JvmPC,
}

pub struct JvmHeap {}
struct MethodArea {
    const_pool: VecDeque<ConstPoolInfo>,
}

struct Jvm {
    method_area: MethodArea,
    heap: JvmHeap,
    threads: VecDeque<JvmThread>,
}

impl Jvm {
    pub fn run(&self, initial_class: &str) {}
}
